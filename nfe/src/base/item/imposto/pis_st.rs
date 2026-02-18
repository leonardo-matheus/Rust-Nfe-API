//! PIS ST - PIS Substituição Tributária
//!
//! Este módulo implementa a estrutura para representar o PIS retido
//! por substituição tributária na NF-e.
//!
//! ## Quando Utilizar
//!
//! O grupo PISST deve ser informado quando houver retenção de PIS
//! por substituição tributária na operação.

use serde::{Deserialize, Serialize};

/// PIS Substituição Tributária (tag `<PISST>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PisSt {
    /// Valor da Base de Cálculo do PIS ST (tag `<vBC>`)
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,

    /// Alíquota do PIS ST em percentual (tag `<pPIS>`)
    #[serde(rename = "$unflatten=pPIS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,

    /// Quantidade vendida (tag `<qBCProd>`)
    /// Para cálculo por quantidade
    #[serde(rename = "$unflatten=qBCProd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantidade_bc_produto: Option<f32>,

    /// Alíquota do PIS ST em reais (tag `<vAliqProd>`)
    /// Para cálculo por quantidade
    #[serde(rename = "$unflatten=vAliqProd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_produto: Option<f32>,

    /// Valor do PIS ST (tag `<vPIS>`)
    #[serde(rename = "$unflatten=vPIS")]
    pub valor: f32,
}

impl Default for PisSt {
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
