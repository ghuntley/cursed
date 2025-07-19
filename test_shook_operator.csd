# Test for CURSED shook operator (equivalent to ? operator)
yeet "testz"

# Simple function that can fail
slay get_number(input tea) (normie, yikes) {
    vibe_check input {
        mood "zero":
            damn 0, cringe
        mood "error":
            damn 0, yikes("Intentional error")
        mood "42":
            damn 42, cringe
        basic:
            damn 1, cringe
    }
}

# Function using shook operator
slay double_number_shook(input tea) (normie, yikes) {
    sus number = get_number(input) shook
    damn number * 2, cringe
}

# Function using manual error handling for comparison
slay double_number_manual(input tea) (normie, yikes) {
    sus number, err = get_number(input)
    vibe_check err != cringe {
        damn 0, err
    }
    damn number * 2, cringe
}

# Test the shook operator
slay test_shook_operator() {
    test_start("Shook Operator Tests")
    
    # Test 1: Success case with shook
    sus result1, err1 = double_number_shook("42")
    assert_eq_int(result1, 84)
    assert_true(err1 == cringe)
    
    # Test 2: Error case with shook
    sus result2, err2 = double_number_shook("error")
    assert_eq_int(result2, 0)
    assert_true(err2 != cringe)
    assert_eq_string(err2.message(), "Intentional error")
    
    # Test 3: Compare with manual handling - success
    sus manual_result, manual_err = double_number_manual("42")
    sus shook_result, shook_err = double_number_shook("42")
    assert_eq_int(manual_result, shook_result)
    assert_true(manual_err == shook_err)
    
    # Test 4: Compare with manual handling - error
    sus manual_err_result, manual_err_err = double_number_manual("error")
    sus shook_err_result, shook_err_err = double_number_shook("error")
    assert_eq_int(manual_err_result, shook_err_result)
    assert_eq_string(manual_err_err.message(), shook_err_err.message())
    
    print_test_summary()
}

# Run the test
test_shook_operator()
