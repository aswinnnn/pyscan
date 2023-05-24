use std::collections::HashMap;
use regex::Regex;

pub fn parse_python_code(code: &str) -> HashMap<String, String> {
    // Create an empty HashMap to store the function name and code
    let mut functions = HashMap::new();

    // Create a regular expression to match the function definition line
    let re = Regex::new(r"^def\s+(\w+)\s*\((.*)\)\s*:\s*$").unwrap();

    // Iterate over the lines of the code
    for line in code.lines() {
        // Check if the line matches the regular expression
        if let Some(caps) = re.captures(line) {
            // Get the function name and parameters from the capture groups
            let name = caps.get(1).unwrap().as_str().to_string();
            let params = caps.get(2).unwrap().as_str().to_string();

            // Create an empty string to store the function body
            let mut body = String::new();

            // Set a flag to indicate that we are inside a function
            let mut in_function = true;

            // Set an indentation level to track the indentation of the function body
            let mut indent = 0;

            // Iterate over the remaining lines of the code
            for line in code.lines().skip_while(|l| l != &line).skip(1) {
                // Check if we are still inside a function
                if in_function {
                    // Get the leading whitespace of the line
                    let whitespace = line.chars().take_while(|c| c.is_whitespace()).count();

                    // Check if this is the first line of the function body
                    if indent == 0 {
                        // Set the indentation level to the whitespace count
                        indent = whitespace;
                    }

                    // Check if the line has less indentation than the function body
                    if whitespace < indent {
                        // We have reached the end of the function
                        in_function = false;
                        break;
                    }

                    // Append the line to the function body
                    body.push_str(line);
                    body.push('\n');
                }
            }

            // Insert the function name and code into the HashMap
            functions.insert(name.clone(), format!("def {}({}):\n{}", name, params, body));
        }
    }

    // Return the HashMap
    functions
}