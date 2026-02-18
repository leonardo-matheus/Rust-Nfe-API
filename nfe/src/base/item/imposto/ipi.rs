//! IPI - Imposto sobre Produtos Industrializados
//!
//! Este módulo implementa as estruturas para representar o IPI na NF-e,
//! conforme definido no layout 4.00 da SEFAZ.
//!
//! ## Códigos de Situação Tributária (CST) do IPI
//!
//! | CST | Descrição |
//! |-----|-----------|
//! | 00 | Entrada com recuperação de crédito |
//! | 01 | Entrada tributada com alíquota zero |
//! | 02 | Entrada isenta |
//! | 03 | Entrada não tributada |
//! | 04 | Entrada imune |
//! | 05 | Entrada com suspensão |
//! | 49 | Outras entradas |
//! | 50 | Saída tributada |
//! | 51 | Saída tributada com alíquota zero |
//! | 52 | Saída isenta |
//! | 53 | Saída não tributada |
//! | 54 | Saída imune |
//! | 55 | Saída com suspensão |
//! | 99 | Outras saídas |

use serde::{Deserialize, Serialize};

/// Container para os grupos de IPI (tag `<IPI>`)
///
/// O IPI é informado através de grupos exclusivos, onde apenas UM grupo
/// deve estar presente por item, dependendo do CST aplicável.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IpiContainer {
    /// Classe de enquadramento do IPI para Cigarros e Bebidas (opcional)
    #[serde(rename = "$unflatten=clEnq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classe_enquadramento: Option<String>,

    /// CNPJ do produtor da mercadoria, quando diferente do emitente
    #[serde(rename = "$unflatten=CNPJProd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnpj_produtor: Option<String>,

    /// Código do selo de controle IPI
    #[serde(rename = "$unflatten=cSelo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codigo_selo: Option<String>,

    /// Quantidade de selo de controle
    #[serde(rename = "$unflatten=qSelo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantidade_selo: Option<u32>,

    /// Código de Enquadramento Legal do IPI
    #[serde(rename = "$unflatten=cEnq")]
    pub codigo_enquadramento: String,

    /// IPI tributado (CST 00, 49, 50, 99)
    #[serde(rename = "IPITrib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipi_trib: Option<IpiTrib>,

    /// IPI não tributado (CST 01, 02, 03, 04, 05, 51, 52, 53, 54, 55)
    #[serde(rename = "IPINT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipi_nt: Option<IpiNt>,
}

/// IPI Tributado (tag `<IPITrib>`)
///
/// Usado para CSTs: 00, 49, 50, 99
///
/// ## Formas de Cálculo
///
/// **Por alíquota (ad valorem):**
/// ```text
/// vIPI = vBC × pIPI / 100
/// ```
///
/// **Por quantidade (específico):**
/// ```text
/// vIPI = qUnid × vUnid
/// ```
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IpiTrib {
    /// Código de Situação Tributária do IPI
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,

    /// Valor da Base de Cálculo do IPI (cálculo por alíquota)
    #[serde(rename = "$unflatten=vBC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc: Option<f32>,

    /// Alíquota do IPI em percentual (cálculo por alíquota)
    #[serde(rename = "$unflatten=pIPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliquota: Option<f32>,

    /// Quantidade total na unidade padrão para tributação (cálculo por quantidade)
    #[serde(rename = "$unflatten=qUnid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantidade_unidade: Option<f32>,

    /// Valor por unidade tributável (cálculo por quantidade)
    #[serde(rename = "$unflatten=vUnid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_unidade: Option<f32>,

    /// Valor do IPI
    #[serde(rename = "$unflatten=vIPI")]
    pub valor: f32,
}

/// IPI Não Tributado (tag `<IPINT>`)
///
/// Usado para CSTs: 01, 02, 03, 04, 05, 51, 52, 53, 54, 55
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IpiNt {
    /// Código de Situação Tributária do IPI
    #[serde(rename = "$unflatten=CST")]
    pub cst: String,
}

impl Default for IpiContainer {
    fn default() -> Self {
        Self {
            classe_enquadramento: None,
            cnpj_produtor: None,
            codigo_selo: None,
            quantidade_selo: None,
            codigo_enquadramento: "999".to_string(),
            ipi_trib: None,
            ipi_nt: None,
        }
    }
}
