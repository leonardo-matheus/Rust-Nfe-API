//! Produto da Nota Fiscal Eletrônica

use super::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

/// Produto da NFe
///
/// Contém os dados do produto ou serviço vendido na nota fiscal.
#[derive(Debug, PartialEq, Clone)]
pub struct Produto {
    /// Código do produto no cadastro do contribuinte
    pub codigo: String,
    /// Código GTIN (EAN) do produto
    pub gtin: Option<String>,
    /// Descrição do produto ou serviço
    pub descricao: String,
    /// Código NCM (Nomenclatura Comum do Mercosul)
    pub ncm: String,
    /// CNPJ do fabricante
    pub fabricante_cnpj: Option<String>,
    /// Dados da tributação do produto
    pub tributacao: ProdutoTributacao,
    /// Unidade comercial
    pub unidade: String,
    /// Quantidade comercial
    pub quantidade: f32,
    /// Valor unitário comercial
    pub valor_unitario: f32,
    /// Valor bruto do produto
    pub valor_bruto: f32,
    /// Valor do frete
    pub valor_frete: Option<f32>,
    /// Valor do seguro
    pub valor_seguro: Option<f32>,
    /// Valor do desconto
    pub valor_desconto: Option<f32>,
    /// Outras despesas acessórias
    pub valor_outros: Option<f32>,
    /// Indica se o valor do produto compõe o total da NF-e
    pub valor_compoe_total_nota: bool,
}

/// Dados referentes a tributação do produto
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ProdutoTributacao {
    /// CEST - Código Especificador da Substituição Tributária
    pub cest: Option<String>,
    /// Indicador de Escala Relevante
    pub escala_relevante: Option<EscalaRelevante>,
    /// Código de Benefício Fiscal
    pub codigo_beneficio_fiscal: Option<String>,
    /// Código de Exceção do IPI
    pub codigo_excecao_ipi: Option<String>,
    /// CFOP - Código Fiscal de Operações e Prestações
    pub cfop: String,
    /// Código GTIN (EAN) tributável
    pub gtin: Option<String>,
    /// Unidade tributável
    pub unidade: String,
    /// Quantidade tributável
    pub quantidade: f32,
    /// Valor unitário tributável
    pub valor_unitario: f32,
}

/// Indicador de Escala Relevante
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum EscalaRelevante {
    /// Produzido em escala relevante
    Sim = 1,
    /// Não produzido em escala relevante
    Nao = 2,
}

impl FromStr for Produto {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Produto {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o produto")
    }
}

impl<'de> Deserialize<'de> for Produto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: Implementar a deserialização com serde flatten
        let prod = ProdContainer::deserialize(deserializer)?;

        // Função auxiliar para verificar se o GTIN é válido
        // "SEM GTIN" ou vazio são considerados ausência de GTIN
        fn parse_gtin(gtin: String) -> Option<String> {
            let trimmed = gtin.trim();
            if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("sem gtin") {
                None
            } else {
                Some(gtin)
            }
        }

        Ok(Self {
            codigo: prod.codigo,
            gtin: parse_gtin(prod.gtin),
            descricao: prod.descricao,
            ncm: prod.ncm,
            fabricante_cnpj: prod.fabricante_cnpj,
            unidade: prod.unidade,
            quantidade: prod.quantidade,
            valor_unitario: prod.valor_unitario,
            valor_bruto: prod.valor_bruto,
            valor_frete: prod.valor_frete,
            valor_seguro: prod.valor_seguro,
            valor_desconto: prod.valor_desconto,
            valor_outros: prod.valor_outros,
            valor_compoe_total_nota: prod.valor_compoe_total_nota == 1,
            tributacao: ProdutoTributacao {
                cest: prod.t_cest,
                escala_relevante: prod.t_escala_relevante,
                codigo_beneficio_fiscal: prod.t_codigo_beneficio_fiscal,
                codigo_excecao_ipi: prod.t_codigo_excecao_ipi,
                cfop: prod.t_cfop,
                gtin: parse_gtin(prod.t_gtin),
                unidade: prod.t_unidade,
                quantidade: prod.t_quantidade,
                valor_unitario: prod.t_valor_unitario,
            },
        })
    }
}

impl Serialize for Produto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let prod = ProdContainer {
            codigo: self.codigo.clone(),
            gtin: match &self.gtin {
                Some(gt) => gt.clone(),
                None => "SEM GTIN".to_string(),
            },
            descricao: self.descricao.clone(),
            ncm: self.ncm.clone(),
            fabricante_cnpj: self.fabricante_cnpj.clone(),
            unidade: self.unidade.clone(),
            quantidade: self.quantidade.clone(),
            valor_unitario: self.valor_unitario.clone(),
            valor_bruto: self.valor_bruto,
            valor_frete: self.valor_frete,
            valor_seguro: self.valor_seguro,
            valor_desconto: self.valor_desconto,
            valor_outros: self.valor_outros,
            valor_compoe_total_nota: if self.valor_compoe_total_nota { 1 } else { 0 },
            t_cest: self.tributacao.cest.clone(),
            t_escala_relevante: self.tributacao.escala_relevante,
            t_codigo_beneficio_fiscal: self.tributacao.codigo_beneficio_fiscal.clone(),
            t_codigo_excecao_ipi: self.tributacao.codigo_excecao_ipi.clone(),
            t_cfop: self.tributacao.cfop.clone(),
            t_gtin: match &self.tributacao.gtin {
                Some(gt) => gt.clone(),
                None => "SEM GTIN".to_string(),
            },
            t_unidade: self.tributacao.unidade.clone(),
            t_quantidade: self.tributacao.quantidade,
            t_valor_unitario: self.tributacao.valor_unitario,
        };

        prod.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "prod")]
struct ProdContainer {
    #[serde(rename = "cProd")]
    pub codigo: String,
    #[serde(rename = "cEAN")]
    pub gtin: String,
    #[serde(rename = "xProd")]
    pub descricao: String,
    #[serde(rename = "NCM")]
    pub ncm: String,
    #[serde(rename = "CNPJFab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fabricante_cnpj: Option<String>,
    #[serde(rename = "uCom")]
    pub unidade: String,
    #[serde(rename = "qCom")]
    pub quantidade: f32,
    #[serde(rename = "vUnCom")]
    pub valor_unitario: f32,
    #[serde(rename = "vProd")]
    pub valor_bruto: f32,
    #[serde(rename = "vFrete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valor_frete: Option<f32>,
    #[serde(rename = "vSeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valor_seguro: Option<f32>,
    #[serde(rename = "vDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valor_desconto: Option<f32>,
    #[serde(rename = "vOutro")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub valor_outros: Option<f32>,
    #[serde(rename = "indTot")]
    pub valor_compoe_total_nota: u8,

    #[serde(rename = "CEST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub t_cest: Option<String>,
    #[serde(rename = "indEscala")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub t_escala_relevante: Option<EscalaRelevante>,
    #[serde(rename = "cBenef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub t_codigo_beneficio_fiscal: Option<String>,
    #[serde(rename = "EXTIPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub t_codigo_excecao_ipi: Option<String>,
    #[serde(rename = "CFOP")]
    pub t_cfop: String,
    #[serde(rename = "cEANTrib")]
    pub t_gtin: String,
    #[serde(rename = "uTrib")]
    pub t_unidade: String,
    #[serde(rename = "qTrib")]
    pub t_quantidade: f32,
    #[serde(rename = "vUnTrib")]
    pub t_valor_unitario: f32,
}

impl Produto {
    /// Cria uma nova instância de Produto
    ///
    /// # Argumentos
    ///
    /// * `codigo` - Código do produto
    /// * `descricao` - Descrição do produto
    /// * `ncm` - Código NCM
    /// * `cfop` - Código CFOP
    /// * `unidade` - Unidade comercial
    /// * `quantidade` - Quantidade comercial
    /// * `valor_unitario` - Valor unitário
    /// * `valor_bruto` - Valor bruto total
    pub fn new(
        codigo: String,
        descricao: String,
        ncm: String,
        cfop: String,
        unidade: String,
        quantidade: f32,
        valor_unitario: f32,
        valor_bruto: f32,
    ) -> Self {
        Produto {
            codigo,
            gtin: None,
            descricao,
            ncm,
            fabricante_cnpj: None,
            tributacao: ProdutoTributacao {
                cest: None,
                escala_relevante: None,
                codigo_beneficio_fiscal: None,
                codigo_excecao_ipi: None,
                cfop,
                gtin: None,
                unidade: unidade.clone(),
                quantidade,
                valor_unitario,
            },
            unidade,
            quantidade,
            valor_unitario,
            valor_bruto,
            valor_frete: None,
            valor_seguro: None,
            valor_desconto: None,
            valor_outros: None,
            valor_compoe_total_nota: true,
        }
    }
}
