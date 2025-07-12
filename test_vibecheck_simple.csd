// Simple test for vibecheck module without external dependencies
yeet "vibecheck"

vibez.spill("Testing vibecheck module...")

// Test basic initialization
sus init_result lit = vibecheck.vibecheck_init()
vibez.spill("✓ Initialization: " + init_result.(tea))

// Test timing functions
sus start_time thicc = vibecheck.get_start_time()
vibez.spill("✓ Start time: " + start_time.(tea))

sus uptime thicc = vibecheck.get_uptime()
vibez.spill("✓ Uptime: " + uptime.(tea) + "ms")

// Test memory functions
sus total_allocs thicc = vibecheck.get_total_allocations()
vibez.spill("✓ Total allocations: " + total_allocs.(tea))

sus current_mem thicc = vibecheck.get_current_memory()
vibez.spill("✓ Current memory: " + current_mem.(tea))

// Test memory allocation tracking
sus alloc_result lit = vibecheck.update_memory_stats(1024, 0)
vibez.spill("✓ Memory allocation tracking: " + alloc_result.(tea))

sus new_current_mem thicc = vibecheck.get_current_memory()
vibez.spill("✓ Memory after allocation: " + new_current_mem.(tea))

// Test GC functions
sus gc_count thicc = vibecheck.get_gc_count()
vibez.spill("✓ GC count: " + gc_count.(tea))

sus gc_trigger lit = vibecheck.trigger_gc()
vibez.spill("✓ GC trigger: " + gc_trigger.(tea))

// Test goroutine functions
sus goroutine_count thicc = vibecheck.get_goroutine_count()
vibez.spill("✓ Goroutine count: " + goroutine_count.(tea))

// Test system info
sus system_info tea = vibecheck.get_system_info()
vibez.spill("✓ System info length: " + system_info.length().(tea))

// Test type reflection
sus test_value normie = 42
sus type_info tea = vibecheck.get_type_info(test_value)
vibez.spill("✓ Type info for 42: " + type_info)

// Test performance monitoring
sus monitor_start lit = vibecheck.start_performance_monitoring()
vibez.spill("✓ Start monitoring: " + monitor_start.(tea))

sus profile_result lit = vibecheck.profile_function_enter("test")
vibez.spill("✓ Profile function: " + profile_result.(tea))

sus monitor_report tea = vibecheck.stop_performance_monitoring()
vibez.spill("✓ Monitor report length: " + monitor_report.length().(tea))

// Test health check
sus health lit = vibecheck.runtime_health_check()
vibez.spill("✓ Runtime health: " + health.(tea))

vibez.spill("🎉 All vibecheck tests completed!")
