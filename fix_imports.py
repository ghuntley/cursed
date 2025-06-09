#!/usr/bin/env python3
"""
Fix import and module structure issues in test files.
"""

import os
import re
import sys

def fix_ast_imports(content):
    """Fix AST module imports"""
    
    # Fix missing ast submodules
    content = re.sub(r'use cursed::ast::expressions::', 'use cursed::ast::', content)
    content = re.sub(r'use cursed::ast::statements::', 'use cursed::ast::', content) 
    content = re.sub(r'use cursed::ast::declarations::', 'use cursed::ast::', content)
    content = re.sub(r'use cursed::ast::control_flow::', 'use cursed::ast::', content)
    content = re.sub(r'use cursed::ast::base::', 'use cursed::ast::', content)
    
    return content

def fix_memory_imports(content):
    """Fix memory module imports"""
    
    # Remove non-existent memory imports
    content = re.sub(r'use cursed::memory::GoroutineGarbageCollector;?', '', content)
    content = re.sub(r'use cursed::memory::get_global_goroutine_gc;?', '', content)
    content = re.sub(r'use cursed::memory::SafePointType;?', '', content)
    
    return content

def fix_runtime_imports(content):
    """Fix runtime module imports"""
    
    # Remove non-existent runtime imports
    content = re.sub(r'use cursed::runtime::goroutine::get_global_scheduler;?', '', content)
    content = re.sub(r'use cursed::runtime::string_conversions;?', '', content)
    
    return content

def fix_codegen_imports(content):
    """Fix codegen module imports"""
    
    # Remove non-existent codegen imports
    content = re.sub(r'use cursed::codegen::llvm::StringConversions;?', '', content)
    content = re.sub(r'use cursed::codegen::llvm::StringConversionUtils;?', '', content)
    content = re.sub(r'use cursed::codegen::MonomorphizationManager;?', '', content)
    
    return content

def fix_tools_imports(content):
    """Fix tools module imports"""
    
    # Remove non-existent tools imports  
    content = re.sub(r'use cursed::tools::LintSeverity;?', '', content)
    content = re.sub(r'use cursed::tools::lint_source;?', '', content)
    
    return content

def fix_common_imports(content):
    """Fix common module imports"""
    
    # Remove non-existent common imports
    content = re.sub(r'use common::tracing::init_tracing;?', '', content)
    content = re.sub(r'use common::tracing::Timer;?', '', content)
    
    # Add init_tracing! macro call instead
    if 'init_tracing!()' not in content and ('tracing::' in content or 'info!' in content or 'debug!' in content):
        # Add macro at start of test function
        content = re.sub(r'(\#\[test\]\s*fn [^{]*\{)', r'\1\n    // init_tracing!();', content, flags=re.MULTILINE)
    
    return content

def fix_benchmark_imports(content):
    """Fix benchmark module imports"""
    
    # Remove non-existent benchmark imports
    content = re.sub(r'use cursed::benchmark;?', '', content)
    
    return content

def fix_syntax_errors(content):
    """Fix simple syntax errors"""
    
    # Fix unterminated string literals (common issue)
    # This is a simple fix - look for unmatched quotes
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Count quotes in line
        quote_count = line.count('"')
        if quote_count % 2 == 1 and not line.strip().endswith('\\'):
            # Odd number of quotes, likely unterminated
            if line.strip().endswith(';'):
                # Add closing quote before semicolon
                line = line.replace(';', '";', 1)
            else:
                # Add closing quote at end
                line = line.rstrip() + '"'
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def process_file(filepath):
    """Process a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_ast_imports(content)
        content = fix_memory_imports(content)
        content = fix_runtime_imports(content)
        content = fix_codegen_imports(content)
        content = fix_tools_imports(content)
        content = fix_common_imports(content)
        content = fix_benchmark_imports(content)
        content = fix_syntax_errors(content)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed imports: {filepath}")
            return True
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False
    
    return False

def main():
    """Main function"""
    if len(sys.argv) > 1:
        # Process specific files
        files = sys.argv[1:]
    else:
        # Find all test files
        files = []
        for root, dirs, filenames in os.walk('tests'):
            for filename in filenames:
                if filename.endswith('.rs'):
                    files.append(os.path.join(root, filename))
    
    fixed_count = 0
    for filepath in files:
        if process_file(filepath):
            fixed_count += 1
    
    print(f"Fixed imports in {fixed_count} files")

if __name__ == '__main__':
    main()
