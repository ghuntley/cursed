yeet "testz"
yeet "vibez"

# Simple memory allocation test
slay test_basic_allocation() {
    test_start("Basic Memory Allocation")
    
    # Test simple array allocation
    sus numbers = [normie]()
    sus i drip = 0
    bestie (i < 100) {
        numbers.append(i)
        i = i + 1
    }
    
    assert_eq_int(numbers.len(), 100)
    vibez.spill("Basic allocation test passed")
    
    print_test_summary()
}

# Test string allocation
slay test_string_allocation() {
    test_start("String Memory Allocation")
    
    sus text tea = "Hello, CURSED!"
    sus repeated tea = text + " " + text
    
    assert_true(repeated.len() > text.len())
    vibez.spill("String allocation test passed")
    
    print_test_summary()
}

# Test nested structure allocation
slay test_nested_allocation() {
    test_start("Nested Structure Allocation")
    
    squad TestStruct {
        spill value normie
        spill name tea
    }
    
    sus items = [TestStruct]()
    sus i drip = 0
    bestie (i < 10) {
        sus item = TestStruct{ value: i, name: "Item " + i.to_string() }
        items.append(item)
        i = i + 1
    }
    
    assert_eq_int(items.len(), 10)
    assert_eq_int(items[0].value, 0)
    assert_eq_string(items[0].name, "Item 0")
    
    vibez.spill("Nested allocation test passed")
    
    print_test_summary()
}

# Run tests
vibez.spill("=== Starting Simple Memory Tests ===")
test_basic_allocation()
test_string_allocation()  
test_nested_allocation()
vibez.spill("=== Simple Memory Tests Completed ===")
