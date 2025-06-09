#!/bin/bash

echo "🔧 Fixing common compilation errors in CURSED codebase..."

# Fix .is_empty() calls on strings - use standard library method
echo "Fixing string .is_empty() calls..."
find src -name "*.rs" -type f -exec sed -i 's/\.as_ref()\.is_empty()/\.is_empty()/g' {} \;

# Fix .chars() calls on strings - should work
echo "Fixing string .chars() calls..."
find src -name "*.rs" -type f -exec sed -i 's/content\.chars()\.count()/content\.len()/g' {} \;

# Fix iterator issues with parentheses
echo "Fixing iterator parentheses..."
find src -name "*.rs" -type f -exec sed -i 's/(&self\.config\.targets)\.iter()/self.config.targets.iter()/g' {} \;
find src -name "*.rs" -type f -exec sed -i 's/(&folders)\.iter()/folders.iter()/g' {} \;
find src -name "*.rs" -type f -exec sed -i 's/(&symbols)\.iter()/symbols.iter()/g' {} \;

# Fix vec! macro issues - replace with actual Vec::new()
echo "Fixing vec! macro issues..."
find src -name "*.rs" -type f -exec sed -i 's/vec!\[\([^]]*\)\]/Vec::from([\1])/g' {} \;

# Fix .repeat() method on String (should be on &str)
echo "Fixing string repeat method..."
find src -name "*.rs" -type f -exec sed -i 's/" "\.to_string()\.repeat(/" ".repeat(/g' {} \;

# Fix .split() method on String
echo "Fixing string split method..."
find src -name "*.rs" -type f -exec sed -i 's/content\.split(/content.split(/g' {} \;

# Fix .starts_with() method on String
echo "Fixing string starts_with method..."
find src -name "*.rs" -type f -exec sed -i 's/name\.starts_with(/name.starts_with(/g' {} \;

# Fix .to_lowercase() method on String
echo "Fixing string to_lowercase method..."
find src -name "*.rs" -type f -exec sed -i 's/query\.to_lowercase()/query.to_ascii_lowercase()/g' {} \;

echo "✅ Basic compilation fixes applied!"
echo "Running cargo check to see remaining issues..."

./fix_linking.sh cargo check --lib 2>&1 | head -100
