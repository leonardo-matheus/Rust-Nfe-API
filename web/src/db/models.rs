//! Modelos para banco de dados

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// NF-e armazenada no banco de dados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfeRecord {
    pub id: String,
    pub chave_acesso: String,
    pub numero: i32,
    pub serie: i16,
    pub data_emissao: DateTime<Utc>,
    pub emit_cnpj: String,
    pub emit_razao_social: String,
    pub dest_cnpj: Option<String>,
    pub dest_razao_social: Option<String>,
    pub valor_total: f64,
    pub xml: String,
    pub json_data: String,
    pub created_at: DateTime<Utc>,
}

/// SQL para criar tabela no PostgreSQL
pub const POSTGRES_CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS nfe (
    id VARCHAR(36) PRIMARY KEY,
    chave_acesso VARCHAR(44) UNIQUE NOT NULL,
    numero INTEGER NOT NULL,
    serie SMALLINT NOT NULL,
    data_emissao TIMESTAMP WITH TIME ZONE NOT NULL,
    emit_cnpj VARCHAR(14) NOT NULL,
    emit_razao_social VARCHAR(255) NOT NULL,
    dest_cnpj VARCHAR(14),
    dest_razao_social VARCHAR(255),
    valor_total DECIMAL(15,2) NOT NULL,
    xml TEXT NOT NULL,
    json_data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_nfe_chave ON nfe(chave_acesso);
CREATE INDEX IF NOT EXISTS idx_nfe_emit_cnpj ON nfe(emit_cnpj);
CREATE INDEX IF NOT EXISTS idx_nfe_data ON nfe(data_emissao);
"#;

/// SQL para criar tabela no MySQL
pub const MYSQL_CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS nfe (
    id VARCHAR(36) PRIMARY KEY,
    chave_acesso VARCHAR(44) UNIQUE NOT NULL,
    numero INT NOT NULL,
    serie SMALLINT NOT NULL,
    data_emissao DATETIME NOT NULL,
    emit_cnpj VARCHAR(14) NOT NULL,
    emit_razao_social VARCHAR(255) NOT NULL,
    dest_cnpj VARCHAR(14),
    dest_razao_social VARCHAR(255),
    valor_total DECIMAL(15,2) NOT NULL,
    xml LONGTEXT NOT NULL,
    json_data JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_nfe_chave (chave_acesso),
    INDEX idx_nfe_emit_cnpj (emit_cnpj),
    INDEX idx_nfe_data (data_emissao)
);
"#;

/// SQL para inserir NF-e (PostgreSQL)
pub const POSTGRES_INSERT: &str = r#"
INSERT INTO nfe (id, chave_acesso, numero, serie, data_emissao, emit_cnpj, emit_razao_social,
                 dest_cnpj, dest_razao_social, valor_total, xml, json_data)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
ON CONFLICT (chave_acesso) DO UPDATE SET
    xml = EXCLUDED.xml,
    json_data = EXCLUDED.json_data
"#;

/// SQL para inserir NF-e (MySQL)
pub const MYSQL_INSERT: &str = r#"
INSERT INTO nfe (id, chave_acesso, numero, serie, data_emissao, emit_cnpj, emit_razao_social,
                 dest_cnpj, dest_razao_social, valor_total, xml, json_data)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
ON DUPLICATE KEY UPDATE
    xml = VALUES(xml),
    json_data = VALUES(json_data)
"#;

/// SQL para buscar NF-e por chave (PostgreSQL/MySQL)
pub const SELECT_BY_CHAVE: &str = "SELECT * FROM nfe WHERE chave_acesso = $1";

/// SQL para listar NF-e (PostgreSQL/MySQL)
pub const SELECT_ALL: &str = "SELECT * FROM nfe ORDER BY created_at DESC LIMIT $1 OFFSET $2";

/// SQL para buscar por CNPJ emitente
pub const SELECT_BY_EMIT_CNPJ: &str = "SELECT * FROM nfe WHERE emit_cnpj = $1 ORDER BY data_emissao DESC";
