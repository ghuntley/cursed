#!/usr/bin/env python3

import os
import re
import glob

def fix_squish_core_specific():
    """Fix the specific squish_core import issues"""
    file_path = "src/stdlib/squish_core/mod.rs"
    if not os.path.exists(file_path):
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Remove the problematic import line and fix the re-exports
    content = re.sub(
        r'use utils::\{\s*is_valid_compression_level[^}]*\};',
        '',
        content
    )
    
    # Add the correct re-exports
    utils_section = """pub use utils::{
    validate_compression_level as is_valid_compression_level,
    convert_quality_to_level as quality_to_level,
    get_recommended_buffer_size as recommended_buffer_size,
    use_parallel_compression as should_use_parallel,
    get_optimal_chunk_size as optimal_chunk_size,
};"""
    
    # Insert after the utils module declaration
    content = content.replace(
        'pub mod utils;',
        f'pub mod utils;\n\n{utils_section}'
    )
    
    with open(file_path, 'w') as f:
        f.write(content)
    print(f"Fixed squish_core specific imports in {file_path}")

def fix_e0308_type_mismatches():
    """Fix common E0308 type mismatch errors"""
    patterns = [
        # Fix `?` operator incompatible types
        (r'\.map_err\(Error::from\)\?', r'.map_err(|e| Error::IoError(e.to_string()))?'),
        (r'Result<.*?Error>', 'Result<(), Error>'),
        (r'Result<.*?CursedError>', 'Result<(), CursedError>'),
    ]
    
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        for old_pattern, new_pattern in patterns:
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed E0308 type mismatches in {file_path}")

def fix_e0659_ambiguous_types():
    """Fix E0659 ambiguous types errors"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix ambiguous `types` imports
        if 'use crate::types;' in content and 'use super::types;' in content:
            content = content.replace('use crate::types;', '')
            changed = True
        
        if 'types::' in content and 'use types' not in content:
            content = content.replace('types::', 'crate::types::')
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed E0659 ambiguous types in {file_path}")

def fix_e0412_missing_types():
    """Fix E0412 missing type errors more carefully"""
    missing_types = {
        'SecurityContext': 'crate::stdlib::web_vibez::SecurityContext',
        'EnhancedProcess': 'crate::stdlib::process::EnhancedProcess', 
        'Ed25519PublicKey': 'crate::stdlib::crypto::asymmetric::Ed25519PublicKey',
        'ASTNode': 'crate::ast::ASTNode',
    }
    
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        for type_name, full_path in missing_types.items():
            if type_name in content and f'use {full_path}' not in content and f'struct {type_name}' not in content:
                # Add import at the top
                lines = content.split('\n')
                import_line = f'use {full_path};'
                
                # Find the best place to insert the import
                insert_idx = 0
                for i, line in enumerate(lines):
                    if line.startswith('use ') or line.startswith('extern crate'):
                        insert_idx = i + 1
                    elif line.strip() and not line.startswith('//'):
                        break
                
                lines.insert(insert_idx, import_line)
                content = '\n'.join(lines)
                changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed E0412 missing types in {file_path}")

def fix_e0433_import_resolution():
    """Fix E0433 import resolution errors"""
    import_fixes = {
        'LlvmPackageConfig': 'crate::codegen::llvm::LlvmPackageConfig',
        'StatusCode': 'crate::stdlib::web_vibez::StatusCode',
        'IdentificationFactor': 'crate::stdlib::web_vibez::ratelimit::IdentificationFactor',
    }
    
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        for type_name, full_path in import_fixes.items():
            if type_name in content and full_path not in content:
                # Replace direct usage with full path
                content = content.replace(f'use {type_name}', f'use {full_path}')
                content = content.replace(f'::{type_name}', f'::{full_path.split("::")[-1]}')
                changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed E0433 import resolution in {file_path}")

def fix_e0404_trait_struct_confusion():
    """Fix E0404 trait vs struct confusion"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix TemplateEngine trait/struct confusion
        if 'impl TemplateEngine for' in content and 'struct TemplateEngine' in content:
            # Use a trait implementation approach
            content = content.replace(
                'impl TemplateEngine for',
                'impl TemplateEngineImpl for'
            )
            # Define the trait
            if 'trait TemplateEngineImpl' not in content:
                content = 'trait TemplateEngineImpl {\n    // Template engine implementation\n}\n\n' + content
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed E0404 trait/struct confusion in {file_path}")

def fix_e0609_field_access():
    """Fix E0609 field access errors"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        changed = False
        
        # Fix .name field access on Identifier
        if '.name' in content and 'Identifier' in content:
            # Replace with method call
            content = re.sub(r'(\w+)\.name\b', r'\1.name()', content)
            changed = True
        
        if changed:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed E0609 field access in {file_path}")

def revert_excessive_imports():
    """Remove excessive imports that were added"""
    for file_path in glob.glob("src/**/*.rs", recursive=True):
        if not os.path.exists(file_path):
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        lines = content.split('\n')
        filtered_lines = []
        
        for line in lines:
            # Skip excessive SecurityContext imports
            if 'use crate::types::SecurityContext;' in line and 'SecurityContext' not in ' '.join(lines[lines.index(line)+1:lines.index(line)+20]):
                continue
            # Skip excessive process imports
            if 'use crate::process::EnhancedProcess;' in line and 'EnhancedProcess' not in ' '.join(lines[lines.index(line)+1:lines.index(line)+20]):
                continue
            # Skip excessive crypto imports
            if 'use crate::crypto::Ed25519PublicKey;' in line and 'Ed25519PublicKey' not in ' '.join(lines[lines.index(line)+1:lines.index(line)+20]):
                continue
            # Skip excessive AST imports
            if 'use crate::ast::ASTNode;' in line and 'ASTNode' not in ' '.join(lines[lines.index(line)+1:lines.index(line)+20]):
                continue
            
            filtered_lines.append(line)
        
        new_content = '\n'.join(filtered_lines)
        if new_content != content:
            with open(file_path, 'w') as f:
                f.write(new_content)
            print(f"Reverted excessive imports in {file_path}")

def main():
    print("🎯 Fixing focused compilation errors...")
    
    # First revert the excessive imports
    revert_excessive_imports()
    
    # Fix specific known issues
    fix_squish_core_specific()
    fix_e0308_type_mismatches()
    fix_e0659_ambiguous_types()
    fix_e0412_missing_types()
    fix_e0433_import_resolution()
    fix_e0404_trait_struct_confusion()
    fix_e0609_field_access()
    
    print("✅ Completed focused compilation error fixes")

if __name__ == "__main__":
    main()
