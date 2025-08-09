fr fr Comprehensive CURSED interpreter test

fr fr Test variable declarations
sus x drip = 42
sus name tea = "CURSED"
sus flag lit = based

fr fr Test output
vibez.spill("Testing variables:")
vibez.spill("x =", x)
vibez.spill("name =", name)
vibez.spill("flag =", flag)

fr fr Test expressions
sus result drip = (x + 10) * 2
vibez.spill("Expression result:", result)

fr fr Test arrays
sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("Array length:", len(numbers))
vibez.spill("First element:", numbers[0])

fr fr Test function definition and call
slay multiply(a drip, b drip) drip {
    damn a * b
}
vibez.spill("multiply(6, 7) =", multiply(6, 7))

fr fr Test stdlib function (without import)
vibez.spill("abs_normie(-42) =", abs_normie(-42))

fr fr Test control flow
ready (x > 40) {
    vibez.spill("x is greater than 40")
} otherwise {
    vibez.spill("x is not greater than 40")
}

fr fr Test loop
vibez.spill("Counting from 0 to 2:")
sus i drip = 0
bestie (i < 3) {
    vibez.spill(i)
    i = i + 1
}

vibez.spill("All tests completed!")
