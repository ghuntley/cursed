// Test different for loop update syntaxes

slay test_increment() {
    bestie i := 0; i < 5; i++ {
        vibez.spill("i++ works")
    }
}

slay test_assignment() {
    bestie i := 5; i < 10; i = i + 1 {
        vibez.spill("i = i + 1")
    }
}

test_increment()
