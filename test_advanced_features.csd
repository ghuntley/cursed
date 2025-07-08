yeet "testz"

// ========================================
// Defer Statement Tests (using 'later')
// ========================================

slay test_defer_basic() {
    test_start("Defer Basic Functionality")
    
    sus executed lit = cap
    
    {
        later {
            executed = based
        }
        vibez.spill("Inside block")
    }
    
    assert_true(executed)
    print_test_summary()
}

// ========================================
// Select Statement Tests (using 'ready')
// ========================================

slay test_select_basic() {
    test_start("Select Statement Basic")
    
    sus result tea = "not_set"
    
    ready {
        basic:
            result = "default_executed"
    }
    
    assert_eq_string(result, "default_executed")
    print_test_summary()
}

// ========================================
// Generic-like Functions
// ========================================

slay identity_int(value normie) normie {
    damn value
}

slay identity_string(value tea) tea {
    damn value
}

slay test_identity_functions() {
    test_start("Identity Functions")
    
    sus int_result = identity_int(42)
    sus string_result = identity_string("hello")
    
    assert_eq_int(int_result, 42)
    assert_eq_string(string_result, "hello")
    print_test_summary()
}

// ========================================
// Stack Data Structure (Generic-like)
// ========================================

be_like IntStack squad {
    items []normie
    size normie
}

slay (s @IntStack) push(item normie) {
    // Simple push implementation - in real implementation would resize array
    s.items[s.size] = item
    s.size++
}

slay (s @IntStack) pop() (normie, lit) {
    vibe_check s.size == 0 {
        damn 0, cap
    }
    
    s.size--
    damn s.items[s.size], based
}

slay (s @IntStack) peek() (normie, lit) {
    vibe_check s.size == 0 {
        damn 0, cap
    }
    
    damn s.items[s.size-1], based
}

slay test_stack_operations() {
    test_start("Stack Operations")
    
    sus stack IntStack
    stack.size = 0
    
    stack.push(10)
    stack.push(20)
    stack.push(30)
    
    assert_eq_int(stack.size, 3)
    
    sus top, ok = stack.peek()
    assert_true(ok)
    assert_eq_int(top, 30)
    
    sus popped, ok = stack.pop()
    assert_true(ok)
    assert_eq_int(popped, 30)
    assert_eq_int(stack.size, 2)
    
    print_test_summary()
}

// ========================================
// Interface-like Behavior
// ========================================

be_like Greeter collab {
    greet(name tea) tea
}

be_like Person squad {
    name tea
    age normie
}

slay (p Person) greet(name tea) tea {
    damn "Hello " + name + ", I'm " + p.name
}

be_like Robot squad {
    model tea
    version normie
}

slay (r Robot) greet(name tea) tea {
    damn "HELLO " + name + ". I AM " + r.model + " VERSION " + tea(r.version)
}

slay test_interface_behavior() {
    test_start("Interface-like Behavior")
    
    sus person Person = Person{name: "Alice", age: 30}
    sus robot Robot = Robot{model: "R2D2", version: 2}
    
    sus greeting1 = person.greet("Bob")
    sus greeting2 = robot.greet("World")
    
    assert_eq_string(greeting1, "Hello Bob, I'm Alice")
    assert_eq_string(greeting2, "HELLO World. I AM R2D2 VERSION 2")
    
    print_test_summary()
}

// ========================================
// Type System Features
// ========================================

// Type aliases (newtype pattern)
be_like UserId normie
be_like UserName tea

be_like User squad {
    id normie
    name tea
    email tea
}

slay test_type_aliases() {
    test_start("Type Aliases")
    
    sus user_id normie = 123
    sus user_name tea = "John Doe"
    
    sus user User = User{
        id: user_id,
        name: user_name,
        email: "john@example.com"
    }
    
    assert_eq_int(user.id, 123)
    assert_eq_string(user.name, "John Doe")
    assert_eq_string(user.email, "john@example.com")
    
    print_test_summary()
}

// ========================================
// Error Handling Pattern
// ========================================

slay divide_safe(a normie, b normie) (normie, lit) {
    vibe_check b == 0 {
        damn 0, cap  // Return error indicator
    }
    damn a / b, based  // Return success
}

slay test_error_handling_pattern() {
    test_start("Error Handling Pattern")
    
    sus result1, ok1 = divide_safe(10, 2)
    assert_true(ok1)
    assert_eq_int(result1, 5)
    
    sus result2, ok2 = divide_safe(10, 0)
    assert_false(ok2)
    assert_eq_int(result2, 0)
    
    print_test_summary()
}

// ========================================
// Advanced Control Flow
// ========================================

slay test_advanced_control_flow() {
    test_start("Advanced Control Flow")
    
    sus values []normie = []normie{1, 2, 3, 4, 5}
    sus sum normie = 0
    
    // For loop with break and continue
    bestie i := 0; i < len(values); i++ {
        vibe_check values[i] == 3 {
            simp  // continue
        }
        vibe_check values[i] == 5 {
            ghosted  // break
        }
        sum = sum + values[i]
    }
    
    assert_eq_int(sum, 3)  // 1 + 2 = 3 (skipped 3, broke at 5)
    
    print_test_summary()
}

// ========================================
// Memory and Resource Management
// ========================================

slay test_resource_management() {
    test_start("Resource Management with Defer")
    
    sus resource_cleaned lit = cap
    
    {
        sus resource normie = 42
        later {
            resource_cleaned = based
            vibez.spill("Resource cleaned:", resource)
        }
        
        vibez.spill("Using resource:", resource)
    }
    
    assert_true(resource_cleaned)
    print_test_summary()
}

// ========================================
// Test Runner
// ========================================

vibez.spill("🚀 Starting Advanced Language Features Tests")
vibez.spill("=========================================")

test_defer_basic()
test_select_basic()
test_identity_functions()
test_stack_operations()
test_interface_behavior()
test_type_aliases()
test_error_handling_pattern()
test_advanced_control_flow()
test_resource_management()

vibez.spill("=========================================")
vibez.spill("✅ All advanced language features tested!")
