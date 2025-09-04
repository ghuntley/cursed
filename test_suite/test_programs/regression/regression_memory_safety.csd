vibe main

yeet "vibez"
yeet "mathz"

fr fr Memory Safety Regression Test
fr fr Tests: Variable reassignment and expression evaluation safety
fr fr Expected: No memory issues or invalid references

slay main_character() {
    vibez.spill("=== Memory Safety Test ===")
    
    fr fr Test variable reassignment
    sus counter normie = 1
    vibez.spill("Initial counter:")
    vibez.spill(counter)
    
    counter = counter + 1
    vibez.spill("After increment:")
    vibez.spill(counter)
    
    counter = counter * 2
    vibez.spill("After doubling:")
    vibez.spill(counter)
    
    fr fr Test multiple variable operations
    sus x normie = 5
    sus y normie = 10
    sus temp normie = x
    x = y
    y = temp
    
    vibez.spill("After swap - x:")
    vibez.spill(x)
    vibez.spill("After swap - y:")
    vibez.spill(y)
    
    fr fr Test complex expressions with function calls
    sus result normie = mathz.add_two(x, y) + mathz.abs_normie(-3)
    vibez.spill("Complex expression result:")
    vibez.spill(result)
    
    fr fr Test variable scope and reuse
    sus i normie = 0
    bestie (i < 3) {
        vibez.spill("Loop iteration:")
        vibez.spill(i)
        i = i + 1
    }
    
    vibez.spill("=== Memory Safety Test Complete ===")
}
