use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::Result;
use file_identify::tags_from_path;
use colored::{Color, Colorize};

use crate::args::{Args, COLORS};
use crate::args::pattern::PatternTrait;

impl Args {
    pub fn run(&self) -> Result<()> {
        // If no path is provided run in current directory
        let paths = self.paths.clone().unwrap_or(vec![PathBuf::from(".")]);
        for path in paths {
            if path.is_file() {
                self.read_file(&path)?;
            } else {
                self.read_dir(&path)?;
            }

        }

        Ok(())
    }

    fn read_dir(&self, root_path: &Path) -> Result<()> {
        let mut walker = WalkDir::new(root_path)
            .follow_links(true);
        // Limit recursiveness
        if !self.recursive || self.depth.is_some() {
            walker = walker.max_depth(self.depth.unwrap_or(1));
        }

        for entry in walker {
            let entry = entry?;
            let path = entry.path();
            let path_str = path.to_str().unwrap_or("");

            // Exclude entry if path is in `exclude` list
            if self.match_patterns(&self.exclude, path_str, &MatchType::Any, !self.exclude_regex) { continue; }
            // Include entry if path is in `include` list
            if !self.match_patterns(&self.include, path_str, &MatchType::All, !self.include_regex) { continue; }


            if !path.is_file() {
                continue;
            }

            // Print file path if no pattern is provided
            if self.patterns.is_empty() {
                println!("{}", path_str);
                continue;
            }
            self.read_file(path)?
        }

        Ok(())
    }
    fn read_file(&self, path: &Path) -> Result<()> {
        let tags = tags_from_path(path)?;
        // Read only text files
        if !tags.contains("text") { return Ok(()); }

        let reader = BufReader::new(File::open(path)?);
        let match_type = if self.all { MatchType::All } else if self.any { MatchType::Any } else { MatchType::All };

        for (line_index, line) in reader.lines().enumerate() {
            let line = line?;

            // Exclude lines which don't match specified patterns
            if !self.match_patterns(&self.patterns, line.as_str(), &match_type, self.fixed_strings) { continue; }
            // Print formatted line
            println!("{}{}", self.format_prefix(path, line_index + 1), self.format_highlight(&line));
        }
        Ok(())
    }

    fn format_highlight(&self, line: &str) -> String {
        // Get all matches and sort by match start indexes
        let mut locations = self.get_matches(line);
        locations.sort_by_key(|m| m.start);

        let mut out = String::new();
        let mut cursor = 0;
        for m in &locations {
            // Inside earlier match
            if m.end <= cursor { continue; }
            let start = m.start.max(cursor);
            // Push everything between cursor and match start without making changes
            out.push_str(&line[cursor..start]);
            // Push everything in between match start and end index and color it using match color
            out.push_str(&line[start..m.end].color(m.color));
            // Move cursor
            cursor = m.end;
        }
        // Push remaining line content
        out.push_str(&line[cursor..]);
        out
    }

    fn format_prefix(&self, path: &Path, number: usize) -> String {
        if self.no_info {
            return String::new();
        }
        format!("[{}:{}] ", path.display().to_string().green(), number.to_string().cyan())
    }


    fn get_matches(&self, text: &str) -> Vec<MatchLocation> {
        let mut matches: Vec<MatchLocation> = Vec::new();
        let mut color_cycle = COLORS.iter().cycle();

        for regex in &self.patterns {
            // Get next color
            let color = color_cycle.next();

            // Find all matches for current regex
            let match_locations: Vec<MatchLocation> = regex
                .find_iter(text)
                .map(|m| MatchLocation {
                    start: m.start(),
                    end: m.end(),
                    color: *color.unwrap_or(&Color::Red)
                })
                .collect();
            // Extend current matches
            matches.extend(match_locations);
        }

        matches
    }
    fn match_patterns<T: PatternTrait>(&self, patterns: &[T], text: &str, match_type: &MatchType, strict: bool) -> bool {
        let mut iter = patterns.iter();

        let match_func = | pattern: &T | pattern.matches_text(text, strict);

        match match_type {
            MatchType::All => { iter.all(match_func) },
            MatchType::Any => { iter.any(match_func) }
        }
    }
}

#[derive(Debug)]
enum MatchType {
    All,
    Any
}

#[derive(Debug)]
struct MatchLocation {
    start: usize,
    end: usize,
    color: Color
}

#[cfg(test)]
mod tests {
    use crate::tests::{get_args, TestArgs};
    use super::*;

    #[test]
    fn _format_prefix() {
        let args = get_args(TestArgs::NoInfo);
        assert!(args.format_prefix(&PathBuf::from("./"), 1).is_empty(), "Should be empty");
        let args = get_args(TestArgs::Default);
        let content = args.format_prefix(&PathBuf::from("./"), 1);
        assert!(!content.is_empty(), "Should not be empty");
    }
}