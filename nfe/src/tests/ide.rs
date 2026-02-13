//! Testes de identificação da NF-e
//!
//! Nota: Os testes de parsing direto do grupo <ide> foram removidos
//! porque a estrutura Identificacao usa deserialização customizada
//! que depende de um container intermediário (IdeContainer).
//!
//! Para testar o parsing de identificação, é necessário testar
//! a NF-e completa através do módulo infnfe.
//!
//! Os testes aqui focam em validar os enums e tipos auxiliares.

use crate::base::ide::{
    DestinoOperacao, FinalidadeEmissao, FormatoImpressaoDanfe,
    ModeloDocumentoFiscal, TipoAmbiente, TipoConsumidor,
    TipoEmissao, TipoOperacao, TipoPresencaComprador, TipoIntermediador,
};

/// Testa que os valores dos enums estão corretos conforme SEFAZ
#[test]
fn valores_modelo_documento_fiscal() {
    assert_eq!(55, ModeloDocumentoFiscal::Nfe as u8);
    assert_eq!(65, ModeloDocumentoFiscal::Nfce as u8);
}

/// Testa os valores do enum TipoAmbiente
#[test]
fn valores_tipo_ambiente() {
    assert_eq!(1, TipoAmbiente::Producao as u8);
    assert_eq!(2, TipoAmbiente::Homologacao as u8);
}

/// Testa os valores do enum TipoOperacao
#[test]
fn valores_tipo_operacao() {
    assert_eq!(0, TipoOperacao::Entrada as u8);
    assert_eq!(1, TipoOperacao::Saida as u8);
}

/// Testa os valores do enum DestinoOperacao
#[test]
fn valores_destino_operacao() {
    assert_eq!(1, DestinoOperacao::Interna as u8);
    assert_eq!(2, DestinoOperacao::Interestadual as u8);
    assert_eq!(3, DestinoOperacao::ComExterior as u8);
}

/// Testa os valores do enum TipoEmissao
#[test]
fn valores_tipo_emissao() {
    assert_eq!(1, TipoEmissao::Normal as u8);
    assert_eq!(2, TipoEmissao::ContigenciaFsIa as u8);
    assert_eq!(3, TipoEmissao::ContingenciaScan as u8);
    assert_eq!(4, TipoEmissao::ContigenciaEpec as u8);
    assert_eq!(5, TipoEmissao::ContigenciaFsDa as u8);
    assert_eq!(6, TipoEmissao::ContigenciaSvcAn as u8);
    assert_eq!(7, TipoEmissao::ContigenciaSvcRs as u8);
    assert_eq!(9, TipoEmissao::ContigenciaOfflineNfce as u8);
}

/// Testa os valores do enum FinalidadeEmissao
#[test]
fn valores_finalidade_emissao() {
    assert_eq!(1, FinalidadeEmissao::Normal as u8);
    assert_eq!(2, FinalidadeEmissao::Complementar as u8);
    assert_eq!(3, FinalidadeEmissao::Ajuste as u8);
    assert_eq!(4, FinalidadeEmissao::Devolucao as u8);
}

/// Testa os valores do enum FormatoImpressaoDanfe
#[test]
fn valores_formato_impressao_danfe() {
    assert_eq!(0, FormatoImpressaoDanfe::SemGeracao as u8);
    assert_eq!(1, FormatoImpressaoDanfe::NormalRetrato as u8);
    assert_eq!(2, FormatoImpressaoDanfe::NormalPaisagem as u8);
    assert_eq!(3, FormatoImpressaoDanfe::Simplificado as u8);
    assert_eq!(4, FormatoImpressaoDanfe::Nfce as u8);
    assert_eq!(5, FormatoImpressaoDanfe::NfceMensagemEletronica as u8);
}

/// Testa os valores do enum TipoConsumidor
#[test]
fn valores_tipo_consumidor() {
    assert_eq!(0, TipoConsumidor::Normal as u8);
    assert_eq!(1, TipoConsumidor::Final as u8);
}

/// Testa os valores do enum TipoPresencaComprador
#[test]
fn valores_tipo_presenca_comprador() {
    assert_eq!(0, TipoPresencaComprador::NaoSeAplica as u8);
    assert_eq!(1, TipoPresencaComprador::Presencial as u8);
    assert_eq!(2, TipoPresencaComprador::ViaInternel as u8);
    assert_eq!(3, TipoPresencaComprador::ViaTeleatendimento as u8);
    assert_eq!(4, TipoPresencaComprador::NfceEmDomicilio as u8);
    assert_eq!(5, TipoPresencaComprador::PresencialForaDoEstabelecimento as u8);
    assert_eq!(9, TipoPresencaComprador::Outros as u8);
}

/// Testa os valores do enum TipoIntermediador
#[test]
fn valores_tipo_intermediador() {
    assert_eq!(0, TipoIntermediador::SemIntermediador as u8);
    assert_eq!(1, TipoIntermediador::EmSiteDeTerceiros as u8);
}
