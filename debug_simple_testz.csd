# Simple testz module for debugging
sus test_count normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
}

slay print_test_summary() {
    vibez.spill("Tests: ", test_count)
}
