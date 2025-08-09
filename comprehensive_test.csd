# Comprehensive CURSED Language Test

# Variables and arithmetic
sus x drip = 10
sus y drip = 5
sus result drip = x + y * 2
vibez.spill("Arithmetic:", result)

# Functions
slay multiply(a drip, b drip) drip {
    damn a * b
}
vibez.spill("Function result:", multiply(6, 7))

# Arrays
sus nums [drip] = [1, 2, 3, 4, 5]
vibez.spill("Array length:", len(nums))
vibez.spill("First element:", nums[0])
vibez.spill("Last element:", nums[4])

# Control structures
ready (result > 15) {
    vibez.spill("Result is greater than 15")
} otherwise {
    vibez.spill("Result is not greater than 15")
}

# Loops
sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}

# Pattern matching with ranges
sus value drip = 25
ready (value) {
    0..10 => vibez.spill("Small number")
    11..30 => vibez.spill("Medium number")  
    _ => vibez.spill("Large number")
}

# Pattern matching with when guards
sus score drip = 85
ready (score) {
    s when s >= 90 => vibez.spill("Grade: A")
    s when s >= 80 => vibez.spill("Grade: B")
    s when s >= 70 => vibez.spill("Grade: C")
    _ => vibez.spill("Grade: F")
}

vibez.spill("All tests completed!")
