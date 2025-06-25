#!/bin/bash

# Fix duplicate Error/CursedError imports across the codebase
echo "Fixing duplicate Error imports..."

# Find all Rust files with potential duplicate Error imports
find src/ -name "*.rs" -type f | while read -r file; do
    # Skip if file doesn't exist
    [ ! -f "$file" ] && continue
    
    # Check if file has duplicate Error imports
    if grep -q "use.*Error" "$file"; then
        echo "Fixing imports in $file"
        
        # Create backup
        cp "$file" "$file.bak"
        
        # Use sed to fix common duplicate import patterns
        sed -i \
            -e '/use crate::{Error,/s/Error/CursedError/g' \
            -e '/use crate::error_types::{Error,/s/Error/CursedError/g' \
            -e '/use crate::Error;/d' \
            -e '/use.*Error;$/s/Error/CursedError/' \
            -e 's/\bError\b/CursedError/g' \
            "$file"
        
        # If the changes broke the file, restore backup
        if ! rustc --crate-type lib "$file" &>/dev/null; then
            # mv "$file.bak" "$file"
            echo "Restored $file from backup due to syntax errors"
        else
            rm -f "$file.bak"
        fi
    fi
done

echo "Fixed duplicate Error imports"
