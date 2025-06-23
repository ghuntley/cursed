//! Tab Completion for CURSED REPL
//! 
//! Provides intelligent tab completion for CURSED keywords,
//! variables, functions, and file paths.

use std::collections::HashSet;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Context, Helper};
use rustyline::line_buffer::LineBuffer;
use once_cell::sync::Lazy;

/// CURSED keywords for completion
static CURSED_KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        // Gen Z slang keywords
        "slay", "yolo", "sus", "facts", "lowkey", "highkey", "periodt",
        "bestie", "flex", "vibe_check", "mood", "basic", "no_cap",
        "straight_fire", "it_hits_different", "absolute_unit", "send_it",
        "that_slaps", "bussin", "fire", "slaps", "bet", "say_less",
        "cap", "no_printer", "frfr", "ong", "sheesh", "valid", "slick",
        
        // Traditional keywords
        "func", "var", "const", "type", "interface", "struct", "enum",
        "if", "else", "for", "while", "switch", "case", "default",
        "break", "continue", "return", "defer", "go", "chan", "select",
        "package", "import", "nil", "true", "false",
        
        // CURSED-specific
        "squad", "collab", "vibes", "energy", "aura", "rizz",
        "main_character", "side_character", "npc",
    ].into_iter().collect()
});

/// CURSED built-in types for completion
static CURSED_TYPES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "int", "int8", "int16", "int32", "int64",
        "uint", "uint8", "uint16", "uint32", "uint64", 
        "float32", "float64", "bool", "string", "byte", "rune",
        "error", "interface{}", "chan", "map", "slice",
        "Energy", "Vibes", "Aura", "Rizz",
    ].into_iter().collect()
});

/// CURSED built-in functions for completion
static CURSED_BUILTINS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "print", "println", "printf", "len", "cap", "make", "new",
        "append", "copy", "delete", "close", "panic", "recover",
        "real", "imag", "complex", "min", "max",
    ].into_iter().collect()
});

/// REPL commands for completion
static REPL_COMMANDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        ":help", ":exit", ":quit", ":load", ":save", ":clear",
        ":history", ":type", ":build", ":test", ":fmt", ":lint",
        ":vars", ":funcs", ":info", ":workspace",
    ].into_iter().collect()
});

/// Tab completion helper for CURSED REPL
pub struct TabCompletion {
    file_completer: FilenameCompleter,
    bracket_highlighter: MatchingBracketHighlighter,
    bracket_validator: MatchingBracketValidator,
    history_hinter: HistoryHinter,
    session_variables: HashSet<String>,
    session_functions: HashSet<String>,
}

impl TabCompletion {
    /// Create a new tab completion helper
    pub fn new() -> Self {
        Self {
            file_completer: FilenameCompleter::new(),
            bracket_highlighter: MatchingBracketHighlighter::new(),
            bracket_validator: MatchingBracketValidator::new(),
            history_hinter: HistoryHinter {},
            session_variables: HashSet::new(),
            session_functions: HashSet::new(),
        }
    }

    /// Update session variables for completion
    pub fn update_variables(&mut self, variables: Vec<String>) {
        self.session_variables = variables.into_iter().collect();
    }

    /// Update session functions for completion
    pub fn update_functions(&mut self, functions: Vec<String>) {
        self.session_functions = functions.into_iter().collect();
    }

    /// Complete CURSED identifiers
    fn complete_identifier(&self, word: &str) -> Vec<String> {
        let mut completions = Vec::new();
        
        // Add matching keywords
        for &keyword in CURSED_KEYWORDS.iter() {
            if keyword.starts_with(word) {
                completions.push(keyword.to_string());
            }
        }
        
        // Add matching types
        for &type_name in CURSED_TYPES.iter() {
            if type_name.starts_with(word) {
                completions.push(type_name.to_string());
            }
        }
        
        // Add matching built-in functions
        for &builtin in CURSED_BUILTINS.iter() {
            if builtin.starts_with(word) {
                completions.push(format!("{}(", builtin));
            }
        }
        
        // Add matching session variables
        for var in &self.session_variables {
            if var.starts_with(word) {
                completions.push(var.clone());
            }
        }
        
        // Add matching session functions
        for func in &self.session_functions {
            if func.starts_with(word) {
                completions.push(format!("{}(", func));
            }
        }
        
        completions.sort();
        completions.dedup();
        completions
    }

    /// Complete REPL commands
    fn complete_command(&self, word: &str) -> Vec<String> {
        let mut completions = Vec::new();
        
        for &command in REPL_COMMANDS.iter() {
            if command.starts_with(word) {
                completions.push(command.to_string());
            }
        }
        
        completions
    }

    /// Get context-aware completions
    fn get_completions(&self, line: &str, pos: usize) -> Vec<String> {
        let before_cursor = &line[..pos];
        let words: Vec<&str> = before_cursor.split_whitespace().collect();
        
        if let Some(last_word) = words.last() {
            if last_word.starts_with(':') {
                // REPL command completion
                self.complete_command(last_word)
            } else if before_cursor.contains(":load ") || before_cursor.contains(":save ") {
                // File path completion
                match self.file_completer.complete_path(last_word, pos) {
                    Ok((_, completions)) => completions.into_iter().map(|p| p.replacement).collect(),
                    Err(_) => Vec::new(),
                }
            } else {
                // Identifier completion
                self.complete_identifier(last_word)
            }
        } else {
            Vec::new()
        }
    }
}

impl Helper for TabCompletion {}

impl Completer for TabCompletion {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(), Error> {
        // Find the word boundary
        let mut start = pos;
        while start > 0 {
            let ch = line.chars().nth(start - 1).unwrap_or(' ');
            if ch.is_whitespace() || "(){}[],.;:".contains(ch) {
                break;
            }
            start -= 1;
        }

        let word = &line[start..pos];
        let completions = self.get_completions(line, pos);
        
        let pairs: Vec<Pair> = completions
            .into_iter()
            .map(|completion| Pair {
                display: completion.clone(),
                replacement: completion,
            })
            .collect();

        Ok((start, pairs))
    }


}

impl Hinter for TabCompletion {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.history_hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for TabCompletion {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        if default {
            std::borrow::Cow::Borrowed("🔥 ")
        } else {
            std::borrow::Cow::Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Owned(format!("\x1b[90m{}\x1b[0m", hint))
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.bracket_highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.bracket_highlighter.highlight_char(line, pos)
    }
}

impl Validator for TabCompletion {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.bracket_validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.bracket_validator.validate_while_typing()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_completion() {
        let completer = TabCompletion::new();
        let completions = completer.complete_identifier("sl");
        
        assert!(completions.contains(&"slay".to_string()));
        assert!(completions.contains(&"slaps".to_string()));
        assert!(completions.contains(&"slick".to_string()));
    }

    #[test]
    fn test_type_completion() {
        let completer = TabCompletion::new();
        let completions = completer.complete_identifier("int");
        
        assert!(completions.contains(&"int".to_string()));
        assert!(completions.contains(&"int8".to_string()));
        assert!(completions.contains(&"int16".to_string()));
        assert!(completions.contains(&"int32".to_string()));
        assert!(completions.contains(&"int64".to_string()));
    }

    #[test]
    fn test_builtin_completion() {
        let completer = TabCompletion::new();
        let completions = completer.complete_identifier("pr");
        
        assert!(completions.contains(&"print(".to_string()));
        assert!(completions.contains(&"println(".to_string()));
        assert!(completions.contains(&"printf(".to_string()));
    }

    #[test]
    fn test_command_completion() {
        let completer = TabCompletion::new();
        let completions = completer.complete_command(":h");
        
        assert!(completions.contains(&":help".to_string()));
        assert!(completions.contains(&":history".to_string()));
    }

    #[test]
    fn test_session_variable_completion() {
        let mut completer = TabCompletion::new();
        completer.update_variables(Vec::from(["my_var".to_string(), "my_other_var".to_string()]));
        
        let completions = completer.complete_identifier("my");
        assert!(completions.contains(&"my_var".to_string()));
        assert!(completions.contains(&"my_other_var".to_string()));
    }

    #[test]
    fn test_session_function_completion() {
        let mut completer = TabCompletion::new();
        completer.update_functions(Vec::from(["my_func".to_string(), "helper_func".to_string()]));
        
        let completions = completer.complete_identifier("my");
        assert!(completions.contains(&"my_func(".to_string()));
        
        let completions = completer.complete_identifier("help");
        assert!(completions.contains(&"helper_func(".to_string()));
    }
}
