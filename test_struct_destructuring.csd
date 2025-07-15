yeet "testz"

// Test struct definition
squad Person {
    name tea
    age normie
    city tea
}

squad Point {
    x normie
    y normie
}

slay test_struct_destructuring() {
    test_start("struct destructuring tests")
    
    // Basic struct destructuring
    sus person = Person { name: "Alice", age: 30, city: "NYC" }
    lowkey person vibes Person { name, age, city } {
        assert_eq_string(name, "Alice")
        assert_eq_int(age, 30)
        assert_eq_string(city, "NYC")
        vibez.spill("Basic struct destructuring works!")
    }
    
    // Struct destructuring with rest
    sus person2 = Person { name: "Bob", age: 25, city: "LA" }
    lowkey person2 vibes Person { name, .. } {
        assert_eq_string(name, "Bob")
        vibez.spill("Struct destructuring with rest works!")
    }
    
    // Nested struct destructuring
    sus point = Point { x: 10, y: 20 }
    lowkey point vibes Point { x, y } {
        assert_eq_int(x, 10)
        assert_eq_int(y, 20)
        vibez.spill("Nested struct destructuring works!")
    }
    
    // Struct field renaming in pattern
    sus person3 = Person { name: "Charlie", age: 35, city: "SF" }
    lowkey person3 vibes Person { name: person_name, age: person_age, .. } {
        assert_eq_string(person_name, "Charlie")
        assert_eq_int(person_age, 35)
        vibez.spill("Struct field renaming works!")
    }
    
    print_test_summary()
}

test_struct_destructuring()
