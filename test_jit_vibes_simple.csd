yeet "jit_vibes"

# Simple JIT vibes test
sus ctx := create_jit_context()
vibez.spill("JIT context created successfully")

sus result := add_code_to_jit(&ctx, "vibez.spill(\"hello\")")
vibez.spill("Code added to JIT buffer")

sus compiled := compile_jit(&ctx)
vibez.spill("JIT compilation completed")

sus stats := get_jit_stats(&ctx)
vibez.spill("JIT Statistics:")
vibez.spill(stats)
