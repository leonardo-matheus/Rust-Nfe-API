use nfe::Item;

fn main() {
    let xml = r#"<det nItem="1"><prod><cProd>1</cProd><cEAN>SEM GTIN</cEAN><xProd>Produto</xProd><NCM>12345678</NCM><CFOP>5102</CFOP><uCom>UN</uCom><qCom>1</qCom><vUnCom>100</vUnCom><vProd>100</vProd><cEANTrib>SEM GTIN</cEANTrib><uTrib>UN</uTrib><qTrib>1</qTrib><vUnTrib>100</vUnTrib><indTot>1</indTot></prod><imposto><ICMS><ICMS00><orig>0</orig><CST>00</CST><modBC>0</modBC><vBC>100</vBC><pICMS>18</pICMS><vICMS>18</vICMS></ICMS00></ICMS><PIS><PISAliq><CST>01</CST><vBC>100</vBC><pPIS>1.65</pPIS><vPIS>1.65</vPIS></PISAliq></PIS><COFINS><COFINSAliq><CST>01</CST><vBC>100</vBC><pCOFINS>7.6</pCOFINS><vCOFINS>7.6</vCOFINS></COFINSAliq></COFINS></imposto></det>"#;

    match xml.parse::<Item>() {
        Ok(item) => {
            println!("Sucesso!");
            println!("Numero: {}", item.numero);
            println!("Produto: {}", item.produto.descricao);
        }
        Err(e) => {
            println!("Erro: {}", e);
        }
    }
}
