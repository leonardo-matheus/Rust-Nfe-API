//! ICMS para UF de Destino (DIFAL)
//!
//! Este módulo implementa a estrutura para representar o ICMS devido
//! para a UF de destino nas operações interestaduais destinadas a
//! consumidor final não contribuinte do ICMS.
//!
//! ## Quando Utilizar
//!
//! O grupo ICMSUFDest deve ser informado nas operações:
//! - Interestaduais (UF origem ≠ UF destino)
//! - Destinadas a consumidor final não contribuinte
//!
//! ## Cálculo do DIFAL (EC 87/2015)
//!
//! ```text
//! Base ICMS UF Destino = (Valor Produto + Frete + Seguro + Outros - Desconto) /
//!                        (1 - (pICMSUFDest + pFCPUFDest) / 100)
//!
//! ICMS Interestadual = Base × pICMSInter / 100
//! Diferença de Alíquota = Base × (pICMSUFDest - pICMSInter) / 100
//!
//! vFCPUFDest = Base × pFCPUFDest / 100
//! vICMSUFDest = Diferença de Alíquota (100% para UF destino desde 2019)
//! vICMSUFRemet = 0 (0% para UF remetente desde 2019)
//! ```

use serde::{Deserialize, Serialize};

/// ICMS para UF de Destino - DIFAL (tag `<ICMSUFDest>`)
///
/// Grupo de informações do ICMS Interestadual nas operações
/// destinadas a consumidor final não contribuinte.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IcmsUfDest {
    /// Valor da Base de Cálculo do ICMS na UF de destino (tag `<vBCUFDest>`)
    #[serde(rename = "$unflatten=vBCUFDest")]
    pub valor_bc_uf_dest: f32,

    /// Valor da Base de Cálculo do FCP na UF de destino (tag `<vBCFCPUFDest>`)
    #[serde(rename = "$unflatten=vBCFCPUFDest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_bc_fcp_uf_dest: Option<f32>,

    /// Percentual do ICMS relativo ao Fundo de Combate à Pobreza na UF de destino (tag `<pFCPUFDest>`)
    #[serde(rename = "$unflatten=pFCPUFDest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentual_fcp_uf_dest: Option<f32>,

    /// Alíquota interna da UF de destino (tag `<pICMSUFDest>`)
    #[serde(rename = "$unflatten=pICMSUFDest")]
    pub aliquota_uf_dest: f32,

    /// Alíquota interestadual das UFs envolvidas (tag `<pICMSInter>`)
    /// 4% para produtos importados (Resolução 13/2012)
    /// 7% ou 12% conforme UFs de origem e destino
    #[serde(rename = "$unflatten=pICMSInter")]
    pub aliquota_interestadual: f32,

    /// Percentual provisório de partilha do ICMS Interestadual (tag `<pICMSInterPart>`)
    /// Desde 2019: 100% para UF destino
    #[serde(rename = "$unflatten=pICMSInterPart")]
    pub percentual_partilha: f32,

    /// Valor do ICMS relativo ao FCP para a UF de destino (tag `<vFCPUFDest>`)
    #[serde(rename = "$unflatten=vFCPUFDest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_fcp_uf_dest: Option<f32>,

    /// Valor do ICMS Interestadual para a UF de destino (tag `<vICMSUFDest>`)
    #[serde(rename = "$unflatten=vICMSUFDest")]
    pub valor_icms_uf_dest: f32,

    /// Valor do ICMS Interestadual para a UF do remetente (tag `<vICMSUFRemet>`)
    /// Desde 2019: sempre 0 (zero)
    #[serde(rename = "$unflatten=vICMSUFRemet")]
    pub valor_icms_uf_remet: f32,
}

impl Default for IcmsUfDest {
    fn default() -> Self {
        Self {
            valor_bc_uf_dest: 0.0,
            valor_bc_fcp_uf_dest: None,
            percentual_fcp_uf_dest: None,
            aliquota_uf_dest: 0.0,
            aliquota_interestadual: 0.0,
            percentual_partilha: 100.0, // 100% para UF destino desde 2019
            valor_fcp_uf_dest: None,
            valor_icms_uf_dest: 0.0,
            valor_icms_uf_remet: 0.0,
        }
    }
}
