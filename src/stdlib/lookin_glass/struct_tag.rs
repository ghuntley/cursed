/// StructTag represents a squad tag in CURSED reflection
use std::collections::HashMap;
use std::fmt;
// use crate::stdlib::lookin_glass::error::{LookinGlassResult, reflection_error};

/// Represents a squad tag string with parsing capabilities
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructTag {
    raw: String,
    parsed: HashMap<String, String>,
}

impl StructTag {
    /// Create a new StructTag from a raw tag string
    pub fn new(raw: String) -> Self {
        let parsed = Self::parse_tag_string(&raw);
        Self { raw, parsed }
    }

    /// Create an empty StructTag
    pub fn empty() -> Self {
        Self {
            raw: String::new(),
            parsed: HashMap::new(),
        }
    }

    /// Get the raw tag string
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Get a tag value by key
    pub fn get(&self, key: &str) -> String {
        self.parsed.get(key).cloned().unwrap_or_default()
    }

    /// Lookup a tag value by key, returning the value and whether it was found
    pub fn lookup(&self, key: &str) -> (String, bool) {
        match self.parsed.get(key) {
            Some(value) => (value.clone(), true),
            None => (String::new(), false),
        }
    }

    /// Check if a tag key exists
    pub fn has_key(&self, key: &str) -> bool {
        self.parsed.contains_key(key)
    }

    /// Get all tag keys
    pub fn keys(&self) -> Vec<String> {
        self.parsed.keys().cloned().collect()
    }

    /// Get all tag values as a HashMap
    pub fn all(&self) -> &HashMap<String, String> {
        &self.parsed
    }

    /// Check if the tag is empty
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Get the number of tag key-value pairs
    pub fn len(&self) -> usize {
        self.parsed.len()
    }

    /// Set a tag value (creates a new StructTag)
    pub fn set(&self, key: &str, value: &str) -> Self {
        let mut new_parsed = self.parsed.clone();
        new_parsed.insert(key.to_string(), value.to_string());
        let new_raw = Self::build_tag_string(&new_parsed);
        Self {
            raw: new_raw,
            parsed: new_parsed,
        }
    }

    /// Remove a tag key (creates a new StructTag)
    pub fn remove(&self, key: &str) -> Self {
        let mut new_parsed = self.parsed.clone();
        new_parsed.remove(key);
        let new_raw = Self::build_tag_string(&new_parsed);
        Self {
            raw: new_raw,
            parsed: new_parsed,
        }
    }

    /// Parse a tag string into key-value pairs
    /// Format: `key1:"value1" key2:"value2,option1,option2" key3:"value3"`
    fn parse_tag_string(tag: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let tag = tag.trim();
        
        if tag.is_empty() {
            return result;
        }

        let mut chars = tag.chars().peekable();
        
        while chars.peek().is_some() {
            // Skip whitespace
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                chars.next();
            }
            
            if chars.peek().is_none() {
                break;
            }

            // Parse key
            let mut key = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == ':' || ch == ' ' || ch == '\t' {
                    break;
                }
                key.push(ch);
                chars.next();
            }

            if key.is_empty() {
                break;
            }

            // Skip whitespace and colon
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                chars.next();
            }
            
            if chars.peek() == Some(&':') {
                chars.next();
            }

            // Skip whitespace after colon
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                chars.next();
            }

            // Parse value (should be quoted)
            let mut value = String::new();
            if chars.peek() == Some(&'"') {
                chars.next(); // Skip opening quote
                
                while let Some(ch) = chars.next() {
                    if ch == '"' {
                        // Check for escaped quote
                        if chars.peek() == Some(&'"') {
                            chars.next();
                            value.push('"');
                        } else {
                            break; // End of quoted value
                        }
                    } else if ch == '\\' {
                        // Handle escape sequences
                        if let Some(escaped) = chars.next() {
                            match escaped {
                                'n' => value.push('\n'),
                                't' => value.push('\t'),
                                'r' => value.push('\r'),
                                '\\' => value.push('\\'),
                                '"' => value.push('"'),
                                _ => {
                                    value.push('\\');
                                    value.push(escaped);
                                }
                            }
                        }
                    } else {
                        value.push(ch);
                    }
                }
            } else {
                // Unquoted value (read until space)
                while let Some(&ch) = chars.peek() {
                    if ch == ' ' || ch == '\t' {
                        break;
                    }
                    value.push(ch);
                    chars.next();
                }
            }

            if !key.is_empty() {
                result.insert(key, value);
            }
        }

        result
    }

    /// Build a tag string from key-value pairs
    fn build_tag_string(tags: &HashMap<String, String>) -> String {
        let mut parts = Vec::new();
        
        for (key, value) in tags {
            let escaped_value = value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\t', "\\t")
                .replace('\r', "\\r");
            parts.push(format!("{}:\"{}\"", key, escaped_value));
        }
        
        parts.sort(); // For consistent ordering
        parts.join(" ")
    }

    /// Validate that a tag string is well-formed
    pub fn validate(tag: &str) -> LookinGlassResult<()> {
        let parsed = Self::parse_tag_string(tag);
        let rebuilt = Self::build_tag_string(&parsed);
        
        // Basic validation - we can parse and rebuild
        if tag.trim().is_empty() && rebuilt.is_empty() {
            return Ok(());
        }

        // Check for common issues
        let chars: Vec<char> = tag.chars().collect();
        let mut in_quotes = false;
        let mut escape_next = false;
        
        for &ch in &chars {
            if escape_next {
                escape_next = false;
                continue;
            }
            
            if ch == '\\' {
                escape_next = true;
                continue;
            }
            
            if ch == '"' {
                in_quotes = !in_quotes;
            }
        }
        
        if in_quotes {
            return Err(reflection_error("Unclosed quoted value in struct tag"));
        }
        
        Ok(())
    }
}

impl fmt::Display for StructTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl From<String> for StructTag {
    fn from(raw: String) -> Self {
        Self::new(raw)
    }
}

impl From<&str> for StructTag {
    fn from(raw: &str) -> Self {
        Self::new(raw.to_string())
    }
}

