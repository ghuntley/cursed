yeet "vibez"
yeet "testz"

# Test exhaustiveness checking for pattern matching

squad Status {
    variant Success
    variant Error(tea)  # error message
    variant Pending
}

squad Option<T> {
    variant Some(T)
    variant None
}

slay test_enum_exhaustiveness() lit {
    test_start("Enum Exhaustiveness Checking")
    
    sus status Status = Status.Success
    sus handled lit = cringe
    
    # Complete exhaustive match
    vibe_check status {
        mood Status.Success: handled = based
        mood Status.Error(msg): handled = based
        mood Status.Pending: handled = based
    }
    assert_true(handled)
    
    # Test with Option enum
    sus maybe Option<drip> = Option.Some(42)
    sus value drip = 0
    vibe_check maybe {
        mood Option.Some(x): value = x
        mood Option.None: value = -1
    }
    assert_eq_int(value, 42)
    
    damn based
}

slay test_boolean_exhaustiveness() lit {
    test_start("Boolean Exhaustiveness Checking")
    
    sus flag lit = based
    sus result tea = "unmatched"
    
    # Exhaustive boolean match
    vibe_check flag {
        mood based: result = "true"
        mood cringe: result = "false"
    }
    assert_eq_string(result, "true")
    
    damn based
}

slay test_integer_range_exhaustiveness() lit {
    test_start("Integer Range Exhaustiveness")
    
    sus grade drip = 85
    sus letter tea = "F"
    
    # Range-based exhaustive match
    vibe_check grade {
        mood 90..100: letter = "A"
        mood 80..89: letter = "B"
        mood 70..79: letter = "C"
        mood 60..69: letter = "D"
        mood 0..59: letter = "F"
        # This should be exhaustive for 0-100 range
    }
    assert_eq_string(letter, "B")
    
    damn based
}

slay test_wildcard_exhaustiveness() lit {
    test_start("Wildcard Exhaustiveness")
    
    sus any_number drip = 12345
    sus caught lit = cringe
    
    # Wildcard makes any match exhaustive
    vibe_check any_number {
        mood 1: caught = based
        mood 2: caught = based
        basic: caught = based  # Wildcard catches all other cases
    }
    assert_true(caught)
    
    damn based
}

slay test_tuple_exhaustiveness() lit {
    test_start("Tuple Exhaustiveness")
    
    sus coordinates (lit, lit) = (based, cringe)
    sus quadrant drip = 0
    
    # Exhaustive match for boolean tuple
    vibe_check coordinates {
        mood (based, based): quadrant = 1
        mood (cringe, based): quadrant = 2
        mood (cringe, cringe): quadrant = 3
        mood (based, cringe): quadrant = 4
    }
    assert_eq_int(quadrant, 4)
    
    damn based
}

slay test_guard_exhaustiveness() lit {
    test_start("Guard Pattern Exhaustiveness")
    
    sus number drip = 15
    sus category tea = "unknown"
    
    # Guards with final catch-all
    vibe_check number {
        mood x when x < 10: category = "small"
        mood x when x >= 10 && x < 20: category = "medium"
        mood x when x >= 20: category = "large"
        # Guards with complete coverage
    }
    assert_eq_string(category, "medium")
    
    damn based
}

slay test_array_exhaustiveness() lit {
    test_start("Array Pattern Exhaustiveness")
    
    sus items []drip = [1, 2, 3]
    sus type tea = "unknown"
    
    # Array patterns with rest element for exhaustiveness
    vibe_check items {
        mood []: type = "empty"
        mood [x]: type = "single"
        mood [x, y]: type = "pair"
        mood [...]: type = "multiple"  # Rest pattern catches all other cases
    }
    assert_eq_string(type, "multiple")
    
    damn based
}

squad Color {
    variant Red
    variant Green  
    variant Blue
    variant Yellow
    variant Custom(drip, drip, drip)
}

slay test_partial_enum_exhaustiveness() lit {
    test_start("Partial Enum Exhaustiveness Warning")
    
    sus color Color = Color.Yellow
    sus name tea = "unknown"
    
    # Non-exhaustive match (missing some variants)
    vibe_check color {
        mood Color.Red: name = "red"
        mood Color.Green: name = "green"
        mood Color.Blue: name = "blue"
        # Missing Yellow and Custom - compiler should warn
        basic: name = "other"  # Wildcard prevents runtime error
    }
    assert_eq_string(name, "other")
    
    damn based
}

slay test_nested_exhaustiveness() lit {
    test_start("Nested Pattern Exhaustiveness")
    
    sus nested_option Option<Status> = Option.Some(Status.Success)
    sus result tea = "unmatched"
    
    # Nested exhaustive matching
    vibe_check nested_option {
        mood Option.Some(Status.Success): result = "success"
        mood Option.Some(Status.Error(msg)): result = "error: " + msg
        mood Option.Some(Status.Pending): result = "pending"
        mood Option.None: result = "none"
    }
    assert_eq_string(result, "success")
    
    damn based
}

slay test_or_pattern_exhaustiveness() lit {
    test_start("OR Pattern Exhaustiveness")
    
    sus day tea = "Saturday"
    sus type tea = "unknown"
    
    # OR patterns for exhaustiveness
    vibe_check day {
        mood "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday": type = "weekday"
        mood "Saturday" | "Sunday": type = "weekend"
        basic: type = "invalid"  # Catch-all for exhaustiveness
    }
    assert_eq_string(type, "weekend")
    
    damn based
}

slay main() drip {
    vibez.spill("=== Exhaustiveness Checking Test Suite ===")
    
    test_enum_exhaustiveness()
    test_boolean_exhaustiveness()
    test_integer_range_exhaustiveness()
    test_wildcard_exhaustiveness()
    test_tuple_exhaustiveness()
    test_guard_exhaustiveness()
    test_array_exhaustiveness()
    test_partial_enum_exhaustiveness()
    test_nested_exhaustiveness()
    test_or_pattern_exhaustiveness()
    
    print_test_summary()
    
    vibez.spill("=== Exhaustiveness Checking Complete ===")
    damn 0
}
