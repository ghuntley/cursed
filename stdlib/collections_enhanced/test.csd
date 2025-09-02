yeet "testz"
yeet "collections_enhanced"

test_start("COLLECTIONS_ENHANCED Advanced Data Structure Tests")

// Test enhanced hash maps with custom algorithms
sus hash_map sus_map<tea, drip> = create_optimized_hashmap(1000)
hash_map["key1"] = 100
hash_map["key2"] = 200
assert_eq_int(hash_map["key1"], 100)
assert_eq_int(len(hash_map), 2)

// Test concurrent hash map
sus concurrent_map sus_concurrent_map<tea, drip> = create_concurrent_hashmap()
concurrent_put(concurrent_map, "thread1", 1)
concurrent_put(concurrent_map, "thread2", 2)
sus val1 drip = concurrent_get(concurrent_map, "thread1")
assert_eq_int(val1, 1)

// Test bloom filter for fast lookups
sus bloom sus_bloom = create_bloom_filter(1000, 0.01)
bloom_add(bloom, "item1")
bloom_add(bloom, "item2")
assert_true(bloom_contains(bloom, "item1"))
assert_false(bloom_contains(bloom, "item3"))

// Test trie for prefix operations
sus trie sus_trie = create_trie()
trie_insert(trie, "hello")
trie_insert(trie, "help")
trie_insert(trie, "world")
assert_true(trie_contains(trie, "hello"))
sus prefixes tea[value] = trie_find_prefixes(trie, "hel")
assert_eq_int(len(prefixes), 2)

// Test priority queue with custom comparator
sus pq sus_priority_queue<drip> = create_priority_queue_max()
pq_push(pq, 3)
pq_push(pq, 1)
pq_push(pq, 4)
sus max_val drip = pq_pop(pq)
assert_eq_int(max_val, 4)

// Test balanced binary search tree
sus bst sus_bst<drip> = create_avl_tree()
bst_insert(bst, 10)
bst_insert(bst, 5)
bst_insert(bst, 15)
bst_insert(bst, 3)
assert_true(bst_contains(bst, 5))
assert_false(bst_contains(bst, 7))
assert_true(bst_is_balanced(bst))

// Test LRU cache implementation
sus lru sus_lru_cache<tea, drip> = create_lru_cache(3)
lru_put(lru, "a", 1)
lru_put(lru, "b", 2)
lru_put(lru, "c", 3)
lru_put(lru, "d", 4) // Should evict "a"
assert_false(lru_contains(lru, "a"))
assert_true(lru_contains(lru, "d"))

// Test skip list for ordered operations
sus skip_list sus_skip_list<drip> = create_skip_list()
skip_list_insert(skip_list, 1)
skip_list_insert(skip_list, 3)
skip_list_insert(skip_list, 2)
sus ordered_values drip[value] = skip_list_to_array(skip_list)
assert_eq_int(ordered_values[0], 1)
assert_eq_int(ordered_values[2], 3)

// Test segment tree for range queries
sus seg_tree sus_segment_tree = create_segment_tree([1, 3, 5, 7, 9, 11])
sus range_sum drip = segment_tree_range_sum(seg_tree, 1, 3) // indices 1-3
assert_eq_int(range_sum, 15) // 3 + 5 + 7

// Test disjoint set (Union-Find)
sus union_find sus_union_find = create_union_find(10)
union_find_union(union_find, 0, 1)
union_find_union(union_find, 2, 3)
assert_true(union_find_connected(union_find, 0, 1))
assert_false(union_find_connected(union_find, 0, 2))

// Test graph data structures
sus graph sus_graph = create_adjacency_list_graph(5)
graph_add_edge(graph, 0, 1)
graph_add_edge(graph, 1, 2)
graph_add_edge(graph, 2, 3)
sus path drip[value] = graph_shortest_path(graph, 0, 3)
assert_eq_int(len(path), 4)

// Test circular buffer
sus circular_buffer sus_circular_buffer<drip> = create_circular_buffer(5)
circular_buffer_push(circular_buffer, 1)
circular_buffer_push(circular_buffer, 2)
circular_buffer_push(circular_buffer, 3)
sus first drip = circular_buffer_pop(circular_buffer)
assert_eq_int(first, 1)

// Test memory-efficient packed arrays
sus packed_bools sus_packed_array<lit> = create_packed_boolean_array(1000)
packed_array_set(packed_bools, 100, based)
packed_array_set(packed_bools, 500, nocap)
assert_true(packed_array_get(packed_bools, 100))
assert_false(packed_array_get(packed_bools, 500))

// Test performance with large collections
sus large_map sus_map<drip, drip> = create_optimized_hashmap(100000)
sus perf_start drip = get_nanoseconds()
bestie (sus i drip = 0; i < 10000; i++) {
    large_map[i] = i * 2
}
sus perf_end drip = get_nanoseconds()
sus perf_duration drip = perf_end - perf_start
assert_true(perf_duration < 100000000) // Less than 100ms for 10k insertions
assert_eq_int(large_map[5000], 10000)

print_test_summary()
