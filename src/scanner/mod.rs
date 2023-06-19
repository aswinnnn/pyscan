pub mod api;
pub mod models;
use std::process::exit;

use crate::ARGS;

use super::parser::structs::Dependency;
use console::{Term, style};


pub fn start(imports: Vec<Dependency>) -> Result<(), std::io::Error> {
    let osv = api::Osv::new().unwrap(); // err handling done inside, unwrapping is safe
    let cons = Term::stdout();
    let s = format!("Found {} dependencies...", style(format!("{}", imports.len()))
    .bold()
    .green());

    cons.write_line(&s)?;

    // collected contains the dependencies with found vulns. imports_info contains a name, version hashmap of all found dependencies so we can display for all imports if vulns have been found or not
    let (collected,mut imports_info) = osv.query_batched(imports); 
    
    // --- displaying query result starts here ---
    for dep in &collected {
        let _ = cons.write_line(format!("|-| {} [{}]{:^5}", style(dep.name.as_str()).bold().bright().yellow(), style(dep.version.as_str()).bold().dim(), style(" -> Found vulnerabilities!").bold().bright().red()).as_str());

    } // displays all the deps where vuln has been found 

    // remove the the deps with vulns from import_info so what remains is the safe deps, which we can display as safe
    for d in collected.iter() {
        imports_info.remove(d.name.as_str());
    }

    for (k,v) in imports_info.iter() {
        let _ = cons.write_line(format!("|-| {} [{}]{}", style(k.as_str()).bold().bright().yellow(), style(v.as_str()).bold().dim(), style(" -> No vulnerabilities found.").bold().bright().green()).as_str());
    } // display the safe deps

    if !collected.is_empty() {
        // thing is, collected only has vulnerable dependencies, if theres a case where no vulns have been found, it will just skip this entire thing.
        

        // --- summary starts here ---
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
                    let vers: Vec<Vec<String>> = vuln.affected.iter().map(|affected| {vec![{
                        if let Some(v) = &affected.versions {
                            v.first().unwrap().to_string()
                        }
                        else {"This version".to_string()}

                    }, {
                        if let Some(v) = &affected.versions {
                            v.last().unwrap().to_string()
                        }
                        else {"Unknown".to_string()}
                    }]}).collect();
                    // let vers: Vec<Vec<String>> = vuln.affected.iter().map(|affected| {vec![affected.versions.first().unwrap().to_string(), affected.versions.last().unwrap().to_string()]}).collect();

                    let version = format!("Versions affected: {} to {}", style(vers.first().expect("No version found affected").first().unwrap()).dim().underlined(), style(vers.last().expect("No version found affected").last().unwrap()).dim().underlined());

                    println!();
    
    
                    cons.write_line(name.as_str())?;
                    cons.write_line(id.as_str())?;
                    cons.write_line(details.as_str())?;
                    cons.write_line(version.as_str())?;
    
                }
    
        }
    }
    else { println!("Finished scanning all found dependencies."); exit(0)}
    
    Ok(())
}


