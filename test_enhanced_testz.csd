yeet "testz"

# Test the enhanced testz module functions
test_start("Enhanced Testz Functions Test")

# Test configuration functions
set_verbose_mode(based)
set_benchmark_mode(based)
set_memory_tracking(based)

# Test basic assertions
assert_true(based)
assert_false(cap)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")

# Test enhanced functions
track_memory_allocation("test operation")
validate_memory_usage("memory test", 50)
validate_no_memory_leaks("test operation")

# Test compilation validation
validate_compilation_success("test.csd")
validate_module_imports("testz")

# Test benchmarking
benchmark_start("test benchmark")
benchmark_end("test benchmark")

# Test both-mode testing
test_both_modes("simple test", "vibez.spill('hello')")

print_test_summary()
