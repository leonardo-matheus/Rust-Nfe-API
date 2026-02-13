# 🦀 Rust NFe API

<p align="center">
  <strong>Biblioteca Rust de alto desempenho para parsing e serialização de Nota Fiscal Eletrônica brasileira</strong>
</p>

<p align="center">
  <a href="https://github.com/leonardo-matheus/Rust-Nfe-API/actions">
    <img src="https://img.shields.io/badge/build-passing-brightgreen" alt="Build Status">
  </a>
  <a href="https://crates.io/crates/nfe">
    <img src="https://img.shields.io/badge/crates.io-0.1.0-orange" alt="Crates.io">
  </a>
  <a href="https://docs.rs/nfe">
    <img src="https://img.shields.io/badge/docs.rs-latest-blue" alt="Documentation">
  </a>
  <a href="https://github.com/leonardo-matheus/Rust-Nfe-API/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  </a>
</p>

<p align="center">
  <a href="https://leonardo-matheus.github.io/Rust-Nfe-API/">📄 Landing Page</a> •
  <a href="#-instalação">📦 Instalação</a> •
  <a href="#-uso-rápido">🚀 Uso Rápido</a> •
  <a href="#-documentação">📚 Documentação</a>
</p>

---

## 📋 Sobre

A **Rust NFe API** é uma biblioteca para manipulação de Notas Fiscais Eletrônicas (NF-e) no formato XML, seguindo a especificação do Layout 4.00 da SEFAZ. Escrita em Rust, oferece alto desempenho, segurança de tipos e facilidade de uso.

### ✨ Funcionalidades

- ⚡ **Alto Desempenho** - Parsing rápido com quick-xml
- 🔒 **Type-Safe** - Tipagem forte com enums para campos codificados
- 📋 **Layout 4.00** - Suporte completo à especificação SEFAZ
- 🔄 **Serialização** - Converta entre Rust structs e XML
- 📦 **NF-e & NFC-e** - Suporte aos modelos 55 e 65
- 🧪 **Testado** - Cobertura de testes abrangente
- 🌐 **API REST & GraphQL** - Servidor web completo
- 📄 **DANFE PDF** - Geração de DANFE profissional
- 🔐 **Certificado A1** - Suporte a certificado digital
- 📡 **SEFAZ WebService** - Cliente SOAP integrado

## ⚡ Benchmark de Performance

Testes realizados em modo release (Windows 11, i7):

| Operação | Tempo Médio |
|----------|-------------|
| **REST API** | |
| Health Check | 2 ms |
| Validar Chave Acesso | 2 ms |
| Parse XML NF-e | 3 ms |
| Export PDF Basico | 4 ms |
| DANFE Profissional | 3 ms |
| **GraphQL API** | |
| GraphQL Health | 5 ms |
| GraphQL Validar Chave | 5 ms |
| GraphQL Schema SDL | 4 ms |

## 📦 Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
nfe = "0.1.0"
```

Ou via cargo:

```bash
cargo add nfe
```

## 🚀 Uso Rápido

### Parse de arquivo XML

```rust
use std::fs::File;
use std::convert::TryFrom;
use nfe::Nfe;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Abrir e fazer parse do arquivo XML
    let file = File::open("nota.xml")?;
    let nfe = Nfe::try_from(file)?;
    
    // Acessar dados da nota
    println!("Chave de Acesso: {}", nfe.chave_acesso);
    println!("Emitente: {:?}", nfe.emit.razao_social);
    println!("Destinatário: {:?}", nfe.dest.razao_social);
    println!("Valor Total: {}", nfe.totais.valor_total);
    
    // Iterar pelos itens
    for item in &nfe.itens {
        println!("Produto: {:?}", item.produto.descricao);
        println!("Quantidade: {}", item.produto.quantidade);
        println!("Valor: {}", item.produto.valor_unitario);
    }
    
    Ok(())
}
```

### Parse de string XML

```rust
use std::convert::TryFrom;
use nfe::Nfe;

fn parse_string(xml_content: &str) -> Result<Nfe, Box<dyn std::error::Error>> {
    let nfe = Nfe::try_from(xml_content.as_bytes())?;
    Ok(nfe)
}
```

### Verificar modelo do documento

```rust
use nfe::{Nfe, ModeloDocumentoFiscal};

fn verificar_modelo(nfe: &Nfe) {
    match nfe.ide.modelo {
        ModeloDocumentoFiscal::Nfe => {
            println!("Nota Fiscal Eletrônica (modelo 55)");
        }
        ModeloDocumentoFiscal::Nfce => {
            println!("Nota Fiscal de Consumidor (modelo 65)");
        }
    }
}
```

## 📚 Documentação

### Estruturas Principais

| Struct | Descrição | Tag XML |
|--------|-----------|---------|
| `Nfe` | Estrutura principal da nota | `<NFe>` |
| `Identificacao` | Dados de identificação | `<ide>` |
| `Emitente` | Dados do emitente | `<emit>` |
| `Destinatario` | Dados do destinatário | `<dest>` |
| `Item` | Item da nota | `<det>` |
| `Produto` | Dados do produto | `<prod>` |
| `Imposto` | Impostos (ICMS, PIS, COFINS) | `<imposto>` |
| `Totalizacao` | Totais da nota | `<total>` |
| `Transporte` | Dados de transporte | `<transp>` |
| `Endereco` | Endereço | `<enderEmit>`, `<enderDest>` |

### Enums Importantes

```rust
// Modelo do documento fiscal
pub enum ModeloDocumentoFiscal {
    Nfe = 55,   // Nota Fiscal Eletrônica
    Nfce = 65,  // Nota Fiscal de Consumidor Eletrônica
}

// Tipo de operação
pub enum TipoOperacao {
    Entrada = 0,
    Saida = 1,
}

// Finalidade da emissão
pub enum FinalidadeEmissao {
    Normal = 1,
    Complementar = 2,
    Ajuste = 3,
    Devolucao = 4,
}

// Modalidade do frete
pub enum ModalidadeFrete {
    ContaEmitente = 0,
    ContaDestinatario = 1,
    ContaTerceiros = 2,
    SemTransporte = 9,
}
```

## 🌐 API REST & GraphQL

O projeto inclui um servidor web completo (`nfe-web`) com:

### Endpoints REST

```bash
# Health check
curl http://localhost:8080/api/health

# Parse XML de NF-e
curl -X POST http://localhost:8080/api/parse \
  -H "Content-Type: application/json" \
  -d '{"xml": "<NFe>...</NFe>"}'

# Validar chave de acesso
curl http://localhost:8080/api/validar-chave/35240508665074000100550010000000011270815480

# Gerar DANFE PDF
curl -X POST http://localhost:8080/api/export/danfe \
  -H "Content-Type: application/json" \
  -d '{"dados": {...}}' -o danfe.pdf
```

### GraphQL

```bash
# Playground: http://localhost:8080/api/graphql/playground

# Query exemplo
curl -X POST http://localhost:8080/api/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ health }"}'

# Validar chave
curl -X POST http://localhost:8080/api/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ validarChave(chave: \"35240508...\") }"}'
```

### Executar servidor

```bash
cd web
cargo run --release
# Servidor em http://localhost:8080
```

## 🧪 Testes

Execute os testes com:

```bash
cd nfe
cargo test
```

Resultado esperado:
```
running 6 tests
test tests::endereco::parse_endereco_sem_complemento ... ok
test tests::endereco::parse_endereco_emitente ... ok
test tests::dest::parse_destinatario ... ok
test tests::dest::parse_destinatario_nao_contribuinte ... ok
test tests::itens::parse_produto ... ok
test tests::itens::parse_item ... ok

test result: ok. 6 passed; 0 failed; 0 ignored
```

## 🗂️ Estrutura do Projeto

```
nfe/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Entrada da biblioteca
│   ├── main.rs          # Exemplo de uso
│   ├── base/            # Estruturas base
│   │   ├── mod.rs       # Estrutura principal NFe
│   │   ├── dest.rs      # Destinatário
│   │   ├── emit.rs      # Emitente
│   │   ├── endereco.rs  # Endereço
│   │   ├── totais.rs    # Totalizações
│   │   ├── transporte.rs# Transporte
│   │   ├── ide/         # Identificação
│   │   │   ├── mod.rs
│   │   │   ├── emissao.rs
│   │   │   └── operacao.rs
│   │   └── item/        # Itens/Produtos
│   │       ├── mod.rs
│   │       ├── produto.rs
│   │       └── imposto/
│   │           ├── mod.rs
│   │           ├── icms.rs
│   │           ├── pis.rs
│   │           └── cofins.rs
│   ├── modelos/         # Modelos NF-e específicos
│   └── tests/           # Testes unitários
└── xmls/                # XMLs de exemplo
```

## 🔧 Dependências

| Crate | Versão | Uso |
|-------|--------|-----|
| `quick-xml` | 0.31 | Parsing XML |
| `serde` | 1.0 | Serialização |
| `serde_repr` | 0.1 | Serialização de enums |
| `chrono` | 0.4 | Data/hora |
| `derive_more` | 0.99 | Derive macros |
| `tokio` | 1.0 | Async runtime |

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Faça um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

## 📬 Contato

- **GitHub**: [@leonardo-matheus](https://github.com/leonardo-matheus)
- **Projeto**: [Rust-Nfe-API](https://github.com/leonardo-matheus/Rust-Nfe-API)
- **Landing Page**: [https://leonardo-matheus.github.io/Rust-Nfe-API/](https://leonardo-matheus.github.io/Rust-Nfe-API/)

---

<p align="center">
  Feito com ❤️ e 🦀 em Rust
</p>
