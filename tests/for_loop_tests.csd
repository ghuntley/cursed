vibe for_loop_tests

slay test_basic_c_style_for() {
    fr fr Basic for loop with short declaration
    sus count normie = 0
    bestie i := 0; i < 3; i++ {
        count = count + 1
    }
    vybe count == 3
}

slay test_for_loop_with_decrement() {
    fr fr For loop with decrement
    sus count normie = 0
    bestie i := 3; i > 0; i-- {
        count = count + 1
    }
    vybe count == 3
}

slay test_for_loop_with_assignment() {
    fr fr For loop with assignment update
    sus count normie = 0
    bestie i := 0; i < 10; i = i + 2 {
        count = count + 1
    }
    vybe count == 5
}

slay test_for_loop_with_empty_init() {
    fr fr For loop with empty initialization
    sus i normie = 0
    sus count normie = 0
    bestie ; i < 3; i++ {
        count = count + 1
    }
    vybe count == 3
}

slay test_for_loop_with_empty_condition() {
    fr fr For loop with empty condition (infinite loop with break)
    sus count normie = 0
    bestie i := 0; ; i++ {
        count = count + 1
        vybe count < 3
        ghosted
    }
    vybe count == 3
}

slay test_for_loop_with_empty_update() {
    fr fr For loop with empty update
    sus count normie = 0
    bestie i := 0; i < 3; {
        count = count + 1
        i++
    }
    vybe count == 3
}

slay test_nested_for_loops() {
    fr fr Nested C-style for loops
    sus total normie = 0
    bestie i := 0; i < 2; i++ {
        bestie j := 0; j < 3; j++ {
            total = total + 1
        }
    }
    vybe total == 6
}

slay test_for_loop_with_different_variable_types() {
    fr fr For loop with different variable types
    sus count normie = 0
    bestie x := 1; x <= 2; x++ {
        count = count + x
    }
    vybe count == 3
}
