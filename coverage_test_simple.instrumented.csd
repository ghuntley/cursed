# Coverage instrumented: coverage_test_simple.csd
# COVERAGE: coverage_test_simple.csd:1
yeet "testz"

# Simple test for coverage analysis basics
# COVERAGE: coverage_test_simple.csd:4
test_start("Simple coverage test")

# COVERAGE: coverage_test_simple.csd:6
sus x normie = 42
# COVERAGE: coverage_test_simple.csd:7
sus y normie = x + 8
# COVERAGE: coverage_test_simple.csd:8
vibez.spill("Testing basic arithmetic")

# COVERAGE: coverage_test_simple.csd:10
assert_eq_int(y, 50)

# COVERAGE: coverage_test_simple.csd:12
print_test_summary()
