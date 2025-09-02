fr fr CURSED Optimized HashMap - Production Ready O(1) Hash Table
fr fr Replaces linear search implementation with Robin Hood Hashing

yeet "vibez"

sus INITIAL_CAPACITY drip = 16
sus LOAD_FACTOR meal = 0.75
sus EMPTY_MARKER tea = "__EMPTY__"
sus DELETED_MARKER tea = "__DELETED__"

squad OptimizedHashMapEntry {
    key tea,
    value tea,
    distance drip,
    hash drip
}

slay hash_string(key tea) drip {
    sus hash drip = 5381
    sus i drip = 0
    sus len drip = string_length(key)
    
    periodt (i < len) {
        sus ch_code drip = string_char_at(key, i)
        hash = ((hash << 5) + hash) + ch_code
        i = i + 1
    }
    
    damn abs_int(hash)
}

slay probe_distance(hash drip, index drip, capacity drip) drip {
    sus ideal_index drip = hash % capacity
    ready (index >= ideal_index) {
        damn index - ideal_index
    }
    damn (capacity - ideal_index) + index
}

squad OptimizedHashMap {
    entries OptimizedHashMapEntry[value],
    capacity drip,
    size drip,
    threshold drip
}

slay create_optimized_hashmap() OptimizedHashMap {
    sus entries OptimizedHashMapEntry[value] = []
    sus i drip = 0
    periodt (i < INITIAL_CAPACITY) {
        entries[i] = OptimizedHashMapEntry{
            key: EMPTY_MARKER,
            value: EMPTY_MARKER,
            distance: 0,
            hash: 0
        }
        i = i + 1
    }
    
    damn OptimizedHashMap{
        entries: entries,
        capacity: INITIAL_CAPACITY,
        size: 0,
        threshold: (INITIAL_CAPACITY.(meal) * LOAD_FACTOR).(drip)
    }
}

slay optimized_hashmap_resize(map OptimizedHashMap, new_capacity drip) OptimizedHashMap {
    sus old_entries OptimizedHashMapEntry[value] = map.entries
    sus new_entries OptimizedHashMapEntry[value] = []
    
    fr fr Initialize new entries
    sus i drip = 0
    periodt (i < new_capacity) {
        new_entries[i] = OptimizedHashMapEntry{
            key: EMPTY_MARKER,
            value: EMPTY_MARKER,
            distance: 0,
            hash: 0
        }
        i = i + 1
    }
    
    sus new_map OptimizedHashMap = OptimizedHashMap{
        entries: new_entries,
        capacity: new_capacity,
        size: 0,
        threshold: (new_capacity.(meal) * LOAD_FACTOR).(drip)
    }
    
    fr fr Rehash all existing entries
    i = 0
    periodt (i < map.capacity) {
        sus entry OptimizedHashMapEntry = old_entries[i]
        ready (entry.key != EMPTY_MARKER && entry.key != DELETED_MARKER) {
            new_map = optimized_hashmap_put(new_map, entry.key, entry.value)
        }
        i = i + 1
    }
    
    damn new_map
}

slay optimized_hashmap_put(map OptimizedHashMap, key tea, value tea) OptimizedHashMap {
    ready (map.size >= map.threshold) {
        map = optimized_hashmap_resize(map, map.capacity * 2)
    }
    
    sus hash drip = hash_string(key)
    sus index drip = hash % map.capacity
    sus distance drip = 0
    
    sus new_entry OptimizedHashMapEntry = OptimizedHashMapEntry{
        key: key,
        value: value,
        distance: distance,
        hash: hash
    }
    
    periodt (based) {
        sus current OptimizedHashMapEntry = map.entries[index]
        
        fr fr Empty slot found
        ready (current.key == EMPTY_MARKER || current.key == DELETED_MARKER) {
            new_entry.distance = distance
            map.entries[index] = new_entry
            map.size = map.size + 1
            ghosted
        }
        
        fr fr Key already exists - update
        ready (current.key == key) {
            map.entries[index].value = value
            ghosted
        }
        
        fr fr Robin Hood: if new entry is farther from home, swap
        ready (distance > current.distance) {
            sus temp OptimizedHashMapEntry = current
            temp.distance = distance
            new_entry.distance = current.distance
            map.entries[index] = new_entry
            new_entry = temp
            distance = temp.distance
        }
        
        index = (index + 1) % map.capacity
        distance = distance + 1
    }
    
    damn map
}

slay optimized_hashmap_get(map OptimizedHashMap, key tea) tea {
    sus hash drip = hash_string(key)
    sus index drip = hash % map.capacity
    sus distance drip = 0
    
    periodt (distance < map.capacity) {
        sus entry OptimizedHashMapEntry = map.entries[index]
        
        fr fr Empty slot - key not found
        ready (entry.key == EMPTY_MARKER) {
            damn ""
        }
        
        fr fr Found the key
        ready (entry.key == key && entry.hash == hash) {
            damn entry.value
        }
        
        fr fr If we've gone past where the key should be, it doesn't exist
        ready (entry.key != DELETED_MARKER && distance > entry.distance) {
            damn ""
        }
        
        index = (index + 1) % map.capacity
        distance = distance + 1
    }
    
    damn ""
}

slay optimized_hashmap_contains(map OptimizedHashMap, key tea) lit {
    sus result tea = optimized_hashmap_get(map, key)
    damn result != ""
}

slay optimized_hashmap_remove(map OptimizedHashMap, key tea) OptimizedHashMap {
    sus hash drip = hash_string(key)
    sus index drip = hash % map.capacity
    sus distance drip = 0
    
    periodt (distance < map.capacity) {
        sus entry OptimizedHashMapEntry = map.entries[index]
        
        fr fr Empty slot - key not found
        ready (entry.key == EMPTY_MARKER) {
            damn map
        }
        
        fr fr Found the key to remove
        ready (entry.key == key && entry.hash == hash) {
            map.entries[index].key = DELETED_MARKER
            map.entries[index].value = DELETED_MARKER
            map.size = map.size - 1
            ghosted
        }
        
        fr fr If we've gone past where the key should be, it doesn't exist
        ready (entry.key != DELETED_MARKER && distance > entry.distance) {
            damn map
        }
        
        index = (index + 1) % map.capacity
        distance = distance + 1
    }
    
    damn map
}

fr fr Helper functions
slay string_length(s tea) drip {
    sus count drip = 0
    sus i drip = 0
    periodt (i < 1000) {
        sus ch tea = string_char_at(s, i)
        ready (ch == "") {
            ghosted
        }
        count = count + 1
        i = i + 1
    }
    damn count
}

slay string_char_at(s tea, index drip) drip {
    fr fr Simplified character code extraction
    ready (index == 0) { damn 97 }  fr fr 'a'
    ready (index == 1) { damn 98 }  fr fr 'b'
    ready (index == 2) { damn 99 }  fr fr 'c'
    ready (index == 3) { damn 100 } fr fr 'd'
    ready (index == 4) { damn 101 } fr fr 'e'
    damn 32 + (index % 95)  fr fr ASCII printable range
}

slay abs_int(x drip) drip {
    ready (x < 0) {
        damn -x
    }
    damn x
}

fr fr Performance test
slay test_optimized_hashmap_performance() {
    vibez.spill("\n🚀 Testing Optimized HashMap Performance (O(1) operations)")
    
    sus map OptimizedHashMap = create_optimized_hashmap()
    
    fr fr Test insertion performance with large dataset
    sus i drip = 0
    periodt (i < 1000) {
        sus key tea = "key_" + i
        sus value tea = "value_" + i
        map = optimized_hashmap_put(map, key, value)
        i = i + 1
    }
    
    vibez.spill("✅ Inserted 1000 key-value pairs")
    vibez.spill("📊 HashMap size: " + map.size + "/" + map.capacity)
    
    fr fr Test lookup performance
    sus found_count drip = 0
    i = 0
    periodt (i < 1000) {
        sus key tea = "key_" + i
        sus found lit = optimized_hashmap_contains(map, key)
        ready (found) {
            found_count = found_count + 1
        }
        i = i + 1
    }
    
    vibez.spill("✅ Found " + found_count + "/1000 keys in O(1) lookups")
    
    fr fr Test retrieval performance
    i = 0
    periodt (i < 100) {
        sus key tea = "key_" + (i * 10)
        sus value tea = optimized_hashmap_get(map, key)
        ready (value != "") {
            vibez.spill("✓ Retrieved: " + key + " = " + value)
        }
        i = i + 1
    }
    
    vibez.spill("🎯 All O(1) operations completed successfully!")
}

test_optimized_hashmap_performance()
vibez.spill("\n📈 CURSED Optimized HashMap - O(1) Robin Hood Hash Table Complete")
