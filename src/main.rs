use std::{path::PathBuf, process::exit};
use clap::{Parser, Subcommand};
use utils::PipCache;

use std::sync::OnceLock;
use once_cell::sync::Lazy;
use console::style;
mod utils;
mod parser;
mod scanner;
mod docker;
mod display;

use std::env;

use crate::{utils::get_version, parser::structs::{Dependency, VersionStatus}};

#[derive(Parser, Debug)]
#[command(author="aswinnnn",version="0.1.5",about="python dependency vulnerability scanner.")]
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
    /// ex. pyscan -s osv snyk
    /// hidden due to only having one database for now.
    #[arg(short, long, value_delimiter=' ', value_name="VAL1 VAL2 VAL3...", hide=true)]
    skip: Vec<String>,
    
    
    /// show the version and information about a package from all available sources. (does not search for vulns, use 'package' subcommand for that).
    /// usage: pyscan show requests pyscan-rs lxml koda
    /// hidden due to unfinished
    #[arg(long, value_delimiter=' ', value_name="package1 package2 package3...", hide=true)]
    show: Vec<String>,

    /// Uses pip to retrieve versions. if not provided it will use the source, falling back on pip if not, pypi.org.
    #[arg(long, required=false, action=clap::ArgAction::SetTrue)]
    pip: bool,

    /// Same as --pip except uses pypi.org to retrieve the latest version for the packages. 
    #[arg(long, required=false,action=clap::ArgAction::SetTrue)]
    pypi: bool,

    /// turns off the caching of pip packages at the starting of execution.
    #[arg(long="cache-off", required=false,action=clap::ArgAction::SetTrue)]
    cache_off: bool,
    
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

static ARGS: Lazy<OnceLock<Cli>> =  Lazy::new(|| {OnceLock::from(Cli::parse())});

// Why is the args a static variable? Some arguments need to be seen by other files in the codebase
// such as --pip or --pypi due to different use cases. Args only get wrote to once so it shouldn't pose a problem (Reason its OnceLock'ed).
// Why is it Lazy? Something about a non-const fn in a const world. Pretty surprised to see the compiler recommend an outside crate for this issue.

static PIPCACHE: Lazy<PipCache> = Lazy::new(|| {utils::PipCache::init()});
// is a hashmap of package name, version from 'pip list'
// because calling 'pip show' everytime might get expensive if theres a lot of dependencies to check. 


#[tokio::main]
async fn main() {
    
    println!("pyscan v{} | by Aswin S (github.com/aswinnnn)", get_version());  

    // init pip cache, if cache-off is false
    if !&ARGS.get().unwrap().cache_off {
        let _ = PIPCACHE.lookup("something");
    }
    // since its in Lazy its first accesss would init the cache, the result is ignorable.

    match &ARGS.get().unwrap().subcommand {
        // subcommand package

        Some(SubCommand::Package { name, version }) => {
            // let osv = Osv::new().expect("Cannot access the API to get the latest package version.");
            let version = if let Some(v) = version {v.to_string()} else {utils::get_package_version_pypi(name.as_str()).expect("Error in retrieving stable version from API").to_string()};

            let dep = Dependency {name: name.to_string(), version: Some(version), comparator: None, version_status: VersionStatus {pypi: false, pip: false, source: false}};
            
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
            docker::list_files_in_docker_image(name, path.to_path_buf()).await
            .expect("Error in scanning files from Docker image.");
            exit(0)
        }
        None => ()
    }

  
    // println!("{:?}", args);

    // --- giving control to parser starts here ---

    // if a directory path is provided
    if let Some(dir) = &ARGS.get().unwrap().dir { parser::scan_dir(dir.as_path()).await } 

    // if not, use cwd
    else if let Ok(dir) = env::current_dir() { parser::scan_dir(dir.as_path()).await } 
    else {eprintln!("the given directory is empty."); exit(1)}; // err when dir is empty

}
