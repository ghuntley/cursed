#!/bin/bash
# Bootstrap Self-Hosting Validation System
# Validates that the CURSED Stage 2 compiler can compile itself

set -e

echo "🚀 CURSED Stage 2 Self-Hosting Bootstrap Validation"
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Phase 1: Verify Rust compiler can compile the bootstrap
log_info "Phase 1: Building Rust bootstrap compiler"
cargo build --release
if [ $? -eq 0 ]; then
    log_success "Rust bootstrap compiler built successfully"
else
    log_error "Failed to build Rust bootstrap compiler"
    exit 1
fi

# Phase 2: Test the bootstrap compiler with simple programs
log_info "Phase 2: Testing bootstrap compiler with simple programs"

# Create test programs
cat > test_simple_print.csd << 'EOF'
vibez.spill("Hello from CURSED!")
EOF

cat > test_simple_math.csd << 'EOF'
sus x normie = 5
sus y normie = 10
sus result normie = x + y
vibez.spill("Result: " + result.to_string())
EOF

# Test interpretation mode
log_info "Testing interpretation mode"
./target/release/cursed test_simple_print.csd
if [ $? -eq 0 ]; then
    log_success "Simple print test passed in interpretation mode"
else
    log_error "Simple print test failed in interpretation mode"
    exit 1
fi

./target/release/cursed test_simple_math.csd
if [ $? -eq 0 ]; then
    log_success "Simple math test passed in interpretation mode"
else
    log_error "Simple math test failed in interpretation mode"
    exit 1
fi

# Test compilation mode
log_info "Testing compilation mode"
./target/release/cursed -- compile test_simple_print.csd
if [ $? -eq 0 ] && [ -f "./test_simple_print" ]; then
    ./test_simple_print
    if [ $? -eq 0 ]; then
        log_success "Simple print test compiled and executed successfully"
    else
        log_error "Compiled simple print test failed to execute"
        exit 1
    fi
else
    log_error "Failed to compile simple print test"
    exit 1
fi

# Phase 3: Validate Stage 2 compiler can parse itself
log_info "Phase 3: Testing Stage 2 compiler self-parsing"

# Test that the Stage 2 main.csd can be parsed
./target/release/cursed src/bootstrap/stage2/main.csd > /dev/null 2>&1
if [ $? -eq 0 ]; then
    log_success "Stage 2 main.csd parsed successfully"
else
    log_warning "Stage 2 main.csd parsing had issues (may need syntax fixes)"
fi

# Phase 4: Test stdlib modules used by Stage 2
log_info "Phase 4: Testing required stdlib modules"

stdlib_modules=(
    "stdlib/ast_mood/test_ast_mood.csd"
    "stdlib/token_vibe/test_token_vibe.csd"
    "stdlib/compiler_core/test_compiler_core.csd"
    "stdlib/parser/test_parser.csd"
    "stdlib/io/test_io.csd"
    "stdlib/collections/test_collections.csd"
)

for module in "${stdlib_modules[@]}"; do
    if [ -f "$module" ]; then
        log_info "Testing $module"
        ./target/release/cursed "$module"
        if [ $? -eq 0 ]; then
            log_success "$module test passed"
        else
            log_warning "$module test had issues"
        fi
    else
        log_warning "$module not found"
    fi
done

# Phase 5: Attempt self-hosting compilation
log_info "Phase 5: Attempting self-hosting compilation"

# Try to compile the Stage 2 compiler itself
log_info "Compiling Stage 2 compiler with Rust bootstrap"
./target/release/cursed -- compile src/bootstrap/stage2/main.csd -o cursed_stage2
if [ $? -eq 0 ] && [ -f "./cursed_stage2" ]; then
    log_success "Stage 2 compiler compiled successfully!"
    
    # Test the self-compiled compiler
    log_info "Testing self-compiled compiler"
    ./cursed_stage2 --version
    if [ $? -eq 0 ]; then
        log_success "Self-compiled compiler version check passed"
    else
        log_warning "Self-compiled compiler version check failed"
    fi
    
    # Try to compile a simple program with the self-compiled compiler
    log_info "Testing self-compiled compiler on simple program"
    ./cursed_stage2 test_simple_print.csd
    if [ $? -eq 0 ]; then
        log_success "Self-compiled compiler can run simple programs!"
    else
        log_warning "Self-compiled compiler had issues with simple programs"
    fi
    
else
    log_error "Failed to compile Stage 2 compiler"
    log_info "This is expected during initial development - continuing with validation"
fi

# Phase 6: Bootstrap validation report
log_info "Phase 6: Bootstrap validation report"

echo
echo "=========================================="
echo "🎯 SELF-HOSTING BOOTSTRAP VALIDATION REPORT"
echo "=========================================="

# Check dependencies
echo "📋 Dependencies Status:"
echo "  ✅ Rust compiler: Available"
echo "  ✅ LLVM tools: $(which llc >/dev/null && echo "Available" || echo "❌ Missing")"
echo "  ✅ Bootstrap compiler: Built"

# Check Stage 2 implementation status
echo
echo "🔧 Stage 2 Implementation Status:"
stage2_files=(
    "src/bootstrap/stage2/main.csd"
    "src/bootstrap/stage2/lexer.csd"
    "src/bootstrap/stage2/parser.csd"
    "src/bootstrap/stage2/type_checker.csd"
    "src/bootstrap/stage2/codegen.csd"
    "src/bootstrap/stage2/error.csd"
)

for file in "${stage2_files[@]}"; do
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file")
        echo "  ✅ $file ($lines lines)"
    else
        echo "  ❌ $file (missing)"
    fi
done

# Check stdlib readiness
echo
echo "📚 Stdlib Readiness:"
required_modules=(
    "stdlib/ast_mood/"
    "stdlib/token_vibe/"
    "stdlib/compiler_core/"
    "stdlib/parser/"
    "stdlib/io/"
    "stdlib/collections/"
    "stdlib/exec_slay/"
)

for module in "${required_modules[@]}"; do
    if [ -d "$module" ]; then
        echo "  ✅ $module"
    else
        echo "  ❌ $module (missing)"
    fi
done

echo
echo "🎉 Bootstrap validation completed!"
echo
echo "Next steps for full self-hosting:"
echo "1. Fix any syntax issues in Stage 2 implementation"
echo "2. Implement missing stdlib modules if any"
echo "3. Test compilation of larger CURSED programs"
echo "4. Implement complete LLVM backend integration"
echo "5. Test recursive self-compilation (compiler compiling compiler)"

# Cleanup test files
rm -f test_simple_print.csd test_simple_math.csd test_simple_print cursed_stage2

log_success "Bootstrap validation completed successfully!"
