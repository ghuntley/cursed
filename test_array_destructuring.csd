yeet "testz"

slay test_array_destructuring() {
    test_start("array destructuring tests")
    
    // Basic array destructuring
    sus arr1 = [1, 2, 3]
    (a, b, c) := arr1
    assert_eq_int(a, 1)
    assert_eq_int(b, 2)
    assert_eq_int(c, 3)
    
    // Array destructuring with rest
    sus arr2 = [10, 20, 30, 40, 50]
    (first, second, ...rest) := arr2
    assert_eq_int(first, 10)
    assert_eq_int(second, 20)
    assert_eq_int(rest.length, 3)
    
    // Nested array destructuring
    sus nested = [[1, 2], [3, 4], [5, 6]]
    (pair1, pair2, pair3) := nested
    (x1, y1) := pair1
    (x2, y2) := pair2
    assert_eq_int(x1, 1)
    assert_eq_int(y1, 2)
    assert_eq_int(x2, 3)
    assert_eq_int(y2, 4)
    
    print_test_summary()
}

test_array_destructuring()
