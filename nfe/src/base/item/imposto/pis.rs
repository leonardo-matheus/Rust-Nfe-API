//! PIS - Programa de Integração Social

use serde::{Deserialize, Serialize};

/// Container para os grupos de PIS
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PisContainer {
    /// PIS Alíquota - CST 01 e 02
    #[serde(rename = "PISAliq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pis_aliq: Option<PisAliq>,
    /// PIS Não Tributado - CST 04, 05, 06, 07, 08 e 09
    #[serde(rename = "PISNT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pis_nt: Option<PisNt>,
    /// PIS Outras Operações
    #[serde(rename = "PISOutr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pis_outr: Option<PisOutr>,
}

/// PIS Alíquota - Tributação por alíquota
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PisAliq {
    /// Código de Situação Tributária do PIS
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
    /// Valor da Base de Cálculo do PIS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,
    /// Alíquota do PIS (em percentual)
    #[serde(rename = "$unflatten=pPIS")]
    pub aliquota: f32,
    /// Valor do PIS
    #[serde(rename = "$unflatten=vPIS")]
    pub valor: f32,
}

/// PIS Não Tributado
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PisNt {
    /// Código de Situação Tributária do PIS
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
}

/// PIS Outras Operações
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PisOutr {
    /// Código de Situação Tributária do PIS
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
    /// Valor da Base de Cálculo do PIS
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,
    /// Alíquota do PIS (em percentual)
    #[serde(rename = "$unflatten=pPIS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,
    /// Valor do PIS
    #[serde(rename = "$unflatten=vPIS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor: Option<f32>,
}
