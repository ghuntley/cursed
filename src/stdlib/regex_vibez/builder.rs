/// PatternBuilder - Fluent interface for building regular expressions
use super::pattern::VibePattern;
use super::error::{RegexVibesResult, compilation_error};
use std::fmt::Write;

/// PatternBuilder provides a fluent interface for constructing regex patterns
/// This makes it easier to build complex regex patterns programmatically
#[derive(Debug, Clone)]
pub struct PatternBuilder {
impl PatternBuilder {
    /// Create a new PatternBuilder
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set case insensitive matching
    pub fn case_insensitive(mut self, enabled: bool) -> Self {
        self.case_insensitive = enabled;
        self
    /// Set multiline mode
    pub fn multiline(mut self, enabled: bool) -> Self {
        self.multiline = enabled;
        self
    /// Set whether dot matches newline
    pub fn dot_matches_newline(mut self, enabled: bool) -> Self {
        self.dot_matches_newline = enabled;
        self
    /// Set unicode mode
    pub fn unicode(mut self, enabled: bool) -> Self {
        self.unicode = enabled;
        self
    /// Add a pattern that matches the start of string
    pub fn starts_with(mut self, s: &str) -> Self {
        if self.pattern.is_empty() {
            self.pattern.push('^');
        }
        self.pattern.push_str(&regex::escape(s));
        self
    /// Add a pattern that matches the end of string
    pub fn ends_with(mut self, s: &str) -> Self {
        self.pattern.push_str(&regex::escape(s));
        if !self.pattern.ends_with('$') {
            self.pattern.push('$');
        }
        self
    /// Add a pattern that contains the given string
    pub fn contains(mut self, s: &str) -> Self {
        self.pattern.push_str(&regex::escape(s));
        self
    /// Add a pattern that matches one or more of the given pattern
    pub fn one_or_more(mut self, s: &str) -> Self {
        self.pattern.push('(');
        self.pattern.push_str(&regex::escape(s));
        self.pattern.push_str(")+");
        self
    /// Add a pattern that matches zero or more of the given pattern
    pub fn zero_or_more(mut self, s: &str) -> Self {
        self.pattern.push('(');
        self.pattern.push_str(&regex::escape(s));
        self.pattern.push_str(")*");
        self
    /// Add an optional pattern
    pub fn optional(mut self, s: &str) -> Self {
        self.pattern.push('(');
        self.pattern.push_str(&regex::escape(s));
        self.pattern.push_str(")?");
        self
    /// Add a capturing group
    pub fn group(mut self, s: &str) -> Self {
        self.pattern.push('(');
        self.pattern.push_str(s);
        self.pattern.push(')');
        self
    /// Add a named capturing group
    pub fn named_group(mut self, name: &str, s: &str) -> Self {
        write!(&mut self.pattern, "(?P<{}>{})", name, s).unwrap();
        self
    /// Add alternatives (OR patterns)
    pub fn or(mut self, patterns: &[&str]) -> Self {
        if !patterns.is_empty() {
            self.pattern.push('(');
            for (i, pattern) in patterns.iter().enumerate() {
                if i > 0 {
                    self.pattern.push('|');
                }
                self.pattern.push_str(&regex::escape(pattern));
            }
            self.pattern.push(')');
        }
        self
    /// Add a digit pattern [0-9]
    pub fn digit(mut self) -> Self {
        self.pattern.push_str(r"\d");
        self
    /// Add one or more digits
    pub fn digits(mut self) -> Self {
        self.pattern.push_str(r"\d+");
        self
    /// Add a word character pattern [a-zA-Z0-9_]
    pub fn word(mut self) -> Self {
        self.pattern.push_str(r"\w");
        self
    /// Add one or more word characters
    pub fn words(mut self) -> Self {
        self.pattern.push_str(r"\w+");
        self
    /// Add a whitespace pattern
    pub fn space(mut self) -> Self {
        self.pattern.push_str(r"\s");
        self
    /// Add one or more whitespace characters
    pub fn spaces(mut self) -> Self {
        self.pattern.push_str(r"\s+");
        self
    /// Add an email pattern
    pub fn email(mut self) -> Self {
        self.pattern.push_str(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}");
        self
    /// Add a URL pattern
    pub fn url(mut self) -> Self {
        self.pattern.push_str(r"https?://[a-zA-Z0-9.-]+(?:\.[a-zA-Z]{2,})?(?:/[^\s]*)?");
        self
    /// Add a phone number pattern (US format)
    pub fn phone(mut self) -> Self {
        self.pattern.push_str(r"\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}");
        self
    /// Add a date pattern (YYYY-MM-DD format)
    pub fn date(mut self) -> Self {
        self.pattern.push_str(r"\d{4}-\d{2}-\d{2}");
        self
    /// Add a time pattern (HH:MM:SS format)
    pub fn time(mut self) -> Self {
        self.pattern.push_str(r"\d{2}:\d{2}:\d{2}");
        self
    /// Add an IPv4 address pattern
    pub fn ipv4(mut self) -> Self {
        self.pattern.push_str(r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)");
        self
    /// Add a hexadecimal color pattern
    pub fn hex_color(mut self) -> Self {
        self.pattern.push_str(r"#[0-9a-fA-F]{6}");
        self
    /// Add a credit card pattern (basic)
    pub fn credit_card(mut self) -> Self {
        self.pattern.push_str(r"\d{4}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}");
        self
    /// Add any character pattern
    pub fn any_char(mut self) -> Self {
        self.pattern.push('.');
        self
    /// Add a character class
    pub fn char_class(mut self, chars: &str) -> Self {
        self.pattern.push('[');
        self.pattern.push_str(chars);
        self.pattern.push(']');
        self
    /// Add a negated character class
    pub fn not_char_class(mut self, chars: &str) -> Self {
        self.pattern.push_str("[^");
        self.pattern.push_str(chars);
        self.pattern.push(']');
        self
    /// Add a quantifier to the last element
    pub fn repeat(mut self, min: usize, max: Option<usize>) -> Self {
        self.pattern.push('{');
        self.pattern.push_str(&min.to_string());
        if let Some(max) = max {
            self.pattern.push(',');
            self.pattern.push_str(&max.to_string());
        }
        self.pattern.push('}');
        self
    /// Add exactly n repetitions
    pub fn exactly(mut self, n: usize) -> Self {
        write!(&mut self.pattern, "{{{}}}", n).unwrap();
        self
    /// Add at least n repetitions
    pub fn at_least(mut self, n: usize) -> Self {
        write!(&mut self.pattern, "{{{},}}", n).unwrap();
        self
    /// Add between min and max repetitions
    pub fn between(mut self, min: usize, max: usize) -> Self {
        write!(&mut self.pattern, "{{{},{}}}", min, max).unwrap();
        self
    /// Add a word boundary
    pub fn word_boundary(mut self) -> Self {
        self.pattern.push_str(r"\b");
        self
    /// Add a non-word boundary
    pub fn not_word_boundary(mut self) -> Self {
        self.pattern.push_str(r"\B");
        self
    /// Add a literal string (escaped)
    pub fn literal(mut self, s: &str) -> Self {
        self.pattern.push_str(&regex::escape(s));
        self
    /// Add raw regex pattern (not escaped)
    pub fn raw(mut self, pattern: &str) -> Self {
        self.pattern.push_str(pattern);
        self
    /// Clear the current pattern
    pub fn clear(mut self) -> Self {
        self.pattern.clear();
        self
    /// Get the current pattern as a string
    pub fn pattern(&self) -> &str {
        &self.pattern
    /// Build the final VibePattern
    pub fn build(self) -> RegexVibesResult<VibePattern> {
        if self.pattern.is_empty() {
            return Err(compilation_error("Cannot build empty pattern"));
        let mut builder = regex::RegexBuilder::new(&self.pattern);
        builder
            .case_insensitive(self.case_insensitive)
            .multi_line(self.multiline)
            .dot_matches_new_line(self.dot_matches_newline)
            .unicode(self.unicode);

        let regex = builder.build()
            .map_err(|e| compilation_error(&format!("Pattern compilation failed: {}", e)))?;

        Ok(VibePattern::from_regex(regex, self.pattern))
    }
}

impl Default for PatternBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Extension to VibePattern to support builder
impl VibePattern {
    /// Create VibePattern from a Regex and pattern string
    pub(crate) fn from_regex(regex: regex::Regex, pattern: String) -> Self {
        Self { regex, pattern }
    }
