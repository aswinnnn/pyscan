mod queries;
mod paths;
use anyhow::Error;
use sqlx::SqlitePool;
use chrono::{Utc, DateTime, NaiveDateTime, NaiveDate};
use paths::{PYSCAN_HOME, PYSCAN_ROOT};

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
    pub async fn update_or_create() -> Result<(),Error> {
        let time = Utc::now();
        let mut conn = SqlitePool::connect(PYSCAN_ROOT.clone().unwrap().to_str().unwrap()).await?.acquire().await?;

        sqlx::query!(r#"CREATE TABLE IF NOT EXISTS pipcache (
            name TEXT NOT NULL,
            version TEXT NOT NULL
        )"#).execute(&mut *conn).await?;

        Ok(())
    }
}