sus numbers []drip = [1, 2, 3]
sus i drip = 0

fr fr This works - pre-calculating the function result
sus array_len drip = len(numbers)
vibez.spill("Pre-calculated length:", array_len)

bestie (i < array_len) {
    vibez.spill("Working loop - i:", i)
    i = i + 1
}

fr fr Reset for direct function call test
i = 0
vibez.spill("Testing direct function call in comparison...")

fr fr This will fail due to expression parsing bug
fr fr bestie (i < len(numbers)) {
fr fr     vibez.spill("Direct function call - i:", i)
fr fr     i = i + 1
fr fr }

vibez.spill("Bug demonstration: Function calls in comparison expressions need fixing")
