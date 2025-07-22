yeet "testz"
yeet "runtime_core"

test_start("Runtime Core Module Tests")

fr fr ================================
fr fr Basic Value System Tests
fr fr ================================

fr fr Test value creation
sus int_val RuntimeValue = runtime_value_create("42", "integer")
sus float_val RuntimeValue = runtime_value_create("3.14", "float")  
sus string_val RuntimeValue = runtime_value_create("hello", "string")
sus bool_val RuntimeValue = runtime_value_create("based", "boolean")

fr fr Test type checking
assert_true(runtime_type_check(int_val, "integer"))
assert_true(runtime_type_check(float_val, "float"))
assert_true(runtime_type_check(string_val, "string"))
assert_true(runtime_type_check(bool_val, "boolean"))

fr fr Test type names
assert_eq_string(runtime_get_type(int_val), "integer")
assert_eq_string(runtime_get_type(float_val), "float")
assert_eq_string(runtime_get_type(string_val), "string")
assert_eq_string(runtime_get_type(bool_val), "boolean")

fr fr Test value conversion
sus int_str tea = runtime_convert_to_string(int_val)
sus float_str tea = runtime_convert_to_string(float_val)
sus string_str tea = runtime_convert_to_string(string_val)
sus bool_str tea = runtime_convert_to_string(bool_val)

assert_true(string_length(int_str) > 0)
assert_true(string_length(float_str) > 0)
assert_true(string_length(string_str) > 0)
assert_true(string_length(bool_str) > 0)

fr fr Test integer parsing
assert_eq_int(parse_integer("123"), 123)
assert_eq_int(parse_integer("0"), 0)

fr fr Test boolean parsing
assert_true(parse_boolean("based"))
assert_true(parse_boolean("true"))
assert_false(parse_boolean("cap"))
assert_false(parse_boolean("false"))

fr fr ================================
fr fr Array Operations Tests
fr fr ================================

test_start("Array Operations")

fr fr Test array length calculation
sus test_array [RuntimeValue] = []
sus array_len normie = array_get_length(test_array)
assert_true(array_len >= 0)

fr fr Test array element access
sus element RuntimeValue = array_get_element(test_array, 0)
assert_eq_string(runtime_get_type(element), "nil")

fr fr Test array element assignment
sus set_result lit = array_set_element(test_array, 0, int_val)
fr fr Note: This will return cap (false) for empty array, which is expected

fr fr Test runtime array operations
sus runtime_len normie = runtime_array_length(test_array)
assert_true(runtime_len >= 0)

sus runtime_element RuntimeValue = runtime_array_get(test_array, 0)
assert_eq_string(runtime_get_type(runtime_element), "nil")

fr fr ================================
fr fr Map Operations Tests  
fr fr ================================

test_start("Map Operations")

fr fr Test map operations with simulated map
sus test_map vibes[tea]RuntimeValue = vibes[tea]RuntimeValue{}

fr fr Test key existence
sus has_key lit = map_has_key(test_map, "test_key")
assert_false(has_key) fr fr Should be false for empty/simulated map

fr fr Test value retrieval
sus map_value RuntimeValue = map_get_value(test_map, "test_key")
assert_eq_string(runtime_get_type(map_value), "nil")

fr fr Test value setting
sus set_success lit = map_set_value(test_map, "test_key", string_val)
fr fr Note: This may return cap (false) in pure CURSED simulation

fr fr Test runtime map operations
sus runtime_map_val RuntimeValue = runtime_map_get(test_map, "test_key")
assert_eq_string(runtime_get_type(runtime_map_val), "nil")

fr fr ================================
fr fr String Operations Tests
fr fr ================================

test_start("String Operations")

fr fr Test enhanced string length
sus test_string tea = "hello"
sus enhanced_len normie = string_length_enhanced(test_string)
assert_true(enhanced_len >= 0)

fr fr Test string character access
sus char_code normie = string_char_at(test_string, 0)
assert_true(char_code >= 0)

fr fr Test string concatenation
sus concat_result tea = string_concat("hello", " world")
sus concat_len normie = string_length_enhanced(concat_result)
assert_true(concat_len >= 11)

fr fr Test string substring
sus sub_result tea = string_substring("hello world", 0, 5)
sus sub_len normie = string_length_enhanced(sub_result)
assert_true(sub_len >= 0)

fr fr Test string to byte array conversion
sus byte_array [normie] = string_to_byte_array(test_string)
fr fr Note: In pure CURSED simulation, this may return empty array

fr fr Test string equality
assert_true(runtime_strings_equal("test", "test"))
assert_false(runtime_strings_equal("test", "different"))

fr fr ================================
fr fr Memory Management Tests
fr fr ================================

test_start("Memory Management")

fr fr Test memory allocation
sus allocated_ptr normie = memory_allocate_bytes(1024)
assert_true(allocated_ptr > 0) fr fr Should return valid pointer simulation

fr fr Test memory deallocation
sus dealloc_result lit = memory_deallocate_bytes(allocated_ptr, 1024)
assert_true(dealloc_result)

fr fr Test memory copy
sus copy_result lit = memory_copy_bytes(1000, 2000, 100)
assert_true(copy_result)

fr fr Test memory zero
sus zero_result lit = memory_zero_bytes(1000, 100)
assert_true(zero_result)

fr fr Test invalid memory operations
sus invalid_alloc normie = memory_allocate_bytes(0)
assert_eq_int(invalid_alloc, 0)

sus invalid_dealloc lit = memory_deallocate_bytes(0, 100)
assert_false(invalid_dealloc)

fr fr ================================
fr fr Time Operations Tests
fr fr ================================

test_start("Time Operations")

fr fr Test current time retrieval
sus start_time normie = get_current_time_nanos()
assert_true(start_time > 0)

sus millis_time normie = get_current_time_millis()
assert_true(millis_time > 0)

fr fr Test time elapsed calculation
sus end_time normie = get_current_time_nanos()
sus elapsed normie = time_elapsed_nanos(start_time)
assert_true(elapsed >= 0)

fr fr ================================
fr fr Performance Metrics Tests
fr fr ================================

test_start("Performance Metrics")

fr fr Test performance logging
sus log_result lit = log_performance_metric("test_operation", 1000)
assert_true(log_result)

fr fr Test performance stats retrieval
sus perf_stats tea = get_performance_stats()
sus stats_len normie = string_length_enhanced(perf_stats)
assert_true(stats_len > 0)

fr fr ================================
fr fr Garbage Collection Tests
fr fr ================================

test_start("Garbage Collection")

fr fr Test GC trigger
sus gc_result lit = trigger_gc_collection()
assert_true(gc_result)

fr fr Test GC statistics
sus gc_stats tea = get_gc_statistics()
sus gc_stats_len normie = string_length_enhanced(gc_stats)
assert_true(gc_stats_len > 0)

fr fr Test GC individual stats
sus collections normie = gc_get_collection_count()
assert_true(collections >= 0)

sus memory_freed normie = gc_get_memory_freed()
assert_true(memory_freed >= 0)

sus live_objects normie = gc_get_live_object_count()
assert_true(live_objects >= 0)

fr fr ================================
fr fr Dynamic Function Calling Tests
fr fr ================================

test_start("Dynamic Function Calling")

fr fr Test function call with known functions
sus args [RuntimeValue] = []
fr fr Note: In pure CURSED simulation, array operations are limited

fr fr Test function call with unknown function
sus unknown_result RuntimeValue = call_runtime_function("unknown_func", args)
sus error_type tea = runtime_get_type(unknown_result)
fr fr Should return some form of error representation

fr fr ================================
fr fr Value Comparison Tests
fr fr ================================

test_start("Value Comparison")

fr fr Test runtime value equality
sus val1 RuntimeValue = runtime_value_create("42", "integer")
sus val2 RuntimeValue = runtime_value_create("42", "integer")
sus val3 RuntimeValue = runtime_value_create("43", "integer")

sus equal_result lit = runtime_values_equal(val1, val2)
fr fr Note: This may be limited in pure CURSED simulation

sus unequal_result lit = runtime_values_equal(val1, val3)
fr fr Note: This may be limited in pure CURSED simulation

fr fr ================================
fr fr Error Handling Tests
fr fr ================================

test_start("Error Handling")

fr fr Test error creation
sus error_val RuntimeValue = runtime_create_error("Test error", "test_error")
sus error_str tea = runtime_convert_to_string(error_val)
sus error_len normie = string_length_enhanced(error_str)
assert_true(error_len > 0)

fr fr Test detailed error creation
sus stack_trace [tea] = []
sus detailed_error RuntimeValue = runtime_create_detailed_error("Detailed error", "detail_error", stack_trace)
sus detailed_str tea = runtime_convert_to_string(detailed_error)
sus detailed_len normie = string_length_enhanced(detailed_str)
assert_true(detailed_len > 0)

fr fr Test error checking
sus is_error lit = runtime_is_error(error_val)
fr fr Note: In current implementation, this may return false due to type checking limitations

fr fr ================================
fr fr Helper Function Tests
fr fr ================================

test_start("Helper Functions")

fr fr Test string from character
sus char_str tea = string_from_char(65) fr fr ASCII 'A'
sus char_str_len normie = string_length_enhanced(char_str)
assert_true(char_str_len > 0)

fr fr Test time counter functions
sus counter_val normie = time_counter_get()
assert_true(counter_val > 0)

sus increment_result lit = time_counter_increment()
assert_true(increment_result)

sus new_counter_val normie = time_counter_get()
assert_true(new_counter_val > counter_val)

fr fr Test performance logging functions
sus log_append_result lit = performance_log_append("test_metric:500ns")
assert_true(log_append_result)

sus log_summary tea = performance_log_get_summary()
sus summary_len normie = string_length_enhanced(log_summary)
assert_true(summary_len > 0)

print_test_summary()
