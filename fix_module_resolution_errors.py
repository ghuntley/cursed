#!/usr/bin/env python3
"""
Systematic fix for E0433 module resolution and E0659 ambiguous import errors.
"""

import os
import re
import subprocess
from pathlib import Path

def run_cargo_check():
    """Run cargo check and return error output."""
    result = subprocess.run(
        ["./fix_linking.sh", "cargo", "check", "--message-format=short"], 
        capture_output=True, text=True, cwd="."
    )
    return result.stdout + result.stderr

def fix_db_core_exports():
    """Fix missing exports in db_core module."""
    print("Fixing db_core module exports...")
    
    # Check if db_core mod.rs exists
    db_core_mod = Path("src/stdlib/packages/db_core/mod.rs")
    if not db_core_mod.exists():
        print(f"Creating {db_core_mod}...")
        db_core_mod.parent.mkdir(parents=True, exist_ok=True)
        db_core_mod.write_text("""/// Database core types and utilities
pub mod connection;
pub mod error;
pub mod query;
pub mod result;
pub mod types;

// Re-export commonly used types
pub use connection::*;
pub use error::*;
pub use query::*;
pub use result::*;
pub use types::*;
""")
    
    # Ensure ResultType is exported from result module
    result_mod = Path("src/stdlib/packages/result.rs")
    if result_mod.exists():
        content = result_mod.read_text()
        if "pub enum ResultType" not in content:
            print("Adding ResultType to result.rs")
            result_mod.write_text(content + """
#[derive(Debug, Clone, PartialEq)]
pub enum ResultType {
    Forward,
    ForwardOnly,
    Static,
    Keyset,
    Dynamic,
}
""")

def fix_ast_exports():
    """Fix AST module exports for AstNodeType."""
    print("Fixing AST module exports...")
    
    ast_mod = Path("src/ast/mod.rs")
    if ast_mod.exists():
        content = ast_mod.read_text()
        
        # Add AstNodeType if missing
        if "AstNodeType" not in content:
            print("Adding AstNodeType to AST module...")
            content = content.replace(
                "pub use statements::*;",
                """pub use statements::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNodeType {
    FunctionDeclaration(Box<FunctionDeclaration>),
    Statement(Box<Statement>),
    Expression(Box<Expression>),
    VariableDeclaration(Box<VariableDeclaration>),
    StructDeclaration(Box<StructDeclaration>),
    InterfaceDeclaration(Box<InterfaceDeclaration>),
}"""
            )
            ast_mod.write_text(content)

def fix_pgo_exports():
    """Fix PGO module exports."""
    print("Fixing PGO module exports...")
    
    pgo_mod = Path("src/optimization/pgo.rs")
    if pgo_mod.exists():
        content = pgo_mod.read_text()
        
        # Add PgoManager if missing
        if "pub struct PgoManager" not in content:
            print("Adding PgoManager to pgo.rs")
            content = content + """

#[derive(Debug, Clone)]
pub struct PgoManager {
    config: PgoConfig,
}

impl PgoManager {
    pub fn new(config: PgoConfig) -> Self {
        Self { config }
    }
}
"""
            pgo_mod.write_text(content)

def fix_optimization_exports():
    """Fix optimization module exports."""
    print("Fixing optimization module exports...")
    
    # Fix PgoConfig export
    optimization_mod = Path("src/optimization/mod.rs")
    if optimization_mod.exists():
        content = optimization_mod.read_text()
        
        if "PgoConfig" not in content:
            print("Adding PgoConfig to optimization module...")
            content = content.replace(
                "pub use comprehensive_performance_system::*;",
                """pub use comprehensive_performance_system::*;

#[derive(Debug, Clone)]
pub struct PgoConfig {
    pub profile_dir: String,
    pub enabled: bool,
}

impl Default for PgoConfig {
    fn default() -> Self {
        Self {
            profile_dir: "pgo-profiles".to_string(),
            enabled: false,
        }
    }
}"""
            )
            optimization_mod.write_text(content)

def fix_lexer_exports():
    """Fix lexer module exports for token access."""
    print("Fixing lexer module exports...")
    
    lexer_mod = Path("src/lexer/mod.rs")
    if lexer_mod.exists():
        content = lexer_mod.read_text()
        
        # Ensure token module is exported
        if "pub mod token;" not in content and "pub use token::*;" not in content:
            print("Adding token exports to lexer module...")
            content = content.replace(
                "pub use lexer::*;",
                "pub use lexer::*;\npub use token::*;"
            )
            lexer_mod.write_text(content)

def fix_ambiguous_imports():
    """Fix ambiguous import errors by using qualified imports."""
    print("Fixing ambiguous imports...")
    
    # Fix stdlib/mod.rs ambiguous imports
    stdlib_mod = Path("src/stdlib/mod.rs")
    if stdlib_mod.exists():
        content = stdlib_mod.read_text()
        
        # Replace ambiguous re-exports with qualified ones
        fixes = [
            # String manipulations
            ("pub use string_manipulation::replace_first;", 
             "pub use string_manipulation::replace_first as string_replace_first;"),
            
            # Process types
            ("pub use process::*;", 
             """pub use process::{
    ProcessOutput as StdProcessOutput,
    Process as StdProcess,
    command_exists as std_command_exists,
};"""),
            
            # Queue types
            ("pub use collections::queues::PriorityQueue;",
             "pub use collections::queues::PriorityQueue as CollectionsPriorityQueue;"),
        ]
        
        for old, new in fixes:
            if old in content:
                print(f"Fixing ambiguous import: {old}")
                content = content.replace(old, new)
        
        stdlib_mod.write_text(content)

def fix_process_imports():
    """Fix process module ambiguous imports."""
    print("Fixing process module imports...")
    
    process_files = [
        "src/stdlib/process/lifecycle.rs",
        "src/stdlib/process/integration.rs", 
        "src/stdlib/process/unified_process_ipc.rs",
        "src/stdlib/process/unix_platform.rs",
    ]
    
    for file_path in process_files:
        path = Path(file_path)
        if path.exists():
            content = path.read_text()
            
            # Replace wildcard imports with specific imports
            if "use crate::stdlib::process::*;" in content:
                print(f"Fixing process imports in {file_path}")
                content = content.replace(
                    "use crate::stdlib::process::*;",
                    """use crate::stdlib::process::{
    Process as ProcessCore,
    ProcessInfo as ProcessInfoCore,
    ProcessStatus as ProcessStatusCore,
    ProcessOutput as ProcessOutputCore,
    Signal as SignalCore,
    EnhancedProcess as EnhancedProcessCore,
    ProcessState as ProcessStateCore,
    ResourceLimits as ResourceLimitsCore,
    SecurityContext as SecurityContextCore,
    ProcessGroup as ProcessGroupCore,
};"""
                )
                path.write_text(content)

def fix_crypto_imports():
    """Fix crypto module ambiguous imports."""
    print("Fixing crypto module imports...")
    
    crypto_cert = Path("src/stdlib/crypto/certificates.rs")
    if crypto_cert.exists():
        content = crypto_cert.read_text()
        
        # Fix pem ambiguous import
        if "use pem;" in content:
            print("Fixing pem import in certificates.rs")
            content = content.replace(
                "use pem;",
                "use pem as pem_crate;"
            )
            
            # Also fix usage
            content = content.replace("pem::", "pem_crate::")
            crypto_cert.write_text(content)

def add_missing_imports():
    """Add missing imports to files that need them."""
    print("Adding missing imports...")
    
    # Fix ML feature extraction
    ml_feature = Path("src/optimization/ml/feature_extraction.rs")
    if ml_feature.exists():
        content = ml_feature.read_text()
        if "use crate::ast::AstNodeType;" not in content:
            print("Adding AstNodeType import to feature_extraction.rs")
            content = "use crate::ast::AstNodeType;\n" + content
            ml_feature.write_text(content)
    
    # Fix database modules
    db_files = [
        "src/stdlib/packages/db_sql/postgresql.rs",
        "src/stdlib/packages/db_sql/mysql.rs", 
        "src/stdlib/packages/db_sql/sqlite.rs",
    ]
    
    for file_path in db_files:
        path = Path(file_path)
        if path.exists():
            content = path.read_text()
            
            # Add missing imports at the top
            needed_imports = []
            
            if "ResultType" in content and "use crate::stdlib::packages::result::ResultType;" not in content:
                needed_imports.append("use crate::stdlib::packages::result::ResultType;")
            
            if "ParameterDirection" in content and "use crate::stdlib::packages::types::ParameterDirection;" not in content:
                needed_imports.append("use crate::stdlib::packages::types::ParameterDirection;")
            
            if "Value::" in content and "use crate::runtime::Value;" not in content:
                needed_imports.append("use crate::runtime::Value;")
            
            if needed_imports:
                print(f"Adding imports to {file_path}: {needed_imports}")
                # Add imports after the first comment block
                lines = content.split('\n')
                insert_pos = 0
                for i, line in enumerate(lines):
                    if line.strip() and not line.strip().startswith('///') and not line.strip().startswith('//'):
                        insert_pos = i
                        break
                
                for import_line in reversed(needed_imports):
                    lines.insert(insert_pos, import_line)
                
                path.write_text('\n'.join(lines))

def ensure_module_structure():
    """Ensure all required modules exist and are properly structured."""
    print("Ensuring module structure...")
    
    # Required modules and their basic structure
    modules = {
        "src/stdlib/packages/db_core/mod.rs": """/// Database core functionality
pub mod connection;
pub mod error;
pub mod query;
pub mod result;
pub mod types;

pub use connection::*;
pub use error::*;
pub use query::*;
pub use result::*;
pub use types::*;
""",
        "src/stdlib/packages/types.rs": """/// Database parameter types
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterDirection {
    In,
    Out,
    InOut,
    Return,
}
""",
        "src/stdlib/packages/query.rs": """/// Database query types
#[derive(Debug, Clone)]
pub struct Query {
    pub sql: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterDirection {
    In,
    Out,
    InOut,
    Return,
}
""",
        "src/lexer/token.rs": """/// Token types for the lexer
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Basic tokens
    Identifier,
    Number,
    String,
    // Keywords
    Function,
    Variable,
    If,
    Else,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    // Special
    EOF,
    Error,
}
""",
    }
    
    for file_path, content in modules.items():
        path = Path(file_path)
        if not path.exists():
            print(f"Creating missing module: {file_path}")
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)

def main():
    """Main function to fix all module resolution errors."""
    print("Starting systematic fix of module resolution errors...")
    
    # Get initial error count
    output = run_cargo_check()
    initial_e0433 = len(re.findall(r'error\[E0433\]', output))
    initial_e0659 = len(re.findall(r'error\[E0659\]', output))
    
    print(f"Initial errors - E0433: {initial_e0433}, E0659: {initial_e0659}")
    
    # Apply fixes systematically
    ensure_module_structure()
    fix_ast_exports()
    fix_db_core_exports()
    fix_pgo_exports()
    fix_optimization_exports()
    fix_lexer_exports()
    fix_ambiguous_imports()
    fix_process_imports()
    fix_crypto_imports()
    add_missing_imports()
    
    # Check results
    print("\nRunning final check...")
    output = run_cargo_check()
    final_e0433 = len(re.findall(r'error\[E0433\]', output))
    final_e0659 = len(re.findall(r'error\[E0659\]', output))
    
    print(f"Final errors - E0433: {final_e0433}, E0659: {final_e0659}")
    print(f"Reduction - E0433: {initial_e0433 - final_e0433}, E0659: {initial_e0659 - final_e0659}")
    
    if final_e0433 + final_e0659 > 0:
        print(f"\nRemaining issues to investigate:")
        remaining_errors = re.findall(r'error\[E0(433|659)\].*', output)
        for error in remaining_errors[:10]:  # Show first 10
            print(f"  {error}")
        if len(remaining_errors) > 10:
            print(f"  ... and {len(remaining_errors) - 10} more")

if __name__ == "__main__":
    main()
