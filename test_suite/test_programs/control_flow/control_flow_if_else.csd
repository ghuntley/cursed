vibe main
yeet "vibez"

fr fr Test: If-else control flow
fr fr Purpose: Test conditional statements with ready/otherwise
fr fr Expected: Control flow should work correctly

slay main_character() {
    fr fr Simple if-else
    sus x normie = 15
    sus y normie = 10
    
    ready x > y {
        sus winner normie = x
        vibez.spill("Winner is x:", winner)
    } otherwise {
        sus winner normie = y
        vibez.spill("Winner is y:", winner)
    }
    
    fr fr Nested if statements  
    sus score normie = 85
    ready score >= 90 {
        sus grade normie = 1  fr fr A grade
        vibez.spill("Grade: A")
        ready score >= 95 {
            sus honor normie = 1  fr fr High honor
            vibez.spill("High honor!")
        }
    } otherwise ready score >= 80 {
        sus grade normie = 2  fr fr B grade
        vibez.spill("Grade: B")
    } otherwise {
        sus grade normie = 3  fr fr C or below
        vibez.spill("Grade: C or below")
    }
    
    fr fr Boolean conditions
    sus isPositive lit = x > 0
    sus isEven lit = (x % 2) == 0
    
    ready isPositive && isEven {
        sus result normie = 1
        vibez.spill("Number is positive and even")
    } otherwise ready isPositive {
        sus result normie = 2
        vibez.spill("Number is positive but odd")
    } otherwise {
        sus result normie = 0
        vibez.spill("Number is negative")
    }
    
    damn 0
}
