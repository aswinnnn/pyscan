use std::path::PathBuf;
use clap::Parser;
mod utils;
mod parser;
mod scanner;
use clap::ArgAction;
use std::env;

use crate::utils::get_version;

#[derive(Parser, Debug)]
#[command(author="aswinnnn",version="0.1.0",about="python dependency vulnerability scanner.")]
struct Cli {

    /// if not provided it will search for files in the current directory.
    #[arg(long,short,default_value=None,value_name="FILE")]
    file: Option<PathBuf>,

    /// scan subdirectories for python files.
    /// [off by default]
    #[arg(long, short, action=ArgAction::SetTrue)]
    recursive: bool,

    /// skip: skip the given sites
    /// ex. pyscan -s osv,snyk
    #[arg(short, long, value_delimiter=',', value_name="VAL1,VAL2,VAL3...")]
    skip: Vec<String>

}


fn main() {
    let args = Cli::parse();

    println!("pyscan v{} | by Aswin (github.com/aswinnnn)", get_version());

    // println!("{:?}", args);

    // --- giving control to parser starts here ---

    // if a file  is provided
    if let Some(dir) = args.file { parser::scan_dir(dir.as_path()) } 

    // if not, use cwd
    else if let Ok(dir) = env::current_dir() { parser::scan_dir(dir.as_path()) } 
    else {eprintln!("the given directory is empty.")}; // err when dir is empty

}