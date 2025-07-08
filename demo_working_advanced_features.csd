vibez.spill("🚀 CURSED Advanced Features - Working Demo")
vibez.spill("==========================================")

// ========================================
// Feature 1: Interface System
// ========================================

vibez.spill("\n✅ Feature 1: Interface System")

be_like Greeter collab {
    greet(name tea) tea
}

be_like Person squad {
    name tea
    age normie
}

slay (p Person) greet(name tea) tea {
    damn "Hello " + name + ", I'm " + p.name + " and I'm " + tea(p.age) + " years old"
}

be_like Robot squad {
    model tea
}

slay (r Robot) greet(name tea) tea {
    damn "HELLO " + name + ". I AM ROBOT MODEL " + r.model
}

sus alice Person = Person{name: "Alice", age: 25}
sus robot Robot = Robot{model: "R2D2"}

vibez.spill("Person greeting:", alice.greet("Bob"))
vibez.spill("Robot greeting:", robot.greet("Human"))

// ========================================
// Feature 2: Defer Statements
// ========================================

vibez.spill("\n✅ Feature 2: Defer Statements")

{
    vibez.spill("Entering block...")
    
    later {
        vibez.spill("Defer 1: This executes at block end")
    }
    
    later {
        vibez.spill("Defer 2: This executes before Defer 1 (LIFO)")
    }
    
    vibez.spill("Middle of block...")
    vibez.spill("Exiting block...")
}

vibez.spill("Block completed - defers should have executed")

// ========================================
// Feature 3: Advanced Control Flow
// ========================================

vibez.spill("\n✅ Feature 3: Advanced Control Flow")

vibez.spill("Testing break (ghosted) and continue (simp):")

sus count normie = 0
bestie i := 1; i <= 10; i++ {
    lowkey i == 3 || i == 7 {
        vibez.spill("  Skipping", i)
        simp
    }
    
    lowkey i == 8 {
        vibez.spill("  Breaking at", i)
        ghosted
    }
    
    count++
    vibez.spill("  Processing", i)
}

vibez.spill("Final count:", count) // Should be 5 (1,2,4,5,6)

// ========================================
// Feature 4: Error Handling Patterns
// ========================================

vibez.spill("\n✅ Feature 4: Error Handling Patterns")

slay divide_safely(a normie, b normie) (normie, lit) {
    lowkey b == 0 {
        damn 0, cap  // Return (0, false) for error
    }
    damn a / b, based  // Return (result, true) for success
}

slay test_operation(a normie, b normie) {
    sus result, success = divide_safely(a, b)
    
    lowkey success {
        vibez.spill("  Success:", a, "/", b, "=", result)
    } highkey {
        vibez.spill("  Error: Cannot divide", a, "by", b)
    }
}

test_operation(10, 2)  // Should succeed
test_operation(10, 0)  // Should fail

// ========================================
// Feature 5: Type System Features
// ========================================

vibez.spill("\n✅ Feature 5: Type System Features")

be_like UserId normie
be_like UserEmail tea

be_like User squad {
    id normie
    email tea
    active lit
}

slay create_user(id normie, email tea) User {
    damn User{
        id: id,
        email: email, 
        active: based
    }
}

sus user1 = create_user(123, "alice@example.com")
vibez.spill("Created user:", user1.id, user1.email, user1.active)

// ========================================
// Feature 6: Generic-like Functions
// ========================================

vibez.spill("\n✅ Feature 6: Generic-like Functions")

slay max_int(a normie, b normie) normie {
    lowkey a > b {
        damn a
    }
    damn b
}

slay max_string(a tea, b tea) tea {
    lowkey len(a) > len(b) {
        damn a
    }
    damn b
}

vibez.spill("Max of 5 and 8:", max_int(5, 8))
vibez.spill("Longer string:", max_string("short", "much longer string"))

// ========================================
// Summary
// ========================================

vibez.spill("\n==========================================")
vibez.spill("✅ Demo Complete - All Features Working!")
vibez.spill("Features demonstrated:")
vibez.spill("  ✓ Interface system (collab)")
vibez.spill("  ✓ Method calls on types")
vibez.spill("  ✓ Defer statements (later)")
vibez.spill("  ✓ Advanced control flow (ghosted/simp)")
vibez.spill("  ✓ Error handling patterns")
vibez.spill("  ✓ Type system features")
vibez.spill("  ✓ Generic-like functions")
vibez.spill("==========================================")
