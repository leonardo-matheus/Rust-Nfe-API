//! Módulo de integração com SEFAZ
//!
//! Permite consultar, emitir, cancelar, inutilizar NF-e/NFC-e no SEFAZ
//!
//! ## Funcionalidades
//!
//! - **Consulta**: Verificar status de NF-e por chave de acesso
//! - **Emissão**: Autorizar NF-e/NFC-e no SEFAZ
//! - **Cancelamento**: Cancelar NF-e autorizada
//! - **Inutilização**: Inutilizar faixas de numeração não utilizadas
//! - **Carta de Correção**: Enviar CC-e para correção de dados
//! - **Manifesto do Destinatário**: Registrar ciência, confirmação ou desconhecimento
//! - **Distribuição DFe**: Consultar documentos emitidos contra o CNPJ

mod consulta;
pub mod distribuicao;
pub mod inutilizacao;
pub mod manifesto;
pub mod webservice;

pub use consulta::*;
pub use distribuicao::*;
pub use inutilizacao::*;
pub use manifesto::*;
pub use webservice::*;
