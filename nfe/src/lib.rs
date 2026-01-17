//! # NFe - Biblioteca Rust para Nota Fiscal Eletrônica
//!
//! Esta biblioteca fornece estruturas e utilitários para parsing e serialização
//! de arquivos XML de Nota Fiscal Eletrônica (NF-e) brasileira.
//!
//! ## Funcionalidades
//!
//! - Parse de XML de NF-e (Layout 4.00)
//! - Serialização de estruturas para XML
//! - Suporte a NF-e (modelo 55) e NFC-e (modelo 65)
//! - Validação de campos obrigatórios
//!
//! ## Exemplo de uso
//!
//! ```rust,ignore
//! use std::fs::File;
//! use std::convert::TryFrom;
//! use nfe::Nfe;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let file = File::open("nota.xml")?;
//!     let nfe = Nfe::try_from(file)?;
//!     
//!     println!("Chave de acesso: {}", nfe.chave_acesso);
//!     println!("Emitente: {:?}", nfe.emit.razao_social);
//!     
//!     Ok(())
//! }
//! ```

pub mod base;
pub mod modelos;

#[cfg(test)]
mod tests;

// Re-exportação dos tipos principais para facilitar o uso
pub use base::dest::{Destinatario, IndicadorContribuicaoIe};
pub use base::emit::Emitente;
pub use base::endereco::Endereco;
pub use base::ide::{
    ComposicaoChaveAcesso, DestinoOperacao, Emissao, FinalidadeEmissao, FormatoImpressaoDanfe,
    Identificacao, ModeloDocumentoFiscal, Operacao, TipoAmbiente, TipoConsumidor, TipoEmissao,
    TipoIntermediador, TipoOperacao, TipoPresencaComprador, TipoProcessoEmissao,
};
pub use base::item::{Imposto, Item, Produto};
pub use base::totais::Totalizacao;
pub use base::transporte::{ModalidadeFrete, Transporte};
pub use base::Error;
pub use base::Nfe;
pub use base::VersaoLayout;
