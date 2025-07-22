yeet "testz"
yeet "macro_slay"

test_start("Basic macro_slay functionality test")

fr fr Test basic constants
assert_true(MACRO_FUNCTION == 1)
assert_true(MACRO_EXPRESSION == 2)
assert_true(EXPAND_IMMEDIATE == 10)

fr fr Test module status
sus version tea = macro_slay_version()
assert_true(version == "1.0.0")

assert_true(is_macro_slay_ready())

print_test_summary()

vibez.spill("✅ Basic macro_slay test completed!")
