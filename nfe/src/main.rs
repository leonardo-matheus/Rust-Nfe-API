//! Exemplo de uso da biblioteca NFe
//!
//! Este módulo demonstra como utilizar a biblioteca para fazer parsing
//! de arquivos XML de Nota Fiscal Eletrônica (NF-e) no padrão SEFAZ.
//!
//! # Funcionalidades demonstradas
//!
//! - Leitura assíncrona de arquivo XML
//! - Remoção do namespace para compatibilidade com o parser
//! - Parsing da estrutura completa da NF-e
//! - Exibição formatada dos dados

use std::io;
use nfe_parser::Nfe;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Carrega um arquivo XML para um buffer de string de forma assíncrona.
///
/// Esta função realiza duas operações importantes:
/// 1. Lê o conteúdo completo do arquivo XML
/// 2. Remove o namespace padrão da SEFAZ para permitir o parsing com quick-xml
///
/// O namespace `xmlns="http://www.portalfiscal.inf.br/nfe"` precisa ser removido
/// porque o quick-xml não suporta namespaces diretamente na deserialização.
///
/// # Argumentos
///
/// * `file_path` - Caminho para o arquivo XML da NF-e
///
/// # Retorno
///
/// Retorna o conteúdo do XML como String, sem o namespace
async fn load_xml_to_buffer(file_path: &str) -> Result<String, tokio::io::Error> {
    let mut file = File::open(file_path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;

    // Remove o namespace padrão da SEFAZ para permitir parsing
    // O quick-xml não lida bem com namespaces na deserialização
    Ok(buffer.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", ""))
}

/// Mascara dados sensíveis em uma string para exibição segura.
///
/// Útil para logs e debug onde não se deseja expor dados completos
/// como CNPJ, chave de acesso, etc.
///
/// # Argumentos
///
/// * `data` - String mutável contendo os dados a serem mascarados
/// * `mask_start` - Texto que indica o início da região a mascarar
/// * `mask_len` - Quantidade de caracteres a substituir por '*'
///
/// # Exemplo
///
/// ```ignore
/// let mut chave = "NFe35150300822602000124550010009923461099234656".to_string();
/// mask_sensitive_data(&mut chave, "NFe", 10);
/// // Resultado: "NFe**********22602000124550010009923461099234656"
/// ```
fn mask_sensitive_data(data: &mut String, mask_start: &str, mask_len: usize) {
    if let Some(start) = data.find(mask_start) {
        let start = start + mask_start.len();
        if data.len() > start + mask_len {
            let replacement = "*".repeat(mask_len);
            data.replace_range(start..start + mask_len, &replacement);
        }
    }
}

/// Exibe os dados da NF-e de forma formatada no console.
///
/// Apresenta as principais informações da nota fiscal de forma legível,
/// incluindo identificação, emitente, destinatário, itens e totais.
fn display_nfe(nfe: &Nfe) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              NOTA FISCAL ELETRÔNICA (NF-e)                   ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    // Identificação
    println!("║ IDENTIFICAÇÃO                                                ║");
    println!("╟──────────────────────────────────────────────────────────────╢");
    println!("║ Chave de Acesso: {}...", &nfe.chave_acesso[..20]);
    println!("║ Versão Layout: {:?}", nfe.versao);
    println!("║ Modelo: {:?} | Série: {} | Número: {}",
        nfe.ide.modelo, nfe.ide.serie, nfe.ide.numero);
    println!("║ Ambiente: {:?}", nfe.ide.ambiente);
    println!("║ Natureza da Operação: {}", nfe.ide.operacao.natureza);

    // Emitente
    println!("╟──────────────────────────────────────────────────────────────╢");
    println!("║ EMITENTE                                                     ║");
    println!("╟──────────────────────────────────────────────────────────────╢");
    if let Some(ref cnpj) = nfe.emit.cnpj {
        println!("║ CNPJ: {}", cnpj);
    }
    if let Some(ref razao) = nfe.emit.razao_social {
        println!("║ Razão Social: {}", razao);
    }
    if let Some(ref fantasia) = nfe.emit.nome_fantasia {
        println!("║ Nome Fantasia: {}", fantasia);
    }
    println!("║ Endereço: {}, {}", nfe.emit.endereco.logradouro, nfe.emit.endereco.numero);
    println!("║ Cidade: {} - {}", nfe.emit.endereco.nome_municipio, nfe.emit.endereco.sigla_uf);

    // Destinatário
    if let Some(ref dest) = nfe.dest {
        println!("╟──────────────────────────────────────────────────────────────╢");
        println!("║ DESTINATÁRIO                                                 ║");
        println!("╟──────────────────────────────────────────────────────────────╢");
        println!("║ CNPJ: {}", dest.cnpj);
        if let Some(ref razao) = dest.razao_social {
            println!("║ Razão Social: {}", razao);
        }
        println!("║ Indicador IE: {:?}", dest.indicador_ie);
    }

    // Itens
    println!("╟──────────────────────────────────────────────────────────────╢");
    println!("║ ITENS ({} produtos)                                          ║", nfe.itens.len());
    println!("╟──────────────────────────────────────────────────────────────╢");
    for item in &nfe.itens {
        println!("║ {}. {} ", item.numero, item.produto.descricao);
        println!("║    Qtd: {} {} x R$ {:.2} = R$ {:.2}",
            item.produto.quantidade,
            item.produto.unidade,
            item.produto.valor_unitario,
            item.produto.valor_bruto
        );
    }

    // Totais
    println!("╟──────────────────────────────────────────────────────────────╢");
    println!("║ TOTAIS                                                       ║");
    println!("╟──────────────────────────────────────────────────────────────╢");
    println!("║ Valor dos Produtos: R$ {:.2}", nfe.totais.valor_produtos);
    println!("║ Valor do Frete: R$ {:.2}", nfe.totais.valor_frete);
    println!("║ Valor do Desconto: R$ {:.2}", nfe.totais.valor_desconto);
    println!("║ Valor Total da Nota: R$ {:.2}", nfe.totais.valor_total);
    println!("║ Tributos Aproximados: R$ {:.2}", nfe.totais.valor_aproximado_tributos);

    // Transporte
    println!("╟──────────────────────────────────────────────────────────────╢");
    println!("║ TRANSPORTE                                                   ║");
    println!("╟──────────────────────────────────────────────────────────────╢");
    println!("║ Modalidade do Frete: {:?}", nfe.transporte.modalidade);

    // Informações complementares
    if let Some(ref info) = nfe.informacao_complementar {
        println!("╟──────────────────────────────────────────────────────────────╢");
        println!("║ INFORMAÇÕES COMPLEMENTARES                                   ║");
        println!("╟──────────────────────────────────────────────────────────────╢");
        // Trunca se for muito longo
        let info_truncated = if info.len() > 60 { &info[..60] } else { info };
        println!("║ {}...", info_truncated);
    }

    println!("╚══════════════════════════════════════════════════════════════╝");
}

/// Ponto de entrada principal da aplicação de exemplo.
///
/// Demonstra o fluxo completo de:
/// 1. Carregar um arquivo XML de NF-e
/// 2. Fazer o parsing usando a biblioteca
/// 3. Exibir os dados de forma formatada
///
/// Para testar com seu próprio arquivo XML, altere o caminho no código
/// ou passe como argumento de linha de comando.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Caminho do arquivo XML de exemplo
    // Em produção, este caminho seria passado como argumento ou configuração
    let xml_path = "xmls/nfe.xml";

    println!("Carregando NF-e de: {}", xml_path);
    println!();

    // Carrega e processa o XML
    let xml_data = match load_xml_to_buffer(xml_path).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Erro ao carregar arquivo XML: {}", e);
            eprintln!("Certifique-se de que o arquivo existe no caminho especificado.");
            return Err(e.into());
        }
    };

    // O XML de exemplo está no formato nfeProc (com protocolo de autorização)
    // A biblioteca espera o formato NFe diretamente, então precisamos extrair
    // Para este exemplo, vamos tentar fazer o parse direto

    // Tenta extrair apenas a tag <NFe> do XML
    let nfe_start = xml_data.find("<NFe");
    let nfe_end = xml_data.find("</NFe>");

    if let (Some(start), Some(end)) = (nfe_start, nfe_end) {
        let nfe_xml = &xml_data[start..end + 6]; // +6 para incluir "</NFe>"

        match nfe_xml.parse::<Nfe>() {
            Ok(nfe) => {
                display_nfe(&nfe);
            }
            Err(e) => {
                eprintln!("Erro ao fazer parsing da NF-e: {}", e);
                eprintln!("\nO XML pode estar em um formato diferente do esperado.");
                eprintln!("Verifique se o arquivo está no layout 4.00 da SEFAZ.");
            }
        }
    } else {
        eprintln!("Não foi possível encontrar a tag <NFe> no arquivo XML.");
    }

    println!("\nPressione Enter para sair...");
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Erro ao ler entrada: {}", e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_sensitive_data() {
        let mut data = "NFe35150300822602000124550010009923461099234656".to_string();
        mask_sensitive_data(&mut data, "NFe", 10);
        assert!(data.contains("**********"));
        assert!(data.starts_with("NFe"));
    }

    #[test]
    fn test_mask_data_not_found() {
        let mut data = "SemPrefixo12345".to_string();
        let original = data.clone();
        mask_sensitive_data(&mut data, "NFe", 5);
        assert_eq!(data, original); // Não deve alterar se não encontrar o prefixo
    }
}
