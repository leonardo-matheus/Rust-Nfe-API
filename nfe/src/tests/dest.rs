//! Testes da tag <dest>

use crate::base::dest::{Destinatario, IndicadorContribuicaoIe};
use crate::Error;

#[test]
fn parse_destinatario() -> Result<(), Error> {
    let xml = "
        <dest>
            <CNPJ>58716523000119</CNPJ>
            <xNome>NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xNome>
            <enderDest>
                <xLgr>Av. Teste</xLgr>
                <nro>2040</nro>
                <xBairro>Centro</xBairro>
                <cMun>3550308</cMun>
                <xMun>SAO PAULO</xMun>
                <UF>SP</UF>
                <CEP>04207040</CEP>
                <cPais>1058</cPais>
                <xPais>BRASIL</xPais>
                <fone>5190909090</fone>
            </enderDest>
            <indIEDest>1</indIEDest>
            <IE>112006603110</IE>
        </dest>
    ";

    let dest = xml.parse::<Destinatario>()?;

    assert_eq!("58716523000119", dest.cnpj);
    assert_eq!(IndicadorContribuicaoIe::ContribuinteIe, dest.indicador_ie);
    assert_eq!(Some("112006603110".to_string()), dest.ie);

    Ok(())
}

#[test]
fn parse_destinatario_nao_contribuinte() -> Result<(), Error> {
    let xml = "
        <dest>
            <CNPJ>99999999000191</CNPJ>
            <xNome>CONSUMIDOR FINAL</xNome>
            <enderDest>
                <xLgr>Rua Teste</xLgr>
                <nro>100</nro>
                <xBairro>Centro</xBairro>
                <cMun>3550308</cMun>
                <xMun>SAO PAULO</xMun>
                <UF>SP</UF>
                <CEP>01000000</CEP>
            </enderDest>
            <indIEDest>9</indIEDest>
        </dest>
    ";

    let dest = xml.parse::<Destinatario>()?;

    assert_eq!("99999999000191", dest.cnpj);
    assert_eq!(IndicadorContribuicaoIe::NaoContribuinteIe, dest.indicador_ie);
    assert_eq!(None, dest.ie);

    Ok(())
}
