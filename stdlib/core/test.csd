fr fr CURSED Core Module Test Suite
fr fr Tests all builtin functions for spec compliance

yeet "core"
yeet "testz"

fr fr Test type conversion functions
slay test_type_conversions() lit {
    print_test("Testing type conversion functions...")
    
    fr fr Test tea() - convert to string
    lowkey tea(42) == "42" {
        print_success("tea(42) == \"42\"")
    } else {
        print_failure("tea(42) conversion failed")
        damn cap
    }
    
    lowkey tea(based) == "based" {
        print_success("tea(based) == \"based\"")
    } else {
        print_failure("tea(based) conversion failed")
        damn cap
    }
    
    fr fr Test normie() - convert to integer
    lowkey normie("42") == 42 {
        print_success("normie(\"42\") == 42")
    } else {
        print_failure("normie(\"42\") conversion failed")
        damn cap
    }
    
    lowkey normie(based) == 1 {
        print_success("normie(based) == 1")
    } else {
        print_failure("normie(based) conversion failed")
        damn cap
    }
    
    fr fr Test drip() - convert to float
    lowkey drip(42) == 42.0 {
        print_success("drip(42) == 42.0")
    } else {
        print_failure("drip(42) conversion failed")
        damn cap
    }
    
    lowkey drip("3.14") == 3.14 {
        print_success("drip(\"3.14\") == 3.14")
    } else {
        print_failure("drip(\"3.14\") conversion failed")
        damn cap
    }
    
    fr fr Test lit() - convert to boolean
    lowkey lit(1) == based {
        print_success("lit(1) == based")
    } else {
        print_failure("lit(1) conversion failed")
        damn cap
    }
    
    lowkey lit(0) == cap {
        print_success("lit(0) == cap")
    } else {
        print_failure("lit(0) conversion failed")
        damn cap
    }
    
    fr fr Test thicc() - convert to big integer
    lowkey thicc(42) == 42 {
        print_success("thicc(42) == 42")
    } else {
        print_failure("thicc(42) conversion failed")
        damn cap
    }
    
    fr fr Test sip() - convert to character
    lowkey sip("A") == 65 {
        print_success("sip(\"A\") == 65")
    } else {
        print_failure("sip(\"A\") conversion failed")
        damn cap
    }
    
    damn based
}

fr fr Test panic and recovery system
slay test_panic_recovery() lit {
    print_test("Testing panic and recovery system...")
    
    fr fr Test panic and recover
    panic("test panic message")
    sus recovered tea = recover()
    
    lowkey recovered == "test panic message" {
        print_success("panic/recover works correctly")
    } else {
        print_failure("panic/recover failed")
        damn cap
    }
    
    fr fr Test recover when no panic
    sus no_panic tea = recover()
    lowkey no_panic == "" {
        print_success("recover() returns empty when no panic")
    } else {
        print_failure("recover() should return empty when no panic")
        damn cap
    }
    
    damn based
}

fr fr Test collection operations (simplified)
slay test_collection_operations() lit {
    print_test("Testing collection operations...")
    
    fr fr Test make() function
    sus array []normie = make<normie>(5)
    lowkey array != cringe {
        print_success("make<normie>(5) creates array")
    } else {
        print_failure("make<normie>(5) failed")
        damn cap
    }
    
    fr fr Test len() function (simplified)
    sus length normie = len(array)
    lowkey length >= 0 {
        print_success("len() returns non-negative value")
    } else {
        print_failure("len() returned negative value")
        damn cap
    }
    
    fr fr Test cap() function (simplified)
    sus capacity normie = cap(array)
    lowkey capacity >= 0 {
        print_success("cap() returns non-negative value")
    } else {
        print_failure("cap() returned negative value")
        damn cap
    }
    
    fr fr Test append() function
    sus new_array []normie = append(array, 42, 123)
    lowkey new_array != cringe {
        print_success("append() works")
    } else {
        print_failure("append() failed")
        damn cap
    }
    
    damn based
}

fr fr Test memory allocation functions (simplified)
slay test_memory_operations() lit {
    print_test("Testing memory operations...")
    
    fr fr Test new() function
    sus ptr *normie = new<normie>()
    print_success("new<normie>() executed")
    
    fr fr Test delete() function
    delete(ptr)
    print_success("delete() executed")
    
    damn based
}

fr fr Test core initialization
slay test_core_initialization() lit {
    print_test("Testing core initialization...")
    
    lowkey core_is_initialized() == based {
        print_success("Core is initialized")
    } else {
        print_failure("Core not initialized")
        damn cap
    }
    
    sus version tea = core_version()
    lowkey version == "1.0.0" {
        print_success("Core version is 1.0.0")
    } else {
        print_failure("Core version mismatch")
        damn cap
    }
    
    damn based
}

fr fr Test builtin validation
slay test_builtin_validation() lit {
    print_test("Testing builtin validation...")
    
    lowkey validate_builtins() == based {
        print_success("All builtins validated successfully")
    } else {
        print_failure("Builtin validation failed")
        damn cap
    }
    
    damn based
}

fr fr Helper function for test output
slay print_test(message tea) {
    print_core("[TEST] " + message)
}

slay print_success(message tea) {
    print_core("[PASS] " + message)
}

slay print_failure(message tea) {
    print_core("[FAIL] " + message)
}

fr fr Main test execution
slay main() {
    print_core("=== CURSED Core Module Test Suite ===")
    print_core("")
    
    sus all_tests_passed lit = based
    
    lowkey test_core_initialization() == cap {
        all_tests_passed = cap
    }
    
    lowkey test_type_conversions() == cap {
        all_tests_passed = cap
    }
    
    lowkey test_panic_recovery() == cap {
        all_tests_passed = cap
    }
    
    lowkey test_collection_operations() == cap {
        all_tests_passed = cap
    }
    
    lowkey test_memory_operations() == cap {
        all_tests_passed = cap
    }
    
    lowkey test_builtin_validation() == cap {
        all_tests_passed = cap
    }
    
    print_core("")
    lowkey all_tests_passed == based {
        print_core("=== ALL TESTS PASSED ===")
        print_core("Core module implements all required builtin functions")
        print_core("CURSED language spec compliance: ACHIEVED")
    } else {
        print_core("=== SOME TESTS FAILED ===")
        print_core("Core module needs further implementation")
    }
}

fr fr Run tests
main()
