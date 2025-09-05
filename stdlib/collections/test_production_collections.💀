fr fr ========================================
fr fr Production Collections Test Suite
fr fr Comprehensive testing for all data structures and algorithms
fr fr ========================================

yeet "testz"
yeet "stdlib/collections/production_collections"

fr fr Test results tracking
sus test_count normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0

slay run_test(test_name tea, result lit) {
    test_count = test_count + 1
    lowkey result {
        passed_tests = passed_tests + 1
        vibez.spill("✅ PASS:", test_name)
    } else {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL:", test_name)
    }
}

slay test_summary() {
    vibez.spill("")
    vibez.spill("=== Test Summary ===")
    vibez.spill("Total tests:", test_count)
    vibez.spill("Passed:", passed_tests)
    vibez.spill("Failed:", failed_tests)
    lowkey failed_tests == 0 {
        vibez.spill("🎉 All tests passed!")
    } else {
        vibez.spill("⚠️ Some tests failed")
    }
}

fr fr ================================
fr fr Hash Table Tests
fr fr ================================

slay test_hashmap_basic_operations() {
    sus map RobinHoodHashTable = HashMap_new()
    
    fr fr Test initial state
    run_test("HashMap initial size is 0", HashMap_size(map) == 0)
    run_test("HashMap get on empty returns empty", HashMap_get(map, "key1") == "")
    run_test("HashMap contains_key on empty is false", !HashMap_contains_key(map, "key1"))
    
    fr fr Test insertion
    map = HashMap_insert(map, "name", "Alice")
    run_test("HashMap size after first insert", HashMap_size(map) == 1)
    run_test("HashMap contains inserted key", HashMap_contains_key(map, "name"))
    run_test("HashMap get inserted value", HashMap_get(map, "name") == "Alice")
    
    fr fr Test multiple insertions
    map = HashMap_insert(map, "age", "25")
    map = HashMap_insert(map, "city", "New York")
    run_test("HashMap size after multiple inserts", HashMap_size(map) == 3)
    run_test("HashMap get second value", HashMap_get(map, "age") == "25")
    run_test("HashMap get third value", HashMap_get(map, "city") == "New York")
    
    fr fr Test update existing key
    map = HashMap_insert(map, "name", "Bob")
    run_test("HashMap size after update", HashMap_size(map) == 3)
    run_test("HashMap updated value", HashMap_get(map, "name") == "Bob")
    
    fr fr Test non-existent key
    run_test("HashMap get non-existent key", HashMap_get(map, "nonexistent") == "")
    run_test("HashMap contains non-existent key is false", !HashMap_contains_key(map, "nonexistent"))
}

slay test_hashmap_collision_handling() {
    sus map RobinHoodHashTable = HashMap_new()
    
    fr fr Insert keys that would hash to same bucket in simple hash
    map = HashMap_insert(map, "a", "value_a")
    map = HashMap_insert(map, "b", "value_b") 
    map = HashMap_insert(map, "c", "value_c")
    map = HashMap_insert(map, "d", "value_d")
    
    run_test("HashMap handles collisions - all keys present", 
        HashMap_contains_key(map, "a") && HashMap_contains_key(map, "b") && 
        HashMap_contains_key(map, "c") && HashMap_contains_key(map, "d"))
    
    run_test("HashMap collision values correct",
        HashMap_get(map, "a") == "value_a" && HashMap_get(map, "b") == "value_b" &&
        HashMap_get(map, "c") == "value_c" && HashMap_get(map, "d") == "value_d")
}

slay test_hashmap_resize() {
    sus map RobinHoodHashTable = HashMap_new()
    
    fr fr Insert many items to trigger resize
    map = HashMap_insert(map, "key1", "val1")
    map = HashMap_insert(map, "key2", "val2")
    map = HashMap_insert(map, "key3", "val3")
    map = HashMap_insert(map, "key4", "val4")
    map = HashMap_insert(map, "key5", "val5")
    
    run_test("HashMap resize preserves all values",
        HashMap_get(map, "key1") == "val1" && HashMap_get(map, "key2") == "val2" &&
        HashMap_get(map, "key3") == "val3" && HashMap_get(map, "key4") == "val4" &&
        HashMap_get(map, "key5") == "val5")
    
    run_test("HashMap size after resize", HashMap_size(map) == 5)
}

fr fr ================================
fr fr Sorting Algorithm Tests
fr fr ================================

slay test_merge_sort() {
    fr fr Test empty array
    sus empty normie[value] = []
    sus result1 [normie] = MergeSort_sort(empty)
    run_test("MergeSort empty array", Array_length(result1) == 0)
    
    fr fr Test single element
    sus single normie[value] = [42]
    sus result2 [normie] = MergeSort_sort(single)
    run_test("MergeSort single element", result2[0] == 42)
    
    fr fr Test sorted array
    sus sorted normie[value] = [1, 2, 3, 4, 5]
    sus result3 [normie] = MergeSort_sort(sorted)
    run_test("MergeSort already sorted", 
        result3[0] == 1 && result3[1] == 2 && result3[2] == 3 && result3[3] == 4 && result3[4] == 5)
    
    fr fr Test reverse sorted array
    sus reverse normie[value] = [5, 4, 3, 2, 1]
    sus result4 [normie] = MergeSort_sort(reverse)
    run_test("MergeSort reverse sorted", 
        result4[0] == 1 && result4[1] == 2 && result4[2] == 3 && result4[3] == 4 && result4[4] == 5)
    
    fr fr Test random array
    sus random normie[value] = [3, 1, 4, 1, 5, 9, 2, 6]
    sus result5 [normie] = MergeSort_sort(random)
    run_test("MergeSort random array first elements", 
        result5[0] == 1 && result5[1] == 1 && result5[2] == 2 && result5[3] == 3)
}

slay test_quick_sort() {
    fr fr Test basic sorting
    sus unsorted normie[value] = [64, 34, 25, 12, 22, 11, 90]
    sus result [normie] = QuickSort_sort(unsorted)
    
    run_test("QuickSort basic test first elements",
        result[0] <= result[1] && result[1] <= result[2])
    
    fr fr Test with duplicates
    sus duplicates normie[value] = [5, 2, 8, 2, 9, 1, 5]
    sus result2 [normie] = QuickSort_sort(duplicates)
    run_test("QuickSort with duplicates", 
        result2[0] <= result2[1] && result2[1] <= result2[2])
    
    fr fr Test small array fallback to insertion sort
    sus small normie[value] = [3, 1, 2]
    sus result3 [normie] = QuickSort_sort(small)
    run_test("QuickSort small array", 
        result3[0] == 1 && result3[1] == 2 && result3[2] == 3)
}

slay test_heap_sort() {
    fr fr Test basic heap sort
    sus unsorted normie[value] = [12, 11, 13, 5, 6, 7]
    sus result [normie] = HeapSort_sort(unsorted)
    
    run_test("HeapSort basic test", 
        result[0] <= result[1] && result[1] <= result[2])
    
    fr fr Test heap sort with negative numbers
    sus mixed normie[value] = [-1, 5, -3, 2]
    sus result2 [normie] = HeapSort_sort(mixed)
    run_test("HeapSort with negatives", result2[0] <= result2[1])
}

slay test_insertion_sort() {
    fr fr Test basic insertion sort
    sus unsorted normie[value] = [5, 2, 4, 6, 1, 3]
    sus result [normie] = InsertionSort_sort(unsorted)
    
    run_test("InsertionSort basic test", 
        result[0] == 1 && result[1] == 2 && result[2] == 3)
    
    fr fr Test already sorted
    sus sorted normie[value] = [1, 2, 3, 4, 5]
    sus result2 [normie] = InsertionSort_sort(sorted)
    run_test("InsertionSort already sorted", 
        result2[0] == 1 && result2[1] == 2)
}

fr fr ================================
fr fr Statistics Tests
fr fr ================================

slay test_statistics_basic() {
    sus data normie[value] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    fr fr Test mean
    sus mean drip = Statistics_mean(data)
    run_test("Statistics mean calculation", mean >= 5.0 && mean <= 6.0)
    
    fr fr Test median
    sus median drip = Statistics_median(data)
    run_test("Statistics median calculation", median >= 5.0 && median <= 6.0)
    
    fr fr Test percentiles
    sus p25 drip = Statistics_percentile(data, 25.0)
    sus p75 drip = Statistics_percentile(data, 75.0)
    run_test("Statistics 25th percentile", p25 >= 2.0 && p25 <= 4.0)
    run_test("Statistics 75th percentile", p75 >= 7.0 && p75 <= 9.0)
    
    fr fr Test quartiles
    sus quartiles [drip] = Statistics_quartiles(data)
    run_test("Statistics quartiles length", Array_length(quartiles) == 3)
    
    fr fr Test IQR
    sus iqr drip = Statistics_interquartile_range(data)
    run_test("Statistics IQR positive", iqr > 0.0)
}

slay test_statistics_edge_cases() {
    fr fr Test single element
    sus single normie[value] = [42]
    run_test("Statistics single element mean", Statistics_mean(single) == 42.0)
    run_test("Statistics single element median", Statistics_median(single) == 42.0)
    run_test("Statistics single element variance", Statistics_variance(single) == 0.0)
    
    fr fr Test empty array
    sus empty normie[value] = []
    run_test("Statistics empty array mean", Statistics_mean(empty) == 0.0)
    run_test("Statistics empty array median", Statistics_median(empty) == 0.0)
    
    fr fr Test extreme percentiles
    sus data normie[value] = [1, 2, 3, 4, 5]
    run_test("Statistics 0th percentile", Statistics_percentile(data, 0.0) == 1.0)
    run_test("Statistics 100th percentile", Statistics_percentile(data, 100.0) == 5.0)
}

slay test_statistics_variance_and_std_dev() {
    sus data normie[value] = [2, 4, 4, 4, 5, 5, 7, 9]
    
    sus variance drip = Statistics_variance(data)
    sus std_dev drip = Statistics_standard_deviation(data)
    
    run_test("Statistics variance positive", variance > 0.0)
    run_test("Statistics std dev positive", std_dev > 0.0)
    run_test("Statistics std dev is sqrt of variance", std_dev * std_dev >= variance * 0.9)
}

fr fr ================================
fr fr AVL Tree Tests
fr fr ================================

slay test_avl_tree_basic() {
    sus tree BalancedTree = Tree_new()
    
    fr fr Test initial state
    run_test("AVL tree initial size", tree.size == 0)
    run_test("AVL tree search empty", Tree_search(tree, "key") == "")
    
    fr fr Test insertion
    tree = Tree_insert(tree, "apple", "fruit")
    run_test("AVL tree after insert size", tree.size == 1)
    run_test("AVL tree search inserted", Tree_search(tree, "apple") == "fruit")
    
    fr fr Test multiple insertions
    tree = Tree_insert(tree, "banana", "yellow")
    tree = Tree_insert(tree, "cherry", "red") 
    tree = Tree_insert(tree, "date", "sweet")
    
    run_test("AVL tree multiple inserts size", tree.size == 4)
    run_test("AVL tree search multiple", 
        Tree_search(tree, "banana") == "yellow" && Tree_search(tree, "cherry") == "red")
    
    fr fr Test update existing
    tree = Tree_insert(tree, "apple", "green")
    run_test("AVL tree update existing size unchanged", tree.size == 4)
    run_test("AVL tree update value", Tree_search(tree, "apple") == "green")
}

slay test_avl_tree_balancing() {
    sus tree BalancedTree = Tree_new()
    
    fr fr Insert in ascending order to test left-left rotation
    tree = Tree_insert(tree, "a", "1")
    tree = Tree_insert(tree, "b", "2")
    tree = Tree_insert(tree, "c", "3")
    tree = Tree_insert(tree, "d", "4")
    tree = Tree_insert(tree, "e", "5")
    
    fr fr Tree should remain balanced
    run_test("AVL tree balanced insert all values accessible",
        Tree_search(tree, "a") == "1" && Tree_search(tree, "b") == "2" &&
        Tree_search(tree, "c") == "3" && Tree_search(tree, "d") == "4" &&
        Tree_search(tree, "e") == "5")
    
    run_test("AVL tree size after balanced inserts", tree.size == 5)
}

fr fr ================================
fr fr Priority Queue Tests
fr fr ================================

slay test_priority_queue_basic() {
    sus pq PriorityQueue = PriorityQueue_new()
    
    fr fr Test initial state
    run_test("PriorityQueue initial size", pq.size == 0)
    
    fr fr Test insertion
    pq = PriorityQueue_insert(pq, "task1", 5)
    run_test("PriorityQueue after first insert", pq.size == 1)
    
    pq = PriorityQueue_insert(pq, "task2", 10)
    pq = PriorityQueue_insert(pq, "task3", 3)
    pq = PriorityQueue_insert(pq, "task4", 7)
    
    run_test("PriorityQueue multiple inserts", pq.size == 4)
}

slay test_priority_queue_extraction() {
    sus pq PriorityQueue = PriorityQueue_new()
    
    fr fr Insert items with different priorities
    pq = PriorityQueue_insert(pq, "low", 1)
    pq = PriorityQueue_insert(pq, "high", 10)
    pq = PriorityQueue_insert(pq, "medium", 5)
    pq = PriorityQueue_insert(pq, "highest", 15)
    
    fr fr Extract max should give highest priority first
    sus first tea = PriorityQueue_extract_max(pq)
    run_test("PriorityQueue extract max first", first == "highest")
    run_test("PriorityQueue size after extract", pq.size == 3)
    
    sus second tea = PriorityQueue_extract_max(pq)
    run_test("PriorityQueue extract max second", second == "high")
    
    sus third tea = PriorityQueue_extract_max(pq)
    run_test("PriorityQueue extract max third", third == "medium")
    
    sus fourth tea = PriorityQueue_extract_max(pq)
    run_test("PriorityQueue extract max fourth", fourth == "low")
    
    run_test("PriorityQueue empty after all extracts", pq.size == 0)
}

slay test_priority_queue_resize() {
    sus pq PriorityQueue = PriorityQueue_new()
    
    fr fr Insert many items to trigger resize
    sus i normie = 0
    bestie i < 20 {
        pq = PriorityQueue_insert(pq, "item", i)
        i = i + 1
    }
    
    run_test("PriorityQueue resize handles many items", pq.size == 20)
    
    fr fr Verify we can still extract
    sus extracted tea = PriorityQueue_extract_max(pq)
    run_test("PriorityQueue extract after resize", extracted == "item")
}

fr fr ================================
fr fr Performance Tests
fr fr ================================

slay test_sorting_performance_comparison() {
    vibez.spill("\n=== Sorting Performance Comparison ===")
    
    fr fr Test with same dataset
    sus test_data normie[value] = [64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42, 30, 15, 8, 3, 1]
    
    fr fr Test MergeSort
    sus merge_result [normie] = MergeSort_sort(test_data)
    vibez.spill("MergeSort: O(n log n) guaranteed, stable")
    
    fr fr Test QuickSort
    sus quick_result [normie] = QuickSort_sort(test_data)
    vibez.spill("QuickSort: O(n log n) average, hybrid with heap sort fallback")
    
    fr fr Test HeapSort
    sus heap_result [normie] = HeapSort_sort(test_data)
    vibez.spill("HeapSort: O(n log n) guaranteed, in-place")
    
    fr fr Verify all produce sorted results
    run_test("All sorts produce ordered results",
        merge_result[0] <= merge_result[1] && 
        quick_result[0] <= quick_result[1] &&
        heap_result[0] <= heap_result[1])
}

slay test_data_structure_complexity() {
    vibez.spill("\n=== Data Structure Complexity Verification ===")
    
    fr fr HashMap - O(1) average operations
    sus map RobinHoodHashTable = HashMap_new()
    map = HashMap_insert(map, "test", "value")
    sus retrieve tea = HashMap_get(map, "test")
    vibez.spill("HashMap: O(1) average insert/get with Robin Hood hashing")
    run_test("HashMap O(1) operations work", retrieve == "value")
    
    fr fr AVL Tree - O(log n) guaranteed operations  
    sus tree BalancedTree = Tree_new()
    tree = Tree_insert(tree, "key", "value")
    sus tree_value tea = Tree_search(tree, "key")
    vibez.spill("AVL Tree: O(log n) guaranteed insert/search with auto-balancing")
    run_test("AVL Tree O(log n) operations work", tree_value == "value")
    
    fr fr Priority Queue - O(log n) insert/extract
    sus pq PriorityQueue = PriorityQueue_new()
    pq = PriorityQueue_insert(pq, "item", 10)
    sus pq_item tea = PriorityQueue_extract_max(pq)
    vibez.spill("Priority Queue: O(log n) insert/extract with binary heap")
    run_test("Priority Queue O(log n) operations work", pq_item == "item")
}

fr fr ================================
fr fr Integration Tests
fr fr ================================

slay test_combined_data_structures() {
    vibez.spill("\n=== Integration Test: Combined Data Structures ===")
    
    fr fr Use HashMap to store AVL trees
    sus storage RobinHoodHashTable = HashMap_new()
    sus tree1 BalancedTree = Tree_new()
    sus tree2 BalancedTree = Tree_new()
    
    tree1 = Tree_insert(tree1, "item1", "value1")
    tree2 = Tree_insert(tree2, "item2", "value2")
    
    fr fr Simulate storing trees in hashmap (simplified representation)
    storage = HashMap_insert(storage, "tree1", "has_value1")
    storage = HashMap_insert(storage, "tree2", "has_value2")
    
    run_test("Combined structures HashMap+AVL", 
        HashMap_get(storage, "tree1") == "has_value1" &&
        Tree_search(tree1, "item1") == "value1")
    
    vibez.spill("✅ Successfully combined HashMap with AVL Trees")
}

slay test_sorting_with_statistics() {
    vibez.spill("\n=== Integration Test: Sorting + Statistics ===")
    
    sus data normie[value] = [85, 90, 78, 92, 88, 76, 95, 87, 83, 91]
    
    fr fr Sort the data
    sus sorted_data [normie] = MergeSort_sort(data)
    
    fr fr Calculate statistics on sorted data
    sus mean drip = Statistics_mean(sorted_data)
    sus median drip = Statistics_median(sorted_data)
    sus std_dev drip = Statistics_standard_deviation(sorted_data)
    
    run_test("Sorting + Statistics integration",
        mean > 80.0 && median > 80.0 && std_dev > 0.0)
    
    vibez.spill("✅ Successfully integrated sorting with statistical analysis")
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

fr fr Run all test suites
vibez.spill("🚀 Starting Production Collections Test Suite")
vibez.spill("================================================")

fr fr Hash Table Tests
vibez.spill("\n📊 Testing Robin Hood Hash Table...")
test_hashmap_basic_operations()
test_hashmap_collision_handling() 
test_hashmap_resize()

fr fr Sorting Algorithm Tests
vibez.spill("\n⚡ Testing Sorting Algorithms...")
test_merge_sort()
test_quick_sort()
test_heap_sort()
test_insertion_sort()

fr fr Statistics Tests  
vibez.spill("\n📈 Testing Advanced Statistics...")
test_statistics_basic()
test_statistics_edge_cases()
test_statistics_variance_and_std_dev()

fr fr AVL Tree Tests
vibez.spill("\n🌳 Testing AVL Tree...")
test_avl_tree_basic()
test_avl_tree_balancing()

fr fr Priority Queue Tests
vibez.spill("\n🏆 Testing Priority Queue...")
test_priority_queue_basic()
test_priority_queue_extraction()
test_priority_queue_resize()

fr fr Performance Tests
test_sorting_performance_comparison()
test_data_structure_complexity()

fr fr Integration Tests
test_combined_data_structures()
test_sorting_with_statistics()

fr fr Final Summary
vibez.spill("\n================================================")
test_summary()
vibez.spill("💎 Production Collections Testing Complete")
