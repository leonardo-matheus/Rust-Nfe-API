//! Servidor web para visualização de NF-e
//!
//! Este servidor expõe uma API REST para parsing de XML de NF-e
//! e serve uma interface web para interação.

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use nfe_parser::Nfe;
use serde::{Deserialize, Serialize};
use std::env;

/// Requisição de parsing de XML
#[derive(Deserialize)]
struct ParseRequest {
    xml: String,
}

/// Resposta com dados da NF-e
#[derive(Serialize)]
struct NfeResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<NfeData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Dados da NF-e para o frontend
#[derive(Serialize)]
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

#[derive(Serialize)]
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

#[derive(Serialize)]
struct EmitenteData {
    cnpj: Option<String>,
    razao_social: Option<String>,
    nome_fantasia: Option<String>,
    inscricao_estadual: Option<String>,
    endereco: EnderecoData,
}

#[derive(Serialize)]
struct DestinatarioData {
    cnpj: String,
    razao_social: Option<String>,
    indicador_ie: String,
    endereco: Option<EnderecoData>,
}

#[derive(Serialize)]
struct EnderecoData {
    logradouro: String,
    numero: String,
    complemento: Option<String>,
    bairro: String,
    municipio: String,
    uf: String,
    cep: Option<String>,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
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

#[derive(Serialize)]
struct TransporteData {
    modalidade: String,
}

/// Converte Nfe para NfeData
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

/// Endpoint para parsing de XML
async fn parse_nfe(body: web::Json<ParseRequest>) -> HttpResponse {
    let xml = &body.xml;

    // Remove namespace da SEFAZ se presente
    let xml_clean = xml.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", "");

    // Tenta extrair apenas a tag <NFe> se for nfeProc
    let xml_to_parse = if let (Some(start), Some(end)) = (xml_clean.find("<NFe"), xml_clean.find("</NFe>")) {
        &xml_clean[start..end + 6]
    } else {
        &xml_clean
    };

    match xml_to_parse.parse::<Nfe>() {
        Ok(nfe) => {
            let data = nfe_to_data(&nfe);
            HttpResponse::Ok().json(NfeResponse {
                success: true,
                data: Some(data),
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(NfeResponse {
                success: false,
                data: None,
                error: Some(format!("Erro ao processar XML: {}", e)),
            })
        }
    }
}

/// Health check
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "nfe-web"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializa logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    log::info!("Iniciando servidor NFe Web em http://{}", bind_addr);
    log::info!("Acesse http://{}/ para abrir a interface", bind_addr);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/api/health", web::get().to(health))
            .route("/api/parse", web::post().to(parse_nfe))
            // Serve arquivos estáticos do diretório static
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
