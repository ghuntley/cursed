#!/usr/bin/env python3
"""
Fix all non-exhaustive Object match patterns to include Char variant.
"""

import os
import re

def fix_match_patterns(content):
    """Add Char variant to Object match patterns."""
    
    # Pattern to match incomplete Object match statements
    patterns = [
        # Basic pattern for text rendering
        (
            r'(CursedObject::String\(s\) => Ok\(s\.clone\(\)\),\s*\n\s*CursedObject::Integer\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Float\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Boolean\(b\) => Ok\(b\.to_string\(\)\),)',
            r'\1\n            CursedObject::Char(c) => Ok(c.to_string()),'
        ),
        # Pattern for HTML rendering  
        (
            r'(CursedObject::String\(s\) => Ok\(html_escape\(s\)\),\s*\n\s*CursedObject::Integer\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Float\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Boolean\(b\) => Ok\(b\.to_string\(\)\),)',
            r'\1\n            CursedObject::Char(c) => Ok(html_escape(&c.to_string())),'
        ),
        # Pattern for JSON rendering
        (
            r'(CursedObject::String\(s\) => \{\s*let escaped = s\.replace\(.*?\);.*?Ok\(format!\("\\\".*?\\\"", escaped\)\)\s*\},\s*\n\s*CursedObject::Integer\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Float\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Boolean\(b\) => Ok\(b\.to_string\(\)\),)',
            r'\1\n            CursedObject::Char(c) => Ok(format!("\"{}\"", c)),'
        ),
        # Pattern for XML rendering  
        (
            r'(CursedObject::String\(s\) => Ok\(xml_escape\(s\)\),\s*\n\s*CursedObject::Integer\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Float\(n\) => Ok\(n\.to_string\(\)\),\s*\n\s*CursedObject::Boolean\(b\) => Ok\(b\.to_string\(\)\),)',
            r'\1\n            CursedObject::Char(c) => Ok(xml_escape(&c.to_string())),'
        ),
        # Pattern for table format
        (
            r'(CursedObject::String\(s\) => s\.clone\(\),\s*\n\s*CursedObject::Integer\(n\) => n\.to_string\(\),\s*\n\s*CursedObject::Float\(n\) => n\.to_string\(\),\s*\n\s*CursedObject::Boolean\(b\) => b\.to_string\(\),)',
            r'\1\n            CursedObject::Char(c) => c.to_string(),'
        ),
    ]
    
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content, flags=re.DOTALL)
    
    return content

def fix_simple_char_additions(content):
    """Add simple char handling to common patterns."""
    
    # Look for patterns that need Char handling
    lines = content.split('\n')
    fixed_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Look for match statements that need Char handling
        if 'CursedObject::Boolean(' in line and i < len(lines) - 10:
            # Look ahead to see if this is an Object match that needs Char
            context = '\n'.join(lines[i:i+10])
            if 'CursedObject::Nil' in context and 'CursedObject::Char' not in context:
                # This is likely a match that needs Char handling
                # Insert after the Boolean line
                fixed_lines.append(line)
                # Find appropriate insertion point based on context
                if 'Ok(' in line or '=>' in line:
                    if 'true' in line or 'false' in line:
                        fixed_lines.append(line.replace('CursedObject::Boolean(b) => *b,', 'CursedObject::Boolean(b) => *b,').replace('CursedObject::Boolean(b) => *b', 'CursedObject::Boolean(b) => *b,\n            CursedObject::Char(_) => true, // Characters are always truthy'))
                        if 'CursedObject::Char' not in fixed_lines[-1]:
                            # Add the Char line manually
                            indent = ' ' * (len(line) - len(line.lstrip()))
                            if 'Ok(' in line:
                                fixed_lines.append(f'{indent}CursedObject::Char(c) => Ok(c.to_string()),')
                            elif 'true' in line or '=> *b' in line:
                                fixed_lines.append(f'{indent}CursedObject::Char(_) => true, // Characters are always truthy,')
                            else:
                                fixed_lines.append(f'{indent}CursedObject::Char(c) => c.to_string(),')
                    else:
                        fixed_lines.append(line)
                else:
                    fixed_lines.append(line)
            else:
                fixed_lines.append(line)
        else:
            fixed_lines.append(line)
        
        i += 1
    
    return '\n'.join(fixed_lines)

def process_file(filepath):
    """Process a single file to fix Char match patterns."""
    
    print(f"Fixing Char patterns in {filepath}")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes
        content = fix_match_patterns(content)
        
        # Manual fixes for specific patterns we know about
        if 'template_formats.rs' in filepath:
            # Add Char handling for render_text
            content = content.replace(
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n            CursedObject::Nil => Ok("".to_string()),',
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n            CursedObject::Char(c) => Ok(c.to_string()),\n            CursedObject::Nil => Ok("".to_string()),'
            )
            
            # Add Char handling for render_html  
            content = content.replace(
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n            CursedObject::Nil => Ok("".to_string()),',
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n            CursedObject::Char(c) => Ok(html_escape(&c.to_string())),\n            CursedObject::Nil => Ok("".to_string()),'
            )
            
            # For JSON format
            content = content.replace(
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n                CursedObject::Nil => Ok("null".to_string()),',
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n                CursedObject::Char(c) => Ok(format!("\\"{}\\""", c)),\n                CursedObject::Nil => Ok("null".to_string()),'
            )
            
            # For XML format  
            content = content.replace(
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n                CursedObject::Nil => Ok("".to_string()),',
                'CursedObject::Boolean(b) => Ok(b.to_string()),\n                CursedObject::Char(c) => Ok(xml_escape(&c.to_string())),\n                CursedObject::Nil => Ok("".to_string()),'
            )
            
            # For table format
            content = content.replace(
                'CursedObject::Boolean(b) => b.to_string(),\n                CursedObject::Nil => "".to_string(),',
                'CursedObject::Boolean(b) => b.to_string(),\n                CursedObject::Char(c) => c.to_string(),\n                CursedObject::Nil => "".to_string(),'
            )
        
        # Only write if content changed
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  ✅ Fixed Char patterns in {filepath}")
        else:
            print(f"  ⏭️  No changes needed in {filepath}")
            
    except Exception as e:
        print(f"  ❌ Error processing {filepath}: {e}")

def main():
    """Main function to fix Char patterns."""
    
    print("🔧 Fixing Object::Char match patterns...")
    
    # Files that likely need fixing
    files_to_fix = [
        "src/stdlib/template/template_formats.rs",
        "src/stdlib/template/template_render.rs", 
        "src/stdlib/template/template_filters.rs"
    ]
    
    for filepath in files_to_fix:
        if os.path.isfile(filepath):
            process_file(filepath)
    
    print("\n✅ Char pattern fixing completed!")

if __name__ == "__main__":
    main()
