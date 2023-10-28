//! This module deals with data storage. Databases, Caches, Paths, etc.
mod queries;
mod paths;
mod cache;
use anyhow::Error;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use queries::retrieve_root;
use async_trait::async_trait;
use sqlx::query;



/// Represents the single, in-database Dependency row. NOT TO BE CONFUSED with the struct with same name in `parser::structs`
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub added: NaiveDate,
    pub updated: NaiveDate
}
/// Represents the single, in-database Vulnerability. NOT TO BE CONFUSED with the struct with same name in `scanner::models`
struct Vulnerability {
    cve: String,
    name: String,
}
/// Represents the (many-to-many) relation between vulnerabilities and python packages.
struct VulnerabilityDependency {
    cve: String,
    package: String
}

enum DatabaseTable {
    Dependency(Dependency),
    Vulnerability(Vulnerability),
    VulnerabilityDependency(VulnerabilityDependency),
}

/// Database operations for different tables.
#[async_trait]
trait DatabaseOps {

    async fn insert(d: DatabaseTable) -> Result<(), Error> {
        let (conn, tx) = retrieve_root().await?;
        match d {
            DatabaseTable::Dependency(d) => {
            query!("
            INSERT INTO Dependency (name, version, added, updated)
            VALUES (?,?,?,?)
            ", d.name, d.version, d.added, d.updated).execute(&conn).await?;
            Ok(())
            },
            DatabaseTable::Vulnerability(v) => {
                Ok(())
            },
            DatabaseTable::VulnerabilityDependency(vd) => {
                Ok(())
            },
        }
    
    }

}

