//! Módulo MySQL

use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use super::models::*;

/// Cliente MySQL
pub struct MysqlClient {
    pool: MySqlPool,
}

impl MysqlClient {
    /// Conecta ao MySQL
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Cria a tabela se não existir
    pub async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(MYSQL_CREATE_TABLE)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Insere uma NF-e
    pub async fn insert(&self, record: &NfeRecord) -> Result<(), sqlx::Error> {
        sqlx::query(MYSQL_INSERT)
            .bind(&record.id)
            .bind(&record.chave_acesso)
            .bind(record.numero)
            .bind(record.serie)
            .bind(record.data_emissao)
            .bind(&record.emit_cnpj)
            .bind(&record.emit_razao_social)
            .bind(&record.dest_cnpj)
            .bind(&record.dest_razao_social)
            .bind(record.valor_total)
            .bind(&record.xml)
            .bind(&record.json_data)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Busca NF-e por chave de acesso
    pub async fn find_by_chave(&self, chave: &str) -> Result<Option<NfeRecord>, sqlx::Error> {
        let row = sqlx::query_as::<_, (String, String, i32, i16, chrono::DateTime<chrono::Utc>, String, String, Option<String>, Option<String>, f64, String, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, chave_acesso, numero, serie, data_emissao, emit_cnpj, emit_razao_social, dest_cnpj, dest_razao_social, valor_total, xml, json_data, created_at FROM nfe WHERE chave_acesso = ?"
        )
            .bind(chave)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| NfeRecord {
            id: r.0,
            chave_acesso: r.1,
            numero: r.2,
            serie: r.3,
            data_emissao: r.4,
            emit_cnpj: r.5,
            emit_razao_social: r.6,
            dest_cnpj: r.7,
            dest_razao_social: r.8,
            valor_total: r.9,
            xml: r.10,
            json_data: r.11,
            created_at: r.12,
        }))
    }

    /// Lista NF-e com paginação
    pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<NfeRecord>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, i32, i16, chrono::DateTime<chrono::Utc>, String, String, Option<String>, Option<String>, f64, String, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, chave_acesso, numero, serie, data_emissao, emit_cnpj, emit_razao_social, dest_cnpj, dest_razao_social, valor_total, xml, json_data, created_at FROM nfe ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(|r| NfeRecord {
            id: r.0,
            chave_acesso: r.1,
            numero: r.2,
            serie: r.3,
            data_emissao: r.4,
            emit_cnpj: r.5,
            emit_razao_social: r.6,
            dest_cnpj: r.7,
            dest_razao_social: r.8,
            valor_total: r.9,
            xml: r.10,
            json_data: r.11,
            created_at: r.12,
        }).collect())
    }
}
