//! Assinatura XML para NF-e
//!
//! Implementa digest e estrutura de assinatura digital conforme padrão SEFAZ
//! Nota: A assinatura RSA requer certificado digital válido

use super::CertificadoA1;
use base64::Engine;
use sha2::{Sha256, Digest};

/// Assina XML de NF-e
pub struct AssinadorXml {
    certificado: CertificadoA1,
}

impl AssinadorXml {
    pub fn new(certificado: CertificadoA1) -> Self {
        Self { certificado }
    }

    /// Prepara o XML para assinatura (calcula digest e monta estrutura)
    /// Nota: A assinatura RSA final precisa ser feita com a chave privada do certificado
    pub fn assinar_nfe(&self, xml: &str) -> Result<String, String> {
        // Encontrar o elemento infNFe para assinar
        let (inf_nfe_start, inf_nfe_end) = self.find_element(xml, "infNFe")?;
        let inf_nfe_content = &xml[inf_nfe_start..inf_nfe_end];

        // Extrair o Id do infNFe
        let id = self.extract_id(inf_nfe_content)?;

        // Canonicalizar o XML (C14N simplificado)
        let canonical = self.canonicalize(inf_nfe_content)?;

        // Calcular digest (SHA-256)
        let digest = self.calculate_digest(&canonical);
        let digest_b64 = base64::engine::general_purpose::STANDARD.encode(&digest);

        // Criar SignedInfo
        let signed_info = self.create_signed_info(&id, &digest_b64);

        // Para assinatura real, precisaria:
        // 1. Carregar chave privada do certificado PFX
        // 2. Assinar o SignedInfo canonicalizado com RSA-SHA256
        // Por enquanto, criar placeholder
        let signature_placeholder = base64::engine::general_purpose::STANDARD.encode(
            format!("SIGNATURE_PLACEHOLDER_{}", self.certificado.info.cnpj.as_deref().unwrap_or(""))
        );

        // Certificado em base64 (placeholder)
        let cert_b64 = base64::engine::general_purpose::STANDARD.encode(
            format!("CERTIFICATE_PLACEHOLDER")
        );

        // Montar elemento Signature
        let signature_element = self.create_signature_element(&signed_info, &signature_placeholder, &cert_b64);

        // Inserir Signature no XML
        let result = self.insert_signature(xml, &signature_element, inf_nfe_end)?;

        Ok(result)
    }

    /// Assina evento (cancelamento, carta de correção, etc)
    pub fn assinar_evento(&self, xml: &str) -> Result<String, String> {
        let (inf_evento_start, inf_evento_end) = self.find_element(xml, "infEvento")?;
        let inf_evento_content = &xml[inf_evento_start..inf_evento_end];

        let id = self.extract_id(inf_evento_content)?;
        let canonical = self.canonicalize(inf_evento_content)?;
        let digest = self.calculate_digest(&canonical);
        let digest_b64 = base64::engine::general_purpose::STANDARD.encode(&digest);

        let signed_info = self.create_signed_info(&id, &digest_b64);

        let signature_placeholder = base64::engine::general_purpose::STANDARD.encode(
            format!("SIGNATURE_PLACEHOLDER_{}", self.certificado.info.cnpj.as_deref().unwrap_or(""))
        );
        let cert_b64 = base64::engine::general_purpose::STANDARD.encode("CERTIFICATE_PLACEHOLDER");

        let signature_element = self.create_signature_element(&signed_info, &signature_placeholder, &cert_b64);
        let result = self.insert_signature(xml, &signature_element, inf_evento_end)?;

        Ok(result)
    }

    fn find_element(&self, xml: &str, element: &str) -> Result<(usize, usize), String> {
        let start_tag = format!("<{}", element);
        let end_tag = format!("</{}>", element);

        let start = xml.find(&start_tag)
            .ok_or(format!("Elemento {} não encontrado", element))?;
        let end = xml.find(&end_tag)
            .ok_or(format!("Fim do elemento {} não encontrado", element))?;

        Ok((start, end + end_tag.len()))
    }

    fn extract_id(&self, xml: &str) -> Result<String, String> {
        let id_start = xml.find("Id=\"")
            .ok_or("Atributo Id não encontrado")?;
        let id_value_start = id_start + 4;
        let id_value_end = xml[id_value_start..].find('"')
            .ok_or("Fim do atributo Id não encontrado")?;

        Ok(xml[id_value_start..id_value_start + id_value_end].to_string())
    }

    fn canonicalize(&self, xml: &str) -> Result<String, String> {
        let mut result = xml.to_string();

        // Remover declaração XML
        if let Some(decl_end) = result.find("?>") {
            result = result[decl_end + 2..].trim_start().to_string();
        }

        // Normalizar espaços em branco
        result = result
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<_>>()
            .join("");

        // Remover espaços entre tags
        let re = regex::Regex::new(r">\s+<").unwrap();
        result = re.replace_all(&result, "><").to_string();

        Ok(result)
    }

    fn calculate_digest(&self, data: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec()
    }

    fn create_signed_info(&self, reference_id: &str, digest_value: &str) -> String {
        format!(
            "<SignedInfo xmlns=\"http://www.w3.org/2000/09/xmldsig#\">\
             <CanonicalizationMethod Algorithm=\"http://www.w3.org/TR/2001/REC-xml-c14n-20010315\"/>\
             <SignatureMethod Algorithm=\"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256\"/>\
             <Reference URI=\"#{reference_id}\">\
             <Transforms>\
             <Transform Algorithm=\"http://www.w3.org/2000/09/xmldsig#enveloped-signature\"/>\
             <Transform Algorithm=\"http://www.w3.org/TR/2001/REC-xml-c14n-20010315\"/>\
             </Transforms>\
             <DigestMethod Algorithm=\"http://www.w3.org/2001/04/xmlenc#sha256\"/>\
             <DigestValue>{digest_value}</DigestValue>\
             </Reference>\
             </SignedInfo>"
        )
    }

    fn create_signature_element(&self, signed_info: &str, signature_value: &str, cert_b64: &str) -> String {
        format!(
            r#"<Signature xmlns="http://www.w3.org/2000/09/xmldsig#">{signed_info}<SignatureValue>{signature_value}</SignatureValue><KeyInfo><X509Data><X509Certificate>{cert_b64}</X509Certificate></X509Data></KeyInfo></Signature>"#
        )
    }

    fn insert_signature(&self, xml: &str, signature: &str, position: usize) -> Result<String, String> {
        let before = &xml[..position];
        let after = &xml[position..];
        Ok(format!("{}{}{}", before, signature, after))
    }
}

/// Valida estrutura de assinatura de XML (não valida a assinatura RSA)
pub fn validar_estrutura_assinatura(xml: &str) -> Result<bool, String> {
    if !xml.contains("<Signature") {
        return Err("XML não possui assinatura".to_string());
    }

    // Verificar elementos obrigatórios
    let required = ["SignedInfo", "SignatureValue", "DigestValue"];
    for elem in required {
        if !xml.contains(&format!("<{}", elem)) {
            return Err(format!("Elemento {} não encontrado na assinatura", elem));
        }
    }

    Ok(true)
}
