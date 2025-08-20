fr fr Test all P0 fixes comprehensively
vibe main {
    fr fr Test 1: Variable evaluation and dereferencing
    sus x drip = 42
    sus y drip = 24
    sus sum drip = x + y
    vibez.spill("Variable evaluation test:", sum)
    
    fr fr Test 2: Multi-argument function calls
    slay add_three(a drip, b drip, c drip) drip {
        damn a + b + c
    }
    sus result drip = add_three(10, 20, 30)
    vibez.spill("Function call test:", result)
    
    fr fr Test 3: Control flow execution
    ready x > y {
        vibez.spill("Control flow test: x is greater")
    } otherwise {
        vibez.spill("Control flow test: y is greater or equal")
    }
    
    fr fr Test 4: Array literals with variables
    sus numbers []drip = [x, y, sum, result]
    vibez.spill("Array test:", numbers)
    
    fr fr Test 5: Struct field access
    squad Point {
        x drip
        y drip
    }
    sus p Point = Point{ x: 100, y: 200 }
    vibez.spill("Struct test:", p.x, p.y)
    
    fr fr Test 6: Error handling (yikes/fam)
    fam {
        slay divide(a drip, b drip) drip {
            ready b == 0 {
                yikes "division by zero"
            }
            damn a / b
        }
        sus division_result drip = divide(10, 2) shook
        vibez.spill("Error handling test:", division_result)
    } sus error {
        vibez.spill("Caught error:", error)
    }
    
    vibez.spill("All P0 tests completed successfully!")
}
