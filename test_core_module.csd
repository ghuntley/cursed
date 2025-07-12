yeet "core"

# Test basic core functionality
vibez.spill("Testing core module...")

# Test type conversion
sus test_bool lit = core.lit(42)
vibez.spill("lit(42) = " + core.string_from_bool(test_bool))

# Test math utilities
sus max_val normie = core.max(10, 20)
vibez.spill("max(10, 20) = " + core.tea(max_val))

# Test option type
sus some_val := core.option_some(42)
sus is_some lit = core.option_is_some(some_val)
vibez.spill("option_some(42) is_some = " + core.string_from_bool(is_some))

vibez.spill("Core module test complete!")
