//! Impostos da Nota Fiscal Eletrônica
//!
//! Este módulo contém as estruturas para representar os impostos
//! incidentes sobre os produtos da NF-e.

mod cofins;
mod icms;
mod pis;

pub use cofins::*;
pub use icms::*;
pub use pis::*;

use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Impostos do item da nota fiscal
///
/// Contém os tributos incidentes sobre o produto.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename = "imposto")]
pub struct Imposto {
    /// Valor aproximado total de tributos
    #[serde(rename = "$unflatten=vTotTrib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_aproximado_tributos: Option<f32>,
    /// ICMS - Imposto sobre Circulação de Mercadorias e Serviços
    #[serde(rename = "ICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms: Option<IcmsContainer>,
    /// PIS - Programa de Integração Social
    #[serde(rename = "PIS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pis: Option<PisContainer>,
    /// COFINS - Contribuição para Financiamento da Seguridade Social
    #[serde(rename = "COFINS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cofins: Option<CofinsContainer>,
}

impl FromStr for Imposto {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Imposto {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o imposto")
    }
}

impl Default for Imposto {
    fn default() -> Self {
        Self {
            valor_aproximado_tributos: None,
            icms: None,
            pis: None,
            cofins: None,
        }
    }
}
