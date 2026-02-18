//! Totalização dos produtos e serviços
//!
//! Este módulo contém as estruturas para totalização dos valores da NF-e,
//! incluindo todos os impostos e valores adicionais conforme layout 4.00.

use super::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// Totalização da nota fiscal
///
/// Contém todos os totais calculados da NF-e, incluindo:
/// - Totais de ICMS (próprio, ST, desonerado, FCP)
/// - Totais de IPI
/// - Totais de PIS/COFINS
/// - Totais de Importação
/// - Totais de DIFAL (partilha interestadual)
/// - Totais de serviços (ISSQN)
#[derive(Debug, PartialEq, Clone)]
pub struct Totalizacao {
    /// Base de cálculo do ICMS
    pub valor_base_calculo: f32,
    /// Valor total do ICMS
    pub valor_icms: f32,
    /// Valor total do ICMS desonerado
    pub valor_icms_desonerado: f32,
    /// Valor total do FCP (Fundo de Combate à Pobreza)
    pub valor_fcp: f32,
    /// Base de cálculo do ICMS ST
    pub valor_base_calculo_st: f32,
    /// Valor total do ICMS ST
    pub valor_icms_st: f32,
    /// Valor total do FCP retido por ST
    pub valor_fcp_st: f32,
    /// Valor total do FCP retido anteriormente por ST
    pub valor_fcp_st_retido: f32,
    /// Valor total dos produtos e serviços
    pub valor_produtos: f32,
    /// Valor total do frete
    pub valor_frete: f32,
    /// Valor total do seguro
    pub valor_seguro: f32,
    /// Valor total do desconto
    pub valor_desconto: f32,
    /// Outras despesas acessórias
    pub valor_outros: f32,
    /// Valor total do IPI
    pub valor_ipi: f32,
    /// Valor total do IPI devolvido
    pub valor_ipi_devolvido: f32,
    /// Valor total do Imposto de Importação
    pub valor_ii: f32,
    /// Valor total do PIS
    pub valor_pis: f32,
    /// Valor total do COFINS
    pub valor_cofins: f32,
    /// Valor total da nota
    pub valor_total: f32,
    /// Valor aproximado total de tributos (Lei 12.741/2012)
    pub valor_aproximado_tributos: f32,

    // Campos de DIFAL (partilha interestadual EC 87/2015)
    /// Valor total do FCP UF Destino
    pub valor_fcp_uf_dest: f32,
    /// Valor total do ICMS UF Destino
    pub valor_icms_uf_dest: f32,
    /// Valor total do ICMS UF Remetente
    pub valor_icms_uf_remet: f32,
}

impl FromStr for Totalizacao {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Totalizacao {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar a totalização")
    }
}

impl Default for Totalizacao {
    fn default() -> Self {
        Self {
            valor_base_calculo: 0.0,
            valor_icms: 0.0,
            valor_icms_desonerado: 0.0,
            valor_fcp: 0.0,
            valor_base_calculo_st: 0.0,
            valor_icms_st: 0.0,
            valor_fcp_st: 0.0,
            valor_fcp_st_retido: 0.0,
            valor_produtos: 0.0,
            valor_frete: 0.0,
            valor_seguro: 0.0,
            valor_desconto: 0.0,
            valor_outros: 0.0,
            valor_ipi: 0.0,
            valor_ipi_devolvido: 0.0,
            valor_ii: 0.0,
            valor_pis: 0.0,
            valor_cofins: 0.0,
            valor_total: 0.0,
            valor_aproximado_tributos: 0.0,
            valor_fcp_uf_dest: 0.0,
            valor_icms_uf_dest: 0.0,
            valor_icms_uf_remet: 0.0,
        }
    }
}

impl Serialize for Totalizacao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let icms = IcmsTot {
            valor_base_calculo: self.valor_base_calculo,
            valor_icms: self.valor_icms,
            valor_icms_desonerado: self.valor_icms_desonerado,
            valor_fcp: if self.valor_fcp > 0.0 {
                Some(self.valor_fcp)
            } else {
                None
            },
            valor_base_calculo_st: self.valor_base_calculo_st,
            valor_icms_st: self.valor_icms_st,
            valor_fcp_st: if self.valor_fcp_st > 0.0 {
                Some(self.valor_fcp_st)
            } else {
                None
            },
            valor_fcp_st_retido: if self.valor_fcp_st_retido > 0.0 {
                Some(self.valor_fcp_st_retido)
            } else {
                None
            },
            valor_produtos: self.valor_produtos,
            valor_frete: self.valor_frete,
            valor_seguro: self.valor_seguro,
            valor_desconto: self.valor_desconto,
            valor_outros: self.valor_outros,
            valor_ipi: self.valor_ipi,
            valor_ipi_devolvido: if self.valor_ipi_devolvido > 0.0 {
                Some(self.valor_ipi_devolvido)
            } else {
                None
            },
            valor_ii: self.valor_ii,
            valor_pis: self.valor_pis,
            valor_cofins: self.valor_cofins,
            valor_total: self.valor_total,
            valor_aproximado_tributos: self.valor_aproximado_tributos,
            valor_fcp_uf_dest: if self.valor_fcp_uf_dest > 0.0 {
                Some(self.valor_fcp_uf_dest)
            } else {
                None
            },
            valor_icms_uf_dest: if self.valor_icms_uf_dest > 0.0 {
                Some(self.valor_icms_uf_dest)
            } else {
                None
            },
            valor_icms_uf_remet: if self.valor_icms_uf_remet > 0.0 {
                Some(self.valor_icms_uf_remet)
            } else {
                None
            },
        };

        let total = TotalContainer { icms };

        total.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Totalizacao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = TotalContainer::deserialize(deserializer)?;
        Ok(Totalizacao {
            valor_base_calculo: helper.icms.valor_base_calculo,
            valor_icms: helper.icms.valor_icms,
            valor_icms_desonerado: helper.icms.valor_icms_desonerado,
            valor_fcp: helper.icms.valor_fcp.unwrap_or(0.0),
            valor_base_calculo_st: helper.icms.valor_base_calculo_st,
            valor_icms_st: helper.icms.valor_icms_st,
            valor_fcp_st: helper.icms.valor_fcp_st.unwrap_or(0.0),
            valor_fcp_st_retido: helper.icms.valor_fcp_st_retido.unwrap_or(0.0),
            valor_produtos: helper.icms.valor_produtos,
            valor_frete: helper.icms.valor_frete,
            valor_seguro: helper.icms.valor_seguro,
            valor_desconto: helper.icms.valor_desconto,
            valor_outros: helper.icms.valor_outros,
            valor_ipi: helper.icms.valor_ipi,
            valor_ipi_devolvido: helper.icms.valor_ipi_devolvido.unwrap_or(0.0),
            valor_ii: helper.icms.valor_ii,
            valor_pis: helper.icms.valor_pis,
            valor_cofins: helper.icms.valor_cofins,
            valor_total: helper.icms.valor_total,
            valor_aproximado_tributos: helper.icms.valor_aproximado_tributos,
            valor_fcp_uf_dest: helper.icms.valor_fcp_uf_dest.unwrap_or(0.0),
            valor_icms_uf_dest: helper.icms.valor_icms_uf_dest.unwrap_or(0.0),
            valor_icms_uf_remet: helper.icms.valor_icms_uf_remet.unwrap_or(0.0),
        })
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "total")]
struct TotalContainer {
    #[serde(rename = "ICMSTot")]
    icms: IcmsTot,
}

#[derive(Deserialize, Serialize)]
struct IcmsTot {
    #[serde(rename = "$unflatten=vBC")]
    valor_base_calculo: f32,

    #[serde(rename = "$unflatten=vICMS")]
    valor_icms: f32,

    #[serde(rename = "$unflatten=vICMSDeson")]
    #[serde(default)]
    valor_icms_desonerado: f32,

    #[serde(rename = "$unflatten=vFCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    valor_fcp: Option<f32>,

    #[serde(rename = "$unflatten=vBCST")]
    #[serde(default)]
    valor_base_calculo_st: f32,

    #[serde(rename = "$unflatten=vST")]
    #[serde(default)]
    valor_icms_st: f32,

    #[serde(rename = "$unflatten=vFCPST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    valor_fcp_st: Option<f32>,

    #[serde(rename = "$unflatten=vFCPSTRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    valor_fcp_st_retido: Option<f32>,

    #[serde(rename = "$unflatten=vProd")]
    valor_produtos: f32,

    #[serde(rename = "$unflatten=vFrete")]
    valor_frete: f32,

    #[serde(rename = "$unflatten=vSeg")]
    valor_seguro: f32,

    #[serde(rename = "$unflatten=vDesc")]
    valor_desconto: f32,

    #[serde(rename = "$unflatten=vOutro")]
    valor_outros: f32,

    #[serde(rename = "$unflatten=vII")]
    #[serde(default)]
    valor_ii: f32,

    #[serde(rename = "$unflatten=vIPI")]
    #[serde(default)]
    valor_ipi: f32,

    #[serde(rename = "$unflatten=vIPIDevol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    valor_ipi_devolvido: Option<f32>,

    #[serde(rename = "$unflatten=vPIS")]
    valor_pis: f32,

    #[serde(rename = "$unflatten=vCOFINS")]
    valor_cofins: f32,

    #[serde(rename = "$unflatten=vNF")]
    valor_total: f32,

    #[serde(rename = "$unflatten=vTotTrib")]
    valor_aproximado_tributos: f32,

    // Campos de DIFAL
    #[serde(rename = "$unflatten=vFCPUFDest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    valor_fcp_uf_dest: Option<f32>,

    #[serde(rename = "$unflatten=vICMSUFDest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    valor_icms_uf_dest: Option<f32>,

    #[serde(rename = "$unflatten=vICMSUFRemet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    valor_icms_uf_remet: Option<f32>,
}
