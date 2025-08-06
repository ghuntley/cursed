yeet "testz"

test_start("Comprehensive Error Handling Features Test")

# Test error creation with yikes
test_start("Yikes error creation")
yikes simple_error := "Simple error message"
vibez.spill("Created error: " + simple_error)
test_end()

# Test error propagation with shook
test_start("Shook error propagation")
slay dangerous_operation() yikes {
    yikes operation_error := "Dangerous operation failed"
    damn operation_error shook
}

slay calling_function() yikes {
    sus result := dangerous_operation() shook
    vibez.spill("This line shouldn't execute")
    damn cringe
}

sus propagated := calling_function()
vibez.spill("Propagated error: " + propagated)
test_end()

# Test fam error recovery
test_start("Fam error recovery")
sus recovered lit := cap
sus error_message tea := ""

fam {
    yikes recovery_test := "Error for recovery testing"
    vibez.spill("This should not print")
} sus caught {
    recovered = based
    error_message = caught
    vibez.spill("Caught error: " + caught)
}

assert_true(recovered)
assert_eq_string(error_message, "Error for recovery testing")
test_end()

# Test defer with error handling
test_start("Defer with error handling")
sus cleanup_done lit := cap

slay test_defer_error() yikes {
    later {
        cleanup_done = based
        vibez.spill("Defer cleanup executed")
    }
    
    yikes defer_test := "Error with defer cleanup"
    damn defer_test shook
}

sus defer_result := test_defer_error()
assert_true(cleanup_done)
assert_eq_string(defer_result, "Error with defer cleanup")
test_end()

# Test nested try/catch with proper cleanup
test_start("Nested try/catch with cleanup")
sus outer_cleanup lit := cap
sus inner_cleanup lit := cap
sus final_error tea := ""

fam {
    later {
        outer_cleanup = based
        vibez.spill("Outer cleanup executed")
    }
    
    fam {
        later {
            inner_cleanup = based
            vibez.spill("Inner cleanup executed")
        }
        
        yikes nested_error := "Nested error"
    } sus inner_caught {
        yikes wrapped := "Wrapped: " + inner_caught
        damn wrapped shook
    }
} sus outer_caught {
    final_error = outer_caught
    vibez.spill("Final caught error: " + outer_caught)
}

assert_true(outer_cleanup)
assert_true(inner_cleanup)
assert_eq_string(final_error, "Wrapped: Nested error")
test_end()

print_test_summary()
vibez.spill("✅ Comprehensive error handling test completed!")
