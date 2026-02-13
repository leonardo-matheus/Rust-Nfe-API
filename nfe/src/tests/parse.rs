//! Testes da API de parse da NF-e
//!
//! Este módulo testa as diferentes formas de fazer parsing de componentes XML de NF-e.
//! Os testes de NF-e completa foram movidos para o módulo de testes de integração
//! pois dependem de XMLs reais com estrutura complexa.

use crate::base::item::Produto;
use crate::Error;

/// Testa o parsing de produto isolado
#[test]
fn parse_produto_completo() -> Result<(), Error> {
    let xml = r#"
        <prod>
            <cProd>PROD001</cProd>
            <cEAN>7891234567890</cEAN>
            <xProd>Produto de Teste Completo</xProd>
            <NCM>12345678</NCM>
            <CEST>1234567</CEST>
            <CFOP>5102</CFOP>
            <uCom>UN</uCom>
            <qCom>10.5000</qCom>
            <vUnCom>25.50</vUnCom>
            <vProd>267.75</vProd>
            <vFrete>10.00</vFrete>
            <vSeg>5.00</vSeg>
            <vDesc>7.75</vDesc>
            <cEANTrib>7891234567890</cEANTrib>
            <uTrib>UN</uTrib>
            <qTrib>10.5000</qTrib>
            <vUnTrib>25.50</vUnTrib>
            <indTot>1</indTot>
        </prod>
    "#;

    let produto: Produto = xml.parse()?;

    assert_eq!("PROD001", produto.codigo);
    assert_eq!(Some("7891234567890".to_string()), produto.gtin);
    assert_eq!("Produto de Teste Completo", produto.descricao);
    assert_eq!("12345678", produto.ncm);
    assert_eq!("UN", produto.unidade);
    assert_eq!(10.5, produto.quantidade);
    assert_eq!(25.50, produto.valor_unitario);
    assert_eq!(267.75, produto.valor_bruto);
    assert_eq!(Some(10.0), produto.valor_frete);
    assert_eq!(Some(5.0), produto.valor_seguro);
    assert_eq!(Some(7.75), produto.valor_desconto);
    assert!(produto.valor_compoe_total_nota);

    // Verifica tributação
    assert_eq!(Some("1234567".to_string()), produto.tributacao.cest);
    assert_eq!("5102", produto.tributacao.cfop);

    Ok(())
}

/// Testa o tratamento de erro para XML inválido
#[test]
fn parse_xml_invalido() {
    let xml_invalido = "<NFe><infNFe></NFe>";

    let resultado = xml_invalido.parse::<crate::Nfe>();

    assert!(resultado.is_err());
}

/// Testa o tratamento de erro para XML vazio
#[test]
fn parse_xml_vazio() {
    let xml_vazio = "";

    let resultado = xml_vazio.parse::<crate::Nfe>();

    assert!(resultado.is_err());
}

/// Testa o tratamento de erro para XML com tag raiz errada
#[test]
fn parse_xml_tag_errada() {
    let xml = "<NotaNFe><infNFe></infNFe></NotaNFe>";

    let resultado = xml.parse::<crate::Nfe>();

    assert!(resultado.is_err());
}

/// Testa o construtor de Produto
#[test]
fn criar_produto_new() {
    let produto = Produto::new(
        "COD001".to_string(),
        "Produto Teste".to_string(),
        "12345678".to_string(),
        "5102".to_string(),
        "UN".to_string(),
        10.0,
        50.0,
        500.0,
    );

    assert_eq!("COD001", produto.codigo);
    assert_eq!("Produto Teste", produto.descricao);
    assert_eq!("12345678", produto.ncm);
    assert_eq!("5102", produto.tributacao.cfop);
    assert_eq!("UN", produto.unidade);
    assert_eq!(10.0, produto.quantidade);
    assert_eq!(50.0, produto.valor_unitario);
    assert_eq!(500.0, produto.valor_bruto);
    assert!(produto.valor_compoe_total_nota);
    assert!(produto.gtin.is_none());
    assert!(produto.valor_frete.is_none());
    assert!(produto.valor_desconto.is_none());
}

/// Testa o tratamento de GTIN "SEM GTIN"
#[test]
fn parse_produto_sem_gtin() -> Result<(), Error> {
    let xml = r#"
        <prod>
            <cProd>PROD001</cProd>
            <cEAN>SEM GTIN</cEAN>
            <xProd>Produto sem GTIN</xProd>
            <NCM>12345678</NCM>
            <CFOP>5102</CFOP>
            <uCom>UN</uCom>
            <qCom>1</qCom>
            <vUnCom>100.00</vUnCom>
            <vProd>100.00</vProd>
            <cEANTrib>SEM GTIN</cEANTrib>
            <uTrib>UN</uTrib>
            <qTrib>1</qTrib>
            <vUnTrib>100.00</vUnTrib>
            <indTot>1</indTot>
        </prod>
    "#;

    let produto: Produto = xml.parse()?;

    // "SEM GTIN" deve ser convertido para None
    assert!(produto.gtin.is_none());
    assert!(produto.tributacao.gtin.is_none());

    Ok(())
}

/// Testa o tratamento de GTIN vazio
#[test]
fn parse_produto_gtin_vazio() -> Result<(), Error> {
    let xml = r#"
        <prod>
            <cProd>PROD001</cProd>
            <cEAN></cEAN>
            <xProd>Produto com GTIN vazio</xProd>
            <NCM>12345678</NCM>
            <CFOP>5102</CFOP>
            <uCom>UN</uCom>
            <qCom>1</qCom>
            <vUnCom>100.00</vUnCom>
            <vProd>100.00</vProd>
            <cEANTrib></cEANTrib>
            <uTrib>UN</uTrib>
            <qTrib>1</qTrib>
            <vUnTrib>100.00</vUnTrib>
            <indTot>1</indTot>
        </prod>
    "#;

    let produto: Produto = xml.parse()?;

    // GTIN vazio deve ser convertido para None
    assert!(produto.gtin.is_none());

    Ok(())
}
