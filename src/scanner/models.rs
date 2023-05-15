// automatically generated. do not change.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    #[serde(rename = "vulns")]
    pub vulns: Vec<Vuln>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vuln {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "summary")]
    pub summary: Option<String>,

    #[serde(rename = "details")]
    pub details: String,

    #[serde(rename = "aliases")]
    pub aliases: Vec<String>,

    #[serde(rename = "modified")]
    pub modified: String,

    #[serde(rename = "published")]
    pub published: String,

    #[serde(rename = "database_specific")]
    pub database_specific: Option<VulnDatabaseSpecific>,

    #[serde(rename = "references")]
    pub references: Vec<Reference>,

    #[serde(rename = "affected")]
    pub affected: Vec<Affected>,

    #[serde(rename = "schema_version")]
    pub schema_version: String,

    #[serde(rename = "severity")]
    pub severity: Option<Vec<Severity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affected {
    #[serde(rename = "package")]
    pub package: Package,

    #[serde(rename = "ranges")]
    pub ranges: Vec<Range>,

    #[serde(rename = "versions")]
    pub versions: Vec<String>,

    #[serde(rename = "database_specific")]
    pub database_specific: AffectedDatabaseSpecific,

    #[serde(rename = "ecosystem_specific")]
    pub ecosystem_specific: Option<EcosystemSpecific>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedDatabaseSpecific {
    #[serde(rename = "source")]
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSpecific {
    #[serde(rename = "affected_functions")]
    pub affected_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "ecosystem")]
    pub ecosystem: String,

    #[serde(rename = "purl")]
    pub purl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "type")]
    pub range_type: String,

    #[serde(rename = "events")]
    pub events: Vec<Event>,

    #[serde(rename = "repo")]
    pub repo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "introduced")]
    pub introduced: Option<String>,

    #[serde(rename = "fixed")]
    pub fixed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnDatabaseSpecific {
    #[serde(rename = "cwe_ids")]
    pub cwe_ids: Vec<String>,

    #[serde(rename = "github_reviewed")]
    pub github_reviewed: bool,

    #[serde(rename = "severity")]
    pub severity: String,

    #[serde(rename = "github_reviewed_at")]
    pub github_reviewed_at: String,

    #[serde(rename = "nvd_published_at")]
    pub nvd_published_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "type")]
    pub reference_type: String,

    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Severity {
    #[serde(rename = "type")]
    pub severity_type: String,

    #[serde(rename = "score")]
    pub score: String,
}

// /// A schema for describing a vulnerability in an open source package.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Vulnerability {
//     #[serde(rename = "vulns")]
//     pub vulns: Vec<Vulns>
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Vulns {

//     #[serde(rename = "affected")]
//     pub affected: Vec<Affected>,

//     #[serde(rename = "aliases")]
//     pub aliases: Vec<String>,

//     #[serde(rename = "credits")]
//     pub credits: Option<Vec<Credit>>,

//     #[serde(rename = "database_specific")]
//     pub database_specific: Option<HashMap<String, Option<serde_json::Value>>>,

//     #[serde(rename = "details")]
//     pub details: String,

//     #[serde(rename = "id")]
//     pub id: String,

//     #[serde(rename = "modified")]
//     pub modified: String,

//     #[serde(rename = "published")]
//     pub published: Option<String>,

//     #[serde(rename = "references")]
//     pub references: Option<Vec<Reference>>,

//     #[serde(rename = "related")]
//     pub related: Option<Vec<String>>,

//     #[serde(rename = "schema_version")]
//     pub schema_version: Option<String>,

//     #[serde(rename = "severity")]
//     pub severity: Option<Vec<VulnerabilitySeverity>>,

//     #[serde(rename = "summary")]
//     pub summary: String,

//     #[serde(rename = "withdrawn")]
//     pub withdrawn: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Affected {
//     #[serde(rename = "database_specific")]
//     pub database_specific: Option<HashMap<String, Option<serde_json::Value>>>,

//     #[serde(rename = "ecosystem_specific")]
//     pub ecosystem_specific: Option<HashMap<String, Option<serde_json::Value>>>,

//     #[serde(rename = "package")]
//     pub package: Option<Package>,

//     #[serde(rename = "ranges")]
//     pub ranges: Option<Vec<Range>>,

//     #[serde(rename = "severity")]
//     pub severity: Option<Vec<AffectedSeverity>>,

//     #[serde(rename = "versions")]
//     pub versions: Option<Vec<String>>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Package {
//     #[serde(rename = "ecosystem")]
//     pub ecosystem: Option<String>,

//     #[serde(rename = "name")]
//     pub name: Option<String>,

//     #[serde(rename = "purl")]
//     pub purl: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Range {
//     #[serde(rename = "database_specific")]
//     pub database_specific: Option<HashMap<String, Option<serde_json::Value>>>,

//     #[serde(rename = "events")]
//     pub events: Option<Vec<Event>>,

//     #[serde(rename = "repo")]
//     pub repo: Option<String>,

//     #[serde(rename = "type")]
//     pub range_type: Option<RangeType>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Event {
//     #[serde(rename = "introduced")]
//     pub introduced: Option<String>,

//     #[serde(rename = "fixed")]
//     pub fixed: Option<String>,

//     #[serde(rename = "last_affected")]
//     pub last_affected: Option<String>,

//     #[serde(rename = "limit")]
//     pub limit: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AffectedSeverity {
//     #[serde(rename = "score")]
//     pub score: Option<String>,

//     #[serde(rename = "type")]
//     pub severity_type: Option<SeverityType>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Credit {
//     #[serde(rename = "contact")]
//     pub contact: Option<Vec<String>>,

//     #[serde(rename = "name")]
//     pub name: Option<String>,

//     #[serde(rename = "type")]
//     pub credit_type: Option<CreditType>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Reference {
//     #[serde(rename = "type")]
//     pub reference_type: Option<ReferenceType>,

//     #[serde(rename = "url")]
//     pub url: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct VulnerabilitySeverity {
//     #[serde(rename = "score")]
//     pub score: Option<String>,

//     #[serde(rename = "type")]
//     pub severity_type: Option<SeverityType>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum RangeType {
//     #[serde(rename = "ECOSYSTEM")]
//     Ecosystem,

//     #[serde(rename = "GIT")]
//     Git,

//     #[serde(rename = "SEMVER")]
//     Semver,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum SeverityType {
//     #[serde(rename = "CVSS_V2")]
//     CvssV2,

//     #[serde(rename = "CVSS_V3")]
//     CvssV3,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum CreditType {
//     #[serde(rename = "ANALYST")]
//     Analyst,

//     #[serde(rename = "COORDINATOR")]
//     Coordinator,

//     #[serde(rename = "FINDER")]
//     Finder,

//     #[serde(rename = "OTHER")]
//     Other,

//     #[serde(rename = "REMEDIATION_DEVELOPER")]
//     RemediationDeveloper,

//     #[serde(rename = "REMEDIATION_REVIEWER")]
//     RemediationReviewer,

//     #[serde(rename = "REMEDIATION_VERIFIER")]
//     RemediationVerifier,

//     #[serde(rename = "REPORTER")]
//     Reporter,

//     #[serde(rename = "SPONSOR")]
//     Sponsor,

//     #[serde(rename = "TOOL")]
//     Tool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ReferenceType {
//     #[serde(rename = "ADVISORY")]
//     Advisory,

//     #[serde(rename = "ARTICLE")]
//     Article,

//     #[serde(rename = "DETECTION")]
//     Detection,

//     #[serde(rename = "DISCUSSION")]
//     Discussion,

//     #[serde(rename = "EVIDENCE")]
//     Evidence,

//     #[serde(rename = "FIX")]
//     Fix,

//     #[serde(rename = "GIT")]
//     Git,

//     #[serde(rename = "INTRODUCED")]
//     Introduced,

//     #[serde(rename = "PACKAGE")]
//     Package,

//     #[serde(rename = "REPORT")]
//     Report,

//     #[serde(rename = "WEB")]
//     Web,
// }


// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Pypi;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Pypi = serde_json::from_str(&json).unwrap();
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pypi {
    #[serde(rename = "packageKey")]
    pub package_key: Option<PackageKey>,

    #[serde(rename = "versions")]
    pub versions: Option<Vec<Version>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageKey {
    #[serde(rename = "system")]
    pub system: String,

    #[serde(rename = "name")]
    pub name: String
    // modify with the schema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "versionKey")]
    pub version_key: Option<VersionKey>,

    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionKey {
    #[serde(rename = "system")]
    pub system: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "version")]
    pub version: String
}

