slay test_defer_basic() {
    vibez.spill("Function start")
    later vibez.spill("Deferred 1")
    later vibez.spill("Deferred 2")
    vibez.spill("Function middle")
    later vibez.spill("Deferred 3")
    vibez.spill("Function end")
}

slay test_defer_nested() {
    vibez.spill("Outer function start")
    later vibez.spill("Outer deferred")
    
    slay inner_func() {
        vibez.spill("Inner function start")
        later vibez.spill("Inner deferred")
        vibez.spill("Inner function end")
    }
    
    inner_func()
    vibez.spill("Outer function end")
}

slay test_defer_with_return() {
    vibez.spill("Function start")
    later vibez.spill("Deferred before return")
    
    bestie i := 0; i < 5; i++ {
        sus x := i
        later vibez.spill("Deferred in loop")
        
        cap i == 3 {
            vibez.spill("Returning early")
            damn normie(42)
        }
    }
    
    vibez.spill("Function end")
    damn normie(0)
}

# Test basic defer
test_defer_basic()

# Test nested defer
test_defer_nested()

# Test defer with return
sus result := test_defer_with_return()
vibez.spill("Result: " + result)
