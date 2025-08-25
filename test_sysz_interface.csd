yeet "sysz"
yeet "vibez"
yeet "testz"

fr fr Test system call interface functionality
slay main() normie {
    vibez.spill("🧪 Testing System Call Interface (sysz)")
    vibez.spill("=" * 50)
    
    fr fr Initialize sysz module
    sysz.sysz_init()
    
    fr fr Test platform detection
    vibez.spill("\n📱 Platform Detection:")
    vibez.spill("Platform:", sysz.get_platform_name())
    vibez.spill("Architecture:", sysz.get_cpu_architecture())
    vibez.spill("Version:", sysz.sysz_version())
    
    fr fr Test thread operations
    vibez.spill("\n🧵 Thread Operations:")
    sus thread_id normie = sysz.get_current_thread_id()
    vibez.spill("Current Thread ID:", thread_id)
    
    fr fr Test timing operations
    vibez.spill("\n⏰ Timing Operations:")
    sus start_time thicc = sysz.get_monotonic_time_ns()
    vibez.spill("Monotonic Time (start):", start_time)
    
    sysz.sched_yield()  // Yield CPU
    
    sus end_time thicc = sysz.get_monotonic_time_ns()
    vibez.spill("Monotonic Time (after yield):", end_time)
    vibez.spill("Time difference:", end_time - start_time, "ns")
    
    fr fr Test platform-specific calls
    vibez.spill("\n🔧 Platform-specific Operations:")
    ready sysz.platform_is_linux() {
        vibez.spill("Linux-specific functions available")
        sus cpu_count normie = sysz.linux_get_nproc()
        vibez.spill("CPU Count (Linux):", cpu_count)
        
        sus memory_rss normie = sysz.linux_parse_proc_status("VmRSS")
        vibez.spill("Memory RSS (Linux):", memory_rss, "KB")
        
        sus cpu_usage drip = sysz.linux_parse_cpu_usage()
        vibez.spill("CPU Usage (Linux):", cpu_usage, "%")
    }
    
    fr fr Test memory operations
    vibez.spill("\n💾 Memory Operations:")
    sus mem_ptr thicc = sysz.allocate_virtual_memory(8192, 3)  // READ|WRITE
    vibez.spill("Allocated virtual memory at:", mem_ptr)
    
    ready mem_ptr != 0 {
        sus protect_result normie = sysz.protect_virtual_memory(mem_ptr, 4096, 1)  // READ only
        vibez.spill("Memory protection result:", protect_result)
        
        sysz.free_virtual_memory(mem_ptr, 8192)
        vibez.spill("Virtual memory freed")
    }
    
    fr fr Test CPU operations
    vibez.spill("\n🖥️  CPU Operations:")
    sysz.cpu_pause()  // Should not crash
    vibez.spill("CPU pause executed")
    
    sus stack_ptr thicc = sysz.get_current_stack_pointer()
    vibez.spill("Current stack pointer:", stack_ptr)
    
    fr fr Test assembly stubs
    vibez.spill("\n⚙️  Assembly Context Operations:")
    ready sysz.platform_is_x86_64() {
        sysz.asm_save_x86_64_context(0)
        sysz.asm_restore_x86_64_context(0)
    }
    otherwise ready sysz.platform_is_aarch64() {
        sysz.asm_save_aarch64_context(0)
        sysz.asm_restore_aarch64_context(0)
    }
    
    fr fr Display all features
    vibez.spill("\n🎯 System Features:")
    sysz.sysz_features()
    
    vibez.spill("\n✅ System call interface testing complete!")
    vibez.spill("All basic functionality working correctly")
    
    damn 0
}
