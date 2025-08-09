//! Advanced tab completion for CURSED REPL
//! Provides intelligent autocomplete for keywords, variables, functions, and modules

use crate::error::CursedError;
use std::collections::{HashMap, HashSet};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::Context;

/// Advanced tab completion engine for CURSED REPL
pub struct CursedCompleter {
    /// CURSED language keywords
    keywords: HashSet<String>,
    /// Built-in functions and types
    builtins: HashSet<String>,
    /// User-defined variables
    variables: HashMap<String, String>,
    /// User-defined functions
    functions: HashMap<String, Vec<String>>, // function_name -> parameter_names
    /// Available modules
    modules: HashSet<String>,
    /// File completion for imports
    file_completer: FilenameCompleter,
}

impl CursedCompleter {
    pub fn new() -> Self {
        let mut keywords = HashSet::new();
        
        // Core CURSED keywords
        keywords.insert("sus".to_string());        // Variable declaration
        keywords.insert("slay".to_string());       // Function definition
        keywords.insert("damn".to_string());       // Return statement
        keywords.insert("vibez".to_string());      // Namespace for I/O
        keywords.insert("yeet".to_string());       // Import statement
        keywords.insert("ready".to_string());      // If statement
        keywords.insert("otherwise".to_string());  // Else statement
        keywords.insert("bestie".to_string());     // While loop
        keywords.insert("squad".to_string());      // Struct definition
        keywords.insert("collab".to_string());     // Interface definition
        keywords.insert("periodt".to_string());    // End statement
        keywords.insert("based".to_string());      // True literal
        keywords.insert("cringe".to_string());     // False literal
        keywords.insert("nocap".to_string());      // Assert statement
        keywords.insert("lowkey".to_string());     // Maybe/optional
        keywords.insert("highkey".to_string());    // Definitely/required
        keywords.insert("deadass".to_string());    // Serious/final
        keywords.insert("onfr".to_string());       // On fire/error handling
        keywords.insert("bet".to_string());        // Confirmation/OK
        keywords.insert("finna".to_string());      // Going to/future action
        keywords.insert("stan".to_string());       // Support/implement
        
        // Type keywords
        keywords.insert("drip".to_string());       // Integer type
        keywords.insert("tea".to_string());        // String type
        keywords.insert("lit".to_string());        // Boolean type
        keywords.insert("flex".to_string());       // Float type
        keywords.insert("vibe".to_string());       // Any type
        
        let mut builtins = HashSet::new();
        
        // Built-in functions
        builtins.insert("spill".to_string());      // Print function
        builtins.insert("len".to_string());        // Length function
        builtins.insert("push".to_string());       // Array push
        builtins.insert("pop".to_string());        // Array pop
        builtins.insert("slice".to_string());      // String/array slice
        
        let mut modules = HashSet::new();
        
        // Standard library modules
        modules.insert("mathz".to_string());       // Math functions
        modules.insert("stringz".to_string());     // String utilities
        modules.insert("arrayz".to_string());      // Array utilities
        modules.insert("testz".to_string());       // Testing framework
        modules.insert("cryptz".to_string());      // Cryptography
        modules.insert("timez".to_string());       // Time functions
        modules.insert("filez".to_string());       // File I/O
        modules.insert("netz".to_string());        // Network utilities
        modules.insert("jsonz".to_string());       // JSON parsing
        modules.insert("regexz".to_string());      // Regular expressions
        
        Self {
            keywords,
            builtins,
            variables: HashMap::new(),
            functions: HashMap::new(),
            modules,
            file_completer: FilenameCompleter::new(),
        }
    }
    
    /// Update the list of user-defined variables
    pub fn update_variables(&mut self, variables: &HashMap<String, String>) {
        self.variables = variables.clone();
    }
    
    /// Add a user-defined function
    pub fn add_function(&mut self, name: String, parameters: Vec<String>) {
        self.functions.insert(name, parameters);
    }
    
    /// Remove a function
    pub fn remove_function(&mut self, name: &str) {
        self.functions.remove(name);
    }
    
    /// Get completions for a given input
    pub fn get_completions(&self, input: &str) -> Vec<String> {
        let mut completions = Vec::new();
        let input_lower = input.to_lowercase();
        
        // Complete keywords
        for keyword in &self.keywords {
            if keyword.starts_with(&input_lower) {
                completions.push(keyword.clone());
            }
        }
        
        // Complete built-in functions
        for builtin in &self.builtins {
            if builtin.starts_with(&input_lower) {
                completions.push(format!("{}(", builtin));
            }
        }
        
        // Complete variables
        for var_name in self.variables.keys() {
            if var_name.to_lowercase().starts_with(&input_lower) {
                completions.push(var_name.clone());
            }
        }
        
        // Complete user-defined functions
        for (func_name, params) in &self.functions {
            if func_name.to_lowercase().starts_with(&input_lower) {
                let param_hints = params.join(", ");
                completions.push(format!("{}({})", func_name, param_hints));
            }
        }
        
        // Complete modules for import statements
        if input.contains("yeet") {
            for module in &self.modules {
                let suggestion = format!("yeet \"{}\"", module);
                if suggestion.to_lowercase().contains(&input_lower) {
                    completions.push(suggestion);
                }
            }
        }
        
        // Special context-aware completions
        completions.extend(self.get_context_completions(input));
        
        completions.sort();
        completions.dedup();
        completions
    }
    
    /// Get context-aware completions based on the current input
    fn get_context_completions(&self, input: &str) -> Vec<String> {
        let mut completions = Vec::new();
        let trimmed = input.trim();
        
        // Variable declaration completions
        if trimmed.starts_with("sus ") && !trimmed.contains('=') {
            completions.push("sus var_name drip = ".to_string());
            completions.push("sus var_name tea = ".to_string());
            completions.push("sus var_name lit = ".to_string());
            completions.push("sus var_name []drip = ".to_string());
        }
        
        // Function definition completions
        if trimmed.starts_with("slay ") && !trimmed.contains('{') {
            completions.push("slay func_name() drip { }".to_string());
            completions.push("slay func_name(param drip) drip { }".to_string());
            completions.push("slay func_name(a drip, b drip) drip { }".to_string());
        }
        
        // Control structure completions
        if trimmed.starts_with("ready ") {
            completions.push("ready (condition) { }".to_string());
            completions.push("ready (condition) { } otherwise { }".to_string());
        }
        
        if trimmed.starts_with("bestie ") {
            completions.push("bestie (condition) { }".to_string());
        }
        
        // Import completions
        if trimmed.starts_with("yeet ") {
            for module in &self.modules {
                completions.push(format!("yeet \"{}\"", module));
            }
        }
        
        // Vibez.spill completions
        if trimmed.contains("vibez.") {
            completions.push("vibez.spill()".to_string());
            completions.push("vibez.spill(\"\")".to_string());
        }
        
        completions
    }
    
    /// Get help text for a specific completion
    pub fn get_completion_help(&self, completion: &str) -> Option<String> {
        // Remove function parentheses for lookup
        let clean_name = completion.split('(').next().unwrap_or(completion);
        
        match clean_name {
            // Keywords
            "sus" => Some("Variable declaration: sus var_name type = value".to_string()),
            "slay" => Some("Function definition: slay func_name(params) return_type { body }".to_string()),
            "damn" => Some("Return statement: damn value".to_string()),
            "yeet" => Some("Import module: yeet \"module_name\"".to_string()),
            "ready" => Some("If statement: ready (condition) { body }".to_string()),
            "bestie" => Some("While loop: bestie (condition) { body }".to_string()),
            
            // Types
            "drip" => Some("Integer type".to_string()),
            "tea" => Some("String type".to_string()),
            "lit" => Some("Boolean type".to_string()),
            "flex" => Some("Float type".to_string()),
            
            // Built-ins
            "spill" => Some("Print function: vibez.spill(value)".to_string()),
            "len" => Some("Get length: len(array_or_string)".to_string()),
            
            // Modules
            "mathz" => Some("Math functions: abs, sqrt, pow, etc.".to_string()),
            "stringz" => Some("String utilities: slice, concat, split, etc.".to_string()),
            "arrayz" => Some("Array utilities: push, pop, filter, map, etc.".to_string()),
            "testz" => Some("Testing framework: assert, test_start, etc.".to_string()),
            "cryptz" => Some("Cryptography: hash, encrypt, decrypt, etc.".to_string()),
            
            _ => {
                // Check if it's a user-defined function
                if let Some(params) = self.functions.get(clean_name) {
                    Some(format!("User function: {}({})", clean_name, params.join(", ")))
                } else if self.variables.contains_key(clean_name) {
                    Some(format!("Variable: {} = {}", clean_name, 
                        self.variables.get(clean_name).unwrap_or(&"?".to_string())))
                } else {
                    None
                }
            }
        }
    }
}

impl Completer for CursedCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // Find the word boundary
        let start = line[..pos].rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);
        
        let word = &line[start..pos];
        
        // Handle file completion for imports
        if line.contains("yeet") && line.contains('"') {
            return self.file_completer.complete(line, pos, _ctx);
        }
        
        let completions = self.get_completions(word);
        
        let pairs: Vec<Pair> = completions
            .into_iter()
            .map(|completion| {
                let help = self.get_completion_help(&completion)
                    .unwrap_or_else(|| "".to_string());
                Pair {
                    display: completion.clone(),
                    replacement: completion,
                }
            })
            .collect();
        
        Ok((start, pairs))
    }
}

/// Helper function to create a configured CURSED completer
pub fn create_cursed_completer() -> Result<CursedCompleter, CursedError> {
    Ok(CursedCompleter::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_completion() {
        let completer = CursedCompleter::new();
        let completions = completer.get_completions("su");
        assert!(completions.contains(&"sus".to_string()));
    }

    #[test]
    fn test_function_completion() {
        let mut completer = CursedCompleter::new();
        completer.add_function("test_func".to_string(), vec!["a".to_string(), "b".to_string()]);
        
        let completions = completer.get_completions("test");
        assert!(completions.iter().any(|c| c.starts_with("test_func")));
    }

    #[test]
    fn test_variable_completion() {
        let mut completer = CursedCompleter::new();
        let mut vars = HashMap::new();
        vars.insert("my_variable".to_string(), "42".to_string());
        completer.update_variables(&vars);
        
        let completions = completer.get_completions("my_");
        assert!(completions.contains(&"my_variable".to_string()));
    }

    #[test]
    fn test_context_completion() {
        let completer = CursedCompleter::new();
        let completions = completer.get_context_completions("sus ");
        assert!(completions.iter().any(|c| c.contains("drip")));
    }
}
