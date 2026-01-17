//! Destinatário da Nota Fiscal Eletrônica
//!
//! Este módulo contém as estruturas para representar o destinatário
//! (comprador/cliente) da NF-e.

use super::endereco::Endereco;
use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

/// Dados do destinatário da NFe
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename = "dest")]
pub struct Destinatario {
    /// CNPJ do destinatário (14 dígitos)
    #[serde(rename = "CNPJ")]
    pub cnpj: String,
    /// Razão social ou nome do destinatário
    #[serde(rename = "xNome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub razao_social: Option<String>,
    /// Endereço do destinatário
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enderDest")]
    #[serde(default)]
    pub endereco: Option<Endereco>,
    /// Inscrição Estadual do destinatário
    #[serde(rename = "IE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ie: Option<String>,
    /// Indicador da IE do destinatário
    #[serde(rename = "indIEDest")]
    pub indicador_ie: IndicadorContribuicaoIe,
}

/// Indicador da IE do destinatário
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
