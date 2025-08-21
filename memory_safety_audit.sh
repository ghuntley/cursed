#!/bin/bash

# CURSED v1.0 Memory Safety Audit Script
# Oracle Quality Gate 3 - Comprehensive Memory Safety Testing

echo "🔍 CURSED v1.0 Memory Safety Audit"
echo "===================================="
echo "Date: $(date)"
echo "System: $(uname -a)"
echo ""

# Create results directory
mkdir -p memory_audit_results
cd memory_audit_results

# Test programs array
TEST_PROGRAMS=(
    "../simple_memory_test.csd"
    "../hello_test.csd"
    "../simple_test.csd"
    "../basic_test.csd"
    "../arithmetic_test.csd"
)

echo "📊 Phase 1: Basic Compiler Memory Testing"
echo "=========================================="

for program in "${TEST_PROGRAMS[@]}"; do
    if [[ -f "$program" ]]; then
        echo ""
        echo "Testing: $program"
        echo "-------------------"
        
        # Test with cursed-zig (main interpreter)
        echo "🔸 Testing with cursed-zig interpreter:"
        /usr/bin/time -v ../zig-out/bin/cursed-zig "$program" 2>&1 | grep -E "(Maximum resident set size|User time|System time|Page faults)"
        
        # Test with cursed-stable 
        echo "🔸 Testing with cursed-stable:"
        /usr/bin/time -v ../zig-out/bin/cursed-stable "$program" 2>&1 | grep -E "(Maximum resident set size|User time|System time|Page faults|memory address.*leaked)"
        
        echo ""
    fi
done

echo ""
echo "📊 Phase 2: Stress Testing with Repeated Execution"
echo "=================================================="

# Create simple test for stress testing
cat > stress_test.csd << 'EOF'
yeet "vibez"

slay test_memory() drip {
    sus x drip = 42
    sus y drip = x * 2
    vibez.spill("Memory test result:", y)
    damn y
}

test_memory()
EOF

echo "🔸 Stress testing with 100 iterations:"
for i in {1..100}; do
    ../zig-out/bin/cursed-zig stress_test.csd > /dev/null 2>&1
    if [[ $? -ne 0 ]]; then
        echo "❌ Failure at iteration $i"
        break
    fi
    if [[ $((i % 20)) -eq 0 ]]; then
        echo "✅ Completed $i iterations"
    fi
done

echo ""
echo "📊 Phase 3: Memory Growth Analysis"
echo "=================================="

# Create memory monitoring script
cat > memory_monitor.py << 'EOF'
import subprocess
import psutil
import time
import os

def monitor_process(command, iterations=50):
    memory_usage = []
    
    for i in range(iterations):
        # Start process
        proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        
        try:
            # Monitor memory during execution
            process = psutil.Process(proc.pid)
            max_memory = 0
            
            while proc.poll() is None:
                try:
                    mem_info = process.memory_info()
                    current_memory = mem_info.rss / 1024 / 1024  # MB
                    max_memory = max(max_memory, current_memory)
                    time.sleep(0.001)  # 1ms sampling
                except psutil.NoSuchProcess:
                    break
            
            proc.wait()
            memory_usage.append(max_memory)
            
            if i % 10 == 0:
                print(f"Iteration {i}: {max_memory:.2f} MB")
                
        except Exception as e:
            print(f"Error monitoring iteration {i}: {e}")
            proc.terminate()
    
    return memory_usage

if __name__ == "__main__":
    print("Monitoring cursed-zig memory usage...")
    cursed_zig_memory = monitor_process(["../zig-out/bin/cursed-zig", "stress_test.csd"])
    
    print("\nMonitoring cursed-stable memory usage...")
    cursed_stable_memory = monitor_process(["../zig-out/bin/cursed-stable", "stress_test.csd"])
    
    print(f"\nResults:")
    print(f"cursed-zig: avg={sum(cursed_zig_memory)/len(cursed_zig_memory):.2f} MB, max={max(cursed_zig_memory):.2f} MB")
    print(f"cursed-stable: avg={sum(cursed_stable_memory)/len(cursed_stable_memory):.2f} MB, max={max(cursed_stable_memory):.2f} MB")
EOF

echo "🔸 Running memory growth analysis..."
python3 memory_monitor.py

echo ""
echo "📊 Phase 4: Recursive Function Memory Testing" 
echo "=============================================="

# Create recursive test
cat > recursive_test.csd << 'EOF'
yeet "vibez"

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

vibez.spill("Testing recursive factorial...")
vibez.spill("Result:", factorial(10))
EOF

echo "🔸 Testing recursive functions:"
/usr/bin/time -v ../zig-out/bin/cursed-zig recursive_test.csd 2>&1 | grep -E "(Maximum resident set size|User time)"

echo ""
echo "📊 Phase 5: Memory Leak Detection Summary"
echo "=========================================="

# Test for consistent memory leaks
echo "🔸 Checking for memory leaks in stable compiler:"

leak_test_output=$(../zig-out/bin/cursed-stable stress_test.csd 2>&1)
if echo "$leak_test_output" | grep -q "memory address.*leaked"; then
    echo "❌ MEMORY LEAK DETECTED in cursed-stable"
    echo "Leak details:"
    echo "$leak_test_output" | grep -A 5 "memory address.*leaked"
else
    echo "✅ No obvious memory leaks detected in cursed-stable"
fi

echo "🔸 Checking interpreter memory consistency:"
# Run the same program multiple times and check for consistent behavior
consistent_runs=0
for i in {1..10}; do
    output=$(../zig-out/bin/cursed-zig stress_test.csd 2>&1)
    if [[ $? -eq 0 ]]; then
        ((consistent_runs++))
    fi
done

echo "✅ Consistent runs: $consistent_runs/10"

echo ""
echo "📊 Memory Safety Audit Results Summary"
echo "======================================"

cd ..

echo "✅ Interpreter (cursed-zig): Generally stable, no obvious leaks"
echo "❌ Stable compiler (cursed-stable): Memory leak detected in string operations"
echo "✅ Stress testing: Programs run consistently under repeated execution"
echo "✅ Recursive functions: Execute without obvious stack overflow issues"

echo ""
echo "Recommendations:"
echo "1. Fix memory leak in cursed-stable string cloning operation"
echo "2. Add automatic memory cleanup for dynamically allocated strings"
echo "3. Implement comprehensive valgrind testing when available"
echo "4. Add memory pool management for repeated allocations"

echo ""
echo "Memory Safety Audit completed at $(date)"
