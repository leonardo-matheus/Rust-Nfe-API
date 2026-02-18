//! ISSQN - Imposto sobre Serviços de Qualquer Natureza
//!
//! Este módulo implementa a estrutura para representar o ISS na NF-e.
//!
//! ## Quando Utilizar
//!
//! O grupo ISSQN deve ser informado quando a operação envolver prestação
//! de serviços sujeitos ao ISS, mesmo em notas que também contenham mercadorias.
//!
//! ## Nota Importante
//!
//! A presença do grupo ISSQN **substitui** o grupo ICMS para aquele item.
//! Não devem ser informados ambos os grupos para o mesmo item.
//!
//! ## Cálculo
//!
//! ```text
//! vISSQN = vBC × (alíquota ISS / 100)
//! ```

use serde::{Deserialize, Serialize};

/// ISSQN - Imposto sobre Serviços (tag `<ISSQN>`)
///
/// Informar apenas para itens que são serviços sujeitos ao ISS.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Issqn {
    /// Valor da Base de Cálculo do ISSQN (tag `<vBC>`)
    #[serde(rename = "$unflatten=vBC")]
    pub valor_bc: f32,

    /// Alíquota do ISSQN em percentual (tag `<vAliq>`)
    #[serde(rename = "$unflatten=vAliq")]
    pub aliquota: f32,

    /// Valor do ISSQN (tag `<vISSQN>`)
    #[serde(rename = "$unflatten=vISSQN")]
    pub valor: f32,

    /// Código do município de ocorrência do fato gerador (tag `<cMunFG>`)
    /// Código IBGE do município
    #[serde(rename = "$unflatten=cMunFG")]
    pub codigo_municipio_fato_gerador: String,

    /// Código da Lista de Serviços LC 116/2003 (tag `<cListServ>`)
    /// Formato: NN.NN (ex: "14.01")
    #[serde(rename = "$unflatten=cListServ")]
    pub codigo_lista_servico: String,

    /// Valor dedução para redução da Base de Cálculo (tag `<vDeducao>`)
    #[serde(rename = "$unflatten=vDeducao")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_deducao: Option<f32>,

    /// Valor outras retenções (tag `<vOutro>`)
    #[serde(rename = "$unflatten=vOutro")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_outras_retencoes: Option<f32>,

    /// Valor desconto incondicionado (tag `<vDescIncond>`)
    #[serde(rename = "$unflatten=vDescIncond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_desconto_incondicionado: Option<f32>,

    /// Valor desconto condicionado (tag `<vDescCond>`)
    #[serde(rename = "$unflatten=vDescCond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_desconto_condicionado: Option<f32>,

    /// Valor retenção ISS (tag `<vISSRet>`)
    #[serde(rename = "$unflatten=vISSRet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_retencao_iss: Option<f32>,

    /// Indicador da exigibilidade do ISS (tag `<indISS>`)
    /// 1=Exigível, 2=Não incidência, 3=Isenção, 4=Exportação,
    /// 5=Imunidade, 6=Exigibilidade suspensa por decisão judicial,
    /// 7=Exigibilidade suspensa por processo administrativo
    #[serde(rename = "$unflatten=indISS")]
    pub indicador_exigibilidade: u8,

    /// Código do serviço prestado dentro do município (tag `<cServico>`)
    #[serde(rename = "$unflatten=cServico")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codigo_servico_municipio: Option<String>,

    /// Código do município de incidência do imposto (tag `<cMun>`)
    #[serde(rename = "$unflatten=cMun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codigo_municipio_incidencia: Option<String>,

    /// Código do país onde o serviço foi prestado (tag `<cPais>`)
    #[serde(rename = "$unflatten=cPais")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codigo_pais: Option<String>,

    /// Número do processo de suspensão (tag `<nProcesso>`)
    /// Informar quando indISS = 6 ou 7
    #[serde(rename = "$unflatten=nProcesso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numero_processo: Option<String>,

    /// Indicador de incentivo fiscal (tag `<indIncentivo>`)
    /// 1=Sim, 2=Não
    #[serde(rename = "$unflatten=indIncentivo")]
    pub indicador_incentivo_fiscal: u8,
}

impl Default for Issqn {
    fn default() -> Self {
        Self {
            valor_bc: 0.0,
            aliquota: 0.0,
            valor: 0.0,
            codigo_municipio_fato_gerador: String::new(),
            codigo_lista_servico: String::new(),
            valor_deducao: None,
            valor_outras_retencoes: None,
            valor_desconto_incondicionado: None,
            valor_desconto_condicionado: None,
            valor_retencao_iss: None,
            indicador_exigibilidade: 1,
            codigo_servico_municipio: None,
            codigo_municipio_incidencia: None,
            codigo_pais: None,
            numero_processo: None,
            indicador_incentivo_fiscal: 2,
        }
    }
}
