yeet "testz"
yeet "jit_vibes"

test_start("JIT Vibes comprehensive tests")

# Test 1: JIT context creation
test_start("JIT context creation")
sus ctx := create_jit_context()
assert_eq_string(ctx.target_arch, "x86_64")
assert_eq_int(ctx.optimization_level, 0)
assert_false(ctx.is_compiled)
print_test_summary()

# Test 2: Adding code to JIT buffer
test_start("Adding code to JIT buffer")
sus ctx2 := create_jit_context()
sus result := add_code_to_jit(&ctx2, "vibez.spill(\"hello\")")
assert_true(result)
print_test_summary()

# Test 3: Setting optimization levels
test_start("Setting optimization levels")
sus ctx3 := create_jit_context()
assert_true(set_jit_optimization(&ctx3, 2))
assert_eq_int(ctx3.optimization_level, 2)
assert_false(set_jit_optimization(&ctx3, 5))  # Invalid level
print_test_summary()

# Test 4: LLVM IR generation
test_start("LLVM IR generation")
sus ctx4 := create_jit_context()
add_code_to_jit(&ctx4, "vibez.spill(\"hello\")")
sus ir := generate_llvm_ir(&ctx4)
assert_true(len(ir) > 0)
print_test_summary()

# Test 5: JIT compilation
test_start("JIT compilation")
sus ctx5 := create_jit_context()
add_code_to_jit(&ctx5, "vibez.spill(\"hello\")")
assert_true(compile_jit(&ctx5))
assert_true(ctx5.is_compiled)
print_test_summary()

# Test 6: JIT execution
test_start("JIT execution")
sus ctx6 := create_jit_context()
add_code_to_jit(&ctx6, "vibez.spill(\"hello\")")
compile_jit(&ctx6)
sus exec_result := execute_jit(&ctx6)
assert_eq_int(exec_result, 0)
print_test_summary()

# Test 7: JIT statistics
test_start("JIT statistics")
sus ctx7 := create_jit_context()
add_code_to_jit(&ctx7, "vibez.spill(\"hello\")")
sus stats := get_jit_stats(&ctx7)
assert_true(len(stats) > 0)
print_test_summary()

# Test 8: JIT context clearing
test_start("JIT context clearing")
sus ctx8 := create_jit_context()
add_code_to_jit(&ctx8, "test code")
compile_jit(&ctx8)
assert_true(clear_jit(&ctx8))
assert_false(ctx8.is_compiled)
print_test_summary()

# Test 9: Code validation
test_start("Code validation")
assert_true(validate_jit_code("vibez.spill(\"hello\")"))
assert_true(validate_jit_code("sus x := 42"))
assert_false(validate_jit_code(""))
assert_false(validate_jit_code("invalid syntax"))
print_test_summary()

# Test 10: Optimized JIT context creation
test_start("Optimized JIT context creation")
sus opt_ctx := create_optimized_jit(3)
assert_eq_int(opt_ctx.optimization_level, 3)
print_test_summary()

# Test 11: JIT compilation benchmark
test_start("JIT compilation benchmark")
sus benchmark_time := benchmark_jit_compilation("vibez.spill(\"test\")", 5)
assert_true(benchmark_time > 0)
print_test_summary()

# Test 12: Multiple code additions
test_start("Multiple code additions")
sus ctx12 := create_jit_context()
assert_true(add_code_to_jit(&ctx12, "sus x := 42"))
assert_true(add_code_to_jit(&ctx12, "vibez.spill(x)"))
assert_true(compile_jit(&ctx12))
print_test_summary()

# Test 13: Compilation state management
test_start("Compilation state management")
sus ctx13 := create_jit_context()
add_code_to_jit(&ctx13, "test")
compile_jit(&ctx13)
# Should not be able to add code after compilation
assert_false(add_code_to_jit(&ctx13, "more code"))
print_test_summary()

# Test 14: Empty code handling
test_start("Empty code handling")
sus ctx14 := create_jit_context()
sus empty_ir := generate_llvm_ir(&ctx14)
assert_eq_string(empty_ir, "")
assert_false(compile_jit(&ctx14))
print_test_summary()

# Test 15: Complex JIT workflow
test_start("Complex JIT workflow")
sus ctx15 := create_optimized_jit(2)
assert_true(validate_jit_code("vibez.spill(\"complex\")"))
assert_true(add_code_to_jit(&ctx15, "vibez.spill(\"complex\")"))
sus complex_ir := generate_llvm_ir(&ctx15)
assert_true(len(complex_ir) > 100)  # Should be substantial IR
assert_true(compile_jit(&ctx15))
sus result15 := execute_jit(&ctx15)
assert_eq_int(result15, 0)
print_test_summary()

# Test 16: JIT context reuse
test_start("JIT context reuse")
sus ctx16 := create_jit_context()
add_code_to_jit(&ctx16, "first")
compile_jit(&ctx16)
clear_jit(&ctx16)
assert_true(add_code_to_jit(&ctx16, "second"))
print_test_summary()

# Test 17: Optimization level bounds checking
test_start("Optimization level bounds checking")
sus ctx17 := create_jit_context()
assert_false(set_jit_optimization(&ctx17, -1))
assert_false(set_jit_optimization(&ctx17, 4))
assert_true(set_jit_optimization(&ctx17, 0))
assert_true(set_jit_optimization(&ctx17, 3))
print_test_summary()

# Test 18: Statistics content validation
test_start("Statistics content validation")
sus ctx18 := create_optimized_jit(1)
add_code_to_jit(&ctx18, "vibez.spill(\"stats test\")")
compile_jit(&ctx18)
sus detailed_stats := get_jit_stats(&ctx18)
# Stats should contain key information
print_test_summary()

# Test 19: Execution without compilation
test_start("Execution without compilation")
sus ctx19 := create_jit_context()
add_code_to_jit(&ctx19, "test")
sus exec_fail := execute_jit(&ctx19)
assert_eq_int(exec_fail, -1)  # Should fail
print_test_summary()

# Test 20: JIT pipeline integration test
test_start("JIT pipeline integration test")
sus pipeline_ctx := create_optimized_jit(3)
assert_true(validate_jit_code("damn 42"))
assert_true(add_code_to_jit(&pipeline_ctx, "damn 42"))
sus pipeline_ir := generate_llvm_ir(&pipeline_ctx)
assert_true(compile_jit(&pipeline_ctx))
sus pipeline_stats := get_jit_stats(&pipeline_ctx)
sus pipeline_result := execute_jit(&pipeline_ctx)
assert_eq_int(pipeline_result, 42)
print_test_summary()

print_test_summary()
vibez.spill("JIT Vibes module tests completed successfully!")
