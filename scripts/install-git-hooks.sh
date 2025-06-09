#!/bin/bash
# Install Git Hooks for CURSED Development
# This script sets up pre-commit hooks for automatic formatting

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}CURSED Git Hooks Installer${NC}"
echo -e "${BLUE}=========================${NC}"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Get the git hooks directory
GIT_HOOKS_DIR=$(git rev-parse --git-dir)/hooks
PRE_COMMIT_HOOK="$GIT_HOOKS_DIR/pre-commit"

echo -e "${YELLOW}Git hooks directory: $GIT_HOOKS_DIR${NC}"

# Check if pre-commit hook already exists
if [ -f "$PRE_COMMIT_HOOK" ]; then
    echo -e "${YELLOW}Pre-commit hook already exists.${NC}"
    
    # Check if it's our hook
    if grep -q "CURSED Pre-commit Hook" "$PRE_COMMIT_HOOK" 2>/dev/null; then
        echo -e "${GREEN}Existing hook is already the CURSED pre-commit hook.${NC}"
        
        read -p "Do you want to update it? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${YELLOW}Keeping existing hook.${NC}"
            exit 0
        fi
    else
        echo -e "${YELLOW}Found existing pre-commit hook (not CURSED).${NC}"
        echo -e "${YELLOW}Creating backup: $PRE_COMMIT_HOOK.backup${NC}"
        cp "$PRE_COMMIT_HOOK" "$PRE_COMMIT_HOOK.backup"
        
        read -p "Do you want to replace it with the CURSED hook? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${YELLOW}Keeping existing hook.${NC}"
            exit 0
        fi
    fi
fi

# Create hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Copy our pre-commit hook
echo -e "${YELLOW}Installing CURSED pre-commit hook...${NC}"
cp "scripts/pre-commit-hook.sh" "$PRE_COMMIT_HOOK"
chmod +x "$PRE_COMMIT_HOOK"

echo -e "${GREEN}✓ Pre-commit hook installed successfully!${NC}"
echo ""
echo -e "${BLUE}What this hook does:${NC}"
echo -e "  • Automatically formats Rust files with ${YELLOW}cargo fmt${NC}"
echo -e "  • Automatically formats CURSED files with ${YELLOW}cursed-fmt${NC}"
echo -e "  • Runs basic compilation checks"
echo -e "  • Runs Rust linter (clippy) for warnings"
echo -e "  • Re-stages modified files automatically"
echo ""
echo -e "${BLUE}Usage:${NC}"
echo -e "  • Just commit as usual with ${YELLOW}git commit${NC}"
echo -e "  • The hook will run automatically before each commit"
echo -e "  • If files are formatted, you'll need to commit again"
echo -e "  • Use ${YELLOW}git commit --no-verify${NC} to skip the hook if needed"
echo ""
echo -e "${BLUE}Manual formatting commands:${NC}"
echo -e "  • ${YELLOW}make fmt${NC}      - Format all CURSED files"
echo -e "  • ${YELLOW}make fmt-check${NC} - Check CURSED formatting (CI)"
echo -e "  • ${YELLOW}make fmt-diff${NC}  - Show formatting differences"
echo -e "  • ${YELLOW}make fmt-fix${NC}   - Format Rust files"
echo ""
echo -e "${GREEN}Happy coding! 🚀${NC}"

# Test the hook installation
echo -e "${YELLOW}Testing hook installation...${NC}"
if [ -x "$PRE_COMMIT_HOOK" ]; then
    echo -e "${GREEN}✓ Hook is executable${NC}"
else
    echo -e "${RED}✗ Hook is not executable${NC}"
    exit 1
fi

# Optional: Build the formatter to ensure it's ready
echo -e "${YELLOW}Building CURSED formatter to ensure it's ready...${NC}"
if cargo build --bin cursed-fmt --quiet; then
    echo -e "${GREEN}✓ CURSED formatter built successfully${NC}"
else
    echo -e "${RED}✗ Failed to build CURSED formatter${NC}"
    echo -e "${YELLOW}The hook will build it automatically when needed.${NC}"
fi

echo -e "${GREEN}Installation complete!${NC}"
