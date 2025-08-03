#!/bin/bash
# Comprehensive Integration Tests
# Tests complete programs with complex interactions between language features

set -e

echo "🎯 Comprehensive Integration Tests"
echo "================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test counters
INTEGRATION_TESTS=0
INTEGRATION_PASSED=0
INTEGRATION_FAILED=0

# Helper function for integration testing
run_integration_test() {
    local test_name="$1"
    local test_file="$2"
    local expected_output="$3"
    local should_compile="${4:-true}"
    
    INTEGRATION_TESTS=$((INTEGRATION_TESTS + 1))
    echo -e "${BLUE}🔍 Integration Test: $test_name${NC}"
    
    # Test interpretation mode
    echo "  📝 Testing interpretation mode..."
    if timeout 60 cargo run --bin cursed "$test_file" > /tmp/integration_interp.txt 2>&1; then
        if [[ -n "$expected_output" ]]; then
            if grep -q "$expected_output" /tmp/integration_interp.txt; then
                echo -e "  ${GREEN}✅ Interpretation passed${NC}"
            else
                echo -e "  ${RED}❌ Interpretation output mismatch${NC}"
                echo "    Expected: $expected_output"
                echo "    Got: $(cat /tmp/integration_interp.txt | tail -n 3)"
                INTEGRATION_FAILED=$((INTEGRATION_FAILED + 1))
                return 1
            fi
        else
            echo -e "  ${GREEN}✅ Interpretation completed${NC}"
        fi
    else
        echo -e "  ${RED}❌ Interpretation failed${NC}"
        cat /tmp/integration_interp.txt | head -n 10
        INTEGRATION_FAILED=$((INTEGRATION_FAILED + 1))
        return 1
    fi
    
    # Test compilation mode (if expected to compile)
    if [[ "$should_compile" == "true" ]]; then
        echo "  🔧 Testing compilation mode..."
        local executable_name=$(basename "$test_file" .csd)
        
        if timeout 120 cargo run --bin cursed -- compile "$test_file" > /tmp/integration_compile.txt 2>&1; then
            echo -e "  ${GREEN}✅ Compilation passed${NC}"
            
            # Test executable execution
            if [[ -f "./$executable_name" ]]; then
                echo "  🚀 Testing executable execution..."
                if timeout 60 "./$executable_name" > /tmp/integration_exec.txt 2>&1; then
                    if [[ -n "$expected_output" ]]; then
                        if grep -q "$expected_output" /tmp/integration_exec.txt; then
                            echo -e "  ${GREEN}✅ Executable execution passed${NC}"
                        else
                            echo -e "  ${RED}❌ Executable output mismatch${NC}"
                            echo "    Expected: $expected_output"
                            echo "    Got: $(cat /tmp/integration_exec.txt | tail -n 3)"
                            INTEGRATION_FAILED=$((INTEGRATION_FAILED + 1))
                            return 1
                        fi
                    else
                        echo -e "  ${GREEN}✅ Executable execution completed${NC}"
                    fi
                    rm -f "./$executable_name"
                else
                    echo -e "  ${RED}❌ Executable execution failed${NC}"
                    cat /tmp/integration_exec.txt | head -n 10
                    INTEGRATION_FAILED=$((INTEGRATION_FAILED + 1))
                    return 1
                fi
            else
                echo -e "  ${RED}❌ Executable not generated${NC}"
                INTEGRATION_FAILED=$((INTEGRATION_FAILED + 1))
                return 1
            fi
        else
            echo -e "  ${RED}❌ Compilation failed${NC}"
            cat /tmp/integration_compile.txt | head -n 10
            INTEGRATION_FAILED=$((INTEGRATION_FAILED + 1))
            return 1
        fi
    fi
    
    INTEGRATION_PASSED=$((INTEGRATION_PASSED + 1))
    echo -e "${GREEN}✅ $test_name: PASSED${NC}"
    echo
}

echo "🔨 Building CURSED compiler..."
if ! cargo build --release > /tmp/integration_build.txt 2>&1; then
    echo -e "${RED}❌ Build failed!${NC}"
    cat /tmp/integration_build.txt
    exit 1
fi
echo -e "${GREEN}✅ Build successful${NC}"
echo

# Create test directory
mkdir -p /tmp/cursed_integration_tests
cd /tmp/cursed_integration_tests

echo "📋 Creating comprehensive integration test programs..."
echo "===================================================="

# Integration Test 1: Complex Data Processing Pipeline
cat > data_processing_pipeline.csd << 'EOF'
yeet "testz"

fr fr Complex data processing with structs, functions, and collections
squad Person {
    spill name tea
    spill age drip
    spill salary meal
}

slay create_person(name tea, age drip, salary meal) Person {
    damn Person{name: name, age: age, salary: salary}
}

slay calculate_average_salary(people []Person) meal {
    sus total meal = 0.0
    bestie person in people {
        total = total + person.salary
    }
    damn total / people.len().(meal)
}

slay filter_by_age(people []Person, min_age drip) []Person {
    sus filtered []Person = []
    bestie person in people {
        lowkey person.age >= min_age {
            filtered.push(person)
        }
    }
    damn filtered
}

test_start("Data Processing Pipeline")

fr fr Create test data
sus employees []Person = [
    create_person("Alice", 25, 50000.0),
    create_person("Bob", 30, 60000.0),
    create_person("Charlie", 35, 70000.0),
    create_person("Diana", 28, 55000.0)
]

fr fr Test filtering
sus senior_employees []Person = filter_by_age(employees, 30)
assert_eq_int(senior_employees.len(), 2)

fr fr Test salary calculation
sus avg_salary meal = calculate_average_salary(employees)
assert_true(avg_salary > 55000.0 && avg_salary < 60000.0)

print_test_summary()
vibez.spill("Data processing pipeline test completed!")
EOF

# Integration Test 2: Error Handling with Resource Management
cat > error_handling_resources.csd << 'EOF'
yeet "testz"

fr fr Error handling with resource management
squad FileHandler {
    spill filename tea
    spill is_open lit
}

slay FileHandler.open() {
    lowkey filename == "invalid.txt" {
        yikes "File not found"
    }
    is_open = based
}

slay FileHandler.close() {
    is_open = cringe
}

slay FileHandler.read() tea {
    lowkey !is_open {
        yikes "File not open"
    }
    damn "file content"
}

slay process_file(filename tea) tea {
    sus handler FileHandler = FileHandler{filename: filename, is_open: cringe}
    
    fam {
        handler.open()
        sus content tea = handler.read()
        handler.close()
        damn content
    } shook error {
        handler.close()  fr fr Ensure cleanup
        yikes "Failed to process file: " + error
    }
}

test_start("Error Handling with Resources")

fr fr Test successful file processing
sus result tea = ""
fam {
    result = process_file("valid.txt")
    assert_eq_string(result, "file content")
} shook error {
    assert_true(cringe)  fr fr Should not reach here
}

fr fr Test error handling
fam {
    result = process_file("invalid.txt")
    assert_true(cringe)  fr fr Should not reach here
} shook error {
    assert_true(based)  fr fr Should catch error
}

print_test_summary()
vibez.spill("Error handling test completed!")
EOF

# Integration Test 3: Concurrent Producer-Consumer
cat > concurrent_producer_consumer.csd << 'EOF'
yeet "testz"

fr fr Concurrent producer-consumer pattern
slay producer(ch channel<drip>, count drip) {
    bestie i := 0; i < count; i = i + 1 {
        dm_send(ch, i)
        fr fr Small delay to simulate work
    }
    dm_close(ch)
}

slay consumer(ch channel<drip>) []drip {
    sus results []drip = []
    bestie {
        select {
            case value, ok := dm_recv(ch):
                lowkey !ok {
                    damn results
                }
                results.push(value)
            default:
                fr fr No value available, continue
        }
    }
}

test_start("Concurrent Producer-Consumer")

sus ch = make_channel<drip>()
sus item_count drip = 10

fr fr Start producer goroutine
stan {
    producer(ch, item_count)
}

fr fr Consume items
sus consumed []drip = consumer(ch)

fr fr Verify all items were consumed
assert_eq_int(consumed.len(), item_count)

fr fr Verify items are in order
bestie i := 0; i < consumed.len(); i = i + 1 {
    assert_eq_int(consumed[i], i)
}

print_test_summary()
vibez.spill("Concurrent producer-consumer test completed!")
EOF

# Integration Test 4: Interface Polymorphism
cat > interface_polymorphism.csd << 'EOF'
yeet "testz"

fr fr Interface polymorphism with multiple implementations
collab Shape {
    slay area() meal
    slay perimeter() meal
}

squad Rectangle {
    spill width meal
    spill height meal
}

squad Circle {
    spill radius meal
}

flex Rectangle => Shape {
    slay area() meal {
        damn width * height
    }
    
    slay perimeter() meal {
        damn 2.0 * (width + height)
    }
}

flex Circle => Shape {
    slay area() meal {
        damn 3.14159 * radius * radius
    }
    
    slay perimeter() meal {
        damn 2.0 * 3.14159 * radius
    }
}

slay calculate_total_area(shapes []Shape) meal {
    sus total meal = 0.0
    bestie shape in shapes {
        total = total + shape.area()
    }
    damn total
}

test_start("Interface Polymorphism")

sus shapes []Shape = [
    Rectangle{width: 5.0, height: 3.0},
    Circle{radius: 2.0},
    Rectangle{width: 4.0, height: 4.0}
]

sus total_area meal = calculate_total_area(shapes)
assert_true(total_area > 40.0 && total_area < 50.0)

print_test_summary()
vibez.spill("Interface polymorphism test completed!")
EOF

# Integration Test 5: Generic Container with Operations
cat > generic_container_ops.csd << 'EOF'
yeet "testz"

fr fr Generic container with type-safe operations
squad Container<T> {
    spill items []T
}

slay Container<T>.add(item T) {
    items.push(item)
}

slay Container<T>.get(index drip) T {
    lowkey index < 0 || index >= items.len() {
        yikes "Index out of bounds"
    }
    damn items[index]
}

slay Container<T>.size() drip {
    damn items.len()
}

slay Container<T>.map<U>(func slay(T) U) Container<U> {
    sus result Container<U> = Container<U>{items: []}
    bestie item in items {
        result.add(func(item))
    }
    damn result
}

slay double_int(x drip) drip {
    damn x * 2
}

slay string_length(s tea) drip {
    damn s.len()
}

test_start("Generic Container Operations")

fr fr Test integer container
sus int_container Container<drip> = Container<drip>{items: []}
int_container.add(1)
int_container.add(2)
int_container.add(3)

assert_eq_int(int_container.size(), 3)
assert_eq_int(int_container.get(1), 2)

fr fr Test map operation
sus doubled Container<drip> = int_container.map<drip>(double_int)
assert_eq_int(doubled.get(0), 2)
assert_eq_int(doubled.get(1), 4)
assert_eq_int(doubled.get(2), 6)

fr fr Test string container
sus string_container Container<tea> = Container<tea>{items: []}
string_container.add("hello")
string_container.add("world")

sus lengths Container<drip> = string_container.map<drip>(string_length)
assert_eq_int(lengths.get(0), 5)
assert_eq_int(lengths.get(1), 5)

print_test_summary()
vibez.spill("Generic container operations test completed!")
EOF

# Integration Test 6: Pattern Matching with Guards
cat > pattern_matching_guards.csd << 'EOF'
yeet "testz"

fr fr Pattern matching with complex guards
squad Result<T> {
    spill value T
    spill success lit
}

slay classify_result<T>(result Result<T>) tea {
    damn match result {
        Result{success: based, value: v} if v > 0 => "positive success",
        Result{success: based, value: 0} => "zero success",
        Result{success: based, value: _} => "negative success",
        Result{success: cringe, _} => "failure"
    }
}

slay process_number(x drip) Result<drip> {
    lowkey x < 0 {
        damn Result{value: x, success: cringe}
    }
    damn Result{value: x * 2, success: based}
}

test_start("Pattern Matching with Guards")

sus positive_result Result<drip> = process_number(5)
sus classification tea = classify_result<drip>(positive_result)
assert_eq_string(classification, "positive success")

sus zero_result Result<drip> = process_number(0)
sus zero_classification tea = classify_result<drip>(zero_result)
assert_eq_string(zero_classification, "zero success")

sus negative_input Result<drip> = process_number(-3)
sus negative_classification tea = classify_result<drip>(negative_input)
assert_eq_string(negative_classification, "failure")

print_test_summary()
vibez.spill("Pattern matching with guards test completed!")
EOF

echo "🚀 Running Comprehensive Integration Tests..."
echo "============================================="

# Set working directory back to CURSED project
cd /home/ghuntley/code/cursed

# Run all integration tests
run_integration_test "Data Processing Pipeline" "/tmp/cursed_integration_tests/data_processing_pipeline.csd" "Data processing pipeline test completed!"
run_integration_test "Error Handling with Resources" "/tmp/cursed_integration_tests/error_handling_resources.csd" "Error handling test completed!"
run_integration_test "Concurrent Producer-Consumer" "/tmp/cursed_integration_tests/concurrent_producer_consumer.csd" "Concurrent producer-consumer test completed!"
run_integration_test "Interface Polymorphism" "/tmp/cursed_integration_tests/interface_polymorphism.csd" "Interface polymorphism test completed!"
run_integration_test "Generic Container Operations" "/tmp/cursed_integration_tests/generic_container_ops.csd" "Generic container operations test completed!"
run_integration_test "Pattern Matching with Guards" "/tmp/cursed_integration_tests/pattern_matching_guards.csd" "Pattern matching with guards test completed!"

echo "📊 Integration Test Results"
echo "==========================="
echo -e "Total integration tests: ${BLUE}$INTEGRATION_TESTS${NC}"
echo -e "Passed: ${GREEN}$INTEGRATION_PASSED${NC}"
echo -e "Failed: ${RED}$INTEGRATION_FAILED${NC}"

if [[ $INTEGRATION_FAILED -eq 0 ]]; then
    echo -e "${GREEN}🎉 All integration tests passed!${NC}"
    echo -e "${GREEN}✅ CURSED language features work correctly together${NC}"
    exit 0
else
    echo -e "${RED}❌ Some integration tests failed. Language feature interactions need fixes.${NC}"
    exit 1
fi

# Cleanup
rm -rf /tmp/cursed_integration_tests
rm -f /tmp/integration_*.txt
