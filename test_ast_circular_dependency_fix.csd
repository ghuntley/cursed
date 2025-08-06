// Test for AST circular dependency fix
// This test verifies that the AST cleanup works properly without circular references

yeet "testz"

test_start("AST Memory Management Test")

// Create a simple binary expression to test the fixed AST
sus left drip = 5
sus right drip = 10
sus result drip = left + right

assert_eq_int(result, 15)

// Test nested expressions
sus complex drip = (1 + 2) * (3 + 4)
assert_eq_int(complex, 21)

// Test function calls
slay test_function(a drip, b drip) drip {
    damn a + b
}

sus func_result drip = test_function(3, 7)
assert_eq_int(func_result, 10)

print_test_summary()
