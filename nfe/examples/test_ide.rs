use nfe::Identificacao;

fn main() {
    let xml = r#"<ide><cUF>35</cUF><cNF>00000001</cNF><natOp>VENDA</natOp><mod>55</mod><serie>1</serie><nNF>1</nNF><dhEmi>2024-01-15T10:30:00-03:00</dhEmi><tpNF>1</tpNF><idDest>1</idDest><cMunFG>3550308</cMunFG><tpImp>1</tpImp><tpEmis>1</tpEmis><cDV>5</cDV><tpAmb>2</tpAmb><finNFe>1</finNFe><indFinal>1</indFinal><indPres>1</indPres><procEmi>0</procEmi><verProc>1.0</verProc></ide>"#;

    match xml.parse::<Identificacao>() {
        Ok(ide) => {
            println!("Sucesso!");
            println!("UF: {}", ide.codigo_uf);
            println!("Numero: {}", ide.numero);
        }
        Err(e) => {
            println!("Erro: {}", e);
        }
    }
}
