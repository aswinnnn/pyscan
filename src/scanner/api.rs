use crate::{parser::structs::Dependency, scanner::models::Vulnerability};
use crate::{
    parser::structs::{ScannedDependency, VersionStatus},
    scanner::models::Vuln,
};
use reqwest::{self, blocking::Client, Method};
use std::collections::HashMap;
/// provides the functions needed to connect to various advisory sources.
use std::process::exit;

use super::{
    super::utils,
    models::{Query, QueryBatched, QueryResponse},
};

/// OSV provides a distrubuted database for vulns, with a free API
#[derive(Debug)]
pub struct Osv {
    /// check if the host is online
    pub online: bool,
    /// time of last query
    pub last_queried: String,
    /// the Client which handles the API.
    client: Client,
}

impl Osv {
    pub fn new() -> Result<Osv, ()> {
        let version = utils::get_version();
        let pyscan_version = format!("pyscan {}", version);
        let client = reqwest::blocking::Client::builder()
            .user_agent(pyscan_version)
            .build();

        if let Ok(client) = client {
            let res = client.get("https://osv.dev").send();

            if let Ok(_success) = res {
                Ok(Osv {
                    online: true,
                    last_queried: { utils::get_time() },
                    client,
                })
            } else {
                eprintln!(
                    "Could not connect to the OSV website. Check your internet or try again."
                ); exit(1)
            }
        } else {
            eprintln!(
                "Could not build the network client to connect to OSV. Report this at github.com/aswinnnn/pyscan/issues"
            ); exit(1)
        }
    }

    pub fn query(&self, d: Dependency) -> Option<Vulnerability> {
        // returns None if no vulns found
        // else Some(Vulnerability)

        let version = if d.version.is_some() {
            d.version
        } else {
            let res = utils::get_package_version_pypi(d.name.as_str());
            if let Err(e) = res {
                eprintln!("PypiError:\n{}", e.to_string());
                exit(1);
            } else if let Ok(res) = res {
                Some(res.to_string())
            } else {
                eprintln!("A very unexpected error occurred while retrieving version info from Pypi. Please report this on https://github.com/aswinnnn/pyscan/issues");
                exit(1);
            }
        };
        // println!("{:?}", self.get_latest_package_version(d.name.clone()));

        let res = self.get_json(d.name.as_str(), &version.unwrap());
        // println!("{:?}", res);

        res
    }

    pub fn query_batched(&self, mut deps: Vec<Dependency>) -> (Vec<ScannedDependency>, HashMap<String, String>) {
        // runs the batch API. Each dep is converted into JSON format here, POSTed, and the response of vuln IDs -> queried into Vec<Vulnerability> -> returned as Vec<ScannedDependency> -> enters the summary part of scanner::start()
        // The dep version conflicts are also solved over here.
        let _ = deps
            .iter_mut()
            .map(|d| {
                d.version = if d.version.is_none() {
                    Some(VersionStatus::choose(d.name.as_str(), &d.version))
                } else {
                    d.version.clone()
                }
            })
            .collect::<Vec<_>>();

        let imports_info = utils::vecdep_to_hashmap(&deps);

        let url = "https://api.osv.dev/v1/querybatch";

        let queries: Vec<Query> = deps.iter().map(|d| d.to_query()).collect();
        let batched = QueryBatched::new(queries);

        let body = serde_json::to_string(&batched).unwrap();
        println!("{:#?}", body.clone());

        let res = self.client.request(Method::POST, url).body(body).send();
        if let Ok(response) = res {
            if response.status().is_client_error() {
                eprintln!("Failed connecting to OSV. [Client error]");
                exit(1)
            } else if response.status().is_server_error() {
                eprintln!("Failed connecting to OSV. [Server error]");
                exit(1)
            }

            let restext = response.text().unwrap();
            println!();

            let parsed: Result<QueryResponse, serde_json::Error> = serde_json::from_str(&restext);
            let mut scanneddeps: Vec<ScannedDependency> = Vec::new();

            if let Ok(p) = parsed {
                for vres in p.results {
                    if let Some(vulns) = vres.vulns {
                        let vecvulns: Vec<Vuln> = vulns.iter().map(|qv| {
                            self.vuln_id(qv.id.as_str()) // retrives vuln info from API with a vuln ID
                        }).collect();

                        // has to be turnt to Vulnerability before becoming a scanned dependency
                        let structvuln = Vulnerability {vulns: vecvulns};

                        scanneddeps.push(structvuln.to_scanned_dependency(&imports_info));

                    }
                    else {continue;}
                }
                (scanneddeps, imports_info)
            } else {
                eprintln!("Invalid parse of API reponse at src/scanner/api.rs::query_batched");
                exit(1);
            }
        } else {
            eprintln!("Could not fetch a response from osv.dev [scanner/api/query_batched]");
            exit(1);
        }
    }

    /// get a Vuln from a vuln ID from OSV
    pub fn vuln_id(&self, id: &str) -> Vuln {
        let url = format!("https://api.osv.dev/v1/vulns/{id}");

        let res = self.client.request(Method::GET, url).send();

        // println!("{:?}", res);

        if let Ok(response) = res {
            if response.status().is_client_error() {
                eprintln!("Failed connecting to OSV. [Client error]")
            } else if response.status().is_server_error() {
                eprintln!("Failed connecting to OSV. [Server error]")
            }
            let restext = response.text().unwrap();
            let parsed: Result<Vuln, serde_json::Error> = serde_json::from_str(&restext);
            if let Ok(p) = parsed {
                p
            } else {
                eprintln!("Invalid parse of API reponse at src/scanner/api.rs::vuln_id");
                exit(1);
            }
        } else {
            eprintln!("Could not fetch a response from osv.dev [scanner/api/vulns_id]");
            exit(1);
        }
    }

    pub fn get_json(&self, name: &str, version: &str) -> Option<Vulnerability> {
        let url = r"https://api.osv.dev/v1/query";

        let body = Query::new(version, name); // struct implementation of query sent to OSV API.
        let body = serde_json::to_string(&body).unwrap();

        // println!("{}", body.clone());

        let res = self.client.request(Method::POST, url).body(body).send();

        // println!("{:?}", res);

        if let Ok(response) = res {
            if response.status().is_client_error() {
                eprintln!("Failed connecting to OSV. [Client error]")
            } else if response.status().is_server_error() {
                eprintln!("Failed connecting to OSV. [Server error]")
            }
            let restext = response.text().unwrap();
            if !restext.len() < 3 {
                // check if vulns exist by char len of json
                // api returns '{}' if none found so this is easy

                let parsed: Result<Vulnerability, serde_json::Error> =
                    serde_json::from_str(&restext);
                // println!("{:?}", parsed);
                if let Ok(v) = parsed {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            eprintln!("Could not fetch a response from osv.dev");
            exit(1);
        }
    }
}
