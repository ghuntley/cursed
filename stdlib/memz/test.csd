fr fr memz module tests - Memory Management Testing
yeet "memz"
yeet "testz"

fr fr Basic allocation tests
slay test_basic_malloc() {
    memz.init_memz()
    
    sus ptr normie = memz.malloc(1024)
    testz.assert_ne_int(ptr, 0)
    
    sus stats = memz.get_memory_stats()
    testz.assert_eq_int(stats.total_allocated, 1)
    testz.assert_gt_int(stats.current_usage, 0)
    
    memz.free(ptr)
    testz.pass("Basic malloc/free test")
}

fr fr Arena allocator tests
slay test_arena_allocation() {
    sus arena = memz.create_arena(4096)
    testz.assert_eq_bool(arena.is_initialized, based)
    
    sus ptr1 normie = memz.arena_alloc(&arena, 256)
    testz.assert_ne_int(ptr1, 0)
    
    sus ptr2 normie = memz.arena_alloc(&arena, 512)  
    testz.assert_ne_int(ptr2, 0)
    testz.assert_ne_int(ptr1, ptr2)
    
    memz.arena_reset(&arena)
    testz.assert_eq_int(arena.offset, 0)
    
    testz.pass("Arena allocation test")
}

fr fr Memory statistics tests
slay test_memory_stats() {
    memz.init_memz()
    
    sus ptr1 normie = memz.malloc(1024)
    sus ptr2 normie = memz.malloc(2048)
    
    sus stats = memz.get_memory_stats()
    testz.assert_eq_int(stats.total_allocated, 2)
    testz.assert_gt_int(stats.current_usage, 3000)
    
    memz.free(ptr1)
    
    sus stats2 = memz.get_memory_stats()
    testz.assert_eq_int(stats2.total_freed, 1)
    testz.assert_lt_int(stats2.current_usage, stats.current_usage)
    
    testz.pass("Memory statistics test")
}

fr fr Garbage collection tests  
slay test_garbage_collection() {
    memz.init_memz()
    
    # Allocate and free some blocks
    sus ptr1 normie = memz.malloc(1024)
    sus ptr2 normie = memz.malloc(2048) 
    sus ptr3 normie = memz.malloc(512)
    
    memz.free(ptr1)
    memz.free(ptr3)
    
    sus freed_bytes normie = memz.gc_collect()
    testz.assert_gt_int(freed_bytes, 0)
    
    testz.pass("Garbage collection test")
}

fr fr Edge case tests
slay test_edge_cases() {
    memz.init_memz()
    
    # Test zero allocation
    sus zero_ptr normie = memz.malloc(0)
    testz.assert_eq_int(zero_ptr, 0)
    
    # Test double free (should be safe)
    sus ptr normie = memz.malloc(1024)
    memz.free(ptr)
    memz.free(ptr)  # Should not crash
    
    # Test free of null pointer
    memz.free(0)  # Should not crash
    
    testz.pass("Edge cases test")
}

fr fr Run all tests
slay test_all() {
    testz.test_start("memz module tests")
    
    test_basic_malloc()
    test_arena_allocation() 
    test_memory_stats()
    test_garbage_collection()
    test_edge_cases()
    
    testz.print_test_summary()
}

test_all()
