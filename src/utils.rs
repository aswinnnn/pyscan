use crate::{parser::structs::Dependency, scanner::models::Vulnerability};
use chrono::{Timelike, Utc};
use reqwest::{
    self,
    blocking::{Client, Response},
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
    "0.1.1".to_string()
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

pub fn get_latest_package_version(name: String) -> Option<String> {
    let url = format!(
        "https://api.deps.dev/v3alpha/systems/pypi/packages/{}",
        name.clone()
    );
    // gets the latest released version of a package from pypi.

    let res = reqwest_send("get", url);

    // println!("{:?}", res.unwrap().text());
    // Some("l".to_string())

    if let Some(response) = res {
        let parsed: Result<Pypi, serde_json::Error> =
            serde_json::from_str(response.text().unwrap().as_str());

        if let Ok(pypi) = parsed {
            // println!("{:?}", pypi);
            // Some("()".to_string())
            if let Some(v) = pypi.versions.iter().last().cloned() {
                let s = v
                    .iter()
                    .last()
                    .unwrap()
                    .to_owned()
                    .version_key
                    .unwrap()
                    .version;
                Some(s)
            } else {
                eprintln!("Could not identify the latest version of the package {}. Please add the version specification to your source and try again.", name.clone());
                None
            }
        } else {
            eprintln!("There was a problem finding the latest version of {}. Either it does not exist or the API cannot identify the latest version. Please provide a version specification in your source instead.", name.clone());
            None
        }
    } else {
        eprintln!("Could not reach the pypi API to fetch the latest version of {}. Please provide a version specification in your source.", name.clone());
        None
    }
}
