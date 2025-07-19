yeet "testz"
yeet "runtime_core"

# Test Runtime Core Value System
test_start("Runtime Core Value System Tests")

# Test 1: Runtime initialization
sus init_result lit = init_runtime_values()
assert_true(init_result)
print_test_status("Runtime initialization", init_result)

# Test 2: Type registration
register_type(100, "custom_type")
sus type_id normie = get_type_id("custom_type")
assert_eq_int(type_id, 100)
print_test_status("Type registration", type_id == 100)

# Test 3: Value creation and basic operations
sus test_value CursedValue = create_value("normie", "42")
assert_true(validate_value(test_value))
assert_true(value_is_type(test_value, "normie"))
print_test_status("Value creation", based)

# Test 4: Value boxing and unboxing
sus large_data tea = "This is a very large string that should trigger boxing due to its size being over 64 bytes for sure"
sus large_value CursedValue = create_value("tea", large_data)
assert_true(large_value.is_boxed)

sus boxed_value CursedValue = box_value(test_value)
sus unboxed_value CursedValue = unbox_value(boxed_value)
assert_true(!unboxed_value.is_boxed)
print_test_status("Boxing/unboxing", based)

# Test 5: Value comparison
sus value1 CursedValue = create_value("normie", "42")
sus value2 CursedValue = create_value("normie", "42")
sus value3 CursedValue = create_value("normie", "43")
assert_true(values_equal(value1, value2))
assert_true(!values_equal(value1, value3))
print_test_status("Value equality", based)

# Test 6: Value copying
sus original CursedValue = create_value("tea", "hello")
sus copy CursedValue = copy_value(original)
assert_true(values_equal(original, copy))
assert_true(original.value_type == copy.value_type)
print_test_status("Value copying", based)

# Test 7: String conversion
sus str_value CursedValue = create_value("tea", "test")
sus str_repr tea = value_to_string(str_value)
assert_true(stringz.contains(str_repr, "tea"))
assert_true(stringz.contains(str_repr, "test"))
print_test_status("String conversion", based)

# Test 8: Memory size calculation
sus small_value CursedValue = create_value("normie", "1")
sus large_value_mem CursedValue = create_value("tea", large_data)
sus small_size normie = value_memory_size(small_value)
sus large_size normie = value_memory_size(large_value_mem)
assert_true(large_size > small_size)
print_test_status("Memory size calculation", based)

# Test 9: GC detection
sus gc_candidate CursedValue = create_value("tea", large_data)
assert_true(value_needs_gc(gc_candidate))
assert_true(!value_needs_gc(small_value))
print_test_status("GC detection", based)

# Test 10: Runtime statistics
sus stats map[tea]normie = get_value_stats()
assert_true(stats["total_types"] >= 10)  # Built-in types
print_test_status("Runtime statistics", stats["total_types"] >= 10)

# Test 11: Health check
sus health lit = runtime_values_health_check()
assert_true(health)
print_test_status("Health check", health)

# Test 12: Cache management
clear_value_cache()
sus stats_after map[tea]normie = get_value_stats()
assert_eq_int(stats_after["cached_values"], 0)
print_test_status("Cache management", stats_after["cached_values"] == 0)

# Test 13: Type ID edge cases
sus unknown_id normie = get_type_id("nonexistent_type")
assert_eq_int(unknown_id, 0)
print_test_status("Unknown type handling", unknown_id == 0)

# Test 14: Value validation edge cases
sus invalid_value CursedValue
invalid_value.type_id = 0
invalid_value.value_type = ""
invalid_value.size = -1
assert_true(!validate_value(invalid_value))
print_test_status("Invalid value detection", based)

# Test 15: Complex type system integration
register_type(200, "interface_type")
register_type(300, "generic_type")
sus complex_value CursedValue = create_value("interface_type", "complex_data")
assert_true(value_is_type(complex_value, "interface_type"))
assert_true(!value_is_type(complex_value, "generic_type"))
print_test_status("Complex type system", based)

print_test_summary()
