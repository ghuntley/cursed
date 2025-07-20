yeet "testz"

// Test basic literal patterns (implemented)
test_start("Basic pattern matching - literals and wildcards")

sus test_int normie = 42
sus result1 := match test_int {
    42 => "answer"
    1 => "one" 
    _ => "other"
}
assert_eq_string(result1, "answer")

sus test_bool lit = based
sus result2 := match test_bool {
    based => "true case"
    cap => "false case"
}
assert_eq_string(result2, "true case")

sus test_string tea = "hello"
sus result3 := match test_string {
    "hello" => "greeting"
    "world" => "planet"
    _ => "unknown"
}
assert_eq_string(result3, "greeting")

// Test wildcard pattern
sus test_wildcard normie = 999
sus result4 := match test_wildcard {
    1 => "one"
    2 => "two"
    _ => "everything else"
}
assert_eq_string(result4, "everything else")

print_test_summary()
