
use chrono::{Timelike, Utc};
use reqwest::{
    self,
    blocking::{Response},
    Method,
};
use crate::scanner::models::Pypi;

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

use std::process::Command;
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

