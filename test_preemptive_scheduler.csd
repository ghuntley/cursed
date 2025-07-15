yeet "testz"

test_start("preemptive scheduler integration test")

sus counter normie = 0

# Simple goroutine test
yolo {
    counter = counter + 1
    vibez.spill("Goroutine executed")
}

# Test that counter was incremented
assert_eq_int(counter, 1)

# Test multiple goroutines
counter = 0
yolo {
    counter = counter + 1
}
yolo {
    counter = counter + 1
}
yolo {
    counter = counter + 1
}

# Allow goroutines to complete
sus i normie = 0
bestie i < 1000; i++ {
    yolo
    lowkey counter >= 3 {
        break
    }
}

# Test that all goroutines completed
assert_eq_int(counter, 3)

print_test_summary()
