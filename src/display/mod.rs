use std::collections::HashMap;
use crate::parser::structs::ScannedDependency;
use console::{Term, style};
use once_cell::sync::Lazy;

static CONS: Lazy<Term> = Lazy::new(|| {Term::stdout()}) ;

pub struct Progress {
    // this progress info only contains progress info about the found vulns.
    count: usize,
    current_displayed: usize
}

impl Progress {
    pub fn new() -> Progress {
        Progress {
            count: 0,
            current_displayed: 0
        }
    }
    pub fn display(&mut self) {
        if self.count > 1 {let _ = CONS.clear_last_lines(1);}

        if self.count > self.current_displayed {
            let _ = CONS.write_line(
                format!("Found {} vulnerabilities so far", style(self.count).bold().bright().red()).as_str()
            );
            self.current_displayed = self.count;
        } 
    }

    pub fn count_one(&mut self) {
        self.count += 1;
    }
}