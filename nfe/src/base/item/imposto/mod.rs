//! Impostos da Nota Fiscal Eletrônica
//!
//! Este módulo contém as estruturas para representar todos os impostos
//! incidentes sobre os produtos da NF-e, conforme layout 4.00 da SEFAZ.
//!
//! ## Impostos Implementados
//!
//! | Imposto | Descrição |
//! |---------|-----------|
//! | ICMS | Imposto sobre Circulação de Mercadorias e Serviços |
//! | IPI | Imposto sobre Produtos Industrializados |
//! | PIS | Programa de Integração Social |
//! | COFINS | Contribuição para Financiamento da Seguridade Social |
//! | PIS ST | PIS Substituição Tributária |
//! | COFINS ST | COFINS Substituição Tributária |
//! | II | Imposto de Importação |
//! | ISSQN | Imposto sobre Serviços de Qualquer Natureza |
//! | ICMSUFDest | ICMS para UF de Destino (DIFAL - EC 87/2015) |

mod cofins;
mod cofins_st;
mod icms;
mod icms_uf_dest;
mod ii;
mod ipi;
mod issqn;
mod pis;
mod pis_st;

pub use cofins::*;
pub use cofins_st::*;
pub use icms::*;
pub use icms_uf_dest::*;
pub use ii::*;
pub use ipi::*;
pub use issqn::*;
pub use pis::*;
pub use pis_st::*;

use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Impostos do item da nota fiscal
///
/// Contém todos os tributos que podem incidir sobre um produto/serviço na NF-e.
/// Apenas os grupos aplicáveis à operação devem ser preenchidos.
///
/// ## Exemplo de Uso
///
/// ```rust
/// use nfe_parser::Imposto;
///
/// let imposto = Imposto {
///     icms: Some(IcmsContainer::default()),
///     pis: Some(PisContainer::default()),
///     cofins: Some(CofinsContainer::default()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename = "imposto")]
pub struct Imposto {
    /// Valor aproximado total de tributos (Lei da Transparência - Lei 12.741/2012)
    #[serde(rename = "$unflatten=vTotTrib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_aproximado_tributos: Option<f32>,

    /// ICMS - Imposto sobre Circulação de Mercadorias e Serviços
    #[serde(rename = "ICMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms: Option<IcmsContainer>,

    /// IPI - Imposto sobre Produtos Industrializados
    #[serde(rename = "IPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipi: Option<IpiContainer>,

    /// II - Imposto de Importação (obrigatório para operações de importação)
    #[serde(rename = "II")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ii: Option<ImpostoImportacao>,

    /// PIS - Programa de Integração Social
    #[serde(rename = "PIS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pis: Option<PisContainer>,

    /// PIS ST - PIS Substituição Tributária
    #[serde(rename = "PISST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pis_st: Option<PisSt>,

    /// COFINS - Contribuição para Financiamento da Seguridade Social
    #[serde(rename = "COFINS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cofins: Option<CofinsContainer>,

    /// COFINS ST - COFINS Substituição Tributária
    #[serde(rename = "COFINSST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cofins_st: Option<CofinsSt>,

    /// ISSQN - Imposto sobre Serviços (substitui o ICMS para serviços)
    #[serde(rename = "ISSQN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issqn: Option<Issqn>,

    /// ICMS para UF de Destino - DIFAL (operações interestaduais para consumidor final)
    #[serde(rename = "ICMSUFDest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icms_uf_dest: Option<IcmsUfDest>,
}

impl FromStr for Imposto {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Imposto {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o imposto")
    }
}

impl Default for Imposto {
    fn default() -> Self {
        Self {
            valor_aproximado_tributos: None,
            icms: None,
            ipi: None,
            ii: None,
            pis: None,
            pis_st: None,
            cofins: None,
            cofins_st: None,
            issqn: None,
            icms_uf_dest: None,
        }
    }
}
