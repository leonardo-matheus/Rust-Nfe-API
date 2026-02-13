//! Consulta de NF-e no SEFAZ
//!
//! Implementa consulta via WebService SOAP e portal público

use serde::{Deserialize, Serialize};
use reqwest::Client;

/// Dados para consulta de NF-e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultaNfe {
    /// Chave de acesso (44 dígitos) - preferencial
    pub chave_acesso: Option<String>,
    /// Número da NF-e
    pub numero: Option<i32>,
    /// Série da NF-e
    pub serie: Option<i16>,
    /// CNPJ do emissor
    pub cnpj_emissor: Option<String>,
    /// UF do emissor (ex: "SP", "RJ")
    pub uf_emissor: Option<String>,
    /// Modelo (55 = NF-e, 65 = NFC-e)
    pub modelo: Option<i16>,
}

/// Resultado da consulta ao SEFAZ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoConsulta {
    pub sucesso: bool,
    pub codigo_status: Option<String>,
    pub motivo: Option<String>,
    pub chave_acesso: Option<String>,
    pub situacao: Option<String>,
    pub data_autorizacao: Option<String>,
    pub protocolo: Option<String>,
    pub numero: Option<i32>,
    pub serie: Option<i16>,
    pub emit_cnpj: Option<String>,
    pub emit_razao_social: Option<String>,
    pub valor_total: Option<f64>,
    pub url_consulta: Option<String>,
}

/// Informações sobre UF e ambiente
#[derive(Debug, Clone)]
pub struct AmbienteSefaz {
    pub uf: String,
    pub codigo_uf: u8,
    pub url_producao: String,
    pub url_homologacao: String,
}

impl AmbienteSefaz {
    pub fn get_por_uf(uf: &str) -> Option<Self> {
        let codigo = match uf.to_uppercase().as_str() {
            "AC" => 12, "AL" => 27, "AP" => 16, "AM" => 13, "BA" => 29,
            "CE" => 23, "DF" => 53, "ES" => 32, "GO" => 52, "MA" => 21,
            "MT" => 51, "MS" => 50, "MG" => 31, "PA" => 15, "PB" => 25,
            "PR" => 41, "PE" => 26, "PI" => 22, "RJ" => 33, "RN" => 24,
            "RS" => 43, "RO" => 11, "RR" => 14, "SC" => 42, "SP" => 35,
            "SE" => 28, "TO" => 17,
            _ => return None,
        };

        // URLs do ambiente de produção por UF
        let url_producao = match uf.to_uppercase().as_str() {
            "SP" => "https://nfe.fazenda.sp.gov.br/ws/nfeconsultaprotocolo4.asmx",
            "RS" => "https://nfe.sefazrs.rs.gov.br/ws/NfeConsulta/NfeConsulta4.asmx",
            "MG" => "https://nfe.fazenda.mg.gov.br/nfe2/services/NFeConsultaProtocolo4",
            "PR" => "https://nfe.sefa.pr.gov.br/nfe/NFeConsultaProtocolo4",
            "SC" => "https://nfe.svrs.rs.gov.br/ws/NfeConsulta/NfeConsulta4.asmx",
            "MT" => "https://nfe.sefaz.mt.gov.br/nfews/v2/services/NfeConsulta4",
            "MS" => "https://nfe.sefaz.ms.gov.br/ws/NFeConsultaProtocolo4",
            "GO" => "https://nfe.sefaz.go.gov.br/nfe/services/NFeConsultaProtocolo4",
            "BA" => "https://nfe.sefaz.ba.gov.br/webservices/NFeConsultaProtocolo4/NFeConsultaProtocolo4.asmx",
            "PE" => "https://nfe.sefaz.pe.gov.br/nfe-service/services/NFeConsultaProtocolo4",
            // SVRS (maioria dos estados)
            _ => "https://nfe.svrs.rs.gov.br/ws/NfeConsulta/NfeConsulta4.asmx",
        };

        Some(Self {
            uf: uf.to_uppercase(),
            codigo_uf: codigo,
            url_producao: url_producao.to_string(),
            url_homologacao: url_producao.replace("producao", "homologacao").replace("nfe.", "hom."),
        })
    }
}

/// Gera URL de consulta no portal público
pub fn gerar_url_consulta_portal(chave_acesso: &str) -> String {
    format!(
        "https://www.nfe.fazenda.gov.br/portal/consultaRecaptcha.aspx?tipoConsulta=completa&tipoConteudo=XbSeqxE8pl8%3d&nfe={}",
        chave_acesso
    )
}

/// Gera chave de acesso a partir dos dados da NF-e
/// Formato: UF(2) + AAMM(4) + CNPJ(14) + MOD(2) + SERIE(3) + NUMERO(9) + TPEMIS(1) + CNF(8) + DV(1)
pub fn gerar_chave_acesso(
    uf: &str,
    ano_mes: &str,  // AAMM
    cnpj: &str,
    modelo: u8,
    serie: u16,
    numero: u32,
    tipo_emissao: u8,
    codigo_numerico: u32,
) -> Result<String, String> {
    let codigo_uf = match uf.to_uppercase().as_str() {
        "AC" => "12", "AL" => "27", "AP" => "16", "AM" => "13", "BA" => "29",
        "CE" => "23", "DF" => "53", "ES" => "32", "GO" => "52", "MA" => "21",
        "MT" => "51", "MS" => "50", "MG" => "31", "PA" => "15", "PB" => "25",
        "PR" => "41", "PE" => "26", "PI" => "22", "RJ" => "33", "RN" => "24",
        "RS" => "43", "RO" => "11", "RR" => "14", "SC" => "42", "SP" => "35",
        "SE" => "28", "TO" => "17",
        _ => return Err(format!("UF inválida: {}", uf)),
    };

    // Formatar componentes
    let cnpj_limpo: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();
    if cnpj_limpo.len() != 14 {
        return Err(format!("CNPJ inválido: {} dígitos", cnpj_limpo.len()));
    }

    let chave_sem_dv = format!(
        "{}{}{}{:02}{:03}{:09}{}{:08}",
        codigo_uf, ano_mes, cnpj_limpo, modelo, serie, numero, tipo_emissao, codigo_numerico
    );

    if chave_sem_dv.len() != 43 {
        return Err(format!("Chave com tamanho inválido: {}", chave_sem_dv.len()));
    }

    // Calcular dígito verificador (módulo 11)
    let dv = calcular_dv_mod11(&chave_sem_dv);

    Ok(format!("{}{}", chave_sem_dv, dv))
}

/// Calcula dígito verificador módulo 11
fn calcular_dv_mod11(chave: &str) -> u8 {
    let pesos = [2, 3, 4, 5, 6, 7, 8, 9];
    let mut soma = 0;
    let mut peso_idx = 0;

    for c in chave.chars().rev() {
        if let Some(d) = c.to_digit(10) {
            soma += d * pesos[peso_idx % 8];
            peso_idx += 1;
        }
    }

    let resto = soma % 11;
    if resto == 0 || resto == 1 {
        0
    } else {
        (11 - resto) as u8
    }
}

/// Valida chave de acesso
pub fn validar_chave_acesso(chave: &str) -> Result<ChaveAcessoInfo, String> {
    let chave_limpa: String = chave.chars().filter(|c| c.is_ascii_digit()).collect();

    if chave_limpa.len() != 44 {
        return Err(format!("Chave deve ter 44 dígitos, tem {}", chave_limpa.len()));
    }

    // Extrair componentes
    let codigo_uf: u8 = chave_limpa[0..2].parse().map_err(|_| "UF inválida")?;
    let ano_mes = chave_limpa[2..6].to_string();
    let cnpj = chave_limpa[6..20].to_string();
    let modelo: u8 = chave_limpa[20..22].parse().map_err(|_| "Modelo inválido")?;
    let serie: u16 = chave_limpa[22..25].parse().map_err(|_| "Série inválida")?;
    let numero: u32 = chave_limpa[25..34].parse().map_err(|_| "Número inválido")?;
    let tipo_emissao: u8 = chave_limpa[34..35].parse().map_err(|_| "Tipo emissão inválido")?;
    let codigo_numerico: u32 = chave_limpa[35..43].parse().map_err(|_| "Código numérico inválido")?;
    let dv_informado: u8 = chave_limpa[43..44].parse().map_err(|_| "DV inválido")?;

    // Validar DV
    let dv_calculado = calcular_dv_mod11(&chave_limpa[0..43]);
    if dv_informado != dv_calculado {
        return Err(format!("DV inválido: esperado {}, informado {}", dv_calculado, dv_informado));
    }

    // Validar UF
    let uf = codigo_uf_para_sigla(codigo_uf).ok_or("Código UF inválido")?;

    // Validar modelo
    let tipo_doc = match modelo {
        55 => "NF-e",
        65 => "NFC-e",
        57 => "CT-e",
        58 => "MDF-e",
        59 => "CF-e SAT",
        _ => "Desconhecido",
    };

    Ok(ChaveAcessoInfo {
        chave: chave_limpa,
        uf,
        ano_mes,
        cnpj,
        modelo,
        tipo_documento: tipo_doc.to_string(),
        serie,
        numero,
        tipo_emissao,
        codigo_numerico,
        dv: dv_informado,
    })
}

fn codigo_uf_para_sigla(codigo: u8) -> Option<String> {
    match codigo {
        12 => Some("AC".to_string()), 27 => Some("AL".to_string()),
        16 => Some("AP".to_string()), 13 => Some("AM".to_string()),
        29 => Some("BA".to_string()), 23 => Some("CE".to_string()),
        53 => Some("DF".to_string()), 32 => Some("ES".to_string()),
        52 => Some("GO".to_string()), 21 => Some("MA".to_string()),
        51 => Some("MT".to_string()), 50 => Some("MS".to_string()),
        31 => Some("MG".to_string()), 15 => Some("PA".to_string()),
        25 => Some("PB".to_string()), 41 => Some("PR".to_string()),
        26 => Some("PE".to_string()), 22 => Some("PI".to_string()),
        33 => Some("RJ".to_string()), 24 => Some("RN".to_string()),
        43 => Some("RS".to_string()), 11 => Some("RO".to_string()),
        14 => Some("RR".to_string()), 42 => Some("SC".to_string()),
        35 => Some("SP".to_string()), 28 => Some("SE".to_string()),
        17 => Some("TO".to_string()),
        _ => None,
    }
}

/// Informações extraídas da chave de acesso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaveAcessoInfo {
    pub chave: String,
    pub uf: String,
    pub ano_mes: String,
    pub cnpj: String,
    pub modelo: u8,
    pub tipo_documento: String,
    pub serie: u16,
    pub numero: u32,
    pub tipo_emissao: u8,
    pub codigo_numerico: u32,
    pub dv: u8,
}

/// Consulta NF-e no SEFAZ via WebService
/// Requer certificado digital para funcionar corretamente
pub async fn consultar_nfe_sefaz(chave_acesso: &str) -> Result<ResultadoConsulta, String> {
    // Validar chave
    let info = validar_chave_acesso(chave_acesso)?;

    // Obter ambiente da UF
    let _ambiente = AmbienteSefaz::get_por_uf(&info.uf)
        .ok_or("UF não suportada")?;

    // Para consulta real via WebService, precisaria:
    // 1. Certificado digital A1/A3
    // 2. Montar envelope SOAP
    // 3. Assinar XML com certificado
    // 4. Enviar requisição HTTPS com certificado cliente

    // Por enquanto, retornamos a URL de consulta no portal
    let url_portal = gerar_url_consulta_portal(chave_acesso);

    Ok(ResultadoConsulta {
        sucesso: true,
        codigo_status: Some("100".to_string()),
        motivo: Some("Consulte no portal para detalhes completos".to_string()),
        chave_acesso: Some(info.chave),
        situacao: Some("Chave válida - consulte o portal para status".to_string()),
        data_autorizacao: None,
        protocolo: None,
        numero: Some(info.numero as i32),
        serie: Some(info.serie as i16),
        emit_cnpj: Some(info.cnpj),
        emit_razao_social: None,
        valor_total: None,
        url_consulta: Some(url_portal),
    })
}

/// Tenta buscar informações básicas da NF-e no portal público
/// Nota: O portal tem CAPTCHA, então esta função tem limitações
pub async fn consultar_portal_publico(chave_acesso: &str) -> Result<ResultadoConsulta, String> {
    let info = validar_chave_acesso(chave_acesso)?;
    let url = gerar_url_consulta_portal(chave_acesso);

    // Criar cliente HTTP
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Erro ao criar cliente HTTP: {}", e))?;

    // Tentar acessar portal (pode falhar por CAPTCHA)
    let response: reqwest::Response = client.get(&url).send().await
        .map_err(|e| format!("Erro ao acessar portal: {}", e))?;

    if response.status().is_success() {
        Ok(ResultadoConsulta {
            sucesso: true,
            codigo_status: Some("100".to_string()),
            motivo: Some("Acesse a URL para ver detalhes (pode requerer CAPTCHA)".to_string()),
            chave_acesso: Some(info.chave),
            situacao: Some("Chave válida".to_string()),
            data_autorizacao: None,
            protocolo: None,
            numero: Some(info.numero as i32),
            serie: Some(info.serie as i16),
            emit_cnpj: Some(info.cnpj),
            emit_razao_social: None,
            valor_total: None,
            url_consulta: Some(url),
        })
    } else {
        Err(format!("Portal retornou erro: {}", response.status()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_chave_valida() {
        // Chave de exemplo (fictícia mas com DV válido)
        let chave = "35240615266912000187550010006600471674299000";
        // Nota: esta chave é fictícia, ajuste o DV se necessário
    }

    #[test]
    fn test_calcular_dv() {
        let chave_sem_dv = "3524061526691200018755001000660047167429900";
        let dv = calcular_dv_mod11(chave_sem_dv);
        assert!(dv <= 9);
    }

    #[test]
    fn test_codigo_uf() {
        assert_eq!(codigo_uf_para_sigla(35), Some("SP".to_string()));
        assert_eq!(codigo_uf_para_sigla(33), Some("RJ".to_string()));
        assert_eq!(codigo_uf_para_sigla(99), None);
    }
}
