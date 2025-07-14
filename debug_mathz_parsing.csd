// Test specific for loops from mathz that might be causing issues

// This one should work (line 54 pattern)
slay test1() {
    sus result normie = 1
    sus absExp normie = 5
    bestie i := 0; i < absExp; i++ {
        result = result * 2
    }
}

// This one might be problematic (line 176 pattern)
slay test2() {
    sus n normie = 100
    bestie i := 5; i * i <= n; i = i + 6 {
        // body
    }
}

test1()
test2()
