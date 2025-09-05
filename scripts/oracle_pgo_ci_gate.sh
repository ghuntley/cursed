#!/bin/bash

# Oracle PGO Performance Regression CI Gate
# Fails CI if performance regression >5% is detected

set -e

echo "🎯 Oracle PGO Performance Regression Gate"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REGRESSION_THRESHOLD_PERCENT=5.0
BASELINE_DB="oracle_pgo_baseline.json"
CURRENT_RESULTS="oracle_pgo_current.json"
PROFILE_BLOB="oracle_pgo_profile.blob"
CI_REPORT="oracle_pgo_ci_report.json"

# Performance test files
PERFORMANCE_TESTS=(
    "benchmarks/pgo_benchmark_suite.💀"
    "comprehensive_stdlib_test.💀" 
    "advanced_features_test.💀"
    "comprehensive_test.💀"
)

# Exit codes
EXIT_SUCCESS=0
EXIT_PERFORMANCE_REGRESSION=1
EXIT_BUILD_FAILURE=2
EXIT_TEST_FAILURE=3

echo -e "${BLUE}Step 1: Building optimized compiler...${NC}"
if ! zig build -Doptimize=ReleaseFast -Dpgo-enabled=true; then
    echo -e "${RED}❌ Build failed${NC}"
    exit $EXIT_BUILD_FAILURE
fi

echo -e "${GREEN}✅ Build completed${NC}"
echo

# Function to measure performance
measure_performance() {
    local test_file=$1
    local profile_mode=$2  # "generate" or "use"
    local iterations=${3:-5}
    
    echo -e "${YELLOW}Measuring performance for ${test_file} (${profile_mode} mode)...${NC}"
    
    local total_compilation_time=0
    local total_execution_time=0
    local total_memory_usage=0
    local success_count=0
    
    for ((i=1; i<=iterations; i++)); do
        echo -n "  Iteration $i/$iterations: "
        
        # Clear cache
        rm -f "${test_file%.💀}" "${test_file%.💀}.ll" "${test_file%.💀}.o" 2>/dev/null || true
        
        # Measure compilation time
        local compile_start=$(date +%s%N)
        
        local compile_cmd="./zig-out/bin/cursed-zig"
        if [ "$profile_mode" = "generate" ]; then
            compile_cmd="$compile_cmd --pgo-generate=$PROFILE_BLOB"
        elif [ "$profile_mode" = "use" ] && [ -f "$PROFILE_BLOB" ]; then
            compile_cmd="$compile_cmd --pgo-use=$PROFILE_BLOB"
        fi
        
        if timeout 120s $compile_cmd "$test_file" >/dev/null 2>&1; then
            local compile_end=$(date +%s%N)
            local compilation_ms=$(((compile_end - compile_start) / 1000000))
            
            # Measure execution time and memory usage
            local exec_start=$(date +%s%N)
            local memory_kb=0
            
            if [ -f "${test_file%.💀}" ]; then
                if timeout 60s /usr/bin/time -f "%M" "./${test_file%.💀}" >/dev/null 2>/tmp/memory_output; then
                    local exec_end=$(date +%s%N)
                    local execution_ms=$(((exec_end - exec_start) / 1000000))
                    
                    memory_kb=$(cat /tmp/memory_output 2>/dev/null || echo "0")
                    rm -f /tmp/memory_output
                    
                    total_compilation_time=$((total_compilation_time + compilation_ms))
                    total_execution_time=$((total_execution_time + execution_ms))
                    total_memory_usage=$((total_memory_usage + memory_kb))
                    success_count=$((success_count + 1))
                    
                    echo -e "${GREEN}${compilation_ms}ms compile, ${execution_ms}ms exec, ${memory_kb}KB mem${NC}"
                else
                    echo -e "${RED}EXEC FAILED${NC}"
                fi
            else
                echo -e "${RED}NO BINARY${NC}"
            fi
        else
            echo -e "${RED}COMPILE FAILED${NC}"
        fi
    done
    
    if [ $success_count -gt 0 ]; then
        local avg_compile_time=$((total_compilation_time / success_count))
        local avg_execution_time=$((total_execution_time / success_count))
        local avg_memory_usage=$((total_memory_usage / success_count))
        
        echo "  Average: ${avg_compile_time}ms compile, ${avg_execution_time}ms exec, ${avg_memory_usage}KB mem"
        
        # Return values via global variables (bash limitation workaround)
        MEASURED_COMPILE_TIME=$avg_compile_time
        MEASURED_EXECUTION_TIME=$avg_execution_time
        MEASURED_MEMORY_USAGE=$avg_memory_usage
        MEASURED_SUCCESS=true
    else
        echo -e "  ${RED}All measurements failed${NC}"
        MEASURED_SUCCESS=false
    fi
}

# Function to generate baseline if it doesn't exist
generate_baseline() {
    echo -e "${BLUE}Step 2a: Generating performance baseline...${NC}"
    
    cat > "$BASELINE_DB" << 'EOF'
{
  "version": "1.0",
  "timestamp": "TIMESTAMP_PLACEHOLDER",
  "baseline_metrics": {
EOF
    
    local first_test=true
    for test_file in "${PERFORMANCE_TESTS[@]}"; do
        if [ ! -f "$test_file" ]; then
            echo -e "${YELLOW}Warning: $test_file not found, skipping${NC}"
            continue
        fi
        
        measure_performance "$test_file" "baseline" 3
        
        if [ "$MEASURED_SUCCESS" = true ]; then
            if [ "$first_test" = false ]; then
                echo "," >> "$BASELINE_DB"
            fi
            
            cat >> "$BASELINE_DB" << EOF
    "${test_file}": {
      "compilation_time_ms": $MEASURED_COMPILE_TIME,
      "execution_time_ms": $MEASURED_EXECUTION_TIME,
      "memory_usage_kb": $MEASURED_MEMORY_USAGE
    }EOF
            first_test=false
        fi
    done
    
    cat >> "$BASELINE_DB" << 'EOF'
  }
}
EOF
    
    # Replace timestamp placeholder
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    sed -i "s/TIMESTAMP_PLACEHOLDER/$timestamp/g" "$BASELINE_DB"
    
    echo -e "${GREEN}✅ Baseline generated and saved to $BASELINE_DB${NC}"
}

# Function to load baseline metrics
load_baseline() {
    if [ ! -f "$BASELINE_DB" ]; then
        echo -e "${YELLOW}No baseline found, generating new baseline...${NC}"
        generate_baseline
        return
    fi
    
    echo -e "${BLUE}Step 2b: Loading performance baseline...${NC}"
    echo "Using baseline from: $BASELINE_DB"
    
    # Extract baseline timestamp
    local baseline_timestamp=$(jq -r '.timestamp' "$BASELINE_DB" 2>/dev/null || echo "unknown")
    echo "Baseline timestamp: $baseline_timestamp"
}

# Function to run PGO profile generation
run_pgo_generation() {
    echo -e "${BLUE}Step 3: Running PGO profile generation...${NC}"
    
    rm -f "$PROFILE_BLOB"
    
    for test_file in "${PERFORMANCE_TESTS[@]}"; do
        if [ ! -f "$test_file" ]; then continue; fi
        
        echo "  Generating profile data for $test_file"
        measure_performance "$test_file" "generate" 1
    done
    
    if [ -f "$PROFILE_BLOB" ]; then
        local blob_size=$(stat -c%s "$PROFILE_BLOB" 2>/dev/null || echo "0")
        echo -e "${GREEN}✅ Profile blob generated (${blob_size} bytes)${NC}"
    else
        echo -e "${YELLOW}⚠️ Warning: Profile blob not created${NC}"
    fi
}

# Function to run performance tests with PGO
run_pgo_performance_tests() {
    echo -e "${BLUE}Step 4: Running PGO-optimized performance tests...${NC}"
    
    cat > "$CURRENT_RESULTS" << 'EOF'
{
  "version": "1.0",
  "timestamp": "TIMESTAMP_PLACEHOLDER",
  "pgo_enabled": true,
  "current_metrics": {
EOF
    
    local first_test=true
    local total_regressions=0
    local total_improvements=0
    
    for test_file in "${PERFORMANCE_TESTS[@]}"; do
        if [ ! -f "$test_file" ]; then continue; fi
        
        measure_performance "$test_file" "use" 5
        
        if [ "$MEASURED_SUCCESS" = true ]; then
            if [ "$first_test" = false ]; then
                echo "," >> "$CURRENT_RESULTS"
            fi
            
            cat >> "$CURRENT_RESULTS" << EOF
    "${test_file}": {
      "compilation_time_ms": $MEASURED_COMPILE_TIME,
      "execution_time_ms": $MEASURED_EXECUTION_TIME,
      "memory_usage_kb": $MEASURED_MEMORY_USAGE
    }EOF
            first_test=false
            
            # Compare with baseline
            if command -v jq >/dev/null 2>&1 && [ -f "$BASELINE_DB" ]; then
                local baseline_compile=$(jq -r ".baseline_metrics[\"$test_file\"].compilation_time_ms" "$BASELINE_DB" 2>/dev/null || echo "0")
                local baseline_exec=$(jq -r ".baseline_metrics[\"$test_file\"].execution_time_ms" "$BASELINE_DB" 2>/dev/null || echo "0")
                
                if [ "$baseline_compile" != "null" ] && [ "$baseline_compile" != "0" ]; then
                    local compile_change_percent=$(echo "scale=2; (($MEASURED_COMPILE_TIME - $baseline_compile) * 100) / $baseline_compile" | bc -l 2>/dev/null || echo "0")
                    local exec_change_percent=$(echo "scale=2; (($MEASURED_EXECUTION_TIME - $baseline_exec) * 100) / $baseline_exec" | bc -l 2>/dev/null || echo "0")
                    
                    echo "    Compilation change: ${compile_change_percent}%"
                    echo "    Execution change: ${exec_change_percent}%"
                    
                    # Check for regression
                    if (( $(echo "$compile_change_percent > $REGRESSION_THRESHOLD_PERCENT" | bc -l) )) || 
                       (( $(echo "$exec_change_percent > $REGRESSION_THRESHOLD_PERCENT" | bc -l) )); then
                        echo -e "    ${RED}⚠️ REGRESSION DETECTED${NC}"
                        total_regressions=$((total_regressions + 1))
                    elif (( $(echo "$compile_change_percent < -2.0" | bc -l) )) || 
                         (( $(echo "$exec_change_percent < -2.0" | bc -l) )); then
                        echo -e "    ${GREEN}✅ IMPROVEMENT DETECTED${NC}"
                        total_improvements=$((total_improvements + 1))
                    fi
                fi
            fi
        fi
    done
    
    cat >> "$CURRENT_RESULTS" << 'EOF'
  }
}
EOF
    
    # Replace timestamp placeholder
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    sed -i "s/TIMESTAMP_PLACEHOLDER/$timestamp/g" "$CURRENT_RESULTS"
    
    echo
    echo -e "${BLUE}Performance Analysis Summary:${NC}"
    echo "  Total regressions detected: $total_regressions"
    echo "  Total improvements detected: $total_improvements"
    
    return $total_regressions
}

# Function to generate CI report
generate_ci_report() {
    local regression_count=$1
    local exit_code=$2
    
    echo -e "${BLUE}Step 5: Generating CI report...${NC}"
    
    local status="PASS"
    local status_message="No performance regressions detected"
    
    if [ $exit_code -eq $EXIT_PERFORMANCE_REGRESSION ]; then
        status="FAIL"
        status_message="Performance regression detected (>$REGRESSION_THRESHOLD_PERCENT%)"
    elif [ $exit_code -eq $EXIT_BUILD_FAILURE ]; then
        status="FAIL"
        status_message="Build failure"
    elif [ $exit_code -eq $EXIT_TEST_FAILURE ]; then
        status="FAIL"
        status_message="Test execution failure"
    fi
    
    cat > "$CI_REPORT" << EOF
{
  "oracle_pgo_ci_gate": {
    "version": "1.0",
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "status": "$status",
    "message": "$status_message",
    "regression_threshold_percent": $REGRESSION_THRESHOLD_PERCENT,
    "regressions_detected": $regression_count,
    "exit_code": $exit_code,
    "artifacts": {
      "baseline_db": "$BASELINE_DB",
      "current_results": "$CURRENT_RESULTS",
      "profile_blob": "$PROFILE_BLOB"
    }
  }
}
EOF
    
    echo -e "${GREEN}✅ CI report saved to $CI_REPORT${NC}"
    
    # Display report summary
    echo
    echo -e "${BLUE}CI Gate Summary:${NC}"
    echo "Status: $status"
    echo "Message: $status_message"
    echo "Exit Code: $exit_code"
    
    if [ -f "$CI_REPORT" ]; then
        echo "Full report available at: $CI_REPORT"
    fi
}

# Main execution
main() {
    echo "Starting Oracle PGO CI Gate at $(date)"
    echo "Regression threshold: $REGRESSION_THRESHOLD_PERCENT%"
    echo
    
    # Ensure required tools are available
    if ! command -v jq >/dev/null 2>&1; then
        echo -e "${YELLOW}Warning: jq not available, baseline comparison will be limited${NC}"
    fi
    
    if ! command -v bc >/dev/null 2>&1; then
        echo -e "${RED}Error: bc calculator required but not available${NC}"
        exit $EXIT_TEST_FAILURE
    fi
    
    local exit_code=$EXIT_SUCCESS
    
    # Load baseline
    load_baseline
    
    # Run PGO generation
    run_pgo_generation
    
    # Run performance tests
    if ! run_pgo_performance_tests; then
        regression_count=$?
        if [ $regression_count -gt 0 ]; then
            echo -e "${RED}❌ Performance regression detected: $regression_count tests${NC}"
            exit_code=$EXIT_PERFORMANCE_REGRESSION
        fi
    fi
    
    # Generate final CI report
    generate_ci_report $regression_count $exit_code
    
    # Final status
    echo
    if [ $exit_code -eq $EXIT_SUCCESS ]; then
        echo -e "${GREEN}🎉 Oracle PGO CI Gate PASSED${NC}"
        echo "No performance regressions detected"
    else
        echo -e "${RED}❌ Oracle PGO CI Gate FAILED${NC}"
        case $exit_code in
            $EXIT_PERFORMANCE_REGRESSION)
                echo "Performance regression exceeds $REGRESSION_THRESHOLD_PERCENT% threshold"
                ;;
            $EXIT_BUILD_FAILURE)
                echo "Build failure prevented testing"
                ;;
            $EXIT_TEST_FAILURE)
                echo "Test execution failure"
                ;;
        esac
    fi
    
    echo "Gate completed at $(date)"
    exit $exit_code
}

# Handle interruption gracefully
trap 'echo -e "\n${YELLOW}CI Gate interrupted${NC}"; exit 130' INT TERM

# Run main function
main "$@"
