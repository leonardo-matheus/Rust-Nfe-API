use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fmt;
use std::io;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, Error};

fn format_optional_field(
    f: &mut fmt::Formatter,
    name: &str,
    value: &Option<String>,
) -> fmt::Result {
    writeln!(f, "\t{}: {:?},", name, value.as_deref().unwrap_or("None"))
}

// convenção snake_case
#[derive(Debug, Deserialize, PartialEq)]
struct Ide {
    #[serde(rename = "cNF")]
    c_nf: Option<String>,
    #[serde(rename = "natOp")]
    nat_op: Option<String>,
    #[serde(rename = "mod")]
    mod_code: Option<String>,
    serie: Option<String>,
    #[serde(rename = "nNF")]
    n_nf: Option<String>,
    #[serde(rename = "dhEmi")]
    dh_emi: Option<String>,
    #[serde(rename = "dhSaiEnt")]
    dh_sai_ent: Option<String>,
    #[serde(rename = "tpNF")]
    tp_nf: Option<String>,
    #[serde(rename = "idDest")]
    id_dest: Option<String>,
    #[serde(rename = "cMunFG")]
    c_mun_fg: Option<String>,
    #[serde(rename = "tpImp")]
    tp_imp: Option<String>,
    #[serde(rename = "tpEmis")]
    tp_emis: Option<String>,
    #[serde(rename = "cDV")]
    c_dv: Option<String>,
    #[serde(rename = "tpAmb")]
    tp_amb: Option<String>,
    #[serde(rename = "finNFe")]
    fin_n_fe: Option<String>,
    #[serde(rename = "indFinal")]
    ind_final: Option<String>,
    #[serde(rename = "indPres")]
    ind_pres: Option<String>,
    #[serde(rename = "procEmi")]
    proc_emi: Option<String>,
    #[serde(rename = "verProc")]
    ver_proc: Option<String>,
}

impl fmt::Display for Ide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Ide {{")?;
        format_optional_field(f, "c_nf", &self.c_nf)?;
        format_optional_field(f, "nat_op", &self.nat_op)?;
        format_optional_field(f, "mod_code", &self.mod_code)?;
        format_optional_field(f, "serie", &self.serie)?;
        format_optional_field(f, "n_nf", &self.n_nf)?;
        format_optional_field(f, "dh_emi", &self.dh_emi)?;
        format_optional_field(f, "dh_sai_ent", &self.dh_sai_ent)?;
        format_optional_field(f, "tp_nf", &self.tp_nf)?;
        format_optional_field(f, "id_dest", &self.id_dest)?;
        format_optional_field(f, "c_mun_fg", &self.c_mun_fg)?;
        format_optional_field(f, "tp_imp", &self.tp_imp)?;
        format_optional_field(f, "tp_emis", &self.tp_emis)?;
        format_optional_field(f, "c_dv", &self.c_dv)?;
        format_optional_field(f, "tp_amb", &self.tp_amb)?;
        format_optional_field(f, "fin_n_fe", &self.fin_n_fe)?;
        format_optional_field(f, "ind_final", &self.ind_final)?;
        format_optional_field(f, "ind_pres", &self.ind_pres)?;
        format_optional_field(f, "proc_emi", &self.proc_emi)?;
        format_optional_field(f, "ver_proc", &self.ver_proc)?;
        writeln!(f, "}}")
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Emit {
    #[serde(rename = "CNPJ")]
    cnpj: Option<String>,
    #[serde(rename = "xNome")]
    x_nome: Option<String>,
    #[serde(rename = "xFant")]
    x_fant: Option<String>,
    #[serde(rename = "enderEmit")]
    ender_emit: EnderEmit,
    #[serde(rename = "IE")]
    ie: Option<String>,
    #[serde(rename = "CRT")]
    crt: Option<String>,
}

impl fmt::Display for Emit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Emit {{")?;
        format_optional_field(f, "cnpj", &self.cnpj)?;
        format_optional_field(f, "x_nome", &self.x_nome)?;
        format_optional_field(f, "x_fant", &self.x_fant)?;
        writeln!(f, "{}", self.ender_emit)?;
        format_optional_field(f, "ie", &self.ie)?;
        format_optional_field(f, "crt", &self.crt)?;
        writeln!(f, "}}")
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct EnderEmit {
    #[serde(rename = "xLgr")]
    x_lgr: Option<String>,
    #[serde(rename = "nro")]
    nro: Option<String>,
    #[serde(rename = "xBairro")]
    x_bairro: Option<String>,
    #[serde(rename = "cMun")]
    c_mun: Option<String>,
    #[serde(rename = "xMun")]
    x_mun: Option<String>,
    #[serde(rename = "UF")]
    uf: Option<String>,
    #[serde(rename = "CEP")]
    cep: Option<String>,
    #[serde(rename = "cPais")]
    c_pais: Option<String>,
    #[serde(rename = "xPais")]
    x_pais: Option<String>,
    #[serde(rename = "fone")]
    fone: Option<String>,
}

impl fmt::Display for EnderEmit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\tEnderEmit {{")?;
        format_optional_field(f, "x_lgr", &self.x_lgr)?;
        format_optional_field(f, "nro", &self.nro)?;
        format_optional_field(f, "x_bairro", &self.x_bairro)?;
        format_optional_field(f, "c_mun", &self.c_mun)?;
        format_optional_field(f, "x_mun", &self.x_mun)?;
        format_optional_field(f, "uf", &self.uf)?;
        format_optional_field(f, "cep", &self.cep)?;
        format_optional_field(f, "c_pais", &self.c_pais)?;
        format_optional_field(f, "x_pais", &self.x_pais)?;
        format_optional_field(f, "fone", &self.fone)?;
        writeln!(f, "\t}}")
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct InfNFe {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "versao")]
    versao: String,
    #[serde(rename = "ide")]
    ide: Ide,
    #[serde(rename = "emit")]
    emit: Emit,
}

#[derive(Debug, Deserialize, PartialEq)]
struct NFe {
    #[serde(rename = "infNFe")]
    inf_nfe: InfNFe,
}

#[derive(Debug, Deserialize, PartialEq)]
struct NfeProc {
    #[serde(rename = "NFe")]
    nfe: NFe,
}

async fn load_xml_to_buffer(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", ""))
}

fn mask_sensitive_data(data: &mut String, mask_start: &str, mask_len: usize) {
    if let Some(start) = data.find(mask_start) {
        let start = start + mask_start.len();
        if data.len() > start + mask_len {
            let replacement = "*".repeat(mask_len);
            data.replace_range(start..start + mask_len, &replacement);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml_data = load_xml_to_buffer("F:\\nfe.xml").await?;
    let mut nfe_proc: NfeProc = from_str(&xml_data)?;

    mask_sensitive_data(&mut nfe_proc.nfe.inf_nfe.id, "NFe", 5);

    println!("{}", nfe_proc.nfe.inf_nfe.ide);
    println!("{}", nfe_proc.nfe.inf_nfe.emit);

    println!("Pressione Enter para sair...");
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Erro ao ler a entrada: {}", e);
    }

    Ok(())
}
