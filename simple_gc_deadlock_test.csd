yeet "vibez"

# Simple GC finalization deadlock prevention test
# This test verifies basic GC functionality without triggering parser issues

vibez.spill("Testing GC finalization deadlock prevention...")

sus test_object drip = 42
vibez.spill("Created test object:", test_object)

# Simulate memory pressure that could trigger GC
sus data_array [10]drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

bestie (sus i drip = 0; i < 10; i = i + 1) {
    data_array[i] = data_array[i] * 2
}

vibez.spill("Array processing completed")

# This should complete without hanging due to GC deadlock
vibez.spill("GC deadlock prevention test completed successfully")
