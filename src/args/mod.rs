mod parser;
mod pattern;

use std::path::{PathBuf};
use clap::Parser;
use colored::Color;
use regex::Regex;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Args {
    #[arg(help = "Files or directories to scan")]
    pub paths: Option<Vec<PathBuf>>,

    #[arg(long = "pattern", short = 'p', help = "Patterns to search for. See `fixed-strings`")]
    pub patterns: Vec<Regex>,

    #[arg(long, help = "Match all patterns", default_value = "false")]
    pub all: bool,

    #[arg(long, help = "Match any pattern", default_value = "false")]
    pub any: bool,

    #[arg(long, help = "Don't print line info", default_value = "false")]
    pub no_info: bool,

    #[arg(long, short = 'F', help = "Interpret PATTERNS as fixed strings, not regular expressions", default_value = "false")]
    pub fixed_strings: bool,

    #[arg(long, short = 'i', help = "Ignore case distinctions in patterns and input data, so that characters that differ only in case match each other.", default_value = "false")]
    pub ignore_case: bool,

    #[arg(long, help = "Skip any command-line file which whole path matches any of the patterns")]
    pub exclude: Vec<String>,

    #[arg(long, help = "Treat `exclude` as regular expression", default_value = "false")]
    pub exclude_regex: bool,

    #[arg(long, help = "Search only files whose whole path matches any of the patterns",)]
    pub include: Vec<String>,

    #[arg(long, help = "Treat `include` as regular expression", default_value = "false")]
    pub include_regex: bool,

    #[arg(long, short, help = "Read all files under each directory, recursively, following symbolic links")]
    pub recursive: bool,

    #[arg(long, short, help = "Max depth to search for inside of directories. Default: 1")]
    pub depth: Option<usize>,


}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}

pub const COLORS: [Color; 6] = [ Color::Red , Color::Blue, Color::Green, Color::Cyan, Color::Magenta, Color::Yellow ];
