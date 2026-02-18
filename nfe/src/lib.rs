//! # NFe Parser - Biblioteca Rust para Documentos Fiscais Eletrônicos
//!
//! Biblioteca de alto desempenho para parsing, serialização e manipulação de
//! documentos fiscais eletrônicos brasileiros: NF-e, NFC-e e NFS-e.
//!
//! ## Funcionalidades
//!
//! - **NF-e (Modelo 55)**: Nota Fiscal Eletrônica para operações B2B
//! - **NFC-e (Modelo 65)**: Nota Fiscal de Consumidor Eletrônica para varejo
//! - **NFS-e**: Nota Fiscal de Serviços Eletrônica (padrão ABRASF)
//! - **Impostos completos**: ICMS (todos os CSTs), IPI, PIS, COFINS, ISS, II, DIFAL
//! - **Municípios**: Tabela IBGE com alíquotas de ISS (Matão, Araraquara e +)
//! - **Alíquotas**: ICMS por UF, ISS por município, DIFAL interestadual
//!
//! ## Exemplo de uso
//!
//! ```rust,ignore
//! use std::fs::File;
//! use std::convert::TryFrom;
//! use nfe_parser::{Nfe, NfeBuilder, ItemBuilder};
//!
//! // Lendo uma NF-e existente
//! let file = File::open("nota.xml")?;
//! let nfe = Nfe::try_from(file)?;
//! println!("Chave: {}", nfe.chave_acesso);
//! println!("Total: R$ {:.2}", nfe.totais.valor_total);
//!
//! // Criando uma nova NF-e
//! let nfe = NfeBuilder::new()
//!     .emitente("12345678000195", "Empresa Ltda", "SP")
//!     .destinatario("98765432000123", "Cliente SA")
//!     .item(ItemBuilder::new("001", "Produto X", "12345678", "5102")
//!         .quantidade(10.0)
//!         .valor_unitario(99.90)
//!         .build())
//!     .build()?;
//! ```
//!
//! ## Módulos
//!
//! - [`base`]: Estruturas fundamentais da NF-e
//! - [`builder`]: API fluente para construção de NF-e
//! - [`modelos`]: Modelos específicos de documentos

pub mod base;
pub mod builder;
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
pub use base::item::imposto::*;
pub use base::municipios::{
    Municipio, ConfiguracaoUf, AliquotaIss, SistemaNfse,
    matao, araraquara, sao_paulo_uf, aliquotas_iss_matao, aliquotas_iss_araraquara,
    aliquotas_icms_por_uf, buscar_municipio_por_codigo, buscar_uf, calcular_aliquota_interestadual,
};
pub use base::nfce::{QrCodeNfce, ConfiguracaoCsc, ValidadorNfce, FormaPagamentoNfce, ModoEmissaoNfce};
pub use base::nfse::{
    Nfse, IdentificacaoNfse, PrestadorServico, TomadorServico, ServicoNfse, ValoresNfse,
    NaturezaOperacaoNfse, RegimeEspecialNfse, StatusNfse, Rps, LoteRps,
    calcular_valores_nfse,
};
pub use base::totais::Totalizacao;
pub use base::transporte::{ModalidadeFrete, Transporte};
pub use base::Error;
pub use base::Nfe;
pub use base::VersaoLayout;
pub use builder::{NfeBuilder, ItemBuilder};
