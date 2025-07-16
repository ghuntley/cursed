yeet "testz"

// Test literal patterns
test_start("Literal pattern matching")

sus test_int normie = 42
sus result1 := match test_int {
    42 -> "answer"
    1 -> "one"
    _ -> "other"
}
assert_eq_string(result1, "answer")

sus test_bool lit = based
sus result2 := match test_bool {
    based -> "true case"
    cap -> "false case"
}
assert_eq_string(result2, "true case")

sus test_string tea = "hello"
sus result3 := match test_string {
    "hello" -> "greeting"
    "world" -> "planet"
    _ -> "unknown"
}
assert_eq_string(result3, "greeting")

// Test range patterns
sus test_range normie = 15
sus result4 := match test_range {
    1..10 -> "small"
    11..20 -> "medium"
    21..30 -> "large"
    _ -> "unknown"
}
assert_eq_string(result4, "medium")

// Test wildcard pattern
sus test_wildcard normie = 999
sus result5 := match test_wildcard {
    1 -> "one"
    2 -> "two"
    _ -> "everything else"
}
assert_eq_string(result5, "everything else")

// Test variable binding
sus test_variable normie = 123
sus result6 := match test_variable {
    x when x > 100 -> "large number"
    x -> "small number"
}
assert_eq_string(result6, "large number")

// Test or patterns
sus test_or normie = 2
sus result7 := match test_or {
    1 | 2 | 3 -> "small"
    4 | 5 | 6 -> "medium"
    _ -> "large"
}
assert_eq_string(result7, "small")

print_test_summary()
