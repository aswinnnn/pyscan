use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REQUIRES_REGEX: Regex = Regex::new(r"Requires: (.*)").unwrap();
}

fn extract_dependencies(pip_show_output: &str) -> Vec<String> {
    if let Some(captures) = REQUIRES_REGEX.captures(pip_show_output) {
        if let Some(deps) = captures.get(1) {
            // Split the dependencies by a comma and trim whitespace
            let deps_str = deps.as_str();
            return deps_str.split(',').map(|s| s.trim().to_string()).collect();
        }
    }

    Vec::new() // Return an empty vector if "Requires:" line is not found or empty
}
