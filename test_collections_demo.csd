// CURSED Collections Demo Program
// Testing the newly implemented collections functionality

vibe main

slay main() {
    vibez.spill("Starting CURSED Collections Demo...")
    
    // Test array operations
    sus arr = collections_array_new()
    facts initial_size = collections_array_len(arr)
    vibez.spill("Created empty array with size: " + initial_size)
    
    // Add some elements
    collections_array_push(arr, 10)
    collections_array_push(arr, 20)
    collections_array_push(arr, 30)
    
    facts new_size = collections_array_len(arr)
    vibez.spill("Array after pushes, size: " + new_size)
    
    // Test getting elements
    facts first_elem = collections_array_get(arr, 0)
    facts second_elem = collections_array_get(arr, 1)
    vibez.spill("First element: " + first_elem + ", Second: " + second_elem)
    
    // Test map operations
    sus map = collections_map_new()
    collections_map_set(map, 1, 100)
    collections_map_set(map, 2, 200)
    
    facts value1 = collections_map_get(map, 1)
    facts value2 = collections_map_get(map, 2)
    vibez.spill("Map values - key 1: " + value1 + ", key 2: " + value2)
    
    // Test set operations
    sus set = collections_set_new()
    collections_set_insert(set, 42)
    collections_set_insert(set, 73)
    
    facts has_42 = collections_set_contains(set, 42)
    facts has_99 = collections_set_contains(set, 99)
    vibez.spill("Set contains 42: " + has_42 + ", contains 99: " + has_99)
    
    vibez.spill("Collections demo complete!")
    yolo 0
}
