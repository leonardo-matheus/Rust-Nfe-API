//! Resolvers GraphQL com integração SEFAZ

use super::types::*;
use crate::sefaz;
use crate::sefaz::webservice::{SefazClient, AmbienteNfe};
use crate::certificado::{CertificadoA1, AssinadorXml};
use async_graphql::{Context, Object, Result as GqlResult};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Estado compartilhado do GraphQL
pub struct GraphQLState {
    pub certificado: Option<CertificadoA1>,
    pub sefaz_client: Option<SefazClient>,
}

impl Default for GraphQLState {
    fn default() -> Self {
        Self {
            certificado: None,
            sefaz_client: None,
        }
    }
}

/// Query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Consulta NF-e por chave de acesso
    async fn nfe(&self, chave_acesso: String) -> GqlResult<Option<NfeType>> {
        // Validar chave
        let info = sefaz::validar_chave_acesso(&chave_acesso)
            .map_err(|e| async_graphql::Error::new(e))?;

        // Retornar dados básicos da chave
        Ok(Some(NfeType {
            id: chave_acesso.clone(),
            chave_acesso: chave_acesso.clone(),
            numero: info.numero as i32,
            serie: info.serie as i32,
            tipo: TipoDocumento::Nfe,
            ambiente: Ambiente::Producao,
            status: StatusNfe::Pendente,
            data_emissao: format!("20{}-{}-01", &info.ano_mes[0..2], &info.ano_mes[2..4]),
            data_autorizacao: None,
            protocolo: None,
            emitente: EmitenteType {
                cnpj: info.cnpj,
                razao_social: "Consulte SEFAZ para dados completos".to_string(),
                nome_fantasia: None,
                inscricao_estadual: None,
                endereco: EnderecoType {
                    logradouro: String::new(),
                    numero: String::new(),
                    complemento: None,
                    bairro: String::new(),
                    municipio: String::new(),
                    uf: info.uf,
                    cep: String::new(),
                    pais: Some("Brasil".to_string()),
                },
            },
            destinatario: None,
            itens: vec![],
            totais: TotaisType {
                base_calculo_icms: 0.0,
                valor_icms: 0.0,
                valor_produtos: 0.0,
                valor_frete: 0.0,
                valor_desconto: 0.0,
                valor_total: 0.0,
            },
            xml: None,
        }))
    }

    /// Lista NF-e com filtros
    async fn nfes(
        &self,
        filter: Option<NfeFilter>,
        pagination: Option<Pagination>,
    ) -> GqlResult<Vec<NfeType>> {
        let _filter = filter.unwrap_or_default();
        let _pagination = pagination.unwrap_or_default();

        // Em produção, buscar do banco de dados
        Ok(vec![])
    }

    /// Consulta status no SEFAZ (usa WebService se certificado disponível)
    async fn consultar_sefaz(&self, ctx: &Context<'_>, chave_acesso: String) -> GqlResult<ConsultaSefazResult> {
        let info = sefaz::validar_chave_acesso(&chave_acesso)
            .map_err(|e| async_graphql::Error::new(e))?;

        // Tentar usar cliente SEFAZ se disponível
        if let Some(state) = ctx.data_opt::<Arc<RwLock<GraphQLState>>>() {
            let state = state.read().await;
            if let Some(ref client) = state.sefaz_client {
                match client.consultar_nfe(&chave_acesso).await {
                    Ok(resultado) => {
                        return Ok(ConsultaSefazResult {
                            sucesso: resultado.sucesso,
                            codigo_status: resultado.codigo_status.unwrap_or_else(|| "000".to_string()),
                            motivo: resultado.motivo.unwrap_or_else(|| "Consulta realizada".to_string()),
                            chave_acesso: resultado.chave_acesso,
                            protocolo: resultado.protocolo,
                            data_recebimento: resultado.data_autorizacao,
                            situacao: resultado.situacao,
                        });
                    }
                    Err(e) => {
                        return Err(async_graphql::Error::new(format!("Erro SEFAZ: {}", e)));
                    }
                }
            }
        }

        // Fallback: retornar URL do portal público
        let url = sefaz::gerar_url_consulta_portal(&chave_acesso);

        Ok(ConsultaSefazResult {
            sucesso: true,
            codigo_status: "100".to_string(),
            motivo: "Chave válida - use certificado para consulta completa".to_string(),
            chave_acesso: Some(info.chave),
            protocolo: None,
            data_recebimento: None,
            situacao: Some(format!("Portal: {}", url)),
        })
    }

    /// Consulta status do serviço SEFAZ
    async fn status_servico(&self, ctx: &Context<'_>) -> GqlResult<StatusServicoType> {
        if let Some(state) = ctx.data_opt::<Arc<RwLock<GraphQLState>>>() {
            let state = state.read().await;
            if let Some(ref client) = state.sefaz_client {
                match client.status_servico().await {
                    Ok(status) => {
                        return Ok(StatusServicoType {
                            codigo_status: status.codigo_status.clone(),
                            motivo: status.motivo,
                            tempo_medio: status.tempo_medio.map(|t| format!("{} ms", t)),
                            uf: None,
                            online: status.online,
                        });
                    }
                    Err(e) => {
                        return Err(async_graphql::Error::new(format!("Erro SEFAZ: {}", e)));
                    }
                }
            }
        }

        Err(async_graphql::Error::new("Certificado digital não configurado"))
    }

    /// Valida chave de acesso
    async fn validar_chave(&self, chave: String) -> GqlResult<bool> {
        match sefaz::validar_chave_acesso(&chave) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Informações do certificado carregado
    async fn certificado(&self, ctx: &Context<'_>) -> GqlResult<Option<CertificadoInfoType>> {
        if let Some(state) = ctx.data_opt::<Arc<RwLock<GraphQLState>>>() {
            let state = state.read().await;
            if let Some(ref cert) = state.certificado {
                return Ok(Some(CertificadoInfoType {
                    cnpj: cert.info.cnpj.clone(),
                    razao_social: cert.info.razao_social.clone(),
                    valido: cert.info.valido,
                    data_validade: cert.info.not_after.clone(),
                    dias_para_expirar: cert.info.dias_para_expirar as i32,
                }));
            }
        }

        // Fallback para contexto antigo
        if let Some(cert_info) = ctx.data_opt::<crate::certificado::CertificadoInfo>() {
            Ok(Some(CertificadoInfoType {
                cnpj: cert_info.cnpj.clone(),
                razao_social: cert_info.razao_social.clone(),
                valido: cert_info.valido,
                data_validade: cert_info.not_after.clone(),
                dias_para_expirar: cert_info.dias_para_expirar as i32,
            }))
        } else {
            Ok(None)
        }
    }

    /// Health check
    async fn health(&self) -> GqlResult<String> {
        Ok("OK".to_string())
    }
}

/// Status do serviço SEFAZ
#[derive(Debug, Clone, async_graphql::SimpleObject)]
pub struct StatusServicoType {
    pub codigo_status: String,
    pub motivo: String,
    pub tempo_medio: Option<String>,
    pub uf: Option<String>,
    pub online: bool,
}

/// Mutation root
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Emite NF-e no SEFAZ
    async fn emitir_nfe(&self, ctx: &Context<'_>, input: NfeInput) -> GqlResult<EmissaoResult> {
        // Validar dados
        if input.itens.is_empty() {
            return Err(async_graphql::Error::new("NF-e deve ter pelo menos um item"));
        }

        // Verificar se certificado está configurado
        if let Some(state) = ctx.data_opt::<Arc<RwLock<GraphQLState>>>() {
            let state = state.read().await;

            if let (Some(ref cert), Some(ref client)) = (&state.certificado, &state.sefaz_client) {
                // Gerar XML da NF-e
                let xml_nfe = gerar_xml_nfe(&input)?;

                // Assinar XML
                let assinador = AssinadorXml::new(cert.clone());
                let xml_assinado = assinador.assinar_nfe(&xml_nfe)
                    .map_err(|e| async_graphql::Error::new(format!("Erro ao assinar: {}", e)))?;

                // Enviar para SEFAZ
                match client.autorizar_nfe(&xml_assinado).await {
                    Ok(resultado) => {
                        return Ok(EmissaoResult {
                            sucesso: resultado.sucesso,
                            codigo_status: resultado.codigo_status,
                            motivo: resultado.motivo,
                            chave_acesso: resultado.chave_acesso,
                            protocolo: resultado.protocolo,
                            xml_autorizado: resultado.xml_autorizado,
                        });
                    }
                    Err(e) => {
                        return Err(async_graphql::Error::new(format!("Erro SEFAZ: {}", e)));
                    }
                }
            }
        }

        Ok(EmissaoResult {
            sucesso: false,
            codigo_status: "999".to_string(),
            motivo: "Certificado digital não configurado. Use carregar_certificado primeiro.".to_string(),
            chave_acesso: None,
            protocolo: None,
            xml_autorizado: None,
        })
    }

    /// Cancela NF-e no SEFAZ
    async fn cancelar_nfe(&self, ctx: &Context<'_>, input: CancelamentoInput) -> GqlResult<CancelamentoResult> {
        // Validar chave
        let _info = sefaz::validar_chave_acesso(&input.chave_acesso)
            .map_err(|e| async_graphql::Error::new(e))?;

        // Validar justificativa (mínimo 15 caracteres)
        if input.justificativa.len() < 15 {
            return Err(async_graphql::Error::new("Justificativa deve ter no mínimo 15 caracteres"));
        }

        // Verificar se certificado está configurado
        if let Some(state) = ctx.data_opt::<Arc<RwLock<GraphQLState>>>() {
            let state = state.read().await;

            if let Some(ref client) = state.sefaz_client {
                match client.cancelar_nfe(&input.chave_acesso, &input.protocolo_autorizacao, &input.justificativa).await {
                    Ok(resultado) => {
                        return Ok(CancelamentoResult {
                            sucesso: resultado.sucesso,
                            codigo_status: resultado.codigo_status,
                            motivo: resultado.motivo,
                            protocolo: resultado.protocolo,
                            data_cancelamento: resultado.data_evento,
                        });
                    }
                    Err(e) => {
                        return Err(async_graphql::Error::new(format!("Erro SEFAZ: {}", e)));
                    }
                }
            }
        }

        Ok(CancelamentoResult {
            sucesso: false,
            codigo_status: "999".to_string(),
            motivo: "Certificado digital não configurado. Use carregar_certificado primeiro.".to_string(),
            protocolo: None,
            data_cancelamento: None,
        })
    }

    /// Envia carta de correção
    async fn carta_correcao(&self, ctx: &Context<'_>, input: CartaCorrecaoInput) -> GqlResult<EmissaoResult> {
        // Validar chave
        let _info = sefaz::validar_chave_acesso(&input.chave_acesso)
            .map_err(|e| async_graphql::Error::new(e))?;

        // Validar correção (mínimo 15 caracteres)
        if input.correcao.len() < 15 {
            return Err(async_graphql::Error::new("Correção deve ter no mínimo 15 caracteres"));
        }

        // Verificar se certificado está configurado
        if let Some(state) = ctx.data_opt::<Arc<RwLock<GraphQLState>>>() {
            let state = state.read().await;

            if let Some(ref client) = state.sefaz_client {
                match client.carta_correcao(&input.chave_acesso, input.sequencia as u32, &input.correcao).await {
                    Ok(resultado) => {
                        return Ok(EmissaoResult {
                            sucesso: resultado.sucesso,
                            codigo_status: resultado.codigo_status,
                            motivo: resultado.motivo,
                            chave_acesso: Some(input.chave_acesso),
                            protocolo: resultado.protocolo,
                            xml_autorizado: None,
                        });
                    }
                    Err(e) => {
                        return Err(async_graphql::Error::new(format!("Erro SEFAZ: {}", e)));
                    }
                }
            }
        }

        Ok(EmissaoResult {
            sucesso: false,
            codigo_status: "999".to_string(),
            motivo: "Certificado digital não configurado. Use carregar_certificado primeiro.".to_string(),
            chave_acesso: None,
            protocolo: None,
            xml_autorizado: None,
        })
    }

    /// Carrega certificado digital e inicializa cliente SEFAZ
    async fn carregar_certificado(
        &self,
        ctx: &Context<'_>,
        pfx_base64: String,
        senha: String,
        uf: String,
        ambiente: Option<String>,
    ) -> GqlResult<CertificadoInfoType> {
        use base64::Engine;

        let pfx_bytes = base64::engine::general_purpose::STANDARD
            .decode(&pfx_base64)
            .map_err(|e| async_graphql::Error::new(format!("Erro ao decodificar certificado: {}", e)))?;

        let cert = CertificadoA1::from_bytes(&pfx_bytes, &senha)
            .map_err(|e| async_graphql::Error::new(e))?;

        // Verificar validade
        if !cert.is_valid() {
            return Err(async_graphql::Error::new("Certificado expirado ou inválido"));
        }

        let info = CertificadoInfoType {
            cnpj: cert.info.cnpj.clone(),
            razao_social: cert.info.razao_social.clone(),
            valido: cert.info.valido,
            data_validade: cert.info.not_after.clone(),
            dias_para_expirar: cert.info.dias_para_expirar as i32,
        };

        // Determinar ambiente
        let amb = match ambiente.as_deref() {
            Some("homologacao") | Some("2") => AmbienteNfe::Homologacao,
            _ => AmbienteNfe::Producao,
        };

        // Criar cliente SEFAZ
        let sefaz_client = SefazClient::new(cert.clone(), &uf, amb)
            .map_err(|e| async_graphql::Error::new(format!("Erro ao criar cliente SEFAZ: {}", e)))?;

        // Atualizar estado
        if let Some(state) = ctx.data_opt::<Arc<RwLock<GraphQLState>>>() {
            let mut state = state.write().await;
            state.certificado = Some(cert);
            state.sefaz_client = Some(sefaz_client);
        }

        Ok(info)
    }

    /// Parse XML de NF-e
    async fn parse_xml(&self, xml: String) -> GqlResult<NfeType> {
        // Usar parser existente
        let xml_clean = xml.replace("xmlns=\"http://www.portalfiscal.inf.br/nfe\"", "");

        let nfe: nfe_parser::Nfe = xml_clean.parse()
            .map_err(|e| async_graphql::Error::new(format!("Erro ao parsear XML: {}", e)))?;

        Ok(NfeType {
            id: nfe.chave_acesso.clone(),
            chave_acesso: nfe.chave_acesso,
            numero: nfe.ide.numero as i32,
            serie: nfe.ide.serie as i32,
            tipo: TipoDocumento::Nfe,
            ambiente: if nfe.ide.ambiente == nfe_parser::TipoAmbiente::Producao {
                Ambiente::Producao
            } else {
                Ambiente::Homologacao
            },
            status: StatusNfe::Pendente,
            data_emissao: nfe.ide.emissao.horario.format("%Y-%m-%dT%H:%M:%S").to_string(),
            data_autorizacao: None,
            protocolo: None,
            emitente: EmitenteType {
                cnpj: nfe.emit.cnpj.clone().unwrap_or_default(),
                razao_social: nfe.emit.razao_social.clone().unwrap_or_default(),
                nome_fantasia: nfe.emit.nome_fantasia.clone(),
                inscricao_estadual: nfe.emit.ie.clone(),
                endereco: EnderecoType {
                    logradouro: nfe.emit.endereco.logradouro.clone(),
                    numero: nfe.emit.endereco.numero.clone(),
                    complemento: nfe.emit.endereco.complemento.clone(),
                    bairro: nfe.emit.endereco.bairro.clone(),
                    municipio: nfe.emit.endereco.nome_municipio.clone(),
                    uf: nfe.emit.endereco.sigla_uf.clone(),
                    cep: nfe.emit.endereco.cep.clone(),
                    pais: Some("Brasil".to_string()),
                },
            },
            destinatario: nfe.dest.as_ref().map(|d| DestinatarioType {
                cnpj: Some(d.cnpj.clone()),
                cpf: None,
                razao_social: d.razao_social.clone().unwrap_or_default(),
                inscricao_estadual: None,
                endereco: d.endereco.as_ref().map(|e| EnderecoType {
                    logradouro: e.logradouro.clone(),
                    numero: e.numero.clone(),
                    complemento: e.complemento.clone(),
                    bairro: e.bairro.clone(),
                    municipio: e.nome_municipio.clone(),
                    uf: e.sigla_uf.clone(),
                    cep: e.cep.clone(),
                    pais: Some("Brasil".to_string()),
                }),
            }),
            itens: nfe.itens.iter().map(|item| ItemType {
                numero: item.numero as i32,
                codigo: item.produto.codigo.clone(),
                descricao: item.produto.descricao.clone(),
                ncm: item.produto.ncm.clone(),
                cfop: item.produto.tributacao.cfop.clone(),
                unidade: item.produto.unidade.clone(),
                quantidade: item.produto.quantidade as f64,
                valor_unitario: item.produto.valor_unitario as f64,
                valor_total: item.produto.valor_bruto as f64,
            }).collect(),
            totais: TotaisType {
                base_calculo_icms: nfe.totais.valor_base_calculo as f64,
                valor_icms: nfe.totais.valor_icms as f64,
                valor_produtos: nfe.totais.valor_produtos as f64,
                valor_frete: nfe.totais.valor_frete as f64,
                valor_desconto: nfe.totais.valor_desconto as f64,
                valor_total: nfe.totais.valor_total as f64,
            },
            xml: Some(xml),
        })
    }
}

/// Gera XML de NF-e a partir do input
fn gerar_xml_nfe(input: &NfeInput) -> Result<String, async_graphql::Error> {
    let mut xml = String::new();

    // Header NFe
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    xml.push_str("<NFe xmlns=\"http://www.portalfiscal.inf.br/nfe\">");

    // infNFe - gerar ID (chave de acesso será calculada)
    let id_placeholder = format!("NFe{}", chrono::Utc::now().format("%Y%m%d%H%M%S%f"));
    xml.push_str(&format!("<infNFe versao=\"4.00\" Id=\"{}\">", id_placeholder));

    // ide - Identificação
    xml.push_str("<ide>");
    xml.push_str(&format!("<cUF>{}</cUF>", get_codigo_uf(&input.emitente.endereco.uf)));
    xml.push_str(&format!("<cNF>{:08}</cNF>", chrono::Utc::now().timestamp() % 100000000));
    xml.push_str("<natOp>Venda de mercadoria</natOp>");
    // NF-e modelo 55 por padrão
    xml.push_str("<mod>55</mod>");
    xml.push_str(&format!("<serie>{}</serie>", input.serie));
    xml.push_str(&format!("<nNF>{}</nNF>", input.numero));
    xml.push_str(&format!("<dhEmi>{}</dhEmi>", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S-03:00")));
    xml.push_str("<tpNF>1</tpNF>"); // Saída
    xml.push_str("<idDest>1</idDest>"); // Operação interna
    xml.push_str(&format!("<cMunFG>{}</cMunFG>", get_codigo_municipio(&input.emitente.endereco.municipio)));
    xml.push_str("<tpImp>1</tpImp>");
    xml.push_str("<tpEmis>1</tpEmis>");
    xml.push_str("<tpAmb>2</tpAmb>"); // Homologação por padrão
    xml.push_str("<finNFe>1</finNFe>");
    xml.push_str("<indFinal>1</indFinal>");
    xml.push_str("<indPres>1</indPres>");
    xml.push_str("<procEmi>0</procEmi>");
    xml.push_str("<verProc>NfeWeb1.0</verProc>");
    xml.push_str("</ide>");

    // emit - Emitente
    xml.push_str("<emit>");
    xml.push_str(&format!("<CNPJ>{}</CNPJ>", input.emitente.cnpj));
    xml.push_str(&format!("<xNome>{}</xNome>", input.emitente.razao_social));
    if let Some(ref fantasia) = input.emitente.nome_fantasia {
        xml.push_str(&format!("<xFant>{}</xFant>", fantasia));
    }
    xml.push_str("<enderEmit>");
    xml.push_str(&format!("<xLgr>{}</xLgr>", input.emitente.endereco.logradouro));
    xml.push_str(&format!("<nro>{}</nro>", input.emitente.endereco.numero));
    xml.push_str(&format!("<xBairro>{}</xBairro>", input.emitente.endereco.bairro));
    xml.push_str(&format!("<cMun>{}</cMun>", get_codigo_municipio(&input.emitente.endereco.municipio)));
    xml.push_str(&format!("<xMun>{}</xMun>", input.emitente.endereco.municipio));
    xml.push_str(&format!("<UF>{}</UF>", input.emitente.endereco.uf));
    xml.push_str(&format!("<CEP>{}</CEP>", input.emitente.endereco.cep));
    xml.push_str("<cPais>1058</cPais>");
    xml.push_str("<xPais>Brasil</xPais>");
    xml.push_str("</enderEmit>");
    if let Some(ref ie) = input.emitente.inscricao_estadual {
        xml.push_str(&format!("<IE>{}</IE>", ie));
    }
    xml.push_str("<CRT>1</CRT>"); // Simples Nacional
    xml.push_str("</emit>");

    // dest - Destinatário
    if let Some(ref dest) = input.destinatario {
        xml.push_str("<dest>");
        if let Some(ref cnpj) = dest.cnpj {
            xml.push_str(&format!("<CNPJ>{}</CNPJ>", cnpj));
        } else if let Some(ref cpf) = dest.cpf {
            xml.push_str(&format!("<CPF>{}</CPF>", cpf));
        }
        xml.push_str(&format!("<xNome>{}</xNome>", dest.razao_social));
        if let Some(ref end) = dest.endereco {
            xml.push_str("<enderDest>");
            xml.push_str(&format!("<xLgr>{}</xLgr>", end.logradouro));
            xml.push_str(&format!("<nro>{}</nro>", end.numero));
            xml.push_str(&format!("<xBairro>{}</xBairro>", end.bairro));
            xml.push_str(&format!("<cMun>{}</cMun>", get_codigo_municipio(&end.municipio)));
            xml.push_str(&format!("<xMun>{}</xMun>", end.municipio));
            xml.push_str(&format!("<UF>{}</UF>", end.uf));
            xml.push_str(&format!("<CEP>{}</CEP>", end.cep));
            xml.push_str("<cPais>1058</cPais>");
            xml.push_str("<xPais>Brasil</xPais>");
            xml.push_str("</enderDest>");
        }
        xml.push_str("<indIEDest>9</indIEDest>");
        xml.push_str("</dest>");
    }

    // det - Itens
    for (i, item) in input.itens.iter().enumerate() {
        xml.push_str(&format!("<det nItem=\"{}\">", i + 1));
        xml.push_str("<prod>");
        xml.push_str(&format!("<cProd>{}</cProd>", item.codigo));
        xml.push_str("<cEAN>SEM GTIN</cEAN>");
        xml.push_str(&format!("<xProd>{}</xProd>", item.descricao));
        xml.push_str(&format!("<NCM>{}</NCM>", item.ncm));
        xml.push_str(&format!("<CFOP>{}</CFOP>", item.cfop));
        xml.push_str(&format!("<uCom>{}</uCom>", item.unidade));
        xml.push_str(&format!("<qCom>{:.4}</qCom>", item.quantidade));
        xml.push_str(&format!("<vUnCom>{:.4}</vUnCom>", item.valor_unitario));
        xml.push_str(&format!("<vProd>{:.2}</vProd>", item.quantidade * item.valor_unitario));
        xml.push_str("<cEANTrib>SEM GTIN</cEANTrib>");
        xml.push_str(&format!("<uTrib>{}</uTrib>", item.unidade));
        xml.push_str(&format!("<qTrib>{:.4}</qTrib>", item.quantidade));
        xml.push_str(&format!("<vUnTrib>{:.4}</vUnTrib>", item.valor_unitario));
        xml.push_str("<indTot>1</indTot>");
        xml.push_str("</prod>");
        xml.push_str("<imposto>");
        xml.push_str("<ICMS><ICMSSN102><orig>0</orig><CSOSN>102</CSOSN></ICMSSN102></ICMS>");
        xml.push_str("<PIS><PISOutr><CST>99</CST><vBC>0.00</vBC><pPIS>0.00</pPIS><vPIS>0.00</vPIS></PISOutr></PIS>");
        xml.push_str("<COFINS><COFINSOutr><CST>99</CST><vBC>0.00</vBC><pCOFINS>0.00</pCOFINS><vCOFINS>0.00</vCOFINS></COFINSOutr></COFINS>");
        xml.push_str("</imposto>");
        xml.push_str("</det>");
    }

    // total
    let total_produtos: f64 = input.itens.iter().map(|i| i.quantidade * i.valor_unitario).sum();
    xml.push_str("<total>");
    xml.push_str("<ICMSTot>");
    xml.push_str("<vBC>0.00</vBC>");
    xml.push_str("<vICMS>0.00</vICMS>");
    xml.push_str("<vICMSDeson>0.00</vICMSDeson>");
    xml.push_str("<vFCP>0.00</vFCP>");
    xml.push_str("<vBCST>0.00</vBCST>");
    xml.push_str("<vST>0.00</vST>");
    xml.push_str("<vFCPST>0.00</vFCPST>");
    xml.push_str("<vFCPSTRet>0.00</vFCPSTRet>");
    xml.push_str(&format!("<vProd>{:.2}</vProd>", total_produtos));
    xml.push_str("<vFrete>0.00</vFrete>");
    xml.push_str("<vSeg>0.00</vSeg>");
    xml.push_str("<vDesc>0.00</vDesc>");
    xml.push_str("<vII>0.00</vII>");
    xml.push_str("<vIPI>0.00</vIPI>");
    xml.push_str("<vIPIDevol>0.00</vIPIDevol>");
    xml.push_str("<vPIS>0.00</vPIS>");
    xml.push_str("<vCOFINS>0.00</vCOFINS>");
    xml.push_str("<vOutro>0.00</vOutro>");
    xml.push_str(&format!("<vNF>{:.2}</vNF>", total_produtos));
    xml.push_str("</ICMSTot>");
    xml.push_str("</total>");

    // transp
    xml.push_str("<transp>");
    xml.push_str("<modFrete>9</modFrete>");
    xml.push_str("</transp>");

    // pag
    xml.push_str("<pag>");
    xml.push_str("<detPag>");
    xml.push_str("<tPag>01</tPag>");
    xml.push_str(&format!("<vPag>{:.2}</vPag>", total_produtos));
    xml.push_str("</detPag>");
    xml.push_str("</pag>");

    xml.push_str("</infNFe>");
    xml.push_str("</NFe>");

    Ok(xml)
}

/// Obtém código UF
fn get_codigo_uf(uf: &str) -> &str {
    match uf {
        "AC" => "12", "AL" => "27", "AP" => "16", "AM" => "13", "BA" => "29",
        "CE" => "23", "DF" => "53", "ES" => "32", "GO" => "52", "MA" => "21",
        "MT" => "51", "MS" => "50", "MG" => "31", "PA" => "15", "PB" => "25",
        "PR" => "41", "PE" => "26", "PI" => "22", "RJ" => "33", "RN" => "24",
        "RS" => "43", "RO" => "11", "RR" => "14", "SC" => "42", "SP" => "35",
        "SE" => "28", "TO" => "17", _ => "35",
    }
}

/// Obtém código município (placeholder)
fn get_codigo_municipio(municipio: &str) -> String {
    // Em produção, usar tabela IBGE
    format!("{}0000", municipio.len())
}
