//! NFS-e - Nota Fiscal de Serviços Eletrônica
//!
//! Este módulo contém estruturas para representar NFS-e conforme padrão ABRASF 2.04
//! e suporte a integrações com sistemas municipais.
//!
//! ## Importante
//!
//! A NFS-e não é padronizada nacionalmente. Cada município pode adotar um sistema
//! diferente. Os padrões mais comuns são:
//!
//! - **ABRASF 1.0 / 2.0 / 2.04**: Padrão nacional mais adotado
//! - **Ginfes**: Sistema da Tecnos
//! - **ISSNet**: Sistema da ISSNet
//! - **Betha**: Sistema da Betha Sistemas
//! - **IPM**: Sistema da IPM Informática
//! - **GISS Online**: Sistema de várias prefeituras (Matão, Araraquara, etc.)

use serde::{Deserialize, Serialize};

/// Nota Fiscal de Serviços Eletrônica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nfse {
    /// Identificação da NFS-e
    pub identificacao: IdentificacaoNfse,
    /// Dados do prestador de serviços
    pub prestador: PrestadorServico,
    /// Dados do tomador de serviços
    pub tomador: Option<TomadorServico>,
    /// Intermediário do serviço (se houver)
    pub intermediario: Option<IntermediarioServico>,
    /// Dados do serviço prestado
    pub servico: ServicoNfse,
    /// Valores da NFS-e
    pub valores: ValoresNfse,
    /// Informações complementares
    pub informacoes_complementares: Option<String>,
}

/// Identificação da NFS-e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentificacaoNfse {
    /// Número da NFS-e
    pub numero: u64,
    /// Código de verificação
    pub codigo_verificacao: String,
    /// Data de emissão
    pub data_emissao: String,
    /// Competência (mês/ano de referência)
    pub competencia: String,
    /// Número do RPS que originou a NFS-e (se houver)
    pub numero_rps: Option<u64>,
    /// Série do RPS
    pub serie_rps: Option<String>,
    /// Tipo do RPS (1=RPS, 2=Nota Fiscal Conjugada, 3=Cupom)
    pub tipo_rps: Option<u8>,
    /// Natureza da operação
    pub natureza_operacao: NaturezaOperacaoNfse,
    /// Regime especial de tributação
    pub regime_especial: Option<RegimeEspecialNfse>,
    /// Optante pelo Simples Nacional
    pub optante_simples_nacional: bool,
    /// Incentivador cultural
    pub incentivador_cultural: bool,
    /// Status da NFS-e
    pub status: StatusNfse,
}

/// Natureza da operação para NFS-e
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NaturezaOperacaoNfse {
    /// Tributação no município
    TributacaoMunicipio = 1,
    /// Tributação fora do município
    TributacaoForaMunicipio = 2,
    /// Isenção
    Isencao = 3,
    /// Imune
    Imune = 4,
    /// Exigibilidade suspensa por decisão judicial
    SuspensaJudicial = 5,
    /// Exigibilidade suspensa por procedimento administrativo
    SuspensaAdministrativo = 6,
}

/// Regime especial de tributação
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RegimeEspecialNfse {
    /// Microempresa Municipal
    MicroempresaMunicipal = 1,
    /// Estimativa
    Estimativa = 2,
    /// Sociedade de Profissionais
    SociedadeProfissionais = 3,
    /// Cooperativa
    Cooperativa = 4,
    /// MEI - Microempreendedor Individual
    Mei = 5,
    /// ME EPP - Simples Nacional
    MeEppSimplesNacional = 6,
}

/// Status da NFS-e
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StatusNfse {
    /// Normal
    Normal,
    /// Cancelada
    Cancelada,
    /// Substituída
    Substituida,
}

/// Prestador de serviços
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestadorServico {
    /// CNPJ do prestador
    pub cnpj: String,
    /// Inscrição Municipal
    pub inscricao_municipal: Option<String>,
    /// Razão Social
    pub razao_social: String,
    /// Nome Fantasia
    pub nome_fantasia: Option<String>,
    /// Endereço
    pub endereco: EnderecoNfse,
    /// Telefone
    pub telefone: Option<String>,
    /// Email
    pub email: Option<String>,
}

/// Tomador de serviços
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomadorServico {
    /// Tipo de documento (1=CPF, 2=CNPJ)
    pub tipo_documento: u8,
    /// CPF ou CNPJ do tomador
    pub documento: String,
    /// Inscrição Municipal (se contribuinte)
    pub inscricao_municipal: Option<String>,
    /// Inscrição Estadual (se contribuinte)
    pub inscricao_estadual: Option<String>,
    /// Razão Social / Nome
    pub razao_social: String,
    /// Endereço
    pub endereco: Option<EnderecoNfse>,
    /// Telefone
    pub telefone: Option<String>,
    /// Email
    pub email: Option<String>,
}

/// Intermediário do serviço
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermediarioServico {
    /// Tipo de documento (1=CPF, 2=CNPJ)
    pub tipo_documento: u8,
    /// CPF ou CNPJ
    pub documento: String,
    /// Razão Social / Nome
    pub razao_social: String,
    /// Inscrição Municipal
    pub inscricao_municipal: Option<String>,
}

/// Endereço para NFS-e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnderecoNfse {
    /// Logradouro
    pub logradouro: String,
    /// Número
    pub numero: String,
    /// Complemento
    pub complemento: Option<String>,
    /// Bairro
    pub bairro: String,
    /// Código do município (IBGE)
    pub codigo_municipio: String,
    /// Nome do município
    pub municipio: String,
    /// UF
    pub uf: String,
    /// CEP
    pub cep: String,
    /// Código do país (1058 = Brasil)
    pub codigo_pais: Option<String>,
}

/// Dados do serviço prestado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicoNfse {
    /// Código do serviço conforme LC 116/2003
    pub codigo_servico: String,
    /// Código CNAE
    pub codigo_cnae: Option<String>,
    /// Código do serviço no município
    pub codigo_servico_municipio: Option<String>,
    /// Código tributação no município
    pub codigo_tributacao_municipio: Option<String>,
    /// Discriminação do serviço
    pub discriminacao: String,
    /// Código do município onde o serviço foi prestado
    pub municipio_prestacao: String,
    /// Código do país (se exterior)
    pub codigo_pais: Option<String>,
    /// Código de obra (construção civil)
    pub codigo_obra: Option<String>,
    /// ART da obra (construção civil)
    pub art: Option<String>,
}

/// Valores da NFS-e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValoresNfse {
    /// Valor dos serviços
    pub valor_servicos: f32,
    /// Valor das deduções
    pub valor_deducoes: f32,
    /// Valor do PIS
    pub valor_pis: f32,
    /// Valor da COFINS
    pub valor_cofins: f32,
    /// Valor do INSS
    pub valor_inss: f32,
    /// Valor do IR
    pub valor_ir: f32,
    /// Valor da CSLL
    pub valor_csll: f32,
    /// Outras retenções
    pub outras_retencoes: f32,
    /// Valor do ISS
    pub valor_iss: f32,
    /// Alíquota do ISS (%)
    pub aliquota_iss: f32,
    /// Desconto incondicionado
    pub desconto_incondicionado: f32,
    /// Desconto condicionado
    pub desconto_condicionado: f32,
    /// Base de cálculo do ISS
    pub base_calculo: f32,
    /// Valor líquido
    pub valor_liquido: f32,
    /// ISS retido? (1=Sim, 2=Não)
    pub iss_retido: bool,
    /// Valor do ISS retido
    pub valor_iss_retido: f32,
    /// Responsável pela retenção (1=Prestador, 2=Tomador, 3=Intermediário)
    pub responsavel_retencao: Option<u8>,
}

impl Default for ValoresNfse {
    fn default() -> Self {
        Self {
            valor_servicos: 0.0,
            valor_deducoes: 0.0,
            valor_pis: 0.0,
            valor_cofins: 0.0,
            valor_inss: 0.0,
            valor_ir: 0.0,
            valor_csll: 0.0,
            outras_retencoes: 0.0,
            valor_iss: 0.0,
            aliquota_iss: 0.0,
            desconto_incondicionado: 0.0,
            desconto_condicionado: 0.0,
            base_calculo: 0.0,
            valor_liquido: 0.0,
            iss_retido: false,
            valor_iss_retido: 0.0,
            responsavel_retencao: None,
        }
    }
}

/// RPS - Recibo Provisório de Serviços
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rps {
    /// Identificação do RPS
    pub identificacao: IdentificacaoRps,
    /// Data de emissão
    pub data_emissao: String,
    /// Status (1=Normal, 2=Cancelado)
    pub status: u8,
    /// Substituição de RPS (se for substituição)
    pub rps_substituido: Option<IdentificacaoRps>,
}

/// Identificação do RPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentificacaoRps {
    /// Número do RPS
    pub numero: u64,
    /// Série do RPS
    pub serie: String,
    /// Tipo (1=RPS, 2=Nota Fiscal Conjugada, 3=Cupom)
    pub tipo: u8,
}

/// Lote de RPS para envio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoteRps {
    /// Número do lote
    pub numero_lote: String,
    /// CNPJ do prestador
    pub cnpj_prestador: String,
    /// Inscrição municipal do prestador
    pub inscricao_municipal_prestador: String,
    /// Quantidade de RPS no lote
    pub quantidade_rps: u32,
    /// Lista de RPS
    pub lista_rps: Vec<RpsCompleto>,
}

/// RPS completo para envio no lote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpsCompleto {
    /// Dados do RPS
    pub rps: Rps,
    /// Prestador
    pub prestador: PrestadorServico,
    /// Tomador
    pub tomador: Option<TomadorServico>,
    /// Serviço
    pub servico: ServicoNfse,
    /// Valores
    pub valores: ValoresNfse,
}

/// Resposta da consulta de NFS-e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespostaConsultaNfse {
    /// Sucesso na consulta
    pub sucesso: bool,
    /// Mensagem de retorno
    pub mensagem: Option<String>,
    /// Lista de NFS-e encontradas
    pub nfses: Vec<Nfse>,
}

/// Resposta do envio de lote de RPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespostaEnvioLote {
    /// Número do lote
    pub numero_lote: String,
    /// Protocolo de recebimento
    pub protocolo: Option<String>,
    /// Data de recebimento
    pub data_recebimento: Option<String>,
    /// Situação do lote (1=Não Recebido, 2=Não Processado, 3=Processado com Erro, 4=Processado com Sucesso)
    pub situacao: u8,
    /// Lista de erros/alertas
    pub mensagens: Vec<MensagemRetorno>,
}

/// Mensagem de retorno do processamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MensagemRetorno {
    /// Código da mensagem
    pub codigo: String,
    /// Descrição da mensagem
    pub mensagem: String,
    /// Correção sugerida
    pub correcao: Option<String>,
}

/// Calcula os valores de uma NFS-e
pub fn calcular_valores_nfse(
    valor_servicos: f32,
    aliquota_iss: f32,
    valor_deducoes: f32,
    iss_retido: bool,
    aliquota_pis: Option<f32>,
    aliquota_cofins: Option<f32>,
    aliquota_inss: Option<f32>,
    aliquota_ir: Option<f32>,
    aliquota_csll: Option<f32>,
) -> ValoresNfse {
    let base_calculo = valor_servicos - valor_deducoes;
    let valor_iss = base_calculo * (aliquota_iss / 100.0);

    let valor_pis = aliquota_pis.map(|a| base_calculo * (a / 100.0)).unwrap_or(0.0);
    let valor_cofins = aliquota_cofins.map(|a| base_calculo * (a / 100.0)).unwrap_or(0.0);
    let valor_inss = aliquota_inss.map(|a| base_calculo * (a / 100.0)).unwrap_or(0.0);
    let valor_ir = aliquota_ir.map(|a| base_calculo * (a / 100.0)).unwrap_or(0.0);
    let valor_csll = aliquota_csll.map(|a| base_calculo * (a / 100.0)).unwrap_or(0.0);

    let valor_iss_retido = if iss_retido { valor_iss } else { 0.0 };
    let outras_retencoes = valor_pis + valor_cofins + valor_inss + valor_ir + valor_csll;

    let valor_liquido = valor_servicos - valor_iss_retido - outras_retencoes;

    ValoresNfse {
        valor_servicos,
        valor_deducoes,
        valor_pis,
        valor_cofins,
        valor_inss,
        valor_ir,
        valor_csll,
        outras_retencoes,
        valor_iss,
        aliquota_iss,
        desconto_incondicionado: 0.0,
        desconto_condicionado: 0.0,
        base_calculo,
        valor_liquido,
        iss_retido,
        valor_iss_retido,
        responsavel_retencao: if iss_retido { Some(2) } else { None }, // 2 = Tomador
    }
}
