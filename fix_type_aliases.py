#!/usr/bin/env python3

import os
import re
import subprocess

def get_error_type_for_file(filepath):
    """Determine the appropriate error type based on the file path and contents."""
    
    # Read the file to see what error types are available
    try:
        with open(filepath, 'r') as f:
            content = f.read()
    except:
        return "Error"
    
    # Map based on file path patterns
    path_mappings = {
        'error_propagation': 'PropagationError',
        'async': 'AsyncError',
        'channels': 'ChannelError', 
        'buffer': 'ChannelError',
        'future': 'AsyncError',
        'preprocessor': 'PreprocessorError',
        'database': 'DatabaseError',
        'sqlite': 'SqliteError',
        'postgres': 'PostgresError',
        'mysql': 'MySqlError',
        'crypto': 'CryptoError',
        'advanced_crypto': 'AdvancedCryptoError',
        'pqc': 'PqcError',
        'hash': 'HashError',
        'signature': 'SignatureError',
        'pki': 'PkiError',
        'kdf': 'KdfError',
        'http': 'HttpError',
        'web': 'WebError',
        'net': 'NetError',
        'io': 'IoError',
        'fs': 'FsError',
        'process': 'ProcessError',
        'ipc': 'IpcError',
        'sync': 'SyncError',
        'time': 'TimeError',
        'json': 'JsonError',
        'csv': 'CsvError',
        'testing': 'TestError',
        'template': 'TemplateError',
        'repl': 'ReplError',
        'string': 'StringError',
        'math': 'MathError',
        'collections': 'CollectionsError',
        'regex': 'RegexError',
        'embed': 'EmbedError',
        'system': 'SystemError',
        'env': 'EnvError',
        'glowup': 'GlowUpError',
        'profiler': 'ProfilerError',
        'squish': 'SquishError',
        'plugin': 'PluginError',
        'signal': 'SignalBoostError',
        'lookin': 'LookinGlassError',
        'chaos': 'ChaosError',
        'vibe': 'VibeError',
        'core': 'CoreError',
        'cursed': 'CursedError'
    }
    
    # Check specific error types mentioned in the file
    if 'Error' in content:
        # Look for specific error enum definitions
        error_enums = re.findall(r'pub enum (\w*Error)', content)
        if error_enums:
            return error_enums[0]
    
    # Use path-based mapping
    filepath_lower = filepath.lower()
    for key, error_type in path_mappings.items():
        if key in filepath_lower:
            return error_type
    
    # Default fallback
    return "Error"

def fix_type_alias(filepath, line_content):
    """Fix a single type alias line."""
    
    # Extract the type alias name
    match = re.search(r'pub type (\w+)<\(\), Error>', line_content)
    if not match:
        return line_content
    
    type_name = match.group(1)
    error_type = get_error_type_for_file(filepath)
    
    # Special handling for some cases
    if 'Result' in type_name and 'Result' not in type_name[:-6]:
        # Generic Result type
        new_line = f"pub type {type_name}<T> = std::result::Result<T, {error_type}>;"
    else:
        # Specific result type
        new_line = f"pub type {type_name}<T> = std::result::Result<T, {error_type}>;"
    
    return new_line

# Get all files with the malformed pattern
def get_files_to_fix():
    result = subprocess.run(['grep', '-r', '-l', 'pub type.*<(), .*>', 'src/'], 
                          capture_output=True, text=True)
    if result.returncode == 0:
        return [f.strip() for f in result.stdout.split('\n') if f.strip()]
    return []

def fix_file(filepath):
    """Fix all type aliases in a single file."""
    try:
        with open(filepath, 'r') as f:
            lines = f.readlines()
        
        modified = False
        for i, line in enumerate(lines):
            if re.search(r'pub type.*<\(\), .*>', line):
                new_line = fix_type_alias(filepath, line.strip()) + '\n'
                if new_line != line:
                    lines[i] = new_line
                    modified = True
                    print(f"Fixed: {filepath}:{i+1}")
        
        if modified:
            with open(filepath, 'w') as f:
                f.writelines(lines)
            return True
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False
    
    return False

def main():
    files_to_fix = get_files_to_fix()
    print(f"Found {len(files_to_fix)} files to fix")
    
    fixed_count = 0
    for filepath in files_to_fix:
        if fix_file(filepath):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()
