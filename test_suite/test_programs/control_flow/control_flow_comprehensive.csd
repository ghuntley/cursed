vibe main

yeet "vibez"

fr fr Test: Comprehensive control flow structures
fr fr Purpose: Validate all control flow constructs work correctly
fr fr Expected: All control flow should execute in proper sequence

slay main_character() {
    vibez.spill("=== Testing Control Flow ===");
    
fr fr Simple if-else
    sus x: normie = 10;
    if x > 5 {
        vibez.spill("x is greater than 5");
    } else {
        vibez.spill("x is 5 or less");
    }
    
fr fr Nested if statements
    sus y: normie = 15;
    if x > 0 {
        if y > x {
            vibez.spill("Both x and y are positive, y > x");
        } else {
            vibez.spill("Both positive, but y <= x");
        }
    }
    
fr fr Complex boolean conditions
    sus a: lit = true;
    sus b: lit = false;
    
    if (a && !b) || (x > y) {
        vibez.spill("Complex boolean condition 1 passed");
    }
    
    if a || b && (x == 10) {
        vibez.spill("Complex boolean condition 2 passed");
    }
    
fr fr Multiple else-if chain (if supported)
    sus grade: normie = 85;
    if grade >= 90 {
        vibez.spill("Grade A");
    } else if grade >= 80 {
        vibez.spill("Grade B");
    } else if grade >= 70 {
        vibez.spill("Grade C");
    } else {
        vibez.spill("Grade F");
    }
    
fr fr Control flow with function calls
    sus result: lit = check_condition(x, y);
    if result {
        vibez.spill("Function returned true");
    } else {
        vibez.spill("Function returned false");
    }
    
fr fr Control flow with expressions
    if (x + y) > 20 && (x * 2) < 25 {
        vibez.spill("Expression-based condition passed");
    }
    
    damn 0;
}

damn check_condition(val1: normie, val2: normie) -> lit {
    damn val1 < val2;
}
