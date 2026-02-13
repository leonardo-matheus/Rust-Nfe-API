//! Transporte da Nota Fiscal Eletrônica (Grupo `<transp>`)
//!
//! Este módulo contém as estruturas para representar as informações
//! de transporte/frete da NF-e conforme layout SEFAZ.
//!
//! ## Estrutura Completa do Grupo Transporte
//!
//! O grupo `<transp>` pode conter:
//! - `<modFrete>`: Modalidade do frete (obrigatório)
//! - `<transporta>`: Dados do transportador (opcional)
//! - `<veicTransp>`: Dados do veículo (opcional)
//! - `<reboque>`: Dados de reboques (opcional, múltiplos)
//! - `<vol>`: Dados dos volumes transportados (opcional, múltiplos)
//!
//! ## Nota sobre Implementação
//!
//! Esta biblioteca implementa apenas o campo obrigatório `<modFrete>`.
//! Os demais campos podem ser adicionados conforme necessidade.

use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

/// Grupo de Informações do Transporte (tag `<transp>`)
///
/// Contém as informações relacionadas ao transporte dos produtos
/// vendidos na NF-e.
///
/// ## Campo Obrigatório
///
/// O único campo obrigatório é `modFrete`, que indica quem é
/// responsável pelo frete (CIF, FOB, etc.).
///
/// ## Campos Opcionais (não implementados)
///
/// - Dados do transportador (CNPJ, nome, endereço)
/// - Dados do veículo (placa, UF, RNTC)
/// - Dados dos volumes (quantidade, peso bruto/líquido)
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename = "transp")]
pub struct Transporte {
    /// Modalidade do frete (tag `<modFrete>`)
    /// Define quem é responsável pela contratação e/ou pagamento do frete
    #[serde(rename = "$unflatten=modFrete")]
    pub modalidade: ModalidadeFrete,
}

impl FromStr for Transporte {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

/// Modalidade do Frete (tag `<modFrete>`)
///
/// Indica quem é responsável pela contratação e pagamento do frete.
///
/// ## Termos Comerciais (Incoterms)
///
/// | Código | Sigla | Responsabilidade |
/// |--------|-------|------------------|
/// | 0 | CIF | Remetente contrata e paga o frete |
/// | 1 | FOB | Destinatário contrata e paga o frete |
/// | 2 | - | Terceiro contrata e paga o frete |
/// | 3 | - | Remetente transporta com veículo próprio |
/// | 4 | - | Destinatário transporta com veículo próprio |
/// | 9 | - | Não há transporte (retirada no local) |
///
/// ## Impacto Fiscal
///
/// A modalidade do frete afeta:
/// - Base de cálculo do ICMS (frete pode compor a BC)
/// - Direito a crédito de ICMS sobre o frete
/// - Obrigatoriedade de informar dados do transportador
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ModalidadeFrete {
    /// 0 - Frete por conta do Remetente (CIF - Cost, Insurance and Freight)
    /// O vendedor é responsável por contratar e pagar o frete.
    /// O valor do frete geralmente está incluído no preço do produto.
    ContratacaoPorContaDoRemetente = 0,

    /// 1 - Frete por conta do Destinatário (FOB - Free On Board)
    /// O comprador é responsável por contratar e pagar o frete.
    /// O preço do produto não inclui o frete.
    ContratacaoPorContaDoDestinatario = 1,

    /// 2 - Frete por conta de Terceiros
    /// Um terceiro (não remetente nem destinatário) é responsável pelo frete.
    ContratacaoPorContaDeTerceiros = 2,

    /// 3 - Transporte Próprio por conta do Remetente
    /// O remetente transporta com veículo próprio, sem contratação externa.
    TransportePorContaDoRemetente = 3,

    /// 4 - Transporte Próprio por conta do Destinatário
    /// O destinatário retira a mercadoria com veículo próprio.
    TransportePorContaDoDestinatario = 4,

    /// 9 - Sem Ocorrência de Transporte
    /// Usado quando não há deslocamento físico da mercadoria.
    /// Ex: venda de energia elétrica, serviços, ou retirada imediata no balcão.
    SemTransporte = 9,
}
