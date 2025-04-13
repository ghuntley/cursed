#!/bin/bash

echo "Tree-sitter Grammar Validation Script"
echo "=================================="
echo 

# Directory checks
echo "Checking directory structure..."
if [ -d "grammar" ] && [ -d "corpus" ] && [ -d "queries" ] && [ -d "examples" ]; then
  echo "✓ Directory structure is correct"
else
  echo "✗ Missing required directories"
  exit 1
fi

# Grammar file check
echo "\nChecking grammar file..."
if [ -f "grammar/cursed.js" ]; then
  echo "✓ Grammar file exists"
  # Basic syntax check (not perfect but can catch obvious issues)
  if grep -q "module.exports = grammar({" grammar/cursed.js; then
    echo "✓ Grammar file has correct structure"
  else
    echo "✗ Grammar file appears malformed"
    exit 1
  fi
else
  echo "✗ Grammar file missing"
  exit 1
fi

# Corpus files check
echo "\nChecking corpus files..."
CORPUS_COUNT=$(ls -1 corpus/*.txt 2>/dev/null | wc -l)
if [ "$CORPUS_COUNT" -gt 0 ]; then
  echo "✓ Found $CORPUS_COUNT corpus files"
  # Basic content check
  for file in corpus/*.txt; do
    if grep -q "=" "$file" && grep -q "\-\-\-" "$file"; then
      echo "  ✓ $file has correct structure"
    else
      echo "  ✗ $file appears malformed"
      exit 1
    fi
  done
else
  echo "✗ No corpus files found"
  exit 1
fi

# Queries check
echo "\nChecking query files..."
QUERY_FILES=("highlights.scm" "folds.scm" "indents.scm" "locals.scm")
for qf in "${QUERY_FILES[@]}"; do
  if [ -f "queries/$qf" ]; then
    echo "✓ $qf exists"
  else
    echo "✗ $qf missing"
    exit 1
  fi
done

# Check language-specific queries
if [ -d "queries/cursed" ]; then
  echo "✓ Language-specific queries directory exists"
  for qf in "${QUERY_FILES[@]}"; do
    if [ -f "queries/cursed/$qf" ]; then
      echo "  ✓ cursed/$qf exists"
    else
      echo "  × cursed/$qf missing (not critical)"
    fi
  done
else
  echo "✗ Language-specific queries directory missing"
fi

# Example files check
echo "\nChecking example files..."
EXAMPLE_COUNT=$(ls -1 examples/*.csd 2>/dev/null | wc -l)
if [ "$EXAMPLE_COUNT" -gt 0 ]; then
  echo "✓ Found $EXAMPLE_COUNT example files"
  # Basic content check
  for file in examples/*.csd; do
    if grep -q "vibe " "$file"; then
      echo "  ✓ $file starts with package declaration"
    else
      echo "  ✗ $file missing package declaration"
    fi
  done
else
  echo "✗ No example files found"
  exit 1
fi

# Package.json check
echo "\nChecking package.json..."
if [ -f "package.json" ]; then
  echo "✓ package.json exists"
  if grep -q "tree-sitter-cursed" package.json && grep -q "tree-sitter" package.json; then
    echo "✓ package.json has correct content"
  else
    echo "✗ package.json appears incorrect"
    exit 1
  fi
else
  echo "✗ package.json missing"
  exit 1
fi

# README check
echo "\nChecking README.md..."
if [ -f "README.md" ]; then
  echo "✓ README.md exists"
  if grep -q "tree-sitter-cursed" README.md; then
    echo "✓ README.md has correct content"
  else
    echo "✗ README.md appears incorrect"
    exit 1
  fi
else
  echo "✗ README.md missing"
  exit 1
fi

# Check for consistent file structure
echo "\nValidating corpus test examples..."
for file in corpus/*.txt; do
  # Check for expected sections (===== headers, --- separators, parse trees)
  if grep -q "=" "$file" && \
     grep -q "\-\-\-" "$file" && \
     grep -q "(source_file" "$file"; then
    echo "✓ $file has valid test structure"
  else
    echo "✗ $file has invalid test structure"
    exit 1
  fi
done

echo "\n✓ All checks passed! The grammar appears valid."
echo "Note: This is a static check only. For full validation, you would need to run:"
echo "  tree-sitter generate"
echo "  tree-sitter test"