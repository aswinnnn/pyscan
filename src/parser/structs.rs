use std::{ffi::OsString};
use super::scanner::models::Vulnerability;

// struct Python;
// struct Requirements;
// struct Pyproject;

#[derive(Debug, PartialEq, Clone)]
pub enum FileTypes {
    Python,
    Requirements,
    Pyproject
}

#[derive(Debug, Clone)]
pub struct FoundFile {
    pub name: OsString,
    pub filetype: FileTypes,
    pub path: OsString
}

impl FoundFile {
    pub fn is_python(&self) -> bool {
        self.filetype == FileTypes::Python
    }
    pub fn is_reqs(&self) -> bool {
        self.filetype == FileTypes::Requirements
    }
    pub fn is_pyproject(&self) -> bool {
        self.filetype == FileTypes::Pyproject
    }
}

#[derive(Debug, Clone)]
pub struct FoundFileResult {
    /// provides overall info about the files found (useful for proritising filetypes)
    pub files: Vec<FoundFile>,
    pub py_found: u64, // no. of said files found
    pub reqs_found: u64,
    pub pyproject_found: u64
}

impl FoundFileResult {
    pub fn new() -> FoundFileResult {
        FoundFileResult {
            files: Vec::new(),
            py_found: 0,
            reqs_found: 0,
            pyproject_found: 0
        }
    }
    pub fn add(&mut self, f: FoundFile) {
        self.files.push(f)
    }
    pub fn python(&mut self) {
        self.py_found += 1
    }
    pub fn reqs(&mut self) {
        self.reqs_found += 1
    }
    pub fn pyproject(&mut self) {
        self.pyproject_found += 1
    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub comparator: Option<pep_508::Comparator>
}

#[derive(Debug, Clone)]
pub struct ScannedDependency {
    pub name: String,
    pub version: String,
    pub vuln: Vulnerability
}