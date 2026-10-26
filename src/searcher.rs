use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
// use std::fmt::Write;
use anyhow::Context;
use clap::Parser;
use colored::{Color, Colorize};
use file_identify::tags_from_path;
use walkdir::WalkDir;
use crate::args::Args;
use crate::{is_broken_pipe};
use crate::matcher::{MatchMode, Matcher, MatchLocation};

const COLORS: [Color; 6] = [ Color::Red , Color::Blue, Color::Green, Color::Cyan, Color::Magenta, Color::Yellow ];
pub struct Searcher {
    paths: Vec<PathBuf>,
    depth: Option<usize>,
    pattern_matcher: Matcher,
    include_matcher: Matcher,
    exclude_matcher: Matcher,
}

impl Searcher {
    pub fn new(args: Args) -> anyhow::Result<Self> {
        let pattern_matcher = Matcher::new(&args.patterns, args.mode, args.fixed_strings, args.ignore_case)?;
        let include_matcher = Matcher::new(&args.include, MatchMode::Any, !args.include_regex, args.ignore_case)?;
        let exclude_matcher = Matcher::new(&args.exclude, MatchMode::Any, !args.exclude_regex, args.ignore_case)?;

        let depth = match (args.recursive, args.depth) {
            (_, Some(depth)) => Some(depth),
            (true, None) => None,
            (false, None) => Some(1),
        };

        Ok(Searcher {
            paths: args.paths, // partial move out of an owned struct: allowed
            depth,
            pattern_matcher,
            include_matcher,
            exclude_matcher,
        })
    }

    fn is_excluded(&self, path: &Path) -> bool {
        !self.exclude_matcher.is_empty() && self.exclude_matcher.is_match(&path.to_string_lossy())
    }

    fn is_included(&self, path: &Path) -> bool {
        self.include_matcher.is_empty() || self.include_matcher.is_match(&path.to_string_lossy())
    }


    fn read_dir(&self, root: &Path, out: &mut impl Write) -> anyhow::Result<bool> {
        let mut walker = WalkDir::new(root).follow_links(true);
        // Limit recursiveness
        if let Some(depth) = self.depth {
            walker = walker.max_depth(depth);
        }

        let mut found = false;

        for entry in walker.into_iter().filter_entry(|e| !self.is_excluded(e.path())) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("grepper: {}", e);
                    continue;
                }
            };

            if !entry.file_type().is_file() || !self.is_included(entry.path()) {
                continue;
            }

            found |= self.search_path(entry.path(), out)?;
        }

        Ok(found)
    }
    fn read_file(&self, path: &Path, out: &mut impl Write) -> anyhow::Result<bool> {
        let tags = tags_from_path(path)?;
        // Read only text files
        if !tags.contains("text") { return Ok(false); }

        let file = File::open(path).with_context(|| format!("cannot open {}", path.display()))?;
        let label = path.display().to_string().green().to_string(); // colored once per file
        let mut found = false;

        for (index, bytes) in BufReader::new(file).split(b'\n').enumerate() {
            let bytes = bytes.with_context(|| format!("cannot read {}", path.display()))?;
            let line = String::from_utf8_lossy(&bytes);
            let line = line.trim_end_matches('\r');

            let Some(locations) = self.pattern_matcher.find(line) else { continue };

            found = true;

            write!(out, "[{}:{}] ", label, (index + 1).to_string().cyan())?;

            writeln!(out, "{}", highlight(line, &locations))?;
        }
        Ok(found)
    }

    fn search_path(&self, path: &Path, out: &mut impl Write) -> anyhow::Result<bool> {
        if self.pattern_matcher.is_empty() {
            writeln!(out, "{}", path.display())?;
            return Ok(true);
        }

        match self.read_file(path, out) {
            Ok(found) => Ok(found),
            Err(e) if is_broken_pipe(&e) => Err(e),
            Err(e) => { eprintln!("grepper: {e:#}"); Ok(false) },
        }
    }
}

pub fn highlight(line: &str, locations: &[MatchLocation]) -> String {
    let mut out = String::with_capacity(line.len() + locations.len());
    // let mut out = String::with_capacity(line.len() + locations.len() * 12);
    let mut cursor = 0;
    for m in locations {
        // Inside earlier match
        if m.end <= cursor { continue; }
        let start = m.start.max(cursor);

        // Push everything between cursor and match start without making changes
        out.push_str(&line[cursor..start]);
        // Get next color
        let color = COLORS[m.pattern_index % COLORS.len()];
        // Push everything in between match start and end index and color it using match color
        out.push_str(&format!("{}", line[start..m.end].color(color)));

        // Move cursor
        cursor = m.end;
    }
    // Push remaining line content
    out.push_str(&line[cursor..]);
    out
}

pub fn run() -> anyhow::Result<bool> {
    let args = Args::parse();
    let mut stdout = std::io::stdout().lock();
    let searcher = Searcher::new(args)?;

    for path in &searcher.paths {
        if path.is_file() {
            searcher.search_path(path, &mut stdout)?;
        } else {
            searcher.read_dir(path, &mut stdout)?;
        }

    }

    stdout.flush()?;

    Ok(true)
}

