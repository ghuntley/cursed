# Minimal test without module import
sus test_count normie = 0

slay simple_test() {
    test_count = test_count + 1
    vibez.spill("Test " + tea(test_count) + " passed")
}

simple_test()
vibez.spill("Minimal test complete")
