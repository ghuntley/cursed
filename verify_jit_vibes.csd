yeet "testz"
yeet "jit_vibes"

test_start("JIT Vibes module verification")

# Test basic JIT context creation
sus ctx := create_jit_context()
assert_eq_string(ctx.target_arch, "x86_64")
assert_eq_int(ctx.optimization_level, 0)
assert_false(ctx.is_compiled)

# Test adding code
sus added := add_code_to_jit(&ctx, "vibez.spill(\"test\")")
assert_true(added)

# Test compilation
sus compiled := compile_jit(&ctx)
assert_true(compiled)
assert_true(ctx.is_compiled)

# Test statistics
sus stats := get_jit_stats(&ctx)
vibez.spill("JIT Statistics generated successfully")

print_test_summary()
vibez.spill("✅ JIT Vibes module verification complete!")
