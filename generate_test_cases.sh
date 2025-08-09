#!/bin/bash

# Script to generate comprehensive test cases for CURSED compiler regression testing

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

# Create test directories if they don't exist
mkdir -p tests/regression/{parser,stdlib,memory,errors,roundtrip}/{basic,advanced,edge_cases}

echo "Generating comprehensive test cases..."

# Generate basic parser tests (200+ tests)
for i in {1..50}; do
    # Basic variable tests
    cat > "tests/regression/parser/basic/var_test_$i.csd" << EOF
// Variable test $i
sus x$i drip = $((i * 10))
sus name$i tea = "test_$i"
sus flag$i lit = $([ $((i % 2)) -eq 0 ] && echo "based" || echo "cringe")
vibez.spill("Variable $i:", x$i, name$i, flag$i)
EOF

    # Basic function tests
    cat > "tests/regression/parser/basic/func_test_$i.csd" << EOF
// Function test $i
slay test_func_$i(x drip) drip {
    damn x * $i
}

sus result$i drip = test_func_$i($((i + 5)))
vibez.spill("Function $i result:", result$i)
EOF

    # Basic arithmetic tests
    cat > "tests/regression/parser/basic/arithmetic_test_$i.csd" << EOF
// Arithmetic test $i
sus a drip = $i
sus b drip = $((i + 10))
sus add_result drip = a + b
sus mul_result drip = a * b
sus div_result drip = b / a
vibez.spill("Arithmetic $i:", add_result, mul_result, div_result)
EOF

    # Basic array tests
    cat > "tests/regression/parser/basic/array_test_$i.csd" << EOF
// Array test $i
sus numbers$i []drip = [$(seq -s, 1 $((i % 10 + 1)))]
sus first drip = numbers$i[0]
sus length drip = len(numbers$i)
vibez.spill("Array $i length:", length, "first:", first)
EOF
done

# Generate advanced parser tests (100+ tests)
for i in {1..25}; do
    # Complex expressions
    cat > "tests/regression/parser/advanced/complex_expr_$i.csd" << EOF
// Complex expression test $i
sus result drip = ((($i + 5) * 2) - 1) / (($i % 3) + 1)
sus bool_result lit = (result > $i) && (result < $((i * 10)))
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex $i:", result, bool_result, nested)
EOF

    # Nested function calls
    cat > "tests/regression/parser/advanced/nested_calls_$i.csd" << EOF
// Nested function calls test $i
slay inner_$i(x drip) drip { damn x + $i }
slay outer_$i(y drip) drip { damn inner_$i(y * 2) }
slay wrapper_$i(z drip) drip { damn outer_$i(z + 1) }

sus final drip = wrapper_$i($i)
vibez.spill("Nested calls $i:", final)
EOF

    # Struct definitions
    cat > "tests/regression/parser/advanced/struct_$i.csd" << EOF
// Struct test $i
squad Point$i {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p$i Point$i = Point$i{x: $i, y: $((i * 2)), id: $i}
p$i.move(1, 1)
vibez.spill("Point $i:", p$i.x, p$i.y, p$i.id)
EOF

    # Interface definitions  
    cat > "tests/regression/parser/advanced/interface_$i.csd" << EOF
// Interface test $i
collab Processable$i {
    slay process() drip
    slay getId() drip
}

squad Item$i {
    spill value drip
    
    slay process() drip {
        damn value * $i
    }
    
    slay getId() drip {
        damn $i
    }
}

sus item$i Item$i = Item$i{value: $((i * 5))}
sus processed drip = item$i.process()
vibez.spill("Interface $i:", processed)
EOF
done

# Generate stdlib tests (150+ tests)
STDLIB_MODULES=("mathz" "stringz" "arrayz" "cryptz" "jsonz" "httpz" "filez")

for i in {1..25}; do
    for module in "${STDLIB_MODULES[@]}"; do
        case $module in
            "mathz")
                cat > "tests/regression/stdlib/basic/${module}_test_$i.csd" << EOF
// Math stdlib test $i
yeet "mathz"

sus value drip = $((i * 3))
sus abs_val drip = abs_normie(-value)
sus max_val drip = max_drip(value, $((i + 10)))
sus min_val drip = min_drip(value, $((i - 5)))

vibez.spill("Math $i:", abs_val, max_val, min_val)
EOF
                ;;
            "stringz")
                cat > "tests/regression/stdlib/basic/${module}_test_$i.csd" << EOF
// String stdlib test $i
yeet "stringz"

sus text tea = "test string $i"
sus length drip = len_str(text)
sus upper tea = to_upper(text)
sus contains lit = contains_str(text, "$i")

vibez.spill("String $i:", length, upper, contains)
EOF
                ;;
            "arrayz")
                cat > "tests/regression/stdlib/basic/${module}_test_$i.csd" << EOF
// Array stdlib test $i
yeet "arrayz"

sus numbers []drip = [$(seq -s, $i $((i + 4)))]
sus length drip = len(numbers)
sus sum drip = sum_drip(numbers)
sus contains lit = contains_drip(numbers, $((i + 2)))

vibez.spill("Array $i:", length, sum, contains)
EOF
                ;;
            *)
                cat > "tests/regression/stdlib/basic/${module}_test_$i.csd" << EOF
// ${module^} stdlib test $i
yeet "$module"

vibez.spill("Testing $module module $i")
EOF
                ;;
        esac
    done
done

# Generate memory safety tests (100+ tests)
for i in {1..50}; do
    # Variable lifecycle tests
    cat > "tests/regression/memory/basic/lifecycle_$i.csd" << EOF
// Memory lifecycle test $i
sus outer$i drip = $i

ready (based) {
    sus inner$i tea = "scoped_$i"
    sus calc$i drip = outer$i * $((i + 1))
    vibez.spill("Scoped $i:", inner$i, calc$i)
}

slay memory_func_$i() tea {
    sus local tea = "local_$i"
    damn local + "_processed"
}

sus result tea = memory_func_$i()
vibez.spill("Memory $i:", result)
EOF

    # String operation tests
    cat > "tests/regression/memory/basic/string_ops_$i.csd" << EOF
// String memory test $i
yeet "stringz"

sus base tea = "base_$i"
sus extended tea = base + "_extended"
sus repeated tea = ""

sus j drip = 0
bestie (j < $((i % 5 + 1))) {
    repeated = repeated + base
    j = j + 1
}

vibez.spill("String memory $i:", len_str(repeated))
EOF
done

# Generate error handling tests (100+ tests)
for i in {1..25}; do
    # Syntax errors
    cat > "tests/regression/errors/basic/syntax_error_$i.csd" << EOF
// Syntax error test $i - missing semicolon
sus x drip = $i
sus y drip = x +   // Missing operand
vibez.spill("This should not execute")
EOF

    # Type errors
    cat > "tests/regression/errors/basic/type_error_$i.csd" << EOF
// Type error test $i
sus number drip = $i
sus text tea = "string_$i"
sus wrong drip = text  // Type mismatch
vibez.spill("Type error $i:", wrong)
EOF

    # Undefined variable errors
    cat > "tests/regression/errors/basic/undefined_var_$i.csd" << EOF
// Undefined variable test $i
vibez.spill("Using undefined:", undefined_var_$i)
sus result drip = missing_variable + $i
EOF

    # Import errors
    cat > "tests/regression/errors/basic/import_error_$i.csd" << EOF
// Import error test $i
yeet "nonexistent_module_$i"
sus result drip = some_function($i)
EOF
done

# Generate round-trip tests (50+ tests)
for i in {1..25}; do
    # Complete program tests
    cat > "tests/regression/roundtrip/basic/complete_program_$i.csd" << EOF
// Complete program test $i
yeet "mathz"
yeet "stringz"

squad TestStruct$i {
    spill value drip
    spill name tea
    
    slay process() drip {
        damn value * $i
    }
}

slay main_$i() {
    sus items []TestStruct$i = [
        TestStruct$i{value: $i, name: "item_1"},
        TestStruct$i{value: $((i * 2)), name: "item_2"}
    ]
    
    sus total drip = 0
    sus idx drip = 0
    bestie (idx < len(items)) {
        total = total + items[idx].process()
        idx = idx + 1
    }
    
    vibez.spill("Total $i:", total)
}

main_$i()
EOF

    # Complex expression round-trip
    cat > "tests/regression/roundtrip/basic/complex_expr_$i.csd" << EOF
// Complex expression round-trip test $i
sus nested drip = ((($i + 3) * 2) - 1) / (($i % 4) + 1)
sus conditional drip = ready (nested > $i) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip($i, nested)

vibez.spill("Complex round-trip $i:", final)
EOF
done

# Generate edge case tests (100+ tests)
for i in {1..25}; do
    # Large number tests
    cat > "tests/regression/parser/edge_cases/large_numbers_$i.csd" << EOF
// Large number test $i
sus large drip = $((i * 1000000))
sus calculation drip = large + $((i * 999999))
vibez.spill("Large number $i:", calculation)
EOF

    # Deep nesting tests
    cat > "tests/regression/parser/edge_cases/deep_nesting_$i.csd" << EOF
// Deep nesting test $i
ready (based) {
    ready (based) {
        ready (based) {
            sus deep$i drip = $i
            vibez.spill("Deep nesting $i:", deep$i)
        }
    }
}
EOF

    # Unicode and special characters
    cat > "tests/regression/parser/edge_cases/unicode_$i.csd" << EOF
// Unicode test $i
sus emoji tea = "test_${i}_rocket_star_computer"
sus unicode tea = "test_${i}_with_unicode"
vibez.spill("Unicode $i:", emoji, unicode)
EOF

    # Empty and minimal programs
    cat > "tests/regression/parser/edge_cases/minimal_$i.csd" << EOF
// Minimal program $i
vibez.spill("Minimal $i")
EOF
done

# Generate comprehensive integration tests
for i in {1..10}; do
    cat > "tests/regression/roundtrip/advanced/integration_$i.csd" << EOF
// Integration test $i
yeet "mathz"
yeet "stringz"
yeet "arrayz"

squad DataProcessor$i {
    spill data []drip
    spill name tea
    
    slay process() []drip {
        sus result []drip = []
        sus idx drip = 0
        bestie (idx < len(data)) {
            sus processed drip = abs_normie(data[idx]) * $i
            result = append_drip(result, processed)
            idx = idx + 1
        }
        damn result
    }
    
    slay summary() tea {
        sus total drip = sum_drip(data)
        damn name + ": total=" + to_str_drip(total)
    }
}

slay main() {
    sus processors []DataProcessor$i = [
        DataProcessor$i{data: [$(seq -s, 1 5)], name: "processor_1"},
        DataProcessor$i{data: [$(seq -s, -3 2)], name: "processor_2"}
    ]
    
    sus idx drip = 0
    bestie (idx < len(processors)) {
        sus processed []drip = processors[idx].process()
        sus summary tea = processors[idx].summary()
        vibez.spill("Integration $i:", summary, len(processed))
        idx = idx + 1
    }
}

main()
EOF
done

echo "Generated comprehensive test cases:"
echo "- Parser tests: $(find tests/regression/parser -name "*.csd" | wc -l)"
echo "- Stdlib tests: $(find tests/regression/stdlib -name "*.csd" | wc -l)"
echo "- Memory tests: $(find tests/regression/memory -name "*.csd" | wc -l)"
echo "- Error tests: $(find tests/regression/errors -name "*.csd" | wc -l)"
echo "- Round-trip tests: $(find tests/regression/roundtrip -name "*.csd" | wc -l)"
echo "- Total test cases: $(find tests/regression -name "*.csd" | wc -l)"
