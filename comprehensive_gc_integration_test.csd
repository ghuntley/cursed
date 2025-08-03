fr fr Comprehensive GC Integration Test
fr fr Tests memory allocation, deallocation, compaction, finalizers, and stack scanning

yeet "testz"
yeet "gc"

fr fr Test data structures for GC testing
squad TestObject {
    spill id normie
    spill data []normie
    spill child ?*TestObject
}

squad LargeObject {
    spill buffer [1024]normie
    spill metadata tea
}

fr fr Finalizer counter for testing
sus finalizer_count drip = 0

slay test_finalizer(obj *anyopaque) {
    finalizer_count = finalizer_count + 1
    vibez.spillf("Finalizer called for object {*}, count: {}", obj, finalizer_count)
}

fr fr Test basic allocation and collection
slay test_basic_allocation() {
    test_start("Basic Allocation and Collection")
    
    fr fr Allocate many small objects
    sus objects []TestObject = []
    bestie i := 0; i < 1000; i = i + 1 {
        sus obj TestObject = TestObject{
            id: i,
            data: [i, i * 2, i * 3],
            child: nil
        }
        objects.append(obj)
    }
    
    fr fr Force garbage collection
    gc.collect()
    
    fr fr Verify objects are still accessible
    sus sum drip = 0
    bestie obj in objects {
        sum = sum + obj.id
    }
    
    assert_eq_int(sum, 499500) fr fr Sum of 0 to 999
    vibez.spillf("Successfully allocated and accessed {} objects", objects.len())
}

fr fr Test heap compaction
slay test_heap_compaction() {
    test_start("Heap Compaction")
    
    fr fr Create fragmentation by allocating and deallocating objects
    sus objects []?TestObject = []
    
    fr fr Allocate objects
    bestie i := 0; i < 100; i = i + 1 {
        objects.append(TestObject{
            id: i,
            data: [i],
            child: nil
        })
    }
    
    fr fr Deallocate every other object to create fragmentation
    bestie i := 0; i < objects.len(); i = i + 2 {
        objects[i] = nil
    }
    
    fr fr Force collection and compaction
    gc.collect()
    gc.compact()
    
    fr fr Verify remaining objects are still valid
    sus valid_count drip = 0
    bestie obj in objects {
        if obj != nil {
            valid_count = valid_count + 1
        }
    }
    
    assert_eq_int(valid_count, 50)
    vibez.spillf("Compaction completed, {} objects remain", valid_count)
}

fr fr Test finalizer registration and execution
slay test_finalizers() {
    test_start("Finalizer System")
    
    finalizer_count = 0
    
    fr fr Create objects with finalizers
    {
        sus objects []LargeObject = []
        bestie i := 0; i < 10; i = i + 1 {
            sus obj LargeObject = LargeObject{
                buffer: [0; 1024],
                metadata: "test_object_" + i.to_string()
            }
            
            fr fr Register finalizer
            gc.add_finalizer(&obj, test_finalizer)
            objects.append(obj)
        }
        
        vibez.spillf("Created {} objects with finalizers", objects.len())
    } fr fr Objects go out of scope here
    
    fr fr Force collection to trigger finalizers
    gc.collect()
    
    fr fr Wait a moment for finalization thread
    sleep(100) fr fr 100ms
    
    assert_true(finalizer_count > 0)
    vibez.spillf("Finalizers executed: {}", finalizer_count)
}

fr fr Test stack root scanning
slay test_stack_scanning() {
    test_start("Stack Root Scanning")
    
    fr fr Create local objects that should be preserved by stack scanning
    sus local_obj TestObject = TestObject{
        id: 12345,
        data: [1, 2, 3, 4, 5],
        child: nil
    }
    
    fr fr Create a child object
    sus child_obj TestObject = TestObject{
        id: 67890,
        data: [6, 7, 8, 9, 10],
        child: nil
    }
    
    local_obj.child = &child_obj
    
    fr fr Force collection - objects should survive due to stack references
    gc.collect()
    
    fr fr Verify objects are still accessible
    assert_eq_int(local_obj.id, 12345)
    assert_eq_int(local_obj.child.?.id, 67890)
    assert_eq_int(local_obj.data[2], 3)
    
    vibez.spill("Stack scanning preserved local objects correctly")
}

fr fr Test memory pressure and allocation tracking
slay test_memory_pressure() {
    test_start("Memory Pressure and Allocation Tracking")
    
    sus initial_stats = gc.get_stats()
    
    fr fr Allocate large amounts of memory
    sus large_objects []LargeObject = []
    bestie i := 0; i < 100; i = i + 1 {
        sus obj LargeObject = LargeObject{
            buffer: [i; 1024],
            metadata: "large_object_" + i.to_string()
        }
        large_objects.append(obj)
        
        fr fr Trigger collection periodically
        if i % 20 == 0 {
            gc.collect()
        }
    }
    
    sus final_stats = gc.get_stats()
    
    fr fr Verify allocation tracking
    assert_true(final_stats.total_allocations > initial_stats.total_allocations)
    assert_true(final_stats.total_bytes_allocated > initial_stats.total_bytes_allocated)
    assert_true(final_stats.gc_cycles > initial_stats.gc_cycles)
    
    vibez.spillf("Memory tracking: {} allocations, {} bytes, {} GC cycles",
        final_stats.total_allocations - initial_stats.total_allocations,
        final_stats.total_bytes_allocated - initial_stats.total_bytes_allocated,
        final_stats.gc_cycles - initial_stats.gc_cycles)
}

fr fr Test weak references
slay test_weak_references() {
    test_start("Weak References")
    
    sus weak_refs []gc.WeakRef = []
    
    fr fr Create objects and weak references
    {
        sus objects []TestObject = []
        bestie i := 0; i < 5; i = i + 1 {
            sus obj TestObject = TestObject{
                id: i + 1000,
                data: [i],
                child: nil
            }
            objects.append(obj)
            
            fr fr Create weak reference
            sus weak_ref = gc.create_weak_ref(&obj)
            weak_refs.append(weak_ref)
        }
        
        fr fr Verify weak references are valid
        bestie i := 0; i < weak_refs.len(); i = i + 1 {
            sus target = weak_refs[i].get()
            assert_true(target != nil)
            if target != nil {
                sus obj = @as(*TestObject, @ptrCast(target))
                assert_eq_int(obj.id, i + 1000)
            }
        }
    } fr fr Objects go out of scope
    
    fr fr Force collection
    gc.collect()
    
    fr fr Verify weak references are now null
    sus null_count drip = 0
    bestie weak_ref in weak_refs {
        if weak_ref.get() == nil {
            null_count = null_count + 1
        }
    }
    
    assert_true(null_count > 0)
    vibez.spillf("Weak references nullified: {}/{}", null_count, weak_refs.len())
}

fr fr Test concurrent allocation and collection
slay test_concurrent_operations() {
    test_start("Concurrent Operations")
    
    fr fr Start background allocation thread
    stan {
        bestie i := 0; i < 50; i = i + 1 {
            sus obj TestObject = TestObject{
                id: i + 2000,
                data: [i, i * 2],
                child: nil
            }
            
            fr fr Small delay to allow interleaving
            sleep(1)
        }
    }
    
    fr fr Perform collections while allocation is happening
    bestie i := 0; i < 10; i = i + 1 {
        sleep(5) fr fr 5ms delay
        gc.collect()
    }
    
    fr fr Wait for allocation thread to complete
    sleep(100)
    
    vibez.spill("Concurrent allocation and collection completed successfully")
}

fr fr Main test runner
slay main() {
    vibez.spill("Starting comprehensive GC integration tests...")
    
    fr fr Initialize GC with test configuration
    gc.configure({
        initial_heap_size: 10 * 1024 * 1024, fr fr 10MB
        max_heap_size: 50 * 1024 * 1024, fr fr 50MB
        gc_trigger_threshold: 0.7,
        enable_write_barriers: based,
        enable_finalization: based,
        concurrent_threads: 2
    })
    
    test_basic_allocation()
    test_heap_compaction()
    test_finalizers()
    test_stack_scanning()
    test_memory_pressure()
    test_weak_references()
    test_concurrent_operations()
    
    fr fr Print final GC statistics
    vibez.spill("\nFinal GC Statistics:")
    gc.print_stats()
    
    print_test_summary()
    
    vibez.spill("All GC integration tests completed!")
}

main()
