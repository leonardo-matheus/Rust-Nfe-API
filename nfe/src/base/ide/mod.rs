//! Identificação da NF-e - Grupo <ide>
//!
//! Este módulo contém as estruturas para o grupo de identificação da NF-e,
//! que é definido pela tag `<ide>` no XML e contém informações como:
//!
//! - Código da UF do emitente
//! - Código numérico que compõe a chave de acesso
//! - Natureza da operação
//! - Modelo do documento (55 = NF-e, 65 = NFC-e)
//! - Série e número da nota
//! - Data e hora de emissão
//! - Tipo de operação (entrada/saída)
//! - Destino da operação (interna, interestadual, exterior)
//! - Tipo de ambiente (produção/homologação)
//!
//! ## Referência SEFAZ
//!
//! Conforme Manual de Orientação do Contribuinte, o grupo `<ide>` é obrigatório
//! e deve conter no mínimo: cUF, cNF, natOp, mod, serie, nNF, dhEmi, tpNF,
//! idDest, cMunFG, tpImp, tpEmis, cDV, tpAmb, finNFe, indFinal, indPres, procEmi.

use super::Error;
use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

mod emissao;
mod operacao;

pub use emissao::*;
pub use operacao::*;

/// Identificação da NF-e (Grupo IDE - tag `<ide>`)
///
/// Contém os dados de identificação da nota fiscal eletrônica conforme
/// layout 4.00 da SEFAZ.
///
/// ## Campos e Tags XML
///
/// | Campo | Tag XML | Descrição | Obrig. |
/// |-------|---------|-----------|--------|
/// | codigo_uf | cUF | Código IBGE da UF do emitente | Sim |
/// | chave.codigo | cNF | Código numérico da chave (8 dígitos) | Sim |
/// | chave.digito_verificador | cDV | Dígito verificador da chave | Sim |
/// | numero | nNF | Número da NF-e (1 a 999999999) | Sim |
/// | serie | serie | Série da NF-e (0 a 999) | Sim |
/// | modelo | mod | Modelo: 55 (NF-e) ou 65 (NFC-e) | Sim |
/// | codigo_municipio | cMunFG | Código IBGE do município de ocorrência | Sim |
/// | formato_danfe | tpImp | Formato de impressão do DANFE | Sim |
/// | ambiente | tpAmb | 1=Produção, 2=Homologação | Sim |
#[derive(Debug, PartialEq, Clone)]
pub struct Identificacao {
    /// Código IBGE da UF do emitente (2 dígitos)
    /// Ex: 35 = SP, 33 = RJ, 43 = RS
    pub codigo_uf: u8,

    /// Componentes que formam parte da chave de acesso
    pub chave: ComposicaoChaveAcesso,

    /// Número da NF-e (1 a 999.999.999)
    pub numero: u32,

    /// Série da NF-e (0 a 999)
    /// Série 0-899: numeração de controle do contribuinte
    /// Série 900-999: uso exclusivo de regime especial
    pub serie: u16,

    /// Modelo do documento fiscal
    /// 55 = NF-e (Nota Fiscal Eletrônica)
    /// 65 = NFC-e (Nota Fiscal de Consumidor Eletrônica)
    pub modelo: ModeloDocumentoFiscal,

    /// Dados de emissão da nota (datas, tipo, finalidade)
    pub emissao: Emissao,

    /// Dados da operação (tipo, destino, natureza)
    pub operacao: Operacao,

    /// Código IBGE do município de ocorrência do fato gerador
    /// Geralmente é o município do emitente
    pub codigo_municipio: u32,

    /// Formato de impressão do DANFE
    pub formato_danfe: FormatoImpressaoDanfe,

    /// Tipo de ambiente: Produção ou Homologação
    /// Em homologação, a NF-e não tem validade fiscal
    pub ambiente: TipoAmbiente,
}

/// Modelo do documento fiscal eletrônico (tag `<mod>`)
///
/// Define se o documento é uma NF-e (modelo 55) para operações B2B
/// ou NFC-e (modelo 65) para vendas ao consumidor final.
///
/// ## Diferenças entre NF-e e NFC-e
///
/// | Característica | NF-e (55) | NFC-e (65) |
/// |----------------|-----------|------------|
/// | Uso principal | B2B, transferências | Varejo, consumidor final |
/// | Destinatário | Obrigatório | Opcional |
/// | Contingência | Vários modos | Offline |
/// | DANFE | A4 | Cupom/QR Code |
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ModeloDocumentoFiscal {
    /// Modelo 55 - Nota Fiscal Eletrônica (NF-e)
    /// Usada para operações B2B, transferências, devoluções
    Nfe = 55,

    /// Modelo 65 - Nota Fiscal de Consumidor Eletrônica (NFC-e)
    /// Usada para vendas presenciais ao consumidor final
    Nfce = 65,
}

/// Formato de impressão do DANFE (tag `<tpImp>`)
///
/// Define como o Documento Auxiliar da NF-e será impresso.
/// O formato deve ser compatível com o modelo do documento.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FormatoImpressaoDanfe {
    /// 0 = Sem geração de DANFE
    SemGeracao = 0,

    /// 1 = DANFE normal, retrato (formato A4 vertical)
    NormalRetrato = 1,

    /// 2 = DANFE normal, paisagem (formato A4 horizontal)
    NormalPaisagem = 2,

    /// 3 = DANFE simplificado
    Simplificado = 3,

    /// 4 = DANFE NFC-e (formato cupom)
    Nfce = 4,

    /// 5 = DANFE NFC-e em mensagem eletrônica
    /// Usado quando o consumidor opta por receber por e-mail/SMS
    NfceMensagemEletronica = 5,
}

/// Tipo do ambiente de transmissão (tag `<tpAmb>`)
///
/// Define se a NF-e está sendo emitida em ambiente de produção
/// (com validade fiscal) ou homologação (para testes).
///
/// ## Importante
///
/// - Em **homologação**, a NF-e NÃO tem validade fiscal
/// - O campo `<xNome>` do destinatário é substituído por:
///   "NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL"
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TipoAmbiente {
    /// 1 = Produção - NF-e com validade fiscal real
    Producao = 1,

    /// 2 = Homologação - Ambiente de testes, sem valor fiscal
    Homologacao = 2,
}

/// Componentes que formam a chave de acesso da NF-e
///
/// A chave de acesso possui 44 dígitos e é composta por:
/// - UF (2) + AAMM (4) + CNPJ (14) + MOD (2) + SERIE (3) + NNF (9) + CODIGO (9) + DV (1)
///
/// Esta estrutura armazena os campos que são informados separadamente no XML:
/// - `cNF`: Código numérico aleatório (8 dígitos que compõem os 9 do código)
/// - `cDV`: Dígito verificador calculado pelo módulo 11
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComposicaoChaveAcesso {
    /// Código numérico aleatório que compõe a chave de acesso (tag `<cNF>`)
    /// São 8 dígitos gerados pelo sistema emissor
    pub codigo: String,

    /// Dígito verificador da chave de acesso (tag `<cDV>`)
    /// Calculado usando módulo 11 sobre os 43 primeiros dígitos
    pub digito_verificador: u8,
}

impl FromStr for Identificacao {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Identificacao {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar a identificação")
    }
}

impl<'de> Deserialize<'de> for Identificacao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: voltar a tentar usar o serde flatten

        let ide = IdeContainer::deserialize(deserializer)?;

        Ok(Self {
            codigo_uf: ide.codigo_uf,
            numero: ide.numero,
            serie: ide.serie,
            modelo: ide.modelo,
            codigo_municipio: ide.codigo_municipio,
            formato_danfe: ide.formato_danfe,
            ambiente: ide.ambiente,
            chave: ComposicaoChaveAcesso {
                codigo: ide.c_codigo.clone(),
                digito_verificador: ide.c_digito_verificador,
            },
            operacao: Operacao {
                horario: ide.o_horario,
                tipo: ide.o_tipo,
                destino: ide.o_destino,
                natureza: ide.o_natureza,
                consumidor: ide.o_consumidor,
                presenca: ide.o_presenca,
                intermediador: ide.o_intermediador,
            },
            emissao: Emissao {
                horario: ide.e_horario,
                tipo: ide.e_tipo,
                finalidade: ide.e_finalidade,
                processo: ide.e_processo,
                versao_processo: ide.e_versao_processo,
            },
        })
    }
}

impl Serialize for Identificacao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ide = IdeContainer {
            codigo_uf: self.codigo_uf,
            numero: self.numero,
            serie: self.serie,
            modelo: self.modelo,
            codigo_municipio: self.codigo_municipio,
            formato_danfe: self.formato_danfe,
            ambiente: self.ambiente,
            c_codigo: self.chave.codigo.clone(),
            c_digito_verificador: self.chave.digito_verificador,
            o_horario: self.operacao.horario,
            o_tipo: self.operacao.tipo,
            o_destino: self.operacao.destino,
            o_natureza: self.operacao.natureza.clone(),
            o_consumidor: self.operacao.consumidor,
            o_presenca: self.operacao.presenca,
            o_intermediador: self.operacao.intermediador,
            e_horario: self.emissao.horario,
            e_tipo: self.emissao.tipo,
            e_finalidade: self.emissao.finalidade,
            e_processo: self.emissao.processo,
            e_versao_processo: self.emissao.versao_processo.clone(),
        };

        ide.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "ide")]
struct IdeContainer {
    #[serde(rename = "$unflatten=cUF")]
    pub codigo_uf: u8,
    #[serde(rename = "$unflatten=nNF")]
    pub numero: u32,
    #[serde(rename = "$unflatten=serie")]
    pub serie: u16,
    #[serde(rename = "$unflatten=mod")]
    pub modelo: ModeloDocumentoFiscal,
    #[serde(rename = "$unflatten=cMunFG")]
    pub codigo_municipio: u32,
    #[serde(rename = "$unflatten=tpImp")]
    pub formato_danfe: FormatoImpressaoDanfe,
    #[serde(rename = "$unflatten=tpAmb")]
    pub ambiente: TipoAmbiente,

    #[serde(rename = "$unflatten=cNF")]
    pub c_codigo: String,
    #[serde(rename = "$unflatten=cDV")]
    pub c_digito_verificador: u8,

    #[serde(rename = "$unflatten=dhEmi")]
    #[serde(serialize_with = "serialize_horario")]
    pub e_horario: DateTime<Utc>,
    #[serde(rename = "$unflatten=tpEmis")]
    pub e_tipo: TipoEmissao,
    #[serde(rename = "$unflatten=finNFe")]
    pub e_finalidade: FinalidadeEmissao,
    #[serde(rename = "$unflatten=procEmi")]
    pub e_processo: TipoProcessoEmissao,
    #[serde(rename = "$unflatten=verProc")]
    pub e_versao_processo: String,

    #[serde(rename = "$unflatten=dhSaiEnt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_horario_op")]
    #[serde(default)]
    pub o_horario: Option<DateTime<Utc>>,
    #[serde(rename = "$unflatten=tpNF")]
    pub o_tipo: TipoOperacao,
    #[serde(rename = "$unflatten=idDest")]
    pub o_destino: DestinoOperacao,
    #[serde(rename = "$unflatten=natOp")]
    pub o_natureza: String,
    #[serde(rename = "$unflatten=indFinal")]
    pub o_consumidor: TipoConsumidor,
    #[serde(rename = "$unflatten=indPres")]
    pub o_presenca: TipoPresencaComprador,
    #[serde(rename = "$unflatten=indIntermed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub o_intermediador: Option<TipoIntermediador>,
}

fn serialize_horario<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_rfc3339())
}

fn serialize_horario_op<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_horario(&date.unwrap(), serializer)
}
