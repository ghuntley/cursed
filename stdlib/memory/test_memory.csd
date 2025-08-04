yeet "testz"
yeet "memory"

fr fr ========================================
fr fr Memory Module Comprehensive Tests
fr fr ========================================

test_start("Basic Memory Allocation Tests")

fr fr Test malloc
sus addr1 normie = malloc(1024)
assert_true(addr1 > 0)

sus addr2 normie = malloc(2048)
assert_true(addr2 > 0)
assert_true(addr2 != addr1)

fr fr Test invalid malloc
sus invalid_addr normie = malloc(0)
assert_eq_int(invalid_addr, 0)

test_start("Memory Deallocation Tests")

fr fr Test free
sus addr_to_free normie = malloc(512)
assert_true(addr_to_free > 0)
assert_true(free(addr_to_free))

fr fr Test freeing invalid address
assert_false(free(0))

test_start("Memory Reallocation Tests")

fr fr Test realloc with existing address
sus original normie = malloc(256)
sus resized normie = realloc(original, 512)
assert_true(resized > 0)

fr fr Test realloc with null address (should act like malloc)
sus new_alloc normie = realloc(0, 1024)
assert_true(new_alloc > 0)

fr fr Test realloc with zero size (should act like free)
sus to_free normie = malloc(128)
sus freed normie = realloc(to_free, 0)
assert_eq_int(freed, 0)

test_start("Calloc Tests")

fr fr Test calloc
sus zeroed normie = calloc(10, 64)
assert_true(zeroed > 0)

fr fr Test calloc with invalid parameters
sus invalid_calloc1 normie = calloc(0, 64)
assert_eq_int(invalid_calloc1, 0)

sus invalid_calloc2 normie = calloc(10, 0)
assert_eq_int(invalid_calloc2, 0)

test_start("Aligned Allocation Tests")

fr fr Test aligned allocation
sus aligned normie = aligned_alloc(16, 1024)
assert_true(aligned > 0)

fr fr Test invalid aligned allocation
sus invalid_aligned normie = aligned_alloc(0, 1024)
assert_eq_int(invalid_aligned, 0)

test_start("Memory Operation Tests")

fr fr Test memcpy
sus src_addr normie = malloc(256)
sus dest_addr normie = malloc(256)
assert_true(memcpy(dest_addr, src_addr, 128))
assert_false(memcpy(0, src_addr, 128))

fr fr Test memmove
assert_true(memmove(dest_addr, src_addr, 128))
assert_false(memmove(0, src_addr, 128))

fr fr Test memcmp
assert_eq_int(memcmp(src_addr, src_addr, 128), 0)  fr fr Same address
assert_true(memcmp(src_addr, dest_addr, 128) != 0)  fr fr Different addresses

fr fr Test memset
assert_true(memset(src_addr, 0, 128))
assert_false(memset(0, 0, 128))

test_start("Memory Statistics Tests")

fr fr Test memory stats
sus stats MemoryPool = get_memory_stats()
assert_true(stats.total_allocated > 0)
assert_true(stats.allocation_count > 0)

fr fr Test current memory usage
sus usage normie = get_current_memory_usage()
assert_true(usage >= 0)

test_start("Address Validation Tests")

fr fr Test valid address check
sus valid_addr normie = malloc(128)
assert_true(is_valid_address(valid_addr))
assert_false(is_valid_address(0))
assert_false(is_valid_address(99999))

fr fr Test block size retrieval
sus size_addr normie = malloc(512)
sus retrieved_size normie = get_block_size(size_addr)
assert_eq_int(retrieved_size, 512)

sus invalid_size normie = get_block_size(0)
assert_eq_int(invalid_size, 0)

test_start("Memory Leak Detection Tests")

fr fr Test leak detection
sus leak_addr1 normie = malloc(64)
sus leak_addr2 normie = malloc(128)

sus leaks []MemoryBlock = find_memory_leaks()
assert_true(leaks.length() >= 2)

fr fr Free one and test again
free(leak_addr1)
sus leaks_after []MemoryBlock = find_memory_leaks()
assert_true(leaks_after.length() < leaks.length())

test_start("Stack Frame Tests")

fr fr Test stack frame operations
sus frame_addr normie = push_stack_frame(256)
assert_true(frame_addr > 0)

sus stack_size normie = get_stack_size()
assert_eq_int(stack_size, 256)

assert_true(pop_stack_frame())

sus empty_stack_size normie = get_stack_size()
assert_eq_int(empty_stack_size, 0)

test_start("Garbage Collection Tests")

fr fr Test garbage collection
sus gc_addr normie = malloc(1024)
sus collected_bytes normie = gc_collect()
assert_true(collected_bytes >= 0)

sus gc_stats GCStats = get_gc_stats()
assert_true(gc_stats.collections >= 1)

test_start("Memory Pressure Tests")

fr fr Test memory pressure monitoring
sus pressure normie = get_memory_pressure()
assert_true(pressure >= 0 && pressure <= 100)

sus should_gc lit = should_trigger_gc()
fr fr should_gc can be either true or false, just ensure it's boolean
assert_true(should_gc == based || should_gc == cap)

test_start("Heap Validation Tests")

fr fr Test heap validation
assert_true(validate_heap())

test_start("Memory Debug Info Tests")

fr fr Test memory info dump
sus info tea = dump_memory_info()
assert_true(info.contains("Memory Info"))
assert_true(info.contains("Total Allocated"))
assert_true(info.contains("Current Usage"))

test_start("Object Pool Tests")

fr fr Test object pool creation
sus pool ObjectPool = create_object_pool(64, 10)
assert_eq_int(pool.object_size, 64)
assert_eq_int(pool.pool_size, 10)
assert_eq_int(pool.available.length(), 10)

fr fr Test object pool allocation
sus pool_addr1 normie = pool.allocate()
assert_true(pool_addr1 > 0)
assert_eq_int(pool.available.length(), 9)
assert_eq_int(pool.allocated.length(), 1)

sus pool_addr2 normie = pool.allocate()
assert_true(pool_addr2 > 0)
assert_true(pool_addr2 != pool_addr1)

fr fr Test object pool deallocation
assert_true(pool.deallocate(pool_addr1))
assert_eq_int(pool.available.length(), 9)
assert_eq_int(pool.allocated.length(), 1)

assert_false(pool.deallocate(99999))  fr fr Invalid address

test_start("Memory Cleanup Tests")

fr fr Test memory cleanup
assert_true(cleanup_memory())

fr fr After cleanup, current usage should be 0
sus final_usage normie = get_current_memory_usage()
assert_eq_int(final_usage, 0)

print_test_summary()
