# Test array bounds checking
sus arr []drip = [1, 2]
vibez.spill("Array length:", len(arr))
vibez.spill("Valid access arr[0]:", arr[0])
vibez.spill("Valid access arr[1]:", arr[1])
vibez.spill("Trying out of bounds access arr[5]:")
vibez.spill(arr[5])
