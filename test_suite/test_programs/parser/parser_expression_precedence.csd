vibe main

yeet "vibez"

fr fr Test: Expression precedence and operator parsing
fr fr Purpose: Ensure mathematical and logical operators follow correct precedence rules
fr fr Expected: Operations should evaluate in proper order (*, / before +, -, etc.)

slay main_character() {
    fr fr Arithmetic precedence tests
    sus result1 drip = 2 + 3 * 4        fr fr Should be 14, not 20
    sus result2 drip = 20 / 4 + 2       fr fr Should be 7, not 3
    sus result3 drip = (2 + 3) * 4      fr fr Should be 20
    sus result4 drip = 2 * 3 + 4 * 5    fr fr Should be 26
    
    vibez.spill("2 + 3 * 4 = {}", result1)
    vibez.spill("20 / 4 + 2 = {}", result2)
    vibez.spill("(2 + 3) * 4 = {}", result3)
    vibez.spill("2 * 3 + 4 * 5 = {}", result4)
    
    fr fr Boolean precedence tests
    sus a lit = based
    sus b lit = cringe
    sus c lit = based
    
    fr fr AND should have higher precedence than OR
    sus bool_result1 lit = a || b && c  fr fr Should be true (a || (b && c))
    sus bool_result2 lit = (a || b) && c fr fr Should be true
    
    vibez.spill("true || false && true = {}", bool_result1)
    vibez.spill("(true || false) && true = {}", bool_result2)
    
    fr fr Comparison precedence
    sus x drip = 5
    sus y drip = 10
    sus z drip = 3
    
    sus comp_result lit = x + z > y - 2  fr fr Should be false (8 > 8 is false)
    vibez.spill("5 + 3 > 10 - 2 = {}", comp_result)
    
    damn 0
}
