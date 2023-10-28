//! This module deals with data storage. Databases, Caches, Paths, etc.
mod cache;
mod paths;
mod queries;
use std::fmt::{write, Display};

use anyhow::Error;
use async_trait::async_trait;
use chrono::{format, NaiveDate, NaiveDateTime, NaiveTime};
use queries::retrieve_root;
use sqlx::query;

/// Represents the single, in-database Dependency row. NOT TO BE CONFUSED with the struct with same name in `parser::structs`
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub added: NaiveDate,
    pub updated: NaiveDate,
}
/// Represents the single, in-database Vulnerability. NOT TO BE CONFUSED with the struct with same name in `scanner::models`
pub struct Vulnerability {
    pub cve: String,
    pub name: String,
}
/// Represents the (many-to-many) relation between vulnerabilities and python packages.
pub struct VulnerabilityDependency {
    pub cve: String,
    pub package: String,
}

enum DatabaseTable {
    Dependency(Dependency),
    Vulnerability(Vulnerability),
    VulnerabilityDependency(VulnerabilityDependency),
}

/// Database of a single individual project being watched by Pyscan.
/// All manipulations are done via functions.
/// To execute queries directly see `queries::retrieve_root`
struct ProjectDatabase;

impl DatabaseOps for ProjectDatabase {}

impl ProjectDatabase {
    fn new() -> ProjectDatabase {
        ProjectDatabase
    }
}

/// Database operations for different tables.
/// - Used by `ProjectDatabase` struct.
/// - Makes it so that its easy to update different tables just by passing structs.
#[async_trait]
trait DatabaseOps {
    async fn insert(d: DatabaseTable) -> Result<(), Error> {
        let (conn, tx) = retrieve_root().await?;
        match d {
            DatabaseTable::Dependency(dep) => {
                query!(
                    "
            INSERT INTO Dependency (name, version, added, updated)
            VALUES (?,?,?,?);
            ",
                    dep.name,
                    dep.version,
                    dep.added,
                    dep.updated
                )
                .execute(&conn)
                .await?;
                tx.commit().await?;
                Ok(())
            }
            DatabaseTable::Vulnerability(v) => {
                query!(
                    "
            INSERT INTO Vulnerability (cve, name)
            VALUES (?,?);
            ",
                    v.cve,
                    v.name
                )
                .execute(&conn)
                .await?;
                tx.commit().await?;
                Ok(())
            }
            DatabaseTable::VulnerabilityDependency(vd) => {
                // have to use function here because the query macro doesnt agree with what
                // i'm doing for some reason
                sqlx::query(
                    r#"
                INSERT INTO VulnerabilityDependency (vulnerability_cve, dependency_name)
                VALUES (?,?);
                "#,
                )
                .bind(vd.cve)
                .bind(vd.package)
                .execute(&conn)
                .await?;
                tx.commit().await?;
                Ok(())
            }
        }
    }

    async fn update(d: DatabaseTable) -> Result<(), Error> {
        let (conn, tx) = retrieve_root().await?;
        match d {
            DatabaseTable::Dependency(dep) => {
            query!("
            UPDATE Dependency SET name = ?,  version = ?, added = ?, updated = ?
            WHERE name = ?;
            ", dep.name, dep.version, dep.added, dep.updated, dep.name).execute(&conn).await?;
            tx.commit().await?;
            Ok(())
            },
            DatabaseTable::Vulnerability(v) => {
                Err(Error::msg(format!("There is no reason to update the Vuln table. Rows should either be removed or created upon discovering and discarding Vulnerabilities.\nAn update attempt was made:\n {}", v)))
            }
            DatabaseTable::VulnerabilityDependency(vd) => {
                Err(Error::msg(format!("There is no reason to update the VD table. Rows should either be removed or created upon discovering and discarding vulns in packages.\nAn update attempt was made with this row:\n {}", vd)))
            },
        }
    }

    async fn delete(d: DatabaseTable) -> Result<(), Error> {
        let (conn, tx) = retrieve_root().await?;
        match d {
            DatabaseTable::Dependency(dep) => {
                sqlx::query(
                    r#"
            DELETE FROM VulnerabilityDependency
            WHERE dependency_name = ?;
            DELETE FROM Dependency
            WHERE name = ?;
            "#,
                )
                .bind(&dep.name)
                .bind(&dep.name)
                .execute(&conn)
                .await?;
                tx.commit().await?;
                Ok(())
            }
            DatabaseTable::Vulnerability(v) => {
                sqlx::query(
                    r#"
                DELETE FROM VulnerabilityDependency
                WHERE vulnerability_cve = ?;
                DELETE FROM Vulnerability
                WHERE cve = ?;
                "#,
                )
                .bind(v.cve)
                .execute(&conn)
                .await?;
                tx.commit().await?;
                Ok(())
            }
            DatabaseTable::VulnerabilityDependency(vd) => {
                sqlx::query(
                    "
                DELETE FROM VulnerabilityDependency
                WHERE vulnerability_cve = ? AND dependency_name = ?;
                ",
                )
                .bind(vd.cve)
                .bind(vd.package)
                .execute(&conn)
                .await?;
                tx.commit().await?;
                Ok(())
            }
        }

    }
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Dependency: {},\nVersion: {},\nAdded: {},\nUpdated: {}\n",
            self.name, self.version, self.added, self.updated
        )
    }
}

impl std::fmt::Display for Vulnerability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "CVE: {},\nName: {}\n", self.cve, self.name)
    }
}

impl std::fmt::Display for VulnerabilityDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Vulnerability {} was found in {}",
            self.cve, self.package
        )
    }
}
