yeet "testz"
yeet "macro_slay"

# Test macro_slay module functionality

test_start("Basic macro_slay module tests")

# Test module status
sus version tea = macro_slay_version()
assert_true(version == "1.0.0")

sus status tea = macro_slay_status()
vibez.spill("Status: " + status)

assert_true(is_macro_slay_ready())

vibez.spill("✅ Basic module tests passed")

test_start("Macro registration and management tests")

# Test macro registration
sus test_macro normie = register_macro("test_func", 1, 10, "test_body")
assert_true(test_macro != 0)

# Test macro count
sus count_before normie = get_macro_count()
sus another_macro normie = register_macro("another", 2, 11, "body")
sus count_after normie = get_macro_count()
assert_true(count_after > count_before)

vibez.spill("✅ Macro registration tests passed")

test_start("Macro type checking tests")

# Test macro type checking
assert_true(is_function_macro(test_macro))
assert_true(!is_expression_macro(test_macro))
assert_true(!is_statement_macro(test_macro))

# Test macro type extraction
assert_true(get_macro_type(test_macro) == 1)
assert_true(get_macro_type(another_macro) == 2)

vibez.spill("✅ Macro type checking tests passed")

test_start("Macro expansion tests")

# Test function macro expansion
sus func_result tea = expand_function_macro(test_macro, "param1", 0)
assert_true(func_result != "")
vibez.spill("Function expansion: " + func_result)

# Test expression macro expansion
sus expr_result tea = expand_expression_macro(another_macro, "add", 0)
assert_true(expr_result == "a + b")

# Test statement macro expansion
sus stmt_macro normie = register_macro("stmt", 3, 10, "body")
sus stmt_result tea = expand_statement_macro(stmt_macro, "print", 0)
assert_true(stmt_result != "")

# Test general macro expansion
sus general_result tea = expand_macro(test_macro, "args", 0)
assert_true(general_result != "")

vibez.spill("✅ Macro expansion tests passed")

test_start("Macro system capabilities tests")

# Test capability queries
sus types_count normie = get_supported_macro_types()
assert_true(types_count == 6)

sus modes_count normie = get_supported_expand_modes()
assert_true(modes_count == 4)

vibez.spill("✅ Capability tests passed")

print_test_summary()

vibez.spill("\n🎉 All macro_slay module tests completed successfully!")
vibez.spill("📊 Test Coverage Summary:")
vibez.spill("   • Basic module functionality")
vibez.spill("   • Macro registration and management")
vibez.spill("   • Type checking and validation")
vibez.spill("   • Macro expansion engine")
vibez.spill("   • System capabilities")
