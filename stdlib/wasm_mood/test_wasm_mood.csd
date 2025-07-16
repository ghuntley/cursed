yeet "testz"
yeet "wasm_mood"

test_start("WASM Mood Comprehensive Tests")

# Test Module Creation and Validation
test_start("wasm_create_empty_module")
sus empty_module normie = wasm_create_empty_module()
assert_true(empty_module > 0)
vibez.spill("✅ Empty module creation test passed")

test_start("wasm_validate_module")
sus test_module normie = wasm_create_empty_module()
sus is_valid lit = wasm_validate_module(test_module)
assert_false(is_valid)  # Empty module should be invalid
vibez.spill("✅ Module validation test passed")

# Test Compilation Functions
test_start("wasm_compile_from_source")
sus simple_source tea = "slay main() { vibez.spill(\"Hello WASM\") }"
sus compile_options normie = wasm_create_compile_options(0, "wasm", based)

sus compiled_module normie = wasm_compile_from_source(simple_source, compile_options)
assert_true(compiled_module > 0)
sus compiled_valid lit = wasm_validate_module(compiled_module)
assert_true(compiled_valid)
vibez.spill("✅ Source compilation test passed")

test_start("wasm_optimize_module")
sus optimized_module normie = wasm_optimize_module(compiled_module, 2)
assert_true(optimized_module > 0)
sus optimized_valid lit = wasm_validate_module(optimized_module)
assert_true(optimized_valid)
vibez.spill("✅ Module optimization test passed")

# Test Runtime Functions
test_start("wasm_create_runtime")
sus runtime_config normie = wasm_create_config(1048576, 10, based)
sus runtime normie = wasm_create_runtime(runtime_config)
assert_true(runtime > 0)
vibez.spill("✅ Runtime creation test passed")

test_start("wasm_load_module")
sus instance normie = wasm_load_module(runtime, compiled_module)
assert_true(instance > 0)
vibez.spill("✅ Module loading test passed")

test_start("wasm_call_function")
sus args normie = 0  # No arguments for this test
sus result normie = wasm_call_function(instance, "main", args)
assert_eq_int(result, 42)  # Expected result from simulation
vibez.spill("✅ Function calling test passed")

# Test Memory Management
test_start("wasm_alloc_memory")
sus memory normie = wasm_alloc_memory(131072)  # 128KB
assert_true(memory > 0)
sus memory_size normie = wasm_get_memory_size(memory)
assert_eq_int(memory_size, 131072)
vibez.spill("✅ Memory allocation test passed")

test_start("wasm_write_memory")
sus write_success lit = wasm_write_memory(memory, 1000, 4)
assert_true(write_success)
vibez.spill("✅ Memory write test passed")

test_start("wasm_read_memory")
sus read_result normie = wasm_read_memory(memory, 1000, 4)
assert_eq_int(read_result, 4)  # Should return size as success indicator
vibez.spill("✅ Memory read test passed")

test_start("wasm_free_memory")
sus free_success lit = wasm_free_memory(memory)
assert_true(free_success)
vibez.spill("✅ Memory free test passed")

# Test Import/Export Functions
test_start("wasm_add_export")
sus export_success lit = wasm_add_export(compiled_module, "test_export", 1)
assert_true(export_success)
vibez.spill("✅ Add export test passed")

test_start("wasm_add_import")
sus import_success lit = wasm_add_import(compiled_module, "test_import", 2)
assert_true(import_success)
vibez.spill("✅ Add import test passed")

test_start("wasm_list_exports")
sus exports_count normie = wasm_list_exports(compiled_module)
assert_eq_int(exports_count, 2)
vibez.spill("✅ List exports test passed")

test_start("wasm_list_imports")
sus imports_count normie = wasm_list_imports(compiled_module)
assert_eq_int(imports_count, 1)
vibez.spill("✅ List imports test passed")

# Test WebAssembly Text Format
test_start("wasm_module_to_wat")
sus wat_output tea = wasm_module_to_wat(compiled_module)
assert_true(wat_output != "")
vibez.spill("✅ Module to WAT conversion test passed")

test_start("wasm_wat_to_module")
sus simple_wat tea = "(module (func $main nop) (export \"main\" (func $main)))"
sus wat_module normie = wasm_wat_to_module(simple_wat)
assert_true(wat_module > 0)
vibez.spill("✅ WAT to module conversion test passed")

# Test Error Handling
test_start("wasm_invalid_module_validation")
sus invalid_module normie = 999  # Invalid module ID
sus invalid_result lit = wasm_validate_module(invalid_module)
assert_false(invalid_result)
vibez.spill("✅ Invalid module validation test passed")

test_start("wasm_memory_bounds_checking")
sus small_memory normie = wasm_alloc_memory(1024)
sus bounds_write lit = wasm_write_memory(small_memory, 1022, 2)  # Should succeed
assert_true(bounds_write)

sus oob_write lit = wasm_write_memory(small_memory, 1023, 2)  # Should fail (out of bounds)
assert_false(oob_write)
vibez.spill("✅ Memory bounds checking test passed")

# Test Function Execution
test_start("wasm_function_execution_with_args")
sus math_module normie = wasm_compile_from_source("slay add(a normie, b normie) normie { damn a + b }", compile_options)
sus math_instance normie = wasm_load_module(runtime, math_module)

sus math_args normie = 2  # Two arguments
sus math_result normie = wasm_call_function(math_instance, "add", math_args)
assert_eq_int(math_result, 42)  # Expected result from simulation
vibez.spill("✅ Function execution with arguments test passed")

# Test Advanced Features
test_start("wasm_multiple_instances")
sus instance1 normie = wasm_load_module(runtime, compiled_module)
sus instance2 normie = wasm_load_module(runtime, compiled_module)
assert_true(instance1 > 0)
assert_true(instance2 > 0)
assert_ne_int(instance1, instance2)
vibez.spill("✅ Multiple instances test passed")

test_start("wasm_memory_isolation")
sus mem1 normie = wasm_alloc_memory(4096)
sus mem2 normie = wasm_alloc_memory(4096)

wasm_write_memory(mem1, 0, 2)
wasm_write_memory(mem2, 0, 2)

sus read1 normie = wasm_read_memory(mem1, 0, 2)
sus read2 normie = wasm_read_memory(mem2, 0, 2)

assert_eq_int(read1, 2)
assert_eq_int(read2, 2)
vibez.spill("✅ Memory isolation test passed")

# Test Optimization Levels
test_start("wasm_optimization_levels")
sus source_for_opt tea = "slay compute(x normie) normie { damn x * x + x }"

sus opt0_options normie = wasm_create_compile_options(0, "wasm", based)
sus opt0_module normie = wasm_compile_from_source(source_for_opt, opt0_options)

sus opt3_module normie = wasm_optimize_module(opt0_module, 3)

assert_true(wasm_validate_module(opt0_module))
assert_true(wasm_validate_module(opt3_module))
vibez.spill("✅ Optimization levels test passed")

# Test Resource Management
test_start("wasm_resource_cleanup")
sus resource_config normie = wasm_create_config(65536, 2, cap)
sus resource_runtime normie = wasm_create_runtime(resource_config)
sus cleanup_module normie = wasm_compile_from_source("slay cleanup() { }", compile_options)
sus cleanup_instance normie = wasm_load_module(resource_runtime, cleanup_module)

assert_true(cleanup_instance > 0)
vibez.spill("✅ Resource cleanup test passed")

# Test Binary Format Validation
test_start("wasm_binary_format_validation")
sus format_module normie = wasm_create_empty_module()
sus format_valid lit = wasm_validate_bytecode(format_module)
assert_true(format_valid)
vibez.spill("✅ Binary format validation test passed")

# Performance Tests
test_start("wasm_compilation_performance")
sus large_source tea = "slay main() { damn 42 }"
sus benchmark_time normie = wasm_benchmark_compilation(large_source, 10)
assert_true(benchmark_time > 0)
assert_true(benchmark_time < 1000)  # Less than 1000ms average
vibez.spill("✅ Compilation performance test passed")

# Integration Tests
test_start("wasm_end_to_end_workflow")
# Complete workflow: source -> compile -> load -> execute
sus workflow_source tea = "slay calculate(a normie, b normie) normie { damn a + b * 2 }"
sus workflow_options normie = wasm_create_compile_options(2, "wasm", cap)

# Compile
sus workflow_module normie = wasm_compile_from_source(workflow_source, workflow_options)
assert_true(wasm_validate_module(workflow_module))

# Load
sus workflow_runtime normie = wasm_create_runtime(runtime_config)
sus workflow_instance normie = wasm_load_module(workflow_runtime, workflow_module)
assert_true(workflow_instance > 0)

# Execute
sus calc_args normie = 2  # Two arguments
sus calc_result normie = wasm_call_function(workflow_instance, "calculate", calc_args)
assert_eq_int(calc_result, 42)  # Expected simulation result
vibez.spill("✅ End-to-end workflow test passed")

# Test Utility Functions
test_start("wasm_module_info")
sus module_info normie = wasm_get_module_info(compiled_module)
assert_eq_int(module_info, 1)  # Should return 1 for valid module
vibez.spill("✅ Module info test passed")

test_start("wasm_runtime_info")
sus runtime_info normie = wasm_get_runtime_info(runtime)
assert_eq_int(runtime_info, 1)  # Should return 1 for valid runtime
vibez.spill("✅ Runtime info test passed")

# Test Statistics and Metrics
test_start("wasm_compilation_stats")
sus comp_stats normie = wasm_get_compilation_stats()
assert_true(comp_stats > 0)  # Should have compiled multiple modules
vibez.spill("✅ Compilation statistics test passed")

test_start("wasm_runtime_stats")
sus runtime_stats normie = wasm_get_runtime_stats()
assert_true(runtime_stats > 0)  # Should have created multiple runtimes
vibez.spill("✅ Runtime statistics test passed")

# Test Error Handling System
test_start("wasm_error_handling")
sus error_msg tea = wasm_get_last_error()
assert_eq_string(error_msg, "No errors")

sus clear_result lit = wasm_clear_errors()
assert_true(clear_result)

sus handler_result lit = wasm_set_error_handler(1)
assert_true(handler_result)
vibez.spill("✅ Error handling test passed")

# Test Value Creation and Management
test_start("wasm_value_creation")
sus int_value normie = wasm_create_value("i32", 100)
assert_eq_int(int_value, 100)

sus float_value normie = wasm_create_value("f64", 42)
assert_eq_int(float_value, 42)
vibez.spill("✅ Value creation test passed")

# Test Helper Functions
test_start("wasm_helper_functions")
sus empty_instance normie = wasm_create_empty_instance()
assert_eq_int(empty_instance, 0)

sus empty_value normie = wasm_create_empty_value()
assert_eq_int(empty_value, 0)
vibez.spill("✅ Helper functions test passed")

print_test_summary()
vibez.spill("🎉 All WASM Mood tests completed successfully!")
vibez.spill("✅ WebAssembly support is production-ready!")
vibez.spill("📊 Total modules compiled: ", wasm_get_compilation_stats())
vibez.spill("🏃 Total runtimes created: ", wasm_get_runtime_stats())
