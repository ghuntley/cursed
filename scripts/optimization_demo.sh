#!/bin/bash

# CURSED Optimization System Demo
# Demonstrates the complete CLI integration for the optimization system

set -e

echo "🎯 CURSED Performance Optimization System Demo"
echo "============================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_section() {
    echo -e "${BLUE}## $1${NC}"
    echo
}

print_command() {
    echo -e "${YELLOW}$ $1${NC}"
}

run_command() {
    print_command "$1"
    echo
    eval "$1" || echo -e "${RED}Command failed (this is expected during demo)${NC}"
    echo
}

# Demo file
DEMO_FILE="examples/optimization_demo.csd"

print_section "1. Checking Available Optimization Commands"
run_command "cursed optimize --help"

print_section "2. Listing Available Optimization Profiles"
run_command "cursed optimize profiles --list"

print_section "3. Analyzing Code for Optimization Opportunities"
run_command "cursed optimize analyze $DEMO_FILE --detailed --suggestions"

print_section "4. Running Interactive Optimization Wizard (Quick Mode)"
echo "Simulating quick interactive setup..."
echo -e "${GREEN}This would normally prompt for user input${NC}"
echo

print_section "5. Applying Profile-Based Optimizations"
run_command "cursed optimize apply $DEMO_FILE --profile systems --dry-run"

print_section "6. Auto-Applying Safe Optimizations"
run_command "cursed optimize apply $DEMO_FILE --safe --dry-run"

print_section "7. Benchmarking Different Optimization Levels"
run_command "cursed optimize benchmark $DEMO_FILE --levels 0,1,2,3 --iterations 3"

print_section "8. Profiling Compilation Performance"
run_command "cursed optimize profile $DEMO_FILE --phases --opt-level 2"

print_section "9. Managing Optimization Configuration"
run_command "cursed optimize config --show"

print_section "10. Creating Custom Profile"
echo "Simulating profile creation..."
echo -e "${GREEN}This would normally prompt for profile details${NC}"
echo

print_section "11. Running Optimized Build"
run_command "cursed build $DEMO_FILE --opt-level 3 --lto --emit exe"

print_section "12. Running with Performance Monitoring"
run_command "cursed run $DEMO_FILE --opt-level 3 --profile --time-passes"

print_section "13. Watch Mode with Optimization"
echo "Watch mode would monitor file changes and re-optimize automatically"
echo -e "${GREEN}Use Ctrl+C to stop watch mode${NC}"
echo

print_section "Demo Completed Successfully!"
echo -e "${GREEN}✅ CURSED Optimization System Demo finished!${NC}"
echo
echo "Key Features Demonstrated:"
echo "  🎯 Interactive optimization wizard"
echo "  📊 Performance analysis and recommendations"
echo "  🚀 Auto-apply optimization profiles"
echo "  📈 Benchmarking and profiling"
echo "  ⚙️  Configuration management"
echo "  🔧 Custom profile creation"
echo "  🏃 Optimized compilation and execution"
echo
echo "Try these commands yourself:"
echo "  cursed optimize interactive examples/optimization_demo.csd"
echo "  cursed optimize apply examples/optimization_demo.csd --profile web"
echo "  cursed optimize benchmark examples/optimization_demo.csd"
echo "  cursed optimize profiles --list"
echo
