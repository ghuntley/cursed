yeet "testz"
yeet "memory_core"

fr fr Test Memory Core System
test_start("Memory Core System Tests")

fr fr Test 1: Memory system initialization
sus init_result lit = init_memory_system()
assert_true(init_result)
print_test_status("Memory system initialization", init_result)

fr fr Test 2: Basic memory allocation
sus obj_id1 normie = allocate_memory(1024, ALLOC_HEAP)
assert_true(obj_id1 > 0)
assert_true(memory_object_exists(obj_id1))
print_test_status("Basic memory allocation", obj_id1 > 0)

fr fr Test 3: Memory object retrieval
sus memory_obj MemoryObject = get_memory_object(obj_id1)
assert_eq_int(memory_obj.size, 1024)
assert_eq_int(memory_obj.alloc_type, ALLOC_HEAP)
assert_eq_int(memory_obj.ref_count, 1)
print_test_status("Memory object retrieval", memory_obj.size == 1024)

fr fr Test 4: Reference counting
sus initial_ref normie = get_ref_count(obj_id1)
assert_eq_int(initial_ref, 1)

sus inc_result lit = inc_ref_count(obj_id1)
assert_true(inc_result)
sus after_inc normie = get_ref_count(obj_id1)
assert_eq_int(after_inc, 2)

sus dec_result lit = dec_ref_count(obj_id1)
assert_true(dec_result)
sus after_dec normie = get_ref_count(obj_id1)
assert_eq_int(after_dec, 1)
print_test_status("Reference counting", based)

fr fr Test 5: Memory deallocation
sus dealloc_result lit = deallocate_memory(obj_id1)
assert_true(dealloc_result)
assert_true(!memory_object_exists(obj_id1))
print_test_status("Memory deallocation", dealloc_result)

fr fr Test 6: Multiple allocations
reset_memory_system()
init_memory_system()

sus obj_ids normie[value] = []
sus i normie = 0
bestie i < 5 {
    sus new_id normie = allocate_memory(512, ALLOC_HEAP)
    assert_true(new_id > 0)
    obj_ids = append(obj_ids, new_id)
    i = i + 1
}

sus stats map[tea]normie = get_memory_stats()
assert_eq_int(stats["live_objects"], 5)
assert_eq_int(stats["heap_size"], 2560) fr fr 5 * 512
print_test_status("Multiple allocations", stats["live_objects"] == 5)

fr fr Test 7: Reference counting auto-deallocation
sus auto_id normie = allocate_memory(256, ALLOC_HEAP)
inc_ref_count(auto_id) fr fr ref_count = 2
dec_ref_count(auto_id) fr fr ref_count = 1
assert_true(memory_object_exists(auto_id))

dec_ref_count(auto_id) fr fr ref_count = 0, should auto-deallocate
assert_true(!memory_object_exists(auto_id))
print_test_status("Auto-deallocation", based)

fr fr Test 8: GC threshold and triggering
reset_memory_system()
init_memory_system()
set_gc_enabled(based)

fr fr Set low threshold to trigger GC
global_heap.gc_threshold = 1000

sus before_gc_collections normie = gc_stats.collections_run
fr fr Allocate enough to trigger GC
sus large_id normie = allocate_memory(2000, ALLOC_HEAP)
sus after_gc_collections normie = gc_stats.collections_run

assert_true(after_gc_collections > before_gc_collections)
print_test_status("GC threshold triggering", after_gc_collections > before_gc_collections)

fr fr Test 9: Manual garbage collection
reset_memory_system()
init_memory_system()

fr fr Create objects with zero ref count
sus gc_id1 normie = allocate_memory(512, ALLOC_HEAP)
sus gc_id2 normie = allocate_memory(512, ALLOC_HEAP)
dec_ref_count(gc_id1) fr fr Make unreachable
dec_ref_count(gc_id2) fr fr Make unreachable

sus before_manual_gc normie = gc_stats.live_objects
sus gc_result lit = force_gc()
assert_true(gc_result)
sus after_manual_gc normie = gc_stats.live_objects

fr fr Objects should be collected during GC
assert_true(after_manual_gc <= before_manual_gc)
print_test_status("Manual garbage collection", based)

fr fr Test 10: Memory pressure detection
reset_memory_system()
init_memory_system()

fr fr Test normal pressure
sus pressure_normal lit = check_memory_pressure()
assert_true(!pressure_normal)

fr fr Simulate high memory usage by adjusting heap size
gc_stats.heap_size = MAX_HEAP_SIZE * 85 / 100 fr fr 85% usage
sus pressure_high lit = check_memory_pressure()
assert_true(pressure_high)
print_test_status("Memory pressure detection", based)

fr fr Test 11: Memory statistics
reset_memory_system()
init_memory_system()

sus stats_id normie = allocate_memory(1024, ALLOC_HEAP)
sus detailed_stats map[tea]normie = get_memory_stats()

assert_true(detailed_stats["total_allocated"] >= 1024)
assert_true(detailed_stats["live_objects"] >= 1)
assert_true(detailed_stats["heap_size"] >= 1024)
assert_true(detailed_stats["heap_utilization"] >= 0)
print_test_status("Memory statistics", based)

fr fr Test 12: Health check
reset_memory_system()
init_memory_system()

sus health_result lit = memory_health_check()
assert_true(health_result)

fr fr Test unhealthy condition
gc_stats.heap_size = MAX_HEAP_SIZE * 95 / 100 fr fr 95% usage
sus unhealthy_result lit = memory_health_check()
assert_true(!unhealthy_result)
print_test_status("Memory health check", based)

fr fr Test 13: Invalid operations
sus invalid_alloc normie = allocate_memory(-1, ALLOC_HEAP)
assert_eq_int(invalid_alloc, -1)

sus invalid_obj MemoryObject = get_memory_object(99999)
assert_eq_int(invalid_obj.id, 0) fr fr Should be empty object

sus invalid_ref normie = get_ref_count(99999)
assert_eq_int(invalid_ref, -1)
print_test_status("Invalid operations handling", based)

fr fr Test 14: GC enable/disable
set_gc_enabled(cap)
sus gc_disabled_alloc normie = allocate_memory(100, ALLOC_HEAP)
assert_true(gc_disabled_alloc > 0)

set_gc_enabled(based)
sus gc_enabled_alloc normie = allocate_memory(100, ALLOC_HEAP)
assert_true(gc_enabled_alloc > 0)
print_test_status("GC enable/disable", based)

fr fr Test 15: Memory allocation types
reset_memory_system()
init_memory_system()

sus stack_obj normie = allocate_memory(64, ALLOC_STACK)
sus heap_obj normie = allocate_memory(64, ALLOC_HEAP)
sus global_obj normie = allocate_memory(64, ALLOC_GLOBAL)

sus stack_mem MemoryObject = get_memory_object(stack_obj)
sus heap_mem MemoryObject = get_memory_object(heap_obj)
sus global_mem MemoryObject = get_memory_object(global_obj)

assert_eq_int(stack_mem.alloc_type, ALLOC_STACK)
assert_eq_int(heap_mem.alloc_type, ALLOC_HEAP)
assert_eq_int(global_mem.alloc_type, ALLOC_GLOBAL)
print_test_status("Memory allocation types", based)

print_test_summary()
