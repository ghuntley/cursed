/// CURSED Programming Language Library (Minimal Build)
/// 
/// A truly minimal implementation focusing only on core functionality:
/// - Basic lexer for CURSED Gen Z slang syntax
/// - Basic parser for CURSED language grammar  
/// - Simple AST representation
/// - Essential error handling

// Core modules only - absolutely minimal
pub mod error;
pub mod lexer;
pub mod parser;
pub mod ast;

// Re-export essential error handling
pub use crate::error::CursedError;

/// Prelude module for minimal imports
pub mod prelude {
    pub use crate::lexer::Lexer;
    pub use crate::parser::Parser;
    pub use crate::ast::*;
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the minimal CURSED runtime environment
pub fn init() {
        // TODO: implement
    }
    // Basic logging setup
    env_logger::init();
}

/// Basic tokenize function - tokenize CURSED source
pub fn tokenize(source: &str) -> crate::error::Result<Vec<crate::lexer::Token>> {
    let lexer = crate::lexer::Lexer::new(source.to_string());
    Ok(lexer.collect())
}

/// Basic parse function - parse CURSED source into AST
pub fn parse(source: &str) -> crate::error::Result<crate::ast::Program> {
    let lexer = crate::lexer::Lexer::new(source.to_string());
    let mut parser = crate::parser::Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    // Check for parse errors
    let errors = parser.errors();
    if !errors.is_empty() {
        return Err(CursedError::Parse(format!("Parse errors: {}", errors.join(", "))));
    }
    
    Ok(program)
}

/// Check CURSED source for syntax errors only (minimal version)
pub fn check(source: &str) -> crate::error::crate::error::Result<()> {
    let _ = parse(source)?;
    println!("✅ Syntax check passed!");
    Ok(())
}

/// Format CURSED source code (minimal version - just return original for now)
pub fn format(source: &str) -> Result<String> {
    // Validate syntax first
    let _ = parse(source)?;
    // For now, just return original source
    Ok(source.to_string())
}

/// Minimal execution - just parse and report what we found
pub fn run(source: &str) -> crate::error::Result<()> {
    let program = parse(source)?;
    println!("🎯 Parsed CURSED program with {} statements", program.statements.len());
    Ok(())
}

/// Minimal file execution - read file and run
pub fn run_file(path: &str) -> crate::error::Result<()> {
    let source = std::fs::read_to_string(path)?;
    run(&source)
}
