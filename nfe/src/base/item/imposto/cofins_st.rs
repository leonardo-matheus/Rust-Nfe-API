//! COFINS ST - COFINS Substituição Tributária
//!
//! Este módulo implementa a estrutura para representar a COFINS retida
//! por substituição tributária na NF-e.
//!
//! ## Quando Utilizar
//!
//! O grupo COFINSST deve ser informado quando houver retenção de COFINS
//! por substituição tributária na operação.

use serde::{Deserialize, Serialize};

/// COFINS Substituição Tributária (tag `<COFINSST>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CofinsSt {
    /// Valor da Base de Cálculo da COFINS ST (tag `<vBC>`)
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,

    /// Alíquota da COFINS ST em percentual (tag `<pCOFINS>`)
    #[serde(rename = "$unflatten=pCOFINS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,

    /// Quantidade vendida (tag `<qBCProd>`)
    /// Para cálculo por quantidade
    #[serde(rename = "$unflatten=qBCProd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantidade_bc_produto: Option<f32>,

    /// Alíquota da COFINS ST em reais (tag `<vAliqProd>`)
    /// Para cálculo por quantidade
    #[serde(rename = "$unflatten=vAliqProd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_produto: Option<f32>,

    /// Valor da COFINS ST (tag `<vCOFINS>`)
    #[serde(rename = "$unflatten=vCOFINS")]
    pub valor: f32,
}

impl Default for CofinsSt {
    fn default() -> Self {
        Self {
            valor_bc: None,
            aliquota: None,
            quantidade_bc_produto: None,
            aliquota_produto: None,
            valor: 0.0,
        }
    }
}
