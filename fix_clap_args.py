#!/usr/bin/env python3
import re
import sys

def fix_clap_args(content):
    # Replace Arg::with_name with Arg::new
    content = re.sub(r'Arg::with_name\(', 'Arg::new(', content)
    
    # Replace .short("x") with .short('x')
    content = re.sub(r'\.short\("([^"]+)"\)', r".short('\1')", content)
    
    # Remove .takes_value(true) as it's the default behavior in clap 4
    content = re.sub(r'\.takes_value\(true\)\s*', '', content)
    
    # Replace .multiple(true) with .num_args(1..)
    content = re.sub(r'\.multiple\(true\)', '.num_args(1..)', content)
    
    # Add .action(clap::ArgAction::SetTrue) for flag arguments (those without .takes_value)
    # This is a bit more complex - we need to find args that are flags
    
    return content

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 fix_clap_args.py <file_path>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    fixed_content = fix_clap_args(content)
    
    with open(file_path, 'w') as f:
        f.write(fixed_content)
    
    print(f"Fixed clap arguments in {file_path}")

if __name__ == '__main__':
    main()
