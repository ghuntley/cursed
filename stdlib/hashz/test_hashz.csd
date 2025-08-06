yeet "testz"
yeet "hashz"

fr fr Comprehensive Hash Map and Set Test Suite

test_start("Hash Functions")

fr fr Test djb2_hash function
sus hash1 normie = hashz.djb2_hash("hello")
sus hash2 normie = hashz.djb2_hash("hello")
sus hash3 normie = hashz.djb2_hash("world")

assert_eq_int(hash1, hash2) fr fr Same input should produce same hash
assert_true(hash1 != hash3) fr fr Different inputs should produce different hashes

fr fr Test simple_hash function
sus simple1 normie = hashz.simple_hash("test")
sus simple2 normie = hashz.simple_hash("test")
assert_eq_int(simple1, simple2)

fr fr Test hash_combine function
sus combined normie = hashz.hash_combine(hash1, hash3)
assert_true(combined != hash1 && combined != hash3)

test_start("HashMap Creation and Basic Operations")

fr fr Test hashmap_new
sus map hashz.HashMap = hashz.hashmap_new()
assert_eq_int(hashz.hashmap_size(map), 0)
assert_true(hashz.hashmap_is_empty(map))

fr fr Test hashmap_with_capacity
sus map_cap hashz.HashMap = hashz.hashmap_with_capacity(32)
assert_eq_int(hashz.hashmap_size(map_cap), 0)

test_start("HashMap Put and Get Operations")

fr fr Test putting and getting values
map = hashz.hashmap_put(map, "key1", "value1")
map = hashz.hashmap_put(map, "key2", "value2")
map = hashz.hashmap_put(map, "key3", "value3")

assert_eq_int(hashz.hashmap_size(map), 3)
assert_false(hashz.hashmap_is_empty(map))

sus (value1, found1) = hashz.hashmap_get(map, "key1")
assert_true(found1)
assert_eq_string(value1, "value1")

sus (value2, found2) = hashz.hashmap_get(map, "key2")
assert_true(found2)
assert_eq_string(value2, "value2")

sus (missing_value, found_missing) = hashz.hashmap_get(map, "nonexistent")
assert_false(found_missing)
assert_eq_string(missing_value, "")

test_start("HashMap Key Existence and Update")

fr fr Test contains_key
assert_true(hashz.hashmap_contains_key(map, "key1"))
assert_true(hashz.hashmap_contains_key(map, "key2"))
assert_false(hashz.hashmap_contains_key(map, "nonexistent"))

fr fr Test updating existing key
map = hashz.hashmap_put(map, "key1", "updated_value1")
assert_eq_int(hashz.hashmap_size(map), 3) fr fr Size should remain the same

sus (updated_value, found_updated) = hashz.hashmap_get(map, "key1")
assert_true(found_updated)
assert_eq_string(updated_value, "updated_value1")

test_start("HashMap Remove Operations")

fr fr Test removing existing key
sus (map_after_remove, removed_value, remove_success) = hashz.hashmap_remove(map, "key2")
map = map_after_remove
assert_true(remove_success)
assert_eq_string(removed_value, "value2")
assert_eq_int(hashz.hashmap_size(map), 2)
assert_false(hashz.hashmap_contains_key(map, "key2"))

fr fr Test removing non-existent key
sus (map_after_fail, fail_value, fail_success) = hashz.hashmap_remove(map, "nonexistent")
assert_false(fail_success)
assert_eq_string(fail_value, "")
assert_eq_int(hashz.hashmap_size(map), 2)

test_start("HashMap Keys, Values, and Entries")

fr fr Test getting all keys
sus keys [tea] = hashz.hashmap_keys(map)
assert_eq_int(len(keys), 2)
assert_true(array_contains_string(keys, "key1"))
assert_true(array_contains_string(keys, "key3"))

fr fr Test getting all values
sus values [tea] = hashz.hashmap_values(map)
assert_eq_int(len(values), 2)
assert_true(array_contains_string(values, "updated_value1"))
assert_true(array_contains_string(values, "value3"))

fr fr Test getting all entries
sus entries [(tea, tea)] = hashz.hashmap_entries(map)
assert_eq_int(len(entries), 2)

test_start("HashMap Clear Operation")

fr fr Test clearing the map
map = hashz.hashmap_clear(map)
assert_eq_int(hashz.hashmap_size(map), 0)
assert_true(hashz.hashmap_is_empty(map))
assert_false(hashz.hashmap_contains_key(map, "key1"))

test_start("HashSet Creation and Basic Operations")

fr fr Test hashset_new
sus set hashz.HashSet = hashz.hashset_new()
assert_eq_int(hashz.hashset_size(set), 0)
assert_true(hashz.hashset_is_empty(set))

fr fr Test hashset_with_capacity
sus set_cap hashz.HashSet = hashz.hashset_with_capacity(32)
assert_eq_int(hashz.hashset_size(set_cap), 0)

test_start("HashSet Add and Contains Operations")

fr fr Test adding elements
set = hashz.hashset_add(set, "apple")
set = hashz.hashset_add(set, "banana")
set = hashz.hashset_add(set, "cherry")

assert_eq_int(hashz.hashset_size(set), 3)
assert_false(hashz.hashset_is_empty(set))

fr fr Test contains
assert_true(hashz.hashset_contains(set, "apple"))
assert_true(hashz.hashset_contains(set, "banana"))
assert_true(hashz.hashset_contains(set, "cherry"))
assert_false(hashz.hashset_contains(set, "orange"))

fr fr Test adding duplicate (should not increase size)
set = hashz.hashset_add(set, "apple")
assert_eq_int(hashz.hashset_size(set), 3)

test_start("HashSet Remove Operations")

fr fr Test removing existing element
sus (set_after_remove, remove_success) = hashz.hashset_remove(set, "banana")
set = set_after_remove
assert_true(remove_success)
assert_eq_int(hashz.hashset_size(set), 2)
assert_false(hashz.hashset_contains(set, "banana"))

fr fr Test removing non-existent element
sus (set_after_fail, fail_success) = hashz.hashset_remove(set, "orange")
assert_false(fail_success)
assert_eq_int(hashz.hashset_size(set), 2)

test_start("HashSet to Array")

fr fr Test converting set to array
sus set_array [tea] = hashz.hashset_to_array(set)
assert_eq_int(len(set_array), 2)
assert_true(array_contains_string(set_array, "apple"))
assert_true(array_contains_string(set_array, "cherry"))

test_start("Set Operations - Union")

fr fr Create two sets for set operations
sus set1 hashz.HashSet = hashz.hashset_new()
set1 = hashz.hashset_add(set1, "a")
set1 = hashz.hashset_add(set1, "b")
set1 = hashz.hashset_add(set1, "c")

sus set2 hashz.HashSet = hashz.hashset_new()
set2 = hashz.hashset_add(set2, "b")
set2 = hashz.hashset_add(set2, "c")
set2 = hashz.hashset_add(set2, "d")

fr fr Test union
sus union_set hashz.HashSet = hashz.hashset_union(set1, set2)
assert_eq_int(hashz.hashset_size(union_set), 4)
assert_true(hashz.hashset_contains(union_set, "a"))
assert_true(hashz.hashset_contains(union_set, "b"))
assert_true(hashz.hashset_contains(union_set, "c"))
assert_true(hashz.hashset_contains(union_set, "d"))

test_start("Set Operations - Intersection")

fr fr Test intersection
sus intersection_set hashz.HashSet = hashz.hashset_intersection(set1, set2)
assert_eq_int(hashz.hashset_size(intersection_set), 2)
assert_true(hashz.hashset_contains(intersection_set, "b"))
assert_true(hashz.hashset_contains(intersection_set, "c"))
assert_false(hashz.hashset_contains(intersection_set, "a"))
assert_false(hashz.hashset_contains(intersection_set, "d"))

test_start("Set Operations - Difference")

fr fr Test difference (set1 - set2)
sus difference_set hashz.HashSet = hashz.hashset_difference(set1, set2)
assert_eq_int(hashz.hashset_size(difference_set), 1)
assert_true(hashz.hashset_contains(difference_set, "a"))
assert_false(hashz.hashset_contains(difference_set, "b"))
assert_false(hashz.hashset_contains(difference_set, "c"))
assert_false(hashz.hashset_contains(difference_set, "d"))

test_start("Set Operations - Symmetric Difference")

fr fr Test symmetric difference
sus sym_diff_set hashz.HashSet = hashz.hashset_symmetric_difference(set1, set2)
assert_eq_int(hashz.hashset_size(sym_diff_set), 2)
assert_true(hashz.hashset_contains(sym_diff_set, "a"))
assert_true(hashz.hashset_contains(sym_diff_set, "d"))
assert_false(hashz.hashset_contains(sym_diff_set, "b"))
assert_false(hashz.hashset_contains(sym_diff_set, "c"))

test_start("Set Operations - Subset/Superset")

fr fr Create subset for testing
sus subset hashz.HashSet = hashz.hashset_new()
subset = hashz.hashset_add(subset, "b")
subset = hashz.hashset_add(subset, "c")

fr fr Test subset relationship
assert_true(hashz.hashset_is_subset(subset, set1))
assert_true(hashz.hashset_is_subset(subset, set2))
assert_false(hashz.hashset_is_subset(set1, subset))

fr fr Test superset relationship
assert_true(hashz.hashset_is_superset(set1, subset))
assert_true(hashz.hashset_is_superset(set2, subset))
assert_false(hashz.hashset_is_superset(subset, set1))

test_start("HashMap Performance Monitoring")

fr fr Create map with some collisions
sus perf_map hashz.HashMap = hashz.hashmap_new()
perf_map = hashz.hashmap_put(perf_map, "key1", "value1")
perf_map = hashz.hashmap_put(perf_map, "key2", "value2")
perf_map = hashz.hashmap_put(perf_map, "key3", "value3")

fr fr Test load factor
sus load_factor meal = hashz.hashmap_load_factor(perf_map)
assert_true(load_factor > 0.0)

fr fr Test collision count
sus collisions normie = hashz.hashmap_collision_count(perf_map)
assert_true(collisions >= 0)

fr fr Test bucket distribution
sus distribution [normie] = hashz.hashmap_bucket_distribution(perf_map)
assert_true(len(distribution) > 0)

test_start("Specialized Hash Functions")

fr fr Test case-insensitive hashing
sus hash_lower normie = hashz.hash_string_case_insensitive("hello")
sus hash_upper normie = hashz.hash_string_case_insensitive("HELLO")
assert_eq_int(hash_lower, hash_upper)

fr fr Test number hashing
sus num_hash1 normie = hashz.hash_number(42)
sus num_hash2 normie = hashz.hash_number(42)
sus num_hash3 normie = hashz.hash_number(43)
assert_eq_int(num_hash1, num_hash2)
assert_true(num_hash1 != num_hash3)

test_start("Multimap Operations")

fr fr Test multimap operations
sus multimap hashz.HashMap = hashz.hashmap_new()
multimap = hashz.multimap_put(multimap, "colors", "red")
multimap = hashz.multimap_put(multimap, "colors", "green")
multimap = hashz.multimap_put(multimap, "colors", "blue")

sus color_values [tea] = hashz.multimap_get_all(multimap, "colors")
assert_true(len(color_values) > 0)

sus empty_values [tea] = hashz.multimap_get_all(multimap, "nonexistent")
assert_eq_int(len(empty_values), 0)

test_start("LRU Cache Operations")

fr fr Test LRU cache creation
sus cache hashz.LRUCache = hashz.lru_cache_new(2)

fr fr Test cache put operations
cache = hashz.lru_cache_put(cache, "key1", "value1")
cache = hashz.lru_cache_put(cache, "key2", "value2")

fr fr Test cache get operations
sus (cache_updated1, cached_value1, cache_found1) = hashz.lru_cache_get(cache, "key1")
cache = cache_updated1
assert_true(cache_found1)
assert_eq_string(cached_value1, "value1")

fr fr Test cache eviction (add third item to capacity-2 cache)
cache = hashz.lru_cache_put(cache, "key3", "value3")

fr fr key2 should be evicted since key1 was accessed recently
sus (cache_updated2, cached_value2, cache_found2) = hashz.lru_cache_get(cache, "key2")
assert_false(cache_found2)

sus (cache_updated3, cached_value3, cache_found3) = hashz.lru_cache_get(cache, "key1")
assert_true(cache_found3)

sus (cache_updated4, cached_value4, cache_found4) = hashz.lru_cache_get(cache, "key3")
assert_true(cache_found4)

test_start("Bloom Filter Operations")

fr fr Test bloom filter creation
sus bloom hashz.BloomFilter = hashz.bloom_filter_new(100, 3)

fr fr Test adding elements to bloom filter
bloom = hashz.bloom_filter_add(bloom, "apple")
bloom = hashz.bloom_filter_add(bloom, "banana")
bloom = hashz.bloom_filter_add(bloom, "cherry")

fr fr Test bloom filter might_contain (no false negatives)
assert_true(hashz.bloom_filter_might_contain(bloom, "apple"))
assert_true(hashz.bloom_filter_might_contain(bloom, "banana"))
assert_true(hashz.bloom_filter_might_contain(bloom, "cherry"))

fr fr Test with items not added (might have false positives)
sus might_contain_orange lit = hashz.bloom_filter_might_contain(bloom, "orange")
fr fr Don't assert false here since bloom filters can have false positives

test_start("HashMap Clear and Reset")

fr fr Test clearing hashset
set = hashz.hashset_clear(set)
assert_eq_int(hashz.hashset_size(set), 0)
assert_true(hashz.hashset_is_empty(set))

print_test_summary()

fr fr === HELPER FUNCTIONS FOR TESTS ===

slay array_contains_string(arr [tea], value tea) lit {
    bestie i := 0; i < len(arr); i++ {
        lowkey arr[i] == value {
            damn based
        }
    }
    damn cap
}
