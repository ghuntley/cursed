//! Output formatting and diff generation for CURSED formatter

use std::fmt;
use colored::*;

/// Diff line type
#[derive(Debug, Clone, PartialEq)]
pub enum DiffLineType {
    Context,
    Addition,
    Deletion,
    Header,
}

/// A single line in a diff
#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
    pub original_line: Option<usize>,
    pub formatted_line: Option<usize>,
}

/// Diff output formatter
#[derive(Debug, Clone)]
pub struct DiffFormatter {
    pub colored: bool,
    pub context_lines: usize,
    pub show_line_numbers: bool,
}

impl Default for DiffFormatter {
    fn default() -> Self {
        Self {
            colored: true,
            context_lines: 3,
            show_line_numbers: true,
        }
    }
}

impl DiffFormatter {
    /// Create a new diff formatter
    pub fn new(colored: bool, context_lines: usize, show_line_numbers: bool) -> Self {
        Self {
            colored,
            context_lines,
            show_line_numbers,
        }
    }

    /// Generate a unified diff between original and formatted code
    pub fn generate_diff(&self, original: &str, formatted: &str, filename: &str) -> String {
        let original_lines: Vec<&str> = original.lines().collect();
        let formatted_lines: Vec<&str> = formatted.lines().collect();
        
        let diff_lines = self.compute_diff(&original_lines, &formatted_lines);
        
        if diff_lines.is_empty() {
            return String::new();
        }
        
        let mut output = String::new();
        
        // Add header
        if self.colored {
            output.push_str(&format!("{}\n", format!("--- {}", filename).cyan()));
            output.push_str(&format!("{}\n", format!("+++ {}", filename).cyan()));
        } else {
            output.push_str(&format!("--- {}\n", filename));
            output.push_str(&format!("+++ {}\n", filename));
        }
        
        // Add diff chunks
        let chunks = self.group_diff_lines(&diff_lines);
        for chunk in chunks {
            output.push_str(&self.format_chunk(&chunk));
        }
        
        output
    }

    /// Compute diff lines using a simple algorithm
    fn compute_diff(&self, original: &[&str], formatted: &[&str]) -> Vec<DiffLine> {
        let mut diff_lines = Vec::new();
        let mut i = 0;
        let mut j = 0;
        
        while i < original.len() || j < formatted.len() {
            if i < original.len() && j < formatted.len() {
                if original[i] == formatted[j] {
                    diff_lines.push(DiffLine {
                        line_type: DiffLineType::Context,
                        content: original[i].to_string(),
                        original_line: Some(i + 1),
                        formatted_line: Some(j + 1),
                    });
                    i += 1;
                    j += 1;
                } else {
                    // Find the next matching line
                    let mut found_match = false;
                    for k in (i + 1)..original.len().min(i + 10) {
                        if let Some(l) = formatted.get(j..).and_then(|slice| {
                            slice.iter().position(|&line| line == original[k])
                        }) {
                            // Found a match, add deletions and additions
                            for idx in i..k {
                                diff_lines.push(DiffLine {
                                    line_type: DiffLineType::Deletion,
                                    content: original[idx].to_string(),
                                    original_line: Some(idx + 1),
                                    formatted_line: None,
                                });
                            }
                            for idx in j..(j + l) {
                                if idx < formatted.len() {
                                    diff_lines.push(DiffLine {
                                        line_type: DiffLineType::Addition,
                                        content: formatted[idx].to_string(),
                                        original_line: None,
                                        formatted_line: Some(idx + 1),
                                    });
                                }
                            }
                            i = k;
                            j = j + l;
                            found_match = true;
                            break;
                        }
                    }
                    
                    if !found_match {
                        // No match found, treat as simple replacement
                        diff_lines.push(DiffLine {
                            line_type: DiffLineType::Deletion,
                            content: original[i].to_string(),
                            original_line: Some(i + 1),
                            formatted_line: None,
                        });
                        diff_lines.push(DiffLine {
                            line_type: DiffLineType::Addition,
                            content: formatted[j].to_string(),
                            original_line: None,
                            formatted_line: Some(j + 1),
                        });
                        i += 1;
                        j += 1;
                    }
                }
            } else if i < original.len() {
                diff_lines.push(DiffLine {
                    line_type: DiffLineType::Deletion,
                    content: original[i].to_string(),
                    original_line: Some(i + 1),
                    formatted_line: None,
                });
                i += 1;
            } else {
                diff_lines.push(DiffLine {
                    line_type: DiffLineType::Addition,
                    content: formatted[j].to_string(),
                    original_line: None,
                    formatted_line: Some(j + 1),
                });
                j += 1;
            }
        }
        
        diff_lines
    }

    /// Group diff lines into chunks with context
    fn group_diff_lines(&self, diff_lines: &[DiffLine]) -> Vec<Vec<DiffLine>> {
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut last_change_index = None;
        
        for (i, line) in diff_lines.iter().enumerate() {
            match line.line_type {
                DiffLineType::Context => {
                    if let Some(last_change) = last_change_index {
                        if i - last_change > self.context_lines * 2 {
                            // Too much context, start a new chunk
                            // Add trailing context to current chunk
                            let end_line = if i > 0 { 
                                std::cmp::min(last_change + self.context_lines, i - 1) 
                            } else { 
                                last_change + self.context_lines 
                            };
                            for j in (last_change + 1)..=end_line {
                                if j < diff_lines.len() {
                                    current_chunk.push(diff_lines[j].clone());
                                }
                            }
                            chunks.push(current_chunk);
                            current_chunk = Vec::new();
                            
                            // Add leading context to new chunk
                            for j in (i - self.context_lines).max(0)..i {
                                current_chunk.push(diff_lines[j].clone());
                            }
                        }
                    }
                    current_chunk.push(line.clone());
                }
                DiffLineType::Addition | DiffLineType::Deletion => {
                    if last_change_index.is_none() {
                        // First change, add leading context
                        let start = i.saturating_sub(self.context_lines);
                        for j in start..i {
                            current_chunk.push(diff_lines[j].clone());
                        }
                    }
                    current_chunk.push(line.clone());
                    last_change_index = Some(i);
                }
                DiffLineType::Header => {
                    current_chunk.push(line.clone());
                }
            }
        }
        
        // Add trailing context to last chunk
        if let Some(last_change) = last_change_index {
            let end = (last_change + self.context_lines + 1).min(diff_lines.len());
            for j in (last_change + 1)..end {
                if !current_chunk.iter().any(|l| l.original_line == diff_lines[j].original_line) {
                    current_chunk.push(diff_lines[j].clone());
                }
            }
        }
        
        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }
        
        chunks
    }

    /// Format a single chunk of diff lines
    fn format_chunk(&self, chunk: &[DiffLine]) -> String {
        if chunk.is_empty() {
            return String::new();
        }
        
        let mut output = String::new();
        
        // Add chunk header
        let (start_orig, count_orig) = self.get_line_range(chunk, true);
        let (start_fmt, count_fmt) = self.get_line_range(chunk, false);
        
        if self.colored {
            output.push_str(&format!("{}\n", 
                format!("@@ -{},{} +{},{} @@", start_orig, count_orig, start_fmt, count_fmt).magenta()));
        } else {
            output.push_str(&format!("@@ -{},{} +{},{} @@\n", start_orig, count_orig, start_fmt, count_fmt));
        }
        
        // Add diff lines
        for line in chunk {
            output.push_str(&self.format_diff_line(line));
        }
        
        output
    }

    /// Format a single diff line
    fn format_diff_line(&self, line: &DiffLine) -> String {
        let prefix = match line.line_type {
            DiffLineType::Context => " ",
            DiffLineType::Addition => "+",
            DiffLineType::Deletion => "-",
            DiffLineType::Header => "",
        };
        
        let mut formatted = String::new();
        
        if self.show_line_numbers {
            let orig_num = line.original_line.map(|n| n.to_string()).unwrap_or_else(|| " ".to_string());
            let fmt_num = line.formatted_line.map(|n| n.to_string()).unwrap_or_else(|| " ".to_string());
            formatted.push_str(&format!("{:>4} {:>4} ", orig_num, fmt_num));
        }
        
        formatted.push_str(prefix);
        formatted.push_str(&line.content);
        
        if self.colored {
            formatted = match line.line_type {
                DiffLineType::Context => formatted.normal().to_string(),
                DiffLineType::Addition => formatted.green().to_string(),
                DiffLineType::Deletion => formatted.red().to_string(),
                DiffLineType::Header => formatted.cyan().to_string(),
            };
        }
        
        formatted.push('\n');
        formatted
    }

    /// Get the line range for a chunk
    fn get_line_range(&self, chunk: &[DiffLine], original: bool) -> (usize, usize) {
        let mut start = usize::MAX;
        let mut end = 0;
        
        for line in chunk {
            let line_num = if original {
                line.original_line
            } else {
                line.formatted_line
            };
            
            if let Some(num) = line_num {
                start = start.min(num);
                end = end.max(num);
            }
        }
        
        if start == usize::MAX {
            (0, 0)
        } else {
            (start, end - start + 1)
        }
    }
}

/// Statistics about formatting changes
#[derive(Debug, Clone, Default)]
pub struct FormattingStats {
    pub files_processed: usize,
    pub files_changed: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub lines_unchanged: usize,
}

impl FormattingStats {
    /// Add stats from a single file
    pub fn add_file(&mut self, original_lines: usize, formatted_lines: usize, changed: bool) {
        self.files_processed += 1;
        if changed {
            self.files_changed += 1;
        }
        
        if formatted_lines > original_lines {
            self.lines_added += formatted_lines - original_lines;
        } else {
            self.lines_removed += original_lines - formatted_lines;
        }
    }
    
    /// Get total lines processed
    pub fn total_lines(&self) -> usize {
        self.lines_added + self.lines_removed + self.lines_unchanged
    }
    
    /// Get percentage of files changed
    pub fn change_percentage(&self) -> f64 {
        if self.files_processed == 0 {
            0.0
        } else {
            (self.files_changed as f64 / self.files_processed as f64) * 100.0
        }
    }
}

impl fmt::Display for FormattingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Processed {} files, {} changed ({:.1}%)", 
            self.files_processed, 
            self.files_changed, 
            self.change_percentage())?;
        
        if self.lines_added > 0 || self.lines_removed > 0 {
            write!(f, ", {} lines added, {} lines removed", 
                self.lines_added, 
                self.lines_removed)?;
        }
        
        Ok(())
    }
}

/// Format check result
#[derive(Debug, Clone)]
pub struct FormatCheckResult {
    pub filename: String,
    pub needs_formatting: bool,
    pub diff: Option<String>,
    pub error: Option<String>,
}

impl FormatCheckResult {
    /// Create a result for a file that doesn't need formatting
    pub fn no_change(filename: String) -> Self {
        Self {
            filename,
            needs_formatting: false,
            diff: None,
            error: None,
        }
    }
    
    /// Create a result for a file that needs formatting
    pub fn needs_formatting(filename: String, diff: Option<String>) -> Self {
        Self {
            filename,
            needs_formatting: true,
            diff,
            error: None,
        }
    }
    
    /// Create a result for a file with an error
    pub fn error(filename: String, error: String) -> Self {
        Self {
            filename,
            needs_formatting: false,
            diff: None,
            error: Some(error),
        }
    }
}

impl fmt::Display for FormatCheckResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(error) = &self.error {
            write!(f, "{}: error: {}", self.filename, error)
        } else if self.needs_formatting {
            write!(f, "{}: needs formatting", self.filename)
        } else {
            write!(f, "{}: already formatted", self.filename)
        }
    }
}
