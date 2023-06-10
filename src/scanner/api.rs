use std::process::exit;

/// provides the functions needed to connect to various advisory sources.

use reqwest::{self, blocking::Client, Method};
use crate::{parser::structs::Dependency, scanner::models::Vulnerability};

use super::{super::utils};

/// OSV provides a distrubuted database for vulns, with a free API
#[derive(Debug)]
pub struct Osv {

    /// check if the host is online 
    pub online: bool,
    /// time of last query
    pub last_queried: String,
    /// the Client which handles the API.
    client: Client
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
                Ok(Osv { online: true, last_queried: {utils::get_time()}, client})
            }
            else { 
                Err(eprintln!("Could not connect to the OSV website. Check your internet or try again."))
            }
        }
        else { 
            Err(eprintln!("Could not build the network client to connect to OSV. Report this at ---"))
        }

    }

    pub fn query(&self, d: Dependency) -> Option<Vulnerability> {
        // returns None if no vulns found
        // else Some(Vulnerability)

        let version = if d.version.is_some() {d.version} else {
            let res = utils::get_package_version_pypi(d.name.as_str());
            if let Err(e) = res {
                eprintln!("PypiError:\n{}", e.to_string()); exit(1);
            }
            else if let Ok(res) = res {
                Some(res.to_string())
            }
            else {eprintln!("A very unexpected error occurred while retrieving version info from Pypi. Please report this on https://github.com/aswinnnn/pyscan/issues"); exit(1);}
        };
        // println!("{:?}", self.get_latest_package_version(d.name.clone()));

        let res = self.get_json(d.name.as_str(), &version.unwrap());
        // println!("{:?}", res);

        res

    }

    pub fn get_json(&self, name: &str, version: &str) -> Option<Vulnerability> {
        let url = r"https://api.osv.dev/v1/query";

        let body = format!("{{\"version\": \"{}\",\"package\": {{\"name\": \"{}\", \"ecosystem\":\"PyPI\"}}}}",version, name);

        // println!("{}", body.clone());

        let res = self.client.request(Method::POST, url).body(body).send();

        // println!("{:?}", res);

        if let Ok(response) = res {
            if response.status().is_client_error() {eprintln!("Failed connecting to OSV. [Client error]")} else if response.status().is_server_error() {eprintln!("Failed connecting to OSV. [Server error]")}
            let restext = response.text().unwrap();
            if restext.len() > 3 {}
            // check if vulns exist by char len of json
            // api returns '{}' if none found so this is easy

            let parsed: Result<Vulnerability, serde_json::Error> = serde_json::from_str(&restext);
            // println!("{:?}", parsed);
            if let Ok(v) = parsed {
                Some(v)
            }
            else {
                None
            }

        }
        else {
            eprintln!("Could not fetch a response from osv.dev"); exit(1);
        }


    }
}

