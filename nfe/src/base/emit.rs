//! Emitente da Nota Fiscal Eletrônica

use super::endereco::Endereco;
use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Emitente da NFe
/// 
/// Contém os dados do emitente (empresa que está emitindo a nota fiscal),
/// incluindo CNPJ, razão social, nome fantasia, inscrição estadual e endereço.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename = "emit")]
pub struct Emitente {
    /// CNPJ do emitente (14 dígitos)
    #[serde(rename = "$unflatten=CNPJ")]
    pub cnpj: Option<String>,
    /// Razão social do emitente
    #[serde(rename = "$unflatten=xNome")]
    pub razao_social: Option<String>,
    /// Nome fantasia do emitente
    #[serde(rename = "$unflatten=xFant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nome_fantasia: Option<String>,
    /// Inscrição Estadual do emitente
    #[serde(rename = "$unflatten=IE")]
    pub ie: Option<String>,
    /// Inscrição Estadual do Substituto Tributário
    #[serde(rename = "$unflatten=IEST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iest: Option<u32>,
    /// Endereço do emitente
    #[serde(rename = "enderEmit")]
    pub endereco: Endereco,
}

impl FromStr for Emitente {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Emitente {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar emitente")
    }
}
