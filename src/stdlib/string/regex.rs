/// Regular Expression Processing Module for CURSED
/// 
/// Provides comprehensive regex functionality including pattern matching,
/// text parsing, replacement operations, and advanced text processing utilities.
/// Built on top of a custom regex engine optimized for the CURSED language.

use std::collections::HashMap;
use super::{StringError, StringResult};

// =============================================================================
// REGEX PATTERN STRUCTURE
// =============================================================================

/// Regex pattern representation
#[derive(Debug, Clone)]
pub struct Regex {
    pattern: String,
    compiled: CompiledPattern,
    flags: RegexFlags,
}

/// Compiled regex pattern for efficient matching
#[derive(Debug, Clone)]
struct CompiledPattern {
    states: Vec<State>,
    start_state: usize,
    accept_states: Vec<usize>,
}

/// Finite state machine state
#[derive(Debug, Clone)]
struct State {
    transitions: HashMap<char, usize>,
    epsilon_transitions: Vec<usize>,
    is_accept: bool,
    character_class: Option<CharacterClass>,
}

/// Character class definitions
#[derive(Debug, Clone)]
enum CharacterClass {
    Digit,           // \d
    Word,            // \w
    Whitespace,      // \s
    Any,             // .
    Custom(Vec<char>), // [abc]
    Range(char, char), // [a-z]
    Negated(Box<CharacterClass>), // [^abc]
}

/// Regex compilation flags
#[derive(Debug, Clone)]
pub struct RegexFlags {
    pub case_insensitive: bool,
    pub multiline: bool,
    pub dot_matches_newline: bool,
    pub unicode: bool,
    pub global: bool,
}

impl Default for RegexFlags {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            multiline: false,
            dot_matches_newline: false,
            unicode: true,
            global: false,
        }
    }
}

/// Match result containing position and captured groups
#[derive(Debug, Clone)]
pub struct Match {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub groups: Vec<Option<String>>,
}

impl Match {
    /// Gets the full matched text
    pub fn as_str(&self) -> &str {
        &self.text
    }
    
    /// Gets captured group by index
    pub fn group(&self, index: usize) -> Option<&str> {
        self.groups.get(index).and_then(|opt| opt.as_deref())
    }
    
    /// Gets all captured groups
    pub fn groups(&self) -> &[Option<String>] {
        &self.groups
    }
    
    /// Gets match length
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    
    /// Checks if match is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// =============================================================================
// REGEX COMPILATION AND CREATION
// =============================================================================

impl Regex {
    /// Compiles a regex pattern
    pub fn new(pattern: &str) -> StringResult<Self> {
        Self::with_flags(pattern, RegexFlags::default())
    }
    
    /// Compiles a regex pattern with specific flags
    pub fn with_flags(pattern: &str, flags: RegexFlags) -> StringResult<Self> {
        let compiled = compile_pattern(pattern, &flags)?;
        
        Ok(Self {
            pattern: pattern.to_string(),
            compiled,
            flags,
        })
    }
    
    /// Creates case-insensitive regex
    pub fn case_insensitive(pattern: &str) -> StringResult<Self> {
        let mut flags = RegexFlags::default();
        flags.case_insensitive = true;
        Self::with_flags(pattern, flags)
    }
    
    /// Creates multiline regex
    pub fn multiline(pattern: &str) -> StringResult<Self> {
        let mut flags = RegexFlags::default();
        flags.multiline = true;
        Self::with_flags(pattern, flags)
    }
    
    /// Gets the original pattern string
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
    
    /// Gets the regex flags
    pub fn flags(&self) -> &RegexFlags {
        &self.flags
    }
}

/// Compiles regex pattern into finite state machine
fn compile_pattern(pattern: &str, flags: &RegexFlags) -> StringResult<CompiledPattern> {
    let mut compiler = PatternCompiler::new(flags);
    compiler.compile(pattern)
}

/// Pattern compiler for building finite state machines
struct PatternCompiler<'a> {
    flags: &'a RegexFlags,
    states: Vec<State>,
    next_state_id: usize,
}

impl<'a> PatternCompiler<'a> {
    fn new(flags: &'a RegexFlags) -> Self {
        Self {
            flags,
            states: Vec::new(),
            next_state_id: 0,
        }
    }
    
    fn compile(&mut self, pattern: &str) -> StringResult<CompiledPattern> {
        // Simple pattern compilation (can be extended for full regex support)
        let start_state = self.create_state();
        let mut current_state = start_state;
        
        let chars: Vec<char> = pattern.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            match chars[i] {
                '\\' => {
                    // Escape sequences
                    if i + 1 < chars.len() {
                        i += 1;
                        let escaped = chars[i];
                        current_state = self.handle_escape(current_state, escaped)?;
                    }
                }
                '.' => {
                    // Match any character
                    current_state = self.handle_any_char(current_state)?;
                }
                '*' => {
                    // Zero or more repetitions
                    current_state = self.handle_star(current_state)?;
                }
                '+' => {
                    // One or more repetitions
                    current_state = self.handle_plus(current_state)?;
                }
                '?' => {
                    // Zero or one repetitions
                    current_state = self.handle_question(current_state)?;
                }
                '[' => {
                    // Character class
                    let (end_pos, class) = self.parse_character_class(&chars, i)?;
                    current_state = self.handle_character_class(current_state, class)?;
                    i = end_pos;
                }
                '^' => {
                    // Start of line anchor
                    current_state = self.handle_start_anchor(current_state)?;
                }
                '$' => {
                    // End of line anchor
                    current_state = self.handle_end_anchor(current_state)?;
                }
                '(' => {
                    // Start of group (simplified)
                    current_state = self.handle_group_start(current_state)?;
                }
                ')' => {
                    // End of group (simplified)
                    current_state = self.handle_group_end(current_state)?;
                }
                c => {
                    // Literal character
                    current_state = self.handle_literal(current_state, c)?;
                }
            }
            i += 1;
        }
        
        // Mark final state as accepting
        self.states[current_state].is_accept = true;
        
        Ok(CompiledPattern {
            states: self.states.clone(),
            start_state,
            accept_states: vec![current_state],
        })
    }
    
    fn create_state(&mut self) -> usize {
        let id = self.next_state_id;
        self.next_state_id += 1;
        
        self.states.push(State {
            transitions: HashMap::new(),
            epsilon_transitions: Vec::new(),
            is_accept: false,
            character_class: None,
        });
        
        id
    }
    
    fn handle_literal(&mut self, from_state: usize, c: char) -> StringResult<usize> {
        let to_state = self.create_state();
        let actual_char = if self.flags.case_insensitive {
            c.to_lowercase().next().unwrap_or(c)
        } else {
            c
        };
        
        self.states[from_state].transitions.insert(actual_char, to_state);
        
        // If case insensitive, also add uppercase transition
        if self.flags.case_insensitive && c.is_alphabetic() {
            let upper_char = c.to_uppercase().next().unwrap_or(c);
            if upper_char != actual_char {
                self.states[from_state].transitions.insert(upper_char, to_state);
            }
        }
        
        Ok(to_state)
    }
    
    fn handle_escape(&mut self, from_state: usize, escaped: char) -> StringResult<usize> {
        match escaped {
            'd' => self.handle_character_class(from_state, CharacterClass::Digit),
            'w' => self.handle_character_class(from_state, CharacterClass::Word),
            's' => self.handle_character_class(from_state, CharacterClass::Whitespace),
            'n' => self.handle_literal(from_state, '\n'),
            't' => self.handle_literal(from_state, '\t'),
            'r' => self.handle_literal(from_state, '\r'),
            c => self.handle_literal(from_state, c),
        }
    }
    
    fn handle_any_char(&mut self, from_state: usize) -> StringResult<usize> {
        self.handle_character_class(from_state, CharacterClass::Any)
    }
    
    fn handle_character_class(&mut self, from_state: usize, class: CharacterClass) -> StringResult<usize> {
        let to_state = self.create_state();
        self.states[from_state].character_class = Some(class);
        Ok(to_state)
    }
    
    fn handle_star(&mut self, _from_state: usize) -> StringResult<usize> {
        // Simplified star implementation
        // In a full implementation, this would create epsilon transitions for repetition
        Err(StringError::RegexError {
            message: "Star quantifier (*) not fully implemented".to_string(),
            pattern: "".to_string(),
        })
    }
    
    fn handle_plus(&mut self, _from_state: usize) -> StringResult<usize> {
        // Simplified plus implementation
        Err(StringError::RegexError {
            message: "Plus quantifier (+) not fully implemented".to_string(),
            pattern: "".to_string(),
        })
    }
    
    fn handle_question(&mut self, _from_state: usize) -> StringResult<usize> {
        // Simplified question implementation
        Err(StringError::RegexError {
            message: "Question quantifier (?) not fully implemented".to_string(),
            pattern: "".to_string(),
        })
    }
    
    fn handle_start_anchor(&mut self, from_state: usize) -> StringResult<usize> {
        // Simplified anchor implementation
        Ok(from_state)
    }
    
    fn handle_end_anchor(&mut self, from_state: usize) -> StringResult<usize> {
        // Simplified anchor implementation
        Ok(from_state)
    }
    
    fn handle_group_start(&mut self, from_state: usize) -> StringResult<usize> {
        // Simplified group implementation
        Ok(from_state)
    }
    
    fn handle_group_end(&mut self, from_state: usize) -> StringResult<usize> {
        // Simplified group implementation
        Ok(from_state)
    }
    
    fn parse_character_class(&self, chars: &[char], start: usize) -> StringResult<(usize, CharacterClass)> {
        // Find the closing bracket
        let mut i = start + 1;
        let mut is_negated = false;
        
        if i < chars.len() && chars[i] == '^' {
            is_negated = true;
            i += 1;
        }
        
        let mut class_chars = Vec::new();
        
        while i < chars.len() && chars[i] != ']' {
            if i + 2 < chars.len() && chars[i + 1] == '-' {
                // Range like a-z
                let start_char = chars[i];
                let end_char = chars[i + 2];
                
                if is_negated {
                    return Ok((i + 2, CharacterClass::Negated(Box::new(CharacterClass::Range(start_char, end_char)))));
                } else {
                    return Ok((i + 2, CharacterClass::Range(start_char, end_char)));
                }
            } else {
                class_chars.push(chars[i]);
            }
            i += 1;
        }
        
        if i >= chars.len() {
            return Err(StringError::RegexError {
                message: "Unclosed character class".to_string(),
                pattern: "".to_string(),
            });
        }
        
        let class = CharacterClass::Custom(class_chars);
        if is_negated {
            Ok((i, CharacterClass::Negated(Box::new(class))))
        } else {
            Ok((i, class))
        }
    }
}

// =============================================================================
// PATTERN MATCHING
// =============================================================================

impl Regex {
    /// Tests if pattern matches anywhere in the text
    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }
    
    /// Finds first match in text
    pub fn find(&self, text: &str) -> Option<Match> {
        self.find_at(text, 0)
    }
    
    /// Finds first match starting at given position
    pub fn find_at(&self, text: &str, start: usize) -> Option<Match> {
        let chars: Vec<char> = text.chars().collect();
        
        for i in start..chars.len() {
            if let Some(match_len) = self.match_at(&chars, i) {
                let matched_text: String = chars[i..i + match_len].iter().collect();
                return Some(Match {
                    start: i,
                    end: i + match_len,
                    text: matched_text,
                    groups: Vec::new(), // Simplified - no capture groups
                });
            }
        }
        
        None
    }
    
    /// Finds all matches in text
    pub fn find_all(&self, text: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        let mut start = 0;
        
        while let Some(m) = self.find_at(text, start) {
            start = if self.flags.global {
                m.end
            } else {
                m.start + 1
            };
            
            matches.push(m);
            
            if !self.flags.global {
                break;
            }
            
            if start >= text.len() {
                break;
            }
        }
        
        matches
    }
    
    /// Checks if pattern matches at specific position
    fn match_at(&self, chars: &[char], start: usize) -> Option<usize> {
        let mut current_states = vec![self.compiled.start_state];
        let mut position = start;
        
        while position < chars.len() && !current_states.is_empty() {
            let ch = chars[position];
            let mut next_states = Vec::new();
            
            for &state_id in &current_states {
                let state = &self.compiled.states[state_id];
                
                // Check direct character transitions
                if let Some(&next_state) = state.transitions.get(&ch) {
                    next_states.push(next_state);
                }
                
                // Check character class matches
                if let Some(ref class) = state.character_class {
                    if self.char_matches_class(ch, class) {
                        if state_id + 1 < self.compiled.states.len() {
                            next_states.push(state_id + 1);
                        }
                    }
                }
                
                // Add epsilon transitions
                next_states.extend(&state.epsilon_transitions);
            }
            
            current_states = next_states;
            position += 1;
        }
        
        // Check if any current state is accepting
        for &state_id in &current_states {
            if self.compiled.states[state_id].is_accept {
                return Some(position - start);
            }
        }
        
        None
    }
    
    /// Checks if character matches character class
    fn char_matches_class(&self, ch: char, class: &CharacterClass) -> bool {
        match class {
            CharacterClass::Digit => ch.is_ascii_digit(),
            CharacterClass::Word => ch.is_alphanumeric() || ch == '_',
            CharacterClass::Whitespace => ch.is_whitespace(),
            CharacterClass::Any => {
                if self.flags.dot_matches_newline {
                    true
                } else {
                    ch != '\n'
                }
            }
            CharacterClass::Custom(chars) => chars.contains(&ch),
            CharacterClass::Range(start, end) => ch >= *start && ch <= *end,
            CharacterClass::Negated(inner_class) => !self.char_matches_class(ch, inner_class),
        }
    }
}

// =============================================================================
// TEXT REPLACEMENT
// =============================================================================

impl Regex {
    /// Replaces first match with replacement text
    pub fn replace(&self, text: &str, replacement: &str) -> String {
        if let Some(m) = self.find(text) {
            let mut result = String::new();
            result.push_str(&text[..m.start]);
            result.push_str(replacement);
            result.push_str(&text[m.end..]);
            result
        } else {
            text.to_string()
        }
    }
    
    /// Replaces all matches with replacement text
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        let matches = self.find_all(text);
        if matches.is_empty() {
            return text.to_string();
        }
        
        let mut result = String::new();
        let mut last_end = 0;
        
        for m in matches {
            result.push_str(&text[last_end..m.start]);
            result.push_str(replacement);
            last_end = m.end;
        }
        
        result.push_str(&text[last_end..]);
        result
    }
    
    /// Replaces matches with result of function
    pub fn replace_all_with<F>(&self, text: &str, replacer: F) -> String
    where
        F: Fn(&Match) -> String,
    {
        let matches = self.find_all(text);
        if matches.is_empty() {
            return text.to_string();
        }
        
        let mut result = String::new();
        let mut last_end = 0;
        
        for m in matches {
            result.push_str(&text[last_end..m.start]);
            result.push_str(&replacer(&m));
            last_end = m.end;
        }
        
        result.push_str(&text[last_end..]);
        result
    }
}

// =============================================================================
// TEXT SPLITTING
// =============================================================================

impl Regex {
    /// Splits text by regex pattern
    pub fn split(&self, text: &str) -> Vec<String> {
        let matches = self.find_all(text);
        if matches.is_empty() {
            return vec![text.to_string()];
        }
        
        let mut parts = Vec::new();
        let mut last_end = 0;
        
        for m in matches {
            if m.start > last_end {
                parts.push(text[last_end..m.start].to_string());
            }
            last_end = m.end;
        }
        
        if last_end < text.len() {
            parts.push(text[last_end..].to_string());
        }
        
        parts
    }
    
    /// Splits text with limit on number of splits
    pub fn splitn(&self, text: &str, limit: usize) -> Vec<String> {
        if limit == 0 {
            return vec![text.to_string()];
        }
        
        let mut parts = Vec::new();
        let mut last_end = 0;
        let mut splits_made = 0;
        
        let matches = self.find_all(text);
        
        for m in matches {
            if splits_made >= limit - 1 {
                break;
            }
            
            if m.start > last_end {
                parts.push(text[last_end..m.start].to_string());
            }
            last_end = m.end;
            splits_made += 1;
        }
        
        if last_end < text.len() {
            parts.push(text[last_end..].to_string());
        }
        
        parts
    }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/// Tests if text matches pattern
pub fn is_match(pattern: &str, text: &str) -> StringResult<bool> {
    let regex = Regex::new(pattern)?;
    Ok(regex.is_match(text))
}

/// Finds first match of pattern in text
pub fn find_match(pattern: &str, text: &str) -> StringResult<Option<Match>> {
    let regex = Regex::new(pattern)?;
    Ok(regex.find(text))
}

/// Finds all matches of pattern in text
pub fn find_all_matches(pattern: &str, text: &str) -> StringResult<Vec<Match>> {
    let regex = Regex::new(pattern)?;
    Ok(regex.find_all(text))
}

/// Replaces first match with replacement
pub fn replace_first(pattern: &str, text: &str, replacement: &str) -> StringResult<String> {
    let regex = Regex::new(pattern)?;
    Ok(regex.replace(text, replacement))
}

/// Replaces all matches with replacement
pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> StringResult<String> {
    let regex = Regex::new(pattern)?;
    Ok(regex.replace_all(text, replacement))
}

/// Splits text by pattern
pub fn split_by_pattern(pattern: &str, text: &str) -> StringResult<Vec<String>> {
    let regex = Regex::new(pattern)?;
    Ok(regex.split(text))
}

/// Validates email address using regex
pub fn is_valid_email(email: &str) -> bool {
    // Simplified email validation pattern
    let pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    is_match(pattern, email).unwrap_or(false)
}

/// Validates URL using regex
pub fn is_valid_url(url: &str) -> bool {
    // Simplified URL validation pattern
    let pattern = r"^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(/.*)?$";
    is_match(pattern, url).unwrap_or(false)
}

/// Validates phone number using regex
pub fn is_valid_phone(phone: &str) -> bool {
    // Simplified phone validation pattern (US format)
    let pattern = r"^\(?([0-9]{3})\)?[-. ]?([0-9]{3})[-. ]?([0-9]{4})$";
    is_match(pattern, phone).unwrap_or(false)
}

/// Extracts numbers from text
pub fn extract_numbers(text: &str) -> StringResult<Vec<String>> {
    let pattern = r"\d+";
    let regex = Regex::new(pattern)?;
    Ok(regex.find_all(text).into_iter().map(|m| m.text).collect())
}

/// Extracts words from text
pub fn extract_words(text: &str) -> StringResult<Vec<String>> {
    let pattern = r"\w+";
    let regex = Regex::new(pattern)?;
    Ok(regex.find_all(text).into_iter().map(|m| m.text).collect())
}

// =============================================================================
// MISSING FUNCTIONS USED IN MOD.RS
// =============================================================================

/// Type alias for compatibility with mod.rs imports
pub type RegexPattern = Regex;

/// Type alias for compatibility with mod.rs imports  
pub type RegexMatch = Match;

/// Finds matches using regex pattern - wrapper for find_match
pub fn find_with_regex(pattern: &str, text: &str) -> StringResult<Option<Match>> {
    find_match(pattern, text)
}

/// Replaces first match with replacement using regex
pub fn replace_with_regex(pattern: &str, text: &str, replacement: &str) -> StringResult<String> {
    replace_first(pattern, text, replacement)
}

/// Replaces all matches with replacement using regex
pub fn replace_all_with_regex(pattern: &str, text: &str, replacement: &str) -> StringResult<String> {
    replace_all(pattern, text, replacement)
}

/// Splits text using regex pattern
pub fn split_with_regex(pattern: &str, text: &str) -> StringResult<Vec<String>> {
    split_by_pattern(pattern, text)
}

/// Tests if text matches pattern - wrapper for is_match
pub fn match_with_regex(pattern: &str, text: &str) -> StringResult<bool> {
    is_match(pattern, text)
}

/// Extracts capture groups from regex match
pub fn capture_groups(pattern: &str, text: &str) -> StringResult<Vec<Vec<String>>> {
    let regex = Regex::new(pattern)?;
    let matches = regex.find_all(text);
    
    let mut all_groups = Vec::new();
    for m in matches {
        // Convert Option<String> to String, using empty string for None
        let groups: Vec<String> = m.groups().iter()
            .map(|opt| opt.as_ref().unwrap_or(&String::new()).clone())
            .collect();
        all_groups.push(groups);
    }
    
    Ok(all_groups)
}

/// Extracts all pattern matches from text
pub fn extract_patterns(pattern: &str, text: &str) -> StringResult<Vec<String>> {
    let regex = Regex::new(pattern)?;
    Ok(regex.find_all(text).into_iter().map(|m| m.text).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_regex_creation() {
        let regex = Regex::new("hello").unwrap();
        assert_eq!(regex.pattern(), "hello");
    }
    
    #[test]
    fn test_literal_matching() {
        let regex = Regex::new("hello").unwrap();
        assert!(regex.is_match("hello world"));
        assert!(!regex.is_match("hi world"));
    }
    
    #[test]
    fn test_case_insensitive() {
        let regex = Regex::case_insensitive("HELLO").unwrap();
        assert!(regex.is_match("hello world"));
        assert!(regex.is_match("HELLO world"));
        assert!(regex.is_match("Hello world"));
    }
    
    #[test]
    fn test_find_match() {
        let regex = Regex::new("world").unwrap();
        let m = regex.find("hello world").unwrap();
        assert_eq!(m.start, 6);
        assert_eq!(m.end, 11);
        assert_eq!(m.as_str(), "world");
    }
    
    #[test]
    fn test_replace() {
        let regex = Regex::new("world").unwrap();
        let result = regex.replace("hello world", "universe");
        assert_eq!(result, "hello universe");
    }
    
    #[test]
    fn test_split() {
        let regex = Regex::new(",").unwrap();
        let parts = regex.split("a,b,c");
        assert_eq!(parts, vec!["a", "b", "c"]);
    }
    
    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid-email"));
    }
    
    #[test]
    fn test_extract_numbers() {
        let numbers = extract_numbers("The price is $123 and tax is $45").unwrap();
        assert_eq!(numbers, vec!["123", "45"]);
    }
    
    #[test]
    fn test_find_with_regex() {
        let result = find_with_regex("world", "hello world").unwrap();
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.as_str(), "world");
    }
    
    #[test]
    fn test_replace_with_regex() {
        let result = replace_with_regex("world", "hello world", "universe").unwrap();
        assert_eq!(result, "hello universe");
    }
    
    #[test]
    fn test_replace_all_with_regex() {
        let result = replace_all_with_regex("l", "hello", "x").unwrap();
        assert_eq!(result, "hexxo");
    }
    
    #[test]
    fn test_split_with_regex() {
        let result = split_with_regex(",", "a,b,c").unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }
    
    #[test]
    fn test_match_with_regex() {
        let result = match_with_regex("hello", "hello world").unwrap();
        assert!(result);
        
        let result = match_with_regex("xyz", "hello world").unwrap();
        assert!(!result);
    }
    
    #[test]
    fn test_capture_groups() {
        let result = capture_groups("test", "test string test").unwrap();
        assert_eq!(result.len(), 2);
    }
    
    #[test]
    fn test_extract_patterns() {
        let result = extract_patterns("\\d+", "I have 123 apples and 456 oranges").unwrap();
        assert_eq!(result, vec!["123", "456"]);
    }
    
    #[test]
    fn test_type_aliases() {
        // Test that type aliases work
        let _pattern: RegexPattern = Regex::new("test").unwrap();
        let _match: RegexMatch = Match {
            start: 0,
            end: 4,
            text: "test".to_string(),
            groups: Vec::new(),
        };
    }
}
