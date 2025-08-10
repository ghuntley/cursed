# Simple validation of the generic function fix

# The original failing case from P0 tests
slay generic_function<T>(value T) T {
    damn value
}

# This line was causing: "Undefined variable in drip assignment: 'generic_function<drip>(100)'"
sus generic_result_int drip = generic_function<drip>(100)

# Test with string too
sus generic_result_string tea = generic_function<tea>("test")

# Simple validation
sus x drip = 100
sus y tea = "test"
