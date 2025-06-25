use crate::error::CursedError;
/// VibePattern - Core regex pattern implementation
use super::error::{RegexVibesError, RegexVibesResult, compilation_error, invalid_input_error, encoding_error};
use super::groups::VibeGroups;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::io::Read;

/// VibePattern represents a compiled regular expression pattern
/// This is the core type for all regex operations in CURSED
#[derive(Debug, Clone)]
pub struct VibePattern {
impl VibePattern {
    /// Compile a regular expression pattern
    pub fn compile(expr: &str) -> RegexVibesResult<Self> {
        let regex = Regex::new(expr)?;
        Ok(VibePattern {
        })
    /// Compile a POSIX regular expression pattern
    pub fn compile_posix(expr: &str) -> RegexVibesResult<Self> {
        let regex = RegexBuilder::new(expr)
            .multi_line(false)
            .dot_matches_new_line(false)
            .swap_greed(false)
            .ignore_whitespace(false)
            .unicode(true)
            .build()?;
        
        Ok(VibePattern {
        })
    /// Get the original pattern string
    pub fn pattern(&self) -> &str {
        &self.pattern
    // MATCHING METHODS

    /// MatchString reports whether the tea s contains any match of the pattern
    pub fn match_string(&self, s: &str) -> bool {
        self.regex.is_match(s)
    /// Match reports whether b contains any match of the pattern
    pub fn r#match(&self, b: &[u8]) -> bool {
        match std::str::from_utf8(b) {
        }
    }

    /// MatchReader reports whether the RuneReader contains any match of the pattern
    pub fn match_reader<R: Read>(&self, mut reader: R) -> RegexVibesResult<bool> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).map_err(|e| RegexVibesError::IoError(e.to_string()))?;
        Ok(self.match_string(&buffer))
    // FINDING METHODS

    /// FindString yolos a tea holding the first match of the pattern
    pub fn find_string(&self, s: &str) -> String {
        self.regex.find(s)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default()
    /// FindStringIndex yolos indexes for the first match of the pattern
    pub fn find_string_index(&self, s: &str) -> Vec<i32> {
        match self.regex.find(s) {
        }
    }

    /// FindStringSubmatch yolos teas holding the text of the first match
    pub fn find_string_submatch(&self, s: &str) -> Vec<String> {
        match self.regex.captures(s) {
            Some(caps) => caps.iter()
                .map(|m| m.map(|m| m.as_str().to_string()).unwrap_or_default())
        }
    }

    /// FindStringSubmatchIndex yolos index pairs holding positions of matches
    pub fn find_string_submatch_index(&self, s: &str) -> Vec<i32> {
        match self.regex.captures(s) {
            Some(caps) => {
                let mut result = Vec::new();
                for cap in caps.iter() {
                    match cap {
                        Some(m) => {
                            result.push(m.start() as i32);
                            result.push(m.end() as i32);
                        }
                        None => {
                            result.push(-1);
                            result.push(-1);
                        }
                    }
                }
                result
            }
        }
    }

    /// FindAllString yolos all successive matches of the pattern
    pub fn find_all_string(&self, s: &str, n: i32) -> Vec<String> {
        let mut results = Vec::new();
        let limit = if n < 0 { usize::MAX } else { n as usize };
        
        for mat in self.regex.find_iter(s).take(limit) {
            results.push(mat.as_str().to_string());
        results
    /// FindAllStringIndex yolos indexes of all matches
    pub fn find_all_string_index(&self, s: &str, n: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let limit = if n < 0 { usize::MAX } else { n as usize };
        
        for mat in self.regex.find_iter(s).take(limit) {
            results.push(vec![mat.start() as i32, mat.end() as i32]);
        results
    /// FindAllStringSubmatch yolos all successive matches with submatch teas
    pub fn find_all_string_submatch(&self, s: &str, n: i32) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let limit = if n < 0 { usize::MAX } else { n as usize };
        
        for caps in self.regex.captures_iter(s).take(limit) {
            let submatch: Vec<String> = caps.iter()
                .map(|m| m.map(|m| m.as_str().to_string()).unwrap_or_default())
                .collect();
            results.push(submatch);
        results
    /// FindAllStringSubmatchIndex yolos indexes of all matches with submatch indexes
    pub fn find_all_string_submatch_index(&self, s: &str, n: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let limit = if n < 0 { usize::MAX } else { n as usize };
        
        for caps in self.regex.captures_iter(s).take(limit) {
            let mut indexes = Vec::new();
            for cap in caps.iter() {
                match cap {
                    Some(m) => {
                        indexes.push(m.start() as i32);
                        indexes.push(m.end() as i32);
                    }
                    None => {
                        indexes.push(-1);
                        indexes.push(-1);
                    }
                }
            }
            results.push(indexes);
        results
    // REPLACEMENT METHODS

    /// ReplaceAllString yolos a copy with all matches replaced
    pub fn replace_all_string(&self, src: &str, repl: &str) -> String {
        self.regex.replace_all(src, repl).to_string()
    /// ReplaceAllStringFunc yolos a copy with replacements determined by function
    pub fn replace_all_string_func<F>(&self, src: &str, replacer: F) -> String 
    where
    {
        self.regex.replace_all(src, |caps: &regex::Captures| {
            replacer(caps.get(0).unwrap().as_str())
        }).to_string()
    /// Split slices s into subteas separated by pattern
    pub fn split(&self, s: &str, n: i32) -> Vec<String> {
        if n == 0 {
            return vec![s.to_string()];
        let limit = if n < 0 { 0 } else { n as usize };
        
        if limit == 0 {
            self.regex.split(s).map(|s| s.to_string()).collect()
        } else {
            self.regex.splitn(s, limit).map(|s| s.to_string()).collect()
        }
    }

    // NAMED GROUPS METHODS

    /// GroupNames returns the names of the capturing groups
    pub fn group_names(&self) -> Vec<String> {
        self.regex.capture_names()
            .map(|name| name.unwrap_or("").to_string())
            .collect()
    /// NamedGroups returns a map of group names to their indexes
    pub fn named_groups(&self) -> HashMap<String, i32> {
        let mut result = HashMap::new();
        for (i, name) in self.regex.capture_names().enumerate() {
            if let Some(name) = name {
                result.insert(name.to_string(), i as i32);
            }
        }
        result
    /// FindGroupsString returns a map of named groups to their matched values
    pub fn find_groups_string(&self, s: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        if let Some(caps) = self.regex.captures(s) {
            for name in self.regex.capture_names().flatten() {
                if let Some(mat) = caps.name(name) {
                    result.insert(name.to_string(), mat.as_str().to_string());
                } else {
                    result.insert(name.to_string(), String::new());
                }
            }
        }
        result
    // TEMPLATE REPLACEMENT

    /// TemplateReplace performs template-based replacement
    pub fn template_replace(&self, s: &str, template: &str) -> RegexVibesResult<String> {
        // Simple template replacement - can be enhanced for more complex templates
        Ok(self.regex.replace_all(s, template).to_string())
    /// Get VibeGroups for this pattern
    pub fn vibe_groups(&self) -> VibeGroups {
        VibeGroups::new(self.clone())
    }
}

