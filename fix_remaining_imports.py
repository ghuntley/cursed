#!/usr/bin/env python3
"""
Fix remaining import issues.
"""

import os
import re
from pathlib import Path

def fix_comprehensive_performance_system():
    """Fix PgoConfig import in comprehensive_performance_system."""
    print("Fixing comprehensive_performance_system...")
    
    comp_perf = Path("src/optimization/comprehensive_performance_system.rs")
    if comp_perf.exists():
        content = comp_perf.read_text()
        
        # Add import if missing
        if "use crate::optimization::PgoConfig;" not in content:
            print("Adding PgoConfig import to comprehensive_performance_system.rs")
            lines = content.split('\n')
            # Find where to insert the import
            for i, line in enumerate(lines):
                if line.startswith("use") and "crate::" in line:
                    lines.insert(i+1, "use crate::optimization::PgoConfig;")
                    break
            content = '\n'.join(lines)
            comp_perf.write_text(content)

def fix_lexer_token_access():
    """Fix lexer token access in LSP."""
    print("Fixing lexer token access...")
    
    lsp_symbols = Path("src/lsp/enhanced_symbols.rs")
    if lsp_symbols.exists():
        content = lsp_symbols.read_text()
        
        # Replace lexer::token with lexer
        content = content.replace("crate::lexer::token::", "crate::lexer::")
        lsp_symbols.write_text(content)

def fix_external_dependencies():
    """Fix external dependency issues."""
    print("Fixing external dependencies...")
    
    # Fix x509_parser issues
    crypto_cert = Path("src/stdlib/crypto/certificates.rs")
    if crypto_cert.exists():
        content = crypto_cert.read_text()
        
        # Replace specific x509_parser usage
        if "x509_parser::name" in content:
            print("Fixing x509_parser usage in certificates.rs")
            content = content.replace("x509_parser::name", "x509_parser::prelude")
            content = content.replace("x509_parser::algorithm", "x509_parser::prelude")
        
        crypto_cert.write_text(content)
    
    # Fix reqwest multipart
    http_client = Path("src/stdlib/glowup_http/client.rs")
    if http_client.exists():
        content = http_client.read_text()
        
        if "reqwest::multipart" in content and "use reqwest::multipart" not in content:
            print("Adding multipart import to client.rs")
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.startswith("use reqwest"):
                    lines.insert(i+1, "use reqwest::multipart;")
                    break
            content = '\n'.join(lines)
            http_client.write_text(content)
    
    # Fix ed25519_dalek pkcs8
    ed25519_file = Path("src/stdlib/packages/crypto_asymmetric/ed25519.rs")
    if ed25519_file.exists():
        content = ed25519_file.read_text()
        
        if "ed25519_dalek::pkcs8" in content:
            print("Fixing ed25519_dalek pkcs8 usage")
            # Replace with available API
            content = content.replace("ed25519_dalek::pkcs8", "pkcs8")
            
            # Add pkcs8 import if needed
            if "use pkcs8" not in content:
                lines = content.split('\n')
                for i, line in enumerate(lines):
                    if line.startswith("use ed25519_dalek"):
                        lines.insert(i+1, "use pkcs8;")
                        break
                content = '\n'.join(lines)
            
            ed25519_file.write_text(content)

def main():
    """Main function."""
    print("Fixing remaining import issues...")
    
    fix_comprehensive_performance_system()
    fix_lexer_token_access()
    fix_external_dependencies()
    
    print("Remaining import fixes completed.")

if __name__ == "__main__":
    main()
