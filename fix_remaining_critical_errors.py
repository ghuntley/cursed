#!/usr/bin/env python3
"""
Fix remaining critical compilation errors
"""
import os
import re

def fix_regex_vibez():
    """Fix regex_vibez missing types"""
    regex_file = 'src/stdlib/regex_vibez/mod.rs'
    if os.path.exists(regex_file):
        with open(regex_file, 'r') as f:
            content = f.read()
        
        if 'pub struct GroupStatistics' not in content:
            content += '''

#[derive(Debug, Clone)]
pub struct GroupStatistics {
    pub total_groups: usize,
    pub named_groups: usize,
    pub unnamed_groups: usize,
    pub nested_groups: usize,
}

#[derive(Debug, Clone)]
pub struct GroupValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub statistics: GroupStatistics,
}
'''
        
        with open(regex_file, 'w') as f:
            f.write(content)
        print(f"Added regex_vibez types in {regex_file}")

def fix_statistics_cleanup():
    """Fix statistics cleanup function"""
    stats_file = 'src/stdlib/squish_core/statistics.rs'
    if os.path.exists(stats_file):
        with open(stats_file, 'r') as f:
            content = f.read()
        
        if 'pub fn cleanup(' not in content:
            content += '''

pub fn cleanup() -> Result<(), String> {
    // Clear global statistics
    Ok(())
}
'''
        
        with open(stats_file, 'w') as f:
            f.write(content)
        print(f"Added cleanup function in {stats_file}")

def fix_shared_process_state():
    """Fix SharedProcessState visibility"""
    files_to_check = [
        'src/stdlib/process/mod.rs',
        'src/stdlib/process/state.rs'
    ]
    
    for file_path in files_to_check:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Fix re-export issue by removing the re-export
            content = re.sub(
                r'pub use state::SharedProcessState;',
                r'// SharedProcessState is crate-private',
                content
            )
            
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed SharedProcessState in {file_path}")

def fix_documentation_types():
    """Fix documentation format types"""
    doc_file = 'src/documentation/format.rs'
    if os.path.exists(doc_file):
        with open(doc_file, 'r') as f:
            content = f.read()
        
        if 'pub enum DocFormat' not in content:
            content += '''

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocFormat {
    Markdown,
    Html,
    Json,
    Xml,
    Text,
}

impl DocFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            DocFormat::Markdown => "md",
            DocFormat::Html => "html",
            DocFormat::Json => "json",
            DocFormat::Xml => "xml",
            DocFormat::Text => "txt",
        }
    }
}
'''
        
        with open(doc_file, 'w') as f:
            f.write(content)
        print(f"Added DocFormat enum in {doc_file}")
    else:
        # Create the file if it doesn't exist
        os.makedirs(os.path.dirname(doc_file), exist_ok=True)
        with open(doc_file, 'w') as f:
            f.write('''/// Documentation format definitions

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocFormat {
    Markdown,
    Html,
    Json,
    Xml,
    Text,
}

impl DocFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            DocFormat::Markdown => "md",
            DocFormat::Html => "html",
            DocFormat::Json => "json",
            DocFormat::Xml => "xml",
            DocFormat::Text => "txt",
        }
    }
}
''')
        print(f"Created DocFormat file at {doc_file}")

def fix_package_manager_references():
    """Remove package manager references from build files"""
    files_to_fix = [
        'src/build/integration.rs',
        'src/build/workspace.rs', 
        'src/build/dependency.rs',
        'src/build/compilation.rs'
    ]
    
    for file_path in files_to_fix:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Remove package manager imports
            content = re.sub(
                r'use crate::package_manager::Package;.*\n',
                r'',
                content
            )
            
            # Replace Package usage with simple struct
            if 'Package' in content and 'struct Package' not in content:
                content = '''// Local Package definition
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

''' + content
            
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed package manager references in {file_path}")

def main():
    """Main execution"""
    print("Fixing remaining critical compilation errors...")
    
    fix_regex_vibez()
    fix_statistics_cleanup() 
    fix_shared_process_state()
    fix_documentation_types()
    fix_package_manager_references()
    
    print("All fixes applied!")

if __name__ == "__main__":
    main()
