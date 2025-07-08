// Advanced Language Features Test Suite for CURSED
// Testing without external dependencies

vibez.spill("🚀 CURSED Advanced Language Features Test Suite")
vibez.spill("========================================")

// ========================================
// Test 1: Defer Statement (using 'later')
// ========================================

vibez.spill("Test 1: Defer Statement")
{
    sus executed lit = cap
    
    {
        later {
            executed = based
            vibez.spill("  ✓ Defer executed successfully")
        }
        vibez.spill("  Inside block before defer")
    }
    
    lowkey executed {
        vibez.spill("  ✓ PASS: Defer functionality working")
    } highkey {
        vibez.spill("  ✗ FAIL: Defer not executed")
    }
}

// ========================================
// Test 2: Select Statement (using 'ready')
// ========================================

vibez.spill("\nTest 2: Select Statement")
{
    sus result tea = "not_set"
    
    ready {
        basic:
            result = "default_executed"
    }
    
    lowkey result == "default_executed" {
        vibez.spill("  ✓ PASS: Select statement working")
    } highkey {
        vibez.spill("  ✗ FAIL: Select statement failed")
    }
}

// ========================================
// Test 3: Advanced Control Flow
// ========================================

vibez.spill("\nTest 3: Advanced Control Flow")
{
    sus sum normie = 0
    
    bestie i := 1; i <= 5; i++ {
        lowkey i == 3 {
            simp  // continue
        }
        lowkey i == 5 {
            ghosted  // break
        }
        sum = sum + i
    }
    
    lowkey sum == 3 {  // 1 + 2 = 3 (skipped 3, broke at 5)
        vibez.spill("  ✓ PASS: Break/continue working")
    } highkey {
        vibez.spill("  ✗ FAIL: Break/continue failed, sum =", sum)
    }
}

// ========================================
// Test 4: Interface-like Behavior
// ========================================

vibez.spill("\nTest 4: Interface-like Behavior")

be_like Person squad {
    name tea
    age normie
}

slay (p Person) greet() tea {
    damn "Hello, I'm " + p.name
}

{
    sus person Person = Person{name: "Alice", age: 30}
    sus greeting = person.greet()
    
    lowkey greeting == "Hello, I'm Alice" {
        vibez.spill("  ✓ PASS: Method calls working")
    } highkey {
        vibez.spill("  ✗ FAIL: Method call failed")
    }
}

// ========================================
// Test 5: Error Handling Pattern
// ========================================

vibez.spill("\nTest 5: Error Handling Pattern")

slay divide_safe(a normie, b normie) (normie, lit) {
    lowkey b == 0 {
        damn 0, cap  // Return error indicator
    }
    damn a / b, based  // Return success
}

{
    sus result1, ok1 = divide_safe(10, 2)
    sus result2, ok2 = divide_safe(10, 0)
    
    lowkey ok1 && !ok2 && result1 == 5 {
        vibez.spill("  ✓ PASS: Error handling pattern working")
    } highkey {
        vibez.spill("  ✗ FAIL: Error handling pattern failed")
    }
}

// ========================================
// Test 6: Type System Features
// ========================================

vibez.spill("\nTest 6: Type System Features")

be_like User squad {
    id normie
    name tea
    email tea
}

{
    sus user User = User{
        id: 123,
        name: "John Doe",
        email: "john@example.com"
    }
    
    lowkey user.id == 123 && user.name == "John Doe" {
        vibez.spill("  ✓ PASS: Struct types working")
    } highkey {
        vibez.spill("  ✗ FAIL: Struct types failed")
    }
}

// ========================================
// Test 7: Generic-like Functions
// ========================================

vibez.spill("\nTest 7: Generic-like Functions")

slay identity_int(value normie) normie {
    damn value
}

slay identity_string(value tea) tea {
    damn value
}

{
    sus int_result = identity_int(42)
    sus string_result = identity_string("hello")
    
    lowkey int_result == 42 && string_result == "hello" {
        vibez.spill("  ✓ PASS: Generic-like functions working")
    } highkey {
        vibez.spill("  ✗ FAIL: Generic-like functions failed")
    }
}

// ========================================
// Test 8: Resource Management
// ========================================

vibez.spill("\nTest 8: Resource Management")
{
    sus resource_cleaned lit = cap
    
    {
        sus resource normie = 42
        later {
            resource_cleaned = based
            vibez.spill("  Resource cleaned:", resource)
        }
        
        vibez.spill("  Using resource:", resource)
    }
    
    lowkey resource_cleaned {
        vibez.spill("  ✓ PASS: Resource management working")
    } highkey {
        vibez.spill("  ✗ FAIL: Resource management failed")
    }
}

// ========================================
// Summary
// ========================================

vibez.spill("\n========================================")
vibez.spill("✅ Advanced Language Features Test Complete!")
vibez.spill("🎯 Key Features Tested:")
vibez.spill("   - Defer statements (later)")
vibez.spill("   - Select statements (ready)")
vibez.spill("   - Advanced control flow")
vibez.spill("   - Interface-like behavior")
vibez.spill("   - Error handling patterns")
vibez.spill("   - Type system features")
vibez.spill("   - Generic-like functions")
vibez.spill("   - Resource management")
vibez.spill("========================================")
