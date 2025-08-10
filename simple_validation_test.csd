# Simple Core Functionality Test
sus x drip = 42
sus name tea = "CURSED"
sus active lit = based

vibez.spill("Number:", x)
vibez.spill("String:", name)
vibez.spill("Boolean:", active)

# Array test
sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("Array:", numbers)

# Function test
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

sus result drip = add_numbers(10, 20)
vibez.spill("Function result:", result)

# Control flow test
ready (x > 40) {
    vibez.spill("X is greater than 40")
} otherwise {
    vibez.spill("X is 40 or less")
}

vibez.spill("Simple test completed!")
