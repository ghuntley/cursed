#!/bin/bash
# CURSED Pre-commit Hook
# This script automatically formats code before commits

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Running CURSED pre-commit hooks...${NC}"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Get list of staged files
STAGED_RUST_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)
STAGED_CURSED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.csd$' || true)

# Track if any files were modified
FILES_MODIFIED=false

# Format Rust files
if [ -n "$STAGED_RUST_FILES" ]; then
    echo -e "${YELLOW}Formatting Rust files...${NC}"
    
    # Check if cargo fmt is available
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo not found. Please install Rust.${NC}"
        exit 1
    fi
    
    # Format the files
    cargo fmt --quiet
    
    # Check if any staged Rust files were modified
    for file in $STAGED_RUST_FILES; do
        if [ -f "$file" ] && git diff --name-only | grep -q "$file"; then
            FILES_MODIFIED=true
            # Re-stage the formatted file
            git add "$file"
            echo -e "${GREEN}Formatted and restaged: $file${NC}"
        fi
    done
fi

# Format CURSED files
if [ -n "$STAGED_CURSED_FILES" ]; then
    echo -e "${YELLOW}Formatting CURSED files...${NC}"
    
    # Build the CURSED formatter if it doesn't exist or is out of date
    if [ ! -f "./target/debug/cursed-fmt" ] || [ "src/bin/cursed_fmt.rs" -nt "./target/debug/cursed-fmt" ]; then
        echo -e "${YELLOW}Building CURSED formatter...${NC}"
        cargo build --bin cursed-fmt --quiet
    fi
    
    # Format each staged CURSED file
    for file in $STAGED_CURSED_FILES; do
        if [ -f "$file" ]; then
            echo -e "${YELLOW}Formatting: $file${NC}"
            
            # Create a backup
            cp "$file" "$file.backup"
            
            # Format the file
            if ./target/debug/cursed-fmt "$file"; then
                # Check if the file was modified
                if ! cmp -s "$file" "$file.backup"; then
                    FILES_MODIFIED=true
                    # Re-stage the formatted file
                    git add "$file"
                    echo -e "${GREEN}Formatted and restaged: $file${NC}"
                fi
            else
                echo -e "${RED}Error formatting $file${NC}"
                # Restore backup on error
                mv "$file.backup" "$file"
                exit 1
            fi
            
            # Clean up backup
            rm -f "$file.backup"
        fi
    done
fi

# Run basic checks
echo -e "${YELLOW}Running basic checks...${NC}"

# Check that the project still builds
if [ -n "$STAGED_RUST_FILES" ]; then
    echo -e "${YELLOW}Checking that Rust code compiles...${NC}"
    if ! cargo check --quiet; then
        echo -e "${RED}Error: Code does not compile after formatting${NC}"
        exit 1
    fi
fi

# Lint check for Rust files
if [ -n "$STAGED_RUST_FILES" ]; then
    echo -e "${YELLOW}Running Rust linter...${NC}"
    if ! cargo clippy --quiet -- -D warnings; then
        echo -e "${RED}Warning: Clippy found issues. Consider fixing them before committing.${NC}"
        # Don't fail on clippy warnings, just warn
    fi
fi

# Final message
if [ "$FILES_MODIFIED" = true ]; then
    echo -e "${GREEN}✓ Pre-commit hooks completed. Some files were formatted and restaged.${NC}"
    echo -e "${YELLOW}Please review the changes and commit again.${NC}"
    exit 1  # Exit with error to prevent commit, so user can review changes
else
    echo -e "${GREEN}✓ Pre-commit hooks completed. All files are properly formatted.${NC}"
fi

echo -e "${GREEN}Ready to commit!${NC}"
