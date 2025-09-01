vibe main;

yeet "vibez";
yeet "mathz";

fr fr Function Definition Validation Test
fr fr Tests: Proper CURSED function syntax and calling conventions
fr fr Expected: Functions work identically in both modes

fr fr Function with single parameter
slay calculate_square(x drip normie) drip normie {
    damn x * x
}

fr fr Function with multiple parameters
slay add_three_numbers(a drip normie, b drip normie, c drip normie) drip normie {
    damn a + b + c
}

fr fr Function using stdlib
slay double_absolute(value drip normie) drip normie {
    sus abs_value drip normie = mathz.abs(value)
    damn abs_value * 2
}

fr fr Function with complex logic
slay grade_calculator(score drip normie) drip normie {
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
    sus square_result drip normie = calculate_square(5)
    vibez.spill(square_result)
    
    vibez.spill("Testing add_three_numbers(2, 3, 4):")
    sus sum_result drip normie = add_three_numbers(2, 3, 4)
    vibez.spill(sum_result)
    
    vibez.spill("Testing double_absolute(-15):")
    sus double_abs_result drip normie = double_absolute(-15)
    vibez.spill(double_abs_result)
    
    vibez.spill("Testing grade_calculator(85):")
    sus grade_result drip normie = grade_calculator(85)
    vibez.spill(grade_result)
    
    vibez.spill("Testing nested function calls:")
    sus nested_result drip normie = calculate_square(add_three_numbers(1, 2, 3))
    vibez.spill(nested_result)
    
    vibez.spill("=== Test Complete ===")
}
