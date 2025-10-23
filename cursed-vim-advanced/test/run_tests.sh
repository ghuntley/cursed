#!/bin/bash

# CURSED Vim Extension Test Runner
# This script runs the test suite for the CURSED Vim extension

set -e

echo "Running CURSED Vim Extension Tests..."

# Check if vim is available
if ! command -v vim &> /dev/null; then
    echo "Error: vim is not installed"
    exit 1
fi

# Check if nvim is available
if command -v nvim &> /dev/null; then
    VIM_CMD="nvim"
else
    VIM_CMD="vim"
fi

echo "Using $VIM_CMD for testing"

# Create test directory if it doesn't exist
mkdir -p test_output

# Run syntax highlighting tests
echo "Testing syntax highlighting..."
$VIM_CMD -u NONE -c "source test/test_syntax.vim" -c "call RunCursedTests()" -c "qa!" > test_output/syntax_test.log 2>&1

if [ $? -eq 0 ]; then
    echo "✓ Syntax highlighting tests passed"
else
    echo "✗ Syntax highlighting tests failed"
    cat test_output/syntax_test.log
    exit 1
fi

# Test filetype detection
echo "Testing filetype detection..."
echo 'vibe main
slay main() {
    vibez.spill("Hello")
}' > test_output/test_file.💀

$VIM_CMD -u NONE -c "source plugin/cursed.vim" -c "edit test_output/test_file.💀" -c "call assert_equal(&filetype, 'cursed')" -c "qa!" > test_output/filetype_test.log 2>&1

if [ $? -eq 0 ]; then
    echo "✓ Filetype detection tests passed"
else
    echo "✗ Filetype detection tests failed"
    cat test_output/filetype_test.log
    exit 1
fi

# Test plugin loading
echo "Testing plugin loading..."
$VIM_CMD -u NONE -c "set rtp+=." -c "source plugin/cursed.vim" -c "call assert_true(exists(':CursedRun'))" -c "qa!" > test_output/plugin_test.log 2>&1

if [ $? -eq 0 ]; then
    echo "✓ Plugin loading tests passed"
else
    echo "✗ Plugin loading tests failed"
    cat test_output/plugin_test.log
    exit 1
fi

# Clean up
rm -rf test_output

echo "All CURSED Vim extension tests passed! 🎉"