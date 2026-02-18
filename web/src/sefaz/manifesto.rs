//! Manifesto do Destinatário
//!
//! Este módulo implementa os eventos de manifestação do destinatário:
//! - Ciência da Operação
//! - Confirmação da Operação
//! - Desconhecimento da Operação
//! - Operação não Realizada

use serde::{Deserialize, Serialize};

/// Tipos de manifestação do destinatário
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum TipoManifestacao {
    /// Ciência da Operação - O destinatário declara ter ciência da operação
    /// Não representa concordância, apenas ciência
    CienciaOperacao = 210210,

    /// Confirmação da Operação - O destinatário confirma a operação e o recebimento da mercadoria
    ConfirmacaoOperacao = 210200,

    /// Desconhecimento da Operação - O destinatário não reconhece a operação
    DesconhecimentoOperacao = 210220,

    /// Operação não Realizada - O destinatário declara que a operação não foi realizada
    /// (ex: devolução, cancelamento, etc.)
    OperacaoNaoRealizada = 210240,
}

impl TipoManifestacao {
    /// Retorna a descrição do tipo de manifestação
    pub fn descricao(&self) -> &'static str {
        match self {
            Self::CienciaOperacao => "Ciência da Operação",
            Self::ConfirmacaoOperacao => "Confirmação da Operação",
            Self::DesconhecimentoOperacao => "Desconhecimento da Operação",
            Self::OperacaoNaoRealizada => "Operação não Realizada",
        }
    }

    /// Retorna o código do evento
    pub fn codigo(&self) -> u32 {
        *self as u32
    }

    /// Indica se a justificativa é obrigatória
    pub fn justificativa_obrigatoria(&self) -> bool {
        matches!(self, Self::OperacaoNaoRealizada)
    }

    /// Converte de código para tipo
    pub fn from_codigo(codigo: u32) -> Option<Self> {
        match codigo {
            210210 => Some(Self::CienciaOperacao),
            210200 => Some(Self::ConfirmacaoOperacao),
            210220 => Some(Self::DesconhecimentoOperacao),
            210240 => Some(Self::OperacaoNaoRealizada),
            _ => None,
        }
    }
}

/// Dados para manifestação do destinatário
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DadosManifestacao {
    /// Chave de acesso da NF-e (44 dígitos)
    pub chave_acesso: String,
    /// CNPJ do destinatário
    pub cnpj_destinatario: String,
    /// Tipo de manifestação
    pub tipo_manifestacao: TipoManifestacao,
    /// Justificativa (obrigatória para Operação não Realizada)
    pub justificativa: Option<String>,
}

/// Resultado da manifestação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoManifestacao {
    /// Sucesso na operação
    pub sucesso: bool,
    /// Código do status
    pub codigo_status: u16,
    /// Descrição do status
    pub descricao_status: String,
    /// Número do protocolo
    pub protocolo: Option<String>,
    /// Data/hora do registro
    pub data_registro: Option<String>,
    /// Tipo de evento registrado
    pub tipo_evento: u32,
}

impl DadosManifestacao {
    /// Valida os dados da manifestação
    pub fn validar(&self) -> Result<(), Vec<String>> {
        let mut erros = Vec::new();

        // Chave deve ter 44 dígitos
        if self.chave_acesso.len() != 44 {
            erros.push(format!(
                "Chave de acesso inválida: {} dígitos (esperado 44)",
                self.chave_acesso.len()
            ));
        }

        if !self.chave_acesso.chars().all(|c| c.is_ascii_digit()) {
            erros.push("Chave de acesso deve conter apenas números".to_string());
        }

        // CNPJ deve ter 14 dígitos
        if self.cnpj_destinatario.len() != 14 {
            erros.push(format!(
                "CNPJ inválido: {} dígitos (esperado 14)",
                self.cnpj_destinatario.len()
            ));
        }

        // Justificativa obrigatória para Operação não Realizada
        if self.tipo_manifestacao.justificativa_obrigatoria() {
            match &self.justificativa {
                None => {
                    erros.push("Justificativa é obrigatória para Operação não Realizada".to_string());
                }
                Some(j) if j.len() < 15 => {
                    erros.push(format!(
                        "Justificativa muito curta ({} caracteres, mínimo 15)",
                        j.len()
                    ));
                }
                Some(j) if j.len() > 255 => {
                    erros.push(format!(
                        "Justificativa muito longa ({} caracteres, máximo 255)",
                        j.len()
                    ));
                }
                _ => {}
            }
        }

        if erros.is_empty() {
            Ok(())
        } else {
            Err(erros)
        }
    }

    /// Gera o ID do evento
    /// Formato: ID + tpEvento + chNFe + nSeqEvento
    pub fn gerar_id(&self, sequencia: u8) -> String {
        format!(
            "ID{}{}{}",
            self.tipo_manifestacao.codigo(),
            self.chave_acesso,
            format!("{:02}", sequencia)
        )
    }
}

/// Gera o XML do evento de manifestação
pub fn gerar_xml_manifestacao(dados: &DadosManifestacao, ambiente: u8, sequencia: u8) -> String {
    let id = dados.gerar_id(sequencia);
    let tipo_evento = dados.tipo_manifestacao.codigo();
    let desc_evento = dados.tipo_manifestacao.descricao();

    // Extrai código da UF da chave de acesso (posições 0-1)
    let codigo_uf = &dados.chave_acesso[0..2];

    let det_evento = if let Some(just) = &dados.justificativa {
        format!(
            r#"<detEvento versao="1.00">
            <descEvento>{}</descEvento>
            <xJust>{}</xJust>
        </detEvento>"#,
            desc_evento, just
        )
    } else {
        format!(
            r#"<detEvento versao="1.00">
            <descEvento>{}</descEvento>
        </detEvento>"#,
            desc_evento
        )
    };

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<envEvento xmlns="http://www.portalfiscal.inf.br/nfe" versao="1.00">
    <idLote>1</idLote>
    <evento versao="1.00">
        <infEvento Id="{id}">
            <cOrgao>{codigo_uf}</cOrgao>
            <tpAmb>{ambiente}</tpAmb>
            <CNPJ>{cnpj}</CNPJ>
            <chNFe>{chave}</chNFe>
            <dhEvento>{data}</dhEvento>
            <tpEvento>{tipo_evento}</tpEvento>
            <nSeqEvento>{sequencia}</nSeqEvento>
            <verEvento>1.00</verEvento>
            {det_evento}
        </infEvento>
    </evento>
</envEvento>"#,
        id = id,
        codigo_uf = codigo_uf,
        ambiente = ambiente,
        cnpj = dados.cnpj_destinatario,
        chave = dados.chave_acesso,
        data = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S-03:00"),
        tipo_evento = tipo_evento,
        sequencia = sequencia,
        det_evento = det_evento,
    )
}

/// Parseia a resposta do evento de manifestação
pub fn parsear_resposta_manifestacao(xml: &str) -> ResultadoManifestacao {
    let codigo_status = extrair_tag(xml, "cStat")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let descricao = extrair_tag(xml, "xMotivo")
        .unwrap_or_else(|| "Erro desconhecido".to_string());

    let protocolo = extrair_tag(xml, "nProt");
    let data_registro = extrair_tag(xml, "dhRegEvento");

    let tipo_evento = extrair_tag(xml, "tpEvento")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    // Códigos de sucesso: 135 (Evento registrado e vinculado a NF-e)
    //                     136 (Evento registrado, mas não vinculado)
    let sucesso = codigo_status == 135 || codigo_status == 136;

    ResultadoManifestacao {
        sucesso,
        codigo_status,
        descricao_status: descricao,
        protocolo,
        data_registro,
        tipo_evento,
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
