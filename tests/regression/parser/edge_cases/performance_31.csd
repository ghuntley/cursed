// Performance test 31
sus large_array []drip = []
sus j drip = 0
bestie (j < 310) {
    large_array = append_drip(large_array, j)
    j = j + 1
}
vibez.spill("Performance 31 array length:", len(large_array))
