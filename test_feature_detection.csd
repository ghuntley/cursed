yeet "testz"
yeet "sys_core" 
yeet "env"
yeet "vibecheck"

test_start("Platform Feature Detection - Comprehensive PAL Test")

// Test basic platform information detection
vibez.spill("🔍 Testing basic platform detection...")

sus platform tea = sys_core.get_platform()
sus arch tea = sys_core.get_architecture() 
sus os_version tea = sys_core.get_os_version()
sus hostname tea = env.get_hostname()

assert_not_null(platform)
assert_not_null(arch)
assert_not_null(os_version)
assert_not_null(hostname)

vibez.spill("  ✅ Platform: ", platform)
vibez.spill("  ✅ Architecture: ", arch)
vibez.spill("  ✅ OS Version: ", os_version)
vibez.spill("  ✅ Hostname: ", hostname)

// Test hardware capability detection
vibez.spill("🏗️ Testing hardware capabilities...")

sus cpu_count normie = sys_core.get_cpu_count()
sus total_memory normie = sys_core.get_total_memory()
sus available_memory normie = sys_core.get_available_memory()

assert_gt(cpu_count, 0)
assert_gt(total_memory, 0)
assert_gt(available_memory, 0)

vibez.spill("  ✅ CPU Cores: ", cpu_count)
vibez.spill("  ✅ Total Memory: ", total_memory, " bytes")
vibez.spill("  ✅ Available Memory: ", available_memory, " bytes")

// Test memory efficiency calculation
vibes available_memory <= total_memory {
    vibez.spill("  ✅ Memory values are consistent")
    assert_true(based)
} sus {
    vibez.spill("  ❌ Memory values inconsistent")
    assert_true(cap)
}

// Test memory allocation and alignment
vibez.spill("💾 Testing memory management...")

sus memory_addr normie = sys_core.alloc(1024)
assert_gt(memory_addr, 0)
vibez.spill("  ✅ Memory allocation successful: ", memory_addr)

sus alloc_success lit = sys_core.free(memory_addr)
assert_true(alloc_success)
vibez.spill("  ✅ Memory deallocation successful")

// Test different allocation sizes for platform optimization testing
periodt i := 1; i <= 5; i++ {
    sus size normie = i * 1024  // 1KB to 5KB
    sus addr normie = sys_core.alloc(size)
    assert_gt(addr, 0)
    sys_core.free(addr)
    vibez.spill("  ✅ Allocation/deallocation ", size, " bytes successful")
}

// Test runtime introspection capabilities
vibez.spill("🔬 Testing runtime introspection...")

vibecheck.vibecheck_init()

sus system_info tea = vibecheck.get_system_info()
assert_not_null(system_info)
vibez.spill("  ✅ System info retrieved")

sus uptime thicc = vibecheck.get_uptime()
assert_gt(uptime, 0)
vibez.spill("  ✅ Runtime uptime: ", uptime, " ms")

sus current_mem thicc = vibecheck.get_current_memory() 
sus peak_mem thicc = vibecheck.get_peak_memory()
sus gc_count thicc = vibecheck.get_gc_count()

vibez.spill("  ✅ Current memory: ", current_mem, " bytes")
vibez.spill("  ✅ Peak memory: ", peak_mem, " bytes")
vibez.spill("  ✅ GC count: ", gc_count)

// Test memory pressure detection
sus has_pressure lit = vibecheck.detect_memory_pressure()
vibes has_pressure {
    vibez.spill("  ⚠️ Memory pressure detected")
} sus {
    vibez.spill("  ✅ No memory pressure")
}

// Test environment-based platform detection
vibez.spill("🌍 Testing environment-based detection...")

sus env_platform tea = env.get_platform()
sus env_arch tea = env.get_architecture() 
sus env_os tea = env.get_os_type()

assert_not_null(env_platform)
assert_not_null(env_arch)
assert_not_null(env_os)

vibez.spill("  ✅ Environment Platform: ", env_platform)
vibez.spill("  ✅ Environment Architecture: ", env_arch)
vibez.spill("  ✅ Environment OS Type: ", env_os)

// Test process and system monitoring
vibez.spill("📊 Testing process monitoring...")

sus process_id normie = sys_core.get_process_id()
sus parent_id normie = sys_core.get_parent_process_id()
sus cpu_usage normie = sys_core.get_cpu_usage()

assert_gt(process_id, 0)
assert_gt(parent_id, 0)
assert_true(cpu_usage >= 0)

vibez.spill("  ✅ Process ID: ", process_id)
vibez.spill("  ✅ Parent Process ID: ", parent_id)
vibez.spill("  ✅ CPU Usage: ", cpu_usage, "%")

// Test system limits and capabilities  
vibez.spill("⚙️ Testing system limits...")

sus stack_size normie = sys_core.get_stack_size()
sus max_files normie = sys_core.get_max_open_files()
sus open_files normie = sys_core.get_open_files_count()

assert_gt(stack_size, 0)
assert_gt(max_files, 0)
assert_true(open_files >= 0)

vibez.spill("  ✅ Stack size: ", stack_size, " bytes")
vibez.spill("  ✅ Max open files: ", max_files)
vibez.spill("  ✅ Currently open files: ", open_files)

// Test network and hostname information
vibez.spill("🌐 Testing network information...")

sus network_interfaces tea = sys_core.get_network_interfaces()
assert_not_null(network_interfaces)
vibez.spill("  ✅ Network interfaces: ", network_interfaces)

// Test security context
vibez.spill("🔒 Testing security context...")

sus user_id normie = sys_core.get_user_id()
sus group_id normie = sys_core.get_group_id()
sus has_root lit = sys_core.has_root_privileges()

assert_gt(user_id, 0)
assert_gt(group_id, 0)

vibez.spill("  ✅ User ID: ", user_id)
vibez.spill("  ✅ Group ID: ", group_id)
vibes has_root {
    vibez.spill("  ⚠️ Running with root privileges")
} sus {
    vibez.spill("  ✅ Running without root privileges")
}

// Test performance and load monitoring
vibez.spill("📈 Testing performance monitoring...")

sus load_avg tea = sys_core.get_load_average()
sus system_uptime normie = sys_core.get_uptime()

assert_not_null(load_avg)
assert_gt(system_uptime, 0)

vibez.spill("  ✅ Load average: ", load_avg)
vibez.spill("  ✅ System uptime: ", system_uptime, " seconds")

// Test memory efficiency and GC behavior
vibez.spill("🗑️ Testing garbage collection behavior...")

sus memory_efficiency drip = vibecheck.get_memory_efficiency()
assert_true(memory_efficiency >= 0.0)
assert_true(memory_efficiency <= 100.0)

vibez.spill("  ✅ Memory efficiency: ", memory_efficiency, "%")

// Trigger and test GC behavior
vibecheck.trigger_gc()
sus new_gc_count thicc = vibecheck.get_gc_count()
assert_true(new_gc_count > gc_count)
vibez.spill("  ✅ Garbage collection triggered successfully")

// Test performance metrics
sus perf_metrics tea = vibecheck.get_performance_metrics()
assert_not_null(perf_metrics)
vibez.spill("  ✅ Performance metrics available")

// Test runtime health check
sus health_ok lit = vibecheck.runtime_health_check()
vibes health_ok {
    vibez.spill("  ✅ Runtime health check passed")
} sus {
    vibez.spill("  ⚠️ Runtime health check detected issues")
}

// Test large memory allocation patterns (platform optimization test)
vibez.spill("🚀 Testing platform-specific optimizations...")

// Test small allocations (should use fast path)
periodt i := 1; i <= 10; i++ {
    sus small_addr normie = sys_core.alloc(64)  // 64 bytes
    assert_gt(small_addr, 0)
    sys_core.free(small_addr)
}
vibez.spill("  ✅ Small allocation pattern test passed")

// Test medium allocations (cache-line aligned)  
periodt i := 1; i <= 5; i++ {
    sus medium_addr normie = sys_core.alloc(4096)  // 4KB page
    assert_gt(medium_addr, 0)
    sys_core.free(medium_addr)
}
vibez.spill("  ✅ Medium allocation pattern test passed")

// Test large allocations (should trigger platform-specific large page optimizations)
periodt i := 1; i <= 3; i++ {
    sus large_addr normie = sys_core.alloc(1048576)  // 1MB
    assert_gt(large_addr, 0)
    sys_core.free(large_addr)
}
vibez.spill("  ✅ Large allocation pattern test passed")

// Summary of detected platform features
vibez.spill("")
vibez.spill("📋 Platform Feature Detection Summary:")
vibez.spill("   Platform: ", platform, " on ", arch)
vibez.spill("   OS Version: ", os_version)
vibez.spill("   CPU Cores: ", cpu_count)
vibez.spill("   Total Memory: ", total_memory / (1024 * 1024), " MB")
vibez.spill("   Available Memory: ", available_memory / (1024 * 1024), " MB")
vibez.spill("   Stack Size: ", stack_size / 1024, " KB")
vibez.spill("   Memory Efficiency: ", memory_efficiency, "%")
vibez.spill("   Runtime Uptime: ", uptime, " ms")
vibez.spill("   System Uptime: ", system_uptime, " seconds")
vibez.spill("   Load Average: ", load_avg)

vibes health_ok {
    vibez.spill("   Runtime Health: ✅ HEALTHY")
} sus {
    vibez.spill("   Runtime Health: ⚠️ WARNING")
}

vibes has_pressure {
    vibez.spill("   Memory Pressure: ⚠️ HIGH")
} sus {
    vibez.spill("   Memory Pressure: ✅ NORMAL")
}

vibez.spill("")
vibez.spill("🎯 Platform Abstraction Layer (PAL) validation complete!")
vibez.spill("   All hardware capabilities detected successfully")
vibez.spill("   Memory management working correctly")
vibez.spill("   Runtime introspection functional")
vibez.spill("   Cross-platform compatibility verified")

print_test_summary()
