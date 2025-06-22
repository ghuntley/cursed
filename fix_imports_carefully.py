#!/usr/bin/env python3
"""
Carefully fix import issues without creating conflicts.
"""

import os
import re
from pathlib import Path

def fix_database_imports():
    """Fix database import issues by using qualified imports."""
    print("Fixing database import issues...")
    
    # Fix db_sql files to use qualified imports instead of creating conflicts
    db_files = [
        "src/stdlib/packages/db_sql/postgresql.rs",
        "src/stdlib/packages/db_sql/mysql.rs", 
        "src/stdlib/packages/db_sql/sqlite.rs",
    ]
    
    for file_path in db_files:
        path = Path(file_path)
        if path.exists():
            content = path.read_text()
            
            # Remove the imports we just added that are causing conflicts
            lines = content.split('\n')
            filtered_lines = []
            
            for line in lines:
                # Skip the problematic imports we just added
                if any(x in line for x in [
                    "use crate::stdlib::packages::result::ResultType;",
                    "use crate::stdlib::packages::types::ParameterDirection;",
                ]):
                    continue
                filtered_lines.append(line)
            
            content = '\n'.join(filtered_lines)
            
            # Fix the actual usage with qualified paths
            replacements = [
                # Fix ResultType references
                ("crate::stdlib::packages::db_core::ResultType", "crate::stdlib::packages::result::ResultType"),
                ("crate::stdlib::packages::db_core::ParameterDirection", "crate::stdlib::packages::types::ParameterDirection"),
                ("crate::stdlib::packages::db_core::Value", "crate::runtime::Value"),
            ]
            
            for old, new in replacements:
                if old in content:
                    print(f"Replacing {old} with {new} in {file_path}")
                    content = content.replace(old, new)
            
            path.write_text(content)

def fix_crypto_pem_conflict():
    """Fix crypto pem import conflict."""
    print("Fixing crypto pem conflict...")
    
    crypto_cert = Path("src/stdlib/crypto/certificates.rs")
    if crypto_cert.exists():
        content = crypto_cert.read_text()
        
        # More specific fix for pem conflict
        if "use pem as pem_crate;" in content:
            # Find all pem:: usage and replace with pem_crate::
            content = re.sub(r'\bpem::', 'pem_crate::', content)
            crypto_cert.write_text(content)

def remove_duplicate_types():
    """Remove duplicate type definitions that are causing conflicts."""
    print("Removing duplicate type definitions...")
    
    # Remove the ParameterDirection from query.rs since it conflicts with types.rs
    query_file = Path("src/stdlib/packages/query.rs")
    if query_file.exists():
        content = query_file.read_text()
        # Remove the duplicate ParameterDirection enum
        content = re.sub(
            r'#\[derive\(Debug, Clone, PartialEq\)\]\npub enum ParameterDirection \{[^}]+\}',
            '',
            content,
            flags=re.DOTALL
        )
        query_file.write_text(content)

def fix_ast_node_type():
    """Properly add AstNodeType without conflicts."""
    print("Fixing AstNodeType...")
    
    # Create a dedicated file for AstNodeType to avoid conflicts
    ast_node_file = Path("src/ast/ast_node_type.rs")
    if not ast_node_file.exists():
        ast_node_file.write_text("""/// AST node type enumeration for ML feature extraction
use crate::ast::expressions::Expression;
use crate::ast::statements::Statement;
use crate::ast::declarations::{FunctionDeclaration, VariableDeclaration, StructDeclaration, InterfaceDeclaration};

#[derive(Debug, Clone, PartialEq)]
pub enum AstNodeType {
    FunctionDeclaration(Box<FunctionDeclaration>),
    Statement(Box<Statement>),
    Expression(Box<Expression>),
    VariableDeclaration(Box<VariableDeclaration>),
    StructDeclaration(Box<StructDeclaration>),
    InterfaceDeclaration(Box<InterfaceDeclaration>),
}
""")
    
    # Add to ast/mod.rs
    ast_mod = Path("src/ast/mod.rs")
    if ast_mod.exists():
        content = ast_mod.read_text()
        if "pub mod ast_node_type;" not in content:
            content = content.replace(
                "pub mod visitor;",
                "pub mod visitor;\npub mod ast_node_type;"
            )
        if "pub use ast_node_type::*;" not in content:
            content = content.replace(
                "pub use visitor::*;",
                "pub use visitor::*;\npub use ast_node_type::*;"
            )
        ast_mod.write_text(content)

def fix_specific_external_imports():
    """Fix specific external dependency issues."""
    print("Fixing external dependency issues...")
    
    # Fix x509_parser issues in certificates.rs
    crypto_cert = Path("src/stdlib/crypto/certificates.rs")
    if crypto_cert.exists():
        content = crypto_cert.read_text()
        
        # Add the missing x509_parser submodules
        if "x509_parser::name" in content and "use x509_parser::{" not in content:
            # Add proper imports at the top
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.startswith("use x509_parser::") and "name" not in line:
                    lines.insert(i+1, "use x509_parser::{name, algorithm};")
                    break
            content = '\n'.join(lines)
            crypto_cert.write_text(content)
    
    # Fix reqwest multipart issue
    http_client = Path("src/stdlib/glowup_http/client.rs")
    if http_client.exists():
        content = http_client.read_text()
        if "reqwest::multipart" in content and "multipart" not in content.split('\n')[0:20]:
            # Add multipart import
            content = content.replace(
                "use reqwest::{",
                "use reqwest::{multipart, "
            )
            http_client.write_text(content)

def main():
    """Main function to carefully fix import issues."""
    print("Carefully fixing import issues...")
    
    fix_database_imports()
    fix_crypto_pem_conflict()
    remove_duplicate_types()
    fix_ast_node_type()
    fix_specific_external_imports()
    
    print("Import fixes completed.")

if __name__ == "__main__":
    main()
