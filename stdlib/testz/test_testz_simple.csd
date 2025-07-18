yeet "testz"

# Simple test of enhanced testz v4.0 functionality
set_verbose_mode(based)
set_benchmark_mode(based)
set_memory_tracking(based)

vibez.spill("🧪 Testing Enhanced Testz Framework v4.0")
vibez.spill("===========================================")

# Test basic functionality
test_start("Basic enhanced assertions")
assert_true(based)
assert_false(cap)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")
assert_ne_int(10, 20)
assert_gt_int(50, 25)
assert_lt_int(25, 50)
test_end()

# Test both-mode functionality
test_start("Both-mode testing")
sus simple_code tea = "vibez.spill(\"test\")"
sus both_result lit = test_both_modes("Simple test", simple_code)
assert_true(both_result)
test_end()

# Test memory validation
test_start("Memory validation")
track_memory_allocation("test operation")
sus memory_ok lit = validate_memory_usage("basic test", 100)
assert_true(memory_ok)
sus no_leaks lit = validate_no_memory_leaks("test operation")
assert_true(no_leaks)
test_end()

# Test compilation validation
test_start("Compilation validation")
sus compile_ok lit = validate_compilation_success("simple_test.csd")
assert_true(compile_ok)
test_end()

# Test module validation
test_start("Module validation")
sus module_ok lit = validate_module_imports("testz")
assert_true(module_ok)
test_end()

# Test performance benchmarking
test_start("Performance benchmarking")
benchmark_start("Simple benchmark")
bestie i := 0; i < 10; i++ {
    sus temp normie = i * 2
}
sus duration normie = benchmark_end("Simple benchmark")
assert_gt_int(duration, 0)
test_end()

# Final validation
test_start("Test framework validation")
assert_eq_int(get_test_results(), 6)
assert_gt_int(get_passed_tests(), 3)
assert_gt_int(get_success_rate(), 60)
test_end()

# Print enhanced summary
print_test_summary()

vibez.spill("")
vibez.spill("🎯 Enhanced Testing Complete!")
vibez.spill("Both-mode tests: " + tea(both_mode_test_count))
vibez.spill("Benchmark mode: " + tea(benchmark_mode))
vibez.spill("Memory tracking: " + tea(memory_tracking))
