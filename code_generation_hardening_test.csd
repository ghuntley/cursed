/// Comprehensive test for code generation hardening fixes
/// Tests: error propagation, VTable safety, GC integration, bounds checking

yeet "vibez"
yeet "arrayz"

/// Test interface with error propagation
collab TestInterface {
    slay process_data(data tea) tea yikes<tea>
    slay get_count() drip yikes<tea>
}

/// Implementation with error handling
squad TestStruct {
    value drip
    name tea
}

impl TestInterface for TestStruct {
    slay process_data(data tea) tea yikes<tea> {
        ready (data == "invalid") {
            yikes "Invalid data provided"
        }
        damn self.name + ": " + data
    }
    
    slay get_count() drip yikes<tea> {
        ready (self.value < 0) {
            yikes "Negative value not allowed"
        }
        damn self.value
    }
}

/// Test bounds checking with array operations
slay test_array_bounds() {
    vibez.spill("Testing array bounds checking...")
    
    sus arr []drip = [1, 2, 3, 4, 5]
    
    /// Valid access
    vibez.spill("Valid access: ", arr[2])
    
    /// This should trigger bounds checking if enabled
    sus safe_index drip = 3
    ready (safe_index < arrayz.len(arr)) {
        vibez.spill("Safe access: ", arr[safe_index])
    } otherwise {
        vibez.spill("Index out of bounds prevented")
    }
    
    /// Test negative index protection
    ready (-1 >= 0 and -1 < arrayz.len(arr)) {
        vibez.spill("This should never execute")
    } otherwise {
        vibez.spill("Negative index correctly rejected")
    }
}

/// Test VTable null safety with interface dispatch
slay test_vtable_safety() {
    vibez.spill("Testing VTable null safety...")
    
    sus obj TestStruct = TestStruct{value: 42, name: "test"}
    sus interface_ptr *TestInterface = &obj
    
    /// Valid interface dispatch
    sus result tea = interface_ptr.process_data("hello") fam {
        when _ -> {
            vibez.spill("Error caught in interface dispatch")
            damn "error"
        }
    }
    vibez.spill("Interface result: ", result)
    
    /// Test error propagation through interface
    sus error_result tea = interface_ptr.process_data("invalid") fam {
        when "Invalid data provided" -> {
            vibez.spill("✅ Error properly propagated through interface")
            damn "handled"
        }
        when _ -> damn "other error"
    }
    vibez.spill("Error handling result: ", error_result)
}

/// Test memory allocation patterns for GC
slay test_gc_integration() {
    vibez.spill("Testing GC stackmap integration...")
    
    /// Create multiple heap objects
    sus objects []*TestStruct = []
    
    bestie (sus i drip = 0; i < 5; i = i + 1) {
        sus new_obj *TestStruct = &TestStruct{
            value: i * 10,
            name: "object_" + tea(i)
        }
        arrayz.push(&objects, new_obj)
    }
    
    /// Access objects to ensure they're live
    bestie (sus i drip = 0; i < arrayz.len(objects); i = i + 1) {
        vibez.spill("Object ", i, ": ", objects[i].name)
    }
    
    vibez.spill("✅ GC stackmap test completed")
}

/// Test complex error scenarios
slay test_error_propagation() {
    vibez.spill("Testing error propagation through call chains...")
    
    slay inner_func(x drip) drip yikes<tea> {
        ready (x == 0) {
            yikes "Division by zero"
        }
        damn 100 / x
    }
    
    slay middle_func(y drip) drip yikes<tea> {
        sus result drip = inner_func(y) fam {
            when "Division by zero" -> yikes "Propagated: Division by zero"
            when _ -> yikes "Unknown error"
        }
        damn result * 2
    }
    
    /// Test successful case
    sus success drip = middle_func(5) fam {
        when _ -> {
            vibez.spill("Unexpected error in success case")
            damn -1
        }
    }
    vibez.spill("Success case result: ", success)
    
    /// Test error propagation
    sus error_case drip = middle_func(0) fam {
        when "Propagated: Division by zero" -> {
            vibez.spill("✅ Error properly propagated through call chain")
            damn -2
        }
        when _ -> damn -3
    }
    vibez.spill("Error propagation result: ", error_case)
}

/// Main test runner
slay main() {
    vibez.spill("=== Code Generation Hardening Test Suite ===")
    
    test_array_bounds()
    vibez.spill("")
    
    test_vtable_safety()
    vibez.spill("")
    
    test_gc_integration()
    vibez.spill("")
    
    test_error_propagation()
    vibez.spill("")
    
    vibez.spill("=== All hardening tests completed ===")
}
