vibez.spill("Testing optimization levels")

# Test constant folding
sus const_result normie = 2 + 3 * 4 + 5
vibez.spill("Constant folding result: ")
vibez.spill(const_result)

# Test simple arithmetic
sus x normie = 10
sus y normie = 20
sus sum normie = x + y
vibez.spill("Simple arithmetic result: ")
vibez.spill(sum)

# Test function calls that may be inlined
slay multiply(a normie, b normie) normie {
    damn a * b
}

sus product normie = multiply(6, 7)
vibez.spill("Function call result: ")
vibez.spill(product)

# Test loops that may be optimized
sus loop_sum normie = 0
bestie i := 0; i < 5; i++ {
    loop_sum = loop_sum + i
}
vibez.spill("Loop sum result: ")
vibez.spill(loop_sum)

vibez.spill("All optimization tests completed successfully!")
