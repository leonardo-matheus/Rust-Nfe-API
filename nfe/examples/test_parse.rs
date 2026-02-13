use nfe::Nfe;

fn main() {
    let xml = r#"<NFe><infNFe versao="4.00" Id="NFe35240112345678901234550010000000011000000015"><ide><cUF>35</cUF><cNF>00000001</cNF><natOp>VENDA</natOp><mod>55</mod><serie>1</serie><nNF>1</nNF><dhEmi>2024-01-15T10:30:00-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>3550308</cMunFG><tpImp>1</tpImp><tpEmis>1</tpEmis><cDV>5</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>1.0</verProc></ide><emit><CNPJ>12345678901234</CNPJ><xNome>EMPRESA</xNome><enderEmit><xLgr>Rua</xLgr><nro>1</nro><xBairro>Centro</xBairro><cMun>3550308</cMun><xMun>SP</xMun><UF>SP</UF><CEP>01310100</CEP></enderEmit><IE>123</IE></emit><det nItem="1"><prod><cProd>1</cProd><cEAN>SEM GTIN</cEAN><xProd>Produto</xProd><NCM>12345678</NCM><CFOP>5102</CFOP><uCom>UN</uCom><qCom>1</qCom><vUnCom>100</vUnCom><vProd>100</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1</qTrib><vUnTrib>100</vUnTrib><indTot>1</indTot></prod><imposto><ICMS><ICMS00><orig>0</orig><CST>00</CST><modBC>0</modBC><vBC>100</vBC><pICMS>18</pICMS><vICMS>18</vICMS></ICMS00></ICMS><PIS><PISAliq><CST>01</CST><vBC>100</vBC><pPIS>1.65</pPIS><vPIS>1.65</vPIS></PISAliq></PIS><COFINS><COFINSAliq><CST>01</CST><vBC>100</vBC><pCOFINS>7.6</pCOFINS><vCOFINS>7.6</vCOFINS></COFINSAliq></COFINS></imposto></det><total><ICMSTot><vBC>100</vBC><vICMS>18</vICMS><vProd>100</vProd><vFrete>0</vFrete><vSeg>0</vSeg><vDesc>0</vDesc><vOutro>0</vOutro><vPIS>1.65</vPIS><vCOFINS>7.6</vCOFINS><vNF>100</vNF><vTotTrib>27.25</vTotTrib></ICMSTot></total><transp><modFrete>9</modFrete></transp></infNFe></NFe>"#;

    match xml.parse::<Nfe>() {
        Ok(nfe) => {
            println!("Sucesso!");
            println!("Chave: {}", nfe.chave_acesso);
            println!("Emitente: {:?}", nfe.emit.razao_social);
        }
        Err(e) => {
            println!("Erro: {}", e);
        }
    }
}
