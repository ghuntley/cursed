#!/bin/bash

# CURSED Memory Management Validation Script
# P1 Critical - Memory leak detection and correctness validation

set -e  # Exit on any error

echo "CURSED Memory Management Validation Suite"
echo "========================================="

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if valgrind is installed
check_valgrind() {
    print_status "Checking for valgrind..."
    if ! command -v valgrind &> /dev/null; then
        print_warning "Valgrind not found. Installing valgrind..."
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y valgrind
        elif command -v yum &> /dev/null; then
            sudo yum install -y valgrind
        elif command -v dnf &> /dev/null; then
            sudo dnf install -y valgrind
        else
            print_error "Cannot install valgrind automatically. Please install manually."
            exit 1
        fi
    fi
    print_status "Valgrind is available"
}

# Build the memory test
build_memory_test() {
    print_status "Building memory management test..."
    
    # Clean previous builds
    rm -f ./cursed_memory_test
    
    # Build with debug symbols for valgrind
    if ! zig build -Doptimize=Debug 2>&1 | tee build_output.log; then
        print_error "Build failed. Check build_output.log for details."
        exit 1
    fi
    
    # Check if the memory test was compiled
    if [[ -f "./zig-out/bin/cursed-zig" ]]; then
        print_status "Build successful"
    else
        print_error "cursed-zig binary not found"
        exit 1
    fi
}

# Run basic memory test
run_basic_memory_test() {
    print_status "Running basic memory management test..."
    
    if ./zig-out/bin/cursed-zig memory_management_comprehensive_test.csd; then
        print_status "Basic memory test passed"
    else
        print_error "Basic memory test failed"
        return 1
    fi
}

# Run valgrind memory check
run_valgrind_check() {
    print_status "Running Valgrind memory leak detection..."
    
    echo "Valgrind Options:"
    echo "  --leak-check=full        : Detailed leak detection"
    echo "  --show-leak-kinds=all    : Show all types of leaks"
    echo "  --track-origins=yes      : Track origin of uninitialized values"
    echo "  --error-exitcode=1       : Exit with code 1 on errors"
    echo "  --suppressions=cursed.supp : Use CURSED-specific suppressions"
    echo ""
    
    # Create valgrind suppressions file for known issues
    create_valgrind_suppressions
    
    # Run valgrind with comprehensive options
    valgrind \
        --tool=memcheck \
        --leak-check=full \
        --show-leak-kinds=all \
        --track-origins=yes \
        --verbose \
        --error-exitcode=1 \
        --suppressions=cursed.supp \
        --log-file=valgrind_memory_test.log \
        ./zig-out/bin/cursed-zig memory_management_comprehensive_test.csd
    
    local valgrind_exit_code=$?
    
    # Analyze valgrind output
    analyze_valgrind_results "$valgrind_exit_code"
}

# Create valgrind suppressions for known issues
create_valgrind_suppressions() {
    print_status "Creating valgrind suppressions file..."
    
    cat > cursed.supp << 'EOF'
# Valgrind suppressions for CURSED language
{
   cursed_bootstrap_static_allocation
   Memcheck:Leak
   match-leak-kinds: reachable
   ...
   fun:system_memory_allocate
}

{
   cursed_global_memory_pool
   Memcheck:Leak
   match-leak-kinds: reachable
   ...
   fun:memory_pool_new
}

{
   cursed_atomic_operations
   Memcheck:Helgrind:Race
   ...
   fun:atomic_*
}

{
   cursed_gc_heap_regions
   Memcheck:Leak
   match-leak-kinds: reachable
   ...
   fun:gc_init_heap_regions
}
EOF
}

# Analyze valgrind results
analyze_valgrind_results() {
    local exit_code=$1
    
    print_status "Analyzing valgrind results..."
    
    if [[ $exit_code -eq 0 ]]; then
        print_status "✅ Valgrind found no memory errors!"
    else
        print_error "❌ Valgrind detected memory errors (exit code: $exit_code)"
    fi
    
    # Parse valgrind log
    if [[ -f "valgrind_memory_test.log" ]]; then
        echo ""
        echo "Valgrind Summary:"
        echo "=================="
        
        # Extract key statistics
        local heap_summary=$(grep -A 10 "HEAP SUMMARY" valgrind_memory_test.log | head -10)
        local leak_summary=$(grep -A 10 "LEAK SUMMARY" valgrind_memory_test.log | head -10)
        local error_summary=$(grep -A 5 "ERROR SUMMARY" valgrind_memory_test.log | head -5)
        
        if [[ -n "$heap_summary" ]]; then
            echo "Heap Usage:"
            echo "$heap_summary"
            echo ""
        fi
        
        if [[ -n "$leak_summary" ]]; then
            echo "Memory Leaks:"
            echo "$leak_summary"
            echo ""
        fi
        
        if [[ -n "$error_summary" ]]; then
            echo "Error Summary:"
            echo "$error_summary"
            echo ""
        fi
        
        # Check for specific error types
        local errors_count=$(grep "ERROR SUMMARY:" valgrind_memory_test.log | awk '{print $4}')
        local leaks_count=$(grep "definitely lost:" valgrind_memory_test.log | awk '{print $4}' | sed 's/,//')
        
        echo "Detailed Analysis:"
        echo "  Total errors: ${errors_count:-0}"
        echo "  Definitely lost: ${leaks_count:-0} bytes"
        
        # Check for critical issues
        if grep -q "Invalid read\|Invalid write" valgrind_memory_test.log; then
            print_error "⚠️  Memory access violations detected!"
        fi
        
        if grep -q "Conditional jump or move depends on uninitialised value" valgrind_memory_test.log; then
            print_warning "⚠️  Uninitialized memory usage detected!"
        fi
        
        if grep -q "definitely lost:" valgrind_memory_test.log && [[ "${leaks_count:-0}" -gt "0" ]]; then
            print_error "⚠️  Definite memory leaks detected!"
        else
            print_status "✅ No definite memory leaks found"
        fi
    else
        print_warning "Valgrind log file not found"
    fi
    
    return $exit_code
}

# Run memory stress test
run_memory_stress_test() {
    print_status "Running memory stress test..."
    
    # Create a stress test program
    cat > memory_stress_test.csd << 'EOF'
yeet "memory/mod"
yeet "memory/profiler"

slay main() {
    vibez.spill("Memory Stress Test Starting...")
    
    # Enable profiler
    profiler.profiler_enable(based, cap, 50000)  # Track without stack traces for speed
    
    # Stress test 1: Rapid allocation/deallocation
    bestie i := 0; i < 10000; i = i + 1 {
        sus size normie = (i % 1000) + 16
        sus ptr *void = mod.memory_alloc(size)
        
        yo ptr != cringe {
            profiler.profiler_track_allocation(ptr, size)
            mod.memory_set(ptr, (i % 256).(byte), size)
            mod.memory_free(ptr)
            profiler.profiler_track_deallocation(ptr)
        }
    }
    
    # Stress test 2: Growing allocations
    sus growing_ptrs []*void = []
    bestie i := 0; i < 100; i = i + 1 {
        sus size normie = i * 1024  # 0KB to 99KB
        sus ptr *void = mod.memory_alloc(size)
        
        yo ptr != cringe {
            growing_ptrs.push(ptr)
            profiler.profiler_track_allocation(ptr, size)
        }
    }
    
    # Clean up
    bestie i := 0; i < growing_ptrs.len(); i = i + 1 {
        mod.memory_free(growing_ptrs[i])
        profiler.profiler_track_deallocation(growing_ptrs[i])
    }
    
    # Final leak check
    profiler.profiler_detect_leaks()
    profiler.profiler_generate_report()
    
    vibez.spill("Memory Stress Test Complete")
}
EOF
    
    print_status "Running stress test with valgrind..."
    
    if valgrind \
        --tool=memcheck \
        --leak-check=yes \
        --error-exitcode=1 \
        --log-file=valgrind_stress_test.log \
        ./zig-out/bin/cursed-zig memory_stress_test.csd; then
        print_status "✅ Memory stress test passed"
    else
        print_error "❌ Memory stress test failed"
        if [[ -f "valgrind_stress_test.log" ]]; then
            echo "Stress test errors:"
            tail -20 valgrind_stress_test.log
        fi
        return 1
    fi
}

# Generate final report
generate_final_report() {
    print_status "Generating final memory validation report..."
    
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local report_file="memory_validation_report_$(date '+%Y%m%d_%H%M%S').txt"
    
    cat > "$report_file" << EOF
CURSED Memory Management Validation Report
==========================================
Generated: $timestamp

BUILD STATUS:
$(if [[ -f "zig-out/bin/cursed-zig" ]]; then echo "✅ Build successful"; else echo "❌ Build failed"; fi)

BASIC TESTS:
$(if [[ $basic_test_result -eq 0 ]]; then echo "✅ Basic memory operations passed"; else echo "❌ Basic memory operations failed"; fi)

VALGRIND ANALYSIS:
$(if [[ $valgrind_result -eq 0 ]]; then echo "✅ No memory errors detected"; else echo "❌ Memory errors detected"; fi)

STRESS TESTS:
$(if [[ $stress_test_result -eq 0 ]]; then echo "✅ Stress tests passed"; else echo "❌ Stress tests failed"; fi)

FILES GENERATED:
- build_output.log: Build process log
- valgrind_memory_test.log: Detailed valgrind output
- valgrind_stress_test.log: Stress test valgrind output
- cursed.supp: Valgrind suppressions file
- memory_stress_test.csd: Generated stress test

RECOMMENDATIONS:
$(if [[ $valgrind_result -eq 0 && $basic_test_result -eq 0 && $stress_test_result -eq 0 ]]; then
    echo "✅ Memory management implementation is production-ready"
    echo "✅ All tests passed with zero memory leaks"
    echo "✅ Ready for P1 critical applications"
else
    echo "⚠️  Issues detected - review logs before production deployment"
    echo "⚠️  Address memory leaks and errors before P1 applications"
fi)

EOF
    
    print_status "Report generated: $report_file"
    cat "$report_file"
}

# Main execution
main() {
    local basic_test_result=1
    local valgrind_result=1  
    local stress_test_result=1
    
    print_status "Starting CURSED Memory Management validation..."
    
    # Step 1: Check dependencies
    check_valgrind
    
    # Step 2: Build
    build_memory_test
    
    # Step 3: Basic tests
    if run_basic_memory_test; then
        basic_test_result=0
    fi
    
    # Step 4: Valgrind analysis
    if run_valgrind_check; then
        valgrind_result=0
    fi
    
    # Step 5: Stress tests
    if run_memory_stress_test; then
        stress_test_result=0
    fi
    
    # Step 6: Generate report
    generate_final_report
    
    # Final status
    if [[ $basic_test_result -eq 0 && $valgrind_result -eq 0 && $stress_test_result -eq 0 ]]; then
        print_status "🎉 All memory management tests PASSED!"
        print_status "🚀 Implementation is ready for P1 critical applications"
        exit 0
    else
        print_error "❌ Some tests FAILED - review issues before production use"
        exit 1
    fi
}

# Run main function
main "$@"
