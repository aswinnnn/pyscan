mod queries;
mod paths;
use anyhow::Error;
use sqlx::SqlitePool;
use chrono::{Utc, DateTime, NaiveDateTime, NaiveDate};
use paths::{PYSCAN_HOME, PYSCAN_ROOT};
use self::queries::retrieve_root;

// single unit of a dependency retrieved from pip
struct PipDependency {
    pkg_name: String,
    pkg_version: String,
    pkg_requires: String,
}


struct PipCache {
    connected: bool,
    last_update: DateTime<Utc>,
}

impl PipCache {
    pub async fn create_table() -> Result<(),Error> {
        let (conn, tx) = retrieve_root().await?;

        sqlx::query!(r#"CREATE TABLE IF NOT EXISTS pipcache (
            name TEXT NOT NULL,
            version TEXT NOT NULL)"#).execute(&conn).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn add(name: &str, version: &str) -> Result<(), Error> {
        let (conn, tx) = retrieve_root().await?;

        sqlx::query!("INSERT INTO pipcache (name, version) VALUES (?, ?)", name, version).execute(&conn).await?;

        tx.commit().await?;
        
        Ok(())
    }
    pub async fn update(name: &str, version: &str) -> Result<(), Error> {
        let (conn, tx) = retrieve_root().await?;

        sqlx::query!("UPDATE pipcache SET name = ?, version = ?", name, version).execute(&conn).await?;

        tx.commit().await?;
        
        Ok(())
    }

    pub async fn remove(name: &str) -> Result<(), Error> {
        let (conn, tx) = retrieve_root().await?;

        sqlx::query!("DELETE FROM pipcache WHERE name = ?;", name).execute(&conn).await?;

        tx.commit().await?;
        
        Ok(())
    }



}