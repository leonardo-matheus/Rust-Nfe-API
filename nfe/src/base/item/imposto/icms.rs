//! ICMS - Imposto sobre Circulação de Mercadorias e Serviços

use serde::{Deserialize, Serialize};

/// Container para os grupos de ICMS
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsContainer {
    /// ICMS Normal
    #[serde(rename = "ICMS00")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms00: Option<Icms00>,
    /// ICMS com redução de base de cálculo
    #[serde(rename = "ICMS10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms10: Option<Icms10>,
    /// ICMS20 - Com redução de base de cálculo
    #[serde(rename = "ICMS20")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms20: Option<Icms20>,
    /// ICMS Simples Nacional
    #[serde(rename = "ICMSSN101")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn101: Option<IcmsSn101>,
    /// ICMS Simples Nacional 102/103/300/400
    #[serde(rename = "ICMSSN102")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn102: Option<IcmsSn102>,
}

/// ICMS Normal - CST 00 - Tributação Integral
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms00 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,
    /// Código de Situação Tributária
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    pub modalidade_bc: u8,
    /// Valor da BC do ICMS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,
    /// Alíquota do ICMS
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,
    /// Valor do ICMS
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,
}

/// ICMS10 - Tributação com ICMS cobrado por ST
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms10 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,
    /// Código de Situação Tributária
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    pub modalidade_bc: u8,
    /// Valor da BC do ICMS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,
    /// Alíquota do ICMS
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,
    /// Valor do ICMS
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,
}

/// ICMS20 - Com redução de base de cálculo
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms20 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,
    /// Código de Situação Tributária
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    pub modalidade_bc: u8,
    /// Percentual de redução da BC
    #[serde(rename = "$unflatten=pRedBC")]
    pub percentual_reducao_bc: f32,
    /// Valor da BC do ICMS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,
    /// Alíquota do ICMS
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,
    /// Valor do ICMS
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,
}

/// ICMS Simples Nacional - CSOSN 101
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn101 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,
    /// Código de Situação da Operação no Simples Nacional
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,
    /// Alíquota do crédito do Simples Nacional
    #[serde(rename = "$unflatten=pCredSN")]
    pub aliquota_credito_sn: f32,
    /// Valor do crédito do ICMS do Simples Nacional
    #[serde(rename = "$unflatten=vCredICMSSN")]
    pub valor_credito_icms_sn: f32,
}

/// ICMS Simples Nacional - CSOSN 102, 103, 300, 400
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn102 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,
    /// Código de Situação da Operação no Simples Nacional
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,
}
