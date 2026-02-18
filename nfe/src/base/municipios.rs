//! Tabela de Municípios IBGE
//!
//! Este módulo contém os códigos IBGE e configurações fiscais dos municípios brasileiros,
//! com foco especial nos municípios de São Paulo.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Informações de um município
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Municipio {
    /// Código IBGE do município (7 dígitos)
    pub codigo_ibge: String,
    /// Nome do município
    pub nome: String,
    /// Sigla da UF
    pub uf: String,
    /// Código da UF (2 dígitos)
    pub codigo_uf: u8,
    /// Alíquota padrão de ISS (%)
    pub aliquota_iss_padrao: f32,
    /// Alíquota mínima de ISS (%)
    pub aliquota_iss_minima: f32,
    /// Alíquota máxima de ISS (%)
    pub aliquota_iss_maxima: f32,
    /// Sistema de NFS-e utilizado
    pub sistema_nfse: Option<SistemaNfse>,
    /// URL do WebService de NFS-e (se disponível)
    pub url_nfse: Option<String>,
}

/// Sistemas de NFS-e utilizados pelas prefeituras
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SistemaNfse {
    /// ABRASF versão 1.0
    Abrasf1,
    /// ABRASF versão 2.0/2.04
    Abrasf2,
    /// Ginfes (Tecnos)
    Ginfes,
    /// IPM Sistemas
    Ipm,
    /// ISSNet
    IssNet,
    /// Betha Sistemas
    Betha,
    /// GISS Online
    GissOnline,
    /// Simpliss
    Simpliss,
    /// EL (E&L Produções de Software)
    El,
    /// Outro sistema proprietário
    Outro(String),
}

/// Configurações fiscais de uma UF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguracaoUf {
    /// Sigla da UF
    pub uf: String,
    /// Código da UF
    pub codigo: u8,
    /// Alíquota interna padrão de ICMS (%)
    pub aliquota_icms_interna: f32,
    /// Alíquota do FCP (Fundo de Combate à Pobreza) - se aplicável
    pub aliquota_fcp: Option<f32>,
    /// Alíquota interestadual para Sul/Sudeste (exceto ES)
    pub aliquota_interestadual_sul_sudeste: f32,
    /// Alíquota interestadual para demais estados
    pub aliquota_interestadual_demais: f32,
    /// Alíquota para produtos importados (Resolução 13/2012)
    pub aliquota_importados: f32,
}

/// Tabela de alíquotas de ISS por código de serviço (LC 116/2003)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliquotaIss {
    /// Código do serviço (ex: "7.02")
    pub codigo_servico: String,
    /// Descrição do serviço
    pub descricao: String,
    /// Alíquota de ISS (%)
    pub aliquota: f32,
    /// ISS retido na fonte?
    pub retencao: bool,
}

/// Retorna as configurações do município de Matão-SP
pub fn matao() -> Municipio {
    Municipio {
        codigo_ibge: "3529302".to_string(),
        nome: "Matão".to_string(),
        uf: "SP".to_string(),
        codigo_uf: 35,
        aliquota_iss_padrao: 5.0,
        aliquota_iss_minima: 2.0,
        aliquota_iss_maxima: 5.0,
        sistema_nfse: Some(SistemaNfse::GissOnline),
        url_nfse: Some("https://matao.gissdigital.com.br".to_string()),
    }
}

/// Retorna as configurações do município de Araraquara-SP
pub fn araraquara() -> Municipio {
    Municipio {
        codigo_ibge: "3503208".to_string(),
        nome: "Araraquara".to_string(),
        uf: "SP".to_string(),
        codigo_uf: 35,
        aliquota_iss_padrao: 5.0,
        aliquota_iss_minima: 2.0,
        aliquota_iss_maxima: 5.0,
        sistema_nfse: Some(SistemaNfse::GissOnline),
        url_nfse: Some("https://araraquara.gissdigital.com.br".to_string()),
    }
}

/// Retorna as configurações fiscais do estado de São Paulo
pub fn sao_paulo_uf() -> ConfiguracaoUf {
    ConfiguracaoUf {
        uf: "SP".to_string(),
        codigo: 35,
        aliquota_icms_interna: 18.0,
        aliquota_fcp: None, // SP não tem FCP
        aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 7.0,
        aliquota_importados: 4.0,
    }
}

/// Tabela de alíquotas de ISS para Matão-SP
pub fn aliquotas_iss_matao() -> Vec<AliquotaIss> {
    vec![
        // Construção Civil
        AliquotaIss {
            codigo_servico: "7.02".to_string(),
            descricao: "Execução de obras de construção civil".to_string(),
            aliquota: 3.0,
            retencao: true,
        },
        AliquotaIss {
            codigo_servico: "7.04".to_string(),
            descricao: "Demolição".to_string(),
            aliquota: 3.0,
            retencao: true,
        },
        AliquotaIss {
            codigo_servico: "7.05".to_string(),
            descricao: "Reparação, conservação e reforma de edifícios".to_string(),
            aliquota: 3.0,
            retencao: true,
        },
        AliquotaIss {
            codigo_servico: "7.19".to_string(),
            descricao: "Acompanhamento e fiscalização de obras".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        // Informática
        AliquotaIss {
            codigo_servico: "1.01".to_string(),
            descricao: "Análise e desenvolvimento de sistemas".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.02".to_string(),
            descricao: "Programação".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.03".to_string(),
            descricao: "Processamento de dados".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.04".to_string(),
            descricao: "Elaboração de programas de computadores".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.05".to_string(),
            descricao: "Licenciamento de software".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        // Saúde
        AliquotaIss {
            codigo_servico: "4.01".to_string(),
            descricao: "Medicina e biomedicina".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "4.02".to_string(),
            descricao: "Análises clínicas".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "4.03".to_string(),
            descricao: "Hospitais, clínicas e similares".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        // Educação
        AliquotaIss {
            codigo_servico: "8.01".to_string(),
            descricao: "Ensino regular pré-escolar, fundamental, médio e superior".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "8.02".to_string(),
            descricao: "Instrução, treinamento, cursos".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        // Transporte
        AliquotaIss {
            codigo_servico: "16.01".to_string(),
            descricao: "Transporte de natureza municipal".to_string(),
            aliquota: 5.0,
            retencao: false,
        },
        // Serviços gerais
        AliquotaIss {
            codigo_servico: "17.01".to_string(),
            descricao: "Assessoria ou consultoria".to_string(),
            aliquota: 5.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "17.02".to_string(),
            descricao: "Datilografia, digitação, estenografia".to_string(),
            aliquota: 5.0,
            retencao: false,
        },
        // Exploração de rodovia
        AliquotaIss {
            codigo_servico: "22.01".to_string(),
            descricao: "Exploração de rodovia mediante cobrança de pedágio".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
    ]
}

/// Tabela de alíquotas de ISS para Araraquara-SP
pub fn aliquotas_iss_araraquara() -> Vec<AliquotaIss> {
    vec![
        // Construção Civil - Alíquota reduzida (LC 793)
        AliquotaIss {
            codigo_servico: "7.02".to_string(),
            descricao: "Execução de obras de construção civil".to_string(),
            aliquota: 2.0, // Alíquota reduzida conforme LC 793
            retencao: true,
        },
        AliquotaIss {
            codigo_servico: "7.04".to_string(),
            descricao: "Demolição".to_string(),
            aliquota: 3.0,
            retencao: true,
        },
        AliquotaIss {
            codigo_servico: "7.05".to_string(),
            descricao: "Reparação, conservação e reforma de edifícios".to_string(),
            aliquota: 3.0,
            retencao: true,
        },
        AliquotaIss {
            codigo_servico: "7.19".to_string(),
            descricao: "Acompanhamento e fiscalização de obras".to_string(),
            aliquota: 2.0, // Alíquota reduzida conforme LC 793
            retencao: false,
        },
        // Informática
        AliquotaIss {
            codigo_servico: "1.01".to_string(),
            descricao: "Análise e desenvolvimento de sistemas".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.02".to_string(),
            descricao: "Programação".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.03".to_string(),
            descricao: "Processamento de dados".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.04".to_string(),
            descricao: "Elaboração de programas de computadores".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "1.05".to_string(),
            descricao: "Licenciamento de software".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        // Saúde
        AliquotaIss {
            codigo_servico: "4.01".to_string(),
            descricao: "Medicina e biomedicina".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "4.02".to_string(),
            descricao: "Análises clínicas".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "4.03".to_string(),
            descricao: "Hospitais, clínicas e similares".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        // Educação
        AliquotaIss {
            codigo_servico: "8.01".to_string(),
            descricao: "Ensino regular".to_string(),
            aliquota: 2.0,
            retencao: false,
        },
        AliquotaIss {
            codigo_servico: "8.02".to_string(),
            descricao: "Instrução, treinamento, cursos".to_string(),
            aliquota: 3.0,
            retencao: false,
        },
        // Transporte
        AliquotaIss {
            codigo_servico: "16.01".to_string(),
            descricao: "Transporte de natureza municipal".to_string(),
            aliquota: 5.0,
            retencao: false,
        },
        // Serviços gerais
        AliquotaIss {
            codigo_servico: "17.01".to_string(),
            descricao: "Assessoria ou consultoria".to_string(),
            aliquota: 5.0,
            retencao: false,
        },
    ]
}

/// Tabela de alíquotas internas de ICMS por UF (2024-2026)
pub fn aliquotas_icms_por_uf() -> HashMap<String, ConfiguracaoUf> {
    let mut mapa = HashMap::new();

    // Região Norte
    mapa.insert("AC".to_string(), ConfiguracaoUf {
        uf: "AC".to_string(), codigo: 12, aliquota_icms_interna: 19.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("AM".to_string(), ConfiguracaoUf {
        uf: "AM".to_string(), codigo: 13, aliquota_icms_interna: 20.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("AP".to_string(), ConfiguracaoUf {
        uf: "AP".to_string(), codigo: 16, aliquota_icms_interna: 18.0,
        aliquota_fcp: None, aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("PA".to_string(), ConfiguracaoUf {
        uf: "PA".to_string(), codigo: 15, aliquota_icms_interna: 19.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("RO".to_string(), ConfiguracaoUf {
        uf: "RO".to_string(), codigo: 11, aliquota_icms_interna: 19.5,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("RR".to_string(), ConfiguracaoUf {
        uf: "RR".to_string(), codigo: 14, aliquota_icms_interna: 20.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("TO".to_string(), ConfiguracaoUf {
        uf: "TO".to_string(), codigo: 17, aliquota_icms_interna: 20.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });

    // Região Nordeste
    mapa.insert("AL".to_string(), ConfiguracaoUf {
        uf: "AL".to_string(), codigo: 27, aliquota_icms_interna: 19.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("BA".to_string(), ConfiguracaoUf {
        uf: "BA".to_string(), codigo: 29, aliquota_icms_interna: 20.5,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("CE".to_string(), ConfiguracaoUf {
        uf: "CE".to_string(), codigo: 23, aliquota_icms_interna: 20.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("MA".to_string(), ConfiguracaoUf {
        uf: "MA".to_string(), codigo: 21, aliquota_icms_interna: 22.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("PB".to_string(), ConfiguracaoUf {
        uf: "PB".to_string(), codigo: 25, aliquota_icms_interna: 20.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("PE".to_string(), ConfiguracaoUf {
        uf: "PE".to_string(), codigo: 26, aliquota_icms_interna: 20.5,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("PI".to_string(), ConfiguracaoUf {
        uf: "PI".to_string(), codigo: 22, aliquota_icms_interna: 21.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("RN".to_string(), ConfiguracaoUf {
        uf: "RN".to_string(), codigo: 24, aliquota_icms_interna: 20.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("SE".to_string(), ConfiguracaoUf {
        uf: "SE".to_string(), codigo: 28, aliquota_icms_interna: 19.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });

    // Região Centro-Oeste
    mapa.insert("DF".to_string(), ConfiguracaoUf {
        uf: "DF".to_string(), codigo: 53, aliquota_icms_interna: 20.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("GO".to_string(), ConfiguracaoUf {
        uf: "GO".to_string(), codigo: 52, aliquota_icms_interna: 19.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("MS".to_string(), ConfiguracaoUf {
        uf: "MS".to_string(), codigo: 50, aliquota_icms_interna: 17.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("MT".to_string(), ConfiguracaoUf {
        uf: "MT".to_string(), codigo: 51, aliquota_icms_interna: 17.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });

    // Região Sudeste
    mapa.insert("ES".to_string(), ConfiguracaoUf {
        uf: "ES".to_string(), codigo: 32, aliquota_icms_interna: 17.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 12.0, aliquota_importados: 4.0,
    });
    mapa.insert("MG".to_string(), ConfiguracaoUf {
        uf: "MG".to_string(), codigo: 31, aliquota_icms_interna: 18.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 7.0, aliquota_importados: 4.0,
    });
    mapa.insert("RJ".to_string(), ConfiguracaoUf {
        uf: "RJ".to_string(), codigo: 33, aliquota_icms_interna: 22.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 7.0, aliquota_importados: 4.0,
    });
    mapa.insert("SP".to_string(), sao_paulo_uf());

    // Região Sul
    mapa.insert("PR".to_string(), ConfiguracaoUf {
        uf: "PR".to_string(), codigo: 41, aliquota_icms_interna: 19.5,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 7.0, aliquota_importados: 4.0,
    });
    mapa.insert("RS".to_string(), ConfiguracaoUf {
        uf: "RS".to_string(), codigo: 43, aliquota_icms_interna: 17.0,
        aliquota_fcp: Some(2.0), aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 7.0, aliquota_importados: 4.0,
    });
    mapa.insert("SC".to_string(), ConfiguracaoUf {
        uf: "SC".to_string(), codigo: 42, aliquota_icms_interna: 17.0,
        aliquota_fcp: None, aliquota_interestadual_sul_sudeste: 12.0,
        aliquota_interestadual_demais: 7.0, aliquota_importados: 4.0,
    });

    mapa
}

/// Busca um município pelo código IBGE
pub fn buscar_municipio_por_codigo(codigo: &str) -> Option<Municipio> {
    match codigo {
        "3529302" => Some(matao()),
        "3503208" => Some(araraquara()),
        // Adicione mais municípios conforme necessário
        _ => None,
    }
}

/// Busca configuração de UF pela sigla
pub fn buscar_uf(sigla: &str) -> Option<ConfiguracaoUf> {
    aliquotas_icms_por_uf().get(sigla).cloned()
}

/// Calcula a alíquota interestadual entre duas UFs
pub fn calcular_aliquota_interestadual(uf_origem: &str, uf_destino: &str) -> f32 {
    // Sul e Sudeste (exceto ES) para demais estados: 7%
    // Demais combinações: 12%
    // Importados: 4%

    let sul_sudeste = ["SP", "RJ", "MG", "PR", "SC", "RS"];
    let origem_sul_sudeste = sul_sudeste.contains(&uf_origem);
    let destino_sul_sudeste = sul_sudeste.contains(&uf_destino);

    if origem_sul_sudeste && !destino_sul_sudeste {
        7.0
    } else {
        12.0
    }
}

/// Principais municípios de SP com seus códigos IBGE
pub fn municipios_sp() -> Vec<(String, String)> {
    vec![
        ("3503208".to_string(), "Araraquara".to_string()),
        ("3529302".to_string(), "Matão".to_string()),
        ("3509502".to_string(), "Campinas".to_string()),
        ("3518800".to_string(), "Guarulhos".to_string()),
        ("3534401".to_string(), "Osasco".to_string()),
        ("3543402".to_string(), "Ribeirão Preto".to_string()),
        ("3547809".to_string(), "Santo André".to_string()),
        ("3548500".to_string(), "Santos".to_string()),
        ("3548708".to_string(), "São Bernardo do Campo".to_string()),
        ("3549805".to_string(), "São José dos Campos".to_string()),
        ("3550308".to_string(), "São Paulo".to_string()),
        ("3552205".to_string(), "Sorocaba".to_string()),
    ]
}
