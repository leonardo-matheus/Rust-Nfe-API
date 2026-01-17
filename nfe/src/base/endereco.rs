//! Endereço da Nota Fiscal Eletrônica
//!
//! Este módulo contém a estrutura para representar endereços
//! do emitente e destinatário da NF-e.

use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Endereço completo
///
/// Representa o endereço do emitente ou destinatário da NF-e.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename = "Endereco")]
pub struct Endereco {
    /// Logradouro (rua, avenida, etc.)
    #[serde(rename = "xLgr")]
    pub logradouro: String,
    /// Número do endereço
    #[serde(rename = "nro")]
    pub numero: String,
    /// Complemento do endereço
    #[serde(rename = "xCpl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub complemento: Option<String>,
    /// Bairro
    #[serde(rename = "xBairro")]
    pub bairro: String,
    /// Código do município (IBGE)
    #[serde(rename = "cMun")]
    pub codigo_municipio: u32,
    /// Nome do município
    #[serde(rename = "xMun")]
    pub nome_municipio: String,
    /// Sigla da UF
    #[serde(rename = "UF")]
    pub sigla_uf: String,
    /// CEP (8 dígitos)
    #[serde(rename = "CEP")]
    pub cep: String,
    /// Código do país (1058 para Brasil)
    #[serde(rename = "cPais")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub codigo_pais: Option<String>,
    /// Nome do país
    #[serde(rename = "xPais")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nome_pais: Option<String>,
    /// Telefone
    #[serde(rename = "fone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub telefone: Option<String>,
}

impl FromStr for Endereco {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Endereco {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o endereço")
    }
}

impl Default for Endereco {
    fn default() -> Self {
        Self {
            logradouro: String::new(),
            numero: String::new(),
            complemento: None,
            bairro: String::new(),
            codigo_municipio: 0,
            nome_municipio: String::new(),
            sigla_uf: String::new(),
            cep: String::new(),
            codigo_pais: Some("1058".to_string()),
            nome_pais: Some("BRASIL".to_string()),
            telefone: None,
        }
    }
}
