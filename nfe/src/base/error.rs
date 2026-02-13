//! Erros de parse e processamento da NF-e
//!
//! Este módulo define os tipos de erro que podem ocorrer durante
//! o parsing e processamento de arquivos XML de NF-e.
//!
//! ## Tipos de Erro
//!
//! - **Io**: Erros de leitura/escrita de arquivo
//! - **Serde**: Erros de deserialização XML (estrutura inválida, campos faltando, etc.)
//!
//! ## Exemplo de Tratamento
//!
//! ```rust,ignore
//! use nfe::{Nfe, Error};
//!
//! fn processar_nfe(xml: &str) -> Result<(), Error> {
//!     match xml.parse::<Nfe>() {
//!         Ok(nfe) => println!("NF-e processada: {}", nfe.chave_acesso),
//!         Err(Error::Io(e)) => eprintln!("Erro de arquivo: {}", e),
//!         Err(Error::Serde(e)) => eprintln!("XML inválido: {}", e),
//!     }
//!     Ok(())
//! }
//! ```

use derive_more::{Display, Error, From};

/// Tipo de erro retornado pelas operações de parsing da NF-e
///
/// Implementa conversão automática (From) dos tipos de erro originais,
/// permitindo uso do operador `?` para propagação de erros.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    /// Erro de entrada/saída (leitura de arquivo, permissões, etc.)
    /// Ocorre ao tentar abrir ou ler um arquivo XML
    #[display(fmt = "Erro de IO: {}", _0)]
    Io(std::io::Error),

    /// Erro de deserialização XML
    /// Ocorre quando o XML não está no formato esperado pela SEFAZ
    /// Exemplos: tags obrigatórias faltando, tipos inválidos, estrutura incorreta
    #[display(fmt = "Erro de XML: {}", _0)]
    Serde(quick_xml::de::DeError),
}
