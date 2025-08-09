// Performance test 39
sus large_array []drip = []
sus j drip = 0
bestie (j < 390) {
    large_array = append_drip(large_array, j)
    j = j + 1
}
vibez.spill("Performance 39 array length:", len(large_array))
