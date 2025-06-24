#!/bin/bash

echo "🔧 Fixing remaining malformed use statements..."

# Function to fix a specific file
fix_file() {
    local file="$1"
    echo "📝 Fixing $file"
    
    # Create a temporary file
    temp_file=$(mktemp)
    
    # Process the file line by line with awk
    awk '
    BEGIN { in_use_block = 0 }
    
    # Detect start of use block
    /^use .+\{$/ { 
        in_use_block = 1
        print $0
        next
    }
    
    # Detect end of use block
    /^\};$/ { 
        in_use_block = 0
        print $0
        next
    }
    
    # If we are in a use block and encounter "use crate::error::Error;"
    # then skip this line and remember to add it after the block
    in_use_block && /^use crate::error::Error;$/ {
        add_error_import = 1
        next
    }
    
    # When use block ends, add the error import if needed
    in_use_block == 0 && add_error_import && /^\};$/ {
        print $0
        print "use crate::error::Error;"
        add_error_import = 0
        next
    }
    
    # Default: print the line
    { print $0 }
    ' "$file" > "$temp_file"
    
    # Replace the original file
    mv "$temp_file" "$file"
}

# Find files that still have the malformed pattern and fix them
while IFS= read -r file; do
    if [[ -f "$file" ]]; then
        fix_file "$file"
    fi
done < <(grep -l "use crate::error::Error;" src/**/*.rs 2>/dev/null)

echo "✅ Additional use statement fixes completed!"
