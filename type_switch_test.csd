yeet "testz"

# Test basic type switch functionality
test_start("Basic type switch")

# Test with integer
sus x normie = 42
sus result tea = typecheck x is {
    normie -> "found integer"
    tea -> "found string"
    _ -> "found other"
}
assert_eq_string(result, "found integer")

# Test with string
sus y tea = "hello"
sus result2 tea = typecheck y is {
    normie -> "found integer"
    tea -> "found string"
    _ -> "found other"
}
assert_eq_string(result2, "found string")

# Test with boolean
sus z lit = based
sus result3 tea = typecheck z is {
    normie -> "found integer"
    tea -> "found string"
    lit -> "found boolean"
    _ -> "found other"
}
assert_eq_string(result3, "found boolean")

# Test with bound variable
sus w normie = 100
sus result4 normie = typecheck w is {
    normie value -> value * 2
    _ -> 0
}
assert_eq_int(result4, 200)

print_test_summary()
