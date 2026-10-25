use std::path::PathBuf;
use crate::args::Args;

pub enum TestArgs {
    Default,
    NoInfo
}

pub fn get_args(args: TestArgs) -> Args {
    match args {
        TestArgs::Default => Args {
            paths: Some(vec![PathBuf::from(".")]),
            patterns: Vec::new(),
            all: true,
            any: false,
            no_info: false,
            fixed_strings: false,
            ignore_case: false,
            exclude: vec![],
            exclude_regex: false,
            include: vec![],
            include_regex: false,
            recursive: false,
            depth: None,
        },

        TestArgs::NoInfo => Args {
            no_info: true,
            ..get_args(TestArgs::Default).clone()
        }
    }
}