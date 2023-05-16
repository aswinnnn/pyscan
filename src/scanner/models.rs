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

// pypi API models (well not really but its for finding the latest stable version of a package.)

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

