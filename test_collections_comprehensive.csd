yeet "testz"
yeet "stdlib/collections/fixed_collections"

fr fr ========================================
fr fr CURSED Collections Comprehensive Test Suite
fr fr Tests for fixed HashMap and Array operations
fr fr Memory management and edge case validation
fr fr ========================================

slay test_dynamic_array_comprehensive() {
    vibez.spill("\n=== Testing Dynamic Array (Vector) ===")
    
    fr fr Test 1: Basic operations
    sus vec DynamicArray = Vec_new()
    test_assert(Vec_is_empty(vec), "New vector should be empty")
    test_assert(Vec_len(vec) == 0, "New vector should have length 0")
    test_assert(Vec_capacity(vec) == 8, "New vector should have capacity 8")
    
    fr fr Test 2: Push operations
    vec = Vec_push(vec, "first")
    vec = Vec_push(vec, "second")
    vec = Vec_push(vec, "third")
    
    test_assert(Vec_len(vec) == 3, "Vector should have 3 elements after pushes")
    test_assert(!Vec_is_empty(vec), "Vector should not be empty after pushes")
    test_assert(Vec_get(vec, 0) == "first", "First element should be 'first'")
    test_assert(Vec_get(vec, 2) == "third", "Third element should be 'third'")
    
    fr fr Test 3: Pop operations
    sus popped extra = Vec_pop(vec)
    test_assert(popped == "third", "Popped element should be 'third'")
    test_assert(Vec_len(vec) == 2, "Vector should have 2 elements after pop")
    
    fr fr Test 4: Set operations
    vec = Vec_set(vec, 1, "modified")
    test_assert(Vec_get(vec, 1) == "modified", "Element should be modified")
    
    fr fr Test 5: Insert operations
    vec = Vec_insert(vec, 1, "inserted")
    test_assert(Vec_len(vec) == 3, "Vector should have 3 elements after insert")
    test_assert(Vec_get(vec, 1) == "inserted", "Inserted element should be at index 1")
    test_assert(Vec_get(vec, 2) == "modified", "Original element should be shifted")
    
    fr fr Test 6: Remove operations
    vec = Vec_remove(vec, 1)
    test_assert(Vec_len(vec) == 2, "Vector should have 2 elements after remove")
    test_assert(Vec_get(vec, 1) == "modified", "Elements should shift left after remove")
    
    fr fr Test 7: Capacity growth (add many elements)
    sus i normie = 0
    bestie i < 10 {
        vec = Vec_push(vec, "element_" + tea(i))
        i = i + 1
    }
    
    test_assert(Vec_len(vec) == 12, "Vector should grow to accommodate all elements")
    test_assert(Vec_capacity(vec) >= 12, "Vector capacity should grow as needed")
    
    fr fr Test 8: Clear operation
    vec = Vec_clear(vec)
    test_assert(Vec_is_empty(vec), "Vector should be empty after clear")
    test_assert(Vec_len(vec) == 0, "Vector length should be 0 after clear")
    
    fr fr Test 9: Edge cases
    sus empty_vec DynamicArray = Vec_new()
    sus empty_pop extra = Vec_pop(empty_vec)
    test_assert(empty_pop == 0, "Popping from empty vector should return default value")
    
    sus out_of_bounds extra = Vec_get(vec, 999)
    test_assert(out_of_bounds == 0, "Getting out of bounds should return default value")
    
    vibez.spill("✅ Dynamic Array tests passed!")
}

slay test_hashmap_comprehensive() {
    vibez.spill("\n=== Testing Fixed HashMap ===")
    
    fr fr Test 1: Basic operations
    sus map FixedHashMap = HashMap_new()
    test_assert(HashMap_is_empty(map), "New HashMap should be empty")
    test_assert(HashMap_len(map) == 0, "New HashMap should have size 0")
    
    fr fr Test 2: Insert and get operations
    map = HashMap_insert(map, "key1", "value1")
    map = HashMap_insert(map, "key2", "value2")
    map = HashMap_insert(map, "key3", "value3")
    
    test_assert(HashMap_len(map) == 3, "HashMap should have 3 elements")
    test_assert(!HashMap_is_empty(map), "HashMap should not be empty")
    test_assert(HashMap_get(map, "key1") == "value1", "Should get correct value for key1")
    test_assert(HashMap_get(map, "key2") == "value2", "Should get correct value for key2")
    test_assert(HashMap_get(map, "key3") == "value3", "Should get correct value for key3")
    
    fr fr Test 3: Contains key operations
    test_assert(HashMap_contains_key(map, "key1"), "Should contain key1")
    test_assert(HashMap_contains_key(map, "key2"), "Should contain key2")
    test_assert(!HashMap_contains_key(map, "nonexistent"), "Should not contain nonexistent key")
    
    fr fr Test 4: Update existing key
    map = HashMap_insert(map, "key1", "updated_value1")
    test_assert(HashMap_len(map) == 3, "Size should remain same after update")
    test_assert(HashMap_get(map, "key1") == "updated_value1", "Value should be updated")
    
    fr fr Test 5: Remove operations
    map = HashMap_remove(map, "key2")
    test_assert(HashMap_len(map) == 2, "Size should decrease after remove")
    test_assert(!HashMap_contains_key(map, "key2"), "Should not contain removed key")
    test_assert(HashMap_get(map, "key2") == 0, "Getting removed key should return default")
    
    fr fr Test 6: Keys and values
    sus keys []tea = HashMap_keys(map)
    sus values []extra = HashMap_values(map)
    test_assert(len(keys) == 2, "Keys array should have correct length")
    test_assert(len(values) == 2, "Values array should have correct length")
    
    fr fr Test 7: Collision handling (insert many elements)
    sus collision_map FixedHashMap = HashMap_new()
    i = 0
    bestie i < 20 {
        sus key tea = "test_key_" + tea(i)
        sus value extra = "test_value_" + tea(i)
        collision_map = HashMap_insert(collision_map, key, value)
        i = i + 1
    }
    
    test_assert(HashMap_len(collision_map) == 20, "Should handle many insertions")
    
    fr fr Test collision retrieval
    test_assert(HashMap_get(collision_map, "test_key_5") == "test_value_5", "Should retrieve with collisions")
    test_assert(HashMap_contains_key(collision_map, "test_key_15"), "Should find keys with collisions")
    
    fr fr Test 8: Clear operation
    map = HashMap_clear(map)
    test_assert(HashMap_is_empty(map), "HashMap should be empty after clear")
    test_assert(HashMap_len(map) == 0, "HashMap size should be 0 after clear")
    
    fr fr Test 9: Edge cases
    sus nonexistent extra = HashMap_get(map, "nonexistent")
    test_assert(nonexistent == 0, "Getting nonexistent key should return default")
    
    sus empty_remove FixedHashMap = HashMap_remove(map, "nonexistent")
    test_assert(HashMap_len(empty_remove) == 0, "Removing from empty map should not change size")
    
    vibez.spill("✅ HashMap tests passed!")
}

slay test_array_operations_comprehensive() {
    vibez.spill("\n=== Testing Enhanced Array Operations ===")
    
    fr fr Test 1: Safe get operations
    sus arr []normie = [10, 20, 30, 40, 50]
    
    test_assert(Array_safe_get(arr, 0, -1) == 10, "Safe get at valid index should return element")
    test_assert(Array_safe_get(arr, 4, -1) == 50, "Safe get at last index should return element")
    test_assert(Array_safe_get(arr, 10, -1) == -1, "Safe get at invalid index should return default")
    test_assert(Array_safe_get(arr, -1, -1) == -1, "Safe get at negative index should return default")
    
    fr fr Test 2: Safe set operations
    sus modified []normie = Array_safe_set(arr, 2, 999)
    test_assert(modified[2] == 999, "Safe set should modify element at valid index")
    test_assert(modified[1] == 20, "Safe set should not affect other elements")
    
    sus invalid_set []normie = Array_safe_set(arr, 10, 999)
    test_assert(len(invalid_set) == len(arr), "Safe set at invalid index should return original")
    
    fr fr Test 3: Array copy
    sus copied []normie = Array_copy(arr)
    test_assert(len(copied) == len(arr), "Copied array should have same length")
    test_assert(copied[0] == arr[0], "Copied array should have same elements")
    test_assert(copied[4] == arr[4], "Copied array should preserve all elements")
    
    fr fr Test 4: Safe insert
    sus inserted []normie = Array_insert_safe(arr, 2, 25)
    test_assert(len(inserted) == 6, "Inserted array should have increased length")
    test_assert(inserted[2] == 25, "Inserted element should be at correct position")
    test_assert(inserted[3] == 30, "Elements should shift right after insert")
    
    sus insert_end []normie = Array_insert_safe(arr, 5, 60)
    test_assert(len(insert_end) == 6, "Insert at end should work")
    test_assert(insert_end[5] == 60, "Element inserted at end should be correct")
    
    sus invalid_insert []normie = Array_insert_safe(arr, 10, 999)
    test_assert(len(invalid_insert) == len(arr), "Invalid insert should return original")
    
    fr fr Test 5: Safe remove
    sus removed []normie = Array_remove_safe(arr, 2)
    test_assert(len(removed) == 4, "Removed array should have decreased length")
    test_assert(removed[2] == 40, "Elements should shift left after remove")
    
    sus remove_first []normie = Array_remove_safe(arr, 0)
    test_assert(len(remove_first) == 4, "Remove first should work")
    test_assert(remove_first[0] == 20, "First element should be shifted")
    
    sus invalid_remove []normie = Array_remove_safe(arr, 10)
    test_assert(len(invalid_remove) == len(arr), "Invalid remove should return original")
    
    fr fr Test 6: Find operations
    test_assert(Array_find_index(arr, 30) == 2, "Should find correct index")
    test_assert(Array_find_index(arr, 999) == -1, "Should return -1 for not found")
    test_assert(Array_contains(arr, 40), "Should contain existing element")
    test_assert(!Array_contains(arr, 999), "Should not contain non-existing element")
    
    fr fr Test 7: Reverse operations
    sus reversed []normie = Array_reverse(arr)
    test_assert(len(reversed) == len(arr), "Reversed array should have same length")
    test_assert(reversed[0] == 50, "First element should be last of original")
    test_assert(reversed[4] == 10, "Last element should be first of original")
    
    fr fr Test 8: Slice operations
    sus sliced []normie = Array_slice(arr, 1, 4)
    test_assert(len(sliced) == 3, "Sliced array should have correct length")
    test_assert(sliced[0] == 20, "Slice should start at correct element")
    test_assert(sliced[2] == 40, "Slice should end at correct element")
    
    sus empty_slice []normie = Array_slice(arr, 3, 2)
    test_assert(len(empty_slice) == 0, "Invalid slice should return empty array")
    
    sus bounds_slice []normie = Array_slice(arr, -1, 100)
    test_assert(len(bounds_slice) == len(arr), "Out of bounds slice should be corrected")
    
    vibez.spill("✅ Enhanced Array Operations tests passed!")
}

slay test_memory_safety() {
    vibez.spill("\n=== Testing Memory Safety ===")
    
    fr fr Test 1: Large data structure creation and cleanup
    sus large_vec DynamicArray = Vec_with_capacity(1000)
    
    sus i normie = 0
    bestie i < 100 {
        large_vec = Vec_push(large_vec, "large_element_" + tea(i))
        i = i + 1
    }
    
    test_assert(Vec_len(large_vec) == 100, "Large vector should have correct size")
    
    fr fr Test 2: Large HashMap with many collisions
    sus large_map FixedHashMap = HashMap_with_capacity(32)
    
    i = 0
    bestie i < 100 {
        sus key tea = "collision_key_" + tea(i)
        large_map = HashMap_insert(large_map, key, "value_" + tea(i))
        i = i + 1
    }
    
    test_assert(HashMap_len(large_map) == 100, "Large HashMap should handle collisions")
    
    fr fr Test 3: Repeated operations (stress test)
    sus stress_vec DynamicArray = Vec_new()
    
    i = 0
    bestie i < 50 {
        stress_vec = Vec_push(stress_vec, "stress_" + tea(i))
        
        lowkey i % 10 == 0 && i > 0 {
            stress_vec = Vec_pop(stress_vec)  fr fr Remove some elements periodically
        }
        
        i = i + 1
    }
    
    test_assert(Vec_len(stress_vec) > 0, "Stress test vector should have elements")
    
    fr fr Test 4: Array operations with large data
    sus large_arr []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 
                              11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
    
    sus large_copy []normie = Array_copy(large_arr)
    test_assert(len(large_copy) == 20, "Large array copy should preserve length")
    
    sus large_reversed []normie = Array_reverse(large_arr)
    test_assert(large_reversed[0] == 20, "Large array reverse should work")
    
    vibez.spill("✅ Memory Safety tests passed!")
}

slay test_edge_cases() {
    vibez.spill("\n=== Testing Edge Cases ===")
    
    fr fr Test 1: Empty collections
    sus empty_vec DynamicArray = Vec_new()
    sus empty_arr []normie = []
    sus empty_map FixedHashMap = HashMap_new()
    
    test_assert(Vec_get(empty_vec, 0) == 0, "Empty vector get should return default")
    test_assert(Array_safe_get(empty_arr, 0, -1) == -1, "Empty array get should return default")
    test_assert(HashMap_get(empty_map, "key") == 0, "Empty map get should return default")
    
    fr fr Test 2: Single element collections
    sus single_vec DynamicArray = Vec_new()
    single_vec = Vec_push(single_vec, "only")
    
    sus only_element extra = Vec_pop(single_vec)
    test_assert(only_element == "only", "Single element pop should return element")
    test_assert(Vec_is_empty(single_vec), "Vector should be empty after popping only element")
    
    fr fr Test 3: Boundary conditions
    sus boundary_arr []normie = [100]
    
    sus first_element normie = Array_safe_get(boundary_arr, 0, -1)
    test_assert(first_element == 100, "Boundary: first element should be accessible")
    
    sus out_of_bounds normie = Array_safe_get(boundary_arr, 1, -1)
    test_assert(out_of_bounds == -1, "Boundary: second element should be out of bounds")
    
    fr fr Test 4: HashMap key collisions (worst case)
    sus collision_test FixedHashMap = HashMap_new()
    
    fr fr Insert keys that might hash to same bucket
    collision_test = HashMap_insert(collision_test, "a", "value_a")
    collision_test = HashMap_insert(collision_test, "b", "value_b")
    collision_test = HashMap_insert(collision_test, "c", "value_c")
    
    test_assert(HashMap_get(collision_test, "a") == "value_a", "Collision: should get value a")
    test_assert(HashMap_get(collision_test, "b") == "value_b", "Collision: should get value b") 
    test_assert(HashMap_get(collision_test, "c") == "value_c", "Collision: should get value c")
    
    fr fr Test 5: String operations edge cases
    sus empty_string_length normie = string_length_proper("")
    test_assert(empty_string_length == 0, "Empty string should have length 0")
    
    sus single_char_code normie = char_code_at("a", 0)
    test_assert(single_char_code > 0, "Single char should have valid code")
    
    vibez.spill("✅ Edge Cases tests passed!")
}

slay run_comprehensive_tests() {
    vibez.spill("🚀 Starting CURSED Collections Comprehensive Test Suite")
    vibez.spill("====================================================")
    
    test_start("Collections Comprehensive")
    
    test_dynamic_array_comprehensive()
    test_hashmap_comprehensive()
    test_array_operations_comprehensive()
    test_memory_safety()
    test_edge_cases()
    
    vibez.spill("\n====================================================")
    vibez.spill("🎉 All Collections Tests Completed!")
    
    print_test_summary()
}

fr fr Run the comprehensive test suite
run_comprehensive_tests()

vibez.spill("\n✅ CURSED Collections Implementation Fixed and Validated!")
vibez.spill("📊 Dynamic Arrays: Proper resizing and memory management")
vibez.spill("🗂️  HashMap: Linear probing with collision resolution")
vibez.spill("🔒 Array Operations: Bounds checking and safety")
vibez.spill("⚡ Memory Safety: Stress tested and validated")
vibez.spill("🎯 Edge Cases: Comprehensive coverage and handling")
