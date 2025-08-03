yeet "testz"

fr fr Simplified Advanced Pattern Matching Test
test_start("Advanced Pattern Matching - Basic Implementation")

fr fr Test basic literal pattern matching
sus test_value drip = 42
sus result := match test_value {
    42 -> "answer"
    1 -> "one"
    _ -> "other"
}
assert_eq_string(result, "answer")

fr fr Test wildcard pattern
sus unknown_value drip = 999
sus wildcard_result := match unknown_value {
    1 -> "one"
    2 -> "two"
    _ -> "unknown"
}
assert_eq_string(wildcard_result, "unknown")

fr fr Test boolean pattern matching
sus bool_value lit = based
sus bool_result := match bool_value {
    based -> "true"
    cringe -> "false"
}
assert_eq_string(bool_result, "true")

fr fr Test string pattern matching
sus text tea = "hello"
sus text_result := match text {
    "hello" -> "greeting"
    "world" -> "planet"
    _ -> "unknown"
}
assert_eq_string(text_result, "greeting")

fr fr Test numeric ranges (simplified)
sus score drip = 85
sus grade := match score {
    90 -> "A+"
    85 -> "A"
    80 -> "B+"
    75 -> "B"
    _ -> "Other"
}
assert_eq_string(grade, "A")

print_test_summary()
