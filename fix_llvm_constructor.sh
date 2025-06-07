#!/bin/bash

# Script to fix LlvmCodeGenerator::new() calls that are missing the 3rd PathBuf parameter

# Find all test files that have the old pattern and fix them
find tests/ -name "*.rs" -type f -exec grep -l 'LlvmCodeGenerator::new(&context, "[^"]*")' {} \; | while read file; do
    echo "Fixing $file"
    # Replace pattern: LlvmCodeGenerator::new(&context, "module_name") with LlvmCodeGenerator::new(&context, "module_name", std::path::PathBuf::from("test.csd"))
    sed -i 's/LlvmCodeGenerator::new(&context, "\([^"]*\)")/LlvmCodeGenerator::new(\&context, "\1", std::path::PathBuf::from("test.csd"))/g' "$file"
done

# Also handle cases with variables like &context
find tests/ -name "*.rs" -type f -exec grep -l 'LlvmCodeGenerator::new(&context, [^,)]*)$' {} \; | while read file; do
    echo "Fixing $file with variable"
    # This handles cases where module name might be a variable
    sed -i 's/LlvmCodeGenerator::new(&context, \([^,)]*\))$/LlvmCodeGenerator::new(\&context, \1, std::path::PathBuf::from("test.csd"))/g' "$file"
done
