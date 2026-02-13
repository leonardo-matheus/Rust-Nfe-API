//! Tipos GraphQL para NF-e

use async_graphql::{SimpleObject, InputObject, Enum};
use serde::{Deserialize, Serialize};

/// Tipo de documento fiscal
#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum TipoDocumento {
    #[graphql(name = "NFE")]
    Nfe,
    #[graphql(name = "NFCE")]
    Nfce,
    #[graphql(name = "NFSE")]
    Nfse,
    #[graphql(name = "CTE")]
    Cte,
    #[graphql(name = "MDFE")]
    Mdfe,
}

/// Ambiente SEFAZ
#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum Ambiente {
    #[graphql(name = "PRODUCAO")]
    Producao,
    #[graphql(name = "HOMOLOGACAO")]
    Homologacao,
}

/// Status da NF-e
#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum StatusNfe {
    #[graphql(name = "PENDENTE")]
    Pendente,
    #[graphql(name = "AUTORIZADA")]
    Autorizada,
    #[graphql(name = "CANCELADA")]
    Cancelada,
    #[graphql(name = "DENEGADA")]
    Denegada,
    #[graphql(name = "REJEITADA")]
    Rejeitada,
}

/// NF-e completa (output)
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct NfeType {
    pub id: String,
    pub chave_acesso: String,
    pub numero: i32,
    pub serie: i32,
    pub tipo: TipoDocumento,
    pub ambiente: Ambiente,
    pub status: StatusNfe,
    pub data_emissao: String,
    pub data_autorizacao: Option<String>,
    pub protocolo: Option<String>,
    pub emitente: EmitenteType,
    pub destinatario: Option<DestinatarioType>,
    pub itens: Vec<ItemType>,
    pub totais: TotaisType,
    pub xml: Option<String>,
}

/// Emitente
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct EmitenteType {
    pub cnpj: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub endereco: EnderecoType,
}

/// Destinatário
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct DestinatarioType {
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub razao_social: String,
    pub inscricao_estadual: Option<String>,
    pub endereco: Option<EnderecoType>,
}

/// Endereço
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct EnderecoType {
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub municipio: String,
    pub uf: String,
    pub cep: String,
    pub pais: Option<String>,
}

/// Item da NF-e
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct ItemType {
    pub numero: i32,
    pub codigo: String,
    pub descricao: String,
    pub ncm: String,
    pub cfop: String,
    pub unidade: String,
    pub quantidade: f64,
    pub valor_unitario: f64,
    pub valor_total: f64,
}

/// Totais da NF-e
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct TotaisType {
    pub base_calculo_icms: f64,
    pub valor_icms: f64,
    pub valor_produtos: f64,
    pub valor_frete: f64,
    pub valor_desconto: f64,
    pub valor_total: f64,
}

/// Resultado de consulta SEFAZ
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct ConsultaSefazResult {
    pub sucesso: bool,
    pub codigo_status: String,
    pub motivo: String,
    pub chave_acesso: Option<String>,
    pub protocolo: Option<String>,
    pub data_recebimento: Option<String>,
    pub situacao: Option<String>,
}

/// Resultado de emissão
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct EmissaoResult {
    pub sucesso: bool,
    pub codigo_status: String,
    pub motivo: String,
    pub chave_acesso: Option<String>,
    pub protocolo: Option<String>,
    pub xml_autorizado: Option<String>,
}

/// Resultado de cancelamento
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct CancelamentoResult {
    pub sucesso: bool,
    pub codigo_status: String,
    pub motivo: String,
    pub protocolo: Option<String>,
    pub data_cancelamento: Option<String>,
}

/// Informações do certificado
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct CertificadoInfoType {
    pub cnpj: Option<String>,
    pub razao_social: Option<String>,
    pub valido: bool,
    pub data_validade: String,
    pub dias_para_expirar: i32,
}

// ============================================================================
// Input Types
// ============================================================================

/// Input para criar NF-e
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct NfeInput {
    pub numero: i32,
    pub serie: i32,
    pub natureza_operacao: String,
    pub ambiente: Ambiente,
    pub emitente: EmitenteInput,
    pub destinatario: Option<DestinatarioInput>,
    pub itens: Vec<ItemInput>,
}

/// Input emitente
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct EmitenteInput {
    pub cnpj: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub endereco: EnderecoInput,
}

/// Input destinatário
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct DestinatarioInput {
    pub cnpj: Option<String>,
    pub cpf: Option<String>,
    pub razao_social: String,
    pub inscricao_estadual: Option<String>,
    pub endereco: Option<EnderecoInput>,
}

/// Input endereço
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct EnderecoInput {
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub municipio: String,
    pub codigo_municipio: String,
    pub uf: String,
    pub cep: String,
}

/// Input item
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct ItemInput {
    pub codigo: String,
    pub descricao: String,
    pub ncm: String,
    pub cfop: String,
    pub unidade: String,
    pub quantidade: f64,
    pub valor_unitario: f64,
}

/// Input para cancelamento
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct CancelamentoInput {
    pub chave_acesso: String,
    pub protocolo_autorizacao: String,
    pub justificativa: String,
}

/// Input para carta de correção
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct CartaCorrecaoInput {
    pub chave_acesso: String,
    pub sequencia: i32,
    pub correcao: String,
}

/// Filtros para listagem
#[derive(Debug, Clone, InputObject, Default, Serialize, Deserialize)]
pub struct NfeFilter {
    pub cnpj_emitente: Option<String>,
    pub data_inicio: Option<String>,
    pub data_fim: Option<String>,
    pub status: Option<StatusNfe>,
    pub numero: Option<i32>,
}

/// Paginação
#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct Pagination {
    #[graphql(default = 0)]
    pub offset: i32,
    #[graphql(default = 20)]
    pub limit: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { offset: 0, limit: 20 }
    }
}
