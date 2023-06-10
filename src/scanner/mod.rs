pub mod api;
pub mod models;
use std::process::exit;
use crate::{parser::structs::ScannedDependency, utils};
use super::parser::structs::Dependency;
use console::{Term, style};

pub fn start(imports: Vec<Dependency>) -> Result<(), std::io::Error> {
    let cons = Term::stdout();
    let s = format!("Found {} dependencies...", style(format!("{}", imports.len()))
    .bold()
    .green());

    cons.write_line(&s)?;

    let osv = api::Osv::new().unwrap(); // err handling done inside, unwrapping is safe
    let mut collected: Vec<ScannedDependency> = Vec::new();

    for mut d in imports {
        
        // check if version was provided
        d.version = if let Some(provided) = d.version {
            Some(provided)
        }
        else {
            if let Ok(v) = utils::get_python_package_version(d.name.as_str()) {
                println!("{} : {}",style(d.name.as_str()).yellow().dim(), style("A version could not be detected in the source file, so retrieving version from pip instead.").dim());
                Some(v)
            }
            else if let Ok(v) = utils::get_package_version_pypi(d.name.as_str()) {
                println!("{} : {}",style(d.name.as_str()).red().dim(), style("A version could not be detected through source or pip, so retrieving latest version from pypi.org instead.").dim());
                Some(v.to_string())
            }
            else {
                eprintln!("A version could not be retrieved for {}. This should not happen as pyscan defaults pip or pypi.org, unless you don't have an internet connection, the provided package name is wrong or if the package does not exist.\nReach out on github.com/aswinnnn/pyscan/issues if the above cases did not take place.", style(d.name.as_str()).bright().red()); exit(1);
            }
        };
        let mut depstr = format!("|-| {} [{}]", style(d.name.clone()).bold().bright().yellow(), style(d.version.clone().unwrap().to_string()).bold().dim());
        cons.write_line(&depstr)?;

        let res = osv.query(d.clone());
        if let Some(v) = res {
            depstr.push_str(
                format!("{}", style(" -> Found vulnerabilities!").bold().bright().red()).as_str()
            );
            cons.clear_last_lines(1)?;
            cons.write_line(&depstr)?;
            collected.push(ScannedDependency { name:  d.name, version: d.version.unwrap(), vuln: v });

        }
        else {
            depstr.push_str(
                format!("{}", style(" -> No vulnerabilities found.").bold().bright().green()).as_str()
            );
            cons.clear_last_lines(1)?;
            cons.write_line(&depstr)?;
        }

    }

    // --- summary starts here ---

    if !collected.is_empty() {
        cons.write_line(&format!("{}", style("SUMMARY").bold().yellow().underlined()))?;
        for v in collected {
    
    
                for vuln in v.vuln.vulns {
                    // DEPENDENCY    
                    let name = format!("Dependency: {}", style(v.name.clone()).bold().bright().red());

                    // ID
                    let id = format!("ID: {}",style(vuln.id).bold().bright().yellow());

                    // DETAILS
                    let details = format!("Details: {}", style(vuln.details).italic());

                    // VERSIONS AFFECTED from ... to
                    let vers: Vec<Vec<String>> = vuln.affected.iter().map(|affected| {vec![affected.versions.first().unwrap().to_string(), affected.versions.last().unwrap().to_string()]}).collect();

                    let version = format!("Versions affected: {} to {}", style(vers.first().expect("No version found affected").first().unwrap()).dim().underlined(), style(vers.last().expect("No version found affected").last().unwrap()).dim().underlined());

                    println!();
    
    
                    cons.write_line(name.as_str())?;
                    cons.write_line(id.as_str())?;
                    cons.write_line(details.as_str())?;
                    cons.write_line(version.as_str())?;
    
                }
    
            }
    }
    else { exit(0)}
    
    Ok(())
}


