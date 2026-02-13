//! Leitor de documentos fiscais em PDF
//!
//! Suporta: NF-e, NFC-e, NFS-e, CT-e, MDF-e, NFA-e, CF-e SAT

use serde::{Deserialize, Serialize};
use regex::Regex;

/// Tipo de documento fiscal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TipoDocumentoFiscal {
    #[serde(rename = "NFe")]
    Nfe,           // Nota Fiscal Eletrônica (ICMS) - Modelo 55
    #[serde(rename = "NFCe")]
    Nfce,          // Nota Fiscal Consumidor Eletrônica - Modelo 65
    #[serde(rename = "NFSe")]
    Nfse,          // Nota Fiscal de Serviços Eletrônica (ISS)
    #[serde(rename = "CTe")]
    Cte,           // Conhecimento de Transporte Eletrônico
    #[serde(rename = "MDFe")]
    Mdfe,          // Manifesto Eletrônico de Documentos Fiscais
    #[serde(rename = "NFAe")]
    Nfae,          // Nota Fiscal Avulsa Eletrônica
    #[serde(rename = "CFeSAT")]
    CfeSat,        // Cupom Fiscal Eletrônico SAT
    #[serde(rename = "Desconhecido")]
    Desconhecido,
}

/// Dados extraídos do documento fiscal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanfeData {
    pub tipo_documento: TipoDocumentoFiscal,

    // Identificação
    pub chave_acesso: Option<String>,
    pub numero: Option<i32>,
    pub serie: Option<i16>,
    pub numero_rps: Option<i32>,
    pub data_emissao: Option<String>,
    pub codigo_verificacao: Option<String>,
    pub competencia: Option<String>,

    // Emitente/Prestador
    pub emit_cnpj: Option<String>,
    pub emit_cpf: Option<String>,
    pub emit_razao_social: Option<String>,
    pub emit_nome_fantasia: Option<String>,
    pub emit_inscricao_estadual: Option<String>,
    pub emit_inscricao_municipal: Option<String>,
    pub emit_endereco: Option<String>,
    pub emit_complemento: Option<String>,
    pub emit_bairro: Option<String>,
    pub emit_municipio: Option<String>,
    pub emit_uf: Option<String>,
    pub emit_cep: Option<String>,
    pub emit_telefone: Option<String>,
    pub emit_email: Option<String>,

    // Destinatário/Tomador
    pub dest_cnpj: Option<String>,
    pub dest_cpf: Option<String>,
    pub dest_razao_social: Option<String>,
    pub dest_endereco: Option<String>,
    pub dest_complemento: Option<String>,
    pub dest_bairro: Option<String>,
    pub dest_municipio: Option<String>,
    pub dest_uf: Option<String>,
    pub dest_cep: Option<String>,

    // Valores
    pub valor_total: Option<f64>,
    pub valor_servicos: Option<f64>,
    pub valor_produtos: Option<f64>,
    pub valor_frete: Option<f64>,
    pub valor_desconto: Option<f64>,
    pub valor_liquido: Option<f64>,
    pub base_calculo: Option<f64>,
    pub aliquota: Option<f64>,
    pub valor_iss: Option<f64>,
    pub valor_icms: Option<f64>,
    pub valor_pis: Option<f64>,
    pub valor_cofins: Option<f64>,
    pub valor_ir: Option<f64>,
    pub valor_inss: Option<f64>,
    pub valor_csll: Option<f64>,
    pub valor_deducoes: Option<f64>,
    pub valor_outras_retencoes: Option<f64>,
    pub tributos_aproximados: Option<f64>,

    // Serviço (NFS-e)
    pub codigo_servico: Option<String>,
    pub descricao_servico: Option<String>,
    pub discriminacao_servico: Option<String>,
    pub natureza_operacao: Option<String>,
    pub regime_tributacao: Option<String>,
    pub municipio_prestacao: Option<String>,
    pub optante_simples: Option<bool>,
    pub incentivador_cultural: Option<bool>,
    pub iss_retido: Option<bool>,

    // Transporte (CT-e/MDF-e)
    pub modal_transporte: Option<String>,
    pub placa_veiculo: Option<String>,
    pub uf_veiculo: Option<String>,
    pub rntrc: Option<String>,

    // Itens
    pub itens: Vec<ItemDanfe>,

    // Texto completo extraído
    pub texto_completo: String,
}

/// Item extraído do documento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDanfe {
    pub numero: Option<i32>,
    pub codigo: Option<String>,
    pub descricao: Option<String>,
    pub ncm: Option<String>,
    pub cfop: Option<String>,
    pub unidade: Option<String>,
    pub quantidade: Option<f64>,
    pub valor_unitario: Option<f64>,
    pub valor_total: Option<f64>,
}

impl Default for DanfeData {
    fn default() -> Self {
        Self {
            tipo_documento: TipoDocumentoFiscal::Desconhecido,
            chave_acesso: None,
            numero: None,
            serie: None,
            numero_rps: None,
            data_emissao: None,
            codigo_verificacao: None,
            competencia: None,
            emit_cnpj: None,
            emit_cpf: None,
            emit_razao_social: None,
            emit_nome_fantasia: None,
            emit_inscricao_estadual: None,
            emit_inscricao_municipal: None,
            emit_endereco: None,
            emit_complemento: None,
            emit_bairro: None,
            emit_municipio: None,
            emit_uf: None,
            emit_cep: None,
            emit_telefone: None,
            emit_email: None,
            dest_cnpj: None,
            dest_cpf: None,
            dest_razao_social: None,
            dest_endereco: None,
            dest_complemento: None,
            dest_bairro: None,
            dest_municipio: None,
            dest_uf: None,
            dest_cep: None,
            valor_total: None,
            valor_servicos: None,
            valor_produtos: None,
            valor_frete: None,
            valor_desconto: None,
            valor_liquido: None,
            base_calculo: None,
            aliquota: None,
            valor_iss: None,
            valor_icms: None,
            valor_pis: None,
            valor_cofins: None,
            valor_ir: None,
            valor_inss: None,
            valor_csll: None,
            valor_deducoes: None,
            valor_outras_retencoes: None,
            tributos_aproximados: None,
            codigo_servico: None,
            descricao_servico: None,
            discriminacao_servico: None,
            natureza_operacao: None,
            regime_tributacao: None,
            municipio_prestacao: None,
            optante_simples: None,
            incentivador_cultural: None,
            iss_retido: None,
            modal_transporte: None,
            placa_veiculo: None,
            uf_veiculo: None,
            rntrc: None,
            itens: Vec::new(),
            texto_completo: String::new(),
        }
    }
}

/// Extrai dados de um documento fiscal em PDF
pub fn extract_danfe_data(pdf_bytes: &[u8]) -> Result<DanfeData, String> {
    let text = pdf_extract::extract_text_from_mem(pdf_bytes)
        .map_err(|e| format!("Erro ao extrair texto do PDF: {}", e))?;

    let mut data = DanfeData::default();
    data.texto_completo = text.clone();

    // Detectar tipo de documento
    data.tipo_documento = detect_document_type(&text);

    // Extrair dados baseado no tipo
    match data.tipo_documento {
        TipoDocumentoFiscal::Nfse => extract_nfse_data(&text, &mut data),
        TipoDocumentoFiscal::Nfe | TipoDocumentoFiscal::Nfce => extract_nfe_data(&text, &mut data),
        TipoDocumentoFiscal::Cte => extract_cte_data(&text, &mut data),
        TipoDocumentoFiscal::Mdfe => extract_mdfe_data(&text, &mut data),
        TipoDocumentoFiscal::CfeSat => extract_cfe_data(&text, &mut data),
        _ => {
            // Tentar extrair o máximo possível
            extract_generic_data(&text, &mut data);
        }
    }

    Ok(data)
}

/// Detecta o tipo de documento fiscal
fn detect_document_type(text: &str) -> TipoDocumentoFiscal {
    let text_upper = text.to_uppercase();

    // NFS-e (Nota Fiscal de Serviços)
    if text_upper.contains("NFS-E")
        || text_upper.contains("NOTA FISCAL DE SERVIÇO")
        || text_upper.contains("NOTA FISCAL ELETRÔNICA DE SERVIÇO")
        || text_upper.contains("NOTA FISCAL ELETRONICA DE SERVICO")
        || (text_upper.contains("PRESTADOR DE SERVIÇO") && text_upper.contains("TOMADOR DE SERVIÇO"))
        || text_upper.contains("ISSQN") {
        return TipoDocumentoFiscal::Nfse;
    }

    // CT-e (Conhecimento de Transporte)
    if text_upper.contains("CT-E")
        || text_upper.contains("CONHECIMENTO DE TRANSPORTE")
        || text_upper.contains("DACTE") {
        return TipoDocumentoFiscal::Cte;
    }

    // MDF-e (Manifesto de Documentos Fiscais)
    if text_upper.contains("MDF-E")
        || text_upper.contains("MANIFESTO ELETRÔNICO")
        || text_upper.contains("DAMDFE") {
        return TipoDocumentoFiscal::Mdfe;
    }

    // CF-e SAT
    if text_upper.contains("CF-E")
        || text_upper.contains("CUPOM FISCAL ELETRÔNICO")
        || text_upper.contains("SAT") && text_upper.contains("CUPOM") {
        return TipoDocumentoFiscal::CfeSat;
    }

    // NFC-e (Nota Fiscal Consumidor)
    if text_upper.contains("NFC-E")
        || text_upper.contains("NOTA FISCAL DE CONSUMIDOR")
        || text_upper.contains("DANFE NFC-E") {
        return TipoDocumentoFiscal::Nfce;
    }

    // NFA-e (Nota Fiscal Avulsa)
    if text_upper.contains("NFA-E")
        || text_upper.contains("NOTA FISCAL AVULSA") {
        return TipoDocumentoFiscal::Nfae;
    }

    // NF-e (Nota Fiscal Eletrônica)
    if text_upper.contains("DANFE")
        || text_upper.contains("NF-E")
        || text_upper.contains("NOTA FISCAL ELETRÔNICA")
        || text_upper.contains("DOCUMENTO AUXILIAR") {
        return TipoDocumentoFiscal::Nfe;
    }

    TipoDocumentoFiscal::Desconhecido
}

// ============================================================================
// Extração de NFS-e
// ============================================================================

fn extract_nfse_data(text: &str, data: &mut DanfeData) {
    // Número da NFS-e - procurar após "FAZENDA" ou padrão de 6 dígitos isolado
    data.numero = extract_numero_nfse(text);

    // Número do RPS
    data.numero_rps = extract_numero_rps(text);

    // Código de verificação (9 caracteres alfanuméricos)
    data.codigo_verificacao = extract_codigo_verificacao(text);

    // Data de emissão
    data.data_emissao = extract_data_hora(text);

    // Extrair todos os CNPJs do texto
    let cnpjs = extract_all_cnpjs(text);
    if !cnpjs.is_empty() {
        data.emit_cnpj = Some(cnpjs[0].clone());
    }

    // Extrair todos os CPFs do texto
    let cpfs = extract_all_cpfs(text);

    // Inscrição Municipal
    data.emit_inscricao_municipal = extract_inscricao_municipal(text);

    // Razão Social do Prestador (empresa com LTDA, S/A, etc)
    // Procurar todas as empresas no texto e pegar a que tem LTDA
    data.emit_razao_social = extract_empresa_name(text);

    // Nome Fantasia
    data.emit_nome_fantasia = extract_nome_fantasia(text);

    // Endereços
    let enderecos = extract_all_enderecos(text);
    if !enderecos.is_empty() {
        data.emit_endereco = Some(enderecos[0].clone());
        if enderecos.len() > 1 {
            data.dest_endereco = Some(enderecos[1].clone());
        }
    }

    // CEPs
    let ceps = extract_all_ceps(text);
    if !ceps.is_empty() {
        data.emit_cep = Some(ceps[0].clone());
        if ceps.len() > 1 {
            data.dest_cep = Some(ceps[1].clone());
        }
    }

    // Telefone
    data.emit_telefone = extract_telefone(text);

    // Email
    data.emit_email = extract_email(text);

    // Município do Prestador
    let municipios = extract_municipios_uf(text);
    if !municipios.is_empty() {
        data.emit_municipio = Some(municipios[0].0.clone());
        data.emit_uf = Some(municipios[0].1.clone());
        if municipios.len() > 1 {
            data.dest_municipio = Some(municipios[1].0.clone());
            data.dest_uf = Some(municipios[1].1.clone());
        }
    }

    // Tomador - nome de pessoa (antes do CPF)
    data.dest_razao_social = extract_pessoa_nome(text);

    // CPF do Tomador
    if !cpfs.is_empty() {
        data.dest_cpf = Some(cpfs[0].clone());
    }

    // Código do Serviço
    data.codigo_servico = extract_codigo_servico(text);

    // Descrição do Serviço
    data.descricao_servico = extract_descricao_servico(text);

    // Discriminação do Serviço
    data.discriminacao_servico = extract_discriminacao(text);

    // Valor do serviço - tentar extração específica primeiro
    if let Some(v) = extract_valor_servico_nfse(text) {
        data.valor_servicos = Some(v);
        data.valor_total = Some(v);
    }

    // Se não encontrou, tentar padrão genérico (ignorar valores zero)
    if data.valor_servicos.is_none() || data.valor_servicos == Some(0.0) {
        if let Some(v) = extract_valor_pattern(text, &["Valor do Serviço", "Valor Serviço"]) {
            if v > 0.0 {
                data.valor_servicos = Some(v);
                data.valor_total = Some(v);
            }
        }
    }

    // Valor líquido
    if let Some(v) = extract_valor_pattern(text, &["Valor Líquido", "Valor Liquido", "(=)  Valor Líquido"]) {
        data.valor_liquido = Some(v);
    }

    // Base de cálculo
    if let Some(v) = extract_valor_pattern(text, &["Base de Cálculo", "Base Cálculo", "Base de Calculo"]) {
        data.base_calculo = Some(v);
    }

    // Alíquota
    data.aliquota = extract_aliquota(text);

    // Valor do ISS
    if let Some(v) = extract_valor_pattern(text, &["Valor do ISSQN", "Valor ISSQN", "Valor do ISS", "Valor ISS"]) {
        data.valor_iss = Some(v);
    }

    // Tributos aproximados
    data.tributos_aproximados = extract_tributos_aproximados(text);

    // Natureza da Operação
    data.natureza_operacao = extract_natureza_operacao(text);

    // Regime de Tributação
    data.regime_tributacao = extract_regime_tributacao(text);

    // Local da Prestação
    data.municipio_prestacao = extract_local_prestacao(text);

    // Opções booleanas
    data.optante_simples = extract_sim_nao(text, "Simples Nacional");
    data.incentivador_cultural = extract_sim_nao(text, "Incentivador Cultural");
    data.iss_retido = extract_sim_nao(text, "ISS Retido");
}

// ============================================================================
// Extração de NF-e / NFC-e
// ============================================================================

fn extract_nfe_data(text: &str, data: &mut DanfeData) {
    // Chave de acesso (44 dígitos)
    data.chave_acesso = extract_chave_acesso(text);

    // Número e série
    data.numero = extract_numero_nfe(text);
    data.serie = extract_serie(text);

    // Data de emissão
    data.data_emissao = extract_data_hora(text);

    // Dados do emitente
    let cnpjs = extract_all_cnpjs(text);
    if !cnpjs.is_empty() {
        data.emit_cnpj = Some(cnpjs[0].clone());
    }

    data.emit_razao_social = extract_empresa_name(text);
    data.emit_inscricao_estadual = extract_inscricao_estadual(text);

    // Endereço
    let enderecos = extract_all_enderecos(text);
    if !enderecos.is_empty() {
        data.emit_endereco = Some(enderecos[0].clone());
    }

    // Valores
    if let Some(v) = extract_valor_pattern(text, &["Valor Total", "VALOR TOTAL"]) {
        data.valor_total = Some(v);
    }
    if let Some(v) = extract_valor_pattern(text, &["Valor Produtos", "VALOR PRODUTOS"]) {
        data.valor_produtos = Some(v);
    }
}

// ============================================================================
// Extração de CT-e
// ============================================================================

fn extract_cte_data(text: &str, data: &mut DanfeData) {
    data.chave_acesso = extract_chave_acesso(text);
    data.numero = extract_numero_nfe(text);
    data.data_emissao = extract_data_hora(text);

    // Modal de transporte
    data.modal_transporte = extract_modal_transporte(text);

    // Placa do veículo
    data.placa_veiculo = extract_placa(text);

    // RNTRC
    data.rntrc = extract_rntrc(text);

    let cnpjs = extract_all_cnpjs(text);
    if !cnpjs.is_empty() {
        data.emit_cnpj = Some(cnpjs[0].clone());
    }

    data.emit_razao_social = extract_empresa_name(text);
}

// ============================================================================
// Extração de MDF-e
// ============================================================================

fn extract_mdfe_data(text: &str, data: &mut DanfeData) {
    data.chave_acesso = extract_chave_acesso(text);
    data.numero = extract_numero_nfe(text);
    data.data_emissao = extract_data_hora(text);

    data.modal_transporte = extract_modal_transporte(text);
    data.placa_veiculo = extract_placa(text);

    let cnpjs = extract_all_cnpjs(text);
    if !cnpjs.is_empty() {
        data.emit_cnpj = Some(cnpjs[0].clone());
    }
}

// ============================================================================
// Extração de CF-e SAT
// ============================================================================

fn extract_cfe_data(text: &str, data: &mut DanfeData) {
    data.chave_acesso = extract_chave_acesso(text);
    data.data_emissao = extract_data_hora(text);

    let cnpjs = extract_all_cnpjs(text);
    if !cnpjs.is_empty() {
        data.emit_cnpj = Some(cnpjs[0].clone());
    }

    if let Some(v) = extract_valor_pattern(text, &["Valor Total", "TOTAL"]) {
        data.valor_total = Some(v);
    }
}

// ============================================================================
// Extração Genérica
// ============================================================================

fn extract_generic_data(text: &str, data: &mut DanfeData) {
    extract_nfe_data(text, data);
    extract_nfse_data(text, data);
}

// ============================================================================
// Funções de Extração Específicas
// ============================================================================

fn extract_numero_nfse(text: &str) -> Option<i32> {
    // Procurar número após "FAZENDA" (padrão comum em NFS-e municipal)
    if let Some(m) = regex_find(text, r"FAZENDA\s*\n?\s*(\d{5,9})") {
        if let Ok(n) = m.parse() {
            return Some(n);
        }
    }

    // Procurar "Número da NFS-e"
    if let Some(m) = regex_find(text, r"(?i)N[úu]mero\s*(?:da)?\s*\n?\s*NFS-?e\s*\n?\s*(\d+)") {
        if let Ok(n) = m.parse() {
            return Some(n);
        }
    }

    // Número isolado de 6 dígitos após quebra de linha
    if let Some(m) = regex_find(text, r"\n\s*(\d{6})\s*\n") {
        if let Ok(n) = m.parse() {
            if n > 100000 {
                return Some(n);
            }
        }
    }

    None
}

fn extract_numero_rps(text: &str) -> Option<i32> {
    if let Some(m) = regex_find(text, r"(?i)(?:N[úu]mero\s*(?:do)?\s*)?RPS\s*\n?\s*(\d+)") {
        if let Ok(n) = m.parse() {
            return Some(n);
        }
    }

    // Segundo número de 6 dígitos (geralmente RPS)
    if let Ok(re) = Regex::new(r"\n\s*(\d{6})\s*\n") {
        let matches: Vec<_> = re.find_iter(text).collect();
        if matches.len() >= 2 {
            if let Ok(n) = matches[1].as_str().trim().parse() {
                return Some(n);
            }
        }
    }

    None
}

fn extract_codigo_verificacao(text: &str) -> Option<String> {
    // Procurar código de 9 letras após data
    if let Some(m) = regex_find(text, r"\d{1,2}/\d{1,2}/\d{4}\s+([A-Z]{9})\b") {
        return Some(m);
    }

    // Código alfanumérico de 8-12 caracteres
    if let Ok(re) = Regex::new(r"\b([A-Z0-9]{8,12})\b") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                let code = m.as_str();
                // Filtrar palavras comuns
                if !is_common_word(code) && code.chars().any(|c| c.is_alphabetic()) {
                    return Some(code.to_string());
                }
            }
        }
    }

    None
}

fn extract_data_hora(text: &str) -> Option<String> {
    // Data com hora: DD/MM/YYYY HH:MM:SS
    if let Some(m) = regex_find(text, r"(\d{2}/\d{2}/\d{4}\s+\d{2}:\d{2}:\d{2})") {
        return Some(m);
    }

    // Data com hora sem segundos
    if let Some(m) = regex_find(text, r"(\d{2}/\d{2}/\d{4}\s+\d{2}:\d{2})") {
        return Some(m);
    }

    // Apenas data
    if let Some(m) = regex_find(text, r"(\d{2}/\d{2}/\d{4})") {
        return Some(m);
    }

    None
}

fn extract_all_cnpjs(text: &str) -> Vec<String> {
    let mut cnpjs = Vec::new();

    if let Ok(re) = Regex::new(r"(\d{2}[.\s]?\d{3}[.\s]?\d{3}[/\s]?\d{4}[-\s]?\d{2})") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                let cnpj: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
                if cnpj.len() == 14 && !cnpjs.contains(&cnpj) {
                    cnpjs.push(cnpj);
                }
            }
        }
    }

    cnpjs
}

fn extract_all_cpfs(text: &str) -> Vec<String> {
    let mut cpfs = Vec::new();

    if let Ok(re) = Regex::new(r"(\d{3}[.\s]?\d{3}[.\s]?\d{3}[-\s]?\d{2})") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                let cpf: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
                if cpf.len() == 11 && !cpfs.contains(&cpf) {
                    cpfs.push(cpf);
                }
            }
        }
    }

    cpfs
}

fn extract_inscricao_municipal(text: &str) -> Option<String> {
    // Procurar número de 7 dígitos após CNPJ
    if let Some(m) = regex_find(text, r"\d{2}\.\d{3}\.\d{3}/\d{4}-\d{2}\s+(\d{5,10})") {
        return Some(m);
    }

    if let Some(m) = regex_find(text, r"(?i)Inscri[çc][ãa]o\s*Municipal[:\s]*(\d+)") {
        return Some(m);
    }

    None
}

fn extract_inscricao_estadual(text: &str) -> Option<String> {
    if let Some(m) = regex_find(text, r"(?i)(?:IE|Inscri[çc][ãa]o\s*Estadual)[:\s]*(\d[\d./-]+)") {
        let ie: String = m.chars().filter(|c| c.is_ascii_digit()).collect();
        if ie.len() >= 8 {
            return Some(ie);
        }
    }
    None
}

fn extract_empresa_name(text: &str) -> Option<String> {
    // Procurar nome de empresa com LTDA especificamente (mais confiável)
    // Prioriza empresas com sufixo corporativo
    let patterns = [
        // Padrão específico para empresa com LTDA
        r"\n([A-Z][A-Z0-9\s]+LTDA)\s*\n",
        // Empresa com "MEIOS DE PAGAMENTO" ou similar + LTDA
        r"([A-Z][A-Z0-9\s]*(?:MEIOS DE PAGAMENTO|SERVICOS|COMERCIO)[A-Z0-9\s]*LTDA)",
        // Empresa genérica com LTDA
        r"([A-Z][A-Z0-9\s]{5,}LTDA)",
        // S/A, EIRELI, etc
        r"([A-Z][A-Z0-9\s]+(?:S/?A|EIRELI|EPP))\b",
    ];

    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            for cap in re.captures_iter(text) {
                if let Some(m) = cap.get(1) {
                    let nome = m.as_str().trim();
                    // Filtrar falsos positivos
                    if nome.len() > 10
                        && !nome.contains("PREFEITURA")
                        && !nome.contains("SECRETARIA")
                        && !nome.contains("NOTA FISCAL")
                        && !nome.contains("MUNICIPAL")
                        && !nome.contains("FAZENDA") {
                        return Some(nome.to_string());
                    }
                }
            }
        }
    }

    None
}

fn extract_pessoa_nome(text: &str) -> Option<String> {
    // Procurar nome de pessoa (2-4 palavras em maiúscula, sem termos técnicos)
    if let Ok(re) = Regex::new(r"\n([A-Z][A-Z]+(?:\s+[A-Z][A-Z]+){1,4})\s*\n") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                let nome = m.as_str().trim();
                if nome.len() > 5
                    && !nome.contains("LTDA")
                    && !nome.contains("CNPJ")
                    && !nome.contains("PREFEITURA")
                    && !nome.contains("SECRETARIA")
                    && !nome.contains("MUNICIPAL")
                    && !nome.contains("FISCAL")
                    && !nome.contains("NOTA")
                    && !nome.contains("SERVIÇO")
                    && !nome.contains("SERVICO")
                    && !nome.contains("TOMADOR")
                    && !nome.contains("PRESTADOR")
                    && !nome.contains("ATIVIDADE")
                    && !nome.contains("TRIBUT")
                    && nome.split_whitespace().count() >= 2
                    && nome.split_whitespace().count() <= 5 {
                    return Some(nome.to_string());
                }
            }
        }
    }

    None
}

fn extract_nome_fantasia(text: &str) -> Option<String> {
    if let Some(m) = regex_find(text, r"(?i)Nome\s*Fantasia\s*([A-Z][A-Z\s]+)") {
        let nome = m.trim();
        if nome.len() > 2 {
            return Some(nome.to_string());
        }
    }
    None
}

fn extract_all_enderecos(text: &str) -> Vec<String> {
    let mut enderecos = Vec::new();

    let patterns = [
        r"((?:RUA|AVENIDA|AV\.|R\.|ALAMEDA|AL\.|TRAVESSA|TV\.)\s+[^\n]+CEP[:\s]*\d{5}-?\d{3})",
        r"((?:RUA|AVENIDA|AV\.|R\.)\s+[A-Z][^\n]{10,60})",
    ];

    for pattern in patterns {
        if let Ok(re) = Regex::new(&format!("(?i){}", pattern)) {
            for cap in re.captures_iter(text) {
                if let Some(m) = cap.get(1) {
                    let end = m.as_str().trim().to_string();
                    if !enderecos.contains(&end) {
                        enderecos.push(end);
                    }
                }
            }
        }
    }

    enderecos
}

fn extract_all_ceps(text: &str) -> Vec<String> {
    let mut ceps = Vec::new();

    if let Ok(re) = Regex::new(r"(?i)CEP[:\s]*(\d{5}-?\d{3})") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                let cep: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
                if cep.len() == 8 && !ceps.contains(&cep) {
                    ceps.push(cep);
                }
            }
        }
    }

    ceps
}

fn extract_telefone(text: &str) -> Option<String> {
    if let Some(m) = regex_find(text, r"\(?\d{2}\)?\s*\d{4,5}[-\s]?\d{4}") {
        let tel: String = m.chars().filter(|c| c.is_ascii_digit()).collect();
        if tel.len() >= 10 {
            return Some(tel);
        }
    }
    None
}

fn extract_email(text: &str) -> Option<String> {
    regex_find(text, r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
}

fn extract_municipios_uf(text: &str) -> Vec<(String, String)> {
    let mut municipios = Vec::new();

    if let Ok(re) = Regex::new(r"([A-Z][A-Za-zÀ-ú\s]+)\s*-\s*([A-Z]{2})\b") {
        for cap in re.captures_iter(text) {
            if let (Some(mun), Some(uf)) = (cap.get(1), cap.get(2)) {
                let mut mun_str = mun.as_str().trim().to_string();
                let uf_str = uf.as_str().to_string();

                // Remover texto antes de quebra de linha (ex: "NOME\n CIDADE" -> "CIDADE")
                if let Some(pos) = mun_str.rfind('\n') {
                    mun_str = mun_str[pos+1..].trim().to_string();
                }

                // Filtrar falsos positivos
                if mun_str.len() > 3
                    && !mun_str.contains("NOTA")
                    && !mun_str.contains("FISCAL")
                    && !mun_str.contains("SERVIÇO")
                    && !mun_str.contains("SERVICO")
                    && !mun_str.contains("SILVESTRINI")  // Filtrar nomes próprios
                    && !mun_str.contains("DRESSANO")
                    && mun_str.chars().filter(|c| c.is_whitespace()).count() < 3  // Máx 2 palavras
                    && !municipios.iter().any(|(m, _)| m == &mun_str) {
                    municipios.push((mun_str, uf_str));
                }
            }
        }
    }

    municipios
}

fn extract_codigo_servico(text: &str) -> Option<String> {
    // Padrão: XX.XX / XXXXXXX
    if let Some(m) = regex_find(text, r"(\d{1,2}\.\d{2}\s*/\s*\d{5,8})") {
        return Some(m);
    }
    None
}

fn extract_descricao_servico(text: &str) -> Option<String> {
    // Descrição após código do serviço
    if let Some(m) = regex_find(text, r"\d{1,2}\.\d{2}\s*/\s*\d+\s*-\s*([^\n]+)") {
        return Some(m.trim().to_string());
    }
    None
}

fn extract_discriminacao(text: &str) -> Option<String> {
    // Procurar texto com "PEDIDO:" ou similar
    if let Some(m) = regex_find(text, r"((?:PEDIDO|SERVICO|SERVIÇO)[:\s][^\n]+)") {
        return Some(m.trim().to_string());
    }
    None
}

fn extract_valores_monetarios(text: &str) -> Vec<(f64, usize)> {
    let mut valores = Vec::new();

    if let Ok(re) = Regex::new(r"(\d{1,3}(?:\.\d{3})*,\d{2})") {
        for (i, cap) in re.captures_iter(text).enumerate() {
            if let Some(m) = cap.get(1) {
                if let Some(v) = parse_valor_br(m.as_str()) {
                    valores.push((v, i));
                }
            }
        }
    }

    valores
}

fn extract_valor_pattern(text: &str, labels: &[&str]) -> Option<f64> {
    for label in labels {
        // Padrão 1: valor na mesma linha
        let pattern = format!(r"(?i){}\s*R?\$?\s*(\d{{1,3}}(?:\.\d{{3}})*,\d{{2}})", regex::escape(label));
        if let Some(m) = regex_find(text, &pattern) {
            if let Some(v) = parse_valor_br(&m) {
                return Some(v);
            }
        }

        // Padrão 2: valor em linhas posteriores (comum em NFS-e)
        let pattern2 = format!(r"(?is){}\s*R?\$?\s*\n(?:[^\d\n]*\n)*?(\d{{1,3}}(?:\.\d{{3}})*,\d{{2}})", regex::escape(label));
        if let Ok(re) = Regex::new(&pattern2) {
            if let Some(caps) = re.captures(text) {
                if let Some(m) = caps.get(1) {
                    if let Some(v) = parse_valor_br(m.as_str()) {
                        return Some(v);
                    }
                }
            }
        }
    }
    None
}

fn extract_valor_servico_nfse(text: &str) -> Option<f64> {
    // Padrão 1: "SERVICO ... X.XX" (formato com ponto US - decimal separator)
    // Procura número no formato X.XX após SERVICO
    if let Ok(re) = Regex::new(r"(?i)SERVICO[A-Z\s]+(\d+\.\d{2})") {
        if let Some(caps) = re.captures(text) {
            if let Some(m) = caps.get(1) {
                if let Ok(v) = m.as_str().parse::<f64>() {
                    if v > 0.0 && v < 1_000_000.0 {
                        return Some(v);
                    }
                }
            }
        }
    }

    // Padrão 2: Procurar "X.XX" isolado após SERVICO (formato US)
    if let Ok(re) = Regex::new(r"SERVICO[^\n]+?(\d{1,5}\.\d{2})") {
        if let Some(caps) = re.captures(text) {
            if let Some(m) = caps.get(1) {
                if let Ok(v) = m.as_str().parse::<f64>() {
                    if v > 0.0 && v < 1_000_000.0 {
                        return Some(v);
                    }
                }
            }
        }
    }

    // Padrão 3: "SERVICO" seguido de valor BR (8,00) na mesma linha
    if let Some(m) = regex_find(text, r"(?i)SERVICO[^\n]*?(\d{1,3}(?:\.\d{3})*,\d{2})") {
        if let Some(v) = parse_valor_br(&m) {
            if v > 0.0 {
                return Some(v);
            }
        }
    }

    // Padrão 4: Último valor maior que 5 no documento
    let valores: Vec<f64> = extract_all_monetary_values(text);
    for v in valores.iter().rev() {
        if *v >= 5.0 && *v < 10000.0 {
            return Some(*v);
        }
    }

    None
}

fn extract_all_monetary_values(text: &str) -> Vec<f64> {
    let mut valores = Vec::new();

    if let Ok(re) = Regex::new(r"(\d{1,3}(?:\.\d{3})*,\d{2})") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                if let Some(v) = parse_valor_br(m.as_str()) {
                    valores.push(v);
                }
            }
        }
    }

    valores
}

fn extract_aliquota(text: &str) -> Option<f64> {
    // Padrão 1: Procurar valor típico de alíquota (1-5%) após contexto de ISSQN
    // Valores típicos: 2,00  3,00  5,00
    if let Ok(re) = Regex::new(r"(?is)(?:Al[íi]quota|ISSQN).*?(\d,\d{2})\n") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                if let Some(v) = parse_valor_br(m.as_str()) {
                    // Alíquota típica de ISS é entre 2% e 5%
                    if v >= 2.0 && v <= 5.0 {
                        return Some(v);
                    }
                }
            }
        }
    }

    // Padrão 2: Procurar valor entre 1 e 25 em contexto de porcentagem
    if let Some(m) = regex_find(text, r"(?i)Al[íi]quota\s*%?\s*\n?[^\d]*(\d{1,2}[,.]?\d{0,2})\s*%?") {
        if let Some(v) = parse_valor_br(&m) {
            if v > 0.0 && v <= 25.0 && v != 0.16 {
                return Some(v);
            }
        }
    }

    // Padrão 3: Procurar "2,00" ou similar que é valor típico de alíquota
    if let Ok(re) = Regex::new(r"\n(\d,\d{2})\n") {
        for cap in re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                if let Some(v) = parse_valor_br(m.as_str()) {
                    if v >= 2.0 && v <= 5.0 {
                        return Some(v);
                    }
                }
            }
        }
    }

    None
}

fn extract_tributos_aproximados(text: &str) -> Option<f64> {
    if let Some(m) = regex_find(text, r"(?i)TRIBUTOS\s*(\d{1,3}[,.]?\d{0,2})\s*%") {
        return parse_valor_br(&m);
    }
    None
}

fn extract_natureza_operacao(text: &str) -> Option<String> {
    if let Some(m) = regex_find(text, r"(?i)Natureza\s*Opera[çc][ãa]o\s*\n?\s*(\d+-[^\n]+)") {
        return Some(m.trim().to_string());
    }
    None
}

fn extract_regime_tributacao(text: &str) -> Option<String> {
    if let Some(m) = regex_find(text, r"(?i)Regime\s*(?:Especial\s*)?Tributa[çc][ãa]o\s*\n?\s*(\d+-[^\n]+)") {
        return Some(m.trim().to_string());
    }
    None
}

fn extract_local_prestacao(text: &str) -> Option<String> {
    if let Some(m) = regex_find(text, r"(?i)Local\s*(?:da\s*)?Presta[çc][ãa]o\s*([A-Z][A-Za-zÀ-ú\s]+-\s*[A-Z]{2})") {
        return Some(m.trim().to_string());
    }
    None
}

fn extract_sim_nao(text: &str, label: &str) -> Option<bool> {
    let pattern = format!(r"(?i){}\s*\n?\s*\(?([X\s])\)?\s*Sim\s*\(?([X\s])\)?\s*N[ãa]o", regex::escape(label));
    if let Ok(re) = Regex::new(&pattern) {
        if let Some(cap) = re.captures(text) {
            if let Some(sim) = cap.get(1) {
                return Some(sim.as_str().contains('X'));
            }
        }
    }

    // Padrão alternativo: "2 - Não"
    let alt_pattern = format!(r"(?i){}\s*\n?\s*(\d)\s*-\s*(?:Sim|N[ãa]o)", regex::escape(label));
    if let Some(m) = regex_find(text, &alt_pattern) {
        return Some(m == "1");
    }

    None
}

fn extract_chave_acesso(text: &str) -> Option<String> {
    let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();

    let valid_ufs = [11, 12, 13, 14, 15, 16, 17, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                    31, 32, 33, 35, 41, 42, 43, 50, 51, 52, 53];

    for i in 0..digits.len().saturating_sub(43) {
        let candidate = &digits[i..i+44];
        if let Ok(uf) = candidate[0..2].parse::<u32>() {
            if valid_ufs.contains(&uf) {
                if let Ok(modelo) = candidate[20..22].parse::<u32>() {
                    if modelo == 55 || modelo == 65 || modelo == 57 || modelo == 58 {
                        return Some(candidate.to_string());
                    }
                }
            }
        }
    }

    None
}

fn extract_numero_nfe(text: &str) -> Option<i32> {
    let patterns = [
        r"(?i)N[º°úu][:.\s]*(\d{1,9})",
        r"(?i)NUMERO[:.\s]*(\d{1,9})",
    ];

    for pattern in patterns {
        if let Some(m) = regex_find(text, pattern) {
            if let Ok(n) = m.parse() {
                return Some(n);
            }
        }
    }

    None
}

fn extract_serie(text: &str) -> Option<i16> {
    if let Some(m) = regex_find(text, r"(?i)S[ée]rie[:.\s]*(\d{1,3})") {
        if let Ok(s) = m.parse() {
            return Some(s);
        }
    }
    None
}

fn extract_modal_transporte(text: &str) -> Option<String> {
    let modais = ["RODOVIÁRIO", "RODOVIARIO", "AÉREO", "AEREO", "AQUAVIÁRIO", "AQUAVIARIO",
                  "FERROVIÁRIO", "FERROVIARIO", "DUTOVIÁRIO", "DUTOVIARIO"];

    let text_upper = text.to_uppercase();
    for modal in modais {
        if text_upper.contains(modal) {
            return Some(modal.to_string());
        }
    }

    None
}

fn extract_placa(text: &str) -> Option<String> {
    // Placa padrão antigo: ABC-1234
    if let Some(m) = regex_find(text, r"([A-Z]{3}-?\d{4})") {
        return Some(m);
    }

    // Placa Mercosul: ABC1D23
    if let Some(m) = regex_find(text, r"([A-Z]{3}\d[A-Z]\d{2})") {
        return Some(m);
    }

    None
}

fn extract_rntrc(text: &str) -> Option<String> {
    if let Some(m) = regex_find(text, r"(?i)RNTRC[:.\s]*(\d{8,14})") {
        return Some(m);
    }
    None
}

// ============================================================================
// Funções Auxiliares
// ============================================================================

fn regex_find(text: &str, pattern: &str) -> Option<String> {
    if let Ok(re) = Regex::new(pattern) {
        if let Some(caps) = re.captures(text) {
            if let Some(m) = caps.get(1) {
                return Some(m.as_str().to_string());
            }
        }
    }
    None
}

fn parse_valor_br(valor: &str) -> Option<f64> {
    let valor = valor.trim();
    if valor.contains(',') {
        let valor = valor.replace(".", "").replace(",", ".");
        valor.parse().ok()
    } else {
        valor.parse().ok()
    }
}

fn is_common_word(word: &str) -> bool {
    let common = [
        "MUNICIPAL", "PREFEITURA", "SECRETARIA", "ARARAQUARA", "PIRACICABA",
        "PRESTADOR", "ATIVIDADE", "INCENTIVA", "SERVICOSP", "NACIONAL",
    ];
    common.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valor_br() {
        assert_eq!(parse_valor_br("1.234,56"), Some(1234.56));
        assert_eq!(parse_valor_br("8,00"), Some(8.0));
    }

    #[test]
    fn test_detect_nfse() {
        let text = "NOTA FISCAL ELETRÔNICA DE SERVIÇO - NFS-e";
        assert_eq!(detect_document_type(text), TipoDocumentoFiscal::Nfse);
    }

    #[test]
    fn test_detect_cte() {
        let text = "DACTE - Conhecimento de Transporte";
        assert_eq!(detect_document_type(text), TipoDocumentoFiscal::Cte);
    }
}
