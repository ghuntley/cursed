# Memory stress test with loops and allocations
yeet "vibez"

vibez.spill("Starting loop memory test...")

# Test array allocations in loops
sus total drip = 0
bestie (sus i drip = 0; i < 100; i++) {
    sus temp_array []drip = [i, i*2, i*3]
    total = total + len(temp_array)
}

vibez.spill("Loop test completed, total:", total)

# Test string operations
sus result tea = "start"
bestie (sus i drip = 0; i < 50; i++) {
    result = result + "test"
}

vibez.spill("String test completed, length:", len(result))
vibez.spill("Memory test finished!")
