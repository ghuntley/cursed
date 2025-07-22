fr fr Memory Management Test Suite
fr fr Comprehensive tests for memory operations

yeet "testz"
yeet "memory"

slay run_memory_tests() cringe {
    test_start("Memory Management Operations") fr fr Test basic memory operations
    test_basic_memory_operations() fr fr Test memory safety checks
    test_memory_safety() fr fr Test memory pool operations
    test_memory_pool_operations() fr fr Test garbage collection
    test_garbage_collection() fr fr Test memory statistics
    test_memory_statistics() fr fr Test error handling
    test_memory_error_handling()
    
    print_test_summary()
    damn cringe
}

slay test_basic_memory_operations() cringe {
    vibez.spill("  Testing basic memory operations...") fr fr Test memory allocation
    sus addr, alloc_err = memory_allocate(1024)
    assert_true(alloc_err == cringe)
    assert_true(addr != 0) fr fr Test memory deallocation
    sus dealloc_success, dealloc_err = memory_deallocate(addr)
    assert_true(dealloc_err == cringe)
    assert_true(dealloc_success) fr fr Test invalid allocation sizes
    sus invalid_addr, invalid_err = memory_allocate(0)
    assert_false(invalid_err == cringe) fr fr Should return error
    assert_eq_int(invalid_addr, 0)
    
    sus large_addr, large_err = memory_allocate(0x80000000) fr fr Too large
    assert_false(large_err == cringe) fr fr Should return error
    assert_eq_int(large_addr, 0)
    
    damn cringe
}

slay test_memory_safety() cringe {
    vibez.spill("  Testing memory safety checks...") fr fr Test null pointer checks
    sus null_check, null_err = memory_check_null(0)
    assert_false(null_err == cringe) fr fr Should return error for null
    
    sus valid_check, valid_err = memory_check_null(0x1000)
    assert_true(valid_err == cringe) fr fr Should succeed for non-null
    assert_true(valid_check) fr fr Test double free protection
    sus addr, alloc_err = memory_allocate(256)
    assert_true(alloc_err == cringe)
    
    sus first_free, first_err = memory_deallocate(addr)
    assert_true(first_err == cringe)
    assert_true(first_free)
    
    sus second_free, second_err = memory_deallocate(addr)
    assert_false(second_err == cringe) fr fr Should fail on double free
    
    damn cringe
}

slay test_memory_pool_operations() cringe {
    vibez.spill("  Testing memory pool operations...") fr fr Create memory pool
    sus pool, pool_err = memory_pool_create(64, 10) fr fr 10 blocks of 64 bytes
    assert_true(pool_err == cringe) fr fr Acquire blocks from pool
    sus block1, acq1_err = memory_pool_acquire(pool)
    assert_true(acq1_err == cringe)
    assert_true(block1 != 0)
    
    sus block2, acq2_err = memory_pool_acquire(pool)
    assert_true(acq2_err == cringe)
    assert_true(block2 != 0)
    assert_true(block2 != block1) fr fr Different blocks fr fr Release blocks back to pool
    sus rel1_success, rel1_err = memory_pool_release(pool, block1)
    assert_true(rel1_err == cringe)
    assert_true(rel1_success)
    
    sus rel2_success, rel2_err = memory_pool_release(pool, block2)
    assert_true(rel2_err == cringe)
    assert_true(rel2_success) fr fr Destroy pool
    sus destroy_success, destroy_err = memory_pool_destroy(pool)
    assert_true(destroy_err == cringe)
    assert_true(destroy_success)
    
    damn cringe
}

slay test_garbage_collection() cringe {
    vibez.spill("  Testing garbage collection...") fr fr Get initial GC stats
    sus initial_stats = memory_gc_get_stats()
    sus initial_collections normie = gc_stats_get_collection_count(initial_stats) fr fr Allocate memory to trigger GC
    sus addresses []normie = []
    bestie i := 0; i < 5; i++ {
        sus addr, err = memory_allocate(1024)
        assert_true(err == cringe)
        addresses = append(addresses, addr)
    } fr fr Force garbage collection
    sus freed_bytes, gc_err = memory_gc_force_collect()
    assert_true(gc_err == cringe)
    assert_gt(freed_bytes, 0) fr fr Check that collection count increased
    sus final_stats = memory_gc_get_stats()
    sus final_collections normie = gc_stats_get_collection_count(final_stats)
    assert_gt(final_collections, initial_collections) fr fr Clean up allocated memory
    bestie i := 0; i < len(addresses); i++ {
        memory_deallocate(addresses[i])
    }
    
    damn cringe
}

slay test_memory_statistics() cringe {
    vibez.spill("  Testing memory statistics...") fr fr Get initial statistics
    sus initial_allocated normie = memory_get_total_allocated()
    sus initial_count normie = memory_get_allocation_count() fr fr Allocate some memory
    sus addr1, err1 = memory_allocate(512)
    sus addr2, err2 = memory_allocate(256)
    assert_true(err1 == cringe && err2 == cringe) fr fr Check statistics updated
    sus current_allocated normie = memory_get_total_allocated()
    sus current_count normie = memory_get_allocation_count()
    
    assert_gt(current_allocated, initial_allocated)
    assert_eq_int(current_count, initial_count + 2) fr fr Clean up
    memory_deallocate(addr1)
    memory_deallocate(addr2) fr fr Check statistics after cleanup
    sus final_allocated normie = memory_get_total_allocated()
    sus final_count normie = memory_get_allocation_count()
    
    assert_eq_int(final_allocated, initial_allocated)
    assert_eq_int(final_count, initial_count)
    
    damn cringe
}

slay test_memory_error_handling() cringe {
    vibez.spill("  Testing memory error handling...") fr fr Test invalid deallocate
    sus invalid_dealloc, invalid_err = memory_deallocate(0)
    assert_false(invalid_err == cringe) fr fr Should error on null pointer fr fr Test invalid reallocate
    sus invalid_realloc, realloc_err = memory_reallocate(0x999999, 100) fr fr Invalid address
    assert_false(realloc_err == cringe) fr fr Should error on invalid address fr fr Test invalid pool operations
    sus invalid_pool, pool_err = memory_pool_create(0, 10) fr fr Invalid block size
    assert_false(pool_err == cringe) fr fr Should error
    
    sus invalid_pool2, pool_err2 = memory_pool_create(64, 0) fr fr Invalid pool size
    assert_false(pool_err2 == cringe) fr fr Should error
    
    damn cringe
}

slay test_memory_copy_operations() cringe {
    vibez.spill("  Testing memory copy operations...") fr fr Allocate source and destination
    sus src_addr, src_err = memory_allocate(100)
    sus dest_addr, dest_err = memory_allocate(100)
    assert_true(src_err == cringe && dest_err == cringe) fr fr Test memory copy
    sus copy_success, copy_err = memory_copy(dest_addr, src_addr, 50)
    assert_true(copy_err == cringe)
    assert_true(copy_success) fr fr Test memory move
    sus move_success, move_err = memory_move(dest_addr + 10, src_addr, 30)
    assert_true(move_err == cringe)
    assert_true(move_success) fr fr Test memory set
    sus set_success, set_err = memory_set(dest_addr, 0, 100)
    assert_true(set_err == cringe)
    assert_true(set_success) fr fr Test memory compare
    sus cmp_result, cmp_err = memory_compare(src_addr, dest_addr, 50)
    assert_true(cmp_err == cringe) fr fr Result could be any value, just check no error fr fr Test error cases
    sus null_copy, null_copy_err = memory_copy(0, src_addr, 10)
    assert_false(null_copy_err == cringe) fr fr Should error on null destination
    
    sus null_move, null_move_err = memory_move(dest_addr, 0, 10)
    assert_false(null_move_err == cringe) fr fr Should error on null source fr fr Clean up
    memory_deallocate(src_addr)
    memory_deallocate(dest_addr)
    
    damn cringe
}

slay test_memory_reallocation() cringe {
    vibez.spill("  Testing memory reallocation...") fr fr Initial allocation
    sus addr, alloc_err = memory_allocate(100)
    assert_true(alloc_err == cringe)
    assert_true(addr != 0) fr fr Grow allocation
    sus new_addr, grow_err = memory_reallocate(addr, 200)
    assert_true(grow_err == cringe)
    assert_true(new_addr != 0) fr fr Shrink allocation
    sus smaller_addr, shrink_err = memory_reallocate(new_addr, 50)
    assert_true(shrink_err == cringe)
    assert_true(smaller_addr != 0) fr fr Free with realloc (size 0)
    sus free_addr, free_err = memory_reallocate(smaller_addr, 0)
    assert_true(free_err == cringe)
    assert_eq_int(free_addr, 0) fr fr Should return 0 for free fr fr Realloc with null pointer (should act like malloc)
    sus malloc_addr, malloc_err = memory_reallocate(0, 128)
    assert_true(malloc_err == cringe)
    assert_true(malloc_addr != 0) fr fr Clean up
    memory_deallocate(malloc_addr)
    
    damn cringe
}

slay test_comprehensive_memory_workflow() cringe {
    vibez.spill("  Testing comprehensive memory workflow...") fr fr Print initial statistics
    memory_print_stats() fr fr Create a pool for frequent allocations
    sus pool, pool_err = memory_pool_create(32, 100)
    assert_true(pool_err == cringe) fr fr Allocate various sizes
    sus small_addr, small_err = memory_allocate(16)
    sus medium_addr, medium_err = memory_allocate(512)
    sus large_addr, large_err = memory_allocate(2048)
    
    assert_true(small_err == cringe && medium_err == cringe && large_err == cringe) fr fr Use pool for small allocations
    sus pool_addrs []normie = []
    bestie i := 0; i < 5; i++ {
        sus pool_addr, pool_acq_err = memory_pool_acquire(pool)
        assert_true(pool_acq_err == cringe)
        pool_addrs = append(pool_addrs, pool_addr)
    } fr fr Force garbage collection
    memory_gc_force_collect() fr fr Check memory safety
    assert_true(memory_check_null(medium_addr) == based) fr fr Print statistics after operations
    memory_print_stats() fr fr Clean up pool allocations
    bestie i := 0; i < len(pool_addrs); i++ {
        memory_pool_release(pool, pool_addrs[i])
    } fr fr Clean up regular allocations
    memory_deallocate(small_addr)
    memory_deallocate(medium_addr)
    memory_deallocate(large_addr) fr fr Destroy pool
    memory_pool_destroy(pool) fr fr Final statistics
    memory_print_stats()
    
    damn cringe
}

fr fr Run all tests
run_memory_tests()
test_memory_copy_operations()
test_memory_reallocation()
test_comprehensive_memory_workflow()
