//! Transporte da Nota Fiscal Eletrônica

use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

/// Transporte da nota fiscal
///
/// Contém as informações sobre o transporte dos produtos.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename = "transp")]
pub struct Transporte {
    /// Modalidade do frete
    #[serde(rename = "$unflatten=modFrete")]
    pub modalidade: ModalidadeFrete,
}

impl FromStr for Transporte {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

/// Modalidade do frete
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ModalidadeFrete {
    /// 0 - Contratação do Frete por conta do Remetente (CIF)
    ContratacaoPorContaDoRemetente = 0,
    /// 1 - Contratação do Frete por conta do Destinatário (FOB)
    ContratacaoPorContaDoDestinatario = 1,
    /// 2 - Contratação do Frete por conta de Terceiros
    ContratacaoPorContaDeTerceiros = 2,
    /// 3 - Transporte Próprio por conta do Remetente
    TransportePorContaDoRemetente = 3,
    /// 4 - Transporte Próprio por conta do Destinatário
    TransportePorContaDoDestinatario = 4,
    /// 9 - Sem Ocorrência de Transporte
    SemTransporte = 9,
}
