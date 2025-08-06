//! GC Integration Test for CURSED
//! Tests the enhanced garbage collection system integration

yeet "testz"
yeet "vibez"

// Test basic GC functionality
test_start("Basic GC Allocation Test")

sus allocated_objects vibe<tea> = vibe.create()

// Allocate objects to test GC
bestie (sus i drip = 0; i < 100; i = i + 1) {
    sus test_string tea = "GC test object " + string(i)
    allocated_objects.push(test_string)
}

assert_eq_int(allocated_objects.length(), 100)
vibez.spill("Successfully allocated 100 objects")

print_test_summary()

// Test object survival across collections
test_start("Object Survival Test")

sus persistent_objects vibe<tea> = vibe.create()

// Create persistent objects
bestie (sus i drip = 0; i < 50; i = i + 1) {
    sus persistent tea = "Persistent object " + string(i)
    persistent_objects.push(persistent)
}

// Create temporary objects that should be collected
bestie (sus cycle drip = 0; cycle < 10; cycle = cycle + 1) {
    sus temporary_objects vibe<tea> = vibe.create()
    
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        sus temp tea = "Temporary " + string(cycle) + "_" + string(i)
        temporary_objects.push(temp)
    }
    
    vibez.spill("Created temporary objects for cycle " + string(cycle))
    // temporary_objects goes out of scope here
}

// Persistent objects should still be alive
assert_eq_int(persistent_objects.length(), 50)
vibez.spill("Persistent objects survived collection cycles")

print_test_summary()

// Test large object handling
test_start("Large Object Handling Test")

sus large_objects vibe<tea> = vibe.create()

bestie (sus i drip = 0; i < 10; i = i + 1) {
    sus large_string tea = ""
    
    // Create large string objects
    bestie (sus j drip = 0; j < 1000; j = j + 1) {
        large_string = large_string + "Large object data segment " + string(i) + "_" + string(j) + " "
    }
    
    large_objects.push(large_string)
    vibez.spill("Created large object " + string(i))
}

assert_eq_int(large_objects.length(), 10)
vibez.spill("Successfully handled large object allocations")

print_test_summary()

vibez.spill("GC integration tests completed successfully!")
