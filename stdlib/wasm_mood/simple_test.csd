yeet "testz"
yeet "wasm_mood"

test_start("Basic WASM Mood Test")

# Test basic module creation
sus module1 normie = wasm_create_empty_module()
assert_true(module1 > 0)
vibez.spill("✅ Created empty module: ", module1)

# Test module compilation
sus options normie = wasm_create_compile_options(0, "wasm", based)
sus module2 normie = wasm_compile_from_source("slay main() { damn 42 }", options)
assert_true(module2 > 0)
vibez.spill("✅ Compiled source to module: ", module2)

# Test runtime creation
sus config normie = wasm_create_config(1048576, 10, based)
sus runtime normie = wasm_create_runtime(config)
assert_true(runtime > 0)
vibez.spill("✅ Created runtime: ", runtime)

# Test module loading
sus instance normie = wasm_load_module(runtime, module2)
assert_true(instance > 0)
vibez.spill("✅ Loaded module into instance: ", instance)

# Test function execution
sus result normie = wasm_call_function(instance, "main", 0)
assert_eq_int(result, 42)
vibez.spill("✅ Function call result: ", result)

# Test memory allocation
sus memory normie = wasm_alloc_memory(4096)
assert_true(memory > 0)
sus memory_size normie = wasm_get_memory_size(memory)
assert_eq_int(memory_size, 4096)
vibez.spill("✅ Allocated memory: ", memory, " size: ", memory_size)

# Test memory operations
sus write_ok lit = wasm_write_memory(memory, 100, 8)
assert_true(write_ok)
sus read_result normie = wasm_read_memory(memory, 100, 8)
assert_eq_int(read_result, 8)
vibez.spill("✅ Memory write/read test passed")

# Test WAT conversion
sus wat_text tea = wasm_module_to_wat(module2)
assert_true(wat_text != "")
vibez.spill("✅ WAT conversion: ", wat_text)

print_test_summary()
vibez.spill("🎉 Basic WASM Mood tests completed!")
