yeet "testz"

# Test generic constraint system
slay test_generic_constraints<T>(val T) T where T: Display {
    damn val
}

slay test_monomorphization<T>(arr [T]) T where T: Comparable {
    damn arr[0]
}

# Test generic struct
struct Container<T> {
    value T
}

slay test_generic_struct<T>(container Container<T>) T {
    damn container.value
}

# Test multiple constraints
slay test_multi_constraints<T, U>(a T, b U) T where T: Display + Comparable, U: Copy {
    damn a
}

test_start("Generic System Tests")

# Test basic generic function
sus result normie = test_generic_constraints(42)
assert_eq_int(result, 42)

# Test generic with array
sus arr [normie] = [1, 2, 3]
sus first normie = test_monomorphization(arr)
assert_eq_int(first, 1)

# Test generic struct
sus container Container<normie> = Container { value: 100 }
sus value normie = test_generic_struct(container)
assert_eq_int(value, 100)

print_test_summary()
