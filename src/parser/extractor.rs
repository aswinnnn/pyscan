/// for the parser module, extractor.rs is the backbone of all parsing
/// it takes a String and a mutable reference to a Vec<Dependency>.
/// String is the contents of a source file, while the mut ref vector will
/// be used to collect the dependencies that we have extracted from the contents.

use lazy_static::lazy_static;
use regex::Regex;
use pep_508::{self, Spec};
use super::structs::{Dependency, VersionStatus};
use toml::Table;

pub fn extract_imports_python(text: String, imp: &mut Vec<Dependency>) {
    lazy_static! {
        static ref IMPORT_REGEX : Regex = Regex::new(
                r"^\s*(?:from|import)\s+(\w+(?:\s*,\s*\w+)*)"
            ).unwrap();
    }

    for x in IMPORT_REGEX.find_iter(&text) {
        let mat = x.as_str().to_string();
        let mat = mat.replacen("import", "", 1).trim().to_string();

        imp.push(Dependency { name: mat, version: None, comparator: None, version_status: VersionStatus {pypi: false, pip: false, source: false} })

    }
}

pub fn extract_imports_reqs(text: String, imp: &mut Vec<Dependency>) {
    // requirements.txt uses a PEP 508 parser to parse dependencies accordingly
    // you might think its just a text file, but I'm gonna decline reinventing the wheel
    // just to parse "requests >= 2.0.8"
    
    let parsed = pep_508::parse(text.as_str());

    if  let Ok(dep) = parsed {
        let dname = dep.name.to_string();
        // println!("{:?}", dep.clone());
        if let Some(ver) = dep.spec {
            if let Spec::Version(verspec) = ver {
                for v in verspec {
                    // pyscan only takes the first version spec found for the dependency
                    // for now.
                    let version = v.version.to_string();
                    let comparator = v.comparator;
                    imp.push(Dependency{name: dname, version: Some(version), comparator: Some(comparator), version_status: VersionStatus {pypi: false, pip: false, source: true}});
                    break;
                }
            }
        }
        else {
            imp.push(Dependency{name: dname, version: None, comparator: None, version_status: VersionStatus {pypi: false, pip: false, source: false}});
        }
    }

    
}

pub fn extract_imports_pyproject(f: String, imp: &mut Vec<Dependency>) {
    let parsed = f.parse::<Table>();
    if let Ok(parsed) = parsed {
        let project = &parsed["project"];
        let deps = &project["dependencies"];
        let deps = deps.as_array()
        .expect("Could not find the dependencies table in your pyproject.toml");
        for d in deps {
            let d = d.as_str().unwrap().to_string();
            imp.push(Dependency { name: d, version: None, comparator: None, version_status: VersionStatus {pypi: false, pip: false, source: false} })

        }
    }
}