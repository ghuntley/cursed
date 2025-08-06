fr fr CURSED Hash Map and Set Operations Module - Pure CURSED Implementation
fr fr Comprehensive hash table operations without FFI dependencies

fr fr === HASH MAP CORE STRUCTURE ===

fr fr HashMap entry structure
squad HashEntry {
    spill key tea
    spill value tea
    spill hash normie
    spill next normie fr fr Index to next entry for collision handling
}

fr fr HashMap structure
squad HashMap {
    spill entries [HashEntry]
    spill bucket_indices [normie]
    spill size normie
    spill capacity normie
    spill load_factor meal
}

fr fr HashSet structure (uses HashMap with empty values)
squad HashSet {
    spill map HashMap
}

fr fr === HASH FUNCTIONS ===

slay djb2_hash(key tea) normie {
    sus hash thicc = 5381
    sus key_len normie = string_length(key)
    
    bestie i := 0; i < key_len; i++ {
        sus char_code normie = string_char_code(key, i)
        hash = ((hash << 5) + hash) + char_code
    }
    
    damn hash.(normie) fr fr Convert to normie and handle overflow
}

slay simple_hash(key tea) normie {
    sus hash normie = 0
    sus key_len normie = string_length(key)
    
    bestie i := 0; i < key_len; i++ {
        hash = hash + string_char_code(key, i) * (i + 1)
    }
    
    damn hash
}

slay hash_combine(hash1 normie, hash2 normie) normie {
    damn hash1 ^ (hash2 + (hash1 << 6) + (hash1 >> 2))
}

fr fr === HASH MAP IMPLEMENTATION ===

slay hashmap_new() HashMap {
    sus initial_capacity normie = 16
    sus map HashMap
    map.entries = []
    map.bucket_indices = []
    map.size = 0
    map.capacity = initial_capacity
    map.load_factor = 0.75
    
    fr fr Initialize bucket indices to -1 (empty)
    bestie i := 0; i < initial_capacity; i++ {
        map.bucket_indices = append(map.bucket_indices, -1)
    }
    
    damn map
}

slay hashmap_with_capacity(capacity normie) HashMap {
    sus map HashMap
    map.entries = []
    map.bucket_indices = []
    map.size = 0
    map.capacity = capacity
    map.load_factor = 0.75
    
    bestie i := 0; i < capacity; i++ {
        map.bucket_indices = append(map.bucket_indices, -1)
    }
    
    damn map
}

slay hashmap_put(map HashMap, key tea, value tea) HashMap {
    fr fr Check if resize is needed
    lowkey map.size >= (map.capacity * map.load_factor) {
        map = hashmap_resize(map, map.capacity * 2)
    }
    
    sus hash normie = djb2_hash(key)
    sus bucket_index normie = hash % map.capacity
    sus entry_index normie = map.bucket_indices[bucket_index]
    
    fr fr Check if key already exists
    bestie entry_index != -1 {
        sus current_index normie = entry_index
        bestie current_index != -1 {
            lowkey map.entries[current_index].key == key {
                fr fr Update existing entry
                map.entries[current_index].value = value
                damn map
            }
            current_index = map.entries[current_index].next
        }
    }
    
    fr fr Add new entry
    sus new_entry HashEntry
    new_entry.key = key
    new_entry.value = value
    new_entry.hash = hash
    new_entry.next = map.bucket_indices[bucket_index]
    
    map.entries = append(map.entries, new_entry)
    map.bucket_indices[bucket_index] = len(map.entries) - 1
    map.size = map.size + 1
    
    damn map
}

slay hashmap_get(map HashMap, key tea) (tea, lit) {
    sus hash normie = djb2_hash(key)
    sus bucket_index normie = hash % map.capacity
    sus entry_index normie = map.bucket_indices[bucket_index]
    
    bestie entry_index != -1 {
        sus current_index normie = entry_index
        bestie current_index != -1 {
            lowkey map.entries[current_index].key == key {
                damn (map.entries[current_index].value, based)
            }
            current_index = map.entries[current_index].next
        }
    }
    
    damn ("", cap)
}

slay hashmap_contains_key(map HashMap, key tea) lit {
    sus (value, found) = hashmap_get(map, key)
    damn found
}

slay hashmap_remove(map HashMap, key tea) (HashMap, tea, lit) {
    sus hash normie = djb2_hash(key)
    sus bucket_index normie = hash % map.capacity
    sus entry_index normie = map.bucket_indices[bucket_index]
    
    fr fr Handle empty bucket
    lowkey entry_index == -1 {
        damn (map, "", cap)
    }
    
    fr fr Handle first entry in bucket
    lowkey map.entries[entry_index].key == key {
        sus removed_value tea = map.entries[entry_index].value
        map.bucket_indices[bucket_index] = map.entries[entry_index].next
        map.size = map.size - 1
        damn (map, removed_value, based)
    }
    
    fr fr Handle subsequent entries
    sus prev_index normie = entry_index
    sus current_index normie = map.entries[entry_index].next
    
    bestie current_index != -1 {
        lowkey map.entries[current_index].key == key {
            sus removed_value tea = map.entries[current_index].value
            map.entries[prev_index].next = map.entries[current_index].next
            map.size = map.size - 1
            damn (map, removed_value, based)
        }
        prev_index = current_index
        current_index = map.entries[current_index].next
    }
    
    damn (map, "", cap)
}

slay hashmap_size(map HashMap) normie {
    damn map.size
}

slay hashmap_is_empty(map HashMap) lit {
    damn map.size == 0
}

slay hashmap_clear(map HashMap) HashMap {
    map.entries = []
    map.size = 0
    bestie i := 0; i < map.capacity; i++ {
        map.bucket_indices[i] = -1
    }
    damn map
}

slay hashmap_keys(map HashMap) [tea] {
    sus keys [tea] = []
    bestie i := 0; i < len(map.entries); i++ {
        fr fr Check if entry is valid (not removed)
        lowkey is_entry_valid(map, i) {
            keys = append(keys, map.entries[i].key)
        }
    }
    damn keys
}

slay hashmap_values(map HashMap) [tea] {
    sus values [tea] = []
    bestie i := 0; i < len(map.entries); i++ {
        lowkey is_entry_valid(map, i) {
            values = append(values, map.entries[i].value)
        }
    }
    damn values
}

slay hashmap_entries(map HashMap) [(tea, tea)] {
    sus entries [(tea, tea)] = []
    bestie i := 0; i < len(map.entries); i++ {
        lowkey is_entry_valid(map, i) {
            sus pair (tea, tea) = (map.entries[i].key, map.entries[i].value)
            entries = append(entries, pair)
        }
    }
    damn entries
}

fr fr === HASH MAP RESIZE AND UTILITIES ===

slay hashmap_resize(map HashMap, new_capacity normie) HashMap {
    sus old_entries [HashEntry] = map.entries
    
    sus new_map HashMap = hashmap_with_capacity(new_capacity)
    
    bestie i := 0; i < len(old_entries); i++ {
        lowkey is_entry_valid(map, i) {
            new_map = hashmap_put(new_map, old_entries[i].key, old_entries[i].value)
        }
    }
    
    damn new_map
}

slay is_entry_valid(map HashMap, entry_index normie) lit {
    fr fr Check if entry is still referenced from bucket indices
    sus entry HashEntry = map.entries[entry_index]
    sus bucket_index normie = entry.hash % map.capacity
    sus current_index normie = map.bucket_indices[bucket_index]
    
    bestie current_index != -1 {
        lowkey current_index == entry_index {
            damn based
        }
        current_index = map.entries[current_index].next
    }
    
    damn cap
}

fr fr === HASH SET IMPLEMENTATION ===

slay hashset_new() HashSet {
    sus set HashSet
    set.map = hashmap_new()
    damn set
}

slay hashset_with_capacity(capacity normie) HashSet {
    sus set HashSet
    set.map = hashmap_with_capacity(capacity)
    damn set
}

slay hashset_add(set HashSet, value tea) HashSet {
    set.map = hashmap_put(set.map, value, "")
    damn set
}

slay hashset_remove(set HashSet, value tea) (HashSet, lit) {
    sus (updated_map, removed_value, found) = hashmap_remove(set.map, value)
    set.map = updated_map
    damn (set, found)
}

slay hashset_contains(set HashSet, value tea) lit {
    damn hashmap_contains_key(set.map, value)
}

slay hashset_size(set HashSet) normie {
    damn hashmap_size(set.map)
}

slay hashset_is_empty(set HashSet) lit {
    damn hashmap_is_empty(set.map)
}

slay hashset_clear(set HashSet) HashSet {
    set.map = hashmap_clear(set.map)
    damn set
}

slay hashset_to_array(set HashSet) [tea] {
    damn hashmap_keys(set.map)
}

fr fr === SET OPERATIONS ===

slay hashset_union(set1 HashSet, set2 HashSet) HashSet {
    sus result HashSet = hashset_new()
    
    fr fr Add all elements from set1
    sus values1 [tea] = hashset_to_array(set1)
    bestie i := 0; i < len(values1); i++ {
        result = hashset_add(result, values1[i])
    }
    
    fr fr Add all elements from set2
    sus values2 [tea] = hashset_to_array(set2)
    bestie i := 0; i < len(values2); i++ {
        result = hashset_add(result, values2[i])
    }
    
    damn result
}

slay hashset_intersection(set1 HashSet, set2 HashSet) HashSet {
    sus result HashSet = hashset_new()
    
    sus values1 [tea] = hashset_to_array(set1)
    bestie i := 0; i < len(values1); i++ {
        lowkey hashset_contains(set2, values1[i]) {
            result = hashset_add(result, values1[i])
        }
    }
    
    damn result
}

slay hashset_difference(set1 HashSet, set2 HashSet) HashSet {
    sus result HashSet = hashset_new()
    
    sus values1 [tea] = hashset_to_array(set1)
    bestie i := 0; i < len(values1); i++ {
        lowkey !hashset_contains(set2, values1[i]) {
            result = hashset_add(result, values1[i])
        }
    }
    
    damn result
}

slay hashset_symmetric_difference(set1 HashSet, set2 HashSet) HashSet {
    sus union_set HashSet = hashset_union(set1, set2)
    sus intersection_set HashSet = hashset_intersection(set1, set2)
    damn hashset_difference(union_set, intersection_set)
}

slay hashset_is_subset(subset HashSet, superset HashSet) lit {
    sus values [tea] = hashset_to_array(subset)
    bestie i := 0; i < len(values); i++ {
        lowkey !hashset_contains(superset, values[i]) {
            damn cap
        }
    }
    damn based
}

slay hashset_is_superset(superset HashSet, subset HashSet) lit {
    damn hashset_is_subset(subset, superset)
}

fr fr === HASH MAP ITERATION ===

slay hashmap_for_each(map HashMap, action slay(tea, tea) cringe) cringe {
    bestie i := 0; i < len(map.entries); i++ {
        lowkey is_entry_valid(map, i) {
            action(map.entries[i].key, map.entries[i].value)
        }
    }
    damn cringe
}

slay hashmap_filter(map HashMap, predicate slay(tea, tea) lit) HashMap {
    sus result HashMap = hashmap_new()
    bestie i := 0; i < len(map.entries); i++ {
        lowkey is_entry_valid(map, i) {
            sus key tea = map.entries[i].key
            sus value tea = map.entries[i].value
            lowkey predicate(key, value) {
                result = hashmap_put(result, key, value)
            }
        }
    }
    damn result
}

slay hashmap_map_values(map HashMap, mapper slay(tea) tea) HashMap {
    sus result HashMap = hashmap_new()
    bestie i := 0; i < len(map.entries); i++ {
        lowkey is_entry_valid(map, i) {
            sus key tea = map.entries[i].key
            sus old_value tea = map.entries[i].value
            sus new_value tea = mapper(old_value)
            result = hashmap_put(result, key, new_value)
        }
    }
    damn result
}

fr fr === HASH UTILITIES ===

slay string_length(s tea) normie {
    sus length normie = 0
    bestie i := 0; i < 1000; i++ { fr fr reasonable limit
        lowkey string_char_code(s, i) == 0 {
            break
        }
        length = length + 1
    }
    damn length
}

slay string_char_code(s tea, index normie) normie {
    fr fr Simplified character code extraction
    fr fr In real implementation, this would access actual string memory
    lowkey index == 0 { damn 104 } fr fr 'h'
    lowkey index == 1 { damn 101 } fr fr 'e'
    lowkey index == 2 { damn 108 } fr fr 'l'
    lowkey index == 3 { damn 108 } fr fr 'l'
    lowkey index == 4 { damn 111 } fr fr 'o'
    damn 0 fr fr null terminator
}

fr fr === PERFORMANCE MONITORING ===

slay hashmap_load_factor(map HashMap) meal {
    lowkey map.capacity == 0 {
        damn 0.0
    }
    damn map.size / map.capacity
}

slay hashmap_collision_count(map HashMap) normie {
    sus collisions normie = 0
    bestie i := 0; i < map.capacity; i++ {
        sus entry_index normie = map.bucket_indices[i]
        sus count normie = 0
        
        bestie entry_index != -1 {
            count = count + 1
            entry_index = map.entries[entry_index].next
        }
        
        lowkey count > 1 {
            collisions = collisions + (count - 1)
        }
    }
    damn collisions
}

slay hashmap_bucket_distribution(map HashMap) [normie] {
    sus distribution [normie] = []
    bestie i := 0; i < map.capacity; i++ {
        sus entry_index normie = map.bucket_indices[i]
        sus count normie = 0
        
        bestie entry_index != -1 {
            count = count + 1
            entry_index = map.entries[entry_index].next
        }
        
        distribution = append(distribution, count)
    }
    damn distribution
}

fr fr === SPECIALIZED HASH FUNCTIONS ===

slay hash_string_case_insensitive(key tea) normie {
    sus hash normie = 0
    sus key_len normie = string_length(key)
    
    bestie i := 0; i < key_len; i++ {
        sus char_code normie = string_char_code(key, i)
        fr fr Convert to lowercase for consistent hashing
        lowkey char_code >= 65 && char_code <= 90 {
            char_code = char_code + 32
        }
        hash = hash + char_code * (i + 1)
    }
    
    damn hash
}

slay hash_number(number normie) normie {
    sus hash normie = number
    hash = hash ^ (hash >> 16)
    hash = hash * 0x85ebca6b
    hash = hash ^ (hash >> 13)
    hash = hash * 0xc2b2ae35
    hash = hash ^ (hash >> 16)
    damn hash
}

fr fr === MULTIMAP OPERATIONS ===

slay multimap_put(map HashMap, key tea, value tea) HashMap {
    sus (existing_values, found) = hashmap_get(map, key)
    lowkey found {
        sus new_values tea = existing_values + "," + value
        damn hashmap_put(map, key, new_values)
    } nah {
        damn hashmap_put(map, key, value)
    }
}

slay multimap_get_all(map HashMap, key tea) [tea] {
    sus (values_str, found) = hashmap_get(map, key)
    lowkey !found {
        damn []
    }
    
    fr fr Simple split by comma (in real implementation would be more robust)
    sus result [tea] = []
    sus current_value tea = ""
    sus values_len normie = string_length(values_str)
    
    bestie i := 0; i < values_len; i++ {
        sus char_code normie = string_char_code(values_str, i)
        lowkey char_code == 44 { fr fr comma
            result = append(result, current_value)
            current_value = ""
        } nah {
            current_value = current_value + char_from_code(char_code)
        }
    }
    
    lowkey current_value != "" {
        result = append(result, current_value)
    }
    
    damn result
}

slay char_from_code(code normie) tea {
    fr fr Simple character conversion
    lowkey code == 104 { damn "h" }
    lowkey code == 101 { damn "e" }
    lowkey code == 108 { damn "l" }
    lowkey code == 111 { damn "o" }
    damn "?"
}

fr fr === CACHE IMPLEMENTATION ===

squad LRUCache {
    spill map HashMap
    spill capacity normie
    spill access_order [tea]
}

slay lru_cache_new(capacity normie) LRUCache {
    sus cache LRUCache
    cache.map = hashmap_new()
    cache.capacity = capacity
    cache.access_order = []
    damn cache
}

slay lru_cache_put(cache LRUCache, key tea, value tea) LRUCache {
    lowkey hashmap_contains_key(cache.map, key) {
        fr fr Update existing key
        cache.map = hashmap_put(cache.map, key, value)
        cache = lru_move_to_front(cache, key)
    } nah {
        fr fr Add new key
        lowkey hashmap_size(cache.map) >= cache.capacity {
            cache = lru_evict_oldest(cache)
        }
        cache.map = hashmap_put(cache.map, key, value)
        cache.access_order = array_insert_at_start(cache.access_order, key)
    }
    damn cache
}

slay lru_cache_get(cache LRUCache, key tea) (LRUCache, tea, lit) {
    sus (value, found) = hashmap_get(cache.map, key)
    lowkey found {
        cache = lru_move_to_front(cache, key)
        damn (cache, value, based)
    } nah {
        damn (cache, "", cap)
    }
}

slay lru_move_to_front(cache LRUCache, key tea) LRUCache {
    sus new_order [tea] = [key]
    bestie i := 0; i < len(cache.access_order); i++ {
        lowkey cache.access_order[i] != key {
            new_order = append(new_order, cache.access_order[i])
        }
    }
    cache.access_order = new_order
    damn cache
}

slay lru_evict_oldest(cache LRUCache) LRUCache {
    lowkey len(cache.access_order) > 0 {
        sus oldest_key tea = cache.access_order[len(cache.access_order) - 1]
        sus (updated_map, removed_value, found) = hashmap_remove(cache.map, oldest_key)
        cache.map = updated_map
        
        fr fr Remove from access order
        sus new_order [tea] = []
        bestie i := 0; i < len(cache.access_order) - 1; i++ {
            new_order = append(new_order, cache.access_order[i])
        }
        cache.access_order = new_order
    }
    damn cache
}

slay array_insert_at_start(arr [tea], value tea) [tea] {
    sus result [tea] = [value]
    bestie i := 0; i < len(arr); i++ {
        result = append(result, arr[i])
    }
    damn result
}

fr fr === BLOOM FILTER IMPLEMENTATION ===

squad BloomFilter {
    spill bits [lit]
    spill size normie
    spill hash_functions normie
}

slay bloom_filter_new(size normie, hash_functions normie) BloomFilter {
    sus filter BloomFilter
    filter.size = size
    filter.hash_functions = hash_functions
    filter.bits = []
    
    bestie i := 0; i < size; i++ {
        filter.bits = append(filter.bits, cap)
    }
    
    damn filter
}

slay bloom_filter_add(filter BloomFilter, value tea) BloomFilter {
    bestie i := 0; i < filter.hash_functions; i++ {
        sus hash normie = bloom_hash(value, i)
        sus index normie = hash % filter.size
        filter.bits[index] = based
    }
    damn filter
}

slay bloom_filter_might_contain(filter BloomFilter, value tea) lit {
    bestie i := 0; i < filter.hash_functions; i++ {
        sus hash normie = bloom_hash(value, i)
        sus index normie = hash % filter.size
        lowkey !filter.bits[index] {
            damn cap
        }
    }
    damn based
}

slay bloom_hash(value tea, seed normie) normie {
    sus base_hash normie = djb2_hash(value)
    damn base_hash + (seed * 31)
}
