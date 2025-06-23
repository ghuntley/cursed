#!/usr/bin/env python3

import os
import re

def fix_squish_core_specific_only():
    """Fix only the specific squish_core import issue"""
    file_path = "src/stdlib/squish_core/mod.rs"
    if not os.path.exists(file_path):
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Only fix the specific import line that was causing the error
    old_import = """use utils::{
    is_valid_compression_level, quality_to_level, recommended_buffer_size,
    should_use_parallel, optimal_chunk_size
};"""
    
    new_import = """use utils::{
    validate_compression_level as is_valid_compression_level,
    convert_quality_to_level as quality_to_level, 
    get_recommended_buffer_size as recommended_buffer_size,
    use_parallel_compression as should_use_parallel,
    get_optimal_chunk_size as optimal_chunk_size
};"""
    
    if old_import in content:
        content = content.replace(old_import, new_import)
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed squish_core import aliases in {file_path}")

def fix_identifier_name_access():
    """Fix .name access on Identifier types"""
    for root, dirs, files in os.walk("src"):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                
                with open(file_path, 'r') as f:
                    content = f.read()
                
                # Fix .name field access - replace with method call
                if '.name' in content and 'Identifier' in content:
                    # Replace identifier.name with identifier.name()
                    content = re.sub(r'(\w+)\.name\b(?!\()', r'\1.name()', content)
                    
                    with open(file_path, 'w') as f:
                        f.write(content)
                    print(f"Fixed identifier .name access in {file_path}")

def fix_template_engine_confusion():
    """Fix TemplateEngine trait vs struct issue"""
    file_path = "src/stdlib/template/template_core.rs"
    if os.path.exists(file_path):
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix trait vs struct issue by using a trait name
        if 'impl TemplateEngine for' in content and 'struct TemplateEngine' in content:
            content = content.replace('trait TemplateEngine', 'trait TemplateEngineCore')
            content = content.replace('impl TemplateEngine for', 'impl TemplateEngineCore for')
            
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed TemplateEngine trait/struct confusion in {file_path}")

def fix_result_operator_types():
    """Fix basic ? operator type mismatches"""
    patterns = [
        # Common error conversions
        (r'\.map_err\(Error::from\)', '.map_err(|e| CursedError::from(e))'),
        (r'\.map_err\(IoError\)', '.map_err(|e| CursedError::IoError(e.to_string()))'),
    ]
    
    for root, dirs, files in os.walk("src"):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                
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
                    print(f"Fixed ? operator types in {file_path}")

def fix_md5_digest_imports():
    """Fix md5::Md5 to md5::Digest"""
    file_path = "src/stdlib/packages/crypto_signatures/hash_algorithms.rs"
    if os.path.exists(file_path):
        with open(file_path, 'r') as f:
            content = f.read()
        
        if 'md5::Md5' in content:
            content = content.replace('md5::Md5', 'md5::Digest')
            
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed md5::Md5 to md5::Digest in {file_path}")

def main():
    print("🎯 Applying minimal focused fixes...")
    
    fix_squish_core_specific_only()
    fix_identifier_name_access()
    fix_template_engine_confusion()
    fix_result_operator_types()
    fix_md5_digest_imports()
    
    print("✅ Completed minimal focused fixes")

if __name__ == "__main__":
    main()
