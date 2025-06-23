#!/usr/bin/env python3

import os
import re
import glob

def fix_squish_core_imports():
    """Fix import issues in squish_core module"""
    file_path = "src/stdlib/squish_core/mod.rs"
    if not os.path.exists(file_path):
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix the import aliases
    fixes = [
        ('is_valid_compression_level', 'validate_compression_level'),
        ('quality_to_level', 'convert_quality_to_level'),
        ('recommended_buffer_size', 'get_recommended_buffer_size'),
        ('should_use_parallel', 'use_parallel_compression'),
        ('optimal_chunk_size', 'get_optimal_chunk_size'),
    ]
    
    for old_name, new_name in fixes:
        content = content.replace(old_name, new_name)
    
    with open(file_path, 'w') as f:
        f.write(content)
    print(f"Fixed squish_core imports in {file_path}")

def fix_missing_types():
    """Fix missing type definitions"""
    # Fix SecurityContext imports
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix SecurityContext references
        if "SecurityContext" in content and "use" not in content or "struct SecurityContext" not in content:
            if "use crate::types::SecurityContext;" not in content:
                content = "use crate::types::SecurityContext;\n" + content
                changed = True
        
        # Fix EnhancedProcess references
        if "EnhancedProcess" in content and "use crate::process::EnhancedProcess;" not in content:
            content = "use crate::process::EnhancedProcess;\n" + content
            changed = True
            
        # Fix Ed25519PublicKey references
        if "Ed25519PublicKey" in content and "use crate::crypto::Ed25519PublicKey;" not in content:
            content = "use crate::crypto::Ed25519PublicKey;\n" + content
            changed = True
            
        # Fix ASTNode references
        if "ASTNode" in content and "use crate::ast::ASTNode;" not in content:
            content = "use crate::ast::ASTNode;\n" + content
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed missing type imports in {file_path}")

def fix_llvm_imports():
    """Fix LLVM-related import issues"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix LlvmPackageConfig
        if "LlvmPackageConfig" in content:
            content = content.replace("llvm::LlvmPackageConfig", "crate::codegen::llvm::LlvmPackageConfig")
            content = content.replace("use crate::llvm::LlvmPackageConfig", "use crate::codegen::llvm::LlvmPackageConfig")
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed LLVM imports in {file_path}")

def fix_stdlib_error_imports():
    """Fix stdlib error import issues"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix stdlib::error imports
        if "stdlib::error" in content:
            content = content.replace("stdlib::error", "crate::stdlib::error")
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed stdlib error imports in {file_path}")

def fix_template_engine_trait_usage():
    """Fix TemplateEngine trait vs struct confusion"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix trait vs struct usage
        if "impl TemplateEngine for" in content and "struct TemplateEngine" in content:
            # Change the impl to use a different trait name
            content = content.replace("impl TemplateEngine for", "impl TemplateEngineImpl for")
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed TemplateEngine trait/struct confusion in {file_path}")

def fix_identifier_field_access():
    """Fix Identifier field access issues"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix .name field access on Identifier
        if ".name" in content and "Identifier" in content:
            content = re.sub(r'(\w+)\.name', r'\1.to_string()', content)
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed Identifier field access in {file_path}")

def fix_statuscode_imports():
    """Fix StatusCode import issues"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Add StatusCode import
        if "StatusCode" in content and "use" in content and "StatusCode" not in content.split('\n')[0:10]:
            content = "use crate::web::StatusCode;\n" + content
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed StatusCode imports in {file_path}")

def fix_md5_imports():
    """Fix md5 crate imports"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix Md5 imports
        if "md5::Md5" in content:
            content = content.replace("md5::Md5", "md5::Digest")
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed md5 imports in {file_path}")

def main():
    print("🔧 Fixing top compilation errors...")
    
    fix_squish_core_imports()
    fix_missing_types() 
    fix_llvm_imports()
    fix_stdlib_error_imports()
    fix_template_engine_trait_usage()
    fix_identifier_field_access()
    fix_statuscode_imports()
    fix_md5_imports()
    
    print("✅ Completed fixing top compilation errors")

if __name__ == "__main__":
    main()
