//! Inutilização de Numeração de NF-e
//!
//! Este módulo implementa o serviço NFeInutilizacao4 para inutilizar
//! faixas de numeração não utilizadas.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Dados para inutilização de numeração
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DadosInutilizacao {
    /// Ano de emissão (AAAA)
    pub ano: u16,
    /// CNPJ do emitente
    pub cnpj: String,
    /// Modelo do documento (55=NF-e, 65=NFC-e)
    pub modelo: u8,
    /// Série do documento
    pub serie: u16,
    /// Número inicial da faixa
    pub numero_inicial: u32,
    /// Número final da faixa
    pub numero_final: u32,
    /// Justificativa (mínimo 15 caracteres)
    pub justificativa: String,
}

/// Resultado da inutilização
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoInutilizacao {
    /// Sucesso na operação
    pub sucesso: bool,
    /// ID da inutilização
    pub id_inutilizacao: String,
    /// Código do status
    pub codigo_status: u16,
    /// Descrição do status
    pub descricao_status: String,
    /// Número do protocolo (se autorizado)
    pub protocolo: Option<String>,
    /// Data/hora do processamento
    pub data_processamento: Option<String>,
    /// XML assinado
    pub xml_assinado: Option<String>,
}

impl DadosInutilizacao {
    /// Valida os dados de inutilização
    pub fn validar(&self) -> Result<(), Vec<String>> {
        let mut erros = Vec::new();

        // Ano deve estar no formato correto
        if self.ano < 2006 || self.ano > 2099 {
            erros.push(format!("Ano inválido: {}", self.ano));
        }

        // CNPJ deve ter 14 dígitos
        if self.cnpj.len() != 14 || !self.cnpj.chars().all(|c| c.is_ascii_digit()) {
            erros.push(format!("CNPJ inválido: {}", self.cnpj));
        }

        // Modelo deve ser 55 ou 65
        if self.modelo != 55 && self.modelo != 65 {
            erros.push(format!("Modelo inválido: {} (use 55 para NF-e ou 65 para NFC-e)", self.modelo));
        }

        // Série deve estar entre 0 e 999
        if self.serie > 999 {
            erros.push(format!("Série inválida: {} (máximo 999)", self.serie));
        }

        // Números devem estar em sequência válida
        if self.numero_inicial > self.numero_final {
            erros.push(format!(
                "Número inicial ({}) maior que o final ({})",
                self.numero_inicial, self.numero_final
            ));
        }

        if self.numero_inicial == 0 {
            erros.push("Número inicial não pode ser zero".to_string());
        }

        // Justificativa mínima de 15 caracteres
        if self.justificativa.len() < 15 {
            erros.push(format!(
                "Justificativa muito curta ({} caracteres, mínimo 15)",
                self.justificativa.len()
            ));
        }

        if self.justificativa.len() > 255 {
            erros.push(format!(
                "Justificativa muito longa ({} caracteres, máximo 255)",
                self.justificativa.len()
            ));
        }

        if erros.is_empty() {
            Ok(())
        } else {
            Err(erros)
        }
    }

    /// Gera o ID da inutilização (43 dígitos)
    /// Formato: ID + cUF + AAAA + CNPJ + mod + serie + nIni + nFin
    pub fn gerar_id(&self, codigo_uf: u8) -> String {
        format!(
            "ID{:02}{:04}{}{:02}{:03}{:09}{:09}",
            codigo_uf,
            self.ano,
            self.cnpj,
            self.modelo,
            self.serie,
            self.numero_inicial,
            self.numero_final
        )
    }
}

/// Gera o XML de inutilização
pub fn gerar_xml_inutilizacao(dados: &DadosInutilizacao, codigo_uf: u8, ambiente: u8) -> String {
    let id = dados.gerar_id(codigo_uf);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<inutNFe xmlns="http://www.portalfiscal.inf.br/nfe" versao="4.00">
    <infInut Id="{id}">
        <tpAmb>{ambiente}</tpAmb>
        <xServ>INUTILIZAR</xServ>
        <cUF>{codigo_uf:02}</cUF>
        <ano>{ano}</ano>
        <CNPJ>{cnpj}</CNPJ>
        <mod>{modelo:02}</mod>
        <serie>{serie}</serie>
        <nNFIni>{numero_inicial}</nNFIni>
        <nNFFin>{numero_final}</nNFFin>
        <xJust>{justificativa}</xJust>
    </infInut>
</inutNFe>"#,
        id = id,
        ambiente = ambiente,
        codigo_uf = codigo_uf,
        ano = dados.ano,
        cnpj = dados.cnpj,
        modelo = dados.modelo,
        serie = dados.serie,
        numero_inicial = dados.numero_inicial,
        numero_final = dados.numero_final,
        justificativa = dados.justificativa,
    )
}

/// Parseia a resposta da inutilização
pub fn parsear_resposta_inutilizacao(xml: &str) -> ResultadoInutilizacao {
    // Extrai o código de status
    let codigo_status = extrair_tag(xml, "cStat")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    // Extrai a descrição
    let descricao = extrair_tag(xml, "xMotivo")
        .unwrap_or_else(|| "Erro desconhecido".to_string());

    // Extrai o protocolo
    let protocolo = extrair_tag(xml, "nProt");

    // Extrai a data de processamento
    let data = extrair_tag(xml, "dhRecbto");

    // Extrai o ID
    let id = extrair_tag(xml, "Id")
        .or_else(|| extrair_atributo(xml, "infInut", "Id"))
        .unwrap_or_default();

    // Status 102 = Inutilização homologada
    let sucesso = codigo_status == 102;

    ResultadoInutilizacao {
        sucesso,
        id_inutilizacao: id,
        codigo_status,
        descricao_status: descricao,
        protocolo,
        data_processamento: data,
        xml_assinado: None,
    }
}

fn extrair_tag(xml: &str, tag: &str) -> Option<String> {
    let inicio = format!("<{}>", tag);
    let fim = format!("</{}>", tag);

    if let Some(start) = xml.find(&inicio) {
        let start = start + inicio.len();
        if let Some(end) = xml[start..].find(&fim) {
            return Some(xml[start..start + end].to_string());
        }
    }
    None
}

fn extrair_atributo(xml: &str, tag: &str, attr: &str) -> Option<String> {
    let pattern = format!("<{}", tag);
    if let Some(start) = xml.find(&pattern) {
        let tag_content = &xml[start..];
        if let Some(end) = tag_content.find('>') {
            let attr_section = &tag_content[..end];
            let attr_pattern = format!("{}=\"", attr);
            if let Some(attr_start) = attr_section.find(&attr_pattern) {
                let value_start = attr_start + attr_pattern.len();
                if let Some(value_end) = attr_section[value_start..].find('"') {
                    return Some(attr_section[value_start..value_start + value_end].to_string());
                }
            }
        }
    }
    None
}
