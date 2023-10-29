//! Functions and statics concerning with paths and directories important for pyscan's functionality.
use super::queries::retrieve_root;
use anyhow::Error;
use once_cell::sync::Lazy;
use std::{env, fs, path::PathBuf, process::exit};

// contains data on all projects being watched across the user's system
pub static PYSCAN_HOME: Lazy<Result<PathBuf, ()>> = Lazy::new(init_data_dir);

// TODO ! : add .pyscan to .gitignore
// TODO ! : create .store file
// TODO ! : reinitialize if db already exists (add project to HOME db)

// at the project's root directory after `pyscan init`
pub static PYSCAN_ROOT: Lazy<Result<PathBuf, ()>> = Lazy::new(init_project_dir);

fn init_data_dir() -> Result<PathBuf, ()> {
    //! checks for a pyscan data directory (different to a project directory), otherwise creates one.
    //! returns `Ok(path)` if exists or has been created, exits otherwise.
    //! exits because:
    //! - whatever needs this function MUST use the data directory,
    //! which only gets made if the user has done `pyscan init`
    //! - it would be troublesome if the user already had `pyscan init`ed
    //! but couldn't find a data directory, which keeps track of the
    //! projects being watched by pyscan.
    let dir = dirs::data_dir();
    if let Some(d) = dir {
        let path = d.join("pyscan");
        if let Err(e) = path.try_exists() {
            eprintln!(
                "There was an error while checking for pyscan's data in {}.\nerror: {}",
                path.display(),
                e
            );
            exit(1)
        } else {
            // unwrapping should be fine since Err value is accounted for above.
            if path.try_exists().unwrap() {
                Ok(path)
            } else {
                let r = fs::create_dir(path.clone());
                if let Err(e) = r {
                    eprintln!("Pyscan failed to create a folder on your system's data directory.\ndirectory: {}\nerror: {}", d.display(), e);
                    exit(1)
                } else {
                    Ok(path)
                }
            }
        }
    } else {
        eprintln!("Pyscan failed to recognize a data directory for your OS. This rarely happens and should be reported at github.com/aswinnnn/pyscan/issues");
        exit(1)
    }
}

fn init_project_dir() -> Result<PathBuf, ()> {
    //! Creates a project directory OR if it exists, returns its path.
    //! This directory contains the sqlite db and most of the persistent stuff useful for an individual project's security.
    //! - created on `pyscan init` and NOWHERE else
    //! - its used inside a lazy static, which should be the main way of getting the project dir's path
    //! - usage of the static should be done in a context where `pyscan init` has been confirmed to be run.
    let dir = env::current_dir();
    if let Ok(d) = dir {
        let mut dpath = d.clone();
        if let Ok(Some(r)) = exists_check(&mut dpath) {
            Ok(r)
        } else {
            let r = fs::create_dir(d.clone().join(".pyscan"));
            if let Err(e) = r {
                eprintln!("Pyscan failed to create a folder on the current directory.\ndirectory: {}\nerror: {}", d.display(), e);
                exit(1)
            } else {
                Ok(d.join(".pyscan"))
            }
        }
    } else {
        eprintln!("Pyscan failed to get the current working directory. This should not happen and should be reported at github.com/aswinnnn/pyscan/issues");
        exit(1)
    }
}

pub async fn populate_project_dir() -> Result<(), Error> {
    //! populates the .pyscan directory with a database and its tables.

    let (conn, tx) = retrieve_root().await?;

    sqlx::query!(
        r#"
    CREATE TABLE IF NOT EXISTS Vulnerability (
        cve TEXT PRIMARY KEY,
        name TEXT NOT NULL
    );    
    "#
    )
    .execute(&conn)
    .await?;

    sqlx::query!(
        r#"
    CREATE TABLE IF NOT EXISTS Dependency (
        name TEXT PRIMARY KEY,
        version TEXT NOT NULL,
        added TEXT NOT NULL,
        updated TEXT NOT NULL
    );    
    "#
    )
    .execute(&conn)
    .await?;

    sqlx::query!(
        r#"
    CREATE TABLE IF NOT EXISTS VulnerabilityDependency (
        vulnerability_cve TEXT NOT NULL,
        dependency_name TEXT NOT NULL,
        FOREIGN KEY (vulnerability_cve) REFERENCES Vulnerability(cve) ON DELETE CASCADE,
        FOREIGN KEY (dependency_name) REFERENCES Dependency(name) ON DELETE CASCADE,
        PRIMARY KEY (vulnerability_cve, dependency_name)
    );
    "#
    )
    .execute(&conn)
    .await?;

    sqlx::query!(
        r#"
    CREATE TABLE IF NOT EXISTS DependencyChanges (
        hash TEXT NOT NULL,
        name TEXT NOT NULL,
        change CHAR(1) NOT NULL,
        timestamp INTEGER NOT NULL
    );
    "#
    )
    .execute(&conn)
    .await?;
    tx.commit().await?;
    Ok(())
}

/// perform checks to see if the .pyscan exists in a directory or its parent directory.
/// - checks cwd, its parent, and then its parent.
/// - `Ok(PathBuf)` => path exists
/// - `Ok(None)` => path does not exist
/// - `Err` => Error
fn exists_check(path: &mut PathBuf) -> Result<Option<PathBuf>, Error> {
    // try_exists guide:
    // ok(true) => exists
    // ok(false) => does not exist
    // Err => Error
    let mut depth = 0;

    loop {
        if depth == 3 {
            return Ok(None);
        }
        let checkpath = path.join(".pyscan");

        if let Ok(r) = checkpath.try_exists() {
            if r {
                // path exists
                return Ok(Some(checkpath));
            } else {
                // doesnt exist
                if let Some(p) = path.parent() {
                    // path has a parent
                    *path = p.to_path_buf(); // parent is checked next
                    depth += 1
                }
            }
        } else {
            if let Err(e) = path.try_exists() {
                return Err(Error::msg(format!(
                    "An error occurred while checking for .pyscan directory.\nerror: {}",
                    e
                )));
            }
        }
    }
}
