#!/bin/bash
# CURSED Development Environment Setup
# This script sets up the complete development environment for CURSED

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${PURPLE}🚀 CURSED Development Environment Setup${NC}"
echo -e "${PURPLE}=====================================${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${BLUE}📋 Checking prerequisites...${NC}"

if ! command_exists git; then
    echo -e "${RED}✗ Git is not installed${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Git found${NC}"

if ! command_exists cargo; then
    echo -e "${RED}✗ Rust/Cargo is not installed${NC}"
    echo -e "${YELLOW}Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Rust/Cargo found${NC}"

# Check if we're in the CURSED repository
if [ ! -f "Cargo.toml" ] || ! grep -q "name = \"cursed\"" Cargo.toml; then
    echo -e "${RED}✗ Not in CURSED repository root${NC}"
    exit 1
fi
echo -e "${GREEN}✓ CURSED repository detected${NC}"

echo ""

# Build the project
echo -e "${BLUE}🔨 Building CURSED project...${NC}"
cargo build
echo -e "${GREEN}✓ Project built successfully${NC}"

# Build the formatter specifically
echo -e "${BLUE}🎨 Building CURSED formatter...${NC}"
cargo build --bin cursed-fmt
echo -e "${GREEN}✓ CURSED formatter built successfully${NC}"

echo ""

# Install git hooks
echo -e "${BLUE}🪝 Setting up Git hooks...${NC}"
if [ -f "scripts/install-git-hooks.sh" ]; then
    ./scripts/install-git-hooks.sh
else
    echo -e "${YELLOW}⚠ Git hooks script not found, skipping...${NC}"
fi

echo ""

# Check code formatting
echo -e "${BLUE}🎨 Checking code formatting...${NC}"

# Check Rust formatting
echo -e "${YELLOW}Checking Rust code formatting...${NC}"
if cargo fmt -- --check; then
    echo -e "${GREEN}✓ Rust code is properly formatted${NC}"
else
    echo -e "${YELLOW}⚠ Rust code needs formatting${NC}"
    echo -e "${YELLOW}Run 'make fmt-fix' to fix Rust formatting${NC}"
fi

# Check CURSED formatting
echo -e "${YELLOW}Checking CURSED code formatting...${NC}"
CURSED_FILES=$(find . -name '*.csd' -not -path './target/*' -not -path './.git/*' | head -5)
if [ -n "$CURSED_FILES" ]; then
    if echo "$CURSED_FILES" | xargs ./target/debug/cursed-fmt --check; then
        echo -e "${GREEN}✓ CURSED code is properly formatted${NC}"
    else
        echo -e "${YELLOW}⚠ CURSED code needs formatting${NC}"
        echo -e "${YELLOW}Run 'make fmt' to fix CURSED formatting${NC}"
    fi
else
    echo -e "${BLUE}ℹ No CURSED (.csd) files found${NC}"
fi

echo ""

# Run basic tests
echo -e "${BLUE}🧪 Running basic tests...${NC}"
if cargo test --quiet 2>/dev/null; then
    echo -e "${GREEN}✓ Basic tests passed${NC}"
else
    echo -e "${YELLOW}⚠ Some tests failed (this might be expected in development)${NC}"
fi

echo ""

# Check linting
echo -e "${BLUE}🔍 Running linter...${NC}"
if cargo clippy --quiet -- -D warnings 2>/dev/null; then
    echo -e "${GREEN}✓ No linting issues found${NC}"
else
    echo -e "${YELLOW}⚠ Linting issues found (run 'make lint' for details)${NC}"
fi

echo ""

# Display configuration
echo -e "${BLUE}⚙️ Configuration files:${NC}"
if [ -f ".cursed_fmt.toml" ]; then
    echo -e "${GREEN}✓ CURSED formatter config: .cursed_fmt.toml${NC}"
else
    echo -e "${YELLOW}⚠ CURSED formatter config not found${NC}"
fi

if [ -f "Cargo.toml" ]; then
    echo -e "${GREEN}✓ Rust project config: Cargo.toml${NC}"
fi

echo ""

# Display useful commands
echo -e "${PURPLE}📚 Useful development commands:${NC}"
echo ""
echo -e "${BLUE}Building:${NC}"
echo -e "  ${YELLOW}make build${NC}              - Build the project"
echo -e "  ${YELLOW}make clean${NC}              - Clean build artifacts"
echo ""
echo -e "${BLUE}Testing:${NC}"
echo -e "  ${YELLOW}make test${NC}               - Run all tests"
echo -e "  ${YELLOW}make test-quiet${NC}         - Run tests without warnings"
echo -e "  ${YELLOW}make jit-test${NC}           - Run JIT integration tests"
echo ""
echo -e "${BLUE}Formatting:${NC}"
echo -e "  ${YELLOW}make fmt${NC}                - Format CURSED files"
echo -e "  ${YELLOW}make fmt-check${NC}          - Check CURSED formatting (CI)"
echo -e "  ${YELLOW}make fmt-diff${NC}           - Show formatting differences"
echo -e "  ${YELLOW}make fmt-fix${NC}            - Format Rust files"
echo -e "  ${YELLOW}make fmt-help${NC}           - Show formatting help"
echo ""
echo -e "${BLUE}Linting:${NC}"
echo -e "  ${YELLOW}make lint${NC}               - Run clippy with strict warnings"
echo -e "  ${YELLOW}make lint-allow${NC}         - Run clippy allowing warnings"
echo ""
echo -e "${BLUE}Examples:${NC}"
echo -e "  ${YELLOW}make example EXAMPLE=fibonacci${NC}  - Run example program"
echo ""
echo -e "${BLUE}Bootstrap Testing:${NC}"
echo -e "  ${YELLOW}make bootstrap-test-help${NC} - Show bootstrap testing help"
echo ""

# Check if configuration needs attention
echo -e "${PURPLE}🔧 Environment Status:${NC}"

# Check git configuration
if git config user.name >/dev/null && git config user.email >/dev/null; then
    echo -e "${GREEN}✓ Git user configured${NC}"
else
    echo -e "${YELLOW}⚠ Git user not configured${NC}"
    echo -e "${YELLOW}  Run: git config user.name 'Your Name'${NC}"
    echo -e "${YELLOW}  Run: git config user.email 'your@email.com'${NC}"
fi

# Check for recommended tools
if command_exists code; then
    echo -e "${GREEN}✓ VS Code detected${NC}"
else
    echo -e "${BLUE}ℹ Consider installing VS Code for better development experience${NC}"
fi

if command_exists rust-analyzer; then
    echo -e "${GREEN}✓ rust-analyzer available${NC}"
else
    echo -e "${BLUE}ℹ Consider installing rust-analyzer for better IDE support${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Development environment setup complete!${NC}"
echo ""
echo -e "${PURPLE}Next steps:${NC}"
echo -e "1. ${YELLOW}Read the documentation in docs/${NC}"
echo -e "2. ${YELLOW}Try running an example: make example EXAMPLE=fibonacci${NC}"
echo -e "3. ${YELLOW}Run the test suite: make test${NC}"
echo -e "4. ${YELLOW}Start coding and have fun! 🚀${NC}"
echo ""
