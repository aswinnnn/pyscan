use std::{path::PathBuf, process::exit};
use clap::{Parser, Subcommand};
use console::style;
mod utils;
mod parser;
mod scanner;
mod docker;

use std::env;

use crate::{utils::get_version, parser::structs::Dependency};

#[derive(Parser, Debug)]
#[command(author="aswinnnn",version="0.1.1",about="python dependency vulnerability scanner.")]
struct Cli {

    /// path to source. if not provided it will use the current directory.
    #[arg(long,short,default_value=None,value_name="DIRECTORY")]
    dir: Option<PathBuf>,

    /// search for a single package, do "pyscan package --help" for more
    #[command(subcommand)]
    subcommand: Option<SubCommand>,
    
    // /// scan a docker image, do "pyscan docker --help" for more
    // #[command(subcommand)]
    // docker: Option<SubCommand>,

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
    },

    /// scan a docker image
    Docker {

        /// name of the docker image
        #[arg(long,short)]
        name: String,

        /// path inside your docker container where requirements.txt is, or just the folder name where your Dockerfile (along with requirements.txt) is.
        #[arg(long,short,value_name="DIRECTORY")]
        path: PathBuf,
        

    }
}

fn main() {
    let args = Cli::parse();
    
    println!("pyscan v{} | by Aswin (github.com/aswinnnn)", get_version());  

    match args.subcommand {
        // subcommand package

        Some(SubCommand::Package { name, version }) => {
            // let osv = Osv::new().expect("Cannot access the API to get the latest package version.");
            let version = if let Some(v) = version {v} else {utils::get_latest_package_version(name.clone())
            .expect("Error in retriving stable version from API")};

            let dep = Dependency {name, version: Some(version), comparator: None};
            // start() from scanner only accepts Vec<Dependency> so
            let vdep = vec![dep];

            let _res = scanner::start(vdep);
            exit(0)

        },
        Some(SubCommand::Docker { name, path}) => {
            println!("{} {}\n{} {}",style("Docker image:").yellow().blink(),
            style(name.clone()).bold().green(),
            style("Path inside container:").yellow().blink(), 
            style(path.to_string_lossy()).bold().green());
            println!("{}", 
        style("--- Make sure you run the command with elevated permissions (sudo/administrator) as pyscan might have trouble accessing files inside docker containers ---").dim());
            docker::list_files_in_docker_image(&name, path)
            .expect("Error in scanning files from Docker image.");
            exit(0)
        }
        None => ()
    }

  
    // println!("{:?}", args);

    // --- giving control to parser starts here ---

    // if a directory path is provided
    if let Some(dir) = args.dir { parser::scan_dir(dir.as_path()) } 

    // if not, use cwd
    else if let Ok(dir) = env::current_dir() { parser::scan_dir(dir.as_path()) } 
    else {eprintln!("the given directory is empty.")}; // err when dir is empty



}