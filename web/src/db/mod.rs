//! Módulo de banco de dados
//!
//! Suporte para PostgreSQL e MySQL para armazenamento de NF-e

pub mod postgres;
pub mod mysql;
pub mod models;

pub use models::*;
