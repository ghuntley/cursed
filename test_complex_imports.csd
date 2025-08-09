# Test complex import patterns that might trigger parser regression

# Multiple imports on same line
yeet "mathz", "stringz", "arrayz"

# Nested module paths (if supported)
yeet "stdlib/advanced/cryptz"

# Import with aliasing (if supported)
yeet "mathz" as math_utils

# Chain imports
yeet "testz"
yeet "vibez"

# Test function calls after complex imports
vibez.spill("Testing complex imports")
test_start("import_test")
assert_eq_int(abs_normie(-10), 10)
print_test_summary()
