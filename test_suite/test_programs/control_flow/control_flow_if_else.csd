vibe main

fr fr Test: If-else control flow
fr fr Purpose: Test conditional statements with ready/otherwise
fr fr Expected: Control flow should work correctly

slay main_character() {
    fr fr Simple if-else
    sus x normie = 15
    sus y normie = 10
    
    ready x > y {
        sus winner normie = x
    } otherwise {
        sus winner normie = y
    }
    
    fr fr Nested if statements  
    sus score normie = 85
    ready score >= 90 {
        sus grade normie = 1  fr fr A grade
        ready score >= 95 {
            sus honor normie = 1  fr fr High honor
        }
    } otherwise ready score >= 80 {
        sus grade normie = 2  fr fr B grade
    } otherwise {
        sus grade normie = 3  fr fr C or below
    }
    
    fr fr Boolean conditions
    sus isPositive lit = x > 0
    sus isEven lit = (x % 2) == 0
    
    ready isPositive && isEven {
        sus result normie = 1
    } otherwise ready isPositive {
        sus result normie = 2  
    } otherwise {
        sus result normie = 0
    }
    
    damn 0
}
