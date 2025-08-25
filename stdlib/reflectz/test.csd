# Test for reflectz module

yeet "testz"
yeet "vibez"
yeet "reflectz"

test_start("reflectz_comprehensive")

# Test TypeOf and ValueOf
sus value lit = based
sus type_info TypeInfo = TypeOf(value)
assert_eq_string(type_info.name, "unknown")  # Default implementation
assert_eq_int(type_info.size, 8)

sus val_obj Value = ValueOf(value)
assert_eq_bool(val_obj.data, based)

# Test interface implementation checking
assert_eq_bool(Implements("drip", "drip"), based)
assert_eq_bool(Implements("drip", "tea"), nocap)

# Test call stack functionality
sus stack []CallFrame = get_call_stack()
assert_gt_int(len(stack), 0)

sus caller CallFrame = get_caller_info()
assert_eq_string(caller.function_name, "unknown")  # Default implementation

# Test runtime statistics
sus alloc_stats AllocationStats = get_allocation_stats()
assert_gt_int(alloc_stats.total_allocations, 0)
assert_gt_int(alloc_stats.current_memory, 0)

sus gc_stats GCStats = get_gc_stats()
assert_ge_int(gc_stats.collection_count, 0)

sus coverage tea = get_coverage_info()
assert_ne_string(coverage, "")

# Test array reflection operations
sus test_array []lit = [based, nocap, based]
assert_eq_int(array_length(test_array), 3)
assert_eq_bool(array_get(test_array, 0), based)
assert_eq_bool(array_get(test_array, 1), nocap)

sus extended_array []lit = array_append(test_array, based)
assert_eq_int(array_length(extended_array), 4)

# Test type information
sus drip_info TypeInfo = get_type_info("drip")
assert_eq_string(drip_info.name, "drip")
assert_eq_int(drip_info.size, 8)
assert_eq_string(drip_info.kind, "drip")

sus tea_info TypeInfo = get_type_info("tea")
assert_eq_string(tea_info.name, "tea")
assert_eq_int(tea_info.size, 24)
assert_eq_string(tea_info.kind, "tea")

sus lit_info TypeInfo = get_type_info("lit")
assert_eq_string(lit_info.name, "lit")
assert_eq_int(lit_info.size, 1)
assert_eq_string(lit_info.kind, "lit")

# Test field operations
sus empty_field FieldInfo = get_field_info(drip_info, "nonexistent")
assert_eq_string(empty_field.name, "nonexistent")
assert_eq_string(empty_field.type_name, "unknown")

sus field_value lit = get_field_value(val_obj, "test_field")
assert_eq_bool(field_value, based)  # Default return

assert_eq_string(type_name(drip_info), "drip")

# Test execution context
sus context tea = get_current_execution_context()
assert_eq_string(context, "main_execution_context")

sus memory_info tea = get_memory_info()
assert_ne_string(memory_info, "")

# Test utility functions
sus value_str tea = value_to_string(based)
assert_eq_string(value_str, "value_representation")

assert_eq_bool(is_type(based, "unknown"), based)  # Default behavior

# Test profiler
start_profiler()
sus profiler_results tea = stop_profiler()
assert_ne_string(profiler_results, "")

# Test dynamic method invocation
sus method_result lit = invoke_method(based, "test_method", [])
assert_eq_bool(method_result, based)

test_complete()
vibez.spill("reflectz tests completed successfully!")

# Test debug output functions
vibez.spill("\n=== Debug Output Tests ===")
print_call_stack()
print_memory_stats()
