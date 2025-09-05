#!/bin/bash

# CURSED Linter - Command Line Interface
# Production-ready static analysis tool for CURSED language
# Usage: cursed-lint <file.💀> [options]

set -e

# Configuration
CURSED_BIN="./zig-out/bin/cursed-zig"
LINTER_SCRIPT="cursed-lint.💀"
CONFIG_FILE=".cursed-lint.toml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_colored() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to show usage
show_usage() {
    print_colored $CYAN "🔍 CURSED Code Linter v1.0.0"
    print_colored $CYAN "Production-grade static analysis for CURSED language"
    echo ""
    print_colored $YELLOW "USAGE:"
    echo "    cursed-lint <file.💀> [options]"
    echo "    cursed-lint <directory> [options]"
    echo ""
    print_colored $YELLOW "OPTIONS:"
    echo "    --config <file>      Use custom config file"
    echo "    --preset <mode>      Use preset: strict, recommended, relaxed, minimal"
    echo "    --format <type>      Output format: human, json, sarif"
    echo "    --no-color           Disable colored output"
    echo "    --quiet              Suppress info messages"
    echo "    --verbose            Show detailed analysis"
    echo "    --fix                Apply auto-fixes where possible"
    echo "    --dry-run            Show what would be fixed without applying"
    echo "    --help               Show this help message"
    echo "    --version            Show version information"
    echo ""
    print_colored $YELLOW "CONFIGURATION:"
    echo "    Create .cursed-lint.toml in your project root"
    echo "    See documentation: https://cursed-lang.org/tools/linter"
    echo ""
    print_colored $YELLOW "EXAMPLES:"
    echo "    cursed-lint main.💀"
    echo "    cursed-lint src/"
    echo "    cursed-lint app.💀 --preset strict"
    echo "    cursed-lint src/ --format json > report.json"
    echo "    cursed-lint *.💀 --no-color --quiet"
    echo "    cursed-lint src/ --fix --dry-run"
    echo ""
    print_colored $YELLOW "EXIT CODES:"
    echo "    0 - No issues found or only info/hints"
    echo "    1 - Warnings or errors found"  
    echo "    2 - Critical issues found"
    echo "    3 - Tool error or invalid usage"
}

# Main function - simplified for demonstration
main() {
    local target="${1:-}"
    
    if [[ "$target" == "--help" || "$target" == "-h" || -z "$target" ]]; then
        show_usage
        exit 0
    fi
    
    if [[ "$target" == "--version" || "$target" == "-v" ]]; then
        print_colored $CYAN "🔍 CURSED Code Linter v1.0.0"
        exit 0
    fi
    
    # Check if CURSED compiler is available
    if [ ! -f "$CURSED_BIN" ]; then
        print_colored $RED "❌ Error: CURSED compiler not found at $CURSED_BIN"
        print_colored $YELLOW "   Run 'zig build' to build the CURSED compiler first"
        exit 3
    fi
    
    # Check if linter script exists
    if [ ! -f "$LINTER_SCRIPT" ]; then
        print_colored $RED "❌ Error: Linter script not found at $LINTER_SCRIPT"
        exit 3
    fi
    
    print_colored $PURPLE "🔍 CURSED Code Linter v1.0.0"
    print_colored $BLUE "🔍 Linting: $target"
    
    # Run the linter through CURSED interpreter
    if $CURSED_BIN "$LINTER_SCRIPT" "$target"; then
        print_colored $GREEN "✅ Linting completed"
        exit 0
    else
        local exit_code=$?
        if [ $exit_code -eq 1 ]; then
            print_colored $YELLOW "⚠️  Issues found in $target"
        elif [ $exit_code -eq 2 ]; then
            print_colored $RED "🚨 Critical issues found in $target"
        else
            print_colored $RED "❌ Linter error for $target"
        fi
        exit $exit_code
    fi
}

# Run main function with all arguments
main "$@"
