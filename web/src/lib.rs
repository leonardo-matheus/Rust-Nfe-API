//! # NFe Web API
//!
//! API REST e GraphQL de alto desempenho para Nota Fiscal Eletronica brasileira.
//!
//! ## Funcionalidades
//!
//! - **REST API**: Endpoints para parse, geracao e consulta de NF-e
//! - **GraphQL API**: Schema completo com queries e mutations
//! - **DANFE PDF**: Geracao de DANFE profissional
//! - **Certificado A1**: Suporte a certificado digital .pfx/.p12
//! - **SEFAZ WebService**: Cliente SOAP para integracao
//! - **Leitura de PDF**: Extracao de dados de DANFE/NFS-e
//!
//! ## Performance
//!
//! Benchmarks em modo release:
//!
//! | Operacao | Tempo |
//! |----------|-------|
//! | Health Check | 2ms |
//! | Parse XML | 3ms |
//! | DANFE PDF | 3ms |
//! | GraphQL | 5ms |
//!
//! ## Exemplo de Uso
//!
//! ```rust,no_run
//! use nfe_web::{certificado, graphql, pdf, sefaz};
//!
//! // Validar chave de acesso
//! let info = sefaz::validar_chave_acesso("35240508665074000100550010000000011270815480");
//!
//! // Gerar URL de consulta
//! let url = sefaz::gerar_url_consulta_portal("35240508665074000100550010000000011270815480");
//! ```
//!
//! ## Modulos
//!
//! - [`certificado`]: Manipulacao de certificados digitais A1
//! - [`graphql`]: Schema e resolvers GraphQL
//! - [`pdf`]: Leitura de PDF e geracao de DANFE
//! - [`sefaz`]: Consulta e integracao com SEFAZ
//! - [`db`]: Modelos e conexao com banco de dados

#![doc(html_root_url = "https://docs.rs/nfe-web/0.2.1")]

/// Modulo de certificado digital A1
///
/// Suporte a arquivos .pfx/.p12 para autenticacao com SEFAZ.
///
/// # Exemplo
///
/// ```rust,no_run
/// use nfe_web::certificado::CertificadoA1;
///
/// let cert = CertificadoA1::from_file("certificado.pfx", "senha123")?;
/// println!("CNPJ: {:?}", cert.info.cnpj);
/// # Ok::<(), String>(())
/// ```
pub mod certificado;

/// Modulo de banco de dados
///
/// Suporte a PostgreSQL e MySQL para persistencia de NF-e.
pub mod db;

/// API GraphQL
///
/// Schema completo com queries e mutations para NF-e.
///
/// # Queries
///
/// - `nfe(chaveAcesso)` - Consulta NF-e por chave
/// - `validarChave(chave)` - Valida chave de acesso
/// - `consultarSefaz(chaveAcesso)` - Consulta status no SEFAZ
///
/// # Mutations
///
/// - `emitirNfe(input)` - Emite NF-e no SEFAZ
/// - `cancelarNfe(input)` - Cancela NF-e
/// - `parseXml(xml)` - Parseia XML de NF-e
pub mod graphql;

/// Modulo de PDF
///
/// Leitura de DANFE/NFS-e e geracao de DANFE profissional.
///
/// # Exemplo
///
/// ```rust,no_run
/// use nfe_web::pdf::{extract_danfe_data, gerar_danfe, DanfeInput};
///
/// // Ler dados de um PDF
/// let bytes = std::fs::read("danfe.pdf")?;
/// let dados = extract_danfe_data(&bytes)?;
///
/// // Gerar DANFE
/// let input = DanfeInput { /* ... */ };
/// let pdf_bytes = gerar_danfe(&input)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub mod pdf;

/// Integracao com SEFAZ
///
/// Cliente SOAP para WebServices da SEFAZ e funcoes de validacao.
///
/// # Funcoes
///
/// - [`validar_chave_acesso`](sefaz::validar_chave_acesso) - Valida e extrai dados da chave
/// - [`gerar_url_consulta_portal`](sefaz::gerar_url_consulta_portal) - Gera URL do portal
/// - [`consultar_portal_publico`](sefaz::consultar_portal_publico) - Consulta via portal
///
/// # Exemplo
///
/// ```rust
/// use nfe_web::sefaz;
///
/// let chave = "35240508665074000100550010000000011270815480";
/// if let Ok(info) = sefaz::validar_chave_acesso(chave) {
///     println!("UF: {}", info.uf);
///     println!("CNPJ: {}", info.cnpj);
///     println!("Numero: {}", info.numero);
/// }
/// ```
pub mod sefaz;

// Re-exports para conveniencia
pub use certificado::{CertificadoA1, CertificadoInfo, AssinadorXml};
pub use graphql::{create_schema, NfeSchema};
pub use pdf::{extract_danfe_data, gerar_danfe, DanfeData, DanfeInput};
pub use sefaz::{validar_chave_acesso, gerar_url_consulta_portal, ChaveAcessoInfo, ResultadoConsulta};
