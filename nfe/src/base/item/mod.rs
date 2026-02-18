//! Detalhamento de produtos e serviços da NF-e
//!
//! Este módulo contém as estruturas para representar os itens
//! (produtos e serviços) da Nota Fiscal Eletrônica.

use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod imposto;
mod produto;

pub use imposto::*;
pub use produto::*;

/// Item da Nota Fiscal Eletrônica
///
/// Representa um produto ou serviço vendido na nota fiscal,
/// incluindo dados do produto, quantidade, valores e impostos.
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename = "det")]
pub struct Item {
    /// Número sequencial do item na NF-e
    #[serde(rename = "nItem")]
    pub numero: u8,
    /// Dados do produto ou serviço
    #[serde(rename = "prod")]
    pub produto: Produto,
    /// Impostos incidentes sobre o produto
    #[serde(rename = "imposto")]
    pub imposto: Imposto,
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o item")
    }
}
