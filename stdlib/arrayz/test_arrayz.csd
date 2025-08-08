fr fr Test for arrayz module

yeet "arrayz"

vibez.spill("Testing arrayz module")

fr fr Test basic array operations
sus nums [drip] = [1, 2, 3, 4, 5]
vibez.spill("Array length:", len(nums))
vibez.spill("First element:", nums[0])
vibez.spill("Sum of first two:", add_numbers(nums[0], nums[1]))

fr fr Test array utility functions
vibez.spill("Sum of 10, 20, 30:", array_sum(10, 20, 30))
vibez.spill("Double 5:", multiply_by_two(5))
vibez.spill("Is 1 equal to 1?", equals_one(1))
vibez.spill("Is 0 equal to 0?", equals_zero(0))

fr fr Test string arrays
sus names [tea] = ["alice", "bob", "charlie"]
vibez.spill("Names length:", len(names))
vibez.spill("Combined name:", concat_two(names[0], names[1]))

vibez.spill("All arrayz tests completed!")
