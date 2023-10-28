//! functions useful for querying the databases.
use anyhow::Error;
use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::query;
use sqlx::SqlitePool;
use super::paths::{PYSCAN_ROOT, PYSCAN_HOME};


pub async fn retrieve_root<'a>() -> Result<(Pool<Sqlite>, sqlx::Transaction<'a, Sqlite>), Error> {
    //! Begins a database connection to the db in .pyscan folder and returns the connection
    //! and an open transaction.
    //! it has a seperate function to make it easier to call and do transactions.
    
    let dburl = format!("sqlite://{}/.store", PYSCAN_ROOT.clone().unwrap().to_str().unwrap());
    let conn = SqlitePool::connect(&dburl).await?;
    let tx: sqlx::Transaction<'_, Sqlite> = conn.begin().await?;
    Ok((conn, tx))
}