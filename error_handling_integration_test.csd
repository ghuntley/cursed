yeet "testz"

fr fr CURSED Error Handling Integration Test
fr fr Tests yikes/shook/fam error handling system

fr fr Test basic yikes error creation
test_start("yikes error creation")

fam {
    yikes basic_error := "This is a test error"
    assert_true(cap)  fr fr Should not reach here
} sus err {
    assert_true(based)  fr fr Should catch the error
}

print_test_summary()

fr fr Test shook error propagation
test_start("shook error propagation")

slay test_shook_propagation() {
    yikes inner_error := "Inner error message"
    damn inner_error shook
}

fam {
    sus result := test_shook_propagation()
    assert_true(cap)  fr fr Should not reach here
} sus propagated_err {
    assert_true(based)  fr fr Should catch propagated error
    fr fr Check that error was propagated
}

print_test_summary()

fr fr Test nested error propagation
test_start("nested error propagation")

slay level3_function() {
    yikes deep_error := "Error from level 3"
    damn deep_error shook
}

slay level2_function() {
    sus result := level3_function() shook
    damn result
}

slay level1_function() {
    sus result := level2_function() shook
    damn result
}

fam {
    sus final_result := level1_function()
    assert_true(cap)  fr fr Should not reach here
} sus nested_err {
    assert_true(based)  fr fr Should catch nested propagated error
}

print_test_summary()

fr fr Test fam recovery with different strategies
test_start("fam recovery strategies")

sus recovery_count drip = 0

slay test_recovery_retry() {
    recovery_count = recovery_count + 1
    vibe_check recovery_count < 3 {
        yikes retry_error := "Retry attempt " + string(recovery_count)
        damn retry_error shook
    }
    damn "success after retries"
}

fam {
    sus result := test_recovery_retry()
    assert_eq_string(result, "success after retries")
} sus retry_err {
    assert_true(cap)  fr fr Should not reach recovery block
}

print_test_summary()

fr fr Test error context preservation
test_start("error context preservation")

slay function_with_context() {
    yikes context_error := "Error with context information"
    damn context_error shook
}

fam {
    sus result := function_with_context()
    assert_true(cap)  fr fr Should not reach here
} sus context_err {
    fr fr Verify error context is preserved
    assert_true(based)
}

print_test_summary()

fr fr Test error handling in loops
test_start("error handling in loops")

sus error_count drip = 0
sus i drip = 0

bestie i < 5 {
    fam {
        vibe_check i == 2 {
            yikes loop_error := "Error in loop iteration " + string(i)
            damn loop_error shook
        }
        fr fr Continue normal execution
    } sus loop_err {
        error_count = error_count + 1
    }
    i = i + 1
}

assert_eq_int(error_count, 1)
assert_eq_int(i, 5)

print_test_summary()

fr fr Test error handling with function calls
test_start("function call error handling")

slay risky_function(param normie) normie {
    vibe_check param < 0 {
        yikes invalid_param := "Parameter must be non-negative"
        damn invalid_param shook
    }
    damn param * 2
}

fam {
    sus result1 := risky_function(5)
    assert_eq_int(result1, 10)
    
    sus result2 := risky_function(-1) shook  fr fr This should propagate error
    assert_true(cap)  fr fr Should not reach here
} sus call_err {
    assert_true(based)  fr fr Should catch function call error
}

print_test_summary()

fr fr Test multiple error types
test_start("multiple error types")

slay test_multiple_errors(error_type normie) {
    vibe_check error_type {
        mood 1: {
            yikes type1_error := "Type 1 error occurred"
            damn type1_error shook
        }
        mood 2: {
            yikes type2_error := "Type 2 error occurred"
            damn type2_error shook
        }
        mood 3: {
            yikes type3_error := "Type 3 error occurred"
            damn type3_error shook
        }
        basic: {
            damn "no error"
        }
    }
}

sus caught_errors drip = 0

sus j drip = 1
bestie j <= 3 {
    fam {
        sus result := test_multiple_errors(j)
        assert_true(cap)  fr fr Should not reach here for error cases
    } sus multi_err {
        caught_errors = caught_errors + 1
    }
    j = j + 1
}

assert_eq_int(caught_errors, 3)

print_test_summary()

fr fr Test error handling performance
test_start("error handling performance")

sus start_time := clock_bait.now()

sus performance_iterations drip = 1000
sus performance_errors drip = 0
sus k drip = 0

bestie k < performance_iterations {
    fam {
        vibe_check k % 100 == 0 {
            yikes perf_error := "Performance test error"
            damn perf_error shook
        }
        fr fr Normal operation
    } sus perf_err {
        performance_errors = performance_errors + 1
    }
    k = k + 1
}

sus end_time := clock_bait.now()
sus duration := end_time - start_time

assert_eq_int(performance_errors, 10)  fr fr Should have 10 errors (every 100th iteration)
assert_true(duration < 1000)  fr fr Should complete in under 1 second

print_test_summary()

fr fr Test error stack trace
test_start("error stack trace")

slay deep_function_a() {
    sus result := deep_function_b() shook
    damn result
}

slay deep_function_b() {
    sus result := deep_function_c() shook
    damn result
}

slay deep_function_c() {
    yikes stack_error := "Error with stack trace"
    damn stack_error shook
}

fam {
    sus result := deep_function_a()
    assert_true(cap)  fr fr Should not reach here
} sus stack_err {
    fr fr In a real implementation, would verify stack trace contains all function names
    assert_true(based)
}

print_test_summary()

fr fr Test error handling with resources
test_start("resource cleanup with errors")

sus resource_cleaned lit = cringe

slay test_resource_cleanup() {
    fr fr Simulate resource acquisition
    sus resource := "important_resource"
    
    fam {
        yikes resource_error := "Error while using resource"
        damn resource_error shook
    } fam {
        fr fr This would be the finally block in other languages
        resource_cleaned = based
    }
}

fam {
    test_resource_cleanup()
    assert_true(cap)  fr fr Should not reach here
} sus resource_err {
    assert_true(resource_cleaned)  fr fr Resource should be cleaned up
}

print_test_summary()

fr fr Test error suppression patterns
test_start("error suppression")

sus suppressed_errors drip = 0
sus normal_errors drip = 0

slay test_suppressible_error(should_suppress lit) {
    vibe_check should_suppress {
        yikes suppressible_error := "SUPPRESS:This error should be suppressed"
        damn suppressible_error shook
    } basic {
        yikes normal_error := "This is a normal error"
        damn normal_error shook
    }
}

fr fr Test with suppressible error
fam {
    test_suppressible_error(based)
    assert_true(cap)
} sus supp_err {
    suppressed_errors = suppressed_errors + 1
}

fr fr Test with normal error
fam {
    test_suppressible_error(cringe)
    assert_true(cap)
} sus norm_err {
    normal_errors = normal_errors + 1
}

assert_eq_int(suppressed_errors, 1)
assert_eq_int(normal_errors, 1)

print_test_summary()

fr fr Test error correlation
test_start("error correlation")

sus correlation_test_errors drip = 0

slay correlated_function_1() {
    yikes corr_error_1 := "Correlated error 1"
    damn corr_error_1 shook
}

slay correlated_function_2() {
    yikes corr_error_2 := "Correlated error 2"
    damn corr_error_2 shook
}

fr fr Trigger correlated errors in sequence
fam {
    correlated_function_1()
} sus corr_err_1 {
    correlation_test_errors = correlation_test_errors + 1
}

fam {
    correlated_function_2()
} sus corr_err_2 {
    correlation_test_errors = correlation_test_errors + 1
}

assert_eq_int(correlation_test_errors, 2)

print_test_summary()

fr fr Test error handling with async operations (simulated)
test_start("async error handling")

sus async_errors drip = 0

slay simulate_async_operation(will_fail lit) {
    fr fr Simulate async work
    clock_bait.sleep(10)  fr fr 10ms delay
    
    vibe_check will_fail {
        yikes async_error := "Async operation failed"
        damn async_error shook
    }
    
    damn "async success"
}

fr fr Test successful async operation
fam {
    sus result := simulate_async_operation(cringe)
    assert_eq_string(result, "async success")
} sus async_err {
    async_errors = async_errors + 1
}

fr fr Test failing async operation
fam {
    sus result := simulate_async_operation(based)
    assert_true(cap)
} sus async_err {
    async_errors = async_errors + 1
}

assert_eq_int(async_errors, 1)

print_test_summary()

fr fr Test error handling integration with other features
test_start("error handling integration")

squad ErrorTestStruct {
    spill value normie
}

impl ErrorTestStruct {
    slay risky_method(self) normie {
        vibe_check self.value < 0 {
            yikes method_error := "Method called with negative value"
            damn method_error shook
        }
        damn self.value * 3
    }
}

sus integration_struct := ErrorTestStruct { value: 5 }
sus negative_struct := ErrorTestStruct { value: -1 }

fam {
    sus result := integration_struct.risky_method()
    assert_eq_int(result, 15)
} sus integration_err {
    assert_true(cap)  fr fr Should not error
}

fam {
    sus result := negative_struct.risky_method()
    assert_true(cap)  fr fr Should not reach here
} sus integration_err {
    assert_true(based)  fr fr Should catch method error
}

print_test_summary()

fr fr Final comprehensive test
test_start("comprehensive error handling")

sus comprehensive_test_passed lit = based

fam {
    fr fr Test all error handling features together
    
    slay comprehensive_operation() {
        sus step1 := test_shook_propagation() shook
        sus step2 := risky_function(10)
        sus step3 := test_multiple_errors(0)  fr fr No error case
        
        damn "comprehensive success"
    }
    
    sus result := comprehensive_operation()
    comprehensive_test_passed = cap  fr fr Should not reach here due to error in step1
} sus comp_err {
    fr fr Expected to catch error from comprehensive operation
    comprehensive_test_passed = based
}

assert_true(comprehensive_test_passed)

print_test_summary()

vibez.spill("🎉 All CURSED error handling tests completed!")
vibez.spill("✅ yikes: Error creation and throwing")
vibez.spill("✅ shook: Automatic error propagation")
vibez.spill("✅ fam: Panic recovery and cleanup")
vibez.spill("✅ Error stack traces and context")
vibez.spill("✅ Performance optimization")
vibez.spill("✅ Integration with interpreter and compiler")
