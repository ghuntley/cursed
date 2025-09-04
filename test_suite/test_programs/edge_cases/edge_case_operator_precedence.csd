vibe main

fr fr Test: Operator precedence edge cases
fr fr Purpose: Test complex precedence scenarios
fr fr Expected: Operators should follow correct precedence rules

slay main_character() {
    fr fr Basic precedence: * and / before + and -
    sus result1 normie = 2 + 3 * 4        fr fr Should be 14
    sus result2 normie = 10 - 8 / 2       fr fr Should be 6
    sus result3 normie = 3 * 4 + 2 * 5    fr fr Should be 22
    
    fr fr Parentheses override precedence
    sus result4 normie = (2 + 3) * 4      fr fr Should be 20
    sus result5 normie = 2 * (3 + 4)      fr fr Should be 14
    sus result6 normie = (10 - 2) / (3 + 1)  fr fr Should be 2
    
    fr fr Comparison operators
    sus comp1 lit = 5 + 3 > 2 * 4         fr fr 8 > 8 = cringe
    sus comp2 lit = 10 / 2 == 15 - 10     fr fr 5 == 5 = based
    sus comp3 lit = 3 * 3 >= 8 + 1        fr fr 9 >= 9 = based
    
    fr fr Boolean operators with comparisons
    sus bool1 lit = 5 > 3 && 2 < 4        fr fr based && based = based
    sus bool2 lit = 10 < 5 || 3 > 1       fr fr cringe || based = based
    sus bool3 lit = !(5 == 3)             fr fr !(cringe) = based
    
    fr fr Mixed precedence
    sus complex1 normie = 2 + 3 * 4 - 1   fr fr 2 + 12 - 1 = 13
    sus complex2 lit = 2 * 3 + 1 > 5 && 10 / 2 == 5  fr fr based && based = based
    
    damn 0
}
