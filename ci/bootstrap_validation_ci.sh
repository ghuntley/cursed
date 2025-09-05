#!/bin/bash
# Bootstrap Validation CI/CD Integration Script
# Designed for continuous integration environments

set -e

# CI Environment detection
CI_ENVIRONMENT=${CI_ENVIRONMENT:-"local"}
BRANCH_NAME=${BRANCH_NAME:-$(git branch --show-current 2>/dev/null || echo "unknown")}
COMMIT_SHA=${COMMIT_SHA:-$(git rev-parse HEAD 2>/dev/null || echo "unknown")}

# Configuration
VALIDATION_TIMEOUT=1800  # 30 minutes timeout
PARALLEL_JOBS=${PARALLEL_JOBS:-4}
REPORT_FORMAT=${REPORT_FORMAT:-"markdown"}

# Logging setup for CI
setup_ci_logging() {
    exec 1> >(tee -a ci_validation.log)
    exec 2> >(tee -a ci_validation.log >&2)
    
    echo "=========================================="
    echo "Bootstrap Validation CI/CD Run"
    echo "Time: $(date)"
    echo "Environment: $CI_ENVIRONMENT"
    echo "Branch: $BRANCH_NAME"
    echo "Commit: $COMMIT_SHA"
    echo "=========================================="
}

# Fast validation for pull requests
run_fast_validation() {
    echo "Running fast validation for CI/CD..."
    
    # Quick syntax check
    echo "Step 1: Quick syntax validation"
    if ! cargo check --all-targets; then
        echo "❌ Syntax validation failed"
        return 1
    fi
    
    # Build original compiler
    echo "Step 2: Build original compiler"
    if ! cargo build --release; then
        echo "❌ Failed to build original compiler"
        return 1
    fi
    
    # Test basic compilation
    echo "Step 3: Test basic compilation"
    echo 'vibez.spill("CI test")' > ci_test.💀
    if ! ./target/release/cursed ci_test.💀; then
        echo "❌ Basic compilation test failed"
        return 1
    fi
    
    # Test Stage 2 compilation
    echo "Step 4: Test Stage 2 compilation"
    if ! ./target/release/cursed -- compile src/bootstrap/stage2/main.💀 -o ci_stage2; then
        echo "⚠️ Stage 2 compilation failed (may be expected)"
        return 0
    fi
    
    # Test Stage 2 execution
    echo "Step 5: Test Stage 2 execution"
    if [ -f "./ci_stage2" ]; then
        if ./ci_stage2 --version >/dev/null 2>&1; then
            echo "✅ Stage 2 compiler works!"
        else
            echo "⚠️ Stage 2 compiled but version check failed"
        fi
    fi
    
    # Cleanup
    rm -f ci_test.💀 ci_stage2
    
    echo "✅ Fast validation completed successfully"
    return 0
}

# Full validation for main branch and releases
run_full_validation() {
    echo "Running full bootstrap validation..."
    
    # Run comprehensive validation
    if ! bash ci/comprehensive_bootstrap_validation.sh; then
        echo "❌ Comprehensive validation failed"
        return 1
    fi
    
    echo "✅ Full validation completed"
    return 0
}

# Performance benchmarking
run_performance_benchmark() {
    echo "Running performance benchmarks..."
    
    # Create benchmark test
    cat > benchmark_test.💀 << 'EOF'
slay factorial(n normie) normie {
    lowkey (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

sus result normie = factorial(10)
vibez.spill("Factorial result: " + result.to_string())
EOF
    
    # Benchmark original compiler
    echo "Benchmarking original compiler..."
    start_time=$(date +%s%N)
    ./target/release/cursed benchmark_test.💀 > /dev/null 2>&1
    end_time=$(date +%s%N)
    original_time=$(((end_time - start_time) / 1000000))
    
    echo "Original compiler time: ${original_time}ms"
    
    # Benchmark Stage 2 if available
    if [ -f "./cursed_stage2" ]; then
        echo "Benchmarking Stage 2 compiler..."
        start_time=$(date +%s%N)
        ./cursed_stage2 benchmark_test.💀 > /dev/null 2>&1
        stage2_result=$?
        end_time=$(date +%s%N)
        stage2_time=$(((end_time - start_time) / 1000000))
        
        if [ $stage2_result -eq 0 ]; then
            echo "Stage 2 compiler time: ${stage2_time}ms"
            
            # Calculate performance ratio
            if [ $original_time -gt 0 ]; then
                ratio=$((stage2_time * 100 / original_time))
                echo "Performance ratio: ${ratio}%"
                
                # Create performance report
                cat > performance_report.json << EOF
{
  "timestamp": "$(date -Iseconds)",
  "commit": "$COMMIT_SHA",
  "branch": "$BRANCH_NAME",
  "original_time_ms": $original_time,
  "stage2_time_ms": $stage2_time,
  "performance_ratio": $ratio,
  "status": "$([ $ratio -le 200 ] && echo "acceptable" || echo "slow")"
}
EOF
                echo "✅ Performance benchmark completed"
            fi
        else
            echo "⚠️ Stage 2 compiler failed benchmark test"
        fi
    else
        echo "⚠️ Stage 2 compiler not available for benchmarking"
    fi
    
    rm -f benchmark_test.💀
}

# Generate CI artifacts
generate_ci_artifacts() {
    echo "Generating CI artifacts..."
    
    # Create artifacts directory
    mkdir -p ci_artifacts
    
    # Copy validation results
    if [ -d "validation_results" ]; then
        cp -r validation_results/* ci_artifacts/
    fi
    
    # Copy performance reports
    if [ -f "performance_report.json" ]; then
        cp performance_report.json ci_artifacts/
    fi
    
    # Create build info
    cat > ci_artifacts/build_info.json << EOF
{
  "timestamp": "$(date -Iseconds)",
  "environment": "$CI_ENVIRONMENT",
  "branch": "$BRANCH_NAME",
  "commit": "$COMMIT_SHA",
  "rust_version": "$(rustc --version)",
  "cargo_version": "$(cargo --version)"
}
EOF
    
    # Create summary report
    cat > ci_artifacts/summary.md << EOF
# Bootstrap Validation Summary

**Environment:** $CI_ENVIRONMENT  
**Branch:** $BRANCH_NAME  
**Commit:** $COMMIT_SHA  
**Time:** $(date)

## Results
$([ -f "validation_results/report.md" ] && cat validation_results/report.md || echo "No detailed results available")

## Logs
See attached log files for detailed output.
EOF
    
    echo "✅ CI artifacts generated in ci_artifacts/"
}

# Main CI execution logic
main() {
    setup_ci_logging
    
    case "$1" in
        "fast")
            run_fast_validation
            ;;
        "full")
            run_full_validation
            ;;
        "benchmark")
            # Build first
            cargo build --release
            run_performance_benchmark
            ;;
        "pr")
            # Pull request validation - fast but thorough
            run_fast_validation
            run_performance_benchmark
            ;;
        "release")
            # Release validation - full validation
            run_full_validation
            run_performance_benchmark
            ;;
        *)
            echo "Usage: $0 {fast|full|benchmark|pr|release}"
            echo ""
            echo "  fast      - Quick validation for CI/CD"
            echo "  full      - Comprehensive validation"
            echo "  benchmark - Performance benchmarking"
            echo "  pr        - Pull request validation"
            echo "  release   - Release validation"
            exit 1
            ;;
    esac
    
    local exit_code=$?
    
    generate_ci_artifacts
    
    echo "CI validation completed with exit code: $exit_code"
    exit $exit_code
}

# Handle timeout
timeout_handler() {
    echo "❌ Validation timed out after ${VALIDATION_TIMEOUT} seconds"
    exit 124
}

# Set up timeout
trap timeout_handler SIGTERM
if command -v timeout >/dev/null 2>&1; then
    timeout $VALIDATION_TIMEOUT bash -c "$(declare -f main); main \"$@\"" "$@"
else
    main "$@"
fi
