use std::io::{BufReader, BufRead};
use std::{ffi::OsString, fs::File};
/// ^\s*(?:from|import)\s+(\w+(?:\s*,\s*\w+)*)
/// regex used to match imports from python filesr
use std::fs;
use std::path::Path;
pub mod structs;
mod extractor;
use super::scanner;
use structs::{FoundFile, FileTypes, FoundFileResult};


pub fn scan_dir(dir: &Path) {
    let mut result = FoundFileResult::new(); // contains found files

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            
            if let Ok(entry) = entry {
                let filename = entry.file_name();

                // check if .py
                // about the slice: [(file length) - 3..] for the extention
                if ".py" == &filename.to_str().unwrap()[{filename.to_str().unwrap().len() - 3}..] {
                    result.add(FoundFile {
                        name: filename,
                        filetype: FileTypes::Python,
                        path: OsString::from(entry.path())
                    });
                    result.python();
                }
                // requirements.txt
                else if *"requirements.txt" == filename.clone() {
                    result.add(FoundFile {
                        name: filename,
                        filetype: FileTypes::Requirements,
                        path: OsString::from(entry.path())
                    });
                    result.reqs();
                }
                // pyproject.toml
                else if *"pyproject.toml" == filename.clone() {
                    result.add(FoundFile {
                        name: filename,
                        filetype: FileTypes::Pyproject,
                        path: OsString::from(entry.path())
                    });
                    result.pyproject();
                } 
            }
        }
    }
    // println!("{:?}", result.clone());

    // --- find_import takes the result ---

    find_import(result)


}

/// A nice abstraction over different ways to find imports for different filetypes.
fn find_import(res: FoundFileResult) {
    let files = res.files;
    if res.reqs_found > res.pyproject_found {
        // if theres a requirements.txt and pyproject.toml isnt there
        find_reqs_imports(&files)
    }
    else if res.reqs_found != 0 {
        // if both reqs and pyproject is present, go for reqs first
        find_reqs_imports(&files)
    }
    else if res.pyproject_found != 0 {
        // use pyproject instead (if it exists)
        find_pyproject_imports(&files)
    }
    else if res.py_found != 0 {
        // make sure theres atleast one python file, then use that
        find_python_imports(&files)
    }
    else {
        eprintln!("Could not find any requirements.txt, pyproject.toml or python files in this directory");
    }
}

fn find_python_imports(f: &Vec<FoundFile>) {
    let cons = console::Term::stdout();
    cons.write_line("Using python file as source...").unwrap();

    let mut imports = Vec::new(); // contains the Dependencies
    for file in f {
        if file.is_python() {
            
            if let Ok(fhandle) = File::open(file.path.clone()) {

                let reader = BufReader::new(fhandle);
    
                for line in reader.lines() {
    
                    if let Ok(l) = line {
                        cons.clear_last_lines(1).unwrap();
                        extractor::extract_imports_python(l, &mut imports);
    
                    }
                }
            }
        } 
    }
    // println!("{:?}", imports.clone());
    cons.clear_last_lines(1).unwrap();
    // --- pass the dependencies to the scanner/api ---
    scanner::start(imports).unwrap(); // unwrapping is ok since the return value doesnt matter.




}
fn find_reqs_imports(f: &Vec<FoundFile>) {
    let cons = console::Term::stdout();
    cons.write_line("Using requirements.txt as source...").unwrap();

    let mut imports = Vec::new();
    for file  in f {
        if file.is_reqs() {
            if let Ok(fhandle) = File::open(file.path.clone()) {

                let reader = BufReader::new(fhandle);
    
                for line in reader.lines() {
    
                    if let Ok(l) = line {
                        extractor::extract_imports_reqs(l.trim().to_string(), &mut imports)
    
                    }
                }
            }
        }
    }
    // println!("{:?}", imports.clone());
    cons.clear_last_lines(1).unwrap();
    // --- pass the dependencies to the scanner/api ---
    scanner::start(imports).unwrap();
}


fn find_pyproject_imports(f: &Vec<FoundFile>) {
    let cons = console::Term::stdout();
    cons.write_line("Using requirements.txt as source...").unwrap();

    let mut imports = Vec::new();
    for file  in f {
        if file.is_pyproject() {
            let readf = fs::read_to_string(file.path.clone());
            if let Ok(f) = readf {
                
                extractor::extract_imports_pyproject(f, &mut imports)
            }
            else {
                eprintln!("There was a problem reading your pyproject.toml")
            }
        }
    }
    // println!("{:?}", imports.clone());
    cons.clear_last_lines(1).unwrap();
    // --- pass the dependencies to the scanner/api ---
    scanner::start(imports).unwrap();
}