/// CURSED Programming Language Library (Truly Minimal Build)
///
/// A truly minimal implementation with just the essentials:
/// - Basic lexer for CURSED Gen Z slang syntax
/// - Basic parser for CURSED language grammar
/// - Simple AST representation
/// - Essential error handling
// Core modules only - absolutely minimal
pub mod error;
pub mod lexer;
pub mod minimal_parser;
pub mod minimal_ast;

// Critical module stubs to prevent compilation errors
pub mod stdlib {
    pub mod glowup_http {
        pub mod client {
            pub struct VibeClient;
        }
        pub mod error {
            pub type GlowUpResult<T> = Result<T, String>;
        }
    }
    pub mod testing {
        pub mod framework {
            pub struct TestFrameworkReport;
        }
    }
    pub mod crypto_pqc {
        pub mod hybrid {}
    }
    pub mod database {
        pub struct DB;
        pub struct Conn;
        pub struct QueryResult;
        pub struct SqliteDriver;
        pub struct DriverConn;
        pub struct DatabaseError;
    }
    pub mod value {
        #[derive(Debug, Clone)]
        pub enum Value {
            Object(std::collections::HashMap<String, String>),
            String(String),
            Int(i64),
        }
    }
    pub mod packages {
        pub mod test_vibes {
            pub mod runners {
                pub struct TestRunnerConfig;
            }
            pub mod fixtures {
                pub struct DatabaseFixture;
            }
        }
    }
}

pub mod optimization {
    pub mod parallel {
        pub struct ParallelCompiler;
        pub struct CompilationJob;
        pub enum JobPriority {
            High,
            Normal,
            Low,
        }
    }
    pub mod llvm_advanced {
        pub mod utils {
            pub fn dev_config() -> String { "dev".to_string() }
            pub fn release_config() -> String { "release".to_string() }
            pub fn pgo_config() -> String { "pgo".to_string() }
        }
    }
}

// Re-export essential types
pub use error::Error;
pub use lexer::{Lexer, Token, TokenType};
pub use minimal_parser::Parser;
pub use minimal_ast::*;

// AST module alias for compatibility
pub mod ast {
    pub use crate::minimal_ast::identifiers;
}

/// Prelude module for minimal imports
pub mod prelude {
    pub use crate::error_types::Error;
    pub use crate::lexer::{Lexer, Token, TokenType};
    pub use crate::minimal_parser::Parser;
    pub use crate::minimal_ast::*;
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the minimal CURSED runtime environment
pub fn init() {
    // Just basic logging setup
    env_logger::init();
}

/// Basic tokenize function - tokenize CURSED source
pub fn tokenize(source: &str) -> Result<Vec<Token>, Error> {
    let lexer = Lexer::new(source.to_string());
    Ok(lexer.collect())
}

/// Basic parse function - parse CURSED source into AST
pub fn parse(source: &str) -> Result<Program, Error> {
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    // Check for parse errors
    let errors = parser.errors();
    if !errors.is_empty() {
        return Err(Error::Parse(format!("Parse errors: {}", errors.join(", "))));
    }
    
    Ok(program)
}

/// Check CURSED source for syntax errors only (minimal version)
pub fn check(source: &str) -> Result<(), Error> {
    let _ = parse(source)?;
    println!("✅ Syntax check passed!");
    Ok(())
}

/// Format CURSED source code (minimal version - just return original for now)
pub fn format(source: &str) -> Result<String, Error> {
    // Validate syntax first
    let _ = parse(source)?;
    // For now, just return original source
    Ok(source.to_string())
}

/// Minimal execution - just parse and report what we found
pub fn run(source: &str) -> Result<(), Error> {
    let program = parse(source)?;
    println!("🎯 Parsed CURSED program with {} statements", program.statements.len());
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("  {}. {:?}", i + 1, stmt);
    }
    Ok(())
}

/// Minimal file execution - read file and run
pub fn run_file(path: &str) -> Result<(), Error> {
    let source = std::fs::read_to_string(path)?;
    run(&source)
}

/// Stub functions for CLI compatibility (always error for now)
pub fn compile_to_ir(_source: &str) -> Result<String, Error> {
    Err(Error::NotImplemented("LLVM codegen not available in minimal build".to_string()))
}

pub fn compile_to_ir_with_optimization(_source: &str, _opt_level: Option<&str>) -> Result<String, Error> {
    Err(Error::NotImplemented("LLVM codegen not available in minimal build".to_string()))
}


// Include the test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let source = r#"facts x = 42;"#;
        
        let tokens = tokenize(source).expect("Tokenization should succeed");
        
        // Should have: facts, x, =, 42, ;
        assert_eq!(tokens.len(), 5);
        
        assert_eq!(tokens[0].token_type, TokenType::Facts);
        assert_eq!(tokens[0].literal, "facts");
        
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].literal, "x");
        
        assert_eq!(tokens[2].token_type, TokenType::Assign);
        assert_eq!(tokens[2].literal, "=");
        
        assert_eq!(tokens[3].token_type, TokenType::Integer);
        assert_eq!(tokens[3].literal, "42");
        
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[4].literal, ";");
    }
    
    #[test]
    fn test_basic_parsing() {
        let source = r#"facts x = 42;"#;
        
        let program = parse(source).expect("Parsing should succeed");
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::Facts(name, expr) => {
                assert_eq!(name, "x");
                match expr {
                    Expression::Integer(val) => assert_eq!(*val, 42),
                    _ => panic!("Expected integer expression"),
                }
            }
            _ => panic!("Expected facts statement"),
        }
    }
    
    #[test]
    fn test_string_parsing() {
        let source = r#"facts name = "CURSED";"#;
        
        let program = parse(source).expect("Parsing should succeed");
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::Facts(name, expr) => {
                assert_eq!(name, "name");
                match expr {
                    Expression::String(val) => assert_eq!(val, "CURSED"),
                    _ => panic!("Expected string expression"),
                }
            }
            _ => panic!("Expected facts statement"),
        }
    }

    #[test]
    fn test_function_declaration() {
        let source = r#"slay greet(name) { facts x = 1; }"#;
        
        let program = parse(source).expect("Parsing should succeed");
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::Slay(name, params, body) => {
                assert_eq!(name, "greet");
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], "name");
                assert_eq!(body.len(), 1);
            }
            _ => panic!("Expected slay statement"),
        }
    }
}
