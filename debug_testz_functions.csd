# Test just the first few functions from testz to isolate the error

sus test_count normie = 0
sus pass_count normie = 0  
sus fail_count normie = 0
sus current_test_name tea = ""

slay test_start(name tea) lit {
    current_test_name = name
    test_count = test_count + 1
    damn based
}

slay assert_true(condition lit) lit {
    lowkey condition == based {
        pass_count = pass_count + 1
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("FAIL: Expected true, got false")
    }
    damn based
}

vibez.spill("Testing testz functions")
test_start("simple test")
assert_true(based)
