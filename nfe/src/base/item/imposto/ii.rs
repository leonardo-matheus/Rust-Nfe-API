//! II - Imposto de Importação
//!
//! Este módulo implementa a estrutura para representar o Imposto de Importação na NF-e.
//!
//! ## Quando Utilizar
//!
//! O grupo II deve ser informado quando a operação for de importação,
//! independente se há ou não incidência do imposto.
//!
//! ## Cálculo
//!
//! ```text
//! vII = vBC × (alíquota II / 100)
//! ```
//!
//! O valor da base de cálculo é geralmente composto por:
//! - Valor aduaneiro (CIF: custo + seguro + frete internacional)
//! - Despesas aduaneiras

use serde::{Deserialize, Serialize};

/// Imposto de Importação (tag `<II>`)
///
/// Obrigatório para operações de importação.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpostoImportacao {
    /// Valor da Base de Cálculo do II (tag `<vBC>`)
    /// Normalmente: valor aduaneiro + despesas aduaneiras
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,

    /// Valor das despesas aduaneiras (tag `<vDespAdu>`)
    #[serde(rename = "$unflatten=vDespAdu")]
    pub valor_despesas_aduaneiras: f32,

    /// Valor do Imposto de Importação (tag `<vII>`)
    #[serde(rename = "$unflatten=vII")]
    pub valor: f32,

    /// Valor do Imposto sobre Operações Financeiras (tag `<vIOF>`)
    #[serde(rename = "$unflatten=vIOF")]
    pub valor_iof: f32,
}

impl Default for ImpostoImportacao {
    fn default() -> Self {
        Self {
            valor_bc: 0.0,
            valor_despesas_aduaneiras: 0.0,
            valor: 0.0,
            valor_iof: 0.0,
        }
    }
}
