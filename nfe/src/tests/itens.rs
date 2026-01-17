//! Testes dos itens/produtos da NF-e

use crate::base::item::{Item, Produto};
use crate::Error;

#[test]
fn parse_item() -> Result<(), Error> {
    let xml = r#"
        <det nItem="1">
            <prod>
                <cProd>11007</cProd>
                <cEAN>SEM GTIN</cEAN>
                <xProd>UM PRODUTO TESTE QUALQUER</xProd>
                <NCM>64011000</NCM>
                <CEST>1234567</CEST>
                <CFOP>6101</CFOP>
                <uCom>UN</uCom>
                <qCom>10.0000</qCom>
                <vUnCom>50</vUnCom>
                <vProd>500.00</vProd>
                <cEANTrib>SEM GTIN</cEANTrib>
                <uTrib>UN</uTrib>
                <qTrib>10.0000</qTrib>
                <vUnTrib>50.0000</vUnTrib>
                <indTot>1</indTot>
            </prod>
            <imposto>
                <vTotTrib>0.00</vTotTrib>
            </imposto>
       </det>
    "#;

    let item = xml.parse::<Item>()?;

    assert_eq!(1, item.numero);
    assert_eq!("11007", item.produto.codigo);
    assert_eq!("UM PRODUTO TESTE QUALQUER", item.produto.descricao);
    assert_eq!("64011000", item.produto.ncm);
    assert_eq!("UN", item.produto.unidade);
    assert_eq!(10.0, item.produto.quantidade);
    assert_eq!(50.0, item.produto.valor_unitario);
    assert_eq!(500.0, item.produto.valor_bruto);
    assert!(item.produto.valor_compoe_total_nota);

    Ok(())
}

#[test]
fn parse_produto() -> Result<(), Error> {
    let xml = r#"
        <prod>
            <cProd>PROD001</cProd>
            <cEAN>7891234567890</cEAN>
            <xProd>Produto de Teste</xProd>
            <NCM>12345678</NCM>
            <CFOP>5102</CFOP>
            <uCom>PC</uCom>
            <qCom>5.0000</qCom>
            <vUnCom>25.50</vUnCom>
            <vProd>127.50</vProd>
            <cEANTrib>7891234567890</cEANTrib>
            <uTrib>PC</uTrib>
            <qTrib>5.0000</qTrib>
            <vUnTrib>25.50</vUnTrib>
            <indTot>1</indTot>
        </prod>
    "#;

    let produto = xml.parse::<Produto>()?;

    assert_eq!("PROD001", produto.codigo);
    assert_eq!(Some("7891234567890".to_string()), produto.gtin);
    assert_eq!("Produto de Teste", produto.descricao);
    assert_eq!("12345678", produto.ncm);
    assert_eq!("PC", produto.unidade);
    assert_eq!(5.0, produto.quantidade);
    assert_eq!(25.50, produto.valor_unitario);
    assert_eq!(127.50, produto.valor_bruto);

    Ok(())
}
