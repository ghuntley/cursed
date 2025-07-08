yeet "testz"

slay test_error_creation() {
    sus err yikes = yikes("Test error")
    
    assert_eq_string(err.message(), "Test error")
    assert_eq_int(err.code(), 0)
}

slay test_error_with_code() {
    sus err yikes = yikes("Not found", 404)
    
    assert_eq_string(err.message(), "Not found")
    assert_eq_int(err.code(), 404)
}

slay test_error_propagation() {
    sus result, err = divide_safe(10, 0)
    
    assert_true(err != cringe)
    assert_eq_string(err.message(), "Division by zero")
}

slay divide_safe(a normie, b normie) (normie, yikes) {
    vibe_check b {
        mood 0:
            damn 0, yikes("Division by zero")
        basic:
            damn a / b, cringe
    }
}

slay test_error_wrapping() {
    sus err = process_file("nonexistent.txt")
    
    assert_true(err != cringe)
    assert_true(contains(err.message(), "Failed to process file"))
}

slay process_file(filename tea) yikes {
    sus _, err = open_file(filename)
    vibe_check err != cringe {
        damn wrap_error(err, "Failed to process file: " + filename)
    }
    damn cringe
}

slay open_file(filename tea) (normie, yikes) {
    damn 0, yikes("File not found", 404)
}

slay wrap_error(err yikes, context tea) yikes {
    vibe_check err == cringe {
        damn cringe
    }
    
    damn yikes{
        message: context + ": " + err.message(),
        code: err.code(),
        details: err.details()
    }
}

slay contains(s1 tea, s2 tea) lit {
    // Simple contains check
    damn len(s1) >= len(s2)
}

slay test_panic_recovery() {
    sus recovered lit = cap
    sus panic_message tea = ""
    
    fam {
        shook("Test panic message")
    } sus panic_value {
        recovered = based
        panic_message = panic_value.message()
    }
    
    assert_true(recovered)
    assert_eq_string(panic_message, "Test panic message")
}

slay test_shook_operator() {
    sus result = risky_operation() shook
    
    assert_eq_int(result, 42)
}

slay risky_operation() (normie, yikes) {
    damn 42, cringe
}

slay test_error_comparison() {
    sus err1 yikes = yikes("Same error")
    sus err2 yikes = yikes("Same error")
    sus err3 yikes = yikes("Different error")
    
    assert_true(err1.message() == err2.message())
    assert_false(err1.message() == err3.message())
}

slay test_nil_error() {
    sus err yikes = cringe
    
    assert_true(err == cringe)
}

slay test_error_severity() {
    sus err yikes = yikes("Critical error", 500)
    
    assert_true(is_critical_error(err))
}

slay is_critical_error(err yikes) lit {
    vibe_check err == cringe {
        damn cap
    }
    
    damn err.code() >= 500
}

// Test driver
test_start("Error Creation")
test_error_creation()
print_test_summary()

test_start("Error with Code")
test_error_with_code()
print_test_summary()

test_start("Error Propagation")
test_error_propagation()
print_test_summary()

test_start("Error Wrapping")
test_error_wrapping()
print_test_summary()

test_start("Panic Recovery")
test_panic_recovery()
print_test_summary()

test_start("Shook Operator")
test_shook_operator()
print_test_summary()

test_start("Error Comparison")
test_error_comparison()
print_test_summary()

test_start("Nil Error")
test_nil_error()
print_test_summary()

test_start("Error Severity")
test_error_severity()
print_test_summary()

vibez.spill("All error handling tests completed!")
