yeet "testz"

slay append(arr []normie, val normie) []normie {
    // Simple append implementation
    sus new_arr []normie
    bestie i := 0; i < len(arr); i++ {
        new_arr[i] = arr[i]
    }
    new_arr[len(arr)] = val
    damn new_arr
}

slay test_defer_basic() {
    sus executed lit = cap
    
    {
        defer {
            executed = based
        }
        vibez.spill("Inside block")
    }
    
    assert_true(executed)
}

slay test_defer_order() {
    sus order []normie
    
    {
        defer {
            order = append(order, 1)
        }
        defer {
            order = append(order, 2)
        }
        defer {
            order = append(order, 3)
        }
        vibez.spill("Block execution")
    }
    
    // Defer executes in reverse order (LIFO)
    assert_eq_int(order[0], 3)
    assert_eq_int(order[1], 2)
    assert_eq_int(order[2], 1)
}

slay test_defer_with_panic() {
    sus cleanup_executed lit = cap
    
    fam {
        defer {
            cleanup_executed = based
        }
        shook("Test panic")
    } sus panic_value {
        // Panic recovered
    }
    
    assert_true(cleanup_executed)
}

slay test_defer_resource_cleanup() {
    sus resource_closed lit = cap
    
    {
        sus resource = acquire_resource()
        defer {
            release_resource(resource)
            resource_closed = based
        }
        
        use_resource(resource)
    }
    
    assert_true(resource_closed)
}

slay acquire_resource() normie {
    damn 42
}

slay use_resource(res normie) {
    vibez.spill("Using resource:", res)
}

slay release_resource(res normie) {
    vibez.spill("Releasing resource:", res)
}

// Test driver
test_start("Defer Basic")
test_defer_basic()
print_test_summary()

test_start("Defer Order")
test_defer_order()
print_test_summary()

test_start("Defer with Panic")
test_defer_with_panic()
print_test_summary()

test_start("Defer Resource Cleanup")
test_defer_resource_cleanup()
print_test_summary()

vibez.spill("All defer tests completed!")
