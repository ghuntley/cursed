// Performance test 37
sus large_array []drip = []
sus j drip = 0
bestie (j < 370) {
    large_array = append_drip(large_array, j)
    j = j + 1
}
vibez.spill("Performance 37 array length:", len(large_array))
