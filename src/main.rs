use std::{path::PathBuf, process::exit};
use clap::{Parser, Subcommand};
mod utils;
mod parser;
mod scanner;

use std::env;

use crate::{utils::get_version, parser::structs::Dependency, scanner::api::Osv};

#[derive(Parser, Debug)]
#[command(author="aswinnnn",version="0.1.0",about="python dependency vulnerability scanner.")]
struct Cli {

    /// path to source. if not provided it will use the current directory.
    #[arg(long,short,default_value=None,value_name="DIRECTORY")]
    dir: Option<PathBuf>,

    /// search for a single package, do "pyscan package --help" for more
    #[command(subcommand)]
    package: Option<SubCommand>,

    /// skip: skip the given databases
    /// ex. pyscan -s osv,snyk
    /// hidden due to only having one database for now.
    #[arg(short, long, value_delimiter=',', value_name="VAL1,VAL2,VAL3...", hide=true)]
    skip: Vec<String>

}

#[derive(Subcommand, Debug, Clone)]
enum SubCommand {
    /// query for a single python package
    Package {
        /// name of the package
        #[arg(long,short)]
        name: String,

        /// version of the package (if not provided, the latest stable will be used)
        #[arg(long, short, default_value=None)]
        version: Option<String>
    }
}

fn main() {
    let args = Cli::parse();

    match args.package {
        // subcommand package

        Some(SubCommand::Package { name, version }) => {
            let osv = Osv::new().expect("Cannot access the API to get the latest package version.");
            let version = if let Some(v) = version {v} else {osv.get_latest_package_version(name.clone())
            .expect("Error in retriving stable version from API")};

            let dep = Dependency {name: name, version: Some(version), comparator: None};
            // start() from scanner only accepts Vec<Dependency> so
            let vdep = vec![dep];

            let _res = scanner::start(vdep);
            exit(0)

        },
        None => ()
    }

    println!("pyscan v{} | by Aswin (github.com/aswinnnn)", get_version());    
    // println!("{:?}", args);

    // --- giving control to parser starts here ---

    // if a directory path is provided
    if let Some(dir) = args.dir { parser::scan_dir(dir.as_path()) } 

    // if not, use cwd
    else if let Ok(dir) = env::current_dir() { parser::scan_dir(dir.as_path()) } 
    else {eprintln!("the given directory is empty.")}; // err when dir is empty



}