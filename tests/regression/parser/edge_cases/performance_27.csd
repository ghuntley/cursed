// Performance test 27
sus large_array []drip = []
sus j drip = 0
bestie (j < 270) {
    large_array = append_drip(large_array, j)
    j = j + 1
}
vibez.spill("Performance 27 array length:", len(large_array))
