# Simple GC performance test
vibez.spill("Testing GC performance improvements...")

# Create some allocation patterns
bestie i := 0; i < 100; i++ {
    sus data := "test_data_" + string(i)
    if (i % 10) == 0 {
        vibez.spill("Allocated", i, "objects")
    }
}

vibez.spill("GC test complete")
