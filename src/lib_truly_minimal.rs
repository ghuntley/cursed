/// CURSED Programming Language Library (Truly Minimal Build)
/// 
/// A truly minimal implementation with just the essentials:
/// - Basic lexer for CURSED Gen Z slang syntax
/// - Basic parser for CURSED language grammar  
/// - Simple AST representation
/// - Essential error handling

// Core modules only - absolutely minimal
pub mod error;
pub mod minimal_lexer;
pub mod minimal_parser;
pub mod minimal_ast;

// Re-export essential types
pub use error::Error;
pub use minimal_lexer::{Lexer, Token, TokenType};
pub use minimal_parser::Parser;
pub use minimal_ast::*;

/// Prelude module for minimal imports
pub mod prelude {
    pub use crate::error_types::Error;
    pub use crate::minimal_lexer::{Lexer, Token, TokenType};
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
