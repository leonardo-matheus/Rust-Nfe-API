//! ICMS - Imposto sobre Circulação de Mercadorias e Serviços
//!
//! Este módulo implementa as estruturas para representar o ICMS na NF-e,
//! conforme definido no layout 4.00 da SEFAZ.
//!
//! ## Códigos de Situação Tributária (CST)
//!
//! Para empresas do **Regime Normal** (Lucro Real/Presumido):
//!
//! | CST | Descrição |
//! |-----|-----------|
//! | 00 | Tributada integralmente |
//! | 10 | Tributada com cobrança do ICMS por ST |
//! | 20 | Com redução de base de cálculo |
//! | 30 | Isenta ou não tributada com cobrança de ST |
//! | 40 | Isenta |
//! | 41 | Não tributada |
//! | 50 | Suspensão |
//! | 51 | Diferimento |
//! | 60 | ICMS cobrado anteriormente por ST |
//! | 70 | Com redução de BC e cobrança de ST |
//! | 90 | Outras |
//!
//! ## Códigos de Situação da Operação no Simples Nacional (CSOSN)
//!
//! Para empresas do **Simples Nacional**:
//!
//! | CSOSN | Descrição |
//! |-------|-----------|
//! | 101 | Tributada com permissão de crédito |
//! | 102 | Tributada sem permissão de crédito |
//! | 103 | Isenção do ICMS para faixa de receita bruta |
//! | 201 | Tributada com permissão de crédito e ST |
//! | 202 | Tributada sem permissão de crédito e ST |
//! | 203 | Isenção do ICMS e ST |
//! | 300 | Imune |
//! | 400 | Não tributada pelo Simples Nacional |
//! | 500 | ICMS cobrado anteriormente por ST |
//! | 900 | Outros |

use serde::{Deserialize, Serialize};

/// Container para os grupos de ICMS (tag `<ICMS>`)
///
/// O ICMS é informado através de grupos exclusivos, onde apenas UM grupo
/// deve estar presente por item, dependendo do CST/CSOSN aplicável.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct IcmsContainer {
    /// ICMS CST 00 - Tributação integral sem redução de BC
    #[serde(rename = "ICMS00")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms00: Option<Icms00>,

    /// ICMS CST 10 - Tributação com ICMS por Substituição Tributária
    #[serde(rename = "ICMS10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms10: Option<Icms10>,

    /// ICMS CST 20 - Tributação com redução de base de cálculo
    #[serde(rename = "ICMS20")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms20: Option<Icms20>,

    /// ICMS CST 30 - Isenta ou não tributada com cobrança do ICMS por ST
    #[serde(rename = "ICMS30")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms30: Option<Icms30>,

    /// ICMS CST 40 - Isenta
    #[serde(rename = "ICMS40")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms40: Option<Icms40>,

    /// ICMS CST 41 - Não tributada
    #[serde(rename = "ICMS41")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms41: Option<Icms41>,

    /// ICMS CST 50 - Suspensão
    #[serde(rename = "ICMS50")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms50: Option<Icms50>,

    /// ICMS CST 51 - Diferimento
    #[serde(rename = "ICMS51")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms51: Option<Icms51>,

    /// ICMS CST 60 - ICMS cobrado anteriormente por Substituição Tributária
    #[serde(rename = "ICMS60")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms60: Option<Icms60>,

    /// ICMS CST 70 - Com redução de BC e cobrança do ICMS por ST
    #[serde(rename = "ICMS70")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms70: Option<Icms70>,

    /// ICMS CST 90 - Outras
    #[serde(rename = "ICMS90")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms90: Option<Icms90>,

    /// ICMS Simples Nacional CSOSN 101 - Tributada com permissão de crédito
    #[serde(rename = "ICMSSN101")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn101: Option<IcmsSn101>,

    /// ICMS Simples Nacional CSOSN 102/103/300/400
    #[serde(rename = "ICMSSN102")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn102: Option<IcmsSn102>,

    /// ICMS Simples Nacional CSOSN 201 - Tributada com permissão de crédito e ST
    #[serde(rename = "ICMSSN201")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn201: Option<IcmsSn201>,

    /// ICMS Simples Nacional CSOSN 202/203 - Tributada sem permissão de crédito e ST
    #[serde(rename = "ICMSSN202")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn202: Option<IcmsSn202>,

    /// ICMS Simples Nacional CSOSN 500 - ICMS cobrado anteriormente por ST
    #[serde(rename = "ICMSSN500")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn500: Option<IcmsSn500>,

    /// ICMS Simples Nacional CSOSN 900 - Outros
    #[serde(rename = "ICMSSN900")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn900: Option<IcmsSn900>,
}

/// ICMS CST 00 - Tributação Integral (tag `<ICMS00>`)
///
/// Usado quando o produto é tributado integralmente pelo ICMS,
/// sem redução de base de cálculo ou substituição tributária.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms00 {
    /// Origem da mercadoria (0-8)
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "00"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Modalidade de determinação da BC do ICMS (0-3)
    #[serde(rename = "$unflatten=modBC")]
    pub modalidade_bc: u8,

    /// Valor da Base de Cálculo do ICMS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,

    /// Alíquota do ICMS em percentual
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,

    /// Valor do ICMS
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,

    /// Percentual do FCP (Fundo de Combate à Pobreza)
    #[serde(rename = "$unflatten=pFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp: Option<f32>,

    /// Valor do FCP
    #[serde(rename = "$unflatten=vFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp: Option<f32>,
}

/// ICMS CST 10 - Tributação com ICMS por Substituição Tributária (tag `<ICMS10>`)
///
/// Usado quando há tributação normal do ICMS E cobrança antecipada do ICMS
/// por substituição tributária (ST) para as operações subsequentes.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms10 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "10"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    pub modalidade_bc: u8,

    /// Valor da Base de Cálculo do ICMS próprio
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,

    /// Alíquota do ICMS próprio em percentual
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,

    /// Valor do ICMS próprio
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,

    /// Percentual do FCP
    #[serde(rename = "$unflatten=pFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp: Option<f32>,

    /// Valor do FCP
    #[serde(rename = "$unflatten=vFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp: Option<f32>,

    /// Modalidade de determinação da BC do ICMS ST (0-5)
    #[serde(rename = "$unflatten=modBCST")]
    pub modalidade_bc_st: u8,

    /// Percentual da margem de valor Adicionado do ICMS ST
    #[serde(rename = "$unflatten=pMVAST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_mva_st: Option<f32>,

    /// Percentual de redução da BC do ICMS ST
    #[serde(rename = "$unflatten=pRedBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_st: Option<f32>,

    /// Valor da Base de Cálculo do ICMS ST
    #[serde(rename = "$unflatten=vBCST")]
    pub valor_bc_st: f32,

    /// Alíquota do ICMS ST em percentual
    #[serde(rename = "$unflatten=pICMSST")]
    pub aliquota_st: f32,

    /// Valor do ICMS ST
    #[serde(rename = "$unflatten=vICMSST")]
    pub valor_st: f32,

    /// Percentual do FCP retido por ST
    #[serde(rename = "$unflatten=pFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st: Option<f32>,
}

/// ICMS CST 20 - Tributação com Redução de Base de Cálculo (tag `<ICMS20>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms20 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "20"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    pub modalidade_bc: u8,

    /// Percentual de redução da BC
    #[serde(rename = "$unflatten=pRedBC")]
    pub percentual_reducao_bc: f32,

    /// Valor da BC do ICMS JÁ REDUZIDA
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,

    /// Alíquota do ICMS em percentual
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,

    /// Valor do ICMS
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,

    /// Percentual do FCP
    #[serde(rename = "$unflatten=pFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp: Option<f32>,

    /// Valor do FCP
    #[serde(rename = "$unflatten=vFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp: Option<f32>,

    /// Valor do ICMS desonerado
    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_desonerado: Option<f32>,

    /// Motivo da desoneração (3-12)
    #[serde(rename = "$unflatten=motDesICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo_desoneracao: Option<u8>,
}

/// ICMS CST 30 - Isenta ou não tributada com cobrança do ICMS por ST (tag `<ICMS30>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms30 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "30"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Modalidade de determinação da BC do ICMS ST
    #[serde(rename = "$unflatten=modBCST")]
    pub modalidade_bc_st: u8,

    /// Percentual da margem de valor Adicionado do ICMS ST
    #[serde(rename = "$unflatten=pMVAST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_mva_st: Option<f32>,

    /// Percentual de redução da BC do ICMS ST
    #[serde(rename = "$unflatten=pRedBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_st: Option<f32>,

    /// Valor da Base de Cálculo do ICMS ST
    #[serde(rename = "$unflatten=vBCST")]
    pub valor_bc_st: f32,

    /// Alíquota do ICMS ST
    #[serde(rename = "$unflatten=pICMSST")]
    pub aliquota_st: f32,

    /// Valor do ICMS ST
    #[serde(rename = "$unflatten=vICMSST")]
    pub valor_st: f32,

    /// Percentual do FCP retido por ST
    #[serde(rename = "$unflatten=pFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st: Option<f32>,

    /// Valor do ICMS desonerado
    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_desonerado: Option<f32>,

    /// Motivo da desoneração
    #[serde(rename = "$unflatten=motDesICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo_desoneracao: Option<u8>,
}

/// ICMS CST 40 - Isenta (tag `<ICMS40>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms40 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "40"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Valor do ICMS desonerado
    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_desonerado: Option<f32>,

    /// Motivo da desoneração (1=Táxi, 3=Produtor Agropecuário, etc.)
    #[serde(rename = "$unflatten=motDesICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo_desoneracao: Option<u8>,
}

/// ICMS CST 41 - Não tributada (tag `<ICMS41>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms41 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "41"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Valor do ICMS desonerado
    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_desonerado: Option<f32>,

    /// Motivo da desoneração
    #[serde(rename = "$unflatten=motDesICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo_desoneracao: Option<u8>,
}

/// ICMS CST 50 - Suspensão (tag `<ICMS50>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms50 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "50"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Valor do ICMS desonerado
    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_desonerado: Option<f32>,

    /// Motivo da desoneração
    #[serde(rename = "$unflatten=motDesICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo_desoneracao: Option<u8>,
}

/// ICMS CST 51 - Diferimento (tag `<ICMS51>`)
///
/// Pode ter tributação parcial com diferimento do restante.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms51 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "51"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalidade_bc: Option<u8>,

    /// Percentual de redução da BC
    #[serde(rename = "$unflatten=pRedBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc: Option<f32>,

    /// Valor da BC do ICMS
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,

    /// Alíquota do ICMS
    #[serde(rename = "$unflatten=pICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,

    /// Valor do ICMS da operação
    #[serde(rename = "$unflatten=vICMSOp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_operacao: Option<f32>,

    /// Percentual do diferimento
    #[serde(rename = "$unflatten=pDif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_diferimento: Option<f32>,

    /// Valor do ICMS diferido
    #[serde(rename = "$unflatten=vICMSDif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_diferido: Option<f32>,

    /// Valor do ICMS realmente devido
    #[serde(rename = "$unflatten=vICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor: Option<f32>,

    /// Percentual do FCP
    #[serde(rename = "$unflatten=pFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp: Option<f32>,

    /// Valor do FCP
    #[serde(rename = "$unflatten=vFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp: Option<f32>,
}

/// ICMS CST 60 - ICMS cobrado anteriormente por ST (tag `<ICMS60>`)
///
/// Usado quando o ICMS já foi retido anteriormente por substituição tributária.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms60 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "60"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Valor da BC do ICMS ST retido anteriormente
    #[serde(rename = "$unflatten=vBCSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_st_retido: Option<f32>,

    /// Alíquota suportada pelo consumidor final
    #[serde(rename = "$unflatten=pST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_st_consumidor: Option<f32>,

    /// Valor do ICMS próprio do substituto
    #[serde(rename = "$unflatten=vICMSSubstituto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_substituto: Option<f32>,

    /// Valor do ICMS ST retido anteriormente
    #[serde(rename = "$unflatten=vICMSSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_st_retido: Option<f32>,

    /// Valor da BC do FCP retido anteriormente por ST
    #[serde(rename = "$unflatten=vBCFCPSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_fcp_st_retido: Option<f32>,

    /// Percentual do FCP retido anteriormente por ST
    #[serde(rename = "$unflatten=pFCPSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st_retido: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st_retido: Option<f32>,

    /// Percentual de redução da BC efetiva
    #[serde(rename = "$unflatten=pRedBCEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_efetiva: Option<f32>,

    /// Valor da BC efetiva
    #[serde(rename = "$unflatten=vBCEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_efetiva: Option<f32>,

    /// Alíquota do ICMS efetiva
    #[serde(rename = "$unflatten=pICMSEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_efetiva: Option<f32>,

    /// Valor do ICMS efetivo
    #[serde(rename = "$unflatten=vICMSEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_efetivo: Option<f32>,
}

/// ICMS CST 70 - Com redução de BC e cobrança do ICMS por ST (tag `<ICMS70>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms70 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "70"
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

    /// Percentual do FCP
    #[serde(rename = "$unflatten=pFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp: Option<f32>,

    /// Valor do FCP
    #[serde(rename = "$unflatten=vFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp: Option<f32>,

    /// Modalidade de determinação da BC do ICMS ST
    #[serde(rename = "$unflatten=modBCST")]
    pub modalidade_bc_st: u8,

    /// Percentual da MVA do ICMS ST
    #[serde(rename = "$unflatten=pMVAST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_mva_st: Option<f32>,

    /// Percentual de redução da BC do ICMS ST
    #[serde(rename = "$unflatten=pRedBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_st: Option<f32>,

    /// Valor da BC do ICMS ST
    #[serde(rename = "$unflatten=vBCST")]
    pub valor_bc_st: f32,

    /// Alíquota do ICMS ST
    #[serde(rename = "$unflatten=pICMSST")]
    pub aliquota_st: f32,

    /// Valor do ICMS ST
    #[serde(rename = "$unflatten=vICMSST")]
    pub valor_st: f32,

    /// Percentual do FCP retido por ST
    #[serde(rename = "$unflatten=pFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st: Option<f32>,

    /// Valor do ICMS desonerado
    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_desonerado: Option<f32>,

    /// Motivo da desoneração
    #[serde(rename = "$unflatten=motDesICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo_desoneracao: Option<u8>,
}

/// ICMS CST 90 - Outras (tag `<ICMS90>`)
///
/// Usado para situações que não se enquadram nos CSTs anteriores.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms90 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária - sempre "90"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalidade_bc: Option<u8>,

    /// Percentual de redução da BC
    #[serde(rename = "$unflatten=pRedBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc: Option<f32>,

    /// Valor da BC do ICMS
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,

    /// Alíquota do ICMS
    #[serde(rename = "$unflatten=pICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,

    /// Valor do ICMS
    #[serde(rename = "$unflatten=vICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor: Option<f32>,

    /// Percentual do FCP
    #[serde(rename = "$unflatten=pFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp: Option<f32>,

    /// Valor do FCP
    #[serde(rename = "$unflatten=vFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp: Option<f32>,

    /// Modalidade de determinação da BC do ICMS ST
    #[serde(rename = "$unflatten=modBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalidade_bc_st: Option<u8>,

    /// Percentual da MVA do ICMS ST
    #[serde(rename = "$unflatten=pMVAST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_mva_st: Option<f32>,

    /// Percentual de redução da BC do ICMS ST
    #[serde(rename = "$unflatten=pRedBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_st: Option<f32>,

    /// Valor da BC do ICMS ST
    #[serde(rename = "$unflatten=vBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_st: Option<f32>,

    /// Alíquota do ICMS ST
    #[serde(rename = "$unflatten=pICMSST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_st: Option<f32>,

    /// Valor do ICMS ST
    #[serde(rename = "$unflatten=vICMSST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_st: Option<f32>,

    /// Percentual do FCP retido por ST
    #[serde(rename = "$unflatten=pFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st: Option<f32>,

    /// Valor do ICMS desonerado
    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_desonerado: Option<f32>,

    /// Motivo da desoneração
    #[serde(rename = "$unflatten=motDesICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo_desoneracao: Option<u8>,
}

// ==================== SIMPLES NACIONAL ====================

/// ICMS Simples Nacional CSOSN 101 (tag `<ICMSSN101>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn101 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação da Operação no Simples Nacional - sempre "101"
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,

    /// Alíquota aplicável de cálculo do crédito
    #[serde(rename = "$unflatten=pCredSN")]
    pub aliquota_credito_sn: f32,

    /// Valor do crédito do ICMS permitido
    #[serde(rename = "$unflatten=vCredICMSSN")]
    pub valor_credito_icms_sn: f32,
}

/// ICMS Simples Nacional CSOSN 102, 103, 300, 400 (tag `<ICMSSN102>`)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn102 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação da Operação no Simples Nacional
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,
}

/// ICMS Simples Nacional CSOSN 201 (tag `<ICMSSN201>`)
///
/// Tributada com permissão de crédito e cobrança do ICMS por ST.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn201 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// CSOSN - sempre "201"
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,

    /// Modalidade de determinação da BC do ICMS ST
    #[serde(rename = "$unflatten=modBCST")]
    pub modalidade_bc_st: u8,

    /// Percentual da MVA do ICMS ST
    #[serde(rename = "$unflatten=pMVAST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_mva_st: Option<f32>,

    /// Percentual de redução da BC do ICMS ST
    #[serde(rename = "$unflatten=pRedBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_st: Option<f32>,

    /// Valor da BC do ICMS ST
    #[serde(rename = "$unflatten=vBCST")]
    pub valor_bc_st: f32,

    /// Alíquota do ICMS ST
    #[serde(rename = "$unflatten=pICMSST")]
    pub aliquota_st: f32,

    /// Valor do ICMS ST
    #[serde(rename = "$unflatten=vICMSST")]
    pub valor_st: f32,

    /// Percentual do FCP retido por ST
    #[serde(rename = "$unflatten=pFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st: Option<f32>,

    /// Alíquota aplicável de cálculo do crédito
    #[serde(rename = "$unflatten=pCredSN")]
    pub aliquota_credito_sn: f32,

    /// Valor do crédito do ICMS
    #[serde(rename = "$unflatten=vCredICMSSN")]
    pub valor_credito_icms_sn: f32,
}

/// ICMS Simples Nacional CSOSN 202/203 (tag `<ICMSSN202>`)
///
/// Tributada sem permissão de crédito e cobrança do ICMS por ST.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn202 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// CSOSN - "202" ou "203"
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,

    /// Modalidade de determinação da BC do ICMS ST
    #[serde(rename = "$unflatten=modBCST")]
    pub modalidade_bc_st: u8,

    /// Percentual da MVA do ICMS ST
    #[serde(rename = "$unflatten=pMVAST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_mva_st: Option<f32>,

    /// Percentual de redução da BC do ICMS ST
    #[serde(rename = "$unflatten=pRedBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_st: Option<f32>,

    /// Valor da BC do ICMS ST
    #[serde(rename = "$unflatten=vBCST")]
    pub valor_bc_st: f32,

    /// Alíquota do ICMS ST
    #[serde(rename = "$unflatten=pICMSST")]
    pub aliquota_st: f32,

    /// Valor do ICMS ST
    #[serde(rename = "$unflatten=vICMSST")]
    pub valor_st: f32,

    /// Percentual do FCP retido por ST
    #[serde(rename = "$unflatten=pFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st: Option<f32>,
}

/// ICMS Simples Nacional CSOSN 500 (tag `<ICMSSN500>`)
///
/// ICMS cobrado anteriormente por substituição tributária.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn500 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// CSOSN - sempre "500"
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,

    /// Valor da BC do ICMS ST retido anteriormente
    #[serde(rename = "$unflatten=vBCSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_st_retido: Option<f32>,

    /// Alíquota suportada pelo consumidor final
    #[serde(rename = "$unflatten=pST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_st_consumidor: Option<f32>,

    /// Valor do ICMS próprio do substituto
    #[serde(rename = "$unflatten=vICMSSubstituto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_icms_substituto: Option<f32>,

    /// Valor do ICMS ST retido
    #[serde(rename = "$unflatten=vICMSSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_st_retido: Option<f32>,

    /// Valor da BC do FCP retido anteriormente
    #[serde(rename = "$unflatten=vBCFCPSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_fcp_st_retido: Option<f32>,

    /// Percentual do FCP retido anteriormente
    #[serde(rename = "$unflatten=pFCPSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st_retido: Option<f32>,

    /// Valor do FCP retido
    #[serde(rename = "$unflatten=vFCPSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st_retido: Option<f32>,

    /// Percentual de redução da BC efetiva
    #[serde(rename = "$unflatten=pRedBCEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_efetiva: Option<f32>,

    /// Valor da BC efetiva
    #[serde(rename = "$unflatten=vBCEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_efetiva: Option<f32>,

    /// Alíquota do ICMS efetiva
    #[serde(rename = "$unflatten=pICMSEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_efetiva: Option<f32>,

    /// Valor do ICMS efetivo
    #[serde(rename = "$unflatten=vICMSEfet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_efetivo: Option<f32>,
}

/// ICMS Simples Nacional CSOSN 900 (tag `<ICMSSN900>`)
///
/// Outros - situações não previstas nos CSOSNs anteriores.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn900 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// CSOSN - sempre "900"
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,

    /// Modalidade de determinação da BC do ICMS
    #[serde(rename = "$unflatten=modBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalidade_bc: Option<u8>,

    /// Percentual de redução da BC
    #[serde(rename = "$unflatten=pRedBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc: Option<f32>,

    /// Valor da BC do ICMS
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,

    /// Alíquota do ICMS
    #[serde(rename = "$unflatten=pICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,

    /// Valor do ICMS
    #[serde(rename = "$unflatten=vICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor: Option<f32>,

    /// Modalidade de determinação da BC do ICMS ST
    #[serde(rename = "$unflatten=modBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalidade_bc_st: Option<u8>,

    /// Percentual da MVA do ICMS ST
    #[serde(rename = "$unflatten=pMVAST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_mva_st: Option<f32>,

    /// Percentual de redução da BC do ICMS ST
    #[serde(rename = "$unflatten=pRedBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_reducao_bc_st: Option<f32>,

    /// Valor da BC do ICMS ST
    #[serde(rename = "$unflatten=vBCST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_st: Option<f32>,

    /// Alíquota do ICMS ST
    #[serde(rename = "$unflatten=pICMSST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_st: Option<f32>,

    /// Valor do ICMS ST
    #[serde(rename = "$unflatten=vICMSST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_st: Option<f32>,

    /// Percentual do FCP retido por ST
    #[serde(rename = "$unflatten=pFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_st: Option<f32>,

    /// Valor do FCP retido por ST
    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_st: Option<f32>,

    /// Alíquota aplicável de cálculo do crédito
    #[serde(rename = "$unflatten=pCredSN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota_credito_sn: Option<f32>,

    /// Valor do crédito do ICMS
    #[serde(rename = "$unflatten=vCredICMSSN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_credito_icms_sn: Option<f32>,
}
