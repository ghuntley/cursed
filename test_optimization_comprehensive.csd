yeet "testz"

test_start("Comprehensive optimization test")

# Test constant folding
sus const_result normie = 2 + 3 * 4 + 5
assert_eq_int(const_result, 19)

# Test simple arithmetic
sus x normie = 10
sus y normie = 20
sus sum normie = x + y
assert_eq_int(sum, 30)

# Test function calls that may be inlined
slay multiply(a normie, b normie) normie {
    damn a * b
}

sus product normie = multiply(6, 7)
assert_eq_int(product, 42)

# Test loops that may be optimized
sus loop_sum normie = 0
bestie i := 0; i < 5; i++ {
    loop_sum = loop_sum + i
}
assert_eq_int(loop_sum, 10)

print_test_summary()
