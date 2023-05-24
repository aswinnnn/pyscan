// A Rust function to check the McCabe cyclomatic complexity of Python source code
// This function takes a string of Python code as input and returns an integer as output
// The function uses a regex crate to parse the code and count the number of edges and nodes in the control flow graph
// The function uses the formula M = E - N + 2 to calculate the complexity
// The function assumes that the input code is syntactically valid and has one entry point and one exit point
// The function also handles errors that may occur during regex parsing or calculation

use regex::Regex;
use std::error::Error;

pub fn check_complexity(code: &str) -> Result<i32, Box<dyn Error>> {
    // Define the regex patterns for different control flow statements in Python
    let if_pattern = Regex::new(r"if\s+.*:")?;
    let elif_pattern = Regex::new(r"elif\s+.*:")?;
    let else_pattern = Regex::new(r"else\s*:")?;
    let for_pattern = Regex::new(r"for\s+.*:")?;
    let while_pattern = Regex::new(r"while\s+.*:")?;
    let try_pattern = Regex::new(r"try\s*:")?;
    let except_pattern = Regex::new(r"except\s+.*:")?;
    let finally_pattern = Regex::new(r"finally\s*:")?;

    // Initialize the number of edges and nodes to zero
    let mut edges = 0;
    let mut nodes = 0;

    // Loop through each line of the code and increment the edges and nodes accordingly
    for line in code.lines() {
        // If the line matches any of the control flow patterns, increment the edges by one
        // and the nodes by two (one for the condition and one for the body)
        if if_pattern.is_match(line) || elif_pattern.is_match(line) || else_pattern.is_match(line) ||
           for_pattern.is_match(line) || while_pattern.is_match(line) || try_pattern.is_match(line) ||
           except_pattern.is_match(line) || finally_pattern.is_match(line) {
               edges += 1;
               nodes += 2;
           }
        // Otherwise, increment the nodes by one (for the statement)
        else {
            nodes += 1;
        }
    }

    // Add one edge and one node for the entry point and one edge and one node for the exit point
    edges += 2;
    nodes += 2;

    // Calculate and return the complexity using the formula M = E - N + 2
    let complexity = edges - nodes + 2;
    Ok(complexity)
}