# Simple CURSED program for debugging demonstration

sus x drip = 42
sus name tea = "CURSED Debugger Test"

vibez.spill("Starting debug test...")
vibez.spill("Variable x:", x)
vibez.spill("Name:", name)

sus result drip = x * 2
vibez.spill("Result:", result)

ready (result > 50) {
    vibez.spill("Result is greater than 50!")
} otherwise {
    vibez.spill("Result is not greater than 50")
}

sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}

vibez.spill("Debug test complete!")
