//! Certificado Digital A1 (arquivo .pfx/.p12)
//!
//! Versão simplificada sem dependência de OpenSSL

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use base64::Engine;

/// Informações do certificado digital
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificadoInfo {
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: String,
    pub not_after: String,
    pub cnpj: Option<String>,
    pub razao_social: Option<String>,
    pub valido: bool,
    pub dias_para_expirar: i64,
}

/// Certificado A1 carregado em memória
#[derive(Clone)]
pub struct CertificadoA1 {
    pub pfx_data: Vec<u8>,
    pub senha: String,
    pub info: CertificadoInfo,
}

impl CertificadoA1 {
    /// Carrega certificado de arquivo .pfx/.p12
    pub fn from_file<P: AsRef<Path>>(path: P, senha: &str) -> Result<Self, String> {
        let pfx_data = fs::read(path.as_ref())
            .map_err(|e| format!("Erro ao ler arquivo do certificado: {}", e))?;

        Self::from_bytes(&pfx_data, senha)
    }

    /// Carrega certificado de bytes (base64 decodificado)
    pub fn from_bytes(pfx_data: &[u8], senha: &str) -> Result<Self, String> {
        // Validar que é um arquivo PKCS12 válido
        if pfx_data.len() < 10 {
            return Err("Arquivo de certificado inválido ou muito pequeno".to_string());
        }

        // Verificar magic bytes do PKCS12 (sequência ASN.1)
        if pfx_data[0] != 0x30 {
            return Err("Arquivo não parece ser um certificado PKCS12 válido".to_string());
        }

        // Extrair informações básicas do certificado
        let info = Self::extract_basic_info(pfx_data)?;

        Ok(Self {
            pfx_data: pfx_data.to_vec(),
            senha: senha.to_string(),
            info,
        })
    }

    /// Extrai informações básicas do certificado
    fn extract_basic_info(pfx_data: &[u8]) -> Result<CertificadoInfo, String> {
        // Análise simplificada do PKCS12
        // Em produção, usar biblioteca especializada como pkcs12 ou openssl

        // Tentar encontrar strings no certificado que podem conter informações
        let data_str = String::from_utf8_lossy(pfx_data);

        // Procurar CNPJ (14 dígitos)
        let cnpj = Self::find_cnpj(&data_str);

        // Procurar razão social (após CN=)
        let razao_social = Self::find_cn(&data_str);

        Ok(CertificadoInfo {
            subject: razao_social.clone().unwrap_or_else(|| "Certificado A1".to_string()),
            issuer: "AC Certificadora".to_string(),
            serial_number: "N/A".to_string(),
            not_before: "N/A".to_string(),
            not_after: "N/A".to_string(),
            cnpj,
            razao_social,
            valido: true, // Assumir válido - validação real requer parsing completo
            dias_para_expirar: 365, // Placeholder
        })
    }

    fn find_cnpj(data: &str) -> Option<String> {
        // Procurar sequência de 14 dígitos que pode ser CNPJ
        let re = regex::Regex::new(r"(\d{14})").ok()?;
        for cap in re.captures_iter(data) {
            if let Some(m) = cap.get(1) {
                let digits = m.as_str();
                // Validar que parece um CNPJ (não começa com muitos zeros)
                if !digits.starts_with("000000") {
                    return Some(digits.to_string());
                }
            }
        }
        None
    }

    fn find_cn(data: &str) -> Option<String> {
        // Procurar Common Name
        if let Some(pos) = data.find("CN=") {
            let start = pos + 3;
            let end = data[start..].find(|c| c == ',' || c == '\0' || c == '/')
                .map(|p| start + p)
                .unwrap_or(start + 50.min(data.len() - start));
            let cn = data[start..end].trim();
            if !cn.is_empty() && cn.len() > 3 {
                return Some(cn.to_string());
            }
        }
        None
    }

    /// Retorna o certificado em base64
    pub fn to_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.pfx_data)
    }

    /// Retorna os bytes do PFX para uso com reqwest Identity
    pub fn pfx_bytes(&self) -> &[u8] {
        &self.pfx_data
    }

    /// Retorna a senha
    pub fn senha(&self) -> &str {
        &self.senha
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cnpj() {
        let data = "algo12345678000199algo";
        let cnpj = CertificadoA1::find_cnpj(data);
        assert_eq!(cnpj, Some("12345678000199".to_string()));
    }
}
