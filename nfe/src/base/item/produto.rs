use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize)]
pub struct Produto {
    pub codigo: String,                  // Código do produto
    pub gtin: Option<String>,            // Código de barras
    pub descricao: String,               // Descrição do produto
    pub ncm: String,                     // NCM
    pub fabricante_cnpj: Option<String>, // CNPJ Fabricante
    pub tributacao: String,              // Tributação
    pub unidade: String,                 // Unidade
    pub quantidade: f32,                 // Quantidade
    pub valor_unitario: f32,             // Valor unitário
    pub valor_bruto: f32,                // Valor bruto
    pub valor_frete: f32,                // Valor frete
    pub valor_seguro: f32,               // Valor seguro
    pub desconto: f32,                   // Desconto
    pub outros: f32,                     // Outras despesas
    pub valor_compoe_total_nota: bool,   // Valor compõe total da nota
}

// Dados referentes a tributação do produto
#[derive(Debug, PartialEq, Serialize)]
pub struct ProdutoTributacao {
    pub cest: Option<String>,                    // CEST
    pub escala_relevante: Option<String>,        // Escala relevante
    pub codigo_beneficio_fiscal: Option<String>, // Código de benefício fiscal
    pub codigo_excecao_ipi: Option<String>,      // Código de exceção do IPI
    pub cfop: String,                            // CFOP
    pub gtin: Option<String>,                    // Código de barras
    pub unidade: String,                         // Unidade
    pub quantidade: f32,                         // Quantidade
    pub valor_unitario: f32,                     // Valor unitário
}

// Dados referentes a tributação do produto
#[derive(Debug, PartialEq, Eq, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]

pub enum escala_relevante {
    Sim = 1,
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

        Ok(Self {
            codigo: prod.codigo,
            gtin: match prod.gtin.to_lowercase().trim() {
                "sem gtin" => None,
                "" => None,
                _ => Some(prod.gtin),
            },
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
                gtin: match prod.t_gtin.to_lowercase().trim() {
                    "sem gtin" => None,
                    "" => None,
                    _ => Some(prod.t_gtin),
                },
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
    #[serde(rename = "$unflatten=cProd")]
    pub codigo: String,
    #[serde(rename = "$unflatten=cEAN")]
    pub gtin: String,
    #[serde(rename = "$unflatten=xProd")]
    pub descricao: String,
    #[serde(rename = "$unflatten=NCM")]
    pub ncm: String,
    #[serde(rename = "$unflatten=CNPJFab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabricante_cnpj: Option<String>,
    #[serde(rename = "$unflatten=uCom")]
    pub unidade: String,
    #[serde(rename = "$unflatten=qCom")]
    pub quantidade: f32,
    #[serde(rename = "$unflatten=vUnCom")]
    pub valor_unitario: f32,
    #[serde(rename = "$unflatten=vProd")]
    pub valor_bruto: f32,
    #[serde(rename = "$unflatten=vFrete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_frete: Option<f32>,
    #[serde(rename = "$unflatten=vDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_seguro: Option<f32>,
    #[serde(rename = "$unflatten=vSeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_desconto: Option<f32>,
    #[serde(rename = "$unflatten=vOutro")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_outros: Option<f32>,
    #[serde(rename = "$unflatten=indTot")]
    pub valor_compoe_total_nota: u8,

    #[serde(rename = "$unflatten=CEST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_cest: Option<String>,
    #[serde(rename = "$unflatten=indEscala")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_escala_relevante: Option<EscalaRelevante>,
    #[serde(rename = "$unflatten=cBenef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_codigo_beneficio_fiscal: Option<String>,
    #[serde(rename = "$unflatten=EXTIPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_codigo_excecao_ipi: Option<String>,
    #[serde(rename = "$unflatten=CFOP")]
    pub t_cfop: String,
    #[serde(rename = "$unflatten=cEANTrib")]
    pub t_gtin: String,
    #[serde(rename = "$unflatten=uTrib")]
    pub t_unidade: String,
    #[serde(rename = "$unflatten=qTrib")]
    pub t_quantidade: f32,
    #[serde(rename = "$unflatten=vUnTrib")]
    pub t_valor_unitario: f32,
}

impl Produto {
    // generate a new instance of Produto
    pub fn new(
        codigo_produto: String,
        codigo_ean: String,
        descricao: String,
        ncm: String,
        cfop: String,
        unidade_comercial: String,
        quantidade_comercial: f32,
        valor_unitario_comercial: f64,
        valor_total: f64,
        codigo_ean_tributavel: String,
        unidade_tributavel: String,
        quantidade_tributavel: f32,
        valor_unitario_tributavel: f64,
    ) -> Self {
        Produto {
            codigo_produto,
            codigo_ean,
            descricao,
            ncm,
            cfop,
            unidade_comercial,
            quantidade_comercial,
            valor_unitario_comercial,
            valor_total,
            codigo_ean_tributavel,
            unidade_tributavel,
            quantidade_tributavel,
            valor_unitario_tributavel,
        }
    }
}
