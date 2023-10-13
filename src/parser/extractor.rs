use std::process::exit;

/// for the parser module, extractor.rs is the backbone of all parsing
/// it takes a String and a mutable reference to a Vec<Dependency>.
/// String is the contents of a source file, while the mut ref vector will
/// be used to collect the dependencies that we have extracted from the contents.
use super::structs::{Dependency, VersionStatus};

use lazy_static::lazy_static;
use pep_508::{self, Spec};
use regex::Regex;



use toml::{de::Error, Value};

pub fn extract_imports_python(text: String, imp: &mut Vec<Dependency>) {
    lazy_static! {
        static ref IMPORT_REGEX: Regex =
            Regex::new(r"^\s*(?:from|import)\s+(\w+(?:\s*,\s*\w+)*)").unwrap();
    }

    for x in IMPORT_REGEX.find_iter(&text) {
        let mat = x.as_str().to_string();
        let mat = mat.replacen("import", "", 1).trim().to_string();

        imp.push(Dependency {
            name: mat,
            version: None,
            comparator: None,
            version_status: VersionStatus {
                pypi: false,
                pip: false,
                source: false,
            },
        })
    }
}

pub fn extract_imports_reqs(text: String, imp: &mut Vec<Dependency>) {
    // requirements.txt uses a PEP 508 parser to parse dependencies accordingly
    // you might think its just a text file, but I'm gonna decline reinventing the wheel
    // just to parse "requests >= 2.0.8"

    let parsed = pep_508::parse(text.as_str());

    if let Ok(ref dep) = parsed {
        let dname = dep.name.to_string();
        // println!("{:?}", parsed.clone());
        if let Some(ver) = &dep.spec {
            if let Spec::Version(verspec) = ver {
                if let Some(v) = verspec.iter().next() {
                    // pyscan only takes the first version spec found for the dependency
                    let version = v.version.to_string();
                    let comparator = v.comparator;
                    imp.push(Dependency {
                        name: dname,
                        version: Some(version),
                        comparator: Some(comparator),
                        version_status: VersionStatus {
                            pypi: false,
                            pip: false,
                            source: true,
                        },
                    });
                }
            }
        } else {
            imp.push(Dependency {
                name: dname,
                version: None,
                comparator: None,
                version_status: VersionStatus {
                    pypi: false,
                    pip: false,
                    source: false,
                },
            });
        }
    } else if let Err(e) = parsed {
        println!("{:#?}", e);
    }
}

// pub fn extract_imports_pyproject(f: String, imp: &mut Vec<Dependency>) {
//     let parsed = f.parse::<Table>();
//     if let Ok(parsed) = parsed {
//         let project = &parsed["project"];
//         let deps = &project["dependencies"];
//         let deps = deps
//             .as_array()
//             .expect("Could not find the dependencies table in your pyproject.toml");
//         for d in deps {
//             let d = d.as_str().unwrap();
//             let parsed = pep_508::parse(d);
//             if let Ok(dep) = parsed {
//                 let dname = dep.name.to_string();
//                 // println!("{:?}", dep.clone());
//                 if let Some(ver) = dep.spec {
//                     if let Spec::Version(verspec) = ver {
//                         for v in verspec {
//                             // pyscan only takes the first version spec found for the dependency
//                             // for now.
//                             let version = v.version.to_string();
//                             let comparator = v.comparator;
//                             imp.push(Dependency {
//                                 name: dname,
//                                 version: Some(version),
//                                 comparator: Some(comparator),
//                                 version_status: VersionStatus {
//                                     pypi: false,
//                                     pip: false,
//                                     source: true,
//                                 },
//                             });
//                             break;
//                         }
//                     }
//                 } else {
//                     imp.push(Dependency {
//                         name: dname,
//                         version: None,
//                         comparator: None,
//                         version_status: VersionStatus {
//                             pypi: false,
//                             pip: false,
//                             source: false,
//                         },
//                     });
//                 }
//             }
//         }
//     }
// }

pub fn extract_imports_setup_py(setup_py_content: &str, imp: &mut Vec<Dependency>) {
    let mut deps = Vec::new();

    // regex for install_requires section
    let re = Regex::new(r"install_requires\s*=\s*\[([^\]]+)\]").expect("Invalid regex pattern");

    for cap in re.captures_iter(setup_py_content) {
        if let Some(matched) = cap.get(1) {
            // Split the matched text by ',' and trim whitespace
            deps.extend(
                matched
                    .as_str()
                    .split(',')
                    .map(|dep| dep.trim().replace("\"", "").replace("\\", "").to_string()),
            );
        }
    }

    for d in deps {
        let d = d.as_str();
        let parsed = pep_508::parse(d);
        if let Ok(dep) = parsed {
            let dname = dep.name.to_string();
            if let Some(ver) = dep.spec {
                if let Spec::Version(verspec) = ver {
                    if let Some(v) = verspec.first() {
                        // pyscan only takes the first version spec found for the dependency
                        // for now.
                        let version = v.version.to_string();
                        let comparator = v.comparator;
                        imp.push(Dependency {
                            name: dname,
                            version: Some(version),
                            comparator: Some(comparator),
                            version_status: VersionStatus {
                                pypi: false,
                                pip: false,
                                source: true,
                            },
                        });
                    }
                }
            } else {
                imp.push(Dependency {
                    name: dname,
                    version: None,
                    comparator: None,
                    version_status: VersionStatus {
                        pypi: false,
                        pip: false,
                        source: false,
                    },
                });
            }
        }
    }
}

pub fn extract_imports_pyproject(
    toml_content: String,
    imp: &mut Vec<Dependency>,
) -> Result<(), Error> {
    // Parse the toml content into a Value
    let toml_value: Value = toml::from_str(toml_content.as_str())?;
    // println!("{:#?}",toml_value);

    // Helper function to extract dependency values (version strings) including nested tables
    fn extract_dependencies(table: &toml::value::Table, poetry: Option<bool>) -> Result<Vec<String>, Error> {
        let mut deps = Vec::new();

        // for [project] in pyproject.toml, the insides require a different sort of parsing
        // for poetry you need both keys and values (as dependency name and version), 
        // for [project] the values are just enough and the keys are in the vec below
        let projectlevel: Vec<&str> = vec!["dependencies", "optional-dependencies.docs"];
        
        for (key, version) in table {
            if projectlevel.contains(&key.as_str()) {
                match version {
                    Value::String(version_str) => {
                        deps.push(version_str.to_string());
                    }
                    Value::Table(nested_table) => {
                        // Recursively extract dependencies from nested tables
                        let nested_deps = extract_dependencies(nested_table,None)?;
                        deps.extend(nested_deps);
                    }
                    Value::Array(array) => {
                        // Extract dependencies from an array (if any)
                        for item in array {
                            if let Value::String(item_str) = item {
                                deps.push(item_str.to_string());
                            }
                        }
                    }
                    _ => eprintln!("ERR: Invalid dependency syntax found while TOML parsing"),
                }
            }
            else if poetry.unwrap_or(false) {
                    match version {
                        Value::String(version_str) => {
                            let verstr = version_str.to_string();
                            if verstr.contains('^') {
                                let s = format!("{} >= {}", key, verstr.strip_prefix('^').unwrap());
                                deps.push(s);
                            }
                            else if verstr == "*" {
                            deps.push(key.to_string());
                            }
                        }
                        Value::Table(nested_table) => {
                            // Recursively extract dependencies from nested tables
                            let nested_deps = extract_dependencies(nested_table,None)?;
                            deps.extend(nested_deps);
                        }
                        Value::Array(array) => {
                            // Extract dependencies from an array (if any)
                            for item in array {
                                if let Value::String(item_str) = item {
                                    deps.push(item_str.to_string());
                                }
                            }
                        }
                        _ => eprintln!("ERR: Invalid dependency syntax found while TOML parsing"),
                    }
            }
        }
        Ok(deps)
    }

    // Extract dependencies from different sections
    let mut all_dependencies = Vec::new();

    // Look for keys like "dependencies" and "optional-dependencies"
    let keys_to_check = vec!["project", "optional-dependencies", "tool"];

    for key in keys_to_check {
        if key.contains("tool") {
            
            if let Some(dependencies_table) = toml_value.get("tool") {
                if let Some(dependencies_table) = dependencies_table.get("poetry") {
                    let poetrylevel: Vec<&str> = vec!["dependencies", "dev-dependencies"];
                    for k in poetrylevel.into_iter() {
                        if let Some(dep) = dependencies_table.get(k) {
                            match dep {
                                Value::Table(table) => {
                                    all_dependencies.extend(extract_dependencies(table, Some(true))?);                            
                                }
                                // its definitely gonna be a table anyway, so...
                                Value::String(_) => todo!(),
                                Value::Integer(_) => todo!(),
                                Value::Float(_) => todo!(),
                                Value::Boolean(_) => todo!(),
                                Value::Datetime(_) => todo!(),
                                Value::Array(_) => todo!(),
                            }
                        }
                    }
                }
            }
        }

        // if its not poetry, check for [project] dependencies
        else if !key.contains("poetry") {

            if let Some(dependencies_table) = toml_value.get(key) {
                if let Some(dependencies) = dependencies_table.as_table() {
                        all_dependencies.extend(extract_dependencies(dependencies, None)?);
                }
            }
        }
        else {
            eprintln!("The pyproject.toml seen here is unlike of a python project. Please check and make
            sure you are in the right directory, or check the toml file."); exit(1)
        }
    }
    // the toml might contain repeated dependencies 
    // for different tools, dev tests, etc.
    all_dependencies.dedup(); 

    for d in all_dependencies {
        let d = d.as_str();
        let parsed = pep_508::parse(d);
        if let Ok(dep) = parsed {
            let dname = dep.name.to_string();
            if let Some(ver) = dep.spec {
                if let Spec::Version(verspec) = ver {
                    if let Some(v) = verspec.into_iter().next() {
                        let version = v.version.to_string();
                        let comparator = v.comparator;
                        imp.push(Dependency {
                            name: dname.clone(),
                            version: Some(version),
                            comparator: Some(comparator),
                            version_status: VersionStatus {
                                pypi: false,
                                pip: false,
                                source: true,
                            },
                        });
                    }
                }
            } else {
                imp.push(Dependency {
                    name: dname.clone(),
                    version: None,
                    comparator: None,
                    version_status: VersionStatus {
                        pypi: false,
                        pip: false,
                        source: false,
                    },
                });
            }
        }
    }
    Ok(())
}
