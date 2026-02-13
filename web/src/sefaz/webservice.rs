//! WebServices SEFAZ
//!
//! Implementa comunicação SOAP com os WebServices da SEFAZ

use super::consulta::ResultadoConsulta;
use crate::certificado::{CertificadoA1, AssinadorXml};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Ambiente de operação
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmbienteNfe {
    Producao = 1,
    Homologacao = 2,
}

/// Serviços disponíveis
#[derive(Debug, Clone, Copy)]
pub enum ServicoNfe {
    Autorizacao,
    RetAutorizacao,
    ConsultaProtocolo,
    Inutilizacao,
    RecepcaoEvento,
    StatusServico,
}

/// URLs dos WebServices por UF
pub struct WebServiceUrls {
    pub uf: String,
    pub ambiente: AmbienteNfe,
}

impl WebServiceUrls {
    pub fn new(uf: &str, ambiente: AmbienteNfe) -> Self {
        Self {
            uf: uf.to_uppercase(),
            ambiente,
        }
    }

    pub fn get_url(&self, servico: ServicoNfe) -> String {
        let base = self.get_base_url();
        let service = match servico {
            ServicoNfe::Autorizacao => "NFeAutorizacao4",
            ServicoNfe::RetAutorizacao => "NFeRetAutorizacao4",
            ServicoNfe::ConsultaProtocolo => "NFeConsultaProtocolo4",
            ServicoNfe::Inutilizacao => "NFeInutilizacao4",
            ServicoNfe::RecepcaoEvento => "NFeRecepcaoEvento4",
            ServicoNfe::StatusServico => "NFeStatusServico4",
        };
        format!("{}/{}", base, service)
    }

    fn get_base_url(&self) -> String {
        let is_prod = self.ambiente == AmbienteNfe::Producao;
        match self.uf.as_str() {
            "SP" => if is_prod { "https://nfe.fazenda.sp.gov.br/ws" } else { "https://homologacao.nfe.fazenda.sp.gov.br/ws" },
            "RS" => if is_prod { "https://nfe.sefazrs.rs.gov.br/ws" } else { "https://nfe-homologacao.sefazrs.rs.gov.br/ws" },
            "MG" => if is_prod { "https://nfe.fazenda.mg.gov.br/nfe2/services" } else { "https://hnfe.fazenda.mg.gov.br/nfe2/services" },
            "PR" => if is_prod { "https://nfe.sefa.pr.gov.br/nfe/NFeServices" } else { "https://homologacao.nfe.sefa.pr.gov.br/nfe/NFeServices" },
            _ => if is_prod { "https://nfe.svrs.rs.gov.br/ws" } else { "https://nfe-homologacao.svrs.rs.gov.br/ws" },
        }.to_string()
    }
}

/// Cliente SEFAZ
pub struct SefazClient {
    certificado: CertificadoA1,
    http_client: Client,
    ambiente: AmbienteNfe,
    uf: String,
}

impl SefazClient {
    /// Cria novo cliente SEFAZ
    pub fn new(certificado: CertificadoA1, uf: &str, ambiente: AmbienteNfe) -> Result<Self, String> {
        // Criar cliente HTTP com certificado PFX
        let identity = reqwest::Identity::from_pkcs12_der(
            certificado.pfx_bytes(),
            certificado.senha()
        ).map_err(|e| format!("Erro ao criar identidade: {}", e))?;

        let http_client = Client::builder()
            .identity(identity)
            .danger_accept_invalid_certs(false)
            .build()
            .map_err(|e| format!("Erro ao criar cliente HTTP: {}", e))?;

        Ok(Self {
            certificado,
            http_client,
            ambiente,
            uf: uf.to_uppercase(),
        })
    }

    /// Consulta status do serviço SEFAZ
    pub async fn status_servico(&self) -> Result<StatusServicoResult, String> {
        let urls = WebServiceUrls::new(&self.uf, self.ambiente);
        let url = urls.get_url(ServicoNfe::StatusServico);
        let envelope = self.criar_envelope_status();

        let response = self.enviar_soap(&url, &envelope, "nfeStatusServicoNF").await?;
        self.parsear_status_servico(&response)
    }

    /// Consulta NF-e por chave de acesso
    pub async fn consultar_nfe(&self, chave_acesso: &str) -> Result<ResultadoConsulta, String> {
        let urls = WebServiceUrls::new(&self.uf, self.ambiente);
        let url = urls.get_url(ServicoNfe::ConsultaProtocolo);
        let envelope = self.criar_envelope_consulta(chave_acesso);

        let response = self.enviar_soap(&url, &envelope, "nfeConsultaNF").await?;
        self.parsear_consulta(&response)
    }

    /// Envia NF-e para autorização
    pub async fn autorizar_nfe(&self, xml_nfe: &str) -> Result<AutorizacaoResult, String> {
        let urls = WebServiceUrls::new(&self.uf, self.ambiente);
        let url = urls.get_url(ServicoNfe::Autorizacao);

        // Assinar XML
        let assinador = AssinadorXml::new(self.certificado.clone());
        let xml_assinado = assinador.assinar_nfe(xml_nfe)?;

        // Criar lote
        let lote_id = uuid::Uuid::new_v4().to_string().replace("-", "")[..15].to_string();
        let xml_lote = self.criar_lote_nfe(&lote_id, &xml_assinado);
        let envelope = self.criar_envelope_autorizacao(&xml_lote);

        let response = self.enviar_soap(&url, &envelope, "nfeAutorizacaoLote").await?;
        self.parsear_autorizacao(&response)
    }

    /// Cancela NF-e
    pub async fn cancelar_nfe(
        &self,
        chave_acesso: &str,
        protocolo: &str,
        justificativa: &str,
    ) -> Result<EventoResult, String> {
        let urls = WebServiceUrls::new(&self.uf, self.ambiente);
        let url = urls.get_url(ServicoNfe::RecepcaoEvento);

        let xml_evento = self.criar_evento_cancelamento(chave_acesso, protocolo, justificativa)?;
        let assinador = AssinadorXml::new(self.certificado.clone());
        let xml_assinado = assinador.assinar_evento(&xml_evento)?;
        let envelope = self.criar_envelope_evento(&xml_assinado);

        let response = self.enviar_soap(&url, &envelope, "nfeRecepcaoEvento").await?;
        self.parsear_evento(&response)
    }

    /// Envia carta de correção
    pub async fn carta_correcao(
        &self,
        chave_acesso: &str,
        sequencia: u32,
        correcao: &str,
    ) -> Result<EventoResult, String> {
        let urls = WebServiceUrls::new(&self.uf, self.ambiente);
        let url = urls.get_url(ServicoNfe::RecepcaoEvento);

        let xml_evento = self.criar_evento_cce(chave_acesso, sequencia, correcao)?;
        let assinador = AssinadorXml::new(self.certificado.clone());
        let xml_assinado = assinador.assinar_evento(&xml_evento)?;
        let envelope = self.criar_envelope_evento(&xml_assinado);

        let response = self.enviar_soap(&url, &envelope, "nfeRecepcaoEvento").await?;
        self.parsear_evento(&response)
    }

    async fn enviar_soap(&self, url: &str, envelope: &str, action: &str) -> Result<String, String> {
        let response = self.http_client
            .post(url)
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("SOAPAction", action)
            .body(envelope.to_string())
            .send()
            .await
            .map_err(|e| format!("Erro na requisição SOAP: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("SEFAZ retornou erro HTTP: {}", response.status()));
        }

        response.text().await.map_err(|e| format!("Erro ao ler resposta: {}", e))
    }

    fn criar_envelope_status(&self) -> String {
        let tp_amb = if self.ambiente == AmbienteNfe::Producao { "1" } else { "2" };
        let c_uf = self.get_codigo_uf();
        format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<soap12:Envelope xmlns:soap12="http://www.w3.org/2003/05/soap-envelope">
  <soap12:Body>
    <nfeDadosMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeStatusServico4">
      <consStatServ versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe">
        <tpAmb>{tp_amb}</tpAmb>
        <cUF>{c_uf}</cUF>
        <xServ>STATUS</xServ>
      </consStatServ>
    </nfeDadosMsg>
  </soap12:Body>
</soap12:Envelope>"#)
    }

    fn criar_envelope_consulta(&self, chave_acesso: &str) -> String {
        let tp_amb = if self.ambiente == AmbienteNfe::Producao { "1" } else { "2" };
        format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<soap12:Envelope xmlns:soap12="http://www.w3.org/2003/05/soap-envelope">
  <soap12:Body>
    <nfeDadosMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeConsultaProtocolo4">
      <consSitNFe versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe">
        <tpAmb>{tp_amb}</tpAmb>
        <xServ>CONSULTAR</xServ>
        <chNFe>{chave_acesso}</chNFe>
      </consSitNFe>
    </nfeDadosMsg>
  </soap12:Body>
</soap12:Envelope>"#)
    }

    fn criar_lote_nfe(&self, lote_id: &str, xml_nfe: &str) -> String {
        format!(r#"<enviNFe versao="4.00" xmlns="http://www.portalfiscal.inf.br/nfe">
  <idLote>{lote_id}</idLote>
  <indSinc>1</indSinc>
  {xml_nfe}
</enviNFe>"#)
    }

    fn criar_envelope_autorizacao(&self, xml_lote: &str) -> String {
        format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<soap12:Envelope xmlns:soap12="http://www.w3.org/2003/05/soap-envelope">
  <soap12:Body>
    <nfeDadosMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4">
      {xml_lote}
    </nfeDadosMsg>
  </soap12:Body>
</soap12:Envelope>"#)
    }

    fn criar_evento_cancelamento(&self, chave_acesso: &str, protocolo: &str, justificativa: &str) -> Result<String, String> {
        let tp_amb = if self.ambiente == AmbienteNfe::Producao { "1" } else { "2" };
        let cnpj = self.certificado.info.cnpj.clone().unwrap_or_default();
        let dh_evento = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S-03:00").to_string();
        let c_orgao = &chave_acesso[0..2];

        Ok(format!(r#"<evento versao="1.00" xmlns="http://www.portalfiscal.inf.br/nfe">
  <infEvento Id="ID110111{chave_acesso}01">
    <cOrgao>{c_orgao}</cOrgao>
    <tpAmb>{tp_amb}</tpAmb>
    <CNPJ>{cnpj}</CNPJ>
    <chNFe>{chave_acesso}</chNFe>
    <dhEvento>{dh_evento}</dhEvento>
    <tpEvento>110111</tpEvento>
    <nSeqEvento>1</nSeqEvento>
    <verEvento>1.00</verEvento>
    <detEvento versao="1.00">
      <descEvento>Cancelamento</descEvento>
      <nProt>{protocolo}</nProt>
      <xJust>{justificativa}</xJust>
    </detEvento>
  </infEvento>
</evento>"#))
    }

    fn criar_evento_cce(&self, chave_acesso: &str, sequencia: u32, correcao: &str) -> Result<String, String> {
        let tp_amb = if self.ambiente == AmbienteNfe::Producao { "1" } else { "2" };
        let cnpj = self.certificado.info.cnpj.clone().unwrap_or_default();
        let dh_evento = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S-03:00").to_string();
        let c_orgao = &chave_acesso[0..2];

        Ok(format!(r#"<evento versao="1.00" xmlns="http://www.portalfiscal.inf.br/nfe">
  <infEvento Id="ID110110{chave_acesso}{sequencia:02}">
    <cOrgao>{c_orgao}</cOrgao>
    <tpAmb>{tp_amb}</tpAmb>
    <CNPJ>{cnpj}</CNPJ>
    <chNFe>{chave_acesso}</chNFe>
    <dhEvento>{dh_evento}</dhEvento>
    <tpEvento>110110</tpEvento>
    <nSeqEvento>{sequencia}</nSeqEvento>
    <verEvento>1.00</verEvento>
    <detEvento versao="1.00">
      <descEvento>Carta de Correcao</descEvento>
      <xCorrecao>{correcao}</xCorrecao>
      <xCondUso>A Carta de Correcao e disciplinada pelo paragrafo 1o-A do art. 7o do Convenio S/N, de 15 de dezembro de 1970...</xCondUso>
    </detEvento>
  </infEvento>
</evento>"#))
    }

    fn criar_envelope_evento(&self, xml_evento: &str) -> String {
        format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<soap12:Envelope xmlns:soap12="http://www.w3.org/2003/05/soap-envelope">
  <soap12:Body>
    <nfeDadosMsg xmlns="http://www.portalfiscal.inf.br/nfe/wsdl/NFeRecepcaoEvento4">
      <envEvento versao="1.00" xmlns="http://www.portalfiscal.inf.br/nfe">
        <idLote>1</idLote>
        {xml_evento}
      </envEvento>
    </nfeDadosMsg>
  </soap12:Body>
</soap12:Envelope>"#)
    }

    fn get_codigo_uf(&self) -> &str {
        match self.uf.as_str() {
            "AC" => "12", "AL" => "27", "AP" => "16", "AM" => "13", "BA" => "29",
            "CE" => "23", "DF" => "53", "ES" => "32", "GO" => "52", "MA" => "21",
            "MT" => "51", "MS" => "50", "MG" => "31", "PA" => "15", "PB" => "25",
            "PR" => "41", "PE" => "26", "PI" => "22", "RJ" => "33", "RN" => "24",
            "RS" => "43", "RO" => "11", "RR" => "14", "SC" => "42", "SP" => "35",
            "SE" => "28", "TO" => "17",
            _ => "35",
        }
    }

    fn parsear_status_servico(&self, xml: &str) -> Result<StatusServicoResult, String> {
        let c_stat = extract_xml_value(xml, "cStat").unwrap_or_default();
        let x_motivo = extract_xml_value(xml, "xMotivo").unwrap_or_default();
        let dh_recbto = extract_xml_value(xml, "dhRecbto");
        let t_med = extract_xml_value(xml, "tMed").and_then(|s| s.parse().ok());

        Ok(StatusServicoResult {
            codigo_status: c_stat.clone(),
            motivo: x_motivo,
            data_hora: dh_recbto,
            tempo_medio: t_med,
            online: c_stat == "107",
        })
    }

    fn parsear_consulta(&self, xml: &str) -> Result<ResultadoConsulta, String> {
        let c_stat = extract_xml_value(xml, "cStat").unwrap_or_default();
        let x_motivo = extract_xml_value(xml, "xMotivo").unwrap_or_default();
        let ch_nfe = extract_xml_value(xml, "chNFe");
        let n_prot = extract_xml_value(xml, "nProt");
        let dh_recbto = extract_xml_value(xml, "dhRecbto");

        let situacao = match c_stat.as_str() {
            "100" => Some("Autorizada".to_string()),
            "101" => Some("Cancelada".to_string()),
            "110" => Some("Denegada".to_string()),
            _ => Some(x_motivo.clone()),
        };

        Ok(ResultadoConsulta {
            sucesso: c_stat == "100" || c_stat == "101",
            codigo_status: Some(c_stat),
            motivo: Some(x_motivo),
            chave_acesso: ch_nfe,
            situacao,
            data_autorizacao: dh_recbto,
            protocolo: n_prot,
            numero: None,
            serie: None,
            emit_cnpj: None,
            emit_razao_social: None,
            valor_total: None,
            url_consulta: None,
        })
    }

    fn parsear_autorizacao(&self, xml: &str) -> Result<AutorizacaoResult, String> {
        let c_stat = extract_xml_value(xml, "cStat").unwrap_or_default();
        let x_motivo = extract_xml_value(xml, "xMotivo").unwrap_or_default();
        let ch_nfe = extract_xml_value(xml, "chNFe");
        let n_prot = extract_xml_value(xml, "nProt");

        Ok(AutorizacaoResult {
            sucesso: c_stat == "100" || c_stat == "104",
            codigo_status: c_stat.clone(),
            motivo: x_motivo,
            chave_acesso: ch_nfe,
            protocolo: n_prot,
            xml_autorizado: if c_stat == "100" { Some(xml.to_string()) } else { None },
        })
    }

    fn parsear_evento(&self, xml: &str) -> Result<EventoResult, String> {
        let c_stat = extract_xml_value(xml, "cStat").unwrap_or_default();
        let x_motivo = extract_xml_value(xml, "xMotivo").unwrap_or_default();
        let n_prot = extract_xml_value(xml, "nProt");
        let dh_reg = extract_xml_value(xml, "dhRegEvento");

        Ok(EventoResult {
            sucesso: c_stat == "135" || c_stat == "136",
            codigo_status: c_stat,
            motivo: x_motivo,
            protocolo: n_prot,
            data_evento: dh_reg,
        })
    }
}

// === Tipos de retorno ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusServicoResult {
    pub codigo_status: String,
    pub motivo: String,
    pub data_hora: Option<String>,
    pub tempo_medio: Option<u32>,
    pub online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutorizacaoResult {
    pub sucesso: bool,
    pub codigo_status: String,
    pub motivo: String,
    pub chave_acesso: Option<String>,
    pub protocolo: Option<String>,
    pub xml_autorizado: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoResult {
    pub sucesso: bool,
    pub codigo_status: String,
    pub motivo: String,
    pub protocolo: Option<String>,
    pub data_evento: Option<String>,
}

fn extract_xml_value(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    let start = xml.find(&start_tag)?;
    let value_start = start + start_tag.len();
    let end = xml[value_start..].find(&end_tag)?;
    Some(xml[value_start..value_start + end].to_string())
}
