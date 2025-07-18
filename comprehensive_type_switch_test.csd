yeet "testz"

# Comprehensive type switch testing
test_start("Comprehensive type switch tests")

# Test 1: Basic primitive type matching
sus x normie = 42
sus result1 tea = typecheck x is {
    normie -> "integer"
    tea -> "string"
    _ -> "unknown"
}
assert_eq_string(result1, "integer")

# Test 2: String type matching
sus y tea = "hello"
sus result2 tea = typecheck y is {
    normie -> "integer"
    tea -> "string"
    _ -> "unknown"
}
assert_eq_string(result2, "string")

# Test 3: Boolean type matching
sus z lit = based
sus result3 tea = typecheck z is {
    lit -> "boolean"
    _ -> "unknown"
}
assert_eq_string(result3, "boolean")

# Test 4: Character type matching
sus ch sip = 'A'
sus result4 tea = typecheck ch is {
    sip -> "character"
    _ -> "unknown"
}
assert_eq_string(result4, "character")

# Test 5: Variable binding in type switch
sus value normie = 100
sus doubled normie = typecheck value is {
    normie num -> num * 2
    _ -> 0
}
assert_eq_int(doubled, 200)

# Test 6: Wildcard pattern
sus any_value tea = "test"
sus wildcard_result tea = typecheck any_value is {
    normie -> "integer"
    _ -> "wildcard matched"
}
assert_eq_string(wildcard_result, "wildcard matched")

# Test 7: Multiple type checking
sus float_val meal = 3.14
sus float_result tea = typecheck float_val is {
    normie -> "integer"
    tea -> "string"
    meal -> "float"
    _ -> "unknown"
}
assert_eq_string(float_result, "float")

# Test 8: Bound variable with different types
sus str_val tea = "bound"
sus bound_result tea = typecheck str_val is {
    normie n -> "integer: " + n
    tea s -> "string: " + s
    _ -> "unknown"
}
assert_eq_string(bound_result, "string: bound")

print_test_summary()
