use std::boxed::Box;
use chrono::{Timelike, Utc};
use reqwest::{
    self,
    blocking::{Response, Client},
    Method, header::USER_AGENT,
};
pub fn get_time() -> String {
    // get the current time in a stting format i like.
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    {
        let time = format!(
            "{:02}:{:02}:{:02} {}",
            hour,
            now.minute(),
            now.second(),
            if is_pm { "PM" } else { "AM" }
        );

        time
    }
}

pub fn get_version() -> String {
    "0.1.3".to_string()
}

pub fn reqwest_send(method: &str, url: String) -> Option<Response> {
    // for easily sending web requests
    
    let client = reqwest::blocking::Client::builder()
        .user_agent(format!("pyscan v{}", get_version()))
        .build();

    if let Ok(client) = client {
        let method = match method {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            "head" => Method::HEAD,
            "connect" => Method::CONNECT,
            "trace" => Method::TRACE,
            &_ => {
                println!("Didn't recognize that method so defaulting to GET");
                Method::GET
            }
        };
        let res = client.request(method, url).send();

        if let Ok(success) = res {
            Some(success)
        } else {
            eprintln!(
                "Could not establish an internet connection. Check your internet or try again."
            );
            None
        }
    } else {
        eprintln!("Could not build the network client. Report this at https://github.com/aswinnnn/pyscan/issues");
        None
    }
}

use std::process::{Command, exit};

use crate::scanner::models::PypiResponse;
// Define a custom error type that wraps a String message
#[derive(Debug)]
pub struct PipError(String);

// Implement the std::error::Error trait for DockerError
impl std::error::Error for PipError {}

// Implement the std::fmt::Display trait for DockerError
impl std::fmt::Display for PipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pip error: {}", self.0)
    }
}

pub fn get_python_package_version(package: &str) -> Result<String, PipError> {
    // gets the version of a package from pip.
    
    let output = Command::new("pip")
        .arg("show")
        .arg(package)
        .output().map_err(|e| {PipError(e.to_string())})?;

    let output = output.stdout;
    let output = String::from_utf8(output)
    .map_err(|e| {PipError(e.to_string())})?;

    let version = output
        .lines()
        .find(|line| line.starts_with("Version: "))
        .map(|line| line[9..].to_string());
    
    if let Some(v) = version { Ok(v)} 
    else { Err(PipError("could not retrive package version from Pip".to_string())) }
}

#[derive(Debug)]
pub struct PypiError(String);

// Implement the std::error::Error trait for DockerError
impl std::error::Error for PypiError {}

// Implement the std::fmt::Display trait for DockerError
impl std::fmt::Display for PypiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pypi.org error: {}", self.0)
    }
}

impl From<reqwest::Error> for PypiError {
    fn from(item: reqwest::Error) -> Self {
        PypiError(item.to_string())
    }
}

pub fn get_package_version_pypi<'a>(package: &str) -> Result<Box<String>, PypiError> {
    let url = format!("https://pypi.org/pypi/{package}/json");

    let client = Client::new();
    let res = client.get(url).header(USER_AGENT, "pyscan").send()?.error_for_status();

    let version = if let Err(e) = res {
        eprintln!("Failed to make a request to pypi.org:\n{}", e); Err(PypiError(e.to_string()))
    }
    else if let Ok(r) = res {
        let restext = r.text();
        let restext = if let Ok(r) = restext {r} else {eprintln!("Failed to connect to pypi.org"); exit(1)};
        // println!("{:#?}", restext.clone());

        let parsed: Result<PypiResponse, serde_json::Error> = serde_json::from_str(&restext.trim());

        let version = if let Err(e) = parsed {
            eprintln!("Failed to parse reponse from pypi.org:\n{}", e); Err(PypiError(e.to_string()))
        }
        else if let Ok(pypi) = parsed {
            let mut version: Vec<String> = pypi.releases.into_keys().collect();
            version.sort();
            Ok(version.last().unwrap().to_owned())
        }
        else {Err(PypiError("pypi.org response error".to_string()))};
        version

    }
    else {exit(1)};
    Ok(Box::new(if let Err(e) = version {eprintln!("{e}"); exit(1)} else {version.unwrap()}))
}

