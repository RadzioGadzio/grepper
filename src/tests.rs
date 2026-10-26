use std::path::PathBuf;
use regex::Regex;
use crate::args::Args;
use crate::matcher::MatchMode;

#[allow(dead_code)]
pub enum TestArgs {
    Default
}

#[allow(dead_code)]
pub fn get_regex(regex: &str) -> Regex {
    Regex::new(regex).expect("Invalid regex")
}

#[allow(dead_code)]
pub fn get_args(args: TestArgs) -> Args {
    match args {
        TestArgs::Default => Args {
            paths: vec![PathBuf::from(".")],
            patterns: Vec::new(),
            mode: MatchMode::All,
            fixed_strings: false,
            ignore_case: false,
            exclude: vec![],
            exclude_regex: false,
            include: vec![],
            include_regex: false,
            recursive: false,
            depth: None,
        }
    }
}