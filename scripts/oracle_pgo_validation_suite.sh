#!/bin/bash

# Oracle PGO Validation Suite
# Comprehensive validation of PGO system with representative CURSED programs

set -e

echo "🎯 Oracle PGO Validation Suite"
echo "=============================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
PROFILE_BLOB="oracle_pgo_validation.blob"
RESULTS_JSON="oracle_pgo_validation_results.json"
DETAILED_LOG="oracle_pgo_validation.log"
MIN_IMPROVEMENT_PERCENT=3.0  # Minimum meaningful improvement
ITERATIONS=3  # Number of test iterations for averaging

# Representative CURSED programs for testing
REPRESENTATIVE_PROGRAMS=(
    "benchmarks/oracle_macro_performance_suite.csd:macro:Oracle macro performance suite"
    "benchmarks/pgo_benchmark_suite.csd:micro:PGO-specific benchmark suite" 
    "comprehensive_stdlib_test.csd:stdlib:Comprehensive standard library test"
    "advanced_features_test.csd:language:Advanced language features test"
    "comprehensive_test.csd:integration:Full integration test"
    "test_suite/basic_syntax.csd:basic:Basic syntax validation"
    "examples/fibonacci_recursive.csd:algorithm:Recursive algorithm test"
    "examples/concurrent_processing.csd:concurrency:Concurrency performance test"
)

# Validation categories
declare -A CATEGORY_RESULTS
declare -A PROGRAM_RESULTS

# Function to log with timestamp
log_with_timestamp() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" | tee -a "$DETAILED_LOG"
}

# Function to extract program info
extract_program_info() {
    local program_spec=$1
    
    PROGRAM_PATH=$(echo "$program_spec" | cut -d: -f1)
    PROGRAM_CATEGORY=$(echo "$program_spec" | cut -d: -f2)
    PROGRAM_DESCRIPTION=$(echo "$program_spec" | cut -d: -f3)
}

# Function to build compiler with specific flags
build_compiler() {
    local build_mode=$1
    local profile_path=$2
    
    log_with_timestamp "Building compiler in $build_mode mode..."
    
    case "$build_mode" in
        "baseline")
            if zig build -Doptimize=ReleaseFast > /dev/null 2>&1; then
                log_with_timestamp "✅ Baseline build completed"
                return 0
            else
                log_with_timestamp "❌ Baseline build failed"
                return 1
            fi
            ;;
        "pgo-generate")
            if zig build -Doptimize=ReleaseFast -Dpgo-generate=true > /dev/null 2>&1; then
                log_with_timestamp "✅ PGO generation build completed"
                return 0
            else
                log_with_timestamp "❌ PGO generation build failed"
                return 1
            fi
            ;;
        "pgo-use")
            if [ ! -f "$profile_path" ]; then
                log_with_timestamp "❌ Profile blob not found: $profile_path"
                return 1
            fi
            
            if zig build -Doptimize=ReleaseFast -Dpgo-use=true -Dpgo-profile="$profile_path" > /dev/null 2>&1; then
                log_with_timestamp "✅ PGO use build completed"
                return 0
            else
                log_with_timestamp "❌ PGO use build failed"
                return 1
            fi
            ;;
        "production")
            if [ ! -f "$profile_path" ]; then
                log_with_timestamp "❌ Profile blob not found: $profile_path"
                return 1
            fi
            
            if zig build -Doptimize=ReleaseFast -Dpgo-use=true -Dpgo-profile="$profile_path" -Dllvm-opt=O3 -Dlto=true > /dev/null 2>&1; then
                log_with_timestamp "✅ Production build completed"
                return 0
            else
                log_with_timestamp "❌ Production build failed"
                return 1
            fi
            ;;
        *)
            log_with_timestamp "❌ Unknown build mode: $build_mode"
            return 1
            ;;
    esac
}

# Function to measure program performance
measure_program_performance() {
    local program_path=$1
    local measurement_label=$2
    local iterations=${3:-$ITERATIONS}
    
    log_with_timestamp "Measuring performance: $measurement_label for $program_path"
    
    if [ ! -f "$program_path" ]; then
        log_with_timestamp "❌ Program not found: $program_path"
        echo "0:0:0"  # compilation_ms:execution_ms:memory_kb
        return 1
    fi
    
    local total_compilation_time=0
    local total_execution_time=0
    local total_memory_usage=0
    local successful_runs=0
    
    for ((i=1; i<=iterations; i++)); do
        log_with_timestamp "  Iteration $i/$iterations"
        
        # Clean previous artifacts
        local binary_path="${program_path%.csd}"
        rm -f "$binary_path" "${program_path%.csd}.ll" "${program_path%.csd}.o" 2>/dev/null || true
        
        # Measure compilation time
        local compile_start=$(date +%s%N)
        
        if timeout 300s ./zig-out/bin/cursed-zig "$program_path" >/dev/null 2>/dev/null; then
            local compile_end=$(date +%s%N)
            local compilation_ms=$(((compile_end - compile_start) / 1000000))
            
            # Measure execution time and memory usage
            if [ -f "$binary_path" ]; then
                local exec_start=$(date +%s%N)
                
                if timeout 300s /usr/bin/time -f "%M" "./$binary_path" >/dev/null 2>/tmp/memory_$i; then
                    local exec_end=$(date +%s%N)
                    local execution_ms=$(((exec_end - exec_start) / 1000000))
                    local memory_kb=$(cat "/tmp/memory_$i" 2>/dev/null || echo "0")
                    
                    total_compilation_time=$((total_compilation_time + compilation_ms))
                    total_execution_time=$((total_execution_time + execution_ms))
                    total_memory_usage=$((total_memory_usage + memory_kb))
                    successful_runs=$((successful_runs + 1))
                    
                    log_with_timestamp "    Compile: ${compilation_ms}ms, Exec: ${execution_ms}ms, Mem: ${memory_kb}KB"
                else
                    log_with_timestamp "    ❌ Execution failed or timed out"
                fi
                
                rm -f "/tmp/memory_$i"
            else
                log_with_timestamp "    ❌ No binary produced"
            fi
        else
            log_with_timestamp "    ❌ Compilation failed or timed out"
        fi
    done
    
    if [ $successful_runs -gt 0 ]; then
        local avg_compilation=$((total_compilation_time / successful_runs))
        local avg_execution=$((total_execution_time / successful_runs))
        local avg_memory=$((total_memory_usage / successful_runs))
        
        log_with_timestamp "  Average: Compile ${avg_compilation}ms, Exec ${avg_execution}ms, Mem ${avg_memory}KB"
        echo "$avg_compilation:$avg_execution:$avg_memory"
        return 0
    else
        log_with_timestamp "  ❌ All runs failed"
        echo "0:0:0"
        return 1
    fi
}

# Function to run PGO profile generation
run_profile_generation() {
    log_with_timestamp "Running PGO profile generation phase..."
    
    # Remove existing profile
    rm -f "$PROFILE_BLOB"
    
    # Build compiler with profile generation
    if ! build_compiler "pgo-generate" ""; then
        return 1
    fi
    
    # Run representative programs to collect profile data
    for program_spec in "${REPRESENTATIVE_PROGRAMS[@]}"; do
        extract_program_info "$program_spec"
        
        if [ ! -f "$PROGRAM_PATH" ]; then
            log_with_timestamp "⚠️ Skipping $PROGRAM_PATH (not found)"
            continue
        fi
        
        log_with_timestamp "Collecting profile data from: $PROGRAM_DESCRIPTION"
        
        # Run program with profile generation
        if timeout 300s ./zig-out/bin/cursed-zig --pgo-generate="$PROFILE_BLOB" "$PROGRAM_PATH" >/dev/null 2>&1; then
            log_with_timestamp "  ✅ Profile data collected"
        else
            log_with_timestamp "  ⚠️ Profile collection failed"
        fi
    done
    
    # Verify profile blob was created
    if [ -f "$PROFILE_BLOB" ]; then
        local blob_size=$(stat -c%s "$PROFILE_BLOB")
        log_with_timestamp "✅ Profile generation completed (${blob_size} bytes)"
        return 0
    else
        log_with_timestamp "❌ Profile blob was not created"
        return 1
    fi
}

# Function to validate PGO improvements
validate_pgo_improvements() {
    log_with_timestamp "Validating PGO performance improvements..."
    
    local total_programs=0
    local improved_programs=0
    local total_improvement=0
    
    # Initialize results structure
    cat > "$RESULTS_JSON" << 'EOF'
{
  "oracle_pgo_validation": {
    "timestamp": "TIMESTAMP_PLACEHOLDER",
    "profile_blob_size": BLOB_SIZE_PLACEHOLDER,
    "validation_results": {
EOF
    
    local first_program=true
    
    # Test each representative program
    for program_spec in "${REPRESENTATIVE_PROGRAMS[@]}"; do
        extract_program_info "$program_spec"
        
        if [ ! -f "$PROGRAM_PATH" ]; then
            log_with_timestamp "⚠️ Skipping $PROGRAM_PATH (not found)"
            continue
        fi
        
        log_with_timestamp "Validating: $PROGRAM_DESCRIPTION ($PROGRAM_CATEGORY)"
        
        # Measure baseline performance
        if ! build_compiler "baseline" ""; then
            log_with_timestamp "❌ Failed to build baseline for $PROGRAM_PATH"
            continue
        fi
        
        local baseline_metrics=$(measure_program_performance "$PROGRAM_PATH" "baseline")
        local baseline_compile=$(echo "$baseline_metrics" | cut -d: -f1)
        local baseline_exec=$(echo "$baseline_metrics" | cut -d: -f2)
        local baseline_memory=$(echo "$baseline_metrics" | cut -d: -f3)
        
        # Measure PGO performance
        if ! build_compiler "pgo-use" "$PROFILE_BLOB"; then
            log_with_timestamp "❌ Failed to build PGO version for $PROGRAM_PATH"
            continue
        fi
        
        local pgo_metrics=$(measure_program_performance "$PROGRAM_PATH" "PGO")
        local pgo_compile=$(echo "$pgo_metrics" | cut -d: -f1)
        local pgo_exec=$(echo "$pgo_metrics" | cut -d: -f2)
        local pgo_memory=$(echo "$pgo_metrics" | cut -d: -f3)
        
        # Calculate improvements
        local compile_improvement=0
        local exec_improvement=0
        local overall_improvement=0
        
        if [ $baseline_compile -gt 0 ] && [ $pgo_compile -gt 0 ]; then
            compile_improvement=$(echo "scale=2; (($baseline_compile - $pgo_compile) * 100) / $baseline_compile" | bc -l)
        fi
        
        if [ $baseline_exec -gt 0 ] && [ $pgo_exec -gt 0 ]; then
            exec_improvement=$(echo "scale=2; (($baseline_exec - $pgo_exec) * 100) / $baseline_exec" | bc -l)
        fi
        
        # Overall improvement is weighted average (70% execution, 30% compilation)
        overall_improvement=$(echo "scale=2; ($exec_improvement * 0.7) + ($compile_improvement * 0.3)" | bc -l)
        
        # Add to results JSON
        if [ "$first_program" = false ]; then
            echo "," >> "$RESULTS_JSON"
        fi
        
        cat >> "$RESULTS_JSON" << EOF
      "${PROGRAM_PATH}": {
        "description": "$PROGRAM_DESCRIPTION",
        "category": "$PROGRAM_CATEGORY",
        "baseline": {
          "compilation_ms": $baseline_compile,
          "execution_ms": $baseline_exec,
          "memory_kb": $baseline_memory
        },
        "pgo_optimized": {
          "compilation_ms": $pgo_compile,
          "execution_ms": $pgo_exec,
          "memory_kb": $pgo_memory
        },
        "improvements": {
          "compilation_percent": $compile_improvement,
          "execution_percent": $exec_improvement,
          "overall_percent": $overall_improvement
        }
      }EOF
        
        first_program=false
        total_programs=$((total_programs + 1))
        
        # Check if improvement is meaningful
        if (( $(echo "$overall_improvement > $MIN_IMPROVEMENT_PERCENT" | bc -l) )); then
            improved_programs=$((improved_programs + 1))
            log_with_timestamp "  ✅ Meaningful improvement: ${overall_improvement}%"
        else
            log_with_timestamp "  ℹ️ Minor/no improvement: ${overall_improvement}%"
        fi
        
        total_improvement=$(echo "$total_improvement + $overall_improvement" | bc -l)
        
        # Store category results
        CATEGORY_RESULTS["$PROGRAM_CATEGORY"]=$(echo "${CATEGORY_RESULTS[$PROGRAM_CATEGORY]:-0} + $overall_improvement" | bc -l)
    done
    
    # Complete results JSON
    cat >> "$RESULTS_JSON" << 'EOF'
    }
  }
}
EOF
    
    # Calculate summary metrics
    local average_improvement=0
    if [ $total_programs -gt 0 ]; then
        average_improvement=$(echo "scale=2; $total_improvement / $total_programs" | bc -l)
    fi
    
    local improvement_rate=0
    if [ $total_programs -gt 0 ]; then
        improvement_rate=$(echo "scale=1; ($improved_programs * 100) / $total_programs" | bc -l)
    fi
    
    # Update JSON with summary and metadata
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    local blob_size=$(stat -c%s "$PROFILE_BLOB" 2>/dev/null || echo "0")
    
    sed -i "s/TIMESTAMP_PLACEHOLDER/$timestamp/g" "$RESULTS_JSON"
    sed -i "s/BLOB_SIZE_PLACEHOLDER/$blob_size/g" "$RESULTS_JSON"
    
    # Add summary to JSON
    local temp_file=$(mktemp)
    cat "$RESULTS_JSON" | sed '$d' > "$temp_file" # Remove last }
    
    cat >> "$temp_file" << EOF
    },
    "summary": {
      "total_programs_tested": $total_programs,
      "programs_with_meaningful_improvement": $improved_programs,
      "improvement_rate_percent": $improvement_rate,
      "average_improvement_percent": $average_improvement,
      "minimum_improvement_threshold_percent": $MIN_IMPROVEMENT_PERCENT
    }
  }
}
EOF
    
    mv "$temp_file" "$RESULTS_JSON"
    
    # Display summary
    log_with_timestamp ""
    log_with_timestamp "📊 PGO Validation Summary"
    log_with_timestamp "========================"
    log_with_timestamp "Total programs tested: $total_programs"
    log_with_timestamp "Programs with meaningful improvement: $improved_programs"
    log_with_timestamp "Improvement rate: ${improvement_rate}%"
    log_with_timestamp "Average improvement: ${average_improvement}%"
    log_with_timestamp ""
    
    # Category breakdown
    log_with_timestamp "📋 Performance by Category:"
    for category in "${!CATEGORY_RESULTS[@]}"; do
        local category_avg=$(echo "scale=1; ${CATEGORY_RESULTS[$category]} / 1" | bc -l 2>/dev/null || echo "0")
        log_with_timestamp "  $category: ${category_avg}%"
    done
    
    # Determine overall success
    if (( $(echo "$improvement_rate > 50" | bc -l) )) && (( $(echo "$average_improvement > $MIN_IMPROVEMENT_PERCENT" | bc -l) )); then
        log_with_timestamp ""
        log_with_timestamp "🎉 PGO VALIDATION PASSED"
        log_with_timestamp "Significant performance improvements detected"
        return 0
    else
        log_with_timestamp ""
        log_with_timestamp "⚠️ PGO VALIDATION MARGINAL"
        log_with_timestamp "Limited performance improvements detected"
        return 1
    fi
}

# Function to generate production recommendations
generate_production_recommendations() {
    local validation_result=$1
    
    log_with_timestamp "Generating production recommendations..."
    
    if [ $validation_result -eq 0 ]; then
        cat >> "$RESULTS_JSON.tmp" << 'EOF'
,
    "production_recommendations": {
      "pgo_recommended": true,
      "recommended_flags": [
        "-Doptimize=ReleaseFast",
        "-Dpgo-use=true",
        "-Dllvm-opt=O3",
        "-Dlto=true"
      ],
      "profile_generation_workflow": [
        "1. Build with -Dpgo-generate=true",
        "2. Run representative workloads",
        "3. Collect profile blob",
        "4. Rebuild with -Dpgo-use=true"
      ],
      "expected_benefits": [
        "Improved execution performance",
        "Better branch prediction",
        "Optimized function inlining",
        "Enhanced loop optimizations"
      ]
    }
EOF
    else
        cat >> "$RESULTS_JSON.tmp" << 'EOF'
,
    "production_recommendations": {
      "pgo_recommended": false,
      "reason": "Limited performance improvements detected",
      "alternative_optimizations": [
        "-Doptimize=ReleaseFast",
        "-Dllvm-opt=O3",
        "-Dlto=true"
      ],
      "suggestions": [
        "Profile workload may not be representative",
        "Consider longer profiling runs",
        "Verify hot paths are exercised"
      ]
    }
EOF
    fi
    
    # Merge recommendations into main results file
    cat "$RESULTS_JSON" | sed '$d' > "$RESULTS_JSON.tmp"  # Remove last }
    cat "$RESULTS_JSON.tmp" >> "$RESULTS_JSON.new"
    echo "  }" >> "$RESULTS_JSON.new"
    echo "}" >> "$RESULTS_JSON.new"
    
    mv "$RESULTS_JSON.new" "$RESULTS_JSON"
    rm -f "$RESULTS_JSON.tmp"
    
    log_with_timestamp "✅ Production recommendations added to results"
}

# Main execution function
main() {
    log_with_timestamp "Starting Oracle PGO Validation Suite"
    log_with_timestamp "Minimum improvement threshold: ${MIN_IMPROVEMENT_PERCENT}%"
    log_with_timestamp "Test iterations per program: $ITERATIONS"
    log_with_timestamp "Representative programs: ${#REPRESENTATIVE_PROGRAMS[@]}"
    
    # Ensure required tools are available
    if ! command -v bc >/dev/null 2>&1; then
        log_with_timestamp "❌ Error: bc calculator required but not available"
        exit 1
    fi
    
    # Step 1: Build baseline compiler
    log_with_timestamp ""
    log_with_timestamp "🔨 Step 1: Building baseline compiler"
    if ! build_compiler "baseline" ""; then
        log_with_timestamp "❌ Failed to build baseline compiler"
        exit 1
    fi
    
    # Step 2: Generate PGO profile
    log_with_timestamp ""
    log_with_timestamp "📊 Step 2: Generating PGO profile"
    if ! run_profile_generation; then
        log_with_timestamp "❌ Failed to generate PGO profile"
        exit 1
    fi
    
    # Step 3: Validate PGO improvements
    log_with_timestamp ""
    log_with_timestamp "🎯 Step 3: Validating PGO improvements"
    if validate_pgo_improvements; then
        validation_result=0
    else
        validation_result=1
    fi
    
    # Step 4: Generate production recommendations
    log_with_timestamp ""
    log_with_timestamp "📋 Step 4: Generating production recommendations"
    generate_production_recommendations $validation_result
    
    # Final summary
    log_with_timestamp ""
    log_with_timestamp "🏁 Oracle PGO Validation Suite completed"
    log_with_timestamp "Results saved to: $RESULTS_JSON"
    log_with_timestamp "Detailed log: $DETAILED_LOG"
    
    if [ $validation_result -eq 0 ]; then
        echo -e "${GREEN}🎉 PGO validation PASSED - Significant improvements detected${NC}"
    else
        echo -e "${YELLOW}⚠️ PGO validation MARGINAL - Limited improvements detected${NC}"
    fi
    
    exit $validation_result
}

# Handle interruption gracefully
trap 'log_with_timestamp "Validation suite interrupted"; exit 130' INT TERM

# Clear previous logs
> "$DETAILED_LOG"

# Run main function
main "$@"
