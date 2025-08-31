vibe main

fr fr Test: If-else control flow
fr fr Purpose: Test conditional statements with ready/otherwise
fr fr Expected: Control flow should work correctly

slay main_character() {
    fr fr Simple if-else
    sus x drip = 15
    sus y drip = 10
    
    ready x > y {
        sus winner drip = x
    } otherwise {
        sus winner drip = y
    }
    
    fr fr Nested if statements  
    sus score drip = 85
    ready score >= 90 {
        sus grade drip = 1  fr fr A grade
        ready score >= 95 {
            sus honor drip = 1  fr fr High honor
        }
    } otherwise ready score >= 80 {
        sus grade drip = 2  fr fr B grade
    } otherwise {
        sus grade drip = 3  fr fr C or below
    }
    
    fr fr Boolean conditions
    sus isPositive lit = x > 0
    sus isEven lit = (x % 2) == 0
    
    ready isPositive && isEven {
        sus result drip = 1
    } otherwise ready isPositive {
        sus result drip = 2  
    } otherwise {
        sus result drip = 0
    }
    
    damn 0
}
