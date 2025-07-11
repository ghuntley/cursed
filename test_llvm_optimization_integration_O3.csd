vibez.spill("Testing LLVM optimization integration")

sus x normie = 10
sus y normie = 20
sus result normie = x + y

vibez.spill("Basic arithmetic result: ")
vibez.spill(result)

# Test loop optimization
bestie i := 0; i < 5; i++ {
    vibez.spill("Loop iteration: ")
    vibez.spill(i)
}

# Test function inlining opportunity
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

sus sum normie = add_numbers(42, 58)
vibez.spill("Function call result: ")
vibez.spill(sum)

# Test optimization with constants
sus constant_result normie = 5 * 3 + 10
vibez.spill("Constant folding result: ")
vibez.spill(constant_result)
