//! COFINS - Contribuição para Financiamento da Seguridade Social

use serde::{Deserialize, Serialize};

/// Container para os grupos de COFINS
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CofinsContainer {
    /// COFINS Alíquota - CST 01 e 02
    #[serde(rename = "COFINSAliq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cofins_aliq: Option<CofinsAliq>,
    /// COFINS Não Tributado - CST 04, 05, 06, 07, 08 e 09
    #[serde(rename = "COFINSNT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cofins_nt: Option<CofinsNt>,
    /// COFINS Outras Operações
    #[serde(rename = "COFINSOutr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cofins_outr: Option<CofinsOutr>,
}

/// COFINS Alíquota - Tributação por alíquota
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CofinsAliq {
    /// Código de Situação Tributária da COFINS
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
    /// Valor da Base de Cálculo da COFINS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,
    /// Alíquota da COFINS (em percentual)
    #[serde(rename = "$unflatten=pCOFINS")]
    pub aliquota: f32,
    /// Valor da COFINS
    #[serde(rename = "$unflatten=vCOFINS")]
    pub valor: f32,
}

/// COFINS Não Tributado
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CofinsNt {
    /// Código de Situação Tributária da COFINS
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
}

/// COFINS Outras Operações
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CofinsOutr {
    /// Código de Situação Tributária da COFINS
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
    /// Valor da Base de Cálculo da COFINS
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,
    /// Alíquota da COFINS (em percentual)
    #[serde(rename = "$unflatten=pCOFINS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,
    /// Valor da COFINS
    #[serde(rename = "$unflatten=vCOFINS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor: Option<f32>,
}