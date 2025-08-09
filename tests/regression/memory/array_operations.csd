// Array operations and memory management
yeet "arrayz"

sus numbers []drip = [1, 2, 3, 4, 5]
sus doubled []drip = map_drip(numbers, double_value)

slay double_value(x drip) drip {
    damn x * 2
}

// Array slicing and manipulation
sus slice1 []drip = numbers[1:4]
sus slice2 []drip = numbers[0:2]
sus combined []drip = append_drip(slice1, slice2)

// Array operations in loops
sus i drip = 0
sus sums []drip = []
bestie (i < len(numbers)) {
    sus partial_sum drip = sum_drip(numbers[0:i+1])
    sums = append_drip(sums, partial_sum)
    i = i + 1
}

vibez.spill("Doubled:", doubled)
vibez.spill("Sums:", sums)
