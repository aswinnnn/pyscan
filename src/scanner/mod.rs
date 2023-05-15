mod api;
pub mod models;
use crate::parser::structs::ScannedDependency;

use self::models::{Vulnerability, Vuln};

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
        if let Some(ok) = d.version {
            continue
        }
        else {
            d.version = osv.get_latest_package_version(d.name.clone());
        }
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

    cons.write_line(&format!("{}", style("SUMMARY").bold().yellow().underlined()).to_string())?;
    for v in collected {


            for vuln in v.vuln.vulns {

                let name = format!("Dependency: {}", style(v.name.clone()).bold().bright().red());
                let id = format!("ID: {}",style(vuln.id).bold().bright().yellow());
                let details = format!("Details: {}", style(vuln.details).italic());
                // let summary = format!("summary: {}", vuln.summary);
                let vers: Vec<Vec<String>> = vuln.affected.iter().map(|affected| {vec![affected.versions.first().unwrap().to_string(), affected.versions.last().unwrap().to_string()]}).collect();
                let version = format!("Versions affected: {} to {}", style(vers.first().expect("No version found affected").first().unwrap()).dim().underlined(), style(vers.last().expect("No version found affected").last().unwrap()).dim().underlined());
                print!("\n");


                cons.write_line(name.as_str())?;
                cons.write_line(id.as_str())?;
                cons.write_line(details.as_str())?;
                cons.write_line(version.as_str())?;

            }

        }
    
    Ok(())
}


fn version_check(against: ScannedDependency) -> bool {
    // Check input version against the affected versions
    // from OSV. Would osv return a vuln that doesnt have our version? idk
    // but im not gonna fear-monger the user. Lets make sure. 
    let mut check = false;
    let vulns = against.vuln.vulns;
    let ver = against.version;

    for vuln in vulns {
        let multi_affected = vuln.affected.iter().map(|x| {x.clone().versions}); // multi-dimensional array of versions
        let pre_single_affected: Vec<Vec<String>> = multi_affected.map(|v| {v}).collect(); // processing it
        let single_affected: Vec<String> = pre_single_affected.iter().map(|x| {x.to_owned().drain(..x.len()).collect()}).collect(); // yay single array of versions

        for a in single_affected {
            if ver == a {
                check = true
            }
            else {continue}
        }
    }

    check
}