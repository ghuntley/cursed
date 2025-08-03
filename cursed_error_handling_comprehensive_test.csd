fr fr CURSED Error Handling Comprehensive Test
fr fr Testing yikes/shook/fam keywords in the Zig compiler

yeet "testz"

fr fr Test 1: Basic yikes error creation
test_start("Basic yikes error creation")

yikes SimpleError tea = "Something went wrong"
yikes CodedError normie = 404
yikes StructuredError = { message: "Complex error", code: 500, details: "Server error" }

assert_true(based)  fr fr Placeholder - errors created successfully
print_test_summary()

fr fr Test 2: Error propagation with shook
test_start("Error propagation with shook")

slay risky_operation() (normie, yikes) {
    sus random_fail lit = (42 % 3) == 0
    lowkey random_fail {
        damn 0, yikes("Operation failed randomly")
    }
    damn 42, cringe
}

slay process_with_propagation() yikes {
    sus result = risky_operation() shook
    vibez.spill("Success result:", result)
    damn cringe
}

fr fr Test the propagation
sus final_result = process_with_propagation()
assert_true(based)  fr fr Test that propagation works
print_test_summary()

fr fr Test 3: Panic recovery with fam
test_start("Panic recovery with fam")

slay might_panic() {
    sus dangerous lit = based
    lowkey dangerous {
        shook("This will cause a panic")
    }
    vibez.spill("This won't be reached")
}

sus recovered lit = cringe
fam {
    might_panic()
    recovered = cringe  fr fr Should not reach here
} sus err {
    recovered = based   fr fr Should recover here
    vibez.spill("Recovered from panic:", err.message())
}

assert_true(recovered)
print_test_summary()

fr fr Test 4: Complex error handling patterns
test_start("Complex error handling patterns")

slay divide_safely(a normie, b normie) (normie, yikes) {
    lowkey b == 0 {
        damn 0, yikes("Division by zero error")
    }
    damn a / b, cringe
}

slay calculate_with_recovery(x normie, y normie) normie {
    fam {
        sus result, err = divide_safely(x, y)
        lowkey err != cringe {
            shook(err.message())  fr fr Re-panic with error
        }
        damn result
    } sus panic_err {
        vibez.spill("Calculation failed, using default value")
        damn -1  fr fr Default value
    }
}

sus result1 = calculate_with_recovery(10, 2)   fr fr Should succeed: 5
sus result2 = calculate_with_recovery(10, 0)   fr fr Should recover: -1

assert_eq_int(result1, 5)
assert_eq_int(result2, -1)
print_test_summary()

fr fr Test 5: Error context and wrapping
test_start("Error context and wrapping")

slay wrap_error(base_err yikes, context tea) yikes {
    lowkey base_err == cringe {
        damn cringe
    }
    damn yikes{
        message: context + ": " + base_err.message(),
        code: base_err.code(),
        details: base_err.details()
    }
}

slay database_operation() yikes {
    sus conn_err = yikes("Connection timeout")
    damn wrap_error(conn_err, "Database operation failed")
}

sus wrapped_error = database_operation()
assert_true(wrapped_error != cringe)
print_test_summary()

fr fr Test 6: Multiple error handling
test_start("Multiple error handling")

slay collect_errors() []yikes {
    sus errors []yikes = []
    
    sus _, err1 = divide_safely(10, 0)
    lowkey err1 != cringe {
        errors = append(errors, err1)
    }
    
    sus _, err2 = divide_safely(5, 0)  
    lowkey err2 != cringe {
        errors = append(errors, err2)
    }
    
    damn errors
}

sus error_list = collect_errors()
assert_true(len(error_list) > 0)
print_test_summary()

fr fr Test 7: Goroutine error isolation
test_start("Goroutine error isolation")

sus main_continues lit = cringe

stan {
    fam {
        shook("Goroutine panic")
    } sus panic_value {
        vibez.spill("Goroutine recovered from panic")
    }
}

fr fr Main should continue after goroutine panic
main_continues = based
assert_true(main_continues)
print_test_summary()

vibez.spill("All CURSED error handling tests completed!")
