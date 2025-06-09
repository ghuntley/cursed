#!/bin/bash

# CURSED Bootstrap Verification Script
#
# This script runs the comprehensive self-compilation verification system
# for the CURSED programming language.

set -e  # Exit on any error

echo "🚀 CURSED Bootstrap Verification System"
echo "======================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the root of the CURSED project"
    exit 1
fi

# Build the verification tool first
echo "🔧 Building bootstrap verification tool..."
cargo build --bin bootstrap-verify --release

if [ $? -ne 0 ]; then
    echo "❌ Failed to build bootstrap verification tool"
    exit 1
fi

echo "✅ Bootstrap verification tool built successfully"

# Check command line arguments
VERIFICATION_ARGS=""

if [ "$1" = "--quick" ]; then
    echo "⚡ Running quick verification (2 cycles, basic tests)"
    VERIFICATION_ARGS="--quick"
elif [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --quick      Run quick verification (faster, fewer cycles)"
    echo "  --verbose    Enable verbose output"
    echo "  --keep       Keep intermediate files for debugging"
    echo "  --help       Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                # Run full verification"
    echo "  $0 --quick       # Run quick verification"
    echo "  $0 --verbose     # Run with verbose output"
    exit 0
elif [ "$1" = "--verbose" ]; then
    echo "📢 Enabling verbose output"
    VERIFICATION_ARGS="--verbose"
elif [ "$1" = "--keep" ]; then
    echo "🗂️  Keeping intermediate files for debugging"
    VERIFICATION_ARGS="--keep-intermediates"
fi

# Ensure we have a clean build first
echo "🧹 Ensuring clean build of Stage 1 (Rust) compiler..."
cargo build --release --bin cursed

if [ $? -ne 0 ]; then
    echo "❌ Failed to build Stage 1 compiler"
    exit 1
fi

echo "✅ Stage 1 compiler built successfully"

# Run the verification
echo ""
echo "🔍 Starting bootstrap verification process..."
echo "This may take several minutes depending on your system..."
echo ""

# Create output directory
mkdir -p reports

# Run the verification tool
./target/release/bootstrap-verify \
    --work-dir ./bootstrap_verification \
    --output ./reports/bootstrap_verification_report.md \
    --timeout 300 \
    --cycles 3 \
    $VERIFICATION_ARGS

VERIFICATION_EXIT_CODE=$?

echo ""
echo "📊 Verification Summary"
echo "======================"

if [ $VERIFICATION_EXIT_CODE -eq 0 ]; then
    echo "✅ Bootstrap verification PASSED"
    echo "📄 Full report available at: ./reports/bootstrap_verification_report.md"
    
    # Show basic stats if report exists
    if [ -f "./reports/bootstrap_verification_report.md" ]; then
        echo ""
        echo "Quick Stats:"
        grep -E "(Overall Success|Verification Time|Issues Found)" ./reports/bootstrap_verification_report.md || true
    fi
else
    echo "❌ Bootstrap verification FAILED"
    echo "📄 Error report available at: ./reports/bootstrap_verification_report.md"
    
    # Show any critical issues
    if [ -f "./reports/bootstrap_verification_report.md" ]; then
        echo ""
        echo "Critical Issues:"
        grep -A 5 "Issues Found" ./reports/bootstrap_verification_report.md || true
    fi
fi

echo ""
echo "🧹 Cleanup"
echo "=========="

if [ "$VERIFICATION_ARGS" != "--keep-intermediates" ]; then
    echo "🗑️  Cleaning up intermediate files..."
    rm -rf ./bootstrap_verification
    echo "✅ Cleanup completed"
else
    echo "🗂️  Keeping intermediate files in ./bootstrap_verification/"
fi

echo ""
echo "📋 Next Steps"
echo "============="

if [ $VERIFICATION_EXIT_CODE -eq 0 ]; then
    echo "✅ The bootstrap verification passed! The CURSED compiler can successfully compile itself."
    echo "📈 Consider running additional tests:"
    echo "   - Performance benchmarks: cargo run --bin language_benchmark"
    echo "   - Comprehensive test suite: cargo test"
    echo "   - Documentation generation: cargo doc"
else
    echo "⚠️  The bootstrap verification found issues that need to be addressed:"
    echo "   1. Review the full report at ./reports/bootstrap_verification_report.md"
    echo "   2. Fix any compilation errors in the CURSED-to-CURSED compiler"
    echo "   3. Ensure Stage 1 and Stage 2 compilers produce equivalent output"
    echo "   4. Re-run the verification after fixes"
    echo ""
    echo "🐛 Debug tips:"
    echo "   - Run with --keep flag to preserve intermediate files"
    echo "   - Run with --verbose flag for detailed output"
    echo "   - Check individual test failures in the work directory"
fi

exit $VERIFICATION_EXIT_CODE
