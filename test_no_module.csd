# Test testz functionality without module imports

sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0

slay assert_eq_int(actual normie, expected normie) lit {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill("PASS: ", actual, " == ", expected)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("FAIL: Expected ", expected, ", got ", actual)
    }
    damn based
}

slay print_summary() lit {
    vibez.spill("Test Results:")
    vibez.spill("Pass: ", pass_count)
    vibez.spill("Fail: ", fail_count)
    damn based
}

# Run tests
assert_eq_int(2 + 2, 4)
assert_eq_int(10 - 5, 5)
assert_eq_int(3 * 3, 9)

print_summary()
