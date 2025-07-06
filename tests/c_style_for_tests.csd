// Test basic C-style for loop
slay test_basic_for_loop() {
    sus result normie = 0
    bestie i := 0; i < 5; i++ {
        result = result + i
    }
    
    // result should be 0 + 1 + 2 + 3 + 4 = 10
    test_framework.assert_equals(result, 10, "Basic for loop sum")
}

// Test for loop with assignment update
slay test_for_with_assignment() {
    sus count normie = 0
    bestie j := 1; j <= 4; j = j + 1 {
        count = count + 1
    }
    
    // count should be 4 (iterations: j=1,2,3,4)
    test_framework.assert_equals(count, 4, "For loop with assignment update")
}

// Test for loop with decrement
slay test_for_with_decrement() {
    sus sum normie = 0
    bestie k := 5; k > 0; k-- {
        sum = sum + k
    }
    
    // sum should be 5 + 4 + 3 + 2 + 1 = 15
    test_framework.assert_equals(sum, 15, "For loop with decrement")
}

// Test for loop with explicit variable declaration
slay test_for_with_declaration() {
    sus total normie = 0
    bestie sus m normie = 2; m < 8; m = m * 2 {
        total = total + m
    }
    
    // total should be 2 + 4 = 6 (iterations: m=2,4, then m=8 which fails condition)
    test_framework.assert_equals(total, 6, "For loop with variable declaration")
}

// Test infinite loop with break
slay test_infinite_loop_with_break() {
    sus iterations normie = 0
    bestie ; ; {
        iterations = iterations + 1
        lowkey iterations >= 3 {
            ghosted
        }
    }
    
    test_framework.assert_equals(iterations, 3, "Infinite loop with break")
}

// Test for loop with continue
slay test_for_with_continue() {
    sus even_sum normie = 0
    bestie n := 0; n < 10; n++ {
        lowkey n % 2 == 1 {
            simp  // Skip odd numbers
        }
        even_sum = even_sum + n
    }
    
    // even_sum should be 0 + 2 + 4 + 6 + 8 = 20
    test_framework.assert_equals(even_sum, 20, "For loop with continue")
}

slay main() {
    test_basic_for_loop()
    test_for_with_assignment()
    test_for_with_decrement()
    test_for_with_declaration()
    test_infinite_loop_with_break()
    test_for_with_continue()
    
    vibez.spill("All C-style for loop tests passed!")
}
