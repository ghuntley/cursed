vibez.spill("Testing break (ghosted) and continue (simp) statements")

# Test 1: Basic break statement
vibez.spill("Test 1: Basic break statement")
sus i normie = 0
sus count normie = 0
yolo i < 10 {
    lowkey i == 5 {
        vibez.spill("Breaking at i = 5")
        ghosted
    }
    vibez.spill("i = " + i)
    i++
    count++
}
vibez.spill("Loop ended after " + count + " iterations")

# Test 2: Basic continue statement 
vibez.spill("Test 2: Basic continue statement")
sus j normie = 0
yolo j < 10 {
    lowkey j % 2 == 0 {
        j++
        simp
    }
    vibez.spill("Odd j = " + j)
    j++
}

# Test 3: Nested loop with break/continue
vibez.spill("Test 3: Nested loop with break/continue")
sus outer normie = 0
yolo outer < 3 {
    sus inner normie = 0
    yolo inner < 5 {
        lowkey inner == 3 {
            vibez.spill("Continue inner at outer=" + outer + ", inner=" + inner)
            inner++
            simp
        }
        lowkey outer == 1 && inner == 2 {
            vibez.spill("Break inner at outer=" + outer + ", inner=" + inner)
            ghosted
        }
        vibez.spill("outer=" + outer + ", inner=" + inner)
        inner++
    }
    outer++
}

vibez.spill("All tests completed!")
