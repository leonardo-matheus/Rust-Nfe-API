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
///
/// ## Grupos Implementados
///
/// - **ICMS00**: Tributação integral (CST 00)
/// - **ICMS10**: Tributação com ST (CST 10)
/// - **ICMS20**: Com redução de BC (CST 20)
/// - **ICMSSN101**: Simples Nacional com crédito (CSOSN 101)
/// - **ICMSSN102**: Simples Nacional sem crédito (CSOSN 102/103/300/400)
///
/// ## Nota sobre Implementação
///
/// Esta biblioteca implementa os grupos mais comuns. Para suporte completo
/// a todos os CSTs (30, 40, 41, 50, 51, 60, 70, 90), será necessário
/// adicionar as estruturas correspondentes.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

    /// ICMS Simples Nacional CSOSN 101 - Tributada com permissão de crédito
    #[serde(rename = "ICMSSN101")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn101: Option<IcmsSn101>,

    /// ICMS Simples Nacional CSOSN 102/103/300/400
    /// Tributada sem permissão de crédito / Isenção / Imune / Não tributada
    #[serde(rename = "ICMSSN102")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_sn102: Option<IcmsSn102>,
}

/// ICMS CST 00 - Tributação Integral (tag `<ICMS00>`)
///
/// Usado quando o produto é tributado integralmente pelo ICMS,
/// sem redução de base de cálculo ou substituição tributária.
///
/// ## Cálculo
///
/// ```text
/// vICMS = vBC × pICMS / 100
/// ```
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Icms00 {
    /// Origem da mercadoria (tag `<orig>`)
    /// 0=Nacional, 1=Estrangeira importação direta, 2=Estrangeira adq. mercado interno
    /// 3 a 8: Nacionais com conteúdo de importação específico
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação Tributária (tag `<CST>`)
    /// Para ICMS00, sempre será "00"
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Modalidade de determinação da BC do ICMS (tag `<modBC>`)
    /// 0=Margem Valor Agregado (%), 1=Pauta (Valor), 2=Preço Tabelado Max.,
    /// 3=Valor da operação
    #[serde(rename = "$unflatten=modBC")]
    pub modalidade_bc: u8,

    /// Valor da Base de Cálculo do ICMS (tag `<vBC>`)
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,

    /// Alíquota do ICMS em percentual (tag `<pICMS>`)
    /// Ex: 18.00 para 18%
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,

    /// Valor do ICMS (tag `<vICMS>`)
    /// Calculado: vBC × pICMS / 100
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,
}

/// ICMS CST 10 - Tributação com ICMS por Substituição Tributária (tag `<ICMS10>`)
///
/// Usado quando há tributação normal do ICMS E cobrança antecipada do ICMS
/// por substituição tributária (ST) para as operações subsequentes.
///
/// ## Nota
///
/// Esta estrutura representa apenas a parte da operação própria.
/// Os campos de ST (vBCST, pICMSST, vICMSST) devem ser adicionados
/// conforme necessidade.
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
}

/// ICMS CST 20 - Tributação com Redução de Base de Cálculo (tag `<ICMS20>`)
///
/// Usado quando há benefício fiscal que reduz a base de cálculo do ICMS.
/// A redução é aplicada ANTES do cálculo do imposto.
///
/// ## Cálculo
///
/// ```text
/// vBC_reduzida = vBC_original × (1 - pRedBC/100)
/// vICMS = vBC_reduzida × pICMS / 100
/// ```
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

    /// Percentual de redução da BC (tag `<pRedBC>`)
    /// Ex: 33.33 para redução de 33,33%
    #[serde(rename = "$unflatten=pRedBC")]
    pub percentual_reducao_bc: f32,

    /// Valor da BC do ICMS JÁ REDUZIDA
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,

    /// Alíquota do ICMS em percentual
    #[serde(rename = "$unflatten=pICMS")]
    pub aliquota: f32,

    /// Valor do ICMS calculado sobre a BC reduzida
    #[serde(rename = "$unflatten=vICMS")]
    pub valor: f32,
}

/// ICMS Simples Nacional CSOSN 101 (tag `<ICMSSN101>`)
///
/// Usado por empresas do Simples Nacional quando a operação
/// permite aproveitamento de crédito de ICMS pelo destinatário.
///
/// ## Quando Usar
///
/// - Venda para contribuinte do ICMS (não consumidor final)
/// - Operação tributada pelo Simples Nacional
/// - Empresa dentro do sublimite estadual para crédito
///
/// ## Cálculo do Crédito
///
/// O crédito é calculado sobre o valor do produto usando a alíquota
/// efetiva do ICMS no Simples Nacional (varia conforme faturamento).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn101 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação da Operação no Simples Nacional
    /// Para esta estrutura, sempre "101"
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,

    /// Alíquota aplicável de cálculo do crédito (tag `<pCredSN>`)
    /// Corresponde à alíquota efetiva do ICMS no Simples Nacional
    #[serde(rename = "$unflatten=pCredSN")]
    pub aliquota_credito_sn: f32,

    /// Valor do crédito do ICMS permitido (tag `<vCredICMSSN>`)
    /// Calculado: valor_produtos × pCredSN / 100
    #[serde(rename = "$unflatten=vCredICMSSN")]
    pub valor_credito_icms_sn: f32,
}

/// ICMS Simples Nacional CSOSN 102, 103, 300, 400 (tag `<ICMSSN102>`)
///
/// Usado por empresas do Simples Nacional em operações que NÃO
/// permitem aproveitamento de crédito pelo destinatário.
///
/// ## CSOSNs Cobertos
///
/// | CSOSN | Descrição |
/// |-------|-----------|
/// | 102 | Tributada sem permissão de crédito |
/// | 103 | Isenção do ICMS para faixa de receita bruta |
/// | 300 | Imune |
/// | 400 | Não tributada pelo Simples Nacional |
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsSn102 {
    /// Origem da mercadoria
    #[serde(rename = "$unflatten=orig")]
    pub origem: u8,

    /// Código de Situação da Operação no Simples Nacional
    /// Pode ser: "102", "103", "300" ou "400"
    #[serde(rename = "$unflatten=CSOSN")]
    pub csosn: String,
}
