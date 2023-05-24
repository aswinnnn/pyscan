use std::path::Path;
use std::fs; 
use std::io::Read;
mod parser;
mod mccabe;

pub fn start(path: &Path) {

    let source_code = open_python_file(path).expect("Error opening python file");
    // return a HashMap<function_name, function>
    let parsed = parser::parse_python_code(source_code.as_str());

    for (funname, function) in parsed.iter() {
        let res = mccabe::check_complexity(function.as_str()).expect("Error in checking compexity of a function.");
        println!("{} : {}", funname, res);

    }
}


fn open_python_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    // use the File::open method to try to open the file in read-only mode
    // if the file does not exist or cannot be opened, return an error
    let mut file = fs::File::open(path)?;

    // create a mutable String variable to store the file contents
    let mut contents = String::new();

    // use the read_to_string method to read the file contents into the String variable
    // if the file cannot be read, return an error
    file.read_to_string(&mut contents)?;

    // return Ok with a reference to the String variable as a &str value
    Ok(contents)
}