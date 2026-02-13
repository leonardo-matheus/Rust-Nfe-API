//! Módulo de consulta ao SEFAZ
//!
//! Permite consultar, emitir, cancelar NF-e/NFC-e no SEFAZ

mod consulta;
pub mod webservice;

pub use consulta::*;
pub use webservice::*;
