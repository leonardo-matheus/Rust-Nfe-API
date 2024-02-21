//!Destinatário da Nfe

use super::endereco::*;
use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

// Dados do destinatário da NFe
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename = "dest")]

pub struct Destinatario {
    #[serde(rename = "$unflatten=CNPJ")]
    pub cnpj: String,
    #[serde(rename = "$unflatten=xNome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub razao_social: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enderDest")]
    pub endereco: Option<Endereco>,
    #[serde(rename = "$unflatten=IE")]
    pub ie: Option<String>,
    #[serde(rename = "$unflatten=indIEDest")]
    pub indicador_ie: IndicadorContribuicaoIe,
}

// Indicador da IE do destinatário
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]

pub enum IndicadorContribuicaoIe {
    /// Contribuinte do ICMS
    ContribuinteIe = 1,
    /// Isento de Inscrição Estadual
    IsentoIe = 2,
    /// Não Contribuinte
    NaoContribuinteIe = 9,
}

impl FromStr for Destinatario {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Destinatario {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar destinatário")
    }
}
