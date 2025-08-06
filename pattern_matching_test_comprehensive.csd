yeet "vibez"
yeet "testz"

slay test_literal_patterns() lit {
    test_start("Literal Pattern Matching")
    
    // Integer patterns
    sus value drip = 42
    sus result lit = cringe
    vibe_check value {
        mood 10: result = cringe
        mood 42: result = based
        mood 100: result = cringe
        basic: result = cringe
    }
    assert_true(result)
    
    // String patterns  
    sus message tea = "hello"
    sus matched lit = cringe
    vibe_check message {
        mood "goodbye": matched = cringe
        mood "hello": matched = based
        mood "world": matched = cringe
        basic: matched = cringe
    }
    assert_true(matched)
    
    // Boolean patterns
    sus flag lit = based
    sus bool_matched lit = cringe
    vibe_check flag {
        mood cringe: bool_matched = cringe
        mood based: bool_matched = based
        basic: bool_matched = cringe
    }
    assert_true(bool_matched)
    
    damn based
}

slay test_variable_patterns() lit {
    test_start("Variable Pattern Matching")
    
    sus value drip = 123
    sus bound_value drip = 0
    vibe_check value {
        mood x: bound_value = x
    }
    assert_eq_int(bound_value, 123)
    
    damn based
}

slay test_wildcard_patterns() lit {
    test_start("Wildcard Pattern Matching")
    
    sus any_value drip = 999
    sus caught lit = cringe
    vibe_check any_value {
        mood _: caught = based
    }
    assert_true(caught)
    
    damn based
}

slay test_guard_patterns() lit {
    test_start("Guard Pattern Matching")
    
    sus number drip = 15
    sus guard_result tea = "none"
    vibe_check number {
        mood x when x > 10 && x < 20: guard_result = "medium"
        mood x when x > 20: guard_result = "large"
        mood x: guard_result = "small"
    }
    assert_eq_string(guard_result, "medium")
    
    damn based
}

slay test_range_patterns() lit {
    test_start("Range Pattern Matching")
    
    sus score drip = 85
    sus grade tea = "F"
    vibe_check score {
        mood 90..100: grade = "A"
        mood 80..89: grade = "B"
        mood 70..79: grade = "C"
        mood 60..69: grade = "D"
        basic: grade = "F"
    }
    assert_eq_string(grade, "B")
    
    damn based
}

slay test_or_patterns() lit {
    test_start("OR Pattern Matching")
    
    sus day tea = "Saturday"
    sus is_weekend lit = cringe
    vibe_check day {
        mood "Saturday" | "Sunday": is_weekend = based
        basic: is_weekend = cringe
    }
    assert_true(is_weekend)
    
    damn based
}

slay test_tuple_patterns() lit {
    test_start("Tuple Pattern Matching")
    
    sus point (drip, drip) = (3, 4)
    sus quadrant drip = 0
    vibe_check point {
        mood (x, y) when x > 0 && y > 0: quadrant = 1
        mood (x, y) when x < 0 && y > 0: quadrant = 2
        mood (x, y) when x < 0 && y < 0: quadrant = 3
        mood (x, y) when x > 0 && y < 0: quadrant = 4
        basic: quadrant = 0
    }
    assert_eq_int(quadrant, 1)
    
    damn based
}

slay test_array_patterns() lit {
    test_start("Array Pattern Matching")
    
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus pattern_type tea = "unknown"
    vibe_check numbers {
        mood []: pattern_type = "empty"
        mood [x]: pattern_type = "single"
        mood [x, y]: pattern_type = "pair"
        mood [1, 2, ...rest]: pattern_type = "starts_with_1_2"
        mood [...]: pattern_type = "any_array"
    }
    assert_eq_string(pattern_type, "starts_with_1_2")
    
    damn based
}

squad Color {
    variant Red
    variant Green
    variant Blue
    variant Custom(drip, drip, drip)  # RGB values
}

slay test_enum_patterns() lit {
    test_start("Enum Pattern Matching")
    
    sus color Color = Color.Red
    sus color_name tea = "unknown"
    vibe_check color {
        mood Color.Red: color_name = "red"
        mood Color.Green: color_name = "green"
        mood Color.Blue: color_name = "blue"
        mood Color.Custom(r, g, b): color_name = "custom"
    }
    assert_eq_string(color_name, "red")
    
    sus custom_color Color = Color.Custom(255, 128, 0)
    sus custom_name tea = "unknown"
    vibe_check custom_color {
        mood Color.Custom(255, g, b) when g > 100: custom_name = "bright_orange"
        mood Color.Custom(r, g, b): custom_name = "custom_color"
        basic: custom_name = "other"
    }
    assert_eq_string(custom_name, "bright_orange")
    
    damn based
}

squad Shape {
    variant Circle(drip)  # radius
    variant Rectangle(drip, drip)  # width, height
    variant Triangle(drip, drip, drip)  # sides
}

slay test_complex_enum_patterns() lit {
    test_start("Complex Enum Pattern Matching")
    
    sus shape Shape = Shape.Rectangle(10, 20)
    sus area drip = 0
    vibe_check shape {
        mood Shape.Circle(r): area = 3 * r * r  # Simplified pi
        mood Shape.Rectangle(w, h): area = w * h
        mood Shape.Triangle(a, b, c): area = a + b + c  # Simplified
    }
    assert_eq_int(area, 200)
    
    damn based
}

slay test_nested_patterns() lit {
    test_start("Nested Pattern Matching")
    
    sus nested ((drip, tea), lit) = ((42, "answer"), based)
    sus extracted_number drip = 0
    sus extracted_string tea = ""
    sus extracted_bool lit = cringe
    
    vibe_check nested {
        mood ((num, str), flag): {
            extracted_number = num
            extracted_string = str
            extracted_bool = flag
        }
    }
    
    assert_eq_int(extracted_number, 42)
    assert_eq_string(extracted_string, "answer")
    assert_true(extracted_bool)
    
    damn based
}

slay test_exhaustiveness_checking() lit {
    test_start("Exhaustiveness Checking")
    
    sus bool_val lit = based
    sus result tea = "unmatched"
    
    # This should be exhaustive for boolean type
    vibe_check bool_val {
        mood based: result = "true_case"
        mood cringe: result = "false_case"
    }
    assert_eq_string(result, "true_case")
    
    # Test with integer and wildcard (always exhaustive)
    sus int_val drip = 999
    sus int_result tea = "unmatched"
    vibe_check int_val {
        mood 1: int_result = "one"
        mood 2: int_result = "two"
        basic: int_result = "other"  # Wildcard makes it exhaustive
    }
    assert_eq_string(int_result, "other")
    
    damn based
}

slay test_pattern_matching_optimization() lit {
    test_start("Pattern Matching Optimization")
    
    # Test jump table optimization for many literal cases
    sus day_number drip = 5
    sus day_name tea = "unknown"
    
    vibe_check day_number {
        mood 1: day_name = "Monday"
        mood 2: day_name = "Tuesday"
        mood 3: day_name = "Wednesday"
        mood 4: day_name = "Thursday"
        mood 5: day_name = "Friday"
        mood 6: day_name = "Saturday"
        mood 7: day_name = "Sunday"
        mood 8: day_name = "Extra1"
        mood 9: day_name = "Extra2"
        mood 10: day_name = "Extra3"
        basic: day_name = "Invalid"
    }
    assert_eq_string(day_name, "Friday")
    
    damn based
}

slay main() drip {
    vibez.spill("=== Comprehensive Pattern Matching Test Suite ===")
    
    test_literal_patterns()
    test_variable_patterns()
    test_wildcard_patterns()
    test_guard_patterns()
    test_range_patterns()
    test_or_patterns()
    test_tuple_patterns()
    test_array_patterns()
    test_enum_patterns()
    test_complex_enum_patterns()
    test_nested_patterns()
    test_exhaustiveness_checking()
    test_pattern_matching_optimization()
    
    print_test_summary()
    
    vibez.spill("=== Pattern Matching Test Suite Complete ===")
    damn 0
}
