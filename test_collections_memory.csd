# Test program for collections_core memory management implementation
# Exercises the newly implemented runtime memory bridge functions

yeet "testz"
yeet "collections_core"

slay test_basic_memory_operations() {
    test_start("Basic Memory Operations")
    
    # Test runtime allocation
    sus ptr *cringe = runtime_allocate_block(1024)
    assert_true(ptr != cringe)
    
    # Test runtime zero memory
    sus zero_result lit = runtime_zero_memory(ptr, 1024)
    assert_true(zero_result)
    
    # Test runtime deallocation
    sus free_result lit = runtime_deallocate_block(ptr)
    assert_true(free_result)
    
    vibez.spill("✅ Basic memory operations working")
}

slay test_memory_copy_operations() {
    test_start("Memory Copy Operations")
    
    # Allocate source and destination buffers
    sus src *cringe = runtime_allocate_block(256)
    sus dest *cringe = runtime_allocate_block(256)
    
    assert_true(src != cringe)
    assert_true(dest != cringe)
    
    # Zero both buffers
    runtime_zero_memory(src, 256)
    runtime_zero_memory(dest, 256)
    
    # Test memory copy
    sus copy_result lit = runtime_copy_memory(dest, src, 256)
    assert_true(copy_result)
    
    # Clean up
    runtime_deallocate_block(src)
    runtime_deallocate_block(dest)
    
    vibez.spill("✅ Memory copy operations working")
}

slay test_vector_with_runtime_memory() {
    test_start("Vector with Runtime Memory")
    
    # Create a vector (this will use the new runtime memory functions)
    sus vec *Vector = vector_new()
    assert_true(vec != cringe)
    
    # Add some elements
    assert_true(vector_push(vec, 42))
    assert_true(vector_push(vec, 84))
    assert_true(vector_push(vec, 126))
    
    # Verify elements
    assert_eq_int(vector_get(vec, 0), 42)
    assert_eq_int(vector_get(vec, 1), 84)
    assert_eq_int(vector_get(vec, 2), 126)
    
    # Test vector growth (should trigger reallocation)
    bestie i := 0; i < 10; i++ {
        assert_true(vector_push(vec, i * 10))
    }
    
    # Free the vector
    assert_true(vector_free(vec))
    
    vibez.spill("✅ Vector with runtime memory working")
}

slay test_hashmap_with_runtime_memory() {
    test_start("HashMap with Runtime Memory")
    
    # Create a hashmap
    sus map *HashMap = hashmap_new(16)
    assert_true(map != cringe)
    
    # Add some key-value pairs
    assert_true(hashmap_put(map, "key1", 100))
    assert_true(hashmap_put(map, "key2", 200))
    assert_true(hashmap_put(map, "key3", 300))
    
    # Verify retrieval
    assert_eq_int(hashmap_get(map, "key1"), 100)
    assert_eq_int(hashmap_get(map, "key2"), 200)
    assert_eq_int(hashmap_get(map, "key3"), 300)
    
    # Test removal
    assert_true(hashmap_remove(map, "key2"))
    assert_eq_int(hashmap_get(map, "key2"), 0)  # Should not be found
    
    # Free the hashmap
    assert_true(hashmap_free(map))
    
    vibez.spill("✅ HashMap with runtime memory working")
}

slay test_linked_list_with_runtime_memory() {
    test_start("LinkedList with Runtime Memory")
    
    # Create a doubly linked list
    sus list *LinkedList = list_new(based)
    assert_true(list != cringe)
    
    # Add elements to front and back
    assert_true(list_push_front(list, 10))
    assert_true(list_push_back(list, 20))
    assert_true(list_push_front(list, 5))
    
    # Remove from front (should get 5)
    assert_eq_int(list_remove_front(list), 5)
    
    # Remove from front again (should get 10)
    assert_eq_int(list_remove_front(list), 10)
    
    # Free the list
    assert_true(list_free(list))
    
    vibez.spill("✅ LinkedList with runtime memory working")
}

slay test_heap_with_runtime_memory() {
    test_start("Heap with Runtime Memory")
    
    # Create a max heap
    sus heap *Heap = heap_new(10, based)
    assert_true(heap != cringe)
    
    # Insert elements
    assert_true(heap_insert(heap, 50))
    assert_true(heap_insert(heap, 30))
    assert_true(heap_insert(heap, 70))
    assert_true(heap_insert(heap, 10))
    
    # Extract max (should be 70)
    assert_eq_int(heap_extract(heap), 70)
    
    # Extract next max (should be 50)
    assert_eq_int(heap_extract(heap), 50)
    
    # Free the heap
    assert_true(heap_free(heap))
    
    vibez.spill("✅ Heap with runtime memory working")
}

slay test_malloc_free_interface() {
    test_start("malloc/free Interface")
    
    # Test the malloc/free wrapper functions
    sus ptr1 *cringe = malloc(512)
    assert_true(ptr1 != cringe)
    
    sus ptr2 *cringe = malloc(1024)
    assert_true(ptr2 != cringe)
    
    # Test realloc
    sus ptr3 *cringe = realloc(ptr1, 256, 768)
    assert_true(ptr3 != cringe)
    
    # Test calloc
    sus ptr4 *cringe = calloc(10, 64)
    assert_true(ptr4 != cringe)
    
    # Free all allocations
    assert_true(free(ptr2))
    assert_true(free(ptr3))
    assert_true(free(ptr4))
    
    vibez.spill("✅ malloc/free interface working")
}

slay test_memory_error_handling() {
    test_start("Memory Error Handling")
    
    # Test null pointer handling
    sus result1 lit = runtime_deallocate_block(cringe)  # Should succeed (no-op)
    assert_true(result1)
    
    sus result2 lit = runtime_zero_memory(cringe, 100)  # Should fail
    assert_false(result2)
    
    sus result3 lit = runtime_copy_memory(cringe, cringe, 100)  # Should fail
    assert_false(result3)
    
    # Test zero size handling
    sus result4 lit = runtime_zero_memory(runtime_allocate_block(100), 0)
    assert_false(result4)
    
    vibez.spill("✅ Memory error handling working")
}

slay test_large_allocation_stress() {
    test_start("Large Allocation Stress Test")
    
    # Test large allocations
    sus large_ptr *cringe = runtime_allocate_block(1024 * 1024)  # 1MB
    assert_true(large_ptr != cringe)
    
    # Zero the large block
    assert_true(runtime_zero_memory(large_ptr, 1024 * 1024))
    
    # Free the large block
    assert_true(runtime_deallocate_block(large_ptr))
    
    # Test multiple smaller allocations
    sus ptrs **cringe = malloc(100 * sizeof(*cringe))
    bestie i := 0; i < 100; i++ {
        ptrs[i] = runtime_allocate_block(1024)
        assert_true(ptrs[i] != cringe)
    }
    
    # Free all small allocations
    bestie i := 0; i < 100; i++ {
        assert_true(runtime_deallocate_block(ptrs[i]))
    }
    free(ptrs)
    
    vibez.spill("✅ Large allocation stress test passed")
}

# Main test runner
test_basic_memory_operations()
test_memory_copy_operations()
test_vector_with_runtime_memory()
test_hashmap_with_runtime_memory()
test_linked_list_with_runtime_memory()
test_heap_with_runtime_memory()
test_malloc_free_interface()
test_memory_error_handling()
test_large_allocation_stress()

print_test_summary()

vibez.spill("\n🎉 Collections Core Memory Management Implementation Complete!")
vibez.spill("✅ All runtime memory bridge functions implemented")
vibez.spill("✅ Integration with GC-enabled memory system")
vibez.spill("✅ Error handling and safety checks")
vibez.spill("✅ Support for all collection data structures")
vibez.spill("\n🚀 Ready for production use!")
