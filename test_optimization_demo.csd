vibez.spill("CURSED Optimization Demo")

# Test various optimization opportunities
sus base_value normie = 100

# Constant folding optimization
sus folded_constant normie = 5 + 10 * 2 + 15  # Should be optimized to 40
vibez.spill("Constant folding result: ")
vibez.spill(folded_constant)

# Function inlining opportunity
slay add_ten(x normie) normie {
    damn x + 10
}

slay multiply_by_two(x normie) normie {
    damn x * 2
}

# Chain of function calls that could be inlined
sus chained_result normie = multiply_by_two(add_ten(base_value))
vibez.spill("Chained function calls result: ")
vibez.spill(chained_result)

# Loop optimization opportunities
sus loop_accumulator normie = 0
bestie i := 0; i < 10; i++ {
    loop_accumulator = loop_accumulator + i
}
vibez.spill("Loop accumulator result: ")
vibez.spill(loop_accumulator)

# Dead code elimination test
sus unused_variable normie = 999  # This might be eliminated if not used
sus used_variable normie = 42
vibez.spill("Used variable: ")
vibez.spill(used_variable)

# Complex expression optimization
sus complex_expr normie = (base_value + 50) * 2 - 100 + folded_constant
vibez.spill("Complex expression result: ")
vibez.spill(complex_expr)

vibez.spill("Optimization demo completed successfully!")
