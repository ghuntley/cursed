yeet "testz"
yeet "memory"

# Test memory management functionality
test_start("memory comprehensive tests")

# Test basic memory allocation and deallocation
sus pointer1 normie = memory_allocate(1024)
assert_true(pointer1 != 0)

sus dealloc_success lit = memory_deallocate(pointer1)
assert_true(dealloc_success)

# Test invalid allocations
sus invalid_pointer normie = memory_allocate(0)
assert_eq_int(invalid_pointer, 0)

sus negative_pointer normie = memory_allocate(-100)
assert_eq_int(negative_pointer, 0)

# Test null pointer deallocation
sus null_dealloc lit = memory_deallocate(0)
assert_false(null_dealloc)

# Test memory reallocation
sus pointer2 normie = memory_allocate(512)
assert_true(pointer2 != 0)

sus realloc_pointer normie = memory_reallocate(pointer2, 1024)
assert_true(realloc_pointer != 0)

# Test reallocation edge cases
sus null_realloc normie = memory_reallocate(0, 256)
assert_true(null_realloc != 0)
memory_deallocate(null_realloc)

sus zero_realloc normie = memory_reallocate(realloc_pointer, 0)
assert_eq_int(zero_realloc, 0)

# Test memory operations
sus src_pointer normie = memory_allocate(64)
sus dest_pointer normie = memory_allocate(64)

assert_true(memory_copy(dest_pointer, src_pointer, 32))
assert_false(memory_copy(0, src_pointer, 32))  # Null destination
assert_false(memory_copy(dest_pointer, 0, 32))  # Null source

assert_true(memory_zero(dest_pointer, 64))
assert_false(memory_zero(0, 64))  # Null pointer

memory_deallocate(src_pointer)
memory_deallocate(dest_pointer)

# Test memory comparison
sus ptr_a normie = memory_allocate(32)
sus ptr_b normie = memory_allocate(32)

assert_eq_int(memory_compare(ptr_a, ptr_a, 32), 0)  # Same pointer
assert_eq_int(memory_compare(ptr_a, ptr_b, 32), 1)  # Different pointers
assert_eq_int(memory_compare(0, ptr_a, 32), -1)   # Null pointer error

memory_deallocate(ptr_a)
memory_deallocate(ptr_b)

# Test garbage collector
sus gc GCState = memory_get_gc()
assert_true(gc != cringe)

sus initial_allocated normie = gc_get_total_allocated(gc)
sus gc_pointer normie = gc_allocate(gc, 256)
assert_true(gc_pointer != 0)

sus after_alloc normie = gc_get_total_allocated(gc)
assert_true(after_alloc >= initial_allocated)

assert_true(gc_deallocate(gc, gc_pointer))
sus after_dealloc normie = gc_get_total_allocated(gc)
assert_true(after_dealloc <= after_alloc)

# Test garbage collection
sus freed_bytes normie = gc_collect(gc)
assert_true(freed_bytes >= 0)

# Test memory allocator
sus allocator MemoryAllocator = allocator_create()
assert_true(allocator != cringe)

sus alloc_ptr normie = allocator_malloc(128)
assert_true(alloc_ptr != 0)

assert_true(allocator_free(alloc_ptr))

# Test allocator edge cases
assert_eq_int(allocator_malloc(0), 0)
assert_false(allocator_free(0))

# Test allocator reallocation
sus orig_ptr normie = allocator_malloc(64)
sus new_ptr normie = allocator_realloc(orig_ptr, 128)
assert_true(new_ptr != 0)

sus zero_ptr normie = allocator_realloc(new_ptr, 0)
assert_eq_int(zero_ptr, 0)

sus from_null normie = allocator_realloc(0, 64)
assert_true(from_null != 0)
allocator_free(from_null)

# Test memory pool
sus pool MemoryPool = memory_pool_create(32, 10)
assert_true(pool != cringe)
assert_false(memory_pool_is_empty(pool))

sus pool_block normie = memory_pool_acquire(pool)
assert_true(pool_block != 0)

assert_true(memory_pool_release(pool, pool_block))
assert_false(memory_pool_release(pool, 0))  # Null pointer

# Test pool exhaustion
sus blocks [normie] = []
sus i normie = 0
bestie i < 10 {  # Exhaust the pool
    sus block normie = memory_pool_acquire(pool)
    lowkey block != 0 {
        blocks = append_block(blocks, block)
    }
    i = i + 1
}

assert_true(memory_pool_is_empty(pool))
sus exhausted_block normie = memory_pool_acquire(pool)
assert_eq_int(exhausted_block, 0)

# Test memory safety
sus safety MemorySafety = memory_safety_create()
assert_true(safety != cringe)

sus safe_ptr normie = memory_allocate(100)
assert_true(memory_check_bounds(safety, safe_ptr, 50))
assert_false(memory_check_bounds(safety, safe_ptr, 200))  # Exceeds bounds
assert_false(memory_check_bounds(safety, 0, 50))         # Null pointer

assert_true(memory_check_null(safety, safe_ptr))
assert_false(memory_check_null(safety, 0))

assert_true(memory_check_double_free(safety, safe_ptr))
memory_deallocate(safe_ptr)

# Test memory block metadata
sus block MemoryBlock = memory_block_create(256, "ast_node")
assert_true(block != cringe)
assert_eq_int(memory_block_get_size(block), 256)
assert_eq_string(memory_block_get_type(block), "ast_node")
assert_true(memory_block_is_valid(block))

# Test memory statistics
sus stats tea = memory_get_stats()
assert_true(string_length(stats) > 0)

assert_true(memory_print_stats())

sus gc_freed normie = memory_force_gc()
assert_true(gc_freed >= 0)

# Test compiler-specific memory utilities
sus ast_node_ptr normie = memory_allocate_ast_node("expression")
assert_true(ast_node_ptr != 0)
memory_deallocate(ast_node_ptr)

sus symbol_table_ptr normie = memory_allocate_symbol_table(50)
assert_true(symbol_table_ptr != 0)
memory_deallocate(symbol_table_ptr)

sus string_buffer_ptr normie = memory_allocate_string_buffer(100)
assert_true(string_buffer_ptr != 0)
memory_deallocate(string_buffer_ptr)

# Test AST node size calculation
assert_eq_int(get_ast_node_size("expression"), 32)
assert_eq_int(get_ast_node_size("statement"), 48)
assert_eq_int(get_ast_node_size("declaration"), 64)
assert_eq_int(get_ast_node_size("unknown"), 32)

# Test string duplication
sus original_string normie = memory_allocate_string_buffer(20)
sus duplicated_string normie = memory_string_duplicate(original_string)
assert_true(duplicated_string != 0)
assert_true(duplicated_string != original_string)

memory_deallocate(original_string)
memory_deallocate(duplicated_string)

# Test helper functions
assert_eq_int(min_size(10, 20), 10)
assert_eq_int(min_size(30, 15), 15)
assert_eq_int(min_size(5, 5), 5)

sus timestamp normie = get_current_time()
assert_true(timestamp > 0)

assert_true(is_valid_pointer(100))
assert_false(is_valid_pointer(0))

sus length normie = string_pointer_length(42)  # Mock string pointer
assert_true(length > 0)

# Test memory operations with realistic scenarios
sus compiler_memory normie = memory_allocate(8192)  # Large block for compiler
assert_true(compiler_memory != 0)

sus temp_buffer normie = memory_allocate(1024)
assert_true(temp_buffer != 0)

# Test memory copy between large blocks
assert_true(memory_copy(compiler_memory, temp_buffer, 512))

# Test memory zeroing
assert_true(memory_zero(temp_buffer, 1024))

# Clean up
memory_deallocate(compiler_memory)
memory_deallocate(temp_buffer)

print_test_summary()

# Helper function for block array operations
slay append_block(blocks [normie], block normie) [normie] {
    # Would actually append block to array
    damn blocks
}
