//! Auto-completion provider for CURSED language server
//! 
//! Provides intelligent code completion for keywords, variables, functions, types, etc.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};

use crate::lexer::{Lexer, TokenType};

/// Completion provider for the LSP server
pub struct CompletionProvider {
    /// Cache for completion items to improve performance
    completion_cache: std::sync::RwLock<HashMap<String, Vec<CompletionItem>>>,
}

impl CompletionProvider {
    /// Create a new completion provider
    pub fn new() -> Self {
        Self {
            completion_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Get completions at a specific position
    #[instrument(skip(self, content))]
    pub async fn get_completions(&self, content: &str, position: Position) -> Vec<CompletionItem> {
        debug!("Getting completions at {:?}", position);
        
        let mut completions = Vec::new();
        
        // Get context at cursor position
        let context = self.get_completion_context(content, position);
        
        match context.completion_type {
            CompletionType::Keyword => {
                completions.extend(self.get_keyword_completions(&context));
            }
            CompletionType::Variable => {
                completions.extend(self.get_variable_completions(content, &context));
            }
            CompletionType::Function => {
                completions.extend(self.get_function_completions(content, &context));
            }
            CompletionType::Type => {
                completions.extend(self.get_type_completions(&context));
            }
            CompletionType::Member => {
                completions.extend(self.get_member_completions(content, &context));
            }
            CompletionType::Import => {
                completions.extend(self.get_import_completions(&context));
            }
            CompletionType::Snippet => {
                completions.extend(self.get_snippet_completions(&context));
            }
            CompletionType::Generic => {
                // Provide all types of completions
                completions.extend(self.get_keyword_completions(&context));
                completions.extend(self.get_variable_completions(content, &context));
                completions.extend(self.get_function_completions(content, &context));
                completions.extend(self.get_type_completions(&context));
            }
        }

        // Sort completions by relevance
        completions.sort_by(|a, b| {
            // Prioritize by sort_text, then by label
            match (&a.sort_text, &b.sort_text) {
                (Some(a_sort), Some(b_sort)) => a_sort.cmp(b_sort),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.label.cmp(&b.label),
            }
        });

        completions
    }

    /// Get completion context at cursor position
    fn get_completion_context(&self, content: &str, position: Position) -> CompletionContext {
        let lines: Vec<&str> = content.lines().collect();
        let line_index = position.line as usize;
        let char_index = position.character as usize;

        if line_index >= lines.len() {
            return CompletionContext::default();
        }

        let line = lines[line_index];
        let before_cursor = if char_index <= line.len() {
            &line[..char_index]
        } else {
            line
        };

        // Determine completion type based on context
        let completion_type = if before_cursor.trim_end().ends_with('.') {
            CompletionType::Member
        } else if before_cursor.contains("use ") || before_cursor.contains("import ") {
            CompletionType::Import
        } else if before_cursor.contains(": ") || before_cursor.contains("-> ") {
            CompletionType::Type
        } else if before_cursor.trim_end().ends_with('(') {
            CompletionType::Function
        } else if self.is_in_keyword_context(before_cursor) {
            CompletionType::Keyword
        } else if self.is_in_variable_context(before_cursor) {
            CompletionType::Variable
        } else {
            CompletionType::Generic
        };

        let word_start = self.find_word_start(before_cursor);
        let prefix = if word_start < before_cursor.len() {
            before_cursor[word_start..].to_string()
        } else {
            String::new()
        };

        CompletionContext {
            completion_type,
            prefix,
            line: line.to_string(),
            position,
            before_cursor: before_cursor.to_string(),
            after_cursor: line[char_index.min(line.len())..].to_string(),
        }
    }

    /// Get keyword completions
    fn get_keyword_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let keywords = vec![
            // Function keywords
            ("slay", "Function declaration", CompletionItemKind::KEYWORD),
            ("yolo", "Async function declaration", CompletionItemKind::KEYWORD),
            
            // Variable keywords
            ("facts", "Immutable variable declaration", CompletionItemKind::KEYWORD),
            ("sus", "Mutable variable declaration", CompletionItemKind::KEYWORD),
            
            // Control flow keywords
            ("lowkey", "If statement", CompletionItemKind::KEYWORD),
            ("highkey", "Else statement", CompletionItemKind::KEYWORD),
            ("periodt", "Loop statement", CompletionItemKind::KEYWORD),
            ("bestie", "For loop", CompletionItemKind::KEYWORD),
            ("flex", "While loop", CompletionItemKind::KEYWORD),
            
            // Type keywords
            ("squad", "Struct declaration", CompletionItemKind::KEYWORD),
            ("collab", "Interface declaration", CompletionItemKind::KEYWORD),
            ("vibes", "Enum declaration", CompletionItemKind::KEYWORD),
            
            // Control keywords
            ("bounce", "Return statement", CompletionItemKind::KEYWORD),
            ("yeet", "Throw/panic statement", CompletionItemKind::KEYWORD),
            ("catch", "Error handling", CompletionItemKind::KEYWORD),
            ("finally", "Cleanup block", CompletionItemKind::KEYWORD),
            
            // Switch statement keywords
            ("vibe_check", "Switch statement", CompletionItemKind::KEYWORD),
            ("mood", "Case statement", CompletionItemKind::KEYWORD),
            ("basic", "Default case", CompletionItemKind::KEYWORD),
            
            // Import keywords
            ("use", "Import statement", CompletionItemKind::KEYWORD),
            ("from", "Import from module", CompletionItemKind::KEYWORD),
            
            // Visibility keywords
            ("public", "Public visibility", CompletionItemKind::KEYWORD),
            ("private", "Private visibility", CompletionItemKind::KEYWORD),
            
            // Async keywords
            ("await", "Await async operation", CompletionItemKind::KEYWORD),
            ("async", "Async modifier", CompletionItemKind::KEYWORD),
            
            // Channel keywords
            ("chan", "Channel type", CompletionItemKind::KEYWORD),
            ("send", "Send to channel", CompletionItemKind::KEYWORD),
            ("recv", "Receive from channel", CompletionItemKind::KEYWORD),
            
            // Generic keywords
            ("where", "Generic constraints", CompletionItemKind::KEYWORD),
        ];

        keywords
            .into_iter()
            .filter(|(keyword, _, _)| {
                context.prefix.is_empty() || keyword.starts_with(&context.prefix.to_lowercase())
            })
            .map(|(keyword, description, kind)| {
                CompletionItem {
                    label: keyword.to_string(),
                    kind: Some(kind),
                    detail: Some(description.to_string()),
                    documentation: Some(Documentation::String(format!(
                        "CURSED keyword: {}",
                        description
                    ))),
                    insert_text: Some(keyword.to_string()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    sort_text: Some(format!("0_{}", keyword)), // Prioritize keywords
                    filter_text: Some(keyword.to_string()),
                    ..CompletionItem::default()
                }
            })
            .collect()
    }

    /// Get variable completions
    fn get_variable_completions(&self, content: &str, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Find variable declarations
        for line in lines {
            if let Some(var_name) = self.extract_variable_name(line) {
                if context.prefix.is_empty() || var_name.starts_with(&context.prefix) {
                    let var_type = self.extract_variable_type(line).unwrap_or_else(|| "unknown".to_string());
                    
                    completions.push(CompletionItem {
                        label: var_name.clone(),
                        kind: Some(CompletionItemKind::VARIABLE),
                        detail: Some(format!("Variable: {}", var_type)),
                        documentation: Some(Documentation::String(format!(
                            "Variable {} of type {}",
                            var_name, var_type
                        ))),
                        insert_text: Some(var_name.clone()),
                        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                        sort_text: Some(format!("1_{}", var_name)), // Lower priority than keywords
                        filter_text: Some(var_name),
                        ..CompletionItem::default()
                    });
                }
            }
        }

        completions
    }

    /// Get function completions
    fn get_function_completions(&self, content: &str, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Add built-in functions
        let builtins = vec![
            ("print", "print(value)", "Print value to stdout"),
            ("println", "println(value)", "Print value with newline"),
            ("len", "len(collection)", "Get length of collection"),
            ("str", "str(value)", "Convert value to string"),
            ("int", "int(value)", "Convert value to integer"),
            ("float", "float(value)", "Convert value to float"),
            ("type", "type(value)", "Get type of value"),
            ("panic", "panic(message)", "Panic with message"),
            ("spawn", "spawn(function)", "Spawn goroutine"),
            ("make", "make(type, size)", "Create collection"),
        ];

        for (name, signature, description) in builtins {
            if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                completions.push(CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(signature.to_string()),
                    documentation: Some(Documentation::String(description.to_string())),
                    insert_text: Some(format!("{}($0)", name)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    sort_text: Some(format!("2_{}", name)),
                    filter_text: Some(name.to_string()),
                    ..CompletionItem::default()
                });
            }
        }
        
        // Find user-defined functions
        for line in lines {
            if let Some((func_name, params, return_type)) = self.extract_function_signature(line) {
                if context.prefix.is_empty() || func_name.starts_with(&context.prefix) {
                    let signature = format!("{}({})", func_name, params);
                    let detail = if !return_type.is_empty() {
                        format!("{} -> {}", signature, return_type)
                    } else {
                        signature.clone()
                    };
                    
                    completions.push(CompletionItem {
                        label: func_name.clone(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some(detail),
                        documentation: Some(Documentation::String(format!(
                            "User-defined function: {}",
                            signature
                        ))),
                        insert_text: Some(format!("{}($0)", func_name)),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        sort_text: Some(format!("3_{}", func_name)),
                        filter_text: Some(func_name),
                        ..CompletionItem::default()
                    });
                }
            }
        }

        completions
    }

    /// Get type completions
    fn get_type_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let types = vec![
            // Primitive types
            ("string", "String type"),
            ("int", "Integer type"),
            ("float", "Floating point type"),
            ("bool", "Boolean type"),
            ("char", "Character type"),
            
            // Collection types
            ("array", "Array type"),
            ("slice", "Slice type"),
            ("map", "Map/dictionary type"),
            ("set", "Set type"),
            
            // Special types
            ("chan", "Channel type"),
            ("interface", "Interface type"),
            ("any", "Any type"),
            ("nil", "Nil type"),
            
            // Generic types
            ("Vec", "Vector type"),
            ("HashMap", "Hash map type"),
            ("Option", "Optional type"),
            ("Result", "Result type"),
        ];

        types
            .into_iter()
            .filter(|(type_name, _)| {
                context.prefix.is_empty() || type_name.starts_with(&context.prefix)
            })
            .map(|(type_name, description)| {
                CompletionItem {
                    label: type_name.to_string(),
                    kind: Some(CompletionItemKind::TYPE_PARAMETER),
                    detail: Some(description.to_string()),
                    documentation: Some(Documentation::String(format!(
                        "Type: {}",
                        description
                    ))),
                    insert_text: Some(type_name.to_string()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    sort_text: Some(format!("4_{}", type_name)),
                    filter_text: Some(type_name.to_string()),
                    ..CompletionItem::default()
                }
            })
            .collect()
    }

    /// Get member completions (for dot notation)
    fn get_member_completions(&self, _content: &str, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Generic member completions
        let members = vec![
            ("length", "Get length", CompletionItemKind::PROPERTY),
            ("size", "Get size", CompletionItemKind::PROPERTY),
            ("clone", "Clone object", CompletionItemKind::METHOD),
            ("toString", "Convert to string", CompletionItemKind::METHOD),
            ("isEmpty", "Check if empty", CompletionItemKind::METHOD),
            ("contains", "Check if contains item", CompletionItemKind::METHOD),
            ("add", "Add item", CompletionItemKind::METHOD),
            ("remove", "Remove item", CompletionItemKind::METHOD),
            ("clear", "Clear all items", CompletionItemKind::METHOD),
        ];

        for (name, description, kind) in members {
            if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                completions.push(CompletionItem {
                    label: name.to_string(),
                    kind: Some(kind),
                    detail: Some(description.to_string()),
                    documentation: Some(Documentation::String(description.to_string())),
                    insert_text: Some(if kind == CompletionItemKind::METHOD {
                        format!("{}($0)", name)
                    } else {
                        name.to_string()
                    }),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    sort_text: Some(format!("5_{}", name)),
                    filter_text: Some(name.to_string()),
                    ..CompletionItem::default()
                });
            }
        }

        completions
    }

    /// Get import completions
    fn get_import_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let modules = vec![
            ("std", "Standard library"),
            ("fmt", "Formatting utilities"),
            ("io", "Input/output operations"),
            ("net", "Network operations"),
            ("http", "HTTP client/server"),
            ("json", "JSON parsing/serialization"),
            ("crypto", "Cryptographic functions"),
            ("regex", "Regular expressions"),
            ("time", "Time and date utilities"),
            ("math", "Mathematical functions"),
            ("collections", "Collection utilities"),
            ("sync", "Synchronization primitives"),
            ("async", "Async utilities"),
        ];

        modules
            .into_iter()
            .filter(|(module, _)| {
                context.prefix.is_empty() || module.starts_with(&context.prefix)
            })
            .map(|(module, description)| {
                CompletionItem {
                    label: module.to_string(),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: Some(description.to_string()),
                    documentation: Some(Documentation::String(format!(
                        "Module: {}",
                        description
                    ))),
                    insert_text: Some(module.to_string()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    sort_text: Some(format!("6_{}", module)),
                    filter_text: Some(module.to_string()),
                    ..CompletionItem::default()
                }
            })
            .collect()
    }

    /// Get snippet completions
    fn get_snippet_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let snippets = vec![
            (
                "main",
                "slay main() {\n    $0\n}",
                "Main function template",
            ),
            (
                "func",
                "slay ${1:function_name}(${2:params}) -> ${3:return_type} {\n    $0\n}",
                "Function template",
            ),
            (
                "struct",
                "squad ${1:StructName} {\n    ${2:field}: ${3:type},\n    $0\n}",
                "Struct template",
            ),
            (
                "interface",
                "collab ${1:InterfaceName} {\n    ${2:method}(${3:params}) -> ${4:return_type}\n    $0\n}",
                "Interface template",
            ),
            (
                "if",
                "lowkey ${1:condition} {\n    $0\n}",
                "If statement",
            ),
            (
                "ifelse",
                "lowkey ${1:condition} {\n    ${2:// if body}\n} highkey {\n    ${3:// else body}\n    $0\n}",
                "If-else statement",
            ),
            (
                "for",
                "bestie ${1:item} in ${2:collection} {\n    $0\n}",
                "For loop",
            ),
            (
                "while",
                "flex ${1:condition} {\n    $0\n}",
                "While loop",
            ),
            (
                "switch",
                "vibe_check ${1:value} {\n    mood ${2:case1}:\n        ${3:// case body}\n    basic:\n        ${4:// default}\n        $0\n}",
                "Switch statement",
            ),
        ];

        snippets
            .into_iter()
            .filter(|(name, _, _)| {
                context.prefix.is_empty() || name.starts_with(&context.prefix)
            })
            .map(|(name, snippet, description)| {
                CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::SNIPPET),
                    detail: Some(description.to_string()),
                    documentation: Some(Documentation::String(description.to_string())),
                    insert_text: Some(snippet.to_string()),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    sort_text: Some(format!("7_{}", name)),
                    filter_text: Some(name.to_string()),
                    ..CompletionItem::default()
                }
            })
            .collect()
    }

    /// Helper methods

    fn is_in_keyword_context(&self, text: &str) -> bool {
        let text = text.trim();
        text.is_empty() || text.ends_with('{') || text.ends_with(';') || text.ends_with('\n')
    }

    fn is_in_variable_context(&self, text: &str) -> bool {
        // Check if we're in a context where variables are expected
        text.contains('=') || text.contains('(') || text.contains(',')
    }

    fn find_word_start(&self, text: &str) -> usize {
        let chars: Vec<char> = text.chars().collect();
        let mut pos = chars.len();
        
        while pos > 0 {
            let ch = chars[pos - 1];
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            pos -= 1;
        }
        
        pos
    }

    fn extract_variable_name(&self, line: &str) -> Option<String> {
        if let Some(facts_pos) = line.find("facts").or_else(|| line.find("sus")) {
            let keyword_len = if line[facts_pos..].starts_with("facts") { 5 } else { 3 };
            let after_keyword = &line[facts_pos + keyword_len..];
            if let Some(equals_pos) = after_keyword.find('=') {
                let var_part = after_keyword[..equals_pos].trim();
                // Handle type annotations
                if let Some(colon_pos) = var_part.find(':') {
                    return Some(var_part[..colon_pos].trim().to_string());
                } else {
                    return Some(var_part.to_string());
                }
            }
        }
        None
    }

    fn extract_variable_type(&self, line: &str) -> Option<String> {
        if let Some(colon_pos) = line.find(':') {
            if let Some(equals_pos) = line.find('=') {
                if colon_pos < equals_pos {
                    let type_part = line[colon_pos + 1..equals_pos].trim();
                    return Some(type_part.to_string());
                }
            }
        }
        // Try to infer type from value
        if line.contains("= \"") {
            Some("string".to_string())
        } else if line.contains("= true") || line.contains("= false") {
            Some("bool".to_string())
        } else if line.contains("= ") && line.chars().any(|c| c.is_ascii_digit()) {
            Some("int".to_string())
        } else {
            None
        }
    }

    fn extract_function_signature(&self, line: &str) -> Option<(String, String, String)> {
        if line.contains("slay") || line.contains("yolo") {
            if let Some(paren_start) = line.find('(') {
                if let Some(paren_end) = line.find(')') {
                    let before_paren = &line[..paren_start];
                    let func_name = before_paren
                        .split_whitespace()
                        .last()?
                        .to_string();
                    
                    let params = line[paren_start + 1..paren_end].to_string();
                    
                    let return_type = if let Some(arrow_pos) = line.find("->") {
                        line[arrow_pos + 2..].split('{').next()?.trim().to_string()
                    } else {
                        String::new()
                    };
                    
                    return Some((func_name, params, return_type));
                }
            }
        }
        None
    }
}

/// Completion context information
#[derive(Debug, Clone)]
struct CompletionContext {
    completion_type: CompletionType,
    prefix: String,
    line: String,
    position: Position,
    before_cursor: String,
    after_cursor: String,
}

impl Default for CompletionContext {
    fn default() -> Self {
        Self {
            completion_type: CompletionType::Generic,
            prefix: String::new(),
            line: String::new(),
            position: Position { line: 0, character: 0 },
            before_cursor: String::new(),
            after_cursor: String::new(),
        }
    }
}

/// Type of completion to provide
#[derive(Debug, Clone, PartialEq)]
enum CompletionType {
    Keyword,
    Variable,
    Function,
    Type,
    Member,
    Import,
    Snippet,
    Generic,
}

impl Default for CompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_keyword_completion() {
        let provider = CompletionProvider::new();
        let content = "sl";
        let position = Position { line: 0, character: 2 };
        
        let completions = provider.get_completions(content, position).await;
        
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "slay"));
    }

    #[tokio::test]
    async fn test_variable_completion() {
        let provider = CompletionProvider::new();
        let content = "facts my_var = 42\nprint(my";
        let position = Position { line: 1, character: 9 };
        
        let completions = provider.get_completions(content, position).await;
        
        assert!(completions.iter().any(|c| c.label == "my_var"));
    }

    #[tokio::test]
    async fn test_function_completion() {
        let provider = CompletionProvider::new();
        let content = "pr";
        let position = Position { line: 0, character: 2 };
        
        let completions = provider.get_completions(content, position).await;
        
        assert!(completions.iter().any(|c| c.label == "print"));
        assert!(completions.iter().any(|c| c.label == "println"));
    }

    #[test]
    fn test_variable_extraction() {
        let provider = CompletionProvider::new();
        
        let name = provider.extract_variable_name("facts my_variable = 42");
        assert_eq!(name, Some("my_variable".to_string()));
        
        let name = provider.extract_variable_name("sus count: int = 0");
        assert_eq!(name, Some("count".to_string()));
    }

    #[test]
    fn test_function_signature_extraction() {
        let provider = CompletionProvider::new();
        
        let sig = provider.extract_function_signature("slay calculate(a: int, b: int) -> int {");
        assert_eq!(sig, Some(("calculate".to_string(), "a: int, b: int".to_string(), "int".to_string())));
    }
}
