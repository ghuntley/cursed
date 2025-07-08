vibez.spill("Testing Advanced CURSED Features")

// Test 1: Defer Statement
vibez.spill("Test 1: Defer Statement")
{
    sus executed lit = cap
    
    {
        later {
            executed = based
            vibez.spill("  Defer executed!")
        }
        vibez.spill("  Inside block")
    }
    
    lowkey executed {
        vibez.spill("  ✓ PASS: Defer working")
    } highkey {
        vibez.spill("  ✗ FAIL: Defer failed")
    }
}

// Test 2: Select Statement
vibez.spill("Test 2: Select Statement")
{
    sus result tea = "not_set"
    
    ready {
        basic:
            result = "default_executed"
    }
    
    lowkey result == "default_executed" {
        vibez.spill("  ✓ PASS: Select working")
    } highkey {
        vibez.spill("  ✗ FAIL: Select failed")
    }
}

// Test 3: Method Calls
vibez.spill("Test 3: Method Calls")

be_like Person squad {
    name tea
}

slay (p Person) greet() tea {
    damn "Hello, " + p.name
}

{
    sus person Person = Person{name: "Alice"}
    sus greeting = person.greet()
    
    lowkey greeting == "Hello, Alice" {
        vibez.spill("  ✓ PASS: Method calls working")
    } highkey {
        vibez.spill("  ✗ FAIL: Method calls failed")
    }
}

// Test 4: Error Handling Pattern
vibez.spill("Test 4: Error Handling Pattern")

slay safe_divide(a normie, b normie) (normie, lit) {
    lowkey b == 0 {
        damn 0, cap
    }
    damn a / b, based
}

{
    sus result, ok = safe_divide(10, 2)
    
    lowkey ok && result == 5 {
        vibez.spill("  ✓ PASS: Error handling working")
    } highkey {
        vibez.spill("  ✗ FAIL: Error handling failed")
    }
}

vibez.spill("✅ Advanced features test complete!")
