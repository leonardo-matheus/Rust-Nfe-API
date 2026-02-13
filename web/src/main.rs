//! Servidor web para NF-e
//!
//! Funcionalidades:
//! - Parse de XML
//! - Geração de XML
//! - Exportação JSON e PDF
//! - Leitura de DANFE em PDF
//! - Armazenamento em PostgreSQL/MySQL
//! - Auto-save

mod db;
mod pdf;
mod sefaz;
mod certificado;
mod graphql;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, middleware};
use actix_multipart::Multipart;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use futures_util::StreamExt;
use nfe_parser::{Nfe, NfeBuilder, ItemBuilder};
use pdf::{extract_danfe_data, DanfeData};
use sefaz::{validar_chave_acesso, consultar_portal_publico, gerar_url_consulta_portal, ChaveAcessoInfo};
use nfe_parser::base::endereco::Endereco;
use nfe_parser::base::transporte::ModalidadeFrete;
use nfe_parser::{TipoAmbiente, TipoOperacao, DestinoOperacao};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use db::{NfeRecord, postgres::PostgresClient, mysql::MysqlClient};

// ============================================================================
// Estruturas de dados
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub postgres: Option<Arc<PostgresClient>>,
    pub mysql: Option<Arc<MysqlClient>>,
    pub auto_save: bool,
}

#[derive(Deserialize)]
struct ParseRequest {
    xml: String,
    #[serde(default)]
    auto_save: bool,
}

#[derive(Serialize)]
struct NfeResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<NfeData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saved: Option<bool>,
}

#[derive(Serialize, Clone)]
struct NfeData {
    chave_acesso: String,
    versao: String,
    identificacao: IdentificacaoData,
    emitente: EmitenteData,
    destinatario: Option<DestinatarioData>,
    itens: Vec<ItemData>,
    totais: TotaisData,
    transporte: TransporteData,
    informacao_complementar: Option<String>,
}

#[derive(Serialize, Clone)]
struct IdentificacaoData {
    numero: u32,
    serie: u16,
    modelo: String,
    ambiente: String,
    natureza_operacao: String,
    tipo_operacao: String,
    destino_operacao: String,
    finalidade: String,
    data_emissao: String,
}

#[derive(Serialize, Clone)]
struct EmitenteData {
    cnpj: Option<String>,
    razao_social: Option<String>,
    nome_fantasia: Option<String>,
    inscricao_estadual: Option<String>,
    endereco: EnderecoData,
}

#[derive(Serialize, Clone)]
struct DestinatarioData {
    cnpj: String,
    razao_social: Option<String>,
    indicador_ie: String,
    endereco: Option<EnderecoData>,
}

#[derive(Serialize, Clone)]
struct EnderecoData {
    logradouro: String,
    numero: String,
    complemento: Option<String>,
    bairro: String,
    municipio: String,
    uf: String,
    cep: Option<String>,
}

#[derive(Serialize, Clone)]
struct ItemData {
    numero: u8,
    codigo: String,
    descricao: String,
    ncm: String,
    cfop: String,
    unidade: String,
    quantidade: f32,
    valor_unitario: f32,
    valor_bruto: f32,
    valor_desconto: Option<f32>,
    gtin: Option<String>,
}

#[derive(Serialize, Clone)]
struct TotaisData {
    valor_produtos: f32,
    valor_frete: f32,
    valor_seguro: f32,
    valor_desconto: f32,
    valor_outros: f32,
    valor_total: f32,
    base_calculo_icms: f32,
    valor_icms: f32,
    valor_pis: f32,
    valor_cofins: f32,
    valor_aproximado_tributos: f32,
}

#[derive(Serialize, Clone)]
struct TransporteData {
    modalidade: String,
}

#[derive(Deserialize)]
struct GenerateRequest {
    codigo_uf: u8,
    numero: u32,
    serie: Option<u16>,
    natureza_operacao: String,
    ambiente: String,
    codigo_municipio: u32,
    emit_cnpj: String,
    emit_razao_social: String,
    emit_nome_fantasia: Option<String>,
    emit_ie: String,
    emit_logradouro: String,
    emit_numero: String,
    emit_bairro: String,
    emit_municipio: String,
    emit_uf: String,
    emit_cep: String,
    dest_cnpj: Option<String>,
    dest_razao_social: Option<String>,
    dest_logradouro: Option<String>,
    dest_numero: Option<String>,
    dest_bairro: Option<String>,
    dest_municipio: Option<String>,
    dest_uf: Option<String>,
    dest_cep: Option<String>,
    itens: Vec<ItemRequest>,
    modalidade_frete: Option<u8>,
    informacao_complementar: Option<String>,
    #[serde(default)]
    auto_save: bool,
}

#[derive(Deserialize)]
struct ItemRequest {
    codigo: String,
    descricao: String,
    ncm: String,
    cfop: String,
    unidade: String,
    quantidade: f32,
    valor_unitario: f32,
}

#[derive(Deserialize)]
struct DbQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Serialize)]
struct ListResponse {
    success: bool,
    data: Vec<NfeRecord>,
    total: usize,
}

#[derive(Serialize)]
struct PdfReadResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<DanfeData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Deserialize)]
struct PdfBase64Request {
    pdf_base64: String,
}

// ============================================================================
// Funções auxiliares
// ============================================================================

fn nfe_to_data(nfe: &Nfe) -> NfeData {
    NfeData {
        chave_acesso: nfe.chave_acesso.clone(),
        versao: format!("{:?}", nfe.versao),
        identificacao: IdentificacaoData {
            numero: nfe.ide.numero,
            serie: nfe.ide.serie,
            modelo: format!("{:?}", nfe.ide.modelo),
            ambiente: format!("{:?}", nfe.ide.ambiente),
            natureza_operacao: nfe.ide.operacao.natureza.clone(),
            tipo_operacao: format!("{:?}", nfe.ide.operacao.tipo),
            destino_operacao: format!("{:?}", nfe.ide.operacao.destino),
            finalidade: format!("{:?}", nfe.ide.emissao.finalidade),
            data_emissao: nfe.ide.emissao.horario.format("%d/%m/%Y %H:%M").to_string(),
        },
        emitente: EmitenteData {
            cnpj: nfe.emit.cnpj.clone(),
            razao_social: nfe.emit.razao_social.clone(),
            nome_fantasia: nfe.emit.nome_fantasia.clone(),
            inscricao_estadual: nfe.emit.ie.clone(),
            endereco: EnderecoData {
                logradouro: nfe.emit.endereco.logradouro.clone(),
                numero: nfe.emit.endereco.numero.clone(),
                complemento: nfe.emit.endereco.complemento.clone(),
                bairro: nfe.emit.endereco.bairro.clone(),
                municipio: nfe.emit.endereco.nome_municipio.clone(),
                uf: nfe.emit.endereco.sigla_uf.clone(),
                cep: Some(nfe.emit.endereco.cep.clone()),
            },
        },
        destinatario: nfe.dest.as_ref().map(|dest| DestinatarioData {
            cnpj: dest.cnpj.clone(),
            razao_social: dest.razao_social.clone(),
            indicador_ie: format!("{:?}", dest.indicador_ie),
            endereco: dest.endereco.as_ref().map(|end| EnderecoData {
                logradouro: end.logradouro.clone(),
                numero: end.numero.clone(),
                complemento: end.complemento.clone(),
                bairro: end.bairro.clone(),
                municipio: end.nome_municipio.clone(),
                uf: end.sigla_uf.clone(),
                cep: Some(end.cep.clone()),
            }),
        }),
        itens: nfe.itens.iter().map(|item| ItemData {
            numero: item.numero,
            codigo: item.produto.codigo.clone(),
            descricao: item.produto.descricao.clone(),
            ncm: item.produto.ncm.clone(),
            cfop: item.produto.tributacao.cfop.clone(),
            unidade: item.produto.unidade.clone(),
            quantidade: item.produto.quantidade,
            valor_unitario: item.produto.valor_unitario,
            valor_bruto: item.produto.valor_bruto,
            valor_desconto: item.produto.valor_desconto,
            gtin: item.produto.gtin.clone(),
        }).collect(),
        totais: TotaisData {
            valor_produtos: nfe.totais.valor_produtos,
            valor_frete: nfe.totais.valor_frete,
            valor_seguro: nfe.totais.valor_seguro,
            valor_desconto: nfe.totais.valor_desconto,
            valor_outros: nfe.totais.valor_outros,
            valor_total: nfe.totais.valor_total,
            base_calculo_icms: nfe.totais.valor_base_calculo,
            valor_icms: nfe.totais.valor_icms,
            valor_pis: nfe.totais.valor_pis,
            valor_cofins: nfe.totais.valor_cofins,
            valor_aproximado_tributos: nfe.totais.valor_aproximado_tributos,
        },
        transporte: TransporteData {
            modalidade: format!("{:?}", nfe.transporte.modalidade),
        },
        informacao_complementar: nfe.informacao_complementar.clone(),
    }
}

async fn save_to_db(state: &AppState, nfe: &Nfe, xml: &str, data: &NfeData) -> bool {
    let record = NfeRecord {
        id: Uuid::new_v4().to_string(),
        chave_acesso: nfe.chave_acesso.clone(),
        numero: nfe.ide.numero as i32,
        serie: nfe.ide.serie as i16,
        data_emissao: nfe.ide.emissao.horario,
        emit_cnpj: nfe.emit.cnpj.clone().unwrap_or_default(),
        emit_razao_social: nfe.emit.razao_social.clone().unwrap_or_default(),
        dest_cnpj: nfe.dest.as_ref().map(|d| d.cnpj.clone()),
        dest_razao_social: nfe.dest.as_ref().and_then(|d| d.razao_social.clone()),
        valor_total: nfe.totais.valor_total as f64,
        xml: xml.to_string(),
        json_data: serde_json::to_string(data).unwrap_or_default(),
        created_at: Utc::now(),
    };

    let mut saved = false;

    if let Some(pg) = &state.postgres {
        if pg.insert(&record).await.is_ok() {
            saved = true;
            log::info!("NF-e {} salva no PostgreSQL", nfe.chave_acesso);
        }
    }

    if let Some(mysql) = &state.mysql {
        if mysql.insert(&record).await.is_ok() {
            saved = true;
            log::info!("NF-e {} salva no MySQL", nfe.chave_acesso);
        }
    }

    saved
}

// ============================================================================
// Endpoints
// ============================================================================

/// Parse de XML
async fn parse_nfe(body: web::Json<ParseRequest>, state: web::Data<AppState>) -> HttpResponse {
    let xml = &body.xml;
    let xml_clean = xml.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", "");

    let xml_to_parse = if let (Some(start), Some(end)) = (xml_clean.find("<NFe"), xml_clean.find("</NFe>")) {
        &xml_clean[start..end + 6]
    } else {
        &xml_clean
    };

    match xml_to_parse.parse::<Nfe>() {
        Ok(nfe) => {
            let data = nfe_to_data(&nfe);

            // Auto-save se habilitado
            let saved = if body.auto_save || state.auto_save {
                save_to_db(&state, &nfe, xml_to_parse, &data).await
            } else {
                false
            };

            HttpResponse::Ok().json(NfeResponse {
                success: true,
                data: Some(data),
                xml: None,
                error: None,
                saved: Some(saved),
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(NfeResponse {
                success: false,
                data: None,
                xml: None,
                error: Some(format!("Erro ao processar XML: {}", e)),
                saved: None,
            })
        }
    }
}

/// Gerar XML de NF-e
async fn generate_nfe(body: web::Json<GenerateRequest>, state: web::Data<AppState>) -> HttpResponse {
    let req = body.into_inner();

    let emit_endereco = Endereco {
        logradouro: req.emit_logradouro,
        numero: req.emit_numero,
        complemento: None,
        bairro: req.emit_bairro,
        codigo_municipio: req.codigo_municipio,
        nome_municipio: req.emit_municipio,
        sigla_uf: req.emit_uf,
        cep: req.emit_cep,
        codigo_pais: Some("1058".to_string()),
        nome_pais: Some("BRASIL".to_string()),
        telefone: None,
    };

    let ambiente = match req.ambiente.to_lowercase().as_str() {
        "producao" | "1" => TipoAmbiente::Producao,
        _ => TipoAmbiente::Homologacao,
    };

    let modalidade = match req.modalidade_frete.unwrap_or(9) {
        0 => ModalidadeFrete::ContratacaoPorContaDoRemetente,
        1 => ModalidadeFrete::ContratacaoPorContaDoDestinatario,
        2 => ModalidadeFrete::ContratacaoPorContaDeTerceiros,
        3 => ModalidadeFrete::TransportePorContaDoRemetente,
        4 => ModalidadeFrete::TransportePorContaDoDestinatario,
        _ => ModalidadeFrete::SemTransporte,
    };

    let mut builder = NfeBuilder::new()
        .codigo_uf(req.codigo_uf)
        .numero(req.numero)
        .serie(req.serie.unwrap_or(1))
        .natureza_operacao(&req.natureza_operacao)
        .tipo_operacao(TipoOperacao::Saida)
        .destino_operacao(DestinoOperacao::Interna)
        .ambiente(ambiente)
        .codigo_municipio(req.codigo_municipio)
        .emit_cnpj(&req.emit_cnpj)
        .emit_razao_social(&req.emit_razao_social)
        .emit_ie(&req.emit_ie)
        .emit_endereco(emit_endereco)
        .modalidade_frete(modalidade);

    if let Some(fantasia) = req.emit_nome_fantasia {
        builder = builder.emit_nome_fantasia(&fantasia);
    }

    if let Some(dest_cnpj) = req.dest_cnpj {
        builder = builder.dest_cnpj(&dest_cnpj);
        if let Some(razao) = req.dest_razao_social {
            builder = builder.dest_razao_social(&razao);
        }
        if let (Some(log), Some(num), Some(bairro), Some(mun), Some(uf), Some(cep)) = (
            req.dest_logradouro, req.dest_numero, req.dest_bairro,
            req.dest_municipio, req.dest_uf, req.dest_cep
        ) {
            let dest_end = Endereco {
                logradouro: log,
                numero: num,
                complemento: None,
                bairro,
                codigo_municipio: 0,
                nome_municipio: mun,
                sigla_uf: uf,
                cep,
                codigo_pais: Some("1058".to_string()),
                nome_pais: Some("BRASIL".to_string()),
                telefone: None,
            };
            builder = builder.dest_endereco(dest_end);
        }
    }

    for item in req.itens {
        let item_builder = ItemBuilder::new(&item.codigo, &item.descricao, &item.ncm, &item.cfop)
            .unidade(&item.unidade)
            .quantidade(item.quantidade)
            .valor_unitario(item.valor_unitario);
        builder = builder.add_item(item_builder);
    }

    if let Some(info) = req.informacao_complementar {
        builder = builder.informacao_complementar(&info);
    }

    match builder.build() {
        Ok(nfe) => {
            let data = nfe_to_data(&nfe);
            let xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", nfe.to_string());

            // Auto-save se habilitado
            let saved = if req.auto_save || state.auto_save {
                save_to_db(&state, &nfe, &xml, &data).await
            } else {
                false
            };

            HttpResponse::Ok().json(NfeResponse {
                success: true,
                data: Some(data),
                xml: Some(xml),
                error: None,
                saved: Some(saved),
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(NfeResponse {
                success: false,
                data: None,
                xml: None,
                error: Some(e),
                saved: None,
            })
        }
    }
}

/// Exportar para JSON
async fn export_json(body: web::Json<ParseRequest>) -> HttpResponse {
    let xml = &body.xml;
    let xml_clean = xml.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", "");

    let xml_to_parse = if let (Some(start), Some(end)) = (xml_clean.find("<NFe"), xml_clean.find("</NFe>")) {
        &xml_clean[start..end + 6]
    } else {
        &xml_clean
    };

    match xml_to_parse.parse::<Nfe>() {
        Ok(nfe) => {
            let data = nfe_to_data(&nfe);
            let json = serde_json::to_string_pretty(&data).unwrap();

            HttpResponse::Ok()
                .content_type("application/json")
                .insert_header(("Content-Disposition", format!("attachment; filename=\"nfe_{}.json\"", nfe.chave_acesso)))
                .body(json)
        }
        Err(e) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Erro ao processar XML: {}", e)
            }))
        }
    }
}

/// Listar NF-e do banco
async fn list_nfe(query: web::Query<DbQuery>, state: web::Data<AppState>) -> HttpResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    if let Some(pg) = &state.postgres {
        match pg.list(limit, offset).await {
            Ok(records) => {
                let total = records.len();
                return HttpResponse::Ok().json(ListResponse {
                    success: true,
                    data: records,
                    total,
                });
            }
            Err(e) => log::error!("Erro PostgreSQL: {}", e),
        }
    }

    if let Some(mysql) = &state.mysql {
        match mysql.list(limit, offset).await {
            Ok(records) => {
                let total = records.len();
                return HttpResponse::Ok().json(ListResponse {
                    success: true,
                    data: records,
                    total,
                });
            }
            Err(e) => log::error!("Erro MySQL: {}", e),
        }
    }

    HttpResponse::ServiceUnavailable().json(serde_json::json!({
        "error": "Nenhum banco de dados configurado"
    }))
}

/// Buscar NF-e por chave
async fn get_nfe(path: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let chave = path.into_inner();

    if let Some(pg) = &state.postgres {
        match pg.find_by_chave(&chave).await {
            Ok(Some(record)) => {
                return HttpResponse::Ok().json(record);
            }
            Ok(None) => {}
            Err(e) => log::error!("Erro PostgreSQL: {}", e),
        }
    }

    if let Some(mysql) = &state.mysql {
        match mysql.find_by_chave(&chave).await {
            Ok(Some(record)) => {
                return HttpResponse::Ok().json(record);
            }
            Ok(None) => {}
            Err(e) => log::error!("Erro MySQL: {}", e),
        }
    }

    HttpResponse::NotFound().json(serde_json::json!({
        "error": "NF-e não encontrada"
    }))
}

/// Health check
async fn health(state: web::Data<AppState>) -> HttpResponse {
    let pg_status = state.postgres.is_some();
    let mysql_status = state.mysql.is_some();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "nfe-web",
        "databases": {
            "postgres": pg_status,
            "mysql": mysql_status
        },
        "auto_save": state.auto_save
    }))
}

/// Configuração do banco
async fn db_config() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "postgres": {
            "env_var": "DATABASE_URL_POSTGRES",
            "example": "postgres://user:pass@localhost/nfe"
        },
        "mysql": {
            "env_var": "DATABASE_URL_MYSQL",
            "example": "mysql://user:pass@localhost/nfe"
        },
        "auto_save": {
            "env_var": "AUTO_SAVE",
            "example": "true"
        }
    }))
}

/// Ler DANFE de PDF (upload multipart)
async fn read_pdf_multipart(mut payload: Multipart) -> HttpResponse {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                return HttpResponse::BadRequest().json(PdfReadResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Erro ao processar upload: {}", e)),
                });
            }
        };

        let mut bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(data) => bytes.extend_from_slice(&data),
                Err(e) => {
                    return HttpResponse::BadRequest().json(PdfReadResponse {
                        success: false,
                        data: None,
                        error: Some(format!("Erro ao ler arquivo: {}", e)),
                    });
                }
            }
        }

        if !bytes.is_empty() {
            match extract_danfe_data(&bytes) {
                Ok(data) => {
                    return HttpResponse::Ok().json(PdfReadResponse {
                        success: true,
                        data: Some(data),
                        error: None,
                    });
                }
                Err(e) => {
                    return HttpResponse::BadRequest().json(PdfReadResponse {
                        success: false,
                        data: None,
                        error: Some(e),
                    });
                }
            }
        }
    }

    HttpResponse::BadRequest().json(PdfReadResponse {
        success: false,
        data: None,
        error: Some("Nenhum arquivo enviado".to_string()),
    })
}

/// Ler DANFE de PDF (base64)
async fn read_pdf_base64(body: web::Json<PdfBase64Request>) -> HttpResponse {
    use base64::Engine;

    let pdf_data = &body.pdf_base64;

    // Remover prefixo data:application/pdf;base64, se presente
    let base64_data = if pdf_data.contains("base64,") {
        pdf_data.split("base64,").last().unwrap_or(pdf_data)
    } else {
        pdf_data
    };

    match base64::engine::general_purpose::STANDARD.decode(base64_data) {
        Ok(bytes) => {
            match extract_danfe_data(&bytes) {
                Ok(data) => {
                    HttpResponse::Ok().json(PdfReadResponse {
                        success: true,
                        data: Some(data),
                        error: None,
                    })
                }
                Err(e) => {
                    HttpResponse::BadRequest().json(PdfReadResponse {
                        success: false,
                        data: None,
                        error: Some(e),
                    })
                }
            }
        }
        Err(e) => {
            HttpResponse::BadRequest().json(PdfReadResponse {
                success: false,
                data: None,
                error: Some(format!("Erro ao decodificar base64: {}", e)),
            })
        }
    }
}

// ============================================================================
// Consulta SEFAZ
// ============================================================================

/// Request para consulta de NF-e
#[derive(Deserialize)]
struct ConsultaRequest {
    /// Chave de acesso (44 dígitos)
    chave_acesso: Option<String>,
    /// Número da NF-e
    numero: Option<i32>,
    /// Série
    serie: Option<i16>,
    /// CNPJ do emissor
    cnpj_emissor: Option<String>,
    /// UF do emissor
    uf_emissor: Option<String>,
}

/// Response da consulta
#[derive(Serialize)]
struct ConsultaResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<sefaz::ResultadoConsulta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info_chave: Option<ChaveAcessoInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Consulta NF-e por chave de acesso
async fn consultar_por_chave(path: web::Path<String>) -> HttpResponse {
    let chave = path.into_inner();

    // Validar e extrair informações da chave
    match validar_chave_acesso(&chave) {
        Ok(info) => {
            let url_portal = gerar_url_consulta_portal(&info.chave);

            HttpResponse::Ok().json(ConsultaResponse {
                success: true,
                data: Some(sefaz::ResultadoConsulta {
                    sucesso: true,
                    codigo_status: Some("100".to_string()),
                    motivo: Some("Chave válida".to_string()),
                    chave_acesso: Some(info.chave.clone()),
                    situacao: Some("Acesse a URL para consultar status no SEFAZ".to_string()),
                    data_autorizacao: None,
                    protocolo: None,
                    numero: Some(info.numero as i32),
                    serie: Some(info.serie as i16),
                    emit_cnpj: Some(info.cnpj.clone()),
                    emit_razao_social: None,
                    valor_total: None,
                    url_consulta: Some(url_portal),
                }),
                info_chave: Some(info),
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(ConsultaResponse {
                success: false,
                data: None,
                info_chave: None,
                error: Some(e),
            })
        }
    }
}

/// Consulta NF-e via POST
async fn consultar_nfe(body: web::Json<ConsultaRequest>) -> HttpResponse {
    // Se tem chave de acesso, usar ela
    if let Some(ref chave) = body.chave_acesso {
        match validar_chave_acesso(chave) {
            Ok(info) => {
                // Tentar consulta no portal
                match consultar_portal_publico(&info.chave).await {
                    Ok(resultado) => {
                        HttpResponse::Ok().json(ConsultaResponse {
                            success: true,
                            data: Some(resultado),
                            info_chave: Some(info),
                            error: None,
                        })
                    }
                    Err(e) => {
                        // Mesmo com erro na consulta, retornar info da chave
                        let url_portal = gerar_url_consulta_portal(&info.chave);
                        HttpResponse::Ok().json(ConsultaResponse {
                            success: true,
                            data: Some(sefaz::ResultadoConsulta {
                                sucesso: true,
                                codigo_status: Some("100".to_string()),
                                motivo: Some(format!("Chave válida. Erro na consulta automática: {}", e)),
                                chave_acesso: Some(info.chave.clone()),
                                situacao: Some("Acesse a URL manualmente".to_string()),
                                data_autorizacao: None,
                                protocolo: None,
                                numero: Some(info.numero as i32),
                                serie: Some(info.serie as i16),
                                emit_cnpj: Some(info.cnpj.clone()),
                                emit_razao_social: None,
                                valor_total: None,
                                url_consulta: Some(url_portal),
                            }),
                            info_chave: Some(info),
                            error: None,
                        })
                    }
                }
            }
            Err(e) => {
                HttpResponse::BadRequest().json(ConsultaResponse {
                    success: false,
                    data: None,
                    info_chave: None,
                    error: Some(e),
                })
            }
        }
    } else {
        // Sem chave de acesso, informar que é necessária
        HttpResponse::BadRequest().json(ConsultaResponse {
            success: false,
            data: None,
            info_chave: None,
            error: Some("Chave de acesso é necessária para consulta. Para consultar pelo número, forneça também CNPJ, UF, série e ano/mês de emissão.".to_string()),
        })
    }
}

/// Valida chave de acesso e retorna informações
async fn validar_chave(path: web::Path<String>) -> HttpResponse {
    let chave = path.into_inner();

    match validar_chave_acesso(&chave) {
        Ok(info) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "valida": true,
                "info": info,
                "url_consulta": gerar_url_consulta_portal(&info.chave)
            }))
        }
        Err(e) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "valida": false,
                "error": e
            }))
        }
    }
}

/// Exportar NF-e para PDF (DANFE)
async fn export_pdf(body: web::Json<ParseRequest>) -> HttpResponse {
    let xml = &body.xml;
    let xml_clean = xml.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", "");

    let xml_to_parse = if let (Some(start), Some(end)) = (xml_clean.find("<NFe"), xml_clean.find("</NFe>")) {
        &xml_clean[start..end + 6]
    } else {
        &xml_clean
    };

    // Parser para o formato interno (nfe-parser)
    match xml_to_parse.parse::<Nfe>() {
        Ok(nfe) => {
            // Converter para o formato nfe_parser::Nfe que o generate_danfe espera
            // Nota: precisamos do formato da crate original
            match generate_danfe_from_parsed(&nfe) {
                Ok(pdf_bytes) => {
                    HttpResponse::Ok()
                        .content_type("application/pdf")
                        .insert_header(("Content-Disposition", format!("attachment; filename=\"danfe_{}.pdf\"", nfe.chave_acesso)))
                        .body(pdf_bytes)
                }
                Err(e) => {
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("Erro ao gerar PDF: {}", e)
                    }))
                }
            }
        }
        Err(e) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Erro ao processar XML: {}", e)
            }))
        }
    }
}

/// Gera DANFE a partir da NF-e parseada
fn generate_danfe_from_parsed(nfe: &Nfe) -> Result<Vec<u8>, String> {
    use printpdf::*;
    use std::io::BufWriter;

    const A4_WIDTH_MM: f32 = 210.0;
    const A4_HEIGHT_MM: f32 = 297.0;

    let (doc, page1, layer1) = PdfDocument::new(
        "DANFE",
        Mm(A4_WIDTH_MM),
        Mm(A4_HEIGHT_MM),
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Erro ao carregar fonte: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Erro ao carregar fonte bold: {}", e))?;

    let mut y = A4_HEIGHT_MM - 10.0;

    // Título
    current_layer.use_text("DANFE", 14.0, Mm(10.0), Mm(y), &font_bold);
    current_layer.use_text("Documento Auxiliar da Nota Fiscal Eletrônica", 8.0, Mm(35.0), Mm(y), &font);

    y -= 5.0;
    let tipo_op = format!("{:?}", nfe.ide.operacao.tipo);
    current_layer.use_text(&format!("Tipo: {}", tipo_op), 10.0, Mm(10.0), Mm(y), &font);

    // Número e série
    y -= 8.0;
    draw_box_pdf(&current_layer, 10.0, y, 60.0, 15.0);
    current_layer.use_text("NÚMERO", 6.0, Mm(12.0), Mm(y + 12.0), &font);
    current_layer.use_text(&format!("{:09}", nfe.ide.numero), 12.0, Mm(12.0), Mm(y + 5.0), &font_bold);

    draw_box_pdf(&current_layer, 72.0, y, 30.0, 15.0);
    current_layer.use_text("SÉRIE", 6.0, Mm(74.0), Mm(y + 12.0), &font);
    current_layer.use_text(&format!("{}", nfe.ide.serie), 12.0, Mm(74.0), Mm(y + 5.0), &font_bold);

    // Chave de acesso
    y -= 18.0;
    draw_box_pdf(&current_layer, 10.0, y, 190.0, 15.0);
    current_layer.use_text("CHAVE DE ACESSO", 6.0, Mm(12.0), Mm(y + 12.0), &font);
    let chave_formatada = format_chave_pdf(&nfe.chave_acesso);
    current_layer.use_text(&chave_formatada, 9.0, Mm(12.0), Mm(y + 5.0), &font_bold);

    // Emitente
    y -= 20.0;
    draw_box_pdf(&current_layer, 10.0, y, 190.0, 25.0);
    current_layer.use_text("EMITENTE", 6.0, Mm(12.0), Mm(y + 22.0), &font);
    current_layer.use_text(nfe.emit.razao_social.as_deref().unwrap_or(""), 10.0, Mm(12.0), Mm(y + 15.0), &font_bold);
    current_layer.use_text(&format!("CNPJ: {}", format_cnpj_pdf(nfe.emit.cnpj.as_deref().unwrap_or(""))), 8.0, Mm(12.0), Mm(y + 8.0), &font);
    let endereco_emit = format!(
        "{}, {} - {} - {}/{}",
        nfe.emit.endereco.logradouro,
        nfe.emit.endereco.numero,
        nfe.emit.endereco.bairro,
        nfe.emit.endereco.nome_municipio,
        nfe.emit.endereco.sigla_uf
    );
    current_layer.use_text(&endereco_emit, 7.0, Mm(12.0), Mm(y + 1.0), &font);

    // Destinatário
    y -= 30.0;
    draw_box_pdf(&current_layer, 10.0, y, 190.0, 25.0);
    current_layer.use_text("DESTINATÁRIO/REMETENTE", 6.0, Mm(12.0), Mm(y + 22.0), &font);
    if let Some(dest) = &nfe.dest {
        current_layer.use_text(dest.razao_social.as_deref().unwrap_or(""), 10.0, Mm(12.0), Mm(y + 15.0), &font_bold);
        current_layer.use_text(&format!("CNPJ/CPF: {}", &dest.cnpj), 8.0, Mm(12.0), Mm(y + 8.0), &font);
        if let Some(end) = &dest.endereco {
            let endereco_dest = format!(
                "{}, {} - {} - {}/{}",
                end.logradouro,
                end.numero,
                end.bairro,
                end.nome_municipio,
                end.sigla_uf
            );
            current_layer.use_text(&endereco_dest, 7.0, Mm(12.0), Mm(y + 1.0), &font);
        }
    }

    // Produtos - Cabeçalho
    y -= 30.0;
    draw_box_pdf(&current_layer, 10.0, y, 190.0, 10.0);
    current_layer.use_text("DADOS DOS PRODUTOS / SERVIÇOS", 8.0, Mm(12.0), Mm(y + 6.0), &font_bold);

    // Cabeçalho da tabela
    y -= 12.0;
    draw_box_pdf(&current_layer, 10.0, y, 190.0, 8.0);
    current_layer.use_text("CÓD", 6.0, Mm(12.0), Mm(y + 5.0), &font);
    current_layer.use_text("DESCRIÇÃO", 6.0, Mm(35.0), Mm(y + 5.0), &font);
    current_layer.use_text("NCM", 6.0, Mm(100.0), Mm(y + 5.0), &font);
    current_layer.use_text("CFOP", 6.0, Mm(120.0), Mm(y + 5.0), &font);
    current_layer.use_text("UN", 6.0, Mm(135.0), Mm(y + 5.0), &font);
    current_layer.use_text("QTD", 6.0, Mm(145.0), Mm(y + 5.0), &font);
    current_layer.use_text("VL UNIT", 6.0, Mm(160.0), Mm(y + 5.0), &font);
    current_layer.use_text("VL TOTAL", 6.0, Mm(180.0), Mm(y + 5.0), &font);

    // Itens
    for item in &nfe.itens {
        y -= 10.0;
        if y < 50.0 { break; }

        draw_box_pdf(&current_layer, 10.0, y, 190.0, 8.0);
        current_layer.use_text(&truncate_pdf(&item.produto.codigo, 10), 5.0, Mm(12.0), Mm(y + 4.0), &font);
        current_layer.use_text(&truncate_pdf(&item.produto.descricao, 35), 5.0, Mm(35.0), Mm(y + 4.0), &font);
        current_layer.use_text(&item.produto.ncm, 5.0, Mm(100.0), Mm(y + 4.0), &font);
        current_layer.use_text(&item.produto.tributacao.cfop, 5.0, Mm(120.0), Mm(y + 4.0), &font);
        current_layer.use_text(&item.produto.unidade, 5.0, Mm(135.0), Mm(y + 4.0), &font);
        current_layer.use_text(&format!("{:.2}", item.produto.quantidade), 5.0, Mm(145.0), Mm(y + 4.0), &font);
        current_layer.use_text(&format!("{:.2}", item.produto.valor_unitario), 5.0, Mm(160.0), Mm(y + 4.0), &font);
        current_layer.use_text(&format!("{:.2}", item.produto.valor_bruto), 5.0, Mm(180.0), Mm(y + 4.0), &font);
    }

    // Totais
    y -= 15.0;
    draw_box_pdf(&current_layer, 10.0, y, 190.0, 20.0);
    current_layer.use_text("CÁLCULO DO IMPOSTO", 6.0, Mm(12.0), Mm(y + 17.0), &font);
    current_layer.use_text(&format!("BASE CÁLC ICMS: {:.2}", nfe.totais.valor_base_calculo), 7.0, Mm(12.0), Mm(y + 10.0), &font);
    current_layer.use_text(&format!("VALOR ICMS: {:.2}", nfe.totais.valor_icms), 7.0, Mm(60.0), Mm(y + 10.0), &font);
    current_layer.use_text(&format!("VALOR FRETE: {:.2}", nfe.totais.valor_frete), 7.0, Mm(110.0), Mm(y + 10.0), &font);
    current_layer.use_text(&format!("VALOR SEGURO: {:.2}", nfe.totais.valor_seguro), 7.0, Mm(160.0), Mm(y + 10.0), &font);
    current_layer.use_text(&format!("VALOR PRODUTOS: {:.2}", nfe.totais.valor_produtos), 7.0, Mm(12.0), Mm(y + 3.0), &font);
    current_layer.use_text(&format!("VALOR DESCONTO: {:.2}", nfe.totais.valor_desconto), 7.0, Mm(60.0), Mm(y + 3.0), &font);
    current_layer.use_text(&format!("OUTROS: {:.2}", nfe.totais.valor_outros), 7.0, Mm(110.0), Mm(y + 3.0), &font);

    // Total
    y -= 25.0;
    draw_box_pdf(&current_layer, 10.0, y, 190.0, 15.0);
    current_layer.use_text("VALOR TOTAL DA NOTA FISCAL", 8.0, Mm(12.0), Mm(y + 12.0), &font_bold);
    current_layer.use_text(&format!("R$ {:.2}", nfe.totais.valor_total), 14.0, Mm(12.0), Mm(y + 3.0), &font_bold);

    // Data
    y -= 20.0;
    current_layer.use_text(&format!("Data de Emissão: {}", nfe.ide.emissao.horario.format("%d/%m/%Y %H:%M")), 8.0, Mm(10.0), Mm(y), &font);

    // Rodapé
    current_layer.use_text("Documento gerado por nfe-parser", 6.0, Mm(10.0), Mm(5.0), &font);

    // Salvar
    let mut buffer = BufWriter::new(Vec::new());
    doc.save(&mut buffer).map_err(|e| format!("Erro ao salvar PDF: {}", e))?;

    buffer.into_inner()
        .map_err(|e| format!("Erro ao extrair bytes do PDF: {}", e))
}

fn draw_box_pdf(layer: &printpdf::PdfLayerReference, x: f32, y: f32, width: f32, height: f32) {
    use printpdf::{Mm, Point, Line};
    let points = vec![
        (Point::new(Mm(x), Mm(y)), false),
        (Point::new(Mm(x + width), Mm(y)), false),
        (Point::new(Mm(x + width), Mm(y + height)), false),
        (Point::new(Mm(x), Mm(y + height)), false),
    ];
    let line = Line { points, is_closed: true };
    layer.add_line(line);
}

fn format_chave_pdf(chave: &str) -> String {
    chave.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_cnpj_pdf(cnpj: &str) -> String {
    if cnpj.len() == 14 {
        format!("{}.{}.{}/{}-{}", &cnpj[0..2], &cnpj[2..5], &cnpj[5..8], &cnpj[8..12], &cnpj[12..14])
    } else {
        cnpj.to_string()
    }
}

fn truncate_pdf(s: &str, max_len: usize) -> String {
    if s.len() > max_len { format!("{}...", &s[..max_len-3]) } else { s.to_string() }
}

// ============================================================================
// GraphQL Handlers
// ============================================================================

/// Handler para queries GraphQL
async fn graphql_handler(
    schema: web::Data<graphql::NfeSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Playground GraphQL para desenvolvimento
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/api/graphql")
        ))
}

/// Retorna o schema GraphQL em SDL
async fn graphql_sdl(schema: web::Data<graphql::NfeSchema>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(schema.sdl())
}

// ============================================================================
// Exportar DANFE Profissional
// ============================================================================

/// Request para gerar DANFE profissional
#[derive(Deserialize)]
struct DanfeRequest {
    xml: Option<String>,
    dados: Option<pdf::DanfeInput>,
}

/// Gerar DANFE profissional em PDF
async fn gerar_danfe_pdf(body: web::Json<DanfeRequest>) -> HttpResponse {
    // Se recebeu XML, parsear primeiro
    if let Some(ref xml) = body.xml {
        let xml_clean = xml.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", "");

        let xml_to_parse = if let (Some(start), Some(end)) = (xml_clean.find("<NFe"), xml_clean.find("</NFe>")) {
            &xml_clean[start..end + 6]
        } else {
            &xml_clean
        };

        match xml_to_parse.parse::<Nfe>() {
            Ok(nfe) => {
                // Converter para DanfeInput
                let input = pdf::DanfeInput {
                    chave_acesso: nfe.chave_acesso.clone(),
                    numero: nfe.ide.numero,
                    serie: nfe.ide.serie,
                    data_emissao: nfe.ide.emissao.horario.format("%d/%m/%Y %H:%M").to_string(),
                    natureza_operacao: nfe.ide.operacao.natureza.clone(),
                    protocolo: None,
                    data_autorizacao: None,
                    emitente: pdf::DanfeEmitente {
                        cnpj: nfe.emit.cnpj.clone().unwrap_or_default(),
                        razao_social: nfe.emit.razao_social.clone().unwrap_or_default(),
                        nome_fantasia: nfe.emit.nome_fantasia.clone(),
                        inscricao_estadual: nfe.emit.ie.clone(),
                        endereco: format!("{}, {}", nfe.emit.endereco.logradouro, nfe.emit.endereco.numero),
                        municipio: nfe.emit.endereco.nome_municipio.clone(),
                        uf: nfe.emit.endereco.sigla_uf.clone(),
                        cep: nfe.emit.endereco.cep.clone(),
                        telefone: nfe.emit.endereco.telefone.clone(),
                    },
                    destinatario: nfe.dest.as_ref().map(|d| pdf::DanfeDestinatario {
                        cnpj_cpf: d.cnpj.clone(),
                        razao_social: d.razao_social.clone().unwrap_or_default(),
                        inscricao_estadual: None,
                        endereco: d.endereco.as_ref().map(|e| format!("{}, {}", e.logradouro, e.numero)).unwrap_or_default(),
                        municipio: d.endereco.as_ref().map(|e| e.nome_municipio.clone()).unwrap_or_default(),
                        uf: d.endereco.as_ref().map(|e| e.sigla_uf.clone()).unwrap_or_default(),
                        cep: d.endereco.as_ref().map(|e| e.cep.clone()).unwrap_or_default(),
                    }),
                    itens: nfe.itens.iter().enumerate().map(|(i, item)| pdf::DanfeItem {
                        numero: (i + 1) as u32,
                        codigo: item.produto.codigo.clone(),
                        descricao: item.produto.descricao.clone(),
                        ncm: item.produto.ncm.clone(),
                        cfop: item.produto.tributacao.cfop.clone(),
                        unidade: item.produto.unidade.clone(),
                        quantidade: item.produto.quantidade as f64,
                        valor_unitario: item.produto.valor_unitario as f64,
                        valor_total: item.produto.valor_bruto as f64,
                    }).collect(),
                    totais: pdf::DanfeTotais {
                        base_calculo_icms: nfe.totais.valor_base_calculo as f64,
                        valor_icms: nfe.totais.valor_icms as f64,
                        base_calculo_st: 0.0,
                        valor_st: 0.0,
                        valor_produtos: nfe.totais.valor_produtos as f64,
                        valor_frete: nfe.totais.valor_frete as f64,
                        valor_seguro: nfe.totais.valor_seguro as f64,
                        valor_desconto: nfe.totais.valor_desconto as f64,
                        valor_ipi: 0.0,
                        valor_total: nfe.totais.valor_total as f64,
                    },
                    transporte: Some(pdf::DanfeTransporte {
                        modalidade: format!("{:?}", nfe.transporte.modalidade),
                        transportadora: None,
                        placa: None,
                        uf: None,
                    }),
                    informacoes_complementares: nfe.informacao_complementar.clone(),
                };

                match pdf::gerar_danfe(&input) {
                    Ok(pdf_bytes) => {
                        return HttpResponse::Ok()
                            .content_type("application/pdf")
                            .insert_header(("Content-Disposition", format!("attachment; filename=\"danfe_{}.pdf\"", nfe.chave_acesso)))
                            .body(pdf_bytes);
                    }
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": format!("Erro ao gerar PDF: {}", e)
                        }));
                    }
                }
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": format!("Erro ao processar XML: {}", e)
                }));
            }
        }
    }

    // Se recebeu dados diretos
    if let Some(ref dados) = body.dados {
        match pdf::gerar_danfe(dados) {
            Ok(pdf_bytes) => {
                return HttpResponse::Ok()
                    .content_type("application/pdf")
                    .insert_header(("Content-Disposition", format!("attachment; filename=\"danfe_{}.pdf\"", dados.chave_acesso)))
                    .body(pdf_bytes);
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Erro ao gerar PDF: {}", e)
                }));
            }
        }
    }

    HttpResponse::BadRequest().json(serde_json::json!({
        "error": "Forneça 'xml' ou 'dados' para gerar o DANFE"
    }))
}

// ============================================================================
// Main
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    // Conectar aos bancos de dados se configurados
    let postgres = if let Ok(url) = env::var("DATABASE_URL_POSTGRES") {
        match PostgresClient::connect(&url).await {
            Ok(client) => {
                if let Err(e) = client.create_table().await {
                    log::error!("Erro ao criar tabela PostgreSQL: {}", e);
                }
                log::info!("PostgreSQL conectado");
                Some(Arc::new(client))
            }
            Err(e) => {
                log::error!("Erro ao conectar PostgreSQL: {}", e);
                None
            }
        }
    } else {
        None
    };

    let mysql = if let Ok(url) = env::var("DATABASE_URL_MYSQL") {
        match MysqlClient::connect(&url).await {
            Ok(client) => {
                if let Err(e) = client.create_table().await {
                    log::error!("Erro ao criar tabela MySQL: {}", e);
                }
                log::info!("MySQL conectado");
                Some(Arc::new(client))
            }
            Err(e) => {
                log::error!("Erro ao conectar MySQL: {}", e);
                None
            }
        }
    } else {
        None
    };

    let auto_save = env::var("AUTO_SAVE").map(|v| v == "true" || v == "1").unwrap_or(false);

    let state = AppState {
        postgres,
        mysql,
        auto_save,
    };

    // Criar schema GraphQL
    let graphql_schema = graphql::create_schema();
    let schema_data = web::Data::new(graphql_schema);

    log::info!("Iniciando servidor NFe Web em http://{}", bind_addr);
    log::info!("Acesse http://{}/ para abrir a interface", bind_addr);
    log::info!("GraphQL Playground: http://{}/api/graphql/playground", bind_addr);
    log::info!("Auto-save: {}", if auto_save { "ativado" } else { "desativado" });

    let state_data = web::Data::new(state);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(state_data.clone())
            .app_data(schema_data.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            // API REST endpoints
            .route("/api/health", web::get().to(health))
            .route("/api/config", web::get().to(db_config))
            .route("/api/parse", web::post().to(parse_nfe))
            .route("/api/generate", web::post().to(generate_nfe))
            .route("/api/export/json", web::post().to(export_json))
            .route("/api/export/pdf", web::post().to(export_pdf))
            .route("/api/export/danfe", web::post().to(gerar_danfe_pdf))
            .route("/api/read-pdf", web::post().to(read_pdf_multipart))
            .route("/api/read-pdf-base64", web::post().to(read_pdf_base64))
            // Consulta SEFAZ
            .route("/api/consultar/chave/{chave}", web::get().to(consultar_por_chave))
            .route("/api/consultar", web::post().to(consultar_nfe))
            .route("/api/validar-chave/{chave}", web::get().to(validar_chave))
            .route("/api/nfe", web::get().to(list_nfe))
            .route("/api/nfe/{chave}", web::get().to(get_nfe))
            // GraphQL endpoints
            .route("/api/graphql", web::post().to(graphql_handler))
            .route("/api/graphql/playground", web::get().to(graphql_playground))
            .route("/api/graphql/schema", web::get().to(graphql_sdl))
            // Arquivos estáticos
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
