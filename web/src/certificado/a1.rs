//! Certificado Digital A1 (arquivo .pfx/.p12)
//!
//! Parsing real de certificado PKCS12 usando crates p12 e x509-cert

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use base64::Engine;
use chrono::{DateTime, Utc};

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
    /// Certificado X509 em DER
    pub cert_der: Vec<u8>,
    /// Chave privada em DER (PKCS8)
    pub private_key_der: Vec<u8>,
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

        // Parsear o arquivo PFX usando p12
        let pfx = p12::PFX::parse(pfx_data)
            .map_err(|e| format!("Erro ao parsear PKCS12: {:?}", e))?;

        // Extrair certificados (DER encoded)
        let certs = pfx.cert_bags(senha)
            .map_err(|e| format!("Erro ao extrair certificado (senha incorreta?): {:?}", e))?;

        // Extrair chaves privadas (DER encoded)
        let keys = pfx.key_bags(senha)
            .map_err(|e| format!("Erro ao extrair chave privada: {:?}", e))?;

        let cert_der = certs.into_iter().next()
            .ok_or("Certificado não encontrado no arquivo PFX")?;

        let private_key_der = keys.into_iter().next()
            .ok_or("Chave privada não encontrada no arquivo PFX")?;

        // Extrair informações do certificado X509
        let info = Self::extract_cert_info(&cert_der)?;

        Ok(Self {
            pfx_data: pfx_data.to_vec(),
            senha: senha.to_string(),
            info,
            cert_der,
            private_key_der,
        })
    }

    /// Extrai informações do certificado X509 DER
    fn extract_cert_info(cert_der: &[u8]) -> Result<CertificadoInfo, String> {
        use x509_cert::Certificate;
        use der::Decode;

        let cert = Certificate::from_der(cert_der)
            .map_err(|e| format!("Erro ao parsear certificado X509: {:?}", e))?;

        // Subject (nome do titular)
        let subject = cert.tbs_certificate.subject.to_string();

        // Issuer (autoridade certificadora)
        let issuer = cert.tbs_certificate.issuer.to_string();

        // Número serial
        let serial_bytes = cert.tbs_certificate.serial_number.as_bytes();
        let serial_number = hex::encode(serial_bytes);

        // Datas de validade
        let not_before = cert.tbs_certificate.validity.not_before.to_system_time();
        let not_after = cert.tbs_certificate.validity.not_after.to_system_time();

        let not_before_dt: DateTime<Utc> = not_before.into();
        let not_after_dt: DateTime<Utc> = not_after.into();

        // Calcular dias para expirar
        let now = Utc::now();
        let dias_para_expirar = (not_after_dt - now).num_days();
        let valido = now >= not_before_dt && now <= not_after_dt;

        // Extrair CNPJ do subject (padrão ICP-Brasil)
        let cnpj = Self::extract_cnpj_from_subject(&subject);

        // Extrair razão social do CN
        let razao_social = Self::extract_cn_from_subject(&subject);

        Ok(CertificadoInfo {
            subject: subject.clone(),
            issuer,
            serial_number,
            not_before: not_before_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            not_after: not_after_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            cnpj,
            razao_social,
            valido,
            dias_para_expirar,
        })
    }

    /// Extrai CNPJ do subject do certificado (padrão ICP-Brasil)
    fn extract_cnpj_from_subject(subject: &str) -> Option<String> {
        // Padrões comuns de CNPJ no certificado ICP-Brasil:
        // serialNumber=12345678000199
        // 2.16.76.1.3.3=12345678000199 (OID para CNPJ)

        // Tentar extrair do serialNumber
        if let Some(pos) = subject.find("serialNumber=") {
            let start = pos + 13;
            let end = subject[start..].find(|c: char| c == ',' || c == '+' || c == '/')
                .map(|p| start + p)
                .unwrap_or(subject.len());
            let value = &subject[start..end];
            // CNPJ tem 14 dígitos
            let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() == 14 {
                return Some(digits);
            }
        }

        // Tentar extrair do OID do CNPJ (2.16.76.1.3.3)
        if let Some(pos) = subject.find("2.16.76.1.3.3=") {
            let start = pos + 14;
            let end = subject[start..].find(|c: char| c == ',' || c == '+' || c == '/')
                .map(|p| start + p)
                .unwrap_or(subject.len());
            let value = &subject[start..end];
            let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() == 14 {
                return Some(digits);
            }
        }

        // Busca genérica por 14 dígitos consecutivos
        let re = regex::Regex::new(r"(\d{14})").ok()?;
        for cap in re.captures_iter(subject) {
            if let Some(m) = cap.get(1) {
                let digits = m.as_str();
                if !digits.starts_with("000000") {
                    return Some(digits.to_string());
                }
            }
        }

        None
    }

    /// Extrai Common Name (CN) do subject
    fn extract_cn_from_subject(subject: &str) -> Option<String> {
        // Procurar CN=...
        if let Some(pos) = subject.find("CN=") {
            let start = pos + 3;
            let end = subject[start..].find(|c: char| c == ',' || c == '+')
                .map(|p| start + p)
                .unwrap_or(subject.len());
            let cn = subject[start..end].trim();
            if !cn.is_empty() {
                return Some(cn.to_string());
            }
        }
        None
    }

    /// Retorna o certificado em base64 (formato PEM sem headers)
    pub fn cert_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(&self.cert_der)
    }

    /// Retorna o certificado em formato PEM
    pub fn cert_pem(&self) -> String {
        let b64 = self.cert_base64();
        let lines: Vec<&str> = b64.as_bytes()
            .chunks(64)
            .map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
            .collect();
        format!("-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----", lines.join("\n"))
    }

    /// Retorna o PFX original em base64
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

    /// Verifica se o certificado ainda é válido
    pub fn is_valid(&self) -> bool {
        self.info.valido && self.info.dias_para_expirar > 0
    }

    /// Retorna a chave privada RSA
    pub fn private_key(&self) -> Result<rsa::RsaPrivateKey, String> {
        use pkcs8::DecodePrivateKey;

        rsa::RsaPrivateKey::from_pkcs8_der(&self.private_key_der)
            .map_err(|e| format!("Erro ao carregar chave privada: {:?}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_cnpj_serial_number() {
        let subject = "CN=EMPRESA TESTE LTDA:12345678000199,serialNumber=12345678000199,C=BR";
        let cnpj = CertificadoA1::extract_cnpj_from_subject(subject);
        assert_eq!(cnpj, Some("12345678000199".to_string()));
    }

    #[test]
    fn test_extract_cn() {
        let subject = "CN=EMPRESA TESTE LTDA,OU=AR,O=ICP-Brasil,C=BR";
        let cn = CertificadoA1::extract_cn_from_subject(subject);
        assert_eq!(cn, Some("EMPRESA TESTE LTDA".to_string()));
    }
}
