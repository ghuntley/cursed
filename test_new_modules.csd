fr fr Test new core modules implementation
yeet "core"
yeet "memz"
yeet "envz"
yeet "result"
yeet "option"
yeet "vibez"

fr fr Test memz module
slay test_memz() {
    vibez.spill("=== Testing memz module ===")
    
    memz.init_memz()
    
    # Test basic allocation
    sus ptr1 normie = memz.malloc(1024)
    vibez.spill("Allocated 1024 bytes at: " + core.int_to_string(ptr1))
    
    sus ptr2 normie = memz.malloc(2048)
    vibez.spill("Allocated 2048 bytes at: " + core.int_to_string(ptr2))
    
    # Test memory stats
    sus stats = memz.get_memory_stats()
    vibez.spill("Total allocated: " + core.int_to_string(stats.total_allocated))
    vibez.spill("Current usage: " + core.int_to_string(stats.current_usage))
    
    # Test arena allocator
    sus arena = memz.create_arena(4096)
    check arena.is_initialized == based {
        vibez.spill("Arena created successfully")
        sus arena_ptr = memz.arena_alloc(&arena, 256)
        vibez.spill("Arena allocated 256 bytes at: " + core.int_to_string(arena_ptr))
    }
    
    # Free memory
    memz.free(ptr1)
    memz.free(ptr2)
    
    vibez.spill("memz test completed\n")
}

fr fr Test envz module
slay test_envz() {
    vibez.spill("=== Testing envz module ===")
    
    envz.init_envz()
    
    # Test environment variables
    check envz.set("TEST_VAR", "test_value") == based {
        vibez.spill("Set TEST_VAR=test_value")
    }
    
    sus value tea = envz.get("TEST_VAR")
    vibez.spill("Retrieved TEST_VAR: " + value)
    
    # Test expansion
    sus expanded tea = envz.expand("Hello ${TEST_VAR}!")
    vibez.spill("Expanded: " + expanded)
    
    # Test platform detection
    sus platform tea = envz.get_platform()
    vibez.spill("Detected platform: " + platform)
    
    # Test common environment variables
    sus home tea = envz.get_home()
    vibez.spill("Home directory: " + home)
    
    vibez.spill("envz test completed\n")
}

fr fr Test result module
slay test_result() {
    vibez.spill("=== Testing result module ===")
    
    # Test Ok result
    sus ok_result = result.ok_int(42)
    check result.is_ok_int(ok_result) == based {
        sus value normie = result.unwrap_int(ok_result)
        vibez.spill("Ok result value: " + core.int_to_string(value))
    }
    
    # Test Err result
    sus err_result = result.err_string("something went wrong")
    check result.is_err_string(err_result) == based {
        sus error_msg tea = result.unwrap_err_string(err_result)
        vibez.spill("Error result message: " + error_msg)
    }
    
    # Test safe operations
    sus divide_result = result.safe_divide(10, 2)
    check result.is_ok_int(divide_result) == based {
        vibez.spill("10 / 2 = " + core.int_to_string(result.unwrap_int(divide_result)))
    }
    
    sus divide_by_zero = result.safe_divide(10, 0)
    check result.is_err_int(divide_by_zero) == based {
        vibez.spill("Division by zero error: " + result.unwrap_err_int(divide_by_zero))
    }
    
    vibez.spill("result test completed\n")
}

fr fr Test option module
slay test_option() {
    vibez.spill("=== Testing option module ===")
    
    # Test Some option
    sus some_value = option.some_string("hello")
    check option.is_some_string(some_value) == based {
        sus value tea = option.unwrap_string(some_value)
        vibez.spill("Some value: " + value)
    }
    
    # Test None option
    sus none_value = option.none_string()
    check option.is_none_string(none_value) == based {
        vibez.spill("None value detected")
        sus default_value tea = option.unwrap_or_string(none_value, "default")
        vibez.spill("None unwrapped with default: " + default_value)
    }
    
    # Test map operation
    sus mapped = option.map_string_to_int_len(some_value)
    check option.is_some_int(mapped) == based {
        sus length normie = option.unwrap_int(mapped)
        vibez.spill("Mapped string length: " + core.int_to_string(length))
    }
    
    vibez.spill("option test completed\n")
}

fr fr Test core module functionality
slay test_core() {
    vibez.spill("=== Testing core module ===")
    
    # Test type conversions
    sus int_str tea = core.to_string(123)
    vibez.spill("Integer to string: " + int_str)
    
    sus str_int normie = core.to_int("42")
    vibez.spill("String to integer: " + core.int_to_string(str_int))
    
    sus str_float meal = core.to_float("3.14")
    vibez.spill("String to float: " + core.float_to_string(str_float))
    
    sus str_bool lit = core.to_bool("based")
    check str_bool == based {
        vibez.spill("String to bool: based")
    }
    
    # Test runtime functions
    check core.runtime_init() == based {
        vibez.spill("Runtime initialized")
    }
    
    sus is_init lit = core.runtime_is_initialized()
    vibez.spill("Runtime initialized: " + core.to_string(is_init))
    
    sus core_info tea = core.core_info()
    vibez.spill("Core info: " + core_info)
    
    vibez.spill("core test completed\n")
}

fr fr Main test function
slay main() {
    vibez.spill("Testing new core modules implementation\n")
    
    test_core()
    test_memz()
    test_envz() 
    test_result()
    test_option()
    
    vibez.spill("All core module tests completed successfully!")
}

main()
