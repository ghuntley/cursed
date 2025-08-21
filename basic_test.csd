fr fr Basic CURSED test without any imports

spill("=== Basic CURSED Test ===")

fr fr Test variables
sus number drip = 42
sus text tea = "Hello CURSED"
sus flag lit = based

spill("Number:", number)
spill("Text:", text)
spill("Flag:", flag)

fr fr Test arithmetic
sus sum drip = 10 + 32
spill("10 + 32 =", sum)

fr fr Test array
sus numbers []drip = [1, 2, 3, 4, 5]
spill("Array length:", len(numbers))
spill("First element:", numbers[0])
spill("Last element:", numbers[4])

fr fr Test string concatenation
sus greeting tea = "Hello" + " " + "World"
spill("Greeting:", greeting)

fr fr Test conditional
ready (number == 42) {
    spill("✅ Conditional works")
} otherwise {
    spill("❌ Conditional failed")
}

fr fr Test loop
sus i drip = 0
bestie (i < 3) {
    spill("Loop", i)
    i = i + 1
}

spill("✅ Basic CURSED features working!")
