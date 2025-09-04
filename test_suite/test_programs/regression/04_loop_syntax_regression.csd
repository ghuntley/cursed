vibe main
yeet "vibez"

fr fr Loop Syntax Regression Test
fr fr Tests: Ensures periodt loop syntax doesn't regress
fr fr Expected: Correct loop execution and termination

slay main_character() {
    vibez.spill("=== Loop Syntax Regression Test ===")
    
    vibez.spill("Testing basic periodt loop...")
    sus counter = 1
    periodt (counter <= 5) {
        vibez.spill("Count:", counter)
        counter = counter + 1
    }
    
    vibez.spill("Testing countdown loop...")
    sus countdown = 3
    periodt (countdown > 0) {
        vibez.spill("Countdown:", countdown)
        countdown = countdown - 1
    }
    vibez.spill("Loop completed!")
    
    vibez.spill("Testing nested loops...")
    sus outer = 1
    periodt (outer <= 2) {
        vibez.spill("Outer loop:", outer)
        
        sus inner = 1
        periodt (inner <= 2) {
            vibez.spill("  Inner loop:", inner)
            inner = inner + 1
        }
        
        outer = outer + 1
    }
    
    vibez.spill("Testing loop with conditions...")
    sus value = 10
    periodt (value > 0) {
        ready (value == 5) {
            vibez.spill("Halfway point reached")
        }
        
        ready (value % 2 == 0) {
            vibez.spill("Even number:", value)
        } otherwise {
            vibez.spill("Odd number:", value)
        }
        
        value = value - 1
    }
    
    vibez.spill("Loop syntax regression test completed")
}
