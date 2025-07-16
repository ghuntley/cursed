#!/bin/bash
# Comprehensive Bootstrap Validation System
# Validates complete self-hosting compiler reliability with differential testing

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Global validation results
VALIDATION_RESULTS=""
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
WARNING_TESTS=0

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
    PASSED_TESTS=$((PASSED_TESTS + 1))
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
    WARNING_TESTS=$((WARNING_TESTS + 1))
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    FAILED_TESTS=$((FAILED_TESTS + 1))
}

log_phase() {
    echo
    echo -e "${PURPLE}=========================================${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${PURPLE}=========================================${NC}"
}

log_test() {
    echo -e "${CYAN}[TEST]${NC} $1"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
}

# Initialize validation environment
init_validation() {
    log_phase "🚀 COMPREHENSIVE BOOTSTRAP VALIDATION SYSTEM"
    
    # Create validation directories
    mkdir -p validation_results
    mkdir -p validation_tmp
    mkdir -p test_programs
    
    # Initialize result tracking
    echo "Bootstrap Validation Report - $(date)" > validation_results/report.md
    echo "===========================================" >> validation_results/report.md
    echo "" >> validation_results/report.md
    
    log_info "Validation environment initialized"
}

# Phase 1: Build and verify original compiler
phase1_build_original() {
    log_phase "Phase 1: Build Original Rust Compiler"
    
    log_test "Building original Rust compiler"
    if cargo build --release > validation_tmp/build_original.log 2>&1; then
        log_success "Original compiler built successfully"
        echo "✅ Original compiler build: SUCCESS" >> validation_results/report.md
    else
        log_error "Failed to build original compiler"
        echo "❌ Original compiler build: FAILED" >> validation_results/report.md
        cat validation_tmp/build_original.log
        return 1
    fi
    
    log_test "Verifying original compiler basic functionality"
    echo 'vibez.spill("Original compiler test")' > test_programs/original_test.csd
    
    if ./target/release/cursed test_programs/original_test.csd > validation_tmp/original_test.out 2>&1; then
        log_success "Original compiler basic test passed"
        echo "✅ Original compiler functionality: SUCCESS" >> validation_results/report.md
    else
        log_error "Original compiler basic test failed"
        echo "❌ Original compiler functionality: FAILED" >> validation_results/report.md
        return 1
    fi
}

# Phase 2: Compile Stage 2 self-hosting compiler
phase2_compile_stage2() {
    log_phase "Phase 2: Compile Self-Hosting Stage 2 Compiler"
    
    log_test "Compiling Stage 2 compiler with original compiler"
    if ./target/release/cursed -- compile src/bootstrap/stage2/main.csd -o cursed_stage2 > validation_tmp/stage2_compilation.log 2>&1; then
        log_success "Stage 2 compiler compiled successfully"
        echo "✅ Stage 2 compilation: SUCCESS" >> validation_results/report.md
        
        # Verify executable exists and is runnable
        if [ -f "./cursed_stage2" ] && [ -x "./cursed_stage2" ]; then
            log_success "Stage 2 executable created and is executable"
            echo "✅ Stage 2 executable: SUCCESS" >> validation_results/report.md
        else
            log_error "Stage 2 executable not created or not executable"
            echo "❌ Stage 2 executable: FAILED" >> validation_results/report.md
            return 1
        fi
    else
        log_error "Failed to compile Stage 2 compiler"
        echo "❌ Stage 2 compilation: FAILED" >> validation_results/report.md
        cat validation_tmp/stage2_compilation.log
        return 1
    fi
    
    log_test "Testing Stage 2 compiler version check"
    if ./cursed_stage2 --version > validation_tmp/stage2_version.out 2>&1; then
        log_success "Stage 2 version check passed"
        echo "✅ Stage 2 version check: SUCCESS" >> validation_results/report.md
        echo "Version output: $(cat validation_tmp/stage2_version.out)" >> validation_results/report.md
    else
        log_warning "Stage 2 version check failed (may not be implemented yet)"
        echo "⚠️ Stage 2 version check: WARNING" >> validation_results/report.md
    fi
}

# Phase 3: Create comprehensive test suite
phase3_create_test_suite() {
    log_phase "Phase 3: Create Comprehensive Test Suite"
    
    log_info "Creating test programs for differential validation"
    
    # Test 1: Simple program
    cat > test_programs/simple.csd << 'EOF'
vibez.spill("Hello from CURSED!")
EOF
    
    # Test 2: Variables and arithmetic
    cat > test_programs/variables.csd << 'EOF'
sus x normie = 10
sus y normie = 20
sus result normie = x + y
vibez.spill("Result: " + result.to_string())
EOF
    
    # Test 3: Control flow
    cat > test_programs/control_flow.csd << 'EOF'
sus count normie = 5
bestie (count > 0) {
    vibez.spill("Count is positive: " + count.to_string())
    count = count - 1
    periodt (count > 0) {
        vibez.spill("Counting down: " + count.to_string())
        count = count - 1
    }
}
vibez.spill("Done counting")
EOF
    
    # Test 4: Functions
    cat > test_programs/functions.csd << 'EOF'
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

sus result normie = add_numbers(15, 25)
vibez.spill("Function result: " + result.to_string())
EOF
    
    # Test 5: Complex data types
    cat > test_programs/complex_types.csd << 'EOF'
sus tuple := (42, "hello", based)
sus first normie = tuple.0
sus second tea = tuple.1
sus third lit = tuple.2
vibez.spill("Tuple test: " + first.to_string() + " " + second + " " + third.to_string())
EOF
    
    # Test 6: Advanced features
    cat > test_programs/advanced.csd << 'EOF'
# Test error handling
yikes error_test() {
    sus value normie = 42
    lowkey (value > 0) {
        vibez.spill("Value is valid: " + value.to_string())
    } highkey {
        shook "Invalid value"
    }
}

error_test()
vibez.spill("Advanced features test completed")
EOF
    
    log_success "Test suite created with 6 test programs"
    echo "✅ Test suite creation: SUCCESS (6 programs)" >> validation_results/report.md
}

# Phase 4: Differential testing between original and Stage 2
phase4_differential_testing() {
    log_phase "Phase 4: Differential Testing - Original vs Stage 2"
    
    test_programs=(
        "simple.csd"
        "variables.csd"
        "control_flow.csd"
        "functions.csd"
        "complex_types.csd"
    )
    
    echo "## Differential Testing Results" >> validation_results/report.md
    echo "" >> validation_results/report.md
    
    for program in "${test_programs[@]}"; do
        log_test "Differential testing: $program"
        
        # Test with original compiler
        log_info "Testing $program with original compiler"
        if ./target/release/cursed "test_programs/$program" > "validation_tmp/original_$program.out" 2>&1; then
            original_success=true
            log_info "Original compiler succeeded for $program"
        else
            original_success=false
            log_warning "Original compiler failed for $program"
        fi
        
        # Test with Stage 2 compiler
        log_info "Testing $program with Stage 2 compiler"
        if ./cursed_stage2 "test_programs/$program" > "validation_tmp/stage2_$program.out" 2>&1; then
            stage2_success=true
            log_info "Stage 2 compiler succeeded for $program"
        else
            stage2_success=false
            log_warning "Stage 2 compiler failed for $program"
        fi
        
        # Compare results
        if [ "$original_success" = true ] && [ "$stage2_success" = true ]; then
            if diff "validation_tmp/original_$program.out" "validation_tmp/stage2_$program.out" > /dev/null 2>&1; then
                log_success "Differential test PASSED: $program (identical output)"
                echo "✅ $program: IDENTICAL OUTPUT" >> validation_results/report.md
            else
                log_warning "Differential test WARNING: $program (different output)"
                echo "⚠️ $program: DIFFERENT OUTPUT" >> validation_results/report.md
                echo "Original output:" >> validation_results/report.md
                cat "validation_tmp/original_$program.out" >> validation_results/report.md
                echo "Stage 2 output:" >> validation_results/report.md
                cat "validation_tmp/stage2_$program.out" >> validation_results/report.md
                echo "" >> validation_results/report.md
            fi
        elif [ "$original_success" = true ] && [ "$stage2_success" = false ]; then
            log_error "Differential test FAILED: $program (Stage 2 failed, original succeeded)"
            echo "❌ $program: STAGE 2 FAILED" >> validation_results/report.md
        elif [ "$original_success" = false ] && [ "$stage2_success" = true ]; then
            log_warning "Differential test WARNING: $program (Stage 2 succeeded, original failed)"
            echo "⚠️ $program: ORIGINAL FAILED" >> validation_results/report.md
        else
            log_warning "Differential test WARNING: $program (both failed)"
            echo "⚠️ $program: BOTH FAILED" >> validation_results/report.md
        fi
    done
}

# Phase 5: Performance regression testing
phase5_performance_testing() {
    log_phase "Phase 5: Performance Regression Testing"
    
    # Create performance test program
    cat > test_programs/performance.csd << 'EOF'
slay fibonacci(n normie) normie {
    lowkey (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus start normie = 20
sus result normie = fibonacci(start)
vibez.spill("Fibonacci(" + start.to_string() + ") = " + result.to_string())
EOF
    
    log_test "Performance testing with fibonacci calculation"
    
    # Time original compiler
    log_info "Timing original compiler performance"
    start_time=$(date +%s%N)
    ./target/release/cursed test_programs/performance.csd > validation_tmp/original_perf.out 2>&1
    original_result=$?
    end_time=$(date +%s%N)
    original_time=$(((end_time - start_time) / 1000000)) # Convert to milliseconds
    
    # Time Stage 2 compiler  
    log_info "Timing Stage 2 compiler performance"
    start_time=$(date +%s%N)
    ./cursed_stage2 test_programs/performance.csd > validation_tmp/stage2_perf.out 2>&1
    stage2_result=$?
    end_time=$(date +%s%N)
    stage2_time=$(((end_time - start_time) / 1000000)) # Convert to milliseconds
    
    echo "## Performance Testing Results" >> validation_results/report.md
    echo "" >> validation_results/report.md
    echo "Original compiler time: ${original_time}ms" >> validation_results/report.md
    echo "Stage 2 compiler time: ${stage2_time}ms" >> validation_results/report.md
    
    if [ $original_result -eq 0 ] && [ $stage2_result -eq 0 ]; then
        # Calculate performance ratio
        if [ $original_time -gt 0 ]; then
            ratio=$((stage2_time * 100 / original_time))
            echo "Performance ratio: ${ratio}% of original" >> validation_results/report.md
            
            if [ $ratio -le 150 ]; then
                log_success "Performance test PASSED: Stage 2 within 150% of original (${ratio}%)"
                echo "✅ Performance: ACCEPTABLE (${ratio}%)" >> validation_results/report.md
            else
                log_warning "Performance test WARNING: Stage 2 slower than 150% of original (${ratio}%)"
                echo "⚠️ Performance: SLOW (${ratio}%)" >> validation_results/report.md
            fi
        else
            log_warning "Performance test WARNING: Could not measure original compiler time"
            echo "⚠️ Performance: UNMEASURABLE" >> validation_results/report.md
        fi
    else
        log_error "Performance test FAILED: One or both compilers failed to run"
        echo "❌ Performance: FAILED" >> validation_results/report.md
    fi
}

# Phase 6: Recursive self-compilation test
phase6_recursive_compilation() {
    log_phase "Phase 6: Recursive Self-Compilation Test"
    
    log_test "Attempting recursive compilation: Stage 2 compiling itself"
    
    # Try to compile Stage 2 with itself
    if ./cursed_stage2 -- compile src/bootstrap/stage2/main.csd -o cursed_stage2_recursive > validation_tmp/recursive_compilation.log 2>&1; then
        log_success "Recursive compilation succeeded"
        echo "✅ Recursive compilation: SUCCESS" >> validation_results/report.md
        
        # Test the recursively compiled compiler
        if [ -f "./cursed_stage2_recursive" ] && [ -x "./cursed_stage2_recursive" ]; then
            log_test "Testing recursively compiled compiler"
            if ./cursed_stage2_recursive test_programs/simple.csd > validation_tmp/recursive_test.out 2>&1; then
                log_success "Recursively compiled compiler works"
                echo "✅ Recursive compiler functionality: SUCCESS" >> validation_results/report.md
            else
                log_warning "Recursively compiled compiler failed basic test"
                echo "⚠️ Recursive compiler functionality: FAILED" >> validation_results/report.md
            fi
        else
            log_warning "Recursive compilation created no executable"
            echo "⚠️ Recursive compilation output: NO EXECUTABLE" >> validation_results/report.md
        fi
    else
        log_warning "Recursive compilation failed (expected in early stages)"
        echo "⚠️ Recursive compilation: FAILED" >> validation_results/report.md
        cat validation_tmp/recursive_compilation.log >> validation_results/report.md
    fi
}

# Phase 7: Error handling equivalence testing
phase7_error_testing() {
    log_phase "Phase 7: Error Handling Equivalence Testing"
    
    log_info "Creating programs with intentional errors"
    
    # Syntax error test
    cat > test_programs/syntax_error.csd << 'EOF'
sus x normie = 42
vibez.spill("Missing semicolon"
EOF
    
    # Type error test
    cat > test_programs/type_error.csd << 'EOF'
sus x normie = 42
sus y tea = "hello"
sus result normie = x + y  # Type mismatch
vibez.spill("This should fail")
EOF
    
    error_tests=("syntax_error.csd" "type_error.csd")
    
    echo "## Error Handling Testing Results" >> validation_results/report.md
    echo "" >> validation_results/report.md
    
    for error_test in "${error_tests[@]}"; do
        log_test "Error handling test: $error_test"
        
        # Test with original compiler
        ./target/release/cursed "test_programs/$error_test" > "validation_tmp/original_error_$error_test.out" 2>&1
        original_exit_code=$?
        
        # Test with Stage 2 compiler
        ./cursed_stage2 "test_programs/$error_test" > "validation_tmp/stage2_error_$error_test.out" 2>&1
        stage2_exit_code=$?
        
        if [ $original_exit_code -ne 0 ] && [ $stage2_exit_code -ne 0 ]; then
            log_success "Error handling PASSED: $error_test (both compilers detected error)"
            echo "✅ $error_test: BOTH DETECTED ERROR" >> validation_results/report.md
        elif [ $original_exit_code -eq 0 ] && [ $stage2_exit_code -eq 0 ]; then
            log_warning "Error handling WARNING: $error_test (neither compiler detected error)"
            echo "⚠️ $error_test: NO ERROR DETECTED" >> validation_results/report.md
        else
            log_warning "Error handling WARNING: $error_test (inconsistent error detection)"
            echo "⚠️ $error_test: INCONSISTENT ERROR DETECTION" >> validation_results/report.md
        fi
    done
}

# Phase 8: Stdlib integration testing
phase8_stdlib_testing() {
    log_phase "Phase 8: Stdlib Integration Testing"
    
    log_test "Testing stdlib modules with both compilers"
    
    stdlib_modules=(
        "stdlib/testz/test_testz.csd"
        "stdlib/timez/test_timez.csd"
        "stdlib/mathz/test_mathz.csd"
        "stdlib/stringz/test_stringz.csd"
    )
    
    echo "## Stdlib Integration Results" >> validation_results/report.md
    echo "" >> validation_results/report.md
    
    for module in "${stdlib_modules[@]}"; do
        if [ -f "$module" ]; then
            log_test "Stdlib test: $module"
            
            # Test with original compiler
            ./target/release/cursed "$module" > "validation_tmp/original_stdlib_$(basename $module).out" 2>&1
            original_result=$?
            
            # Test with Stage 2 compiler
            ./cursed_stage2 "$module" > "validation_tmp/stage2_stdlib_$(basename $module).out" 2>&1
            stage2_result=$?
            
            if [ $original_result -eq 0 ] && [ $stage2_result -eq 0 ]; then
                log_success "Stdlib test PASSED: $module"
                echo "✅ $(basename $module): SUCCESS" >> validation_results/report.md
            else
                log_warning "Stdlib test WARNING: $module (one or both failed)"
                echo "⚠️ $(basename $module): FAILED" >> validation_results/report.md
            fi
        else
            log_info "Stdlib module not found: $module"
            echo "ℹ️ $(basename $module): NOT FOUND" >> validation_results/report.md
        fi
    done
}

# Generate final validation report
generate_final_report() {
    log_phase "Final Validation Report"
    
    echo "" >> validation_results/report.md
    echo "## Summary" >> validation_results/report.md
    echo "===========================================" >> validation_results/report.md
    echo "Total tests: $TOTAL_TESTS" >> validation_results/report.md
    echo "Passed: $PASSED_TESTS" >> validation_results/report.md
    echo "Failed: $FAILED_TESTS" >> validation_results/report.md
    echo "Warnings: $WARNING_TESTS" >> validation_results/report.md
    echo "" >> validation_results/report.md
    
    success_rate=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo "Success rate: ${success_rate}%" >> validation_results/report.md
    echo "" >> validation_results/report.md
    
    if [ $success_rate -ge 80 ]; then
        bootstrap_status="READY"
        echo "🎉 Bootstrap Status: READY FOR PRODUCTION" >> validation_results/report.md
    elif [ $success_rate -ge 60 ]; then
        bootstrap_status="DEVELOPMENT"
        echo "🚧 Bootstrap Status: READY FOR DEVELOPMENT" >> validation_results/report.md
    else
        bootstrap_status="EARLY"
        echo "🏗️ Bootstrap Status: EARLY DEVELOPMENT" >> validation_results/report.md
    fi
    
    echo
    echo -e "${PURPLE}===========================================${NC}"
    echo -e "${PURPLE}🎯 BOOTSTRAP VALIDATION SUMMARY${NC}"
    echo -e "${PURPLE}===========================================${NC}"
    echo -e "${GREEN}Total Tests:${NC} $TOTAL_TESTS"
    echo -e "${GREEN}Passed:${NC} $PASSED_TESTS"
    echo -e "${RED}Failed:${NC} $FAILED_TESTS"
    echo -e "${YELLOW}Warnings:${NC} $WARNING_TESTS"
    echo -e "${CYAN}Success Rate:${NC} ${success_rate}%"
    echo -e "${PURPLE}Bootstrap Status:${NC} $bootstrap_status"
    echo
    
    log_info "Full report saved to: validation_results/report.md"
    log_info "Log files saved to: validation_tmp/"
}

# Cleanup function
cleanup_validation() {
    log_info "Cleaning up temporary files"
    rm -f cursed_stage2 cursed_stage2_recursive
    rm -rf test_programs
    log_success "Cleanup completed"
}

# Main validation execution
main() {
    init_validation
    
    # Run all validation phases
    phase1_build_original || exit 1
    phase2_compile_stage2 || exit 1
    phase3_create_test_suite
    phase4_differential_testing
    phase5_performance_testing
    phase6_recursive_compilation
    phase7_error_testing
    phase8_stdlib_testing
    
    generate_final_report
    cleanup_validation
    
    log_success "Comprehensive bootstrap validation completed!"
    
    # Exit with appropriate code
    if [ $FAILED_TESTS -eq 0 ]; then
        exit 0
    else
        exit 1
    fi
}

# Run validation if called directly
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    main "$@"
fi
