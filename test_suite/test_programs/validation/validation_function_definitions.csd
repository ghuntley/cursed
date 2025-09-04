vibe main

yeet "vibez";
yeet "mathz";

fr fr Function Definition Validation Test
fr fr Tests: Proper CURSED function syntax and calling conventions
fr fr Expected: Functions work identically in both modes

fr fr Function with single parameter
slay calculate_square(x normie) normie {
    damn x * x
}

fr fr Function with multiple parameters
slay add_three_numbers(a normie, b normie, c normie) normie {
    damn a + b + c
}

fr fr Function using stdlib
slay double_absolute(value normie) normie {
    sus abs_value normie = mathz.abs_normie(value)
    damn abs_value * 2
}

fr fr Function with complex logic
slay grade_calculator(score normie) normie {
    ready score >= 90 {
        damn 1  fr fr A grade
    } otherwise ready score >= 80 {
        damn 2  fr fr B grade
    } otherwise ready score >= 70 {
        damn 3  fr fr C grade
    } otherwise {
        damn 4  fr fr Below C
    }
}

slay main_character() {
    vibez.spill("=== Function Definition Test ===")
    
    vibez.spill("Testing calculate_square(5):")
    sus square_result normie = calculate_square(5)
    vibez.spill(square_result)
    
    vibez.spill("Testing add_three_numbers(2, 3, 4):")
    sus sum_result normie = add_three_numbers(2, 3, 4)
    vibez.spill(sum_result)
    
    vibez.spill("Testing double_absolute(-15):")
    sus double_abs_result normie = double_absolute(-15)
    vibez.spill(double_abs_result)
    
    vibez.spill("Testing grade_calculator(85):")
    sus grade_result normie = grade_calculator(85)
    vibez.spill(grade_result)
    
    vibez.spill("Testing nested function calls:")
    sus nested_result normie = calculate_square(add_three_numbers(1, 2, 3))
    vibez.spill(nested_result)
    
    vibez.spill("=== Test Complete ===")
}
