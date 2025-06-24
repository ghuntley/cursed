#!/bin/bash

# Fix malformed use statements where "use crate::error::Error;" 
# was incorrectly inserted in the middle of other use blocks

echo "🔧 Fixing malformed use statements across the codebase..."

# Find all .rs files and fix the pattern where "use crate::error::Error;" 
# is incorrectly inserted inside other use blocks
find src -name "*.rs" -type f | while read file; do
    # Check if file contains the malformed pattern
    if grep -q "use crate::error::Error;" "$file"; then
        echo "📝 Fixing $file"
        
        # Use sed to remove the malformed line that's inside use blocks
        # This is a complex pattern but we can use perl for better regex
        perl -i -pe '
            # If we see a line that is just "use crate::error::Error;" 
            # and the previous line ends with "{" or contains "use"
            # then remove this line
            if (/^use crate::error::Error;$/) {
                # Look for context - if previous lines suggest we are in a use block
                $_ = "" if $context_in_use_block;
            }
            # Track if we are in a use block context
            $context_in_use_block = 1 if /^use .+\{$/;
            $context_in_use_block = 0 if /^\};$/;
        ' "$file"
        
        # Also handle the case where it's embedded in inkwell use blocks specifically
        sed -i '/^use inkwell::{$/,/^};$/{
            /^use crate::error::Error;$/d
        }' "$file"
        
        # Handle the case where it's embedded in crate::ast use blocks
        sed -i '/^use crate::ast::{$/,/^};$/{
            /^use crate::error::Error;$/d
        }' "$file"
        
        # Handle the case where it's embedded in crate::optimization use blocks  
        sed -i '/^use crate::optimization::{$/,/^};$/{
            /^use crate::error::Error;$/d
        }' "$file"
        
        # Add proper error import if it was removed but still needed
        if ! grep -q "use crate::error::Error;" "$file" && grep -q "Error" "$file"; then
            # Add the import at the top after other crate imports
            sed -i '/^use crate::/a use crate::error::Error;' "$file"
        fi
    fi
done

echo "✅ Malformed use statement fixes completed!"
