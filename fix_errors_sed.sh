#!/bin/bash

# Replace all TypeCheckError { ... } with TypeCheckError::new(...) calls
# This is a complex sed script to handle the multiline patterns

# Create a backup
cp src/type_system/checker.rs src/type_system/checker.rs.backup

# Simple replacement for the most common pattern
sed -i 's/TypeCheckError {$/TypeCheckError::new(/g' src/type_system/checker.rs

echo "Replaced TypeCheckError struct initializers with constructor calls"
