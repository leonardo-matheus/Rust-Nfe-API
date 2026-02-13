//! Builder para criação de NF-e
//!
//! Este módulo fornece uma API fluente para construir uma NF-e do zero.

use crate::base::dest::{Destinatario, IndicadorContribuicaoIe};
use crate::base::emit::Emitente;
use crate::base::endereco::Endereco;
use crate::base::ide::*;
use crate::base::item::{Item, Produto, Imposto};
use crate::base::totais::Totalizacao;
use crate::base::transporte::{Transporte, ModalidadeFrete};
use crate::base::{Nfe, VersaoLayout};
use chrono::{DateTime, Utc};

/// Builder para construção de NF-e
#[derive(Debug, Default)]
pub struct NfeBuilder {
    // Identificação
    codigo_uf: Option<u8>,
    numero: Option<u32>,
    serie: Option<u16>,
    modelo: Option<ModeloDocumentoFiscal>,
    natureza_operacao: Option<String>,
    tipo_operacao: Option<TipoOperacao>,
    destino_operacao: Option<DestinoOperacao>,
    finalidade: Option<FinalidadeEmissao>,
    ambiente: Option<TipoAmbiente>,
    codigo_municipio: Option<u32>,

    // Emitente
    emit_cnpj: Option<String>,
    emit_razao_social: Option<String>,
    emit_nome_fantasia: Option<String>,
    emit_ie: Option<String>,
    emit_endereco: Option<Endereco>,

    // Destinatário
    dest_cnpj: Option<String>,
    dest_razao_social: Option<String>,
    dest_indicador_ie: Option<IndicadorContribuicaoIe>,
    dest_endereco: Option<Endereco>,

    // Itens
    itens: Vec<ItemBuilder>,

    // Transporte
    modalidade_frete: Option<ModalidadeFrete>,

    // Informações adicionais
    informacao_complementar: Option<String>,
}

/// Builder para itens da NF-e
#[derive(Debug, Clone)]
pub struct ItemBuilder {
    pub codigo: String,
    pub descricao: String,
    pub ncm: String,
    pub cfop: String,
    pub unidade: String,
    pub quantidade: f32,
    pub valor_unitario: f32,
    pub gtin: Option<String>,
    pub valor_desconto: Option<f32>,
}

impl NfeBuilder {
    /// Cria um novo builder
    pub fn new() -> Self {
        Self::default()
    }

    // === Identificação ===

    /// Define o código da UF (ex: 35 para SP)
    pub fn codigo_uf(mut self, uf: u8) -> Self {
        self.codigo_uf = Some(uf);
        self
    }

    /// Define o número da NF-e
    pub fn numero(mut self, numero: u32) -> Self {
        self.numero = Some(numero);
        self
    }

    /// Define a série da NF-e
    pub fn serie(mut self, serie: u16) -> Self {
        self.serie = Some(serie);
        self
    }

    /// Define o modelo (55 = NF-e, 65 = NFC-e)
    pub fn modelo(mut self, modelo: ModeloDocumentoFiscal) -> Self {
        self.modelo = Some(modelo);
        self
    }

    /// Define a natureza da operação
    pub fn natureza_operacao(mut self, natureza: &str) -> Self {
        self.natureza_operacao = Some(natureza.to_string());
        self
    }

    /// Define o tipo de operação (Entrada/Saída)
    pub fn tipo_operacao(mut self, tipo: TipoOperacao) -> Self {
        self.tipo_operacao = Some(tipo);
        self
    }

    /// Define o destino da operação
    pub fn destino_operacao(mut self, destino: DestinoOperacao) -> Self {
        self.destino_operacao = Some(destino);
        self
    }

    /// Define a finalidade da emissão
    pub fn finalidade(mut self, finalidade: FinalidadeEmissao) -> Self {
        self.finalidade = Some(finalidade);
        self
    }

    /// Define o ambiente (Produção/Homologação)
    pub fn ambiente(mut self, ambiente: TipoAmbiente) -> Self {
        self.ambiente = Some(ambiente);
        self
    }

    /// Define o código do município
    pub fn codigo_municipio(mut self, codigo: u32) -> Self {
        self.codigo_municipio = Some(codigo);
        self
    }

    // === Emitente ===

    /// Define o CNPJ do emitente
    pub fn emit_cnpj(mut self, cnpj: &str) -> Self {
        self.emit_cnpj = Some(cnpj.replace(&['.', '/', '-'][..], ""));
        self
    }

    /// Define a razão social do emitente
    pub fn emit_razao_social(mut self, razao: &str) -> Self {
        self.emit_razao_social = Some(razao.to_string());
        self
    }

    /// Define o nome fantasia do emitente
    pub fn emit_nome_fantasia(mut self, fantasia: &str) -> Self {
        self.emit_nome_fantasia = Some(fantasia.to_string());
        self
    }

    /// Define a IE do emitente
    pub fn emit_ie(mut self, ie: &str) -> Self {
        self.emit_ie = Some(ie.to_string());
        self
    }

    /// Define o endereço do emitente
    pub fn emit_endereco(mut self, endereco: Endereco) -> Self {
        self.emit_endereco = Some(endereco);
        self
    }

    // === Destinatário ===

    /// Define o CNPJ do destinatário
    pub fn dest_cnpj(mut self, cnpj: &str) -> Self {
        self.dest_cnpj = Some(cnpj.replace(&['.', '/', '-'][..], ""));
        self
    }

    /// Define a razão social do destinatário
    pub fn dest_razao_social(mut self, razao: &str) -> Self {
        self.dest_razao_social = Some(razao.to_string());
        self
    }

    /// Define o indicador de IE do destinatário
    pub fn dest_indicador_ie(mut self, indicador: IndicadorContribuicaoIe) -> Self {
        self.dest_indicador_ie = Some(indicador);
        self
    }

    /// Define o endereço do destinatário
    pub fn dest_endereco(mut self, endereco: Endereco) -> Self {
        self.dest_endereco = Some(endereco);
        self
    }

    // === Itens ===

    /// Adiciona um item à NF-e
    pub fn add_item(mut self, item: ItemBuilder) -> Self {
        self.itens.push(item);
        self
    }

    // === Transporte ===

    /// Define a modalidade do frete
    pub fn modalidade_frete(mut self, modalidade: ModalidadeFrete) -> Self {
        self.modalidade_frete = Some(modalidade);
        self
    }

    // === Informações Adicionais ===

    /// Define informações complementares
    pub fn informacao_complementar(mut self, info: &str) -> Self {
        self.informacao_complementar = Some(info.to_string());
        self
    }

    /// Constrói a NF-e
    pub fn build(self) -> Result<Nfe, String> {
        // Validações básicas
        let codigo_uf = self.codigo_uf.ok_or("Código UF é obrigatório")?;
        let numero = self.numero.ok_or("Número é obrigatório")?;
        let serie = self.serie.unwrap_or(1);
        let modelo = self.modelo.unwrap_or(ModeloDocumentoFiscal::Nfe);
        let natureza = self.natureza_operacao.ok_or("Natureza da operação é obrigatória")?;
        let tipo_op = self.tipo_operacao.unwrap_or(TipoOperacao::Saida);
        let destino = self.destino_operacao.unwrap_or(DestinoOperacao::Interna);
        let finalidade = self.finalidade.unwrap_or(FinalidadeEmissao::Normal);
        let ambiente = self.ambiente.unwrap_or(TipoAmbiente::Homologacao);
        let codigo_mun = self.codigo_municipio.ok_or("Código do município é obrigatório")?;

        if self.itens.is_empty() {
            return Err("Pelo menos um item é obrigatório".to_string());
        }

        // Gerar código numérico aleatório (8 dígitos)
        let codigo_numerico = format!("{:08}", rand_u32() % 100000000);

        // Data/hora atual
        let agora: DateTime<Utc> = Utc::now();

        // Construir itens
        let mut itens_nfe = Vec::new();
        let mut total_produtos = 0.0f32;
        let mut total_desconto = 0.0f32;

        for (idx, item) in self.itens.iter().enumerate() {
            let valor_bruto = item.quantidade * item.valor_unitario;
            total_produtos += valor_bruto;
            if let Some(desc) = item.valor_desconto {
                total_desconto += desc;
            }

            let produto = Produto::new(
                item.codigo.clone(),
                item.descricao.clone(),
                item.ncm.clone(),
                item.cfop.clone(),
                item.unidade.clone(),
                item.quantidade,
                item.valor_unitario,
                valor_bruto,
            );

            // Criar imposto básico (ICMS 00, PIS e COFINS)
            let imposto = Imposto::default();

            itens_nfe.push(Item {
                numero: (idx + 1) as u8,
                produto,
                imposto,
            });
        }

        // Calcular totais
        let valor_total = total_produtos - total_desconto;

        // Gerar chave de acesso (44 dígitos)
        let aamm = agora.format("%y%m").to_string();
        let cnpj_emit = self.emit_cnpj.clone().unwrap_or_default();
        let chave_sem_dv = format!(
            "{:02}{}{:014}{:02}{:03}{:09}{:01}{:08}",
            codigo_uf,
            aamm,
            cnpj_emit,
            modelo as u8,
            serie,
            numero,
            1, // tpEmis
            codigo_numerico
        );
        let dv = calcular_dv(&chave_sem_dv);
        let chave_acesso = format!("{}{}", chave_sem_dv, dv);

        // Construir endereço do emitente
        let emit_endereco = self.emit_endereco.unwrap_or_else(|| Endereco::default());

        // Construir NF-e
        Ok(Nfe {
            versao: VersaoLayout::V4_00,
            chave_acesso,
            ide: Identificacao {
                codigo_uf,
                chave: ComposicaoChaveAcesso {
                    codigo: codigo_numerico,
                    digito_verificador: dv,
                },
                numero,
                serie,
                modelo,
                emissao: Emissao {
                    horario: agora,
                    tipo: TipoEmissao::Normal,
                    finalidade,
                    processo: TipoProcessoEmissao::ViaAplicativoDoContribuinte,
                    versao_processo: "1.0.0".to_string(),
                },
                operacao: Operacao {
                    horario: None,
                    tipo: tipo_op,
                    destino,
                    natureza,
                    consumidor: TipoConsumidor::Normal,
                    presenca: TipoPresencaComprador::Presencial,
                    intermediador: None,
                },
                codigo_municipio: codigo_mun,
                formato_danfe: FormatoImpressaoDanfe::NormalRetrato,
                ambiente,
            },
            emit: Emitente {
                cnpj: self.emit_cnpj,
                razao_social: self.emit_razao_social,
                nome_fantasia: self.emit_nome_fantasia,
                ie: self.emit_ie,
                iest: None,
                endereco: emit_endereco,
            },
            dest: if self.dest_cnpj.is_some() {
                Some(Destinatario {
                    cnpj: self.dest_cnpj.unwrap_or_default(),
                    razao_social: self.dest_razao_social,
                    indicador_ie: self.dest_indicador_ie.unwrap_or(IndicadorContribuicaoIe::NaoContribuinteIe),
                    ie: None,
                    endereco: self.dest_endereco,
                })
            } else {
                None
            },
            itens: itens_nfe,
            totais: Totalizacao {
                valor_base_calculo: 0.0,
                valor_icms: 0.0,
                valor_produtos: total_produtos,
                valor_frete: 0.0,
                valor_seguro: 0.0,
                valor_desconto: total_desconto,
                valor_outros: 0.0,
                valor_pis: 0.0,
                valor_cofins: 0.0,
                valor_total,
                valor_aproximado_tributos: 0.0,
            },
            transporte: Transporte {
                modalidade: self.modalidade_frete.unwrap_or(ModalidadeFrete::SemTransporte),
            },
            informacao_complementar: self.informacao_complementar,
        })
    }
}

impl ItemBuilder {
    /// Cria um novo item
    pub fn new(codigo: &str, descricao: &str, ncm: &str, cfop: &str) -> Self {
        Self {
            codigo: codigo.to_string(),
            descricao: descricao.to_string(),
            ncm: ncm.to_string(),
            cfop: cfop.to_string(),
            unidade: "UN".to_string(),
            quantidade: 1.0,
            valor_unitario: 0.0,
            gtin: None,
            valor_desconto: None,
        }
    }

    /// Define a unidade
    pub fn unidade(mut self, unidade: &str) -> Self {
        self.unidade = unidade.to_string();
        self
    }

    /// Define a quantidade
    pub fn quantidade(mut self, qtd: f32) -> Self {
        self.quantidade = qtd;
        self
    }

    /// Define o valor unitário
    pub fn valor_unitario(mut self, valor: f32) -> Self {
        self.valor_unitario = valor;
        self
    }

    /// Define o GTIN/EAN
    pub fn gtin(mut self, gtin: &str) -> Self {
        self.gtin = Some(gtin.to_string());
        self
    }

    /// Define o valor do desconto
    pub fn desconto(mut self, valor: f32) -> Self {
        self.valor_desconto = Some(valor);
        self
    }
}

/// Calcula o dígito verificador da chave de acesso (módulo 11)
fn calcular_dv(chave: &str) -> u8 {
    let pesos = [2, 3, 4, 5, 6, 7, 8, 9];
    let mut soma = 0u32;

    for (i, c) in chave.chars().rev().enumerate() {
        if let Some(digito) = c.to_digit(10) {
            soma += digito * pesos[i % 8];
        }
    }

    let resto = soma % 11;
    if resto < 2 { 0 } else { (11 - resto) as u8 }
}

/// Gera um número pseudo-aleatório simples
fn rand_u32() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (duration.as_nanos() % u32::MAX as u128) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basico() {
        let nfe = NfeBuilder::new()
            .codigo_uf(35)
            .numero(1)
            .serie(1)
            .natureza_operacao("VENDA DE MERCADORIA")
            .codigo_municipio(3550308)
            .emit_cnpj("12.345.678/0001-90")
            .emit_razao_social("EMPRESA TESTE LTDA")
            .emit_ie("123456789")
            .add_item(
                ItemBuilder::new("PROD001", "Produto Teste", "12345678", "5102")
                    .quantidade(10.0)
                    .valor_unitario(100.0)
            )
            .build();

        assert!(nfe.is_ok());
        let nfe = nfe.unwrap();
        assert_eq!(nfe.ide.numero, 1);
        assert_eq!(nfe.itens.len(), 1);
        assert_eq!(nfe.totais.valor_produtos, 1000.0);
    }

    #[test]
    fn test_calculo_dv() {
        // Exemplo de chave sem DV
        let chave = "35150312345678901234550010000000011000000011";
        let dv = calcular_dv(&chave[..43]);
        assert!(dv < 10);
    }
}
