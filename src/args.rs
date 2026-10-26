use std::path::{PathBuf};
use clap::Parser;
use crate::matcher::MatchMode;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(help = "Files or directories to scan", default_value = ".")]
    pub paths: Vec<PathBuf>,

    #[arg(long = "pattern", short = 'p', help = "Patterns to search for. See `fixed-strings`")]
    pub patterns: Vec<String>,

    #[arg(long, value_enum, help = "Match mode. Can be 'any' or 'all'", default_value_t = MatchMode::All)]
    pub mode: MatchMode,

    #[arg(long, short = 'F', help = "Interpret PATTERNS as fixed strings, not regular expressions")]
    pub fixed_strings: bool,

    #[arg(long, short = 'i', help = "Ignore case distinctions in patterns and input data, so that characters that differ only in case match each other.")]
    pub ignore_case: bool,

    #[arg(long, help = "Skip any command-line file which whole path matches any of the patterns")]
    pub exclude: Vec<String>,

    #[arg(long, help = "Treat `exclude` as regular expression")]
    pub exclude_regex: bool,

    #[arg(long, help = "Search only files whose whole path matches any of the patterns",)]
    pub include: Vec<String>,

    #[arg(long, help = "Treat `include` as regular expression")]
    pub include_regex: bool,

    #[arg(long, short, help = "Read all files under each directory, recursively, following symbolic links")]
    pub recursive: bool,

    #[arg(long, short, help = "Max depth to search for inside of directories. Default: 1")]
    pub depth: Option<usize>,


}

