// Advanced error handling patterns test
yeet "testz"

test_start("Advanced Error Patterns")

// Test finally-like behavior with fam
slay test_finally_behavior() lit {
    sus cleanup_executed lit = cringe
    
    shook {
        yikes "Error with cleanup"
    } fam err {
        cleanup_executed = based
        vibez.spill("Cleanup executed in fam block")
    }
    
    damn cleanup_executed
}

assert_true(test_finally_behavior())

// Test error chaining
slay test_error_chaining() lit {
    shook {
        shook {
            yikes "Original error"
        } fam e1 {
            vibez.spill("First handler:", e1)
            yikes "Chained error from first handler"
        }
    } fam e2 {
        vibez.spill("Second handler:", e2)
        damn based
    }
}

assert_true(test_error_chaining())

// Test custom error types with shook
slay test_custom_error_types() lit {
    shook {
        yikes "CustomError: Something specific went wrong"
        damn cringe
    } fam custom_err {
        ready (std.mem.startsWith(custom_err, "CustomError:")) {
            vibez.spill("Correctly caught custom error")
            damn based
        } otherwise {
            damn cringe
        }
    }
}

assert_true(test_custom_error_types())

// Test error handling with struct operations
squad ErrorTestStruct {
    spill value drip
    spill error_prone lit
}

slay test_struct_error_handling() lit {
    sus test_struct ErrorTestStruct = ErrorTestStruct{value: 42, error_prone: based}
    
    shook {
        ready (test_struct.error_prone) {
            yikes "Struct indicates error condition"
        }
        vibez.spill("Struct value:", test_struct.value)
        damn cringe
    } fam struct_err {
        vibez.spill("Handled struct error:", struct_err)
        damn based
    }
}

assert_true(test_struct_error_handling())

// Test shook with return values
slay error_or_value(should_error lit) drip {
    shook {
        ready (should_error) {
            yikes "Requested error"
        }
        damn 123
    } fam err {
        vibez.spill("Error in function:", err)
        damn -1
    }
}

slay test_shook_return_values() lit {
    sus success_value drip = error_or_value(cringe)
    sus error_value drip = error_or_value(based)
    
    ready (success_value == 123 && error_value == -1) {
        damn based
    } otherwise {
        vibez.spill("Wrong values:", success_value, error_value)
        damn cringe
    }
}

assert_true(test_shook_return_values())

print_test_summary()
vibez.spill("🔥 Advanced error patterns test completed!")
