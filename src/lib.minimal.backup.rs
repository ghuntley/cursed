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
pub use crate::error::CursedError;
pub use lexer::{Lexer, Token, TokenType};
pub use minimal_parser::Parser;
pub use minimal_ast::*;

// AST module alias for compatibility
pub mod ast {
    pub use crate::minimal_ast::identifiers;
}

/// Prelude module for minimal imports
pub mod prelude {
    pub use crate::lexer::{Lexer, Token, TokenType};
    pub use crate::minimal_parser::Parser;
    pub use crate::minimal_ast::*;
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the minimal CURSED runtime environment
pub fn init() {
        // TODO: implement
    }
    // Just basic logging setup
    env_logger::init();
}

/// Basic tokenize function - tokenize CURSED source
pub fn tokenize(source: &str) -> crate::error::Result<Vec<Token>> {
    let lexer = Lexer::new(source.to_string());
    Ok(lexer.collect())
}

/// Basic parse function - parse CURSED source into AST
pub fn parse(source: &str) -> crate::error::Result<Program> {
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
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
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("  {}. {:?}", i + 1, stmt);
    }
    Ok(())
}

/// Minimal file execution - read file and run
pub fn run_file(path: &str) -> crate::error::crate::error::Result<()> {
    let source = std::fs::read_to_string(path)?;
    run(&source)
}

/// Stub functions for CLI compatibility (always error for now)
pub fn compile_to_ir(_source: &str) -> Result<String> {
    Err(CursedError::NotImplemented("LLVM codegen not available in minimal build".to_string()))
}

pub fn compile_to_ir_with_optimization(_source: &str, _opt_level: Option<&str>) -> crate::error::Result<String> {
    Err(CursedError::NotImplemented("LLVM codegen not available in minimal build".to_string()))
}


// Include the test module
