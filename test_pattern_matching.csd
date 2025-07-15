yeet "testz"

slay test_pattern_matching() {
    test_start("pattern matching tests")
    
    // Test array pattern matching
    sus arr = [1, 2, 3, 4, 5]
    lowkey arr vibes [first, second, ...rest] {
        assert_eq_int(first, 1)
        assert_eq_int(second, 2)
        assert_eq_int(rest.length, 3)
        vibez.spill("Array pattern matching works!")
    }
    
    // Test struct pattern matching
    sus person = Person { name: "Alice", age: 30, city: "NYC" }
    lowkey person vibes Person { name, age, .. } {
        assert_eq_string(name, "Alice")
        assert_eq_int(age, 30)
        vibez.spill("Struct pattern matching works!")
    }
    
    print_test_summary()
}

// Test struct definition
squad Person {
    name tea
    age normie
    city tea
}

test_pattern_matching()
