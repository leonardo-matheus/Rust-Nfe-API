//! Módulo de Certificado Digital
//!
//! Suporta certificados A1 (arquivo .pfx/.p12) e A3 (token/smartcard)

mod a1;
mod assinatura;

pub use a1::*;
pub use assinatura::*;
