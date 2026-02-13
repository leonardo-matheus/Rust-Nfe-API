//! Base da NF-e - Estruturas fundamentais da Nota Fiscal Eletrônica
//!
//! Este módulo contém os tipos e estruturas base para tratamento da NF-e,
//! independente do modelo fiscal (NF-e modelo 55 ou NFC-e modelo 65).
//!
//! ## Estrutura do XML da NF-e (Layout 4.00 - SEFAZ)
//!
//! A NF-e segue a estrutura definida no Manual de Orientação do Contribuinte (MOC):
//!
//! ```text
//! <NFe>
//!   <infNFe versao="4.00" Id="NFe...">
//!     <ide>      <!-- Identificação da NF-e -->
//!     <emit>     <!-- Emitente -->
//!     <dest>     <!-- Destinatário (opcional em alguns casos) -->
//!     <det>      <!-- Detalhamento de produtos/serviços (1 a 990 itens) -->
//!     <total>    <!-- Totais da NF-e -->
//!     <transp>   <!-- Transporte -->
//!     <infAdic>  <!-- Informações adicionais -->
//!   </infNFe>
//! </NFe>
//! ```
//!
//! ## Referências
//!
//! - [Manual de Orientação do Contribuinte v6.00](https://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=ndIjl+iEFdE%3D)
//! - [Esquemas XML NF-e](https://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=BMPFMBoln3w%3D)

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

// Submódulos que compõem a estrutura da NF-e
pub mod dest;      // Destinatário (comprador/cliente)
pub mod emit;      // Emitente (vendedor/empresa)
pub mod endereco;  // Endereço (usado por emit e dest)
mod error;         // Tipos de erro da biblioteca
pub mod ide;       // Identificação da nota fiscal
pub mod item;      // Itens/produtos da nota
pub mod totais;    // Totalização de valores
pub mod transporte; // Dados de transporte/frete

use dest::Destinatario;
use emit::Emitente;
pub use error::Error;
use ide::Identificacao;
use item::Item;
use totais::Totalizacao;
use transporte::Transporte;

/// Estrutura principal da Nota Fiscal Eletrônica (NF-e)
///
/// Esta estrutura representa uma NF-e parseada, contendo todos os grupos
/// de informações definidos no layout 4.00 da SEFAZ.
///
/// A NF-e pode ser de dois modelos:
/// - **Modelo 55 (NF-e)**: Nota Fiscal Eletrônica tradicional para operações B2B
/// - **Modelo 65 (NFC-e)**: Nota Fiscal de Consumidor Eletrônica para varejo
///
/// ## Campos Principais
///
/// | Campo | Tag XML | Descrição |
/// |-------|---------|-----------|
/// | versao | @versao | Versão do layout (4.00) |
/// | chave_acesso | @Id | Chave de 44 dígitos que identifica a nota |
/// | ide | \<ide\> | Dados de identificação |
/// | emit | \<emit\> | Dados do emitente |
/// | dest | \<dest\> | Dados do destinatário |
/// | itens | \<det\> | Lista de produtos (1 a 990) |
/// | totais | \<total\> | Valores totalizados |
/// | transporte | \<transp\> | Informações de frete |
///
/// ## Exemplo de Uso
///
/// ```rust,ignore
/// use std::fs::File;
/// use nfe::Nfe;
///
/// let file = File::open("nota.xml")?;
/// let nfe = Nfe::try_from(file)?;
///
/// println!("Chave: {}", nfe.chave_acesso);
/// println!("Total: R$ {:.2}", nfe.totais.valor_total);
/// ```
#[derive(Debug, PartialEq)]
pub struct Nfe {
    /// Versão do layout XML da NF-e (atualmente 4.00)
    pub versao: VersaoLayout,

    /// Chave de acesso de 44 dígitos que identifica unicamente a NF-e
    /// Formato: UF(2) + AAMM(4) + CNPJ(14) + MOD(2) + SERIE(3) + NNF(9) + CODIGO(9) + DV(1)
    pub chave_acesso: String,

    /// Grupo de identificação da NF-e (tag <ide>)
    /// Contém: UF, número, série, modelo, datas, tipo de emissão, etc.
    pub ide: Identificacao,

    /// Dados do emitente da nota fiscal (tag <emit>)
    /// Contém: CNPJ, razão social, endereço, IE, etc.
    pub emit: Emitente,

    /// Dados do destinatário/comprador (tag <dest>)
    /// Opcional em algumas operações (ex: NFC-e para consumidor não identificado)
    pub dest: Option<Destinatario>,

    /// Lista de itens/produtos da nota fiscal (tags <det>)
    /// Cada NF-e pode conter de 1 a 990 itens
    pub itens: Vec<Item>,

    /// Totalização de valores da nota fiscal (tag <total>)
    /// Contém: BC ICMS, valor ICMS, valor produtos, frete, desconto, total, etc.
    pub totais: Totalizacao,

    /// Informações de transporte/frete (tag <transp>)
    /// Contém: modalidade do frete (CIF/FOB), transportador, volumes, etc.
    pub transporte: Transporte,

    /// Informações complementares de interesse do contribuinte (tag <infCpl>)
    /// Campo de texto livre para observações adicionais
    pub informacao_complementar: Option<String>,
}

/// Versão do layout XML da NF-e conforme definido pela SEFAZ
///
/// A versão do layout determina a estrutura do XML e as regras de validação.
/// Desde 2019, a versão 4.00 é obrigatória em todo o Brasil.
///
/// ## Histórico de Versões
///
/// | Versão | Vigência | Observação |
/// |--------|----------|------------|
/// | 1.00 | 2005-2006 | Versão inicial |
/// | 2.00 | 2006-2010 | Expansão nacional |
/// | 3.00 | 2010-2017 | Eventos, cancelamento |
/// | 3.10 | 2017-2019 | Ajustes menores |
/// | 4.00 | 2019-atual | Versão atual obrigatória |
///
/// ## Referência SEFAZ
///
/// Tag XML: `@versao` no elemento `<infNFe>`
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum VersaoLayout {
    /// Layout 4.00 - Versão atual e obrigatória desde 2019
    #[serde(rename = "4.00")]
    V4_00 = 4,
}

impl FromStr for Nfe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl TryFrom<File> for Nfe {
    type Error = Error;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {
        let mut xml = String::new();
        f.read_to_string(&mut xml).map_err(|e| Error::Io(e))?;

        xml.parse::<Nfe>()
    }
}

impl ToString for Nfe {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar a nota")
    }
}

impl<'de> Deserialize<'de> for Nfe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nfe = NfeRootContainer::deserialize(deserializer)?;

        Ok(Self {
            versao: nfe.inf.versao,
            chave_acesso: nfe.inf.chave_acesso.replace("NFe", ""),
            ide: nfe.inf.ide,
            emit: nfe.inf.emit,
            dest: nfe.inf.dest,
            itens: nfe.inf.itens,
            totais: nfe.inf.totais,
            transporte: nfe.inf.transporte,
            informacao_complementar: match nfe.inf.add {
                Some(add) => add.informacao_complementar,
                None => None,
            },
        })
    }
}

impl Serialize for Nfe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let inf = NfeInfContainer {
            versao: self.versao,
            chave_acesso: format!("NFe{}", self.chave_acesso),
            ide: self.ide.clone(),
            emit: self.emit.clone(),
            dest: self.dest.clone(),
            itens: self.itens.clone(),
            totais: self.totais.clone(),
            transporte: self.transporte.clone(),
            add: match self.informacao_complementar.clone() {
                Some(ic) => Some(InfAddContainer {
                    informacao_complementar: Some(ic),
                }),
                None => None,
            },
        };

        let root = NfeRootContainer { inf };

        root.serialize(serializer)
    }
}

impl Serialize for VersaoLayout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            VersaoLayout::V4_00 => "4.00",
        })
    }
}

/// Container auxiliar para deserialização do elemento raiz <NFe>
///
/// O quick-xml necessita de estruturas intermediárias para mapear
/// a hierarquia do XML corretamente. Esta estrutura representa
/// o elemento raiz `<NFe>` que contém `<infNFe>`.
#[derive(Deserialize, Serialize)]
#[serde(rename = "NFe")]
struct NfeRootContainer {
    /// Elemento <infNFe> que contém todas as informações da nota
    #[serde(rename = "infNFe")]
    pub inf: NfeInfContainer,
}

/// Container para informações adicionais (tag <infAdic>)
///
/// Grupo opcional que pode conter informações complementares
/// de interesse do contribuinte e do Fisco.
#[derive(Deserialize, Serialize)]
struct InfAddContainer {
    /// Informações complementares de interesse do contribuinte
    /// Tag: <infCpl> - Máximo 5000 caracteres
    #[serde(rename = "$unflatten=infCpl")]
    pub informacao_complementar: Option<String>,
}

/// Container para o elemento <infNFe> - Informações da NF-e
///
/// Este container mapeia diretamente os atributos e elementos filhos
/// do grupo `<infNFe>`, que é o container principal de dados da nota.
///
/// ## Mapeamento XML -> Rust
///
/// | Atributo/Tag | Campo | Descrição |
/// |--------------|-------|-----------|
/// | @versao | versao | Versão do layout |
/// | @Id | chave_acesso | Chave de acesso (44 dígitos com prefixo "NFe") |
/// | \<ide\> | ide | Identificação |
/// | \<emit\> | emit | Emitente |
/// | \<dest\> | dest | Destinatário |
/// | \<det\> | itens | Itens/produtos (vetor) |
/// | \<total\> | totais | Totalização |
/// | \<transp\> | transporte | Transporte |
/// | \<infAdic\> | add | Informações adicionais |
#[derive(Deserialize, Serialize)]
struct NfeInfContainer {
    /// Versão do layout (atributo @versao)
    #[serde(rename = "versao")]
    pub versao: VersaoLayout,

    /// Chave de acesso com prefixo "NFe" (atributo @Id)
    /// Formato: "NFe" + 44 dígitos
    #[serde(rename = "Id")]
    pub chave_acesso: String,

    /// Grupo de identificação da NF-e
    #[serde(rename = "ide")]
    pub ide: Identificacao,

    /// Grupo de dados do emitente
    #[serde(rename = "emit")]
    pub emit: Emitente,

    /// Grupo de dados do destinatário (opcional)
    #[serde(rename = "dest")]
    pub dest: Option<Destinatario>,

    /// Lista de itens/detalhamento de produtos
    /// Cada elemento <det> representa um item da nota
    #[serde(rename = "det")]
    pub itens: Vec<Item>,

    /// Grupo de totais da NF-e
    #[serde(rename = "total")]
    pub totais: Totalizacao,

    /// Grupo de informações de transporte
    #[serde(rename = "transp")]
    pub transporte: Transporte,

    /// Grupo de informações adicionais (opcional)
    #[serde(rename = "infAdic")]
    pub add: Option<InfAddContainer>,
}
