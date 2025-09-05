#!/bin/bash
# CURSED code formatting utility script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
CHECK_MODE=false
QUIET_MODE=false
CONFIG_FILE=""
INCLUDE_PATTERN="**/*.💀"
EXCLUDE_PATTERN=""
TARGET_DIR="."

# Help function
show_help() {
    echo "CURSED Code Formatter Script"
    echo "Usage: $0 [options] [directory]"
    echo ""
    echo "Options:"
    echo "  -c, --check       Check formatting without making changes"
    echo "  -q, --quiet       Suppress output except errors"
    echo "  -f, --config FILE Use specific config file"
    echo "  -i, --include PAT Include files matching pattern (default: **/*.💀)"
    echo "  -e, --exclude PAT Exclude files matching pattern"
    echo "  -h, --help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                          # Format all .💀 files in current directory"
    echo "  $0 --check src/             # Check formatting in src/ directory"
    echo "  $0 --config=team.toml ./    # Use custom config file"
    echo "  $0 --exclude='**/test_*'    # Exclude test files"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -c|--check)
            CHECK_MODE=true
            shift
            ;;
        -q|--quiet)
            QUIET_MODE=true
            shift
            ;;
        -f|--config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        -i|--include)
            INCLUDE_PATTERN="$2"
            shift 2
            ;;
        -e|--exclude)
            EXCLUDE_PATTERN="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        -*)
            echo "Unknown option $1"
            show_help
            exit 1
            ;;
        *)
            TARGET_DIR="$1"
            shift
            ;;
    esac
done

# Check if cursed-fmt is available
if ! command -v cursed-fmt &> /dev/null; then
    echo -e "${RED}Error: cursed-fmt not found in PATH${NC}"
    echo "Please install the CURSED formatter first:"
    echo "  cargo install --path . --bin cursed-fmt"
    exit 1
fi

# Build the command
CMD="cursed-fmt"

if [ "$CHECK_MODE" = true ]; then
    CMD="$CMD --check"
else
    CMD="$CMD --write"
fi

if [ "$QUIET_MODE" = true ]; then
    CMD="$CMD --quiet"
fi

if [ -n "$CONFIG_FILE" ]; then
    if [ ! -f "$CONFIG_FILE" ]; then
        echo -e "${RED}Error: Config file '$CONFIG_FILE' not found${NC}"
        exit 1
    fi
    CMD="$CMD --config=$CONFIG_FILE"
fi

if [ -n "$INCLUDE_PATTERN" ]; then
    CMD="$CMD --include='$INCLUDE_PATTERN'"
fi

if [ -n "$EXCLUDE_PATTERN" ]; then
    CMD="$CMD --exclude='$EXCLUDE_PATTERN'"
fi

# Add target directory
CMD="$CMD $TARGET_DIR"

# Show what we're doing (unless quiet)
if [ "$QUIET_MODE" != true ]; then
    if [ "$CHECK_MODE" = true ]; then
        echo -e "${BLUE}Checking CURSED code formatting in: $TARGET_DIR${NC}"
    else
        echo -e "${BLUE}Formatting CURSED code in: $TARGET_DIR${NC}"
    fi
    
    if [ -n "$CONFIG_FILE" ]; then
        echo -e "${YELLOW}Using config: $CONFIG_FILE${NC}"
    fi
fi

# Execute the command
if eval "$CMD"; then
    if [ "$QUIET_MODE" != true ]; then
        if [ "$CHECK_MODE" = true ]; then
            echo -e "${GREEN}✓ All files are properly formatted!${NC}"
        else
            echo -e "${GREEN}✓ Formatting complete!${NC}"
        fi
    fi
    exit 0
else
    EXIT_CODE=$?
    if [ "$QUIET_MODE" != true ]; then
        if [ "$CHECK_MODE" = true ]; then
            echo -e "${RED}✗ Some files need formatting${NC}"
            echo "Run without --check to fix formatting issues"
        else
            echo -e "${RED}✗ Formatting failed${NC}"
        fi
    fi
    exit $EXIT_CODE
fi
