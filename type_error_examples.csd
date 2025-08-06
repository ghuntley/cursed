# Type Error Examples - Testing Comprehensive Type System

yeet "testz"

test_start("Type Error Detection")

# Test 1: Valid type inference
sus valid_int := 42
sus valid_float := 3.14
sus valid_string := "hello"
sus valid_bool := based

assert_true(based)  # This should work

# Test 2: Type compatibility in arithmetic
sus sum := valid_int + 10        # drip + drip = drip (OK)
sus mixed_ok := valid_int + 5    # drip + drip = drip (OK)

# Test 3: Function type checking
slay multiply(a drip, b drip) drip {
    damn a * b
}

sus product := multiply(5, 6)  # Should work
assert_eq_int(product, 30)

# Test 4: Struct field access
squad Person {
    spill name tea
    spill age drip
}

sus person := Person { name: "Alice", age: 25 }
sus person_name := person.name  # Should infer tea
sus person_age := person.age    # Should infer drip

assert_eq_string(person_name, "Alice")
assert_eq_int(person_age, 25)

# Test 5: Array type inference
sus numbers := [1, 2, 3, 4, 5]     # Should infer []drip
sus first := numbers[0]             # Should infer drip
assert_eq_int(first, 1)

# Test 6: Generic function (simplified)
slay get_first[T](arr []T) T {
    damn arr[0]
}

sus first_num := get_first([10, 20, 30])    # T = drip
sus first_str := get_first(["a", "b", "c"]) # T = tea

assert_eq_int(first_num, 10)
assert_eq_string(first_str, "a")

print_test_summary()

vibez.spill("✅ Type system validation complete!")

# Commented out examples that should cause type errors:
# These would fail type checking if uncommented:

# Type mismatch examples (commented out to avoid compilation errors):
# sus bad_add := "hello" + 42         # string + int should fail
# sus bad_call := multiply("a", "b")  # function expects drip, got tea
# sus bad_field := person.height      # field doesn't exist
# sus bad_index := numbers["0"]       # array index must be int, not string
