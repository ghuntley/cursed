yeet "testz"

# Test basic match expressions
slay test_basic_match() lit {
    sus value normie = 42
    sus result tea = hella value {
        1 -> "one"
        42 -> "forty-two"
        _ -> "other"
    }
    damn result == "forty-two"
}

# Test match with variable binding
slay test_match_variable_binding() lit {
    sus value normie = 100
    sus result normie = hella value {
        x -> x * 2
    }
    damn result == 200
}

# Test match with range patterns
slay test_match_range_patterns() lit {
    sus value normie = 15
    sus result tea = hella value {
        1..10 -> "small"
        11..20 -> "medium"
        21..30 -> "large"
        _ -> "other"
    }
    damn result == "medium"
}

# Test match with tuple patterns
slay test_match_tuple_patterns() lit {
    sus pair := (1, 2)
    sus result normie = hella pair {
        (1, 2) -> 3
        (x, y) -> x + y
        _ -> 0
    }
    damn result == 3
}

# Test match with or patterns
slay test_match_or_patterns() lit {
    sus value normie = 5
    sus result tea = hella value {
        1 | 2 | 3 -> "low"
        4 | 5 | 6 -> "medium"
        _ -> "high"
    }
    damn result == "medium"
}

# Test match with boolean patterns
slay test_match_boolean_patterns() lit {
    sus flag lit = based
    sus result tea = hella flag {
        based -> "true"
        cap -> "false"
    }
    damn result == "true"
}

# Test match with string patterns
slay test_match_string_patterns() lit {
    sus greeting tea = "hello"
    sus result tea = hella greeting {
        "hello" -> "greeting"
        "goodbye" -> "farewell"
        _ -> "unknown"
    }
    damn result == "greeting"
}

# Test comprehensive match expression
slay test_comprehensive_match() lit {
    sus values := [(1, "one"), (2, "two"), (3, "three")]
    sus results := []
    
    bestie i := 0; i < values.length; i++ {
        sus current := values[i]
        sus result := hella current {
            (1, msg) -> "first: " + msg
            (2, msg) -> "second: " + msg
            (n, msg) -> "other: " + msg
            _ -> "unknown"
        }
        results.push(result)
    }
    
    damn results.length == 3
}

# Run all tests
test_start("Match Expression Tests")
assert_true(test_basic_match())
assert_true(test_match_variable_binding())
assert_true(test_match_range_patterns())
assert_true(test_match_tuple_patterns())
assert_true(test_match_or_patterns())
assert_true(test_match_boolean_patterns())
assert_true(test_match_string_patterns())
assert_true(test_comprehensive_match())
print_test_summary()
