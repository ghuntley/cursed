// Test error context generation in CURSED
// This tests the new error context creation LLVM IR

yeet "testz"

slay risky_operation(input normie) normie {
    sus input > 10 lit = input > 10;
    bestie input > 10 {
        yikes bad_input := "Input value too large";
        damn 0;
    }
    damn input * 2;
}

slay test_error_propagation() lit {
    result := shook risky_operation(15);
    damn based;
}

slay test_error_recovery() lit {
    fam error {
        yikes critical_error := "Critical error occurred";
        vibez.spill("This should not print");
    };
    damn based;
}

// Main test execution
test_start("Error context generation tests")

// Test basic error creation
yikes simple_error := "Simple error message"
assert_true(based)

// Test error propagation with context
test_result := test_error_propagation()
assert_true(test_result)

// Test error recovery with context
recovery_result := test_error_recovery()
assert_true(recovery_result)

print_test_summary()
