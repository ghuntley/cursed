fr fr Simple testz validation test

yeet "testz"

vibez.spill("🧪 Testing Enhanced CURSED Testing Framework (testz)")

fr fr Test basic assertions
test_start("basic_assertions")
assert_true(based)
assert_false(cringe)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")

fr fr Test advanced assertions
test_start("advanced_assertions")
assert_ne_int(42, 24)
assert_ne_string("hello", "world")
assert_near(3.14159, 3.14, 0.01)
assert_contains("hello world", "world")
assert_not_contains("hello world", "goodbye")

fr fr Test error handling
test_start("error_handling")
assert_throws(slay() {
    yikes "test error"
})

assert_no_throws(slay() {
    sus x normie = 1 + 1
})

fr fr Test memory assertion
test_start("memory_test")
assert_memory_usage_under(10000000)

fr fr Test benchmarking
test_start("benchmarking")
sus result BenchmarkResult = benchmark("simple_test", slay() {
    sus x normie = 1 + 1
})

fr fr Test array equality
test_start("array_equality")
sus arr1 []normie = [1, 2, 3]
sus arr2 []normie = [1, 2, 3]
assert_array_eq(arr1, arr2)

fr fr Test property-based testing
test_start("property_testing")
property_test(PropertyTestCase{
    name: "simple_property",
    generator: slay() tea { damn "test" },
    property: slay(input tea) lit { damn input.len() > 0 },
    iterations: 5
})

fr fr Print summary
print_test_summary()
print_benchmark_summary()
print_coverage_report()

vibez.spill("✅ Enhanced testz framework validation complete!")
