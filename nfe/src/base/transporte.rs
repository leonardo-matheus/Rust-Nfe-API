//! Transporte

use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

// Transporte da nota
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename = "transp")]

pub struct Transporte {
    #[serde(rename = "$unflatten=modFrete")]
    pub modalidade: ModalidadeFrete,
}

impl FromStr for Transporte {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

//Modalidade do frete
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]

pub enum ModalidadeFrete {
    ContratacaoPorContaDoRemetente = 0,

    ContratacaoPorContaDoDestinatario = 1,

    ContratacaoPorContaDeTerceiros = 2,

    TransportePorContaDoRemetente = 3,

    TransportePorContaDoDestinatario = 4,

    semTransporte = 9,
}
