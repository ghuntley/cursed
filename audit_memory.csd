// Test memory allocation and GC
sus data = []normie

bestie (sus i drip = 0; i < 1000; i = i + 1) {
    data.push(i)
}

vibez.spill("Allocated 1000 integers")
vibez.spill(data.len())

// Force GC
gc_collect()
vibez.spill("GC completed")
