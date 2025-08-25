fr fr Test the fixed testz module with real test execution
yeet "testz"

fr fr Test that should pass
slay test_basic_math() lit {
    assert_eq_int(2 + 2, 4)
    damn based
}

fr fr Test that should fail
slay test_intentional_failure() lit {
    assert_eq_int(2 + 2, 5)  fr fr This should fail
    damn based
}

fr fr Run the tests
test_start("Testz Real Execution Validation")

fr fr Test real interpretation execution
sus simple_test_code tea = "yeet \"vibez\"; vibez.spill(\"Hello, World!\")"
sus result lit = execute_interpretation_test(simple_test_code)
ready (result) {
    vibez.spill("✅ Real interpretation test execution working")
} otherwise {
    vibez.spill("❌ Real interpretation test execution failed")
}

fr fr Test real compilation execution
sus compile_test_code tea = "yeet \"vibez\"; vibez.spill(\"Compiled Hello!\")"
sus compile_result lit = execute_compilation_test(compile_test_code)
ready (compile_result) {
    vibez.spill("✅ Real compilation test execution working")
} otherwise {
    vibez.spill("❌ Real compilation test execution failed")
}

fr fr Test function invocation
sus func_result lit = invoke_test_function("test_basic_math")
ready (func_result) {
    vibez.spill("✅ Real function invocation working")
} otherwise {
    vibez.spill("❌ Real function invocation failed")
}

print_test_summary()
