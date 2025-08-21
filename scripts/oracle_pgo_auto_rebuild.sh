#!/bin/bash

# Oracle PGO Auto-Rebuild System
# Automatically rebuilds with PGO optimization when performance improvements are detected

set -e

echo "🔄 Oracle PGO Auto-Rebuild System"
echo "================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROFILE_BLOB="oracle_pgo_profile.blob"
REBUILD_LOG="oracle_pgo_rebuild.log"
PERFORMANCE_THRESHOLD=10.0  # 10% improvement threshold for auto-rebuild
MIN_REBUILD_INTERVAL=3600   # Minimum 1 hour between rebuilds
LAST_REBUILD_FILE=".oracle_pgo_last_rebuild"

# Build configurations
PROFILE_GENERATE_FLAGS="-Doptimize=ReleaseFast -Dpgo-generate=true"
PROFILE_USE_FLAGS="-Doptimize=ReleaseFast -Dpgo-use=true -Dpgo-profile=$PROFILE_BLOB"
PRODUCTION_FLAGS="-Doptimize=ReleaseFast -Dpgo-use=true -Dllvm-opt=O3 -Dlto=true"

# Test programs for performance measurement
TEST_PROGRAMS=(
    "benchmarks/oracle_macro_performance_suite.csd"
    "benchmarks/pgo_benchmark_suite.csd"
    "comprehensive_stdlib_test.csd"
)

# Function to log with timestamp
log_with_timestamp() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" | tee -a "$REBUILD_LOG"
}

# Function to check if rebuild is needed
check_rebuild_needed() {
    if [ -f "$LAST_REBUILD_FILE" ]; then
        local last_rebuild=$(cat "$LAST_REBUILD_FILE" 2>/dev/null || echo "0")
        local current_time=$(date +%s)
        local time_diff=$((current_time - last_rebuild))
        
        if [ $time_diff -lt $MIN_REBUILD_INTERVAL ]; then
            log_with_timestamp "Rebuild skipped: only $time_diff seconds since last rebuild (minimum: $MIN_REBUILD_INTERVAL)"
            return 1
        fi
    fi
    
    return 0
}

# Function to measure baseline performance
measure_baseline_performance() {
    log_with_timestamp "Measuring baseline performance..."
    
    local total_time=0
    local successful_tests=0
    
    for test_program in "${TEST_PROGRAMS[@]}"; do
        if [ ! -f "$test_program" ]; then
            log_with_timestamp "Warning: Test program $test_program not found"
            continue
        fi
        
        log_with_timestamp "Testing baseline performance of $test_program"
        
        # Build without PGO
        if zig build -Doptimize=ReleaseFast > /dev/null 2>&1; then
            # Measure execution time
            local start_time=$(date +%s%N)
            
            if timeout 300s ./zig-out/bin/cursed-zig "$test_program" > /dev/null 2>&1; then
                local end_time=$(date +%s%N)
                local execution_time=$(((end_time - start_time) / 1000000)) # Convert to milliseconds
                
                total_time=$((total_time + execution_time))
                successful_tests=$((successful_tests + 1))
                
                log_with_timestamp "  Baseline time for $test_program: ${execution_time}ms"
            else
                log_with_timestamp "  Warning: $test_program execution failed or timed out"
            fi
        else
            log_with_timestamp "  Warning: Baseline build failed for $test_program"
        fi
    done
    
    if [ $successful_tests -gt 0 ]; then
        BASELINE_AVERAGE_TIME=$((total_time / successful_tests))
        log_with_timestamp "Baseline average execution time: ${BASELINE_AVERAGE_TIME}ms"
        return 0
    else
        log_with_timestamp "Error: No successful baseline measurements"
        return 1
    fi
}

# Function to generate PGO profile
generate_pgo_profile() {
    log_with_timestamp "Generating PGO profile data..."
    
    # Remove existing profile
    rm -f "$PROFILE_BLOB"
    
    # Build with profile generation
    log_with_timestamp "Building with profile generation enabled..."
    if ! zig build $PROFILE_GENERATE_FLAGS > /dev/null 2>&1; then
        log_with_timestamp "Error: Failed to build with profile generation"
        return 1
    fi
    
    # Run test programs to collect profile data
    for test_program in "${TEST_PROGRAMS[@]}"; do
        if [ ! -f "$test_program" ]; then continue; fi
        
        log_with_timestamp "Collecting profile data from $test_program"
        
        if timeout 300s ./zig-out/bin/cursed-zig --pgo-generate="$PROFILE_BLOB" "$test_program" > /dev/null 2>&1; then
            log_with_timestamp "  Profile data collected successfully"
        else
            log_with_timestamp "  Warning: Profile collection failed for $test_program"
        fi
    done
    
    if [ -f "$PROFILE_BLOB" ]; then
        local profile_size=$(stat -c%s "$PROFILE_BLOB" 2>/dev/null || echo "0")
        log_with_timestamp "PGO profile generated successfully (${profile_size} bytes)"
        return 0
    else
        log_with_timestamp "Error: PGO profile not generated"
        return 1
    fi
}

# Function to measure PGO performance
measure_pgo_performance() {
    log_with_timestamp "Measuring PGO-optimized performance..."
    
    # Build with PGO profile use
    log_with_timestamp "Building with PGO profile use enabled..."
    if ! zig build $PROFILE_USE_FLAGS > /dev/null 2>&1; then
        log_with_timestamp "Error: Failed to build with PGO profile use"
        return 1
    fi
    
    local total_time=0
    local successful_tests=0
    
    for test_program in "${TEST_PROGRAMS[@]}"; do
        if [ ! -f "$test_program" ]; then continue; fi
        
        log_with_timestamp "Testing PGO performance of $test_program"
        
        # Measure execution time
        local start_time=$(date +%s%N)
        
        if timeout 300s ./zig-out/bin/cursed-zig "$test_program" > /dev/null 2>&1; then
            local end_time=$(date +%s%N)
            local execution_time=$(((end_time - start_time) / 1000000)) # Convert to milliseconds
            
            total_time=$((total_time + execution_time))
            successful_tests=$((successful_tests + 1))
            
            log_with_timestamp "  PGO time for $test_program: ${execution_time}ms"
        else
            log_with_timestamp "  Warning: $test_program execution failed or timed out"
        fi
    done
    
    if [ $successful_tests -gt 0 ]; then
        PGO_AVERAGE_TIME=$((total_time / successful_tests))
        log_with_timestamp "PGO average execution time: ${PGO_AVERAGE_TIME}ms"
        return 0
    else
        log_with_timestamp "Error: No successful PGO measurements"
        return 1
    fi
}

# Function to calculate performance improvement
calculate_improvement() {
    if [ $BASELINE_AVERAGE_TIME -eq 0 ]; then
        log_with_timestamp "Error: Cannot calculate improvement, baseline time is zero"
        return 1
    fi
    
    local improvement_absolute=$((BASELINE_AVERAGE_TIME - PGO_AVERAGE_TIME))
    local improvement_percent=$(echo "scale=2; ($improvement_absolute * 100) / $BASELINE_AVERAGE_TIME" | bc -l)
    
    PERFORMANCE_IMPROVEMENT=$improvement_percent
    
    log_with_timestamp "Performance Analysis:"
    log_with_timestamp "  Baseline Average: ${BASELINE_AVERAGE_TIME}ms"
    log_with_timestamp "  PGO Optimized: ${PGO_AVERAGE_TIME}ms"
    log_with_timestamp "  Improvement: ${improvement_absolute}ms (${improvement_percent}%)"
    
    # Return success if improvement meets threshold
    if (( $(echo "$improvement_percent > $PERFORMANCE_THRESHOLD" | bc -l) )); then
        log_with_timestamp "✅ Performance improvement (${improvement_percent}%) exceeds threshold (${PERFORMANCE_THRESHOLD}%)"
        return 0
    else
        log_with_timestamp "ℹ️ Performance improvement (${improvement_percent}%) below threshold (${PERFORMANCE_THRESHOLD}%)"
        return 1
    fi
}

# Function to perform production rebuild
perform_production_rebuild() {
    log_with_timestamp "Performing production rebuild with full optimizations..."
    
    # Create backup of current build
    if [ -d "zig-out" ]; then
        cp -r zig-out zig-out.backup
        log_with_timestamp "Created backup of current build"
    fi
    
    # Build with production flags including PGO
    log_with_timestamp "Building with production optimizations: $PRODUCTION_FLAGS"
    
    if zig build $PRODUCTION_FLAGS > /dev/null 2>&1; then
        log_with_timestamp "✅ Production rebuild completed successfully"
        
        # Update last rebuild timestamp
        date +%s > "$LAST_REBUILD_FILE"
        log_with_timestamp "Updated last rebuild timestamp"
        
        # Clean up backup if rebuild was successful
        rm -rf zig-out.backup
        log_with_timestamp "Removed backup (rebuild successful)"
        
        return 0
    else
        log_with_timestamp "❌ Production rebuild failed"
        
        # Restore backup if it exists
        if [ -d "zig-out.backup" ]; then
            rm -rf zig-out
            mv zig-out.backup zig-out
            log_with_timestamp "Restored backup due to rebuild failure"
        fi
        
        return 1
    fi
}

# Function to validate rebuild
validate_rebuild() {
    log_with_timestamp "Validating production build..."
    
    local validation_passed=0
    
    for test_program in "${TEST_PROGRAMS[@]}"; do
        if [ ! -f "$test_program" ]; then continue; fi
        
        log_with_timestamp "Validating with $test_program"
        
        if timeout 300s ./zig-out/bin/cursed-zig "$test_program" > /dev/null 2>&1; then
            log_with_timestamp "  ✅ $test_program validation passed"
            validation_passed=$((validation_passed + 1))
        else
            log_with_timestamp "  ❌ $test_program validation failed"
        fi
    done
    
    if [ $validation_passed -gt 0 ]; then
        log_with_timestamp "✅ Rebuild validation completed: $validation_passed tests passed"
        return 0
    else
        log_with_timestamp "❌ Rebuild validation failed: no tests passed"
        return 1
    fi
}

# Function to generate rebuild report
generate_rebuild_report() {
    local rebuild_status=$1
    
    cat > "oracle_pgo_rebuild_report.json" << EOF
{
  "oracle_pgo_auto_rebuild": {
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "rebuild_triggered": $([ $rebuild_status -eq 0 ] && echo "true" || echo "false"),
    "performance_improvement_percent": ${PERFORMANCE_IMPROVEMENT:-0},
    "performance_threshold_percent": $PERFORMANCE_THRESHOLD,
    "baseline_time_ms": ${BASELINE_AVERAGE_TIME:-0},
    "pgo_time_ms": ${PGO_AVERAGE_TIME:-0},
    "profile_blob_size": $(stat -c%s "$PROFILE_BLOB" 2>/dev/null || echo "0"),
    "test_programs": [
$(printf '      "%s",' "${TEST_PROGRAMS[@]}" | sed 's/,$//')
    ],
    "rebuild_status": "$([ $rebuild_status -eq 0 ] && echo "success" || echo "failed")"
  }
}
EOF
    
    log_with_timestamp "Generated rebuild report: oracle_pgo_rebuild_report.json"
}

# Main execution function
main() {
    log_with_timestamp "Starting Oracle PGO Auto-Rebuild System"
    log_with_timestamp "Performance threshold: ${PERFORMANCE_THRESHOLD}%"
    log_with_timestamp "Minimum rebuild interval: ${MIN_REBUILD_INTERVAL} seconds"
    
    # Check if rebuild is needed (time-based throttling)
    if ! check_rebuild_needed; then
        generate_rebuild_report 2  # Status 2 = skipped
        exit 0
    fi
    
    # Initialize performance variables
    BASELINE_AVERAGE_TIME=0
    PGO_AVERAGE_TIME=0
    PERFORMANCE_IMPROVEMENT=0
    
    # Step 1: Measure baseline performance
    if ! measure_baseline_performance; then
        log_with_timestamp "❌ Failed to measure baseline performance"
        generate_rebuild_report 1
        exit 1
    fi
    
    # Step 2: Generate PGO profile
    if ! generate_pgo_profile; then
        log_with_timestamp "❌ Failed to generate PGO profile"
        generate_rebuild_report 1
        exit 1
    fi
    
    # Step 3: Measure PGO performance
    if ! measure_pgo_performance; then
        log_with_timestamp "❌ Failed to measure PGO performance"
        generate_rebuild_report 1
        exit 1
    fi
    
    # Step 4: Calculate improvement and decide on rebuild
    if calculate_improvement; then
        log_with_timestamp "🚀 Performance improvement detected, triggering rebuild..."
        
        # Step 5: Perform production rebuild
        if perform_production_rebuild; then
            # Step 6: Validate rebuild
            if validate_rebuild; then
                log_with_timestamp "🎉 Auto-rebuild completed successfully!"
                log_with_timestamp "Performance improved by ${PERFORMANCE_IMPROVEMENT}%"
                generate_rebuild_report 0
                exit 0
            else
                log_with_timestamp "❌ Rebuild validation failed"
                generate_rebuild_report 1
                exit 1
            fi
        else
            log_with_timestamp "❌ Production rebuild failed"
            generate_rebuild_report 1
            exit 1
        fi
    else
        log_with_timestamp "ℹ️ Performance improvement below threshold, no rebuild needed"
        generate_rebuild_report 2  # Status 2 = no rebuild needed
        exit 0
    fi
}

# Handle interruption gracefully
trap 'log_with_timestamp "Auto-rebuild interrupted"; exit 130' INT TERM

# Ensure required tools are available
if ! command -v bc >/dev/null 2>&1; then
    echo -e "${RED}Error: bc calculator required but not available${NC}"
    exit 1
fi

# Run main function
main "$@"
