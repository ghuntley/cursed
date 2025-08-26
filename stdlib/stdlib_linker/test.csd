yeet "testz"
yeet "stdlib_linker"

test_start("stdlib_linker Tests")

// Test Stdlib Linker Initialization
test_case("Stdlib Linker Initialization") {
    sus linker StdlibLinker = init_stdlib_linker()
    
    assert(linker.initialized)
    assert_eq_int(len(linker.linked_modules), 0)
    assert_eq_int(map_size(linker.function_table), 0)
}

// Test Core Stdlib Module Linking
test_case("Core Stdlib Module Linking") {
    sus linker StdlibLinker = init_stdlib_linker()
    link_core_stdlib_modules(linker)
    
    // Verify core modules are linked
    assert(array_contains(linker.linked_modules, "testz"))
    assert(array_contains(linker.linked_modules, "error_drip"))
    assert(array_contains(linker.linked_modules, "atomic_drip"))
    assert(array_contains(linker.linked_modules, "big_mood"))
    
    // Verify function table entries
    assert(map_has_key(linker.function_table, "test_start"))
    assert(map_has_key(linker.function_table, "assert_true"))
    assert(map_has_key(linker.function_table, "handle_error"))
    assert(map_has_key(linker.function_table, "atomic_load"))
    assert(map_has_key(linker.function_table, "allocate"))
    
    // Verify function mappings
    assert_eq_string(linker.function_table["test_start"], "testz_test_start")
    assert_eq_string(linker.function_table["assert_eq_string"], "testz_assert_eq_string")
    assert_eq_string(linker.function_table["handle_error"], "error_drip_handle_error")
    assert_eq_string(linker.function_table["atomic_store"], "atomic_drip_store")
    assert_eq_string(linker.function_table["deallocate"], "big_mood_deallocate")
}

// Test Stdlib Linking Validation
test_case("Stdlib Linking Validation") {
    sus linker StdlibLinker = init_stdlib_linker()
    
    // Should fail validation before linking
    assert(!validate_stdlib_linking(linker))
    
    // Link core modules
    link_core_stdlib_modules(linker)
    
    // Should pass validation after linking
    assert(validate_stdlib_linking(linker))
}

// Test Individual Module Linking
test_case("Individual Module Linking") {
    sus linker StdlibLinker = init_stdlib_linker()
    
    // Link individual modules
    link_module(linker, "vibez", {
        "spill": "vibez_spill",
        "print": "vibez_print",
        "println": "vibez_println"
    })
    
    assert(array_contains(linker.linked_modules, "vibez"))
    assert_eq_string(linker.function_table["spill"], "vibez_spill")
    assert_eq_string(linker.function_table["print"], "vibez_print")
    assert_eq_string(linker.function_table["println"], "vibez_println")
    
    // Link mathz module
    link_module(linker, "mathz", {
        "add": "mathz_add",
        "subtract": "mathz_subtract",
        "multiply": "mathz_multiply",
        "divide": "mathz_divide",
        "sqrt": "mathz_sqrt",
        "pow": "mathz_pow"
    })
    
    assert(array_contains(linker.linked_modules, "mathz"))
    assert_eq_string(linker.function_table["add"], "mathz_add")
    assert_eq_string(linker.function_table["sqrt"], "mathz_sqrt")
}

// Test Function Resolution
test_case("Function Resolution") {
    sus linker StdlibLinker = init_stdlib_linker()
    link_core_stdlib_modules(linker)
    
    // Test resolving existing functions
    sus resolved_function tea = resolve_function(linker, "test_start")
    assert_eq_string(resolved_function, "testz_test_start")
    
    resolved_function = resolve_function(linker, "atomic_load")
    assert_eq_string(resolved_function, "atomic_drip_load")
    
    // Test resolving non-existent function
    resolved_function = resolve_function(linker, "nonexistent_function")
    assert_eq_string(resolved_function, "")
    
    // Test function existence check
    assert(has_function(linker, "assert_true"))
    assert(has_function(linker, "handle_error"))
    assert(!has_function(linker, "nonexistent_function"))
}

// Test Module Dependency Resolution
test_case("Module Dependency Resolution") {
    sus linker StdlibLinker = init_stdlib_linker()
    
    // Define module dependencies
    sus dependencies map[tea][]tea = {
        "stringz": ["mathz", "arrayz"],
        "filez": ["stringz", "error_drip"],
        "networkz": ["stringz", "filez", "concurrenz"],
        "dbz": ["networkz", "stringz", "cryptz"]
    }
    
    // Resolve dependencies for networkz
    sus resolved_deps []tea = resolve_dependencies(linker, "networkz", dependencies)
    
    // Should include direct and indirect dependencies in correct order
    assert(array_contains(resolved_deps, "mathz"))
    assert(array_contains(resolved_deps, "arrayz"))
    assert(array_contains(resolved_deps, "stringz"))
    assert(array_contains(resolved_deps, "error_drip"))
    assert(array_contains(resolved_deps, "filez"))
    assert(array_contains(resolved_deps, "concurrenz"))
    
    // Dependencies should be in topological order (dependencies before dependents)
    sus mathz_index normie = array_index_of(resolved_deps, "mathz")
    sus stringz_index normie = array_index_of(resolved_deps, "stringz")
    sus filez_index normie = array_index_of(resolved_deps, "filez")
    
    assert(mathz_index < stringz_index)  // mathz before stringz
    assert(stringz_index < filez_index)  // stringz before filez
}

// Test Circular Dependency Detection
test_case("Circular Dependency Detection") {
    sus linker StdlibLinker = init_stdlib_linker()
    
    // Define circular dependencies
    sus circular_deps map[tea][]tea = {
        "module_a": ["module_b"],
        "module_b": ["module_c"],
        "module_c": ["module_a"]  // Creates cycle
    }
    
    // Should detect circular dependency
    sus has_cycle lit = has_circular_dependencies("module_a", circular_deps)
    assert(has_cycle)
    
    // Test with non-circular dependencies
    sus clean_deps map[tea][]tea = {
        "module_a": ["module_b"],
        "module_b": ["module_c"],
        "module_c": []
    }
    
    has_cycle = has_circular_dependencies("module_a", clean_deps)
    assert(!has_cycle)
}

// Test Link Map Generation
test_case("Link Map Generation") {
    sus linker StdlibLinker = init_stdlib_linker()
    link_core_stdlib_modules(linker)
    
    // Add more modules for comprehensive test
    link_module(linker, "vibez", {"spill": "vibez_spill"})
    link_module(linker, "mathz", {"add": "mathz_add", "sqrt": "mathz_sqrt"})
    link_module(linker, "stringz", {"length": "stringz_length", "concat": "stringz_concat"})
    
    // Generate link map
    sus link_map tea = generate_link_map(linker)
    
    // Verify link map contains all modules and functions
    assert(string_contains(link_map, "testz"))
    assert(string_contains(link_map, "vibez"))
    assert(string_contains(link_map, "mathz"))
    assert(string_contains(link_map, "stringz"))
    
    assert(string_contains(link_map, "test_start -> testz_test_start"))
    assert(string_contains(link_map, "spill -> vibez_spill"))
    assert(string_contains(link_map, "add -> mathz_add"))
    assert(string_contains(link_map, "length -> stringz_length"))
    
    // Should be well-formatted
    assert(string_contains(link_map, "CURSED Standard Library Link Map"))
    assert(string_contains(link_map, "Linked Modules:"))
    assert(string_contains(link_map, "Function Mappings:"))
}

// Test Symbol Table Generation
test_case("Symbol Table Generation") {
    sus linker StdlibLinker = init_stdlib_linker()
    link_core_stdlib_modules(linker)
    
    // Generate symbol table
    sus symbol_table map[tea]SymbolInfo = generate_symbol_table(linker)
    
    // Verify symbol table contains function information
    assert(map_has_key(symbol_table, "test_start"))
    assert(map_has_key(symbol_table, "assert_true"))
    assert(map_has_key(symbol_table, "atomic_load"))
    
    // Check symbol info
    sus test_start_info SymbolInfo = symbol_table["test_start"]
    assert_eq_string(test_start_info.name, "test_start")
    assert_eq_string(test_start_info.mangled_name, "testz_test_start")
    assert_eq_string(test_start_info.module, "testz")
    assert_eq_string(test_start_info.type, "function")
    
    sus atomic_load_info SymbolInfo = symbol_table["atomic_load"]
    assert_eq_string(atomic_load_info.module, "atomic_drip")
    assert_eq_string(atomic_load_info.mangled_name, "atomic_drip_load")
}

// Test Linking Statistics
test_case("Linking Statistics") {
    sus linker StdlibLinker = init_stdlib_linker()
    link_core_stdlib_modules(linker)
    
    // Add more modules
    link_module(linker, "vibez", {"spill": "vibez_spill", "print": "vibez_print"})
    link_module(linker, "mathz", {"add": "mathz_add", "sqrt": "mathz_sqrt", "pow": "mathz_pow"})
    
    // Get linking statistics
    sus stats LinkingStats = get_linking_statistics(linker)
    
    assert_eq_int(stats.total_modules, 6)  // 4 core + vibez + mathz
    assert(stats.total_functions >= 10)    // At least 10 functions linked
    assert_eq_int(stats.memory_usage, calculate_linker_memory_usage(linker))
    
    // Verify module statistics
    assert(stats.modules_by_category["core"] >= 4)
    assert(stats.modules_by_category["utility"] >= 2)
    
    // Function statistics
    assert(stats.functions_by_module["testz"] >= 3)
    assert(stats.functions_by_module["vibez"] >= 2)
    assert(stats.functions_by_module["mathz"] >= 3)
}

// Test Linker Performance
test_case("Linker Performance Test") {
    sus start_time drip = get_current_time_ms()
    
    sus linker StdlibLinker = init_stdlib_linker()
    
    // Link many modules to test performance
    bestie (sus i normie = 0; i < 50; i += 1) {
        sus module_name tea = "test_module_" + string_from_int(i)
        sus functions map[tea]tea = {}
        
        bestie (sus j normie = 0; j < 20; j += 1) {
            sus func_name tea = "func_" + string_from_int(j)
            sus mangled_name tea = module_name + "_" + func_name
            functions[func_name] = mangled_name
        }
        
        link_module(linker, module_name, functions)
    }
    
    sus linking_time drip = get_current_time_ms() - start_time
    
    // Verify linking completed successfully
    assert_eq_int(len(linker.linked_modules), 50)
    assert_eq_int(map_size(linker.function_table), 1000)  // 50 modules * 20 functions
    
    // Performance should be reasonable (under 1 second for 1000 functions)
    print_test_status("Linked 50 modules with 1000 functions in " + string_from_int(linking_time) + "ms")
    assert(linking_time < 1000)
    
    // Test function resolution performance
    start_time = get_current_time_ms()
    bestie (sus i normie = 0; i < 100; i += 1) {
        sus func_name tea = "func_" + string_from_int(i % 20)
        resolve_function(linker, func_name)
    }
    sus resolution_time drip = get_current_time_ms() - start_time
    
    print_test_status("100 function resolutions completed in " + string_from_int(resolution_time) + "ms")
    assert(resolution_time < 100)
}

print_test_summary()
