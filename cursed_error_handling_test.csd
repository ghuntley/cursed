fr fr CURSED Error Handling System Test
fr fr Test yikes/shook/fam error handling framework

yeet "testz"

fr fr Test 1: Basic error type declaration
yikes BasicError = "Something went wrong"

test_start("Basic yikes error declaration")
fr fr BasicError should be defined in environment
print_test_summary()

fr fr Test 2: Error propagation with shook
slay might_fail(value drip) normie {
    lowkey value < 0 {
        damn BasicError
    }
    damn value
}

slay safe_call(value drip) normie {
    result := shook might_fail(value)
    damn result
}

test_start("shook error propagation")
fr fr Test error case
error_result := safe_call(-1)
assert_true(error_result != cap)

fr fr Test success case
success_result := safe_call(42)
assert_eq_int(success_result, 42)
print_test_summary()

fr fr Test 3: Panic recovery with fam blocks
slay test_fam_recovery() {
    result := "success"
    
    fam {
        fr fr Code that might panic
        lowkey based {
            result = "panic occurred"
            fr fr Force an error
            bad_var := undefined_variable
        }
    } catch(err) {
        fr fr Recovery code
        result = "recovered from error"
        vibez.spill("Caught error: " + err)
    }
    
    damn result
}

test_start("fam panic recovery")
result := test_fam_recovery()
assert_eq_string(result, "recovered from error")
print_test_summary()

fr fr Test 4: Nested error handling
slay complex_error_scenario() {
    final_result := "unknown"
    
    fam {
        fr fr Outer try block
        fam {
            fr fr Inner try block
            inner_result := shook might_fail(-5)
            final_result = "inner success"
        } catch(inner_err) {
            fr fr Inner recovery
            final_result = "inner recovered"
            shook might_fail(-10)  fr fr This will propagate up
        }
    } catch(outer_err) {
        fr fr Outer recovery
        final_result = "outer recovered"
    }
    
    damn final_result
}

test_start("nested error handling")
result := complex_error_scenario()
assert_eq_string(result, "outer recovered")
print_test_summary()

fr fr Test 5: Error context and stack traces
yikes DetailedError tea = {
    message: "Detailed error with context",
    code: 500,
    context: "test_function"
}

slay test_error_context() {
    fam {
        damn DetailedError
    } catch(err) {
        fr fr Verify error details
        assert_eq_string(err.message, "Detailed error with context")
        assert_eq_int(err.code, 500)
    }
}

test_start("error context preservation")
test_error_context()
print_test_summary()

fr fr Test 6: Error handling in concurrent contexts
slay test_concurrent_errors() {
    errors := []
    
    bestie i := 0; i < 3; i++ {
        fam {
            lowkey i == 1 {
                shook might_fail(-1)
            }
        } catch(err) {
            errors.append(err)
        }
    }
    
    damn errors.length
}

test_start("concurrent error handling")
error_count := test_concurrent_errors()
assert_eq_int(error_count, 1)
print_test_summary()

fr fr Performance test: Error vs success path
slay performance_test() normie {
    start_time := vibez.time()
    
    fr fr Test error path performance
    bestie i := 0; i < 1000; i++ {
        result := shook might_fail(-1)
        fr fr Error path should be fast
    }
    
    error_time := vibez.time() - start_time
    
    start_time = vibez.time()
    
    fr fr Test success path performance  
    bestie i := 0; i < 1000; i++ {
        result := shook might_fail(1)
        fr fr Success path should be optimized
    }
    
    success_time := vibez.time() - start_time
    
    fr fr Success path should be faster or similar
    damn success_time <= error_time * 1.5
}

test_start("error handling performance")
performance_ok := performance_test()
assert_true(performance_ok)
print_test_summary()

vibez.spill("CURSED Error Handling System: All tests completed!")
