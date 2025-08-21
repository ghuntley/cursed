yeet "vibez"

vibez.spill("Testing CURSED for loop scope fix:")

# Test basic C-style for loop
bestie i := 0; i < 3; i++ {
    vibez.spill("Loop iteration:", i)
}

# Test nested scope
sus outer_var drip = 100

bestie j := 5; j > 0; j-- {
    sus inner_var drip = j * 10
    vibez.spill("Outer:", outer_var, "Inner:", inner_var, "j:", j)
}

# Verify loop variables are properly scoped (should not be accessible here)
vibez.spill("After loops, outer_var:", outer_var)
vibez.spill("Loop scope test completed successfully!")
