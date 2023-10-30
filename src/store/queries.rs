//! functions useful for querying the databases.
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::path::PathBuf;

use anyhow::Error;
use chrono::NaiveDateTime;
use sqlx::Column;
use sqlx::Pool;
use sqlx::Row;
use sqlx::Sqlite;

use sqlx::SqlitePool;
use super::paths::PYSCAN_HOME;
use super::paths::{PYSCAN_ROOT};


pub async fn retrieve_root<'a>() -> Result<(Pool<Sqlite>, sqlx::Transaction<'a, Sqlite>), Error> {
    //! Begins a database connection to the db in .pyscan folder and returns the connection
    //! and an open transaction.
    //! it has a seperate function to make it easier to call and do transactions.
    
    let dburl = format!("sqlite://{}/.store", PYSCAN_ROOT.clone().unwrap().to_str().unwrap());
    let conn = SqlitePool::connect(&dburl).await?;
    let tx: sqlx::Transaction<'_, Sqlite> = conn.begin().await?;
    Ok((conn, tx))
}

pub async fn retrieve_home<'a>() -> Result<(Pool<Sqlite>, sqlx::Transaction<'a, Sqlite>), Error> {
    //! Begins a database connection to the HOME db in `{daata_directory}/pyscan` folder and returns the connection
    //! and an open transaction.
    //! it has a seperate function to make it easier to call and do transactions.
    
    let dburl = format!("sqlite://{}/pdata", PYSCAN_HOME.clone().unwrap().to_str().unwrap());
    let conn = SqlitePool::connect(&dburl).await?;
    let tx: sqlx::Transaction<'_, Sqlite> = conn.begin().await?;
    Ok((conn, tx))
}
/// Represents project metadata stored in pyscan's home database.
#[derive(PartialEq, Eq, Hash)]
struct Project {
    identifier: String,
    path: String,
    added: String
}

type Projects = HashSet<Project>;

/// An abstraction over the pyscan home database.
/// this is just like the ProjectDatabase struct but for pyscan as a whole
pub struct PyscanData;

impl PyscanData {
    pub async fn new() -> PyscanData {
        PyscanData
    }
    pub async fn settings(&self) -> Result<HashMap<String, String>, Error> {
        //! view global settings.
        struct Setting {key: String, value: String}
        let mut map: HashMap<String, String> = HashMap::new();
        let (conn, tx) = retrieve_home().await?;

        let rows = sqlx::query_as!(Setting, r#"
        SELECT key,value FROM Settings;
        "#).fetch_all(&conn).await?;
        tx.commit().await?;
        
        for row in rows {
            map.insert(row.key, row.value);
        }

        Ok(map)
    }

    async fn update_settings(key: &str, value: &str) -> anyhow::Result<()> {
        //! update or add a global setting.
        let (conn, tx) = retrieve_home().await?;

        sqlx::query!(r#"
        INSERT OR IGNORE INTO Settings (key,value) VALUES (?,?);
        UPDATE Settings SET key = ?, value = ?
        WHERE key = ?;
        "#,
        key,value,key,value,key
    ).execute(&conn).await?;
    tx.commit().await?;

        Ok(())
    }
    async fn projects() -> anyhow::Result<Projects> {
        //! view all the projects' information.
        let mut r = Projects::new();
        let (conn, tx) = retrieve_home().await?;
        
        let p = sqlx::query_as!(Project, r#"
        SELECT project_id as identifier, path, added FROM ProjectInfo;
        "#).fetch_all(&conn).await?;

        for project in p {
            r.insert(project);
        }
        tx.commit().await?;
        Ok(r)
    

    }
    async fn projects_add(project: Project) -> anyhow::Result<()> {
        let (conn, tx) = retrieve_home().await?;
        sqlx::query!(r#"
        INSERT INTO ProjectInfo (project_id, path, added)
        VALUES (?,?,?)
        "#, project.identifier, project.path, project.added).execute(&conn).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn projects_update(project: Project) -> anyhow::Result<()> {
        //! the only thing that can be updated in this table is path.
        //! the others, one is a primary key and the other is a timestamp of when that project 
        //! was inited.
        let (conn, tx) = retrieve_home().await?;
        sqlx::query!(r#"
        UPDATE ProjectInfo SET path = ?
        WHERE project_id = ?
        "#, project.path, project.identifier).execute(&conn).await?;
        tx.commit().await?;
        Ok(())
    }
}


