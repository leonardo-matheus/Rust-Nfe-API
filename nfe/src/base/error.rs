//! Erros de parse da NF-e

use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]

pub enum Error {
    #[display(fmt = "Erro de IO: {}", _0)]
    Io(std::io::Error),
    #[display(fmt = "Erro de XML: {}", _0)]
    Serde(quick_xml::de::DeError),
}
