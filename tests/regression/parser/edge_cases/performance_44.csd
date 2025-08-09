// Performance test 44
sus large_array []drip = []
sus j drip = 0
bestie (j < 440) {
    large_array = append_drip(large_array, j)
    j = j + 1
}
vibez.spill("Performance 44 array length:", len(large_array))
