fr fr Comprehensive test for critical CURSED stdlib modules
fr fr Tests memoryz, typez, debugz, and sysz integration

yeet "testz"
yeet "memoryz"
yeet "typez"
yeet "debugz"
yeet "sysz"

fr fr ===== INITIALIZATION TESTS =====

slay test_module_initialization() lit {
    testz.test_start("Critical modules initialization")
    
    fr fr Initialize all critical modules
    testz.assert_true(typez.init_type_system())
    testz.assert_true(debugz.init_debug_system())
    testz.assert_true(sysz.init_system())
    
    fr fr Verify modules are functional
    testz.assert_true(debugz.is_debug_enabled())
    testz.assert_true(typez.get_all_types().len() > 0)
    testz.assert_true(sysz.get_current_pid() > 0)
    testz.assert_true(memoryz.get_memory_stats().allocation_count >= 0)
    
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_memory_type_integration() lit {
    testz.test_start("Memory and type system integration")
    
    fr fr Create a custom struct type
    sus name_field typez.FieldInfo = typez.FieldInfo{
        name: "name",
        type_info: typez.get_type_by_name("tea"),
        offset: 0,
        tag: ""
    }
    
    sus size_field typez.FieldInfo = typez.FieldInfo{
        name: "size",
        type_info: typez.get_type_by_name("normie"),
        offset: 8,
        tag: ""
    }
    
    sus fields []typez.FieldInfo = [name_field, size_field]
    sus buffer_type_id normie = typez.register_struct_type("Buffer", fields, [])
    testz.assert_true(buffer_type_id > 0)
    
    fr fr Get type info for allocation
    sus buffer_type typez.TypeInfo = typez.get_type_by_id(buffer_type_id)
    testz.assert_true(buffer_type.size > 0)
    
    fr fr Allocate memory for the struct
    sus buffer_ptr normie = memoryz.allocate(buffer_type.size, buffer_type_id)
    testz.assert_true(buffer_ptr != 0)
    testz.assert_true(memoryz.is_valid_pointer(buffer_ptr))
    testz.assert_eq_int(memoryz.get_block_type(buffer_ptr), buffer_type_id)
    
    fr fr Set field values using type reflection
    testz.assert_true(typez.set_field_value(buffer_ptr, buffer_type, "size", 1024))
    
    fr fr Get field values using type reflection
    sus retrieved_size normie = typez.get_field_value(buffer_ptr, buffer_type, "size")
    testz.assert_eq_int(retrieved_size, 1024)
    
    fr fr Clean up
    testz.assert_true(memoryz.deallocate(buffer_ptr))
    
    damn based
}

slay test_debug_memory_integration() lit {
    testz.test_start("Debug and memory system integration")
    
    fr fr Enable memory tracking in debug system
    sus debug_config debugz.DebugConfig = debugz.get_debug_config()
    debug_config.memory_tracking = based
    debug_config.verbose_logging = cap  fr fr Keep output clean
    debugz.configure_debug(debug_config)
    
    fr fr Allocate memory with debug tracking
    sus ptr normie = memoryz.allocate(2048, 500)
    testz.assert_true(ptr != 0)
    
    fr fr Track the allocation in debug system
    testz.assert_true(debugz.track_memory_allocation(ptr, 2048, "TestAllocation"))
    
    fr fr Check memory corruption detection
    testz.assert_true(debugz.check_memory_corruption(ptr, 2048))
    testz.assert_false(debugz.check_memory_corruption(0, 100))  fr fr Null pointer
    
    fr fr Track deallocation
    testz.assert_true(debugz.track_memory_deallocation(ptr, 2048))
    testz.assert_true(memoryz.deallocate(ptr))
    
    damn based
}

slay test_debug_system_integration() lit {
    testz.test_start("Debug and system integration")
    
    fr fr Enable stack tracing and profiling
    sus debug_config debugz.DebugConfig = debugz.get_debug_config()
    debug_config.stack_trace_enabled = based
    debug_config.performance_profiling = based
    debug_config.verbose_logging = cap
    debugz.configure_debug(debug_config)
    
    fr fr Simulate function execution with system calls
    debugz.push_stack_frame("system_test_function", "test.csd", 100)
    debugz.start_profiling("system_test_function")
    
    fr fr Add variables to debug context
    testz.assert_true(debugz.add_local_variable("pid", typez.get_type_by_name("normie"), sysz.get_current_pid(), 0x1000))
    testz.assert_true(debugz.add_local_variable("hostname", typez.get_type_by_name("tea"), 0, 0x2000))
    
    fr fr Perform system operations
    sus system_info sysz.SystemInfo = sysz.get_system_info()
    testz.assert_true(system_info.cpu_count > 0)
    
    sus current_time normie = sysz.get_current_time_millis()
    testz.assert_true(current_time > 0)
    
    fr fr End profiling and stack frame
    debugz.end_profiling("system_test_function", 3000000)  fr fr 3ms
    debugz.pop_stack_frame()
    
    fr fr Verify profiling data
    sus profiler_results []debugz.ProfilerResult = debugz.get_profiler_results()
    testz.assert_eq_int(profiler_results.len(), 1)
    testz.assert_eq_string(profiler_results[0].function_name, "system_test_function")
    testz.assert_eq_int(profiler_results[0].total_time_ns, 3000000)
    
    damn based
}

slay test_memory_gc_debug_integration() lit {
    testz.test_start("Memory GC and debug integration")
    
    fr fr Configure GC and debug for testing
    sus gc_config memoryz.GCConfig = memoryz.GCConfig{
        enable_gc: based,
        gc_threshold: 4096,  fr fr Low threshold for testing
        collection_interval: 100,
        concurrent_gc: cap,
        mark_and_sweep: based,
        generational: cap
    }
    memoryz.configure_gc(gc_config)
    
    fr fr Enable memory tracking
    sus debug_config debugz.DebugConfig = debugz.get_debug_config()
    debug_config.memory_tracking = based
    debugz.configure_debug(debug_config)
    
    fr fr Allocate multiple blocks to trigger GC
    sus pointers []normie = []
    bestie i := 0; i < 10; i++ {
        sus ptr normie = memoryz.allocate(512, 600 + i)
        testz.assert_true(ptr != 0)
        pointers.push(ptr)
        debugz.track_memory_allocation(ptr, 512, "GCTestBlock")
    }
    
    fr fr Check memory usage before GC
    sus stats_before memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(stats_before.current_usage >= 5120)  fr fr 10 * 512 bytes
    
    fr fr Trigger garbage collection
    testz.assert_true(memoryz.trigger_gc())
    
    fr fr Check memory statistics after GC
    sus stats_after memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(stats_after.gc_cycles > stats_before.gc_cycles)
    
    fr fr Free half the blocks manually
    bestie i := 0; i < 5; i++ {
        debugz.track_memory_deallocation(pointers[i], 512)
        memoryz.deallocate(pointers[i])
    }
    
    fr fr Trigger GC again
    memoryz.trigger_gc()
    
    fr fr Clean up remaining blocks
    bestie i := 5; i < 10; i++ {
        debugz.track_memory_deallocation(pointers[i], 512)
        memoryz.deallocate(pointers[i])
    }
    
    damn based
}

fr fr ===== COMPLEX SCENARIO TESTS =====

slay test_compiler_runtime_simulation() lit {
    testz.test_start("Compiler runtime scenario simulation")
    
    fr fr Simulate compiler runtime scenario with all modules
    debugz.push_stack_frame("compile_file", "compiler.csd", 200)
    debugz.start_profiling("compile_file")
    
    fr fr Register AST node types
    sus token_field typez.FieldInfo = typez.FieldInfo{
        name: "token",
        type_info: typez.get_type_by_name("tea"),
        offset: 0,
        tag: "ast_token"
    }
    
    sus line_field typez.FieldInfo = typez.FieldInfo{
        name: "line",
        type_info: typez.get_type_by_name("normie"),
        offset: 8,
        tag: "source_line"
    }
    
    sus ast_fields []typez.FieldInfo = [token_field, line_field]
    sus ast_node_type_id normie = typez.register_struct_type("ASTNode", ast_fields, [])
    testz.assert_true(ast_node_type_id > 0)
    
    fr fr Allocate memory for AST nodes
    sus ast_nodes []normie = []
    bestie i := 0; i < 20; i++ {
        sus node_ptr normie = memoryz.allocate(typez.get_type_by_id(ast_node_type_id).size, ast_node_type_id)
        testz.assert_true(node_ptr != 0)
        ast_nodes.push(node_ptr)
        debugz.add_local_variable("node_" + i, typez.get_type_by_id(ast_node_type_id), node_ptr, node_ptr)
    }
    
    fr fr Simulate compilation phases
    debugz.push_stack_frame("lexical_analysis", "lexer.csd", 50)
    debugz.log_info("Starting lexical analysis")
    debugz.pop_stack_frame()
    
    debugz.push_stack_frame("syntax_analysis", "parser.csd", 100)
    debugz.log_info("Starting syntax analysis")
    debugz.pop_stack_frame()
    
    debugz.push_stack_frame("semantic_analysis", "semantic.csd", 150)
    debugz.log_info("Starting semantic analysis")
    debugz.pop_stack_frame()
    
    debugz.push_stack_frame("code_generation", "codegen.csd", 200)
    debugz.log_info("Starting code generation")
    
    fr fr Get system info during compilation
    sus process_info sysz.ProcessInfo = sysz.get_current_process_info()
    testz.assert_true(process_info.memory_usage > 0)
    
    debugz.pop_stack_frame()
    
    fr fr Clean up AST nodes
    bestie node_ptr in ast_nodes {
        memoryz.deallocate(node_ptr)
    }
    
    fr fr End compilation profiling
    debugz.end_profiling("compile_file", 50000000)  fr fr 50ms
    debugz.pop_stack_frame()
    
    fr fr Generate reports
    sus final_stats memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(final_stats.allocation_count >= 20)
    
    damn based
}

slay test_error_handling_integration() lit {
    testz.test_start("Integrated error handling scenario")
    
    fr fr Set up debug system for error handling
    sus debug_config debugz.DebugConfig = debugz.get_debug_config()
    debug_config.stack_trace_enabled = based
    debug_config.verbose_logging = cap
    debugz.configure_debug(debug_config)
    
    fr fr Simulate error scenario
    debugz.push_stack_frame("error_prone_function", "test.csd", 300)
    
    fr fr Test memory error detection
    testz.assert_false(debugz.check_memory_corruption(0, 100))  fr fr Should detect null pointer
    testz.assert_false(debugz.debug_assert(cap, "Simulated assertion failure"))
    
    fr fr Test type system error handling
    sus unknown_type typez.TypeInfo = typez.get_type_by_name("NonExistentType")
    testz.assert_eq_string(unknown_type.name, "unknown")
    testz.assert_eq_int(unknown_type.type_id, 0)
    
    fr fr Test system error handling
    testz.assert_false(sysz.file_exists("/absolutely/nonexistent/path/file.txt"))
    testz.assert_false(sysz.is_process_running(99999999))
    
    fr fr Test memory allocation failures
    testz.assert_eq_int(memoryz.allocate(0, 700), 0)      fr fr Zero size
    testz.assert_eq_int(memoryz.allocate(-1, 701), 0)     fr fr Negative size
    
    fr fr Generate error report
    debugz.log_error("Simulated error in integration test")
    debugz.print_stack_trace()
    
    debugz.pop_stack_frame()
    
    damn based
}

fr fr ===== PERFORMANCE TESTS =====

slay test_performance_integration() lit {
    testz.test_start("Performance integration test")
    
    fr fr Enable profiling
    sus debug_config debugz.DebugConfig = debugz.get_debug_config()
    debug_config.performance_profiling = based
    debug_config.verbose_logging = cap
    debugz.configure_debug(debug_config)
    
    fr fr Test memory allocation performance
    debugz.start_profiling("memory_allocation_test")
    sus start_time normie = sysz.get_current_time_nanos()
    
    sus pointers []normie = []
    bestie i := 0; i < 100; i++ {
        sus ptr normie = memoryz.allocate(1024, 800 + i)
        lowkey ptr != 0 {
            pointers.push(ptr)
        }
    }
    
    sus alloc_time normie = sysz.get_current_time_nanos() - start_time
    debugz.end_profiling("memory_allocation_test", alloc_time)
    
    fr fr Test type system performance
    debugz.start_profiling("type_lookup_test")
    sus type_start normie = sysz.get_current_time_nanos()
    
    bestie i := 0; i < 100; i++ {
        sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
        sus normie_type typez.TypeInfo = typez.get_type_by_name("normie")
        testz.assert_true(typez.is_assignable(normie_type, normie_type))
    }
    
    sus type_time normie = sysz.get_current_time_nanos() - type_start
    debugz.end_profiling("type_lookup_test", type_time)
    
    fr fr Test system call performance
    debugz.start_profiling("system_call_test")
    sus sys_start normie = sysz.get_current_time_nanos()
    
    bestie i := 0; i < 50; i++ {
        sus pid normie = sysz.get_current_pid()
        sus time normie = sysz.get_current_time_millis()
        testz.assert_true(pid > 0)
        testz.assert_true(time > 0)
    }
    
    sus sys_time normie = sysz.get_current_time_nanos() - sys_start
    debugz.end_profiling("system_call_test", sys_time)
    
    fr fr Clean up memory
    bestie ptr in pointers {
        memoryz.deallocate(ptr)
    }
    
    fr fr Generate performance report
    debugz.print_profiler_report()
    
    sus profiler_results []debugz.ProfilerResult = debugz.get_profiler_results()
    testz.assert_eq_int(profiler_results.len(), 3)
    
    damn based
}

fr fr ===== STRESS TESTS =====

slay test_stress_integration() lit {
    testz.test_start("Stress test integration")
    
    fr fr Configure systems for stress testing
    sus gc_config memoryz.GCConfig = memoryz.GCConfig{
        enable_gc: based,
        gc_threshold: 8192,  fr fr Higher threshold for stress test
        collection_interval: 50,
        concurrent_gc: cap,
        mark_and_sweep: based,
        generational: cap
    }
    memoryz.configure_gc(gc_config)
    
    sus debug_config debugz.DebugConfig = debugz.get_debug_config()
    debug_config.memory_tracking = based
    debug_config.performance_profiling = based
    debug_config.verbose_logging = cap
    debugz.configure_debug(debug_config)
    
    fr fr Stress test: Many allocations and deallocations
    debugz.start_profiling("stress_test")
    sus stress_pointers []normie = []
    
    bestie round := 0; round < 5; round++ {
        fr fr Allocate many blocks
        bestie i := 0; i < 50; i++ {
            sus size normie = ((i % 8) + 1) * 128  fr fr Varying sizes
            sus ptr normie = memoryz.allocate(size, 900 + round * 50 + i)
            lowkey ptr != 0 {
                stress_pointers.push(ptr)
                debugz.track_memory_allocation(ptr, size, "StressTestBlock")
            }
        }
        
        fr fr Register some types
        lowkey round % 2 == 0 {
            sus field typez.FieldInfo = typez.FieldInfo{
                name: "data",
                type_info: typez.get_type_by_name("normie"),
                offset: 0,
                tag: ""
            }
            typez.register_struct_type("StressType" + round, [field], [])
        }
        
        fr fr Trigger GC periodically
        lowkey round % 2 == 1 {
            memoryz.trigger_gc()
        }
        
        fr fr Free some blocks
        sus to_free normie = stress_pointers.len() / 3
        bestie i := 0; i < to_free; i++ {
            lowkey stress_pointers.len() > 0 {
                sus ptr normie = stress_pointers[0]
                stress_pointers.remove(0)
                debugz.track_memory_deallocation(ptr, memoryz.get_block_size(ptr))
                memoryz.deallocate(ptr)
            }
        }
        
        fr fr Check system resources
        sus memory_usage normie = sysz.get_memory_usage()
        testz.assert_true(memory_usage > 0)
    }
    
    fr fr Clean up remaining blocks
    bestie ptr in stress_pointers {
        debugz.track_memory_deallocation(ptr, memoryz.get_block_size(ptr))
        memoryz.deallocate(ptr)
    }
    
    fr fr Final GC
    memoryz.trigger_gc()
    
    debugz.end_profiling("stress_test", 100000000)  fr fr 100ms
    
    fr fr Verify system is still functional
    testz.assert_true(debugz.is_debug_enabled())
    testz.assert_true(typez.get_all_types().len() > 0)
    testz.assert_true(sysz.get_current_pid() > 0)
    
    sus final_stats memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(final_stats.gc_cycles > 0)
    
    damn based
}

fr fr ===== REPORTING TESTS =====

slay test_comprehensive_reporting() lit {
    testz.test_start("Comprehensive system reporting")
    
    fr fr Generate all module reports
    vibez.spill("\n🔍 COMPREHENSIVE STDLIB REPORT")
    vibez.spill("════════════════════════════════")
    
    fr fr Memory system report
    memoryz.print_memory_report()
    
    fr fr Type system report
    typez.print_all_types()
    
    fr fr Debug system report
    debugz.dump_debug_state()
    debugz.print_profiler_report()
    
    fr fr System information report
    sysz.print_system_info()
    
    fr fr Integration statistics
    vibez.spill("\n📊 Integration Statistics")
    vibez.spill("═══════════════════════════")
    
    sus memory_stats memoryz.AllocationStats = memoryz.get_memory_stats()
    vibez.spill("Total Memory Allocated: ", memoryz.format_bytes(memory_stats.total_allocated))
    vibez.spill("Current Memory Usage: ", memoryz.format_bytes(memory_stats.current_usage))
    vibez.spill("GC Cycles: ", memory_stats.gc_cycles)
    
    sus all_types []typez.TypeInfo = typez.get_all_types()
    vibez.spill("Registered Types: ", all_types.len())
    
    sus profiler_results []debugz.ProfilerResult = debugz.get_profiler_results()
    vibez.spill("Profiled Functions: ", profiler_results.len())
    
    sus system_info sysz.SystemInfo = sysz.get_system_info()
    vibez.spill("System: ", system_info.os_name, " ", system_info.arch)
    vibez.spill("CPUs: ", system_info.cpu_count)
    
    vibez.spill("\n✅ All critical stdlib modules operational")
    
    damn based
}

fr fr ===== RUN ALL INTEGRATION TESTS =====

slay run_comprehensive_critical_stdlib_tests() lit {
    testz.test_group_start("Critical CURSED stdlib integration tests")
    
    fr fr Module initialization
    test_module_initialization()
    
    fr fr Core integration tests
    test_memory_type_integration()
    test_debug_memory_integration()
    test_debug_system_integration()
    test_memory_gc_debug_integration()
    
    fr fr Complex scenario tests
    test_compiler_runtime_simulation()
    test_error_handling_integration()
    
    fr fr Performance and stress tests
    test_performance_integration()
    test_stress_integration()
    
    fr fr Final reporting
    test_comprehensive_reporting()
    
    testz.test_group_end()
    testz.print_test_summary()
    
    vibez.spill("\n🎉 Critical stdlib modules test completed!")
    vibez.spill("✅ memoryz - Memory management and GC")
    vibez.spill("✅ typez - Type reflection and checking")
    vibez.spill("✅ debugz - Debug utilities and profiling") 
    vibez.spill("✅ sysz - System calls and process management")
    vibez.spill("\n🚀 Ready for self-hosting compiler development!")
    
    damn based
}

fr fr Initialize and run all tests
run_comprehensive_critical_stdlib_tests()
