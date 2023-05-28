use std::path::{Path, PathBuf};
use std::fs; 
use std::io::Read;
mod parser;
mod mccabe;
use console::style;

pub fn start(path: &PathBuf) {
    println!("--> {}", style(path.clone().to_string_lossy()).bold().underlined());

    let source_code = fs::read_to_string(path).expect("Error reading file");
    // return a HashMap<function_name, function>
    let parsed = parser::parse_python_code(source_code.as_str());

    for (funname, function) in parsed.iter() {
        let res = mccabe::check_complexity(function.as_str()).expect("Error in checking compexity of a function.");
        println!("{} : {}", funname, res);

    }
}