vibe main_character

yeet "vibez"

fr fr Test: Comprehensive control flow structures
fr fr Purpose: Validate all control flow constructs work correctly
fr fr Expected: All control flow should execute in proper sequence

slay main_character() normie {
    vibez.spill("=== Testing Control Flow ===")
    
fr fr Simple if-else
    sus x normie = 10
    ready (x > 5) {
        vibez.spill("x is greater than 5")
    } otherwise {
        vibez.spill("x is 5 or less")
    }
    
fr fr Nested if statements
    sus y normie = 15
    ready (x > 0) {
        ready (y > x) {
            vibez.spill("Both x and y are positive, y > x")
        } otherwise {
            vibez.spill("Both positive, but y <= x")
        }
    }
    
fr fr Complex boolean conditions
    sus a lit = based
    sus b lit = cringe
    
    ready ((a && !b) || (x > y)) {
        vibez.spill("Complex boolean condition 1 passed")
    }
    
    ready (a || b && (x == 10)) {
        vibez.spill("Complex boolean condition 2 passed")
    }
    
fr fr Multiple else-if chain (if supported)
    sus grade normie = 85
    ready (grade >= 90) {
        vibez.spill("Grade A")
    } otherwise ready (grade >= 80) {
        vibez.spill("Grade B")
    } otherwise ready (grade >= 70) {
        vibez.spill("Grade C")
    } otherwise {
        vibez.spill("Grade F")
    }
    
fr fr Control flow with function calls
    sus result lit = check_condition(x, y)
    ready (result) {
        vibez.spill("Function returned true")
    } otherwise {
        vibez.spill("Function returned false")
    }
    
fr fr Control flow with expressions
    ready ((x + y) > 20 && (x * 2) < 25) {
        vibez.spill("Expression-based condition passed")
    }
    
    damn 0
}

slay check_condition(val1 normie, val2 normie) lit {
    damn val1 < val2
}
