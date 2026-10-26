use regex::{Regex, RegexBuilder};
use anyhow::{Context, Result};
use clap::ValueEnum;

pub struct Matcher {
    regexes: Vec<Regex>,
    mode: MatchMode,
}

impl Matcher {
    pub fn new(patterns: &[String], mode: MatchMode, fixed: bool, ignore_case: bool) -> Result<Self> {
        let mut regexes = Vec::with_capacity(patterns.len());
        for pattern in patterns {
            let source = if fixed { regex::escape(pattern) } else { pattern.clone() };
            let regex = RegexBuilder::new(&source)
                .case_insensitive(ignore_case)
                .build()
                .with_context(|| format!("Failed to compile pattern \"{}\"", pattern))?;
        regexes.push(regex);
        }
        Ok(Matcher {
            mode,
            regexes,
        })
    }

    /**
        @returns
        - None: the text does not satisfy the mode.
        - Some: non-empty match locations, sorted by start offset.
    */
    pub fn find(&self, text: &str) -> Option<Vec<MatchLocation>> {
        let mut locations = Vec::new();
        let mut matched_patterns = 0;

        for (pattern_index, regex) in self.regexes.iter().enumerate() {
            let mut found = false;
            for m in regex.find_iter(text) {
                found = true;
                if m.start() < m.end() {
                    locations.push(MatchLocation { start: m.start(), end: m.end(), pattern_index });
                }
            }
            if found {
                matched_patterns += 1;
            }
        }
        
        let satisfied = match self.mode {
            MatchMode::All => matched_patterns == self.regexes.len(),
            MatchMode::Any => matched_patterns > 0,
        };
        if !satisfied {
            return None;
        }

        locations.sort_by_key(|m| m.start);
        Some(locations)
    }

    pub fn is_match(&self, text: &str) -> bool {
        match self.mode {
            MatchMode::All => self.regexes.iter().all(|r| r.is_match(text)),
            MatchMode::Any => self.regexes.iter().any(|r| r.is_match(text)),
        }
    }

    pub fn regexes(&self) -> &[Regex] {
        &self.regexes
    }
    pub fn is_empty(&self) -> bool {
        self.regexes().is_empty()
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum MatchMode {
    All,
    Any
}

#[derive(Debug)]
pub struct MatchLocation {
    pub start: usize,
    pub end: usize,
    pub pattern_index: usize
}

