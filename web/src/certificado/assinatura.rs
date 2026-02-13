//! Assinatura XML para NF-e
//!
//! Implementa assinatura digital RSA-SHA256 conforme padrão SEFAZ/ICP-Brasil

use super::CertificadoA1;
use base64::Engine;
use sha2::{Sha256, Digest};
use rsa::pkcs1v15::SigningKey;
use rsa::signature::{Signer, SignatureEncoding};

/// Assina XML de NF-e com certificado digital A1
pub struct AssinadorXml {
    certificado: CertificadoA1,
}

impl AssinadorXml {
    pub fn new(certificado: CertificadoA1) -> Self {
        Self { certificado }
    }

    /// Assina XML de NF-e com RSA-SHA256 real
    pub fn assinar_nfe(&self, xml: &str) -> Result<String, String> {
        self.assinar_elemento(xml, "infNFe")
    }

    /// Assina evento (cancelamento, carta de correção, etc)
    pub fn assinar_evento(&self, xml: &str) -> Result<String, String> {
        self.assinar_elemento(xml, "infEvento")
    }

    /// Assina lote de NF-e
    pub fn assinar_lote(&self, xml: &str) -> Result<String, String> {
        self.assinar_elemento(xml, "infRec")
    }

    /// Assina inutilização de numeração
    pub fn assinar_inutilizacao(&self, xml: &str) -> Result<String, String> {
        self.assinar_elemento(xml, "infInut")
    }

    /// Assina um elemento XML específico
    fn assinar_elemento(&self, xml: &str, element_name: &str) -> Result<String, String> {
        // Encontrar o elemento para assinar
        let (elem_start, elem_end) = self.find_element(xml, element_name)?;
        let elem_content = &xml[elem_start..elem_end];

        // Extrair o Id do elemento
        let id = self.extract_id(elem_content)?;

        // Canonicalizar o XML (C14N)
        let canonical = self.canonicalize(elem_content)?;

        // Calcular digest (SHA-256) do conteúdo canonicalizado
        let digest = self.calculate_digest(&canonical);
        let digest_b64 = base64::engine::general_purpose::STANDARD.encode(&digest);

        // Criar SignedInfo
        let signed_info = self.create_signed_info(&id, &digest_b64);

        // Canonicalizar SignedInfo para assinatura
        let signed_info_canonical = self.canonicalize(&signed_info)?;

        // Assinar o SignedInfo com RSA-SHA256
        let signature_value = self.sign_rsa_sha256(&signed_info_canonical)?;
        let signature_b64 = base64::engine::general_purpose::STANDARD.encode(&signature_value);

        // Certificado X509 em base64
        let cert_b64 = self.certificado.cert_base64();

        // Montar elemento Signature completo
        let signature_element = self.create_signature_element(&signed_info, &signature_b64, &cert_b64);

        // Inserir Signature no XML após o elemento assinado
        let result = self.insert_signature(xml, &signature_element, elem_end)?;

        Ok(result)
    }

    /// Assina dados com RSA-SHA256 usando a chave privada do certificado
    fn sign_rsa_sha256(&self, data: &str) -> Result<Vec<u8>, String> {
        // Obter chave privada do certificado
        let private_key = self.certificado.private_key()?;

        // Criar signing key com SHA-256
        let signing_key: SigningKey<Sha256> = SigningKey::new(private_key);

        // Assinar os dados
        let signature = signing_key.sign(data.as_bytes());

        Ok(signature.to_vec())
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

    /// Canonicalização XML (C14N simplificado)
    /// Para produção, considerar usar uma biblioteca C14N completa
    fn canonicalize(&self, xml: &str) -> Result<String, String> {
        let mut result = xml.to_string();

        // Remover declaração XML se presente
        if let Some(_decl_start) = result.find("<?xml") {
            if let Some(decl_end) = result.find("?>") {
                result = result[decl_end + 2..].trim_start().to_string();
            }
        }

        // Normalizar quebras de linha para LF
        result = result.replace("\r\n", "\n").replace("\r", "\n");

        // Remover espaços em branco entre tags
        let re = regex::Regex::new(r">\s+<").unwrap();
        result = re.replace_all(&result, "><").to_string();

        // Normalizar atributos (ordenar e normalizar espaços)
        // Nota: C14N completo requer ordenação lexicográfica de atributos

        Ok(result)
    }

    fn calculate_digest(&self, data: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec()
    }

    fn create_signed_info(&self, reference_id: &str, digest_value: &str) -> String {
        let mut s = String::new();
        s.push_str("<SignedInfo xmlns=\"http://www.w3.org/2000/09/xmldsig#\">");
        s.push_str("<CanonicalizationMethod Algorithm=\"http://www.w3.org/TR/2001/REC-xml-c14n-20010315\"/>");
        s.push_str("<SignatureMethod Algorithm=\"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256\"/>");
        s.push_str("<Reference URI=\"#");
        s.push_str(reference_id);
        s.push_str("\">");
        s.push_str("<Transforms>");
        s.push_str("<Transform Algorithm=\"http://www.w3.org/2000/09/xmldsig#enveloped-signature\"/>");
        s.push_str("<Transform Algorithm=\"http://www.w3.org/TR/2001/REC-xml-c14n-20010315\"/>");
        s.push_str("</Transforms>");
        s.push_str("<DigestMethod Algorithm=\"http://www.w3.org/2001/04/xmlenc#sha256\"/>");
        s.push_str("<DigestValue>");
        s.push_str(digest_value);
        s.push_str("</DigestValue>");
        s.push_str("</Reference>");
        s.push_str("</SignedInfo>");
        s
    }

    fn create_signature_element(&self, signed_info: &str, signature_value: &str, cert_b64: &str) -> String {
        let mut s = String::new();
        s.push_str("<Signature xmlns=\"http://www.w3.org/2000/09/xmldsig#\">");
        s.push_str(signed_info);
        s.push_str("<SignatureValue>");
        s.push_str(signature_value);
        s.push_str("</SignatureValue>");
        s.push_str("<KeyInfo>");
        s.push_str("<X509Data>");
        s.push_str("<X509Certificate>");
        s.push_str(cert_b64);
        s.push_str("</X509Certificate>");
        s.push_str("</X509Data>");
        s.push_str("</KeyInfo>");
        s.push_str("</Signature>");
        s
    }

    fn insert_signature(&self, xml: &str, signature: &str, position: usize) -> Result<String, String> {
        let before = &xml[..position];
        let after = &xml[position..];
        Ok(format!("{}{}{}", before, signature, after))
    }
}

/// Valida estrutura de assinatura de XML (não valida a assinatura RSA criptograficamente)
pub fn validar_estrutura_assinatura(xml: &str) -> Result<bool, String> {
    if !xml.contains("<Signature") {
        return Err("XML não possui assinatura".to_string());
    }

    // Verificar elementos obrigatórios
    let required = ["SignedInfo", "SignatureValue", "DigestValue", "X509Certificate"];
    for elem in required {
        if !xml.contains(&format!("<{}", elem)) {
            return Err(format!("Elemento {} não encontrado na assinatura", elem));
        }
    }

    Ok(true)
}

/// Verifica o digest de um XML assinado (não verifica a assinatura RSA)
pub fn verificar_digest(xml: &str, element_name: &str) -> Result<bool, String> {
    // Extrair DigestValue do XML
    let digest_start = xml.find("<DigestValue>")
        .ok_or("DigestValue não encontrado")?;
    let digest_value_start = digest_start + 13;
    let digest_value_end = xml[digest_value_start..].find("</DigestValue>")
        .ok_or("Fim do DigestValue não encontrado")?;
    let stored_digest = &xml[digest_value_start..digest_value_start + digest_value_end];

    // Encontrar e canonicalizar o elemento assinado
    let start_tag = format!("<{}", element_name);
    let end_tag = format!("</{}>", element_name);

    let elem_start = xml.find(&start_tag)
        .ok_or(format!("Elemento {} não encontrado", element_name))?;
    let elem_end = xml.find(&end_tag)
        .ok_or(format!("Fim do elemento {} não encontrado", element_name))?;

    let elem_content = &xml[elem_start..elem_end + end_tag.len()];

    // Canonicalizar (simplificado)
    let canonical = elem_content
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let re = regex::Regex::new(r">\s+<").unwrap();
    let canonical = re.replace_all(&canonical, "><").to_string();

    // Calcular digest
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let calculated_digest = hasher.finalize();
    let calculated_b64 = base64::engine::general_purpose::STANDARD.encode(&calculated_digest);

    Ok(stored_digest == calculated_b64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_estrutura_assinatura() {
        let xml_com_assinatura = r#"
            <NFe>
                <infNFe Id="NFe123">...</infNFe>
                <Signature xmlns="http://www.w3.org/2000/09/xmldsig#">
                    <SignedInfo>
                        <DigestValue>abc123</DigestValue>
                    </SignedInfo>
                    <SignatureValue>xyz789</SignatureValue>
                    <KeyInfo>
                        <X509Data>
                            <X509Certificate>cert</X509Certificate>
                        </X509Data>
                    </KeyInfo>
                </Signature>
            </NFe>
        "#;

        assert!(validar_estrutura_assinatura(xml_com_assinatura).is_ok());
    }

    #[test]
    fn test_xml_sem_assinatura() {
        let xml_sem_assinatura = "<NFe><infNFe>...</infNFe></NFe>";
        assert!(validar_estrutura_assinatura(xml_sem_assinatura).is_err());
    }
}
