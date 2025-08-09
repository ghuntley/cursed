sus sum drip = 0
sus numbers []drip = [1, 2, 3, 4, 5]
sus i drip = 0

vibez.spill("Initial values:")
vibez.spill("sum:", sum)
vibez.spill("numbers length:", len(numbers))
vibez.spill("i:", i)

bestie (i < len(numbers)) {
    vibez.spill("In loop - i:", i)
    vibez.spill("In loop - sum before:", sum)
    vibez.spill("In loop - numbers[i]:", numbers[i])
    sum = sum + numbers[i]
    vibez.spill("In loop - sum after:", sum)
    i = i + 1
}

vibez.spill("Final values:")
vibez.spill("sum:", sum)
vibez.spill("i:", i)
