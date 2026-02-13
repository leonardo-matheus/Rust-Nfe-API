//! Módulo de PDF para leitura e geração de DANFE
//!
//! - Extração de dados de DANFE em PDF
//! - Geração de DANFE profissional

mod reader;
mod danfe;

pub use reader::*;
pub use danfe::*;
