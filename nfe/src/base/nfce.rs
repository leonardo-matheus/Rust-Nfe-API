//! NFC-e - Nota Fiscal de Consumidor Eletrônica (Modelo 65)
//!
//! Este módulo contém estruturas e funções específicas para NFC-e,
//! incluindo geração de QR Code e validações específicas.

use serde::{Deserialize, Serialize};
use sha1::{Sha1, Digest};

/// Dados para geração do QR Code da NFC-e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeNfce {
    /// Chave de acesso (44 dígitos)
    pub chave_acesso: String,
    /// Ambiente (1=Produção, 2=Homologação)
    pub ambiente: u8,
    /// Código do CSC (Código de Segurança do Contribuinte)
    pub csc: String,
    /// ID do CSC (Token)
    pub id_csc: String,
}

/// Configuração do CSC (Código de Segurança do Contribuinte)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguracaoCsc {
    /// ID do token CSC (sequencial fornecido pela SEFAZ)
    pub id_token: String,
    /// Código CSC (alfanumérico de 36 caracteres)
    pub codigo_csc: String,
}

impl QrCodeNfce {
    /// Gera a URL do QR Code para NFC-e
    ///
    /// ## Formato do QR Code (versão 2.0)
    ///
    /// ```text
    /// URL_Base?p=CHAVE|VERSAO|AMBIENTE|ID_CSC|HASH
    /// ```
    ///
    /// Onde HASH = SHA1(CHAVE|VERSAO|AMBIENTE|CSC)
    pub fn gerar_url(&self) -> String {
        let versao_qrcode = "2";

        // Monta a string para hash: chave|versao|ambiente|csc
        let dados_hash = format!(
            "{}|{}|{}|{}",
            self.chave_acesso,
            versao_qrcode,
            self.ambiente,
            self.csc
        );

        // Calcula SHA1
        let mut hasher = Sha1::new();
        hasher.update(dados_hash.as_bytes());
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash);

        // URL base por ambiente
        let url_base = if self.ambiente == 1 {
            "https://www.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaQRCode.aspx"
        } else {
            "https://www.homologacao.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaQRCode.aspx"
        };

        // Monta URL final
        format!(
            "{}?p={}|{}|{}|{}|{}",
            url_base,
            self.chave_acesso,
            versao_qrcode,
            self.ambiente,
            self.id_csc,
            hash_hex.to_uppercase()
        )
    }

    /// Gera o conteúdo do QR Code para NFC-e em contingência offline
    ///
    /// Em contingência, inclui informações adicionais do digest do XML
    pub fn gerar_url_contingencia(&self, digest_value: &str, data_emissao: &str, valor_total: f32) -> String {
        let versao_qrcode = "2";

        // Em contingência: chave|versao|ambiente|dia_emissao|valor|digest|csc
        let dia = &data_emissao[8..10];
        let valor_str = format!("{:.2}", valor_total);

        let dados_hash = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.chave_acesso,
            versao_qrcode,
            self.ambiente,
            dia,
            valor_str,
            digest_value,
            self.csc
        );

        let mut hasher = Sha1::new();
        hasher.update(dados_hash.as_bytes());
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash);

        let url_base = if self.ambiente == 1 {
            "https://www.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaQRCode.aspx"
        } else {
            "https://www.homologacao.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaQRCode.aspx"
        };

        format!(
            "{}?p={}|{}|{}|{}|{}|{}|{}|{}",
            url_base,
            self.chave_acesso,
            versao_qrcode,
            self.ambiente,
            dia,
            valor_str,
            digest_value,
            self.id_csc,
            hash_hex.to_uppercase()
        )
    }
}

/// Validações específicas para NFC-e
#[derive(Debug)]
pub struct ValidadorNfce;

impl ValidadorNfce {
    /// Valida se a NFC-e atende aos requisitos do modelo 65
    pub fn validar(
        modelo: u8,
        valor_total: f32,
        qtd_itens: usize,
        tem_destinatario: bool,
        cfop: &str,
    ) -> Result<(), Vec<String>> {
        let mut erros = Vec::new();

        // Deve ser modelo 65
        if modelo != 65 {
            erros.push("NFC-e deve usar modelo 65".to_string());
        }

        // Limite de 990 itens
        if qtd_itens > 990 {
            erros.push(format!("NFC-e permite no máximo 990 itens (encontrado: {})", qtd_itens));
        }

        // Destinatário obrigatório para valores acima de R$ 10.000
        if valor_total > 10000.0 && !tem_destinatario {
            erros.push("Destinatário é obrigatório para NFC-e com valor acima de R$ 10.000,00".to_string());
        }

        // CFOP deve ser de saída (5xxx ou 6xxx para venda)
        if !cfop.starts_with('5') && !cfop.starts_with('6') {
            erros.push(format!("CFOP {} não permitido para NFC-e (use CFOP de saída)", cfop));
        }

        // NFC-e não permite CFOP de entrada (1xxx, 2xxx, 3xxx)
        if cfop.starts_with('1') || cfop.starts_with('2') || cfop.starts_with('3') {
            erros.push("NFC-e não permite CFOP de entrada".to_string());
        }

        if erros.is_empty() {
            Ok(())
        } else {
            Err(erros)
        }
    }

    /// Valida se a chave de acesso é de uma NFC-e (modelo 65)
    pub fn validar_chave(chave: &str) -> bool {
        if chave.len() != 44 {
            return false;
        }

        // Posições 20-21 contêm o modelo
        if let Ok(modelo) = chave[20..22].parse::<u8>() {
            modelo == 65
        } else {
            false
        }
    }
}

/// URLs dos WebServices de NFC-e por UF
pub fn url_nfce_por_uf(uf: &str, ambiente: u8) -> Option<UrlsNfce> {
    let producao = ambiente == 1;

    match uf {
        "SP" => Some(UrlsNfce {
            autorizacao: if producao {
                "https://nfce.fazenda.sp.gov.br/ws/NFeAutorizacao4.asmx"
            } else {
                "https://homologacao.nfce.fazenda.sp.gov.br/ws/NFeAutorizacao4.asmx"
            }.to_string(),
            consulta: if producao {
                "https://nfce.fazenda.sp.gov.br/ws/NFeConsultaProtocolo4.asmx"
            } else {
                "https://homologacao.nfce.fazenda.sp.gov.br/ws/NFeConsultaProtocolo4.asmx"
            }.to_string(),
            qrcode: if producao {
                "https://www.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaQRCode.aspx"
            } else {
                "https://www.homologacao.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaQRCode.aspx"
            }.to_string(),
            consulta_publica: if producao {
                "https://www.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaPublica.aspx"
            } else {
                "https://www.homologacao.nfce.fazenda.sp.gov.br/NFCeConsultaPublica/Paginas/ConsultaPublica.aspx"
            }.to_string(),
        }),
        // Adicione mais UFs conforme necessário
        _ => None,
    }
}

/// URLs dos WebServices de NFC-e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlsNfce {
    pub autorizacao: String,
    pub consulta: String,
    pub qrcode: String,
    pub consulta_publica: String,
}

/// Modos de emissão da NFC-e
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ModoEmissaoNfce {
    /// Normal - transmissão online
    Normal = 1,
    /// Contingência offline
    ContingenciaOffline = 9,
}

/// Formas de pagamento aceitas em NFC-e
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FormaPagamentoNfce {
    Dinheiro = 1,
    Cheque = 2,
    CartaoCredito = 3,
    CartaoDebito = 4,
    CreditoLoja = 5,
    ValeAlimentacao = 10,
    ValeRefeicao = 11,
    ValePresente = 12,
    ValeCombustivel = 13,
    BoletoBancario = 15,
    DepositoBancario = 16,
    Pix = 17,
    TransferenciaBancaria = 18,
    CashbackDebito = 19,
    SemPagamento = 90,
    Outros = 99,
}

impl FormaPagamentoNfce {
    pub fn descricao(&self) -> &'static str {
        match self {
            Self::Dinheiro => "Dinheiro",
            Self::Cheque => "Cheque",
            Self::CartaoCredito => "Cartão de Crédito",
            Self::CartaoDebito => "Cartão de Débito",
            Self::CreditoLoja => "Crédito Loja",
            Self::ValeAlimentacao => "Vale Alimentação",
            Self::ValeRefeicao => "Vale Refeição",
            Self::ValePresente => "Vale Presente",
            Self::ValeCombustivel => "Vale Combustível",
            Self::BoletoBancario => "Boleto Bancário",
            Self::DepositoBancario => "Depósito Bancário",
            Self::Pix => "PIX",
            Self::TransferenciaBancaria => "Transferência Bancária",
            Self::CashbackDebito => "Cashback Débito",
            Self::SemPagamento => "Sem Pagamento",
            Self::Outros => "Outros",
        }
    }
}
