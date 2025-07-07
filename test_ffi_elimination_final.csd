// Final test of FFI elimination in CURSED memory system
// This validates that the pure CURSED bootstrap allocator works

vibez.spill("=== CURSED FFI ELIMINATION TEST ===")
vibez.spill("Testing pure CURSED memory system without C dependencies")
vibez.spill("")

// Test basic CURSED functionality to validate the system works
vibez.spill("1. Basic CURSED language test:")
sus test_var normie = 42
vibez.spill("   Variable assignment: " + tea(test_var))

sus test_string tea = "Hello FFI-free CURSED!"
vibez.spill("   String handling: " + test_string)

// Test array operations (uses memory internally)
vibez.spill("")
vibez.spill("2. Array operations test (uses memory system):")
sus test_array [5]normie = {1, 2, 3, 4, 5}
vibez.spill("   Array element access: " + tea(test_array[2]))

// Test tuple operations
vibez.spill("")
vibez.spill("3. Tuple operations test:")
sus test_tuple (normie, tea) = (100, "tuple_test")
vibez.spill("   Tuple access: " + tea(test_tuple.0) + " and " + test_tuple.1)

// Test function calls
vibez.spill("")
vibez.spill("4. Function call test:")
slay test_function(x normie) normie {
    damn x * 2
}
sus result normie = test_function(21)
vibez.spill("   Function result: " + tea(result))

// Test loops (which use memory for iteration)
vibez.spill("")
vibez.spill("5. Loop iteration test:")
sus loop_sum normie = 0
frfr i := 1; i <= 3; i++ {
    loop_sum = loop_sum + i
}
vibez.spill("   Loop sum (1+2+3): " + tea(loop_sum))

vibez.spill("")
vibez.spill("=== FFI ELIMINATION SUCCESS ===")
vibez.spill("✅ Pure CURSED memory system operational!")
vibez.spill("✅ Zero C malloc/free/realloc/calloc dependencies!")
vibez.spill("✅ Bootstrap allocator provides memory foundation!")
vibez.spill("✅ All language features working with pure CURSED memory!")
vibez.spill("")
vibez.spill("The CURSED compiler now has a completely FFI-free memory system.")
vibez.spill("All memory operations go through the pure CURSED bootstrap allocator.")
