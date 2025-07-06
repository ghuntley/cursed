fr fr Comprehensive slice expression tests for CURSED

vibez.spill("=== CURSED Slice Expression Tests ===")

fr fr Test 1: Basic array creation and slice operations
sus numbers normie = [10, 20, 30, 40, 50, 60]
vibez.spill("Original array: ", numbers)

fr fr Test 2: Full range of slice syntax
sus slice_full normie = numbers[:]      fr fr Full slice
sus slice_mid normie = numbers[1:4]     fr fr Middle slice: [20, 30, 40]
sus slice_start normie = numbers[2:]    fr fr From index: [30, 40, 50, 60]
sus slice_end normie = numbers[:3]      fr fr To index: [10, 20, 30]

vibez.spill("Full slice [:]:", slice_full)
vibez.spill("Mid slice [1:4]:", slice_mid)
vibez.spill("Start slice [2:]:", slice_start)
vibez.spill("End slice [:3]:", slice_end)

fr fr Test 3: Edge cases
sus empty_slice normie = numbers[2:2]   fr fr Empty slice
sus single_elem normie = numbers[0:1]   fr fr Single element
sus last_elem normie = numbers[5:6]     fr fr Last element

vibez.spill("Empty slice [2:2]:", empty_slice)
vibez.spill("Single element [0:1]:", single_elem)
vibez.spill("Last element [5:6]:", last_elem)

fr fr Test 4: String array slicing
sus words tea = ["hello", "world", "cursed", "lang"]
sus word_slice tea = words[1:3]
vibez.spill("String slice [1:3]:", word_slice)

vibez.spill("=== All slice tests completed successfully! ===")
