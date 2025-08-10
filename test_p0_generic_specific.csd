# Test the specific P0 failing case
slay generic_function<T>(value T) T {
    damn value
}

# This was the failing line from P0 tests
sus generic_result_int drip = generic_function<drip>(100)

# Simple output test
sus result_message tea = "Generic function works!"

# No stdlib import to avoid parsing errors
# Just use direct print
