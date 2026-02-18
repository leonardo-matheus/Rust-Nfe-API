//! Distribuição de DF-e (Documentos Fiscais Eletrônicos)
//!
//! Este módulo implementa o serviço NFeDistribuicaoDFe para consultar
//! documentos fiscais emitidos contra o CNPJ/CPF do interessado.

use serde::{Deserialize, Serialize};

/// Tipos de consulta de distribuição
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TipoConsultaDistribuicao {
    /// Consulta por NSU (Número Sequencial Único)
    ConsultaNsu,
    /// Consulta por chave de acesso
    ConsultaChave,
    /// Consulta por último NSU recebido
    DistribuicaoNsu,
}

/// Dados para consulta de distribuição
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DadosConsultaDistribuicao {
    /// CNPJ ou CPF do interessado
    pub documento: String,
    /// Tipo de consulta
    pub tipo_consulta: TipoConsultaDistribuicao,
    /// NSU para consulta específica
    pub nsu: Option<String>,
    /// Último NSU recebido (para continuar distribuição)
    pub ultimo_nsu: Option<String>,
    /// Chave de acesso (para consulta por chave)
    pub chave_acesso: Option<String>,
}

/// Resultado da consulta de distribuição
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoDistribuicao {
    /// Sucesso na operação
    pub sucesso: bool,
    /// Código do status
    pub codigo_status: u16,
    /// Descrição do status
    pub descricao_status: String,
    /// Último NSU retornado
    pub ultimo_nsu: Option<String>,
    /// Máximo NSU disponível
    pub max_nsu: Option<String>,
    /// Lista de documentos retornados
    pub documentos: Vec<DocumentoDistribuido>,
}

/// Documento retornado na distribuição
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentoDistribuido {
    /// NSU do documento
    pub nsu: String,
    /// Tipo do documento
    pub tipo: TipoDocumentoDistribuido,
    /// Schema do documento
    pub schema: String,
    /// Conteúdo do documento (XML ou resumo)
    pub conteudo: String,
    /// Chave de acesso (se disponível)
    pub chave_acesso: Option<String>,
    /// CNPJ do emitente (se disponível)
    pub cnpj_emitente: Option<String>,
    /// Data de emissão (se disponível)
    pub data_emissao: Option<String>,
    /// Valor total (se disponível)
    pub valor_total: Option<f32>,
}

/// Tipos de documentos retornados na distribuição
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TipoDocumentoDistribuido {
    /// Resumo de NF-e
    ResumoNfe,
    /// NF-e completa
    NfeCompleta,
    /// Resumo de evento
    ResumoEvento,
    /// Evento completo
    EventoCompleto,
}

impl DadosConsultaDistribuicao {
    /// Valida os dados da consulta
    pub fn validar(&self) -> Result<(), Vec<String>> {
        let mut erros = Vec::new();

        // Documento deve ter 11 (CPF) ou 14 (CNPJ) dígitos
        let doc_len = self.documento.len();
        if doc_len != 11 && doc_len != 14 {
            erros.push(format!(
                "Documento inválido: {} dígitos (esperado 11 para CPF ou 14 para CNPJ)",
                doc_len
            ));
        }

        if !self.documento.chars().all(|c| c.is_ascii_digit()) {
            erros.push("Documento deve conter apenas números".to_string());
        }

        // Validações específicas por tipo
        match self.tipo_consulta {
            TipoConsultaDistribuicao::ConsultaNsu => {
                if self.nsu.is_none() {
                    erros.push("NSU é obrigatório para consulta por NSU".to_string());
                }
            }
            TipoConsultaDistribuicao::ConsultaChave => {
                match &self.chave_acesso {
                    None => {
                        erros.push("Chave de acesso é obrigatória para consulta por chave".to_string());
                    }
                    Some(chave) if chave.len() != 44 => {
                        erros.push(format!(
                            "Chave de acesso inválida: {} dígitos (esperado 44)",
                            chave.len()
                        ));
                    }
                    _ => {}
                }
            }
            TipoConsultaDistribuicao::DistribuicaoNsu => {
                // ultimo_nsu é opcional, usa 0 se não informado
            }
        }

        if erros.is_empty() {
            Ok(())
        } else {
            Err(erros)
        }
    }
}

/// Gera o XML da consulta de distribuição
pub fn gerar_xml_distribuicao(dados: &DadosConsultaDistribuicao, codigo_uf: u8, ambiente: u8) -> String {
    let is_cnpj = dados.documento.len() == 14;
    let doc_tag = if is_cnpj { "CNPJ" } else { "CPF" };

    let consulta = match dados.tipo_consulta {
        TipoConsultaDistribuicao::ConsultaNsu => {
            format!(
                r#"<consNSU>
            <NSU>{}</NSU>
        </consNSU>"#,
                dados.nsu.as_deref().unwrap_or("0")
            )
        }
        TipoConsultaDistribuicao::ConsultaChave => {
            format!(
                r#"<consChNFe>
            <chNFe>{}</chNFe>
        </consChNFe>"#,
                dados.chave_acesso.as_deref().unwrap_or("")
            )
        }
        TipoConsultaDistribuicao::DistribuicaoNsu => {
            format!(
                r#"<distNSU>
            <ultNSU>{}</ultNSU>
        </distNSU>"#,
                dados.ultimo_nsu.as_deref().unwrap_or("0").trim_start_matches('0')
            )
        }
    };

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<distDFeInt xmlns="http://www.portalfiscal.inf.br/nfe" versao="1.01">
    <tpAmb>{ambiente}</tpAmb>
    <cUFAutor>{codigo_uf:02}</cUFAutor>
    <{doc_tag}>{documento}</{doc_tag}>
    {consulta}
</distDFeInt>"#,
        ambiente = ambiente,
        codigo_uf = codigo_uf,
        doc_tag = doc_tag,
        documento = dados.documento,
        consulta = consulta,
    )
}

/// Parseia a resposta da distribuição
pub fn parsear_resposta_distribuicao(xml: &str) -> ResultadoDistribuicao {
    let codigo_status = extrair_tag(xml, "cStat")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let descricao = extrair_tag(xml, "xMotivo")
        .unwrap_or_else(|| "Erro desconhecido".to_string());

    let ultimo_nsu = extrair_tag(xml, "ultNSU");
    let max_nsu = extrair_tag(xml, "maxNSU");

    // Extrai documentos do lote
    let documentos = extrair_documentos(xml);

    // Códigos de sucesso: 137 (Nenhum documento localizado), 138 (Documento localizado)
    let sucesso = codigo_status == 137 || codigo_status == 138;

    ResultadoDistribuicao {
        sucesso,
        codigo_status,
        descricao_status: descricao,
        ultimo_nsu,
        max_nsu,
        documentos,
    }
}

fn extrair_documentos(xml: &str) -> Vec<DocumentoDistribuido> {
    let mut docs = Vec::new();

    // Procura por tags <docZip>
    let mut pos = 0;
    while let Some(start) = xml[pos..].find("<docZip") {
        let start = pos + start;
        if let Some(end) = xml[start..].find("</docZip>") {
            let doc_xml = &xml[start..start + end + 9];

            // Extrai NSU
            let nsu = extrair_atributo_str(doc_xml, "NSU")
                .unwrap_or_default();

            // Extrai schema
            let schema = extrair_atributo_str(doc_xml, "schema")
                .unwrap_or_default();

            // Determina o tipo
            let tipo = if schema.contains("resNFe") {
                TipoDocumentoDistribuido::ResumoNfe
            } else if schema.contains("procNFe") || schema.contains("nfeProc") {
                TipoDocumentoDistribuido::NfeCompleta
            } else if schema.contains("resEvento") {
                TipoDocumentoDistribuido::ResumoEvento
            } else {
                TipoDocumentoDistribuido::EventoCompleto
            };

            // Extrai conteúdo (base64 compactado)
            let conteudo = extrair_conteudo_tag(doc_xml, "docZip")
                .unwrap_or_default();

            docs.push(DocumentoDistribuido {
                nsu,
                tipo,
                schema,
                conteudo,
                chave_acesso: None, // Extrair do conteúdo decodificado
                cnpj_emitente: None,
                data_emissao: None,
                valor_total: None,
            });

            pos = start + end + 9;
        } else {
            break;
        }
    }

    docs
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

fn extrair_atributo_str(xml: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    if let Some(start) = xml.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = xml[start..].find('"') {
            return Some(xml[start..start + end].to_string());
        }
    }
    None
}

fn extrair_conteudo_tag(xml: &str, tag: &str) -> Option<String> {
    if let Some(start) = xml.find('>') {
        let start = start + 1;
        let fim = format!("</{}>", tag);
        if let Some(end) = xml[start..].find(&fim) {
            return Some(xml[start..start + end].to_string());
        }
    }
    None
}
