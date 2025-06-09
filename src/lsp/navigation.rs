//! Navigation provider for CURSED language server
//! 
//! Provides go to definition, find references, hover information, etc.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};

use crate::lexer::{Lexer, TokenType};

/// Navigation provider for the LSP server
pub struct NavigationProvider {
    /// Symbol cache for quick lookups
    symbol_cache: std::sync::RwLock<HashMap<String, Vec<SymbolInfo>>>,
}

/// Symbol information for navigation
#[derive(Debug, Clone)]
struct SymbolInfo {
    name: String,
    kind: SymbolKind,
    location: Location,
    definition_location: Option<Location>,
    documentation: Option<String>,
    type_info: Option<String>,
}

impl NavigationProvider {
    /// Create a new navigation provider
    pub fn new() -> Self {
        Self {
            symbol_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Get hover information at position
    #[instrument(skip(self, content))]
    pub async fn get_hover_info(&self, content: &str, position: Position) -> Option<Hover> {
        debug!("Getting hover info at {:?}", position);
        
        let word = self.get_word_at_position(content, position)?;
        let hover_content = self.get_symbol_hover_content(content, &word, position);
        
        if let Some(content_str) = hover_content {
            Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: content_str,
                }),
                range: Some(self.get_word_range(content, position, &word)),
            })
        } else {
            None
        }
    }

    /// Get definition location
    #[instrument(skip(self, content))]
    pub async fn get_definition(
        &self,
        content: &str,
        position: Position,
        uri: &Url,
    ) -> Option<GotoDefinitionResponse> {
        debug!("Getting definition at {:?}", position);
        
        let word = self.get_word_at_position(content, position)?;
        let definition_location = self.find_symbol_definition(content, &word, uri);
        
        if let Some(location) = definition_location {
            Some(GotoDefinitionResponse::Scalar(location))
        } else {
            None
        }
    }

    /// Find all references to symbol
    #[instrument(skip(self, content))]
    pub async fn find_references(
        &self,
        content: &str,
        position: Position,
        uri: &Url,
    ) -> Vec<Location> {
        debug!("Finding references at {:?}", position);
        
        if let Some(word) = self.get_word_at_position(content, position) {
            self.find_symbol_references(content, &word, uri)
        } else {
            Vec::new()
        }
    }

    /// Get word at position
    fn get_word_at_position(&self, content: &str, position: Position) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let line_index = position.line as usize;
        let char_index = position.character as usize;

        if line_index >= lines.len() {
            return None;
        }

        let line = lines[line_index];
        let chars: Vec<char> = line.chars().collect();

        if char_index >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = char_index;
        let mut end = char_index;

        // Move start backward
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }

        // Move end forward
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }

        if start < end {
            Some(chars[start..end].iter().collect())
        } else {
            None
        }
    }

    /// Get word range at position
    fn get_word_range(&self, content: &str, position: Position, word: &str) -> Range {
        let lines: Vec<&str> = content.lines().collect();
        let line_index = position.line as usize;

        if line_index < lines.len() {
            let line = lines[line_index];
            if let Some(word_start) = line.find(word) {
                return Range {
                    start: Position {
                        line: position.line,
                        character: word_start as u32,
                    },
                    end: Position {
                        line: position.line,
                        character: (word_start + word.len()) as u32,
                    },
                };
            }
        }

        // Fallback to single character range
        Range {
            start: position,
            end: Position {
                line: position.line,
                character: position.character + 1,
            },
        }
    }

    /// Get symbol hover content
    fn get_symbol_hover_content(
        &self,
        content: &str,
        symbol: &str,
        position: Position,
    ) -> Option<String> {
        // Check if it's a built-in function
        if let Some(builtin_info) = self.get_builtin_function_info(symbol) {
            return Some(builtin_info);
        }

        // Check if it's a keyword
        if let Some(keyword_info) = self.get_keyword_info(symbol) {
            return Some(keyword_info);
        }

        // Check if it's a user-defined symbol
        if let Some(user_info) = self.get_user_symbol_info(content, symbol, position) {
            return Some(user_info);
        }

        None
    }

    /// Get built-in function information
    fn get_builtin_function_info(&self, symbol: &str) -> Option<String> {
        let builtins = HashMap::from([
            ("print", "```cursed\nslay print(value: any)\n```\n\nPrints a value to stdout without a newline."),
            ("println", "```cursed\nslay println(value: any)\n```\n\nPrints a value to stdout with a newline."),
            ("len", "```cursed\nslay len(collection: array|string|map) -> int\n```\n\nReturns the length of a collection or string."),
            ("str", "```cursed\nslay str(value: any) -> string\n```\n\nConverts a value to its string representation."),
            ("int", "```cursed\nslay int(value: string|float) -> int\n```\n\nConverts a value to an integer."),
            ("float", "```cursed\nslay float(value: string|int) -> float\n```\n\nConverts a value to a floating-point number."),
            ("type", "```cursed\nslay type(value: any) -> string\n```\n\nReturns the type name of a value."),
            ("panic", "```cursed\nslay panic(message: string)\n```\n\nTerminates the program with an error message."),
            ("spawn", "```cursed\nslay spawn(function: () -> any)\n```\n\nSpawns a new goroutine to execute the given function."),
            ("make", "```cursed\nslay make(type: Type, size?: int) -> Type\n```\n\nCreates a new instance of the specified collection type."),
            ("append", "```cursed\nslay append(slice: []T, elements: ...T) -> []T\n```\n\nAppends elements to a slice and returns the result."),
            ("copy", "```cursed\nslay copy(dst: []T, src: []T) -> int\n```\n\nCopies elements from src to dst and returns the number of elements copied."),
            ("delete", "```cursed\nslay delete(map: map[K]V, key: K)\n```\n\nDeletes a key from a map."),
            ("close", "```cursed\nslay close(channel: chan T)\n```\n\nCloses a channel."),
        ]);

        builtins.get(symbol).map(|info| info.to_string())
    }

    /// Get keyword information
    fn get_keyword_info(&self, symbol: &str) -> Option<String> {
        let keywords = HashMap::from([
            ("slay", "**slay** - Function declaration keyword\n\nDeclares a new function in CURSED.\n\n```cursed\nslay functionName(params) -> returnType {\n    // function body\n}\n```"),
            ("yolo", "**yolo** - Async function declaration keyword\n\nDeclares an asynchronous function in CURSED.\n\n```cursed\nyolo asyncFunction() {\n    await someAsyncOperation()\n}\n```"),
            ("facts", "**facts** - Immutable variable declaration\n\nDeclares an immutable variable (constant).\n\n```cursed\nfacts pi = 3.14159\nfacts name: string = \"CURSED\"\n```"),
            ("sus", "**sus** - Mutable variable declaration\n\nDeclares a mutable variable.\n\n```cursed\nsus counter = 0\nsus name: string = \"changeable\"\n```"),
            ("lowkey", "**lowkey** - If statement\n\nConditional execution keyword.\n\n```cursed\nlowkey condition {\n    // execute if true\n}\n```"),
            ("highkey", "**highkey** - Else statement\n\nElse clause for conditional statements.\n\n```cursed\nlowkey condition {\n    // if true\n} highkey {\n    // if false\n}\n```"),
            ("periodt", "**periodt** - Loop statement\n\nGeneral loop construct.\n\n```cursed\nperiodt condition {\n    // loop body\n}\n```"),
            ("bestie", "**bestie** - For loop\n\nIterates over collections.\n\n```cursed\nbestie item in collection {\n    // process item\n}\n```"),
            ("flex", "**flex** - While loop\n\nConditional loop.\n\n```cursed\nflex condition {\n    // loop body\n}\n```"),
            ("squad", "**squad** - Struct declaration\n\nDefines a new struct type.\n\n```cursed\nsquad Person {\n    name: string,\n    age: int,\n}\n```"),
            ("collab", "**collab** - Interface declaration\n\nDefines a new interface.\n\n```cursed\ncollab Drawable {\n    draw() -> void,\n}\n```"),
            ("vibes", "**vibes** - Enum declaration\n\nDefines an enumeration.\n\n```cursed\nvibes Color {\n    Red,\n    Green,\n    Blue,\n}\n```"),
            ("bounce", "**bounce** - Return statement\n\nReturns a value from a function.\n\n```cursed\nbounce result\n```"),
            ("yeet", "**yeet** - Throw/panic statement\n\nThrows an error or panics.\n\n```cursed\nyeet \"Something went wrong!\"\n```"),
            ("vibe_check", "**vibe_check** - Switch statement\n\nMulti-way conditional.\n\n```cursed\nvibe_check value {\n    mood case1:\n        // handle case1\n    basic:\n        // default case\n}\n```"),
            ("mood", "**mood** - Case statement\n\nCase clause in switch statements.\n\n```cursed\nmood value:\n    // handle this case\n```"),
            ("basic", "**basic** - Default case\n\nDefault clause in switch statements.\n\n```cursed\nbasic:\n    // default handling\n```"),
            ("chan", "**chan** - Channel type\n\nDeclares a channel for goroutine communication.\n\n```cursed\nfacts ch: chan int = make(chan int)\n```"),
            ("use", "**use** - Import statement\n\nImports modules or packages.\n\n```cursed\nuse \"std/fmt\"\nuse \"./mymodule\"\n```"),
            ("await", "**await** - Await async operation\n\nWaits for an async operation to complete.\n\n```cursed\nfacts result = await asyncFunction()\n```"),
        ]);

        keywords.get(symbol).map(|info| info.to_string())
    }

    /// Get user-defined symbol information
    fn get_user_symbol_info(
        &self,
        content: &str,
        symbol: &str,
        _position: Position,
    ) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();

        // Look for variable declarations
        for (line_num, line) in lines.iter().enumerate() {
            if let Some((var_name, var_type, is_mutable)) = self.extract_variable_info(line) {
                if var_name == symbol {
                    let mutability = if is_mutable { "mutable" } else { "immutable" };
                    let type_str = var_type.unwrap_or_else(|| "inferred".to_string());
                    return Some(format!(
                        "**{}** - {} variable\n\n```cursed\n{}\n```\n\nType: `{}`\nDefined at line {}",
                        symbol, mutability, line.trim(), type_str, line_num + 1
                    ));
                }
            }
        }

        // Look for function declarations
        for (line_num, line) in lines.iter().enumerate() {
            if let Some((func_name, params, return_type)) = self.extract_function_info(line) {
                if func_name == symbol {
                    let signature = if !return_type.is_empty() {
                        format!("{}({}) -> {}", func_name, params, return_type)
                    } else {
                        format!("{}({})", func_name, params)
                    };
                    return Some(format!(
                        "**{}** - user-defined function\n\n```cursed\n{}\n```\n\nDefined at line {}",
                        symbol, signature, line_num + 1
                    ));
                }
            }
        }

        // Look for struct declarations
        for (line_num, line) in lines.iter().enumerate() {
            if line.contains("squad") && line.contains(symbol) {
                return Some(format!(
                    "**{}** - struct type\n\n```cursed\n{}\n```\n\nDefined at line {}",
                    symbol, line.trim(), line_num + 1
                ));
            }
        }

        // Look for interface declarations
        for (line_num, line) in lines.iter().enumerate() {
            if line.contains("collab") && line.contains(symbol) {
                return Some(format!(
                    "**{}** - interface type\n\n```cursed\n{}\n```\n\nDefined at line {}",
                    symbol, line.trim(), line_num + 1
                ));
            }
        }

        None
    }

    /// Find symbol definition location
    fn find_symbol_definition(&self, content: &str, symbol: &str, uri: &Url) -> Option<Location> {
        let lines: Vec<&str> = content.lines().collect();

        // Look for variable declarations
        for (line_num, line) in lines.iter().enumerate() {
            if let Some((var_name, _, _)) = self.extract_variable_info(line) {
                if var_name == symbol {
                    if let Some(name_start) = line.find(&var_name) {
                        return Some(Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: name_start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: (name_start + var_name.len()) as u32,
                                },
                            },
                        });
                    }
                }
            }
        }

        // Look for function declarations
        for (line_num, line) in lines.iter().enumerate() {
            if let Some((func_name, _, _)) = self.extract_function_info(line) {
                if func_name == symbol {
                    if let Some(name_start) = line.find(&func_name) {
                        return Some(Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: name_start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: (name_start + func_name.len()) as u32,
                                },
                            },
                        });
                    }
                }
            }
        }

        // Look for type declarations
        for (line_num, line) in lines.iter().enumerate() {
            if (line.contains("squad") || line.contains("collab") || line.contains("vibes")) 
                && line.contains(symbol) {
                if let Some(name_start) = line.find(symbol) {
                    return Some(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: name_start as u32,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: (name_start + symbol.len()) as u32,
                            },
                        },
                    });
                }
            }
        }

        None
    }

    /// Find all references to a symbol
    fn find_symbol_references(&self, content: &str, symbol: &str, uri: &Url) -> Vec<Location> {
        let mut references = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let mut search_pos = 0;
            while let Some(pos) = line[search_pos..].find(symbol) {
                let actual_pos = search_pos + pos;
                
                // Check if it's a whole word (not part of another identifier)
                let is_word_boundary = {
                    let before_ok = actual_pos == 0 || 
                        !line.chars().nth(actual_pos - 1).unwrap_or(' ').is_alphanumeric();
                    let after_ok = {
                        let end_pos = actual_pos + symbol.len();
                        end_pos >= line.len() || 
                        !line.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric()
                    };
                    before_ok && after_ok
                };

                if is_word_boundary {
                    references.push(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: actual_pos as u32,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: (actual_pos + symbol.len()) as u32,
                            },
                        },
                    });
                }

                search_pos = actual_pos + 1;
            }
        }

        references
    }

    /// Extract variable information from a line
    fn extract_variable_info(&self, line: &str) -> Option<(String, Option<String>, bool)> {
        if let Some(facts_pos) = line.find("facts") {
            let after_facts = &line[facts_pos + 5..];
            self.parse_variable_declaration(after_facts, false)
        } else if let Some(sus_pos) = line.find("sus") {
            let after_sus = &line[sus_pos + 3..];
            self.parse_variable_declaration(after_sus, true)
        } else {
            None
        }
    }

    /// Parse variable declaration
    fn parse_variable_declaration(&self, declaration: &str, is_mutable: bool) -> Option<(String, Option<String>, bool)> {
        if let Some(equals_pos) = declaration.find('=') {
            let name_and_type = declaration[..equals_pos].trim();
            
            if let Some(colon_pos) = name_and_type.find(':') {
                // Has explicit type annotation
                let name = name_and_type[..colon_pos].trim().to_string();
                let type_str = name_and_type[colon_pos + 1..].trim().to_string();
                Some((name, Some(type_str), is_mutable))
            } else {
                // No explicit type, infer from value
                let name = name_and_type.trim().to_string();
                let value_part = declaration[equals_pos + 1..].trim();
                let inferred_type = self.infer_type_from_value(value_part);
                Some((name, inferred_type, is_mutable))
            }
        } else {
            None
        }
    }

    /// Extract function information from a line
    fn extract_function_info(&self, line: &str) -> Option<(String, String, String)> {
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

    /// Infer type from value
    fn infer_type_from_value(&self, value: &str) -> Option<String> {
        let value = value.trim();
        
        if value.starts_with('"') && value.ends_with('"') {
            Some("string".to_string())
        } else if value == "true" || value == "false" {
            Some("bool".to_string())
        } else if value.contains('.') && value.parse::<f64>().is_ok() {
            Some("float".to_string())
        } else if value.parse::<i64>().is_ok() {
            Some("int".to_string())
        } else if value.starts_with('[') && value.ends_with(']') {
            Some("array".to_string())
        } else if value.starts_with('{') && value.ends_with('}') {
            Some("map".to_string())
        } else {
            None
        }
    }
}

impl Default for NavigationProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hover_builtin_function() {
        let provider = NavigationProvider::new();
        let content = "print(\"hello\")";
        let position = Position { line: 0, character: 2 }; // On "print"
        
        let hover = provider.get_hover_info(content, position).await;
        assert!(hover.is_some());
        
        if let Some(hover) = hover {
            if let HoverContents::Markup(markup) = hover.contents {
                assert!(markup.value.contains("print"));
                assert!(markup.value.contains("stdout"));
            }
        }
    }

    #[tokio::test]
    async fn test_find_variable_definition() {
        let provider = NavigationProvider::new();
        let content = "facts my_var = 42\nprint(my_var)";
        let uri = Url::parse("file:///test.csd").unwrap();
        let position = Position { line: 1, character: 8 }; // On "my_var" in print statement
        
        let definition = provider.get_definition(content, position, &uri).await;
        assert!(definition.is_some());
        
        if let Some(GotoDefinitionResponse::Scalar(location)) = definition {
            assert_eq!(location.range.start.line, 0); // Defined on first line
        }
    }

    #[tokio::test]
    async fn test_find_references() {
        let provider = NavigationProvider::new();
        let content = "facts my_var = 42\nprint(my_var)\nsus other = my_var + 1";
        let uri = Url::parse("file:///test.csd").unwrap();
        let position = Position { line: 0, character: 6 }; // On variable declaration
        
        let references = provider.find_references(content, position, &uri).await;
        assert_eq!(references.len(), 3); // Declaration + 2 usages
    }

    #[test]
    fn test_variable_info_extraction() {
        let provider = NavigationProvider::new();
        
        let info = provider.extract_variable_info("facts my_var: string = \"hello\"");
        assert_eq!(info, Some(("my_var".to_string(), Some("string".to_string()), false)));
        
        let info = provider.extract_variable_info("sus counter = 42");
        assert_eq!(info, Some(("counter".to_string(), Some("int".to_string()), true)));
    }

    #[test]
    fn test_function_info_extraction() {
        let provider = NavigationProvider::new();
        
        let info = provider.extract_function_info("slay calculate(a: int, b: int) -> int {");
        assert_eq!(info, Some(("calculate".to_string(), "a: int, b: int".to_string(), "int".to_string())));
    }
}
