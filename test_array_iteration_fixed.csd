# Test array iteration - step by step
sus nums []drip = [100, 200, 300]
sus i drip = 0

vibez.spill("Starting iteration...")
vibez.spill("i =", i, "len =", len(nums), "condition:", i < len(nums))

bestie (i < len(nums)) {
    vibez.spill("Element", i, ":", nums[i])
    i = i + 1
    vibez.spill("Updated i =", i)
}

vibez.spill("Iteration finished")
