//! Resolvers GraphQL

use super::types::*;
use crate::sefaz;
use async_graphql::{Context, Object, Result as GqlResult};

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

    /// Consulta status no SEFAZ
    async fn consultar_sefaz(&self, chave_acesso: String) -> GqlResult<ConsultaSefazResult> {
        let info = sefaz::validar_chave_acesso(&chave_acesso)
            .map_err(|e| async_graphql::Error::new(e))?;

        let url = sefaz::gerar_url_consulta_portal(&chave_acesso);

        Ok(ConsultaSefazResult {
            sucesso: true,
            codigo_status: "100".to_string(),
            motivo: "Chave válida - acesse URL para consulta completa".to_string(),
            chave_acesso: Some(info.chave),
            protocolo: None,
            data_recebimento: None,
            situacao: Some(format!("URL: {}", url)),
        })
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
        // Verificar se há certificado no contexto
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

/// Mutation root
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Emite NF-e no SEFAZ
    async fn emitir_nfe(&self, input: NfeInput) -> GqlResult<EmissaoResult> {
        // Validar dados
        if input.itens.is_empty() {
            return Err(async_graphql::Error::new("NF-e deve ter pelo menos um item"));
        }

        // Em produção:
        // 1. Gerar XML
        // 2. Assinar com certificado
        // 3. Enviar para SEFAZ
        // 4. Processar retorno

        Ok(EmissaoResult {
            sucesso: false,
            codigo_status: "999".to_string(),
            motivo: "Emissão requer certificado digital configurado".to_string(),
            chave_acesso: None,
            protocolo: None,
            xml_autorizado: None,
        })
    }

    /// Cancela NF-e no SEFAZ
    async fn cancelar_nfe(&self, input: CancelamentoInput) -> GqlResult<CancelamentoResult> {
        // Validar chave
        let _info = sefaz::validar_chave_acesso(&input.chave_acesso)
            .map_err(|e| async_graphql::Error::new(e))?;

        // Validar justificativa (mínimo 15 caracteres)
        if input.justificativa.len() < 15 {
            return Err(async_graphql::Error::new("Justificativa deve ter no mínimo 15 caracteres"));
        }

        // Em produção:
        // 1. Gerar XML do evento de cancelamento
        // 2. Assinar com certificado
        // 3. Enviar para SEFAZ
        // 4. Processar retorno

        Ok(CancelamentoResult {
            sucesso: false,
            codigo_status: "999".to_string(),
            motivo: "Cancelamento requer certificado digital configurado".to_string(),
            protocolo: None,
            data_cancelamento: None,
        })
    }

    /// Envia carta de correção
    async fn carta_correcao(&self, input: CartaCorrecaoInput) -> GqlResult<EmissaoResult> {
        // Validar chave
        let _info = sefaz::validar_chave_acesso(&input.chave_acesso)
            .map_err(|e| async_graphql::Error::new(e))?;

        // Validar correção (mínimo 15 caracteres)
        if input.correcao.len() < 15 {
            return Err(async_graphql::Error::new("Correção deve ter no mínimo 15 caracteres"));
        }

        Ok(EmissaoResult {
            sucesso: false,
            codigo_status: "999".to_string(),
            motivo: "Carta de correção requer certificado digital configurado".to_string(),
            chave_acesso: None,
            protocolo: None,
            xml_autorizado: None,
        })
    }

    /// Carrega certificado digital
    async fn carregar_certificado(
        &self,
        pfx_base64: String,
        senha: String,
    ) -> GqlResult<CertificadoInfoType> {
        use base64::Engine;

        let pfx_bytes = base64::engine::general_purpose::STANDARD
            .decode(&pfx_base64)
            .map_err(|e| async_graphql::Error::new(format!("Erro ao decodificar certificado: {}", e)))?;

        let cert = crate::certificado::CertificadoA1::from_bytes(&pfx_bytes, &senha)
            .map_err(|e| async_graphql::Error::new(e))?;

        Ok(CertificadoInfoType {
            cnpj: cert.info.cnpj,
            razao_social: cert.info.razao_social,
            valido: cert.info.valido,
            data_validade: cert.info.not_after,
            dias_para_expirar: cert.info.dias_para_expirar as i32,
        })
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
