yeet "testz"

test_start("Complete Control Structures Test")

# Test all comparison operators
sus a drip = 5
sus b drip = 10
sus c drip = 5

# Less than
lowkey a < b {
    vibez.spill("5 < 10: true")
    assert_true(based)
}

# Greater than  
lowkey b > a {
    vibez.spill("10 > 5: true")
    assert_true(based)
}

# Less than or equal
lowkey a <= c {
    vibez.spill("5 <= 5: true")
    assert_true(based)
}

# Greater than or equal
lowkey c >= a {
    vibez.spill("5 >= 5: true")
    assert_true(based)
}

# Equal
lowkey a == c {
    vibez.spill("5 == 5: true")
    assert_true(based)
}

# Not equal
lowkey a != b {
    vibez.spill("5 != 10: true")
    assert_true(based)
}

# Test boolean operators
lowkey a < b && c == a {
    vibez.spill("AND operator works")
    assert_true(based)
}

lowkey a > b || c == a {
    vibez.spill("OR operator works") 
    assert_true(based)
}

lowkey !(a > b) {
    vibez.spill("NOT operator works")
    assert_true(based)
}

# Test nested control structures
sus nested_result drip = 0
bestie outer := 0; outer < 3; outer++ {
    bestie inner := 0; inner < 3; inner++ {
        lowkey outer == 1 && inner == 1 {
            nested_result = nested_result + 10
        } highkey lowkey outer == 2 {
            nested_result++
        } highkey {
            nested_result = nested_result + 100  
        }
    }
}
vibez.spill("Nested control structures work")

# Test complex for loop with multiple conditions
sus complex_result drip = 0
bestie i := 0; i < 10; i++ {
    lowkey i % 2 == 0 {
        lowkey i > 6 {
            ghosted
        }
        complex_result = complex_result + i
    } highkey {
        simp
    }
}
assert_eq_int(complex_result, 12) # 0 + 2 + 4 + 6

# Test while loop with complex condition
sus x drip = 1
sus y drip = 1
periodt x < 100 && y < 50 {
    x = x * 2
    y = y + 5
}
vibez.spill("Complex while condition works")

# Test variable scoping in blocks
sus scope_test drip = 1
lowkey based {
    sus scope_test drip = 2
    lowkey scope_test == 2 {
        vibez.spill("Inner scope works")
        assert_true(based)
        {
            sus scope_test drip = 3
            assert_eq_int(scope_test, 3)
        }
        assert_eq_int(scope_test, 2)
    }
}
assert_eq_int(scope_test, 1)

print_test_summary()
