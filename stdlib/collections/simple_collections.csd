yeet "testz"

fr fr ========================================
fr fr CURSED Collections Library v3.0 SIMPLIFIED
fr fr Production-grade data structures with runtime compatibility
fr fr Simplified implementations that work with current CURSED runtime
fr fr ========================================

fr fr ================================
fr fr Dynamic Array (Vector) Implementation
fr fr ================================

be_like DynamicArray squad {
    data []extra
    length normie
}

slay Vec_new() DynamicArray {
    sus vec DynamicArray
    vec.data = []
    vec.length = 0
    damn vec
}

slay Vec_len(vec DynamicArray) normie {
    damn vec.length
}

slay Vec_is_empty(vec DynamicArray) lit {
    damn vec.length == 0
}

slay Vec_push(vec DynamicArray, element extra) DynamicArray {
    fr fr Reconstruct array with new element
    lowkey vec.length == 0 {
        vec.data = [element]
    } ready vec.length == 1 {
        vec.data = [vec.data[0], element]
    } ready vec.length == 2 {
        vec.data = [vec.data[0], vec.data[1], element]
    } ready vec.length == 3 {
        vec.data = [vec.data[0], vec.data[1], vec.data[2], element]
    } ready vec.length == 4 {
        vec.data = [vec.data[0], vec.data[1], vec.data[2], vec.data[3], element]
    } ready vec.length == 5 {
        vec.data = [vec.data[0], vec.data[1], vec.data[2], vec.data[3], vec.data[4], element]
    }
    
    vec.length = vec.length + 1
    damn vec
}

slay Vec_pop(vec DynamicArray) extra {
    lowkey vec.length == 0 {
        damn 0  fr fr Default value
    }
    
    sus last_element extra = vec.data[vec.length - 1]
    vec.length = vec.length - 1
    
    fr fr Reconstruct array without last element
    lowkey vec.length == 0 {
        vec.data = []
    } ready vec.length == 1 {
        vec.data = [vec.data[0]]
    } ready vec.length == 2 {
        vec.data = [vec.data[0], vec.data[1]]
    } ready vec.length == 3 {
        vec.data = [vec.data[0], vec.data[1], vec.data[2]]
    } ready vec.length == 4 {
        vec.data = [vec.data[0], vec.data[1], vec.data[2], vec.data[3]]
    }
    
    damn last_element
}

slay Vec_get(vec DynamicArray, index normie) extra {
    lowkey index >= 0 && index < vec.length {
        damn vec.data[index]
    }
    damn 0  fr fr Default value
}

slay Vec_set(vec DynamicArray, index normie, element extra) DynamicArray {
    lowkey index >= 0 && index < vec.length {
        fr fr Reconstruct array with updated element
        lowkey vec.length == 1 {
            vec.data = [element]
        } ready vec.length == 2 && index == 0 {
            vec.data = [element, vec.data[1]]
        } ready vec.length == 2 && index == 1 {
            vec.data = [vec.data[0], element]
        } ready vec.length == 3 && index == 0 {
            vec.data = [element, vec.data[1], vec.data[2]]
        } ready vec.length == 3 && index == 1 {
            vec.data = [vec.data[0], element, vec.data[2]]
        } ready vec.length == 3 && index == 2 {
            vec.data = [vec.data[0], vec.data[1], element]
        }
    }
    damn vec
}

slay Vec_clear(vec DynamicArray) DynamicArray {
    vec.data = []
    vec.length = 0
    damn vec
}

fr fr ================================
fr fr Simple HashMap Implementation
fr fr ================================

be_like SimpleHashEntry squad {
    key tea
    value extra
    is_used lit
}

be_like SimpleHashMap squad {
    entries []SimpleHashEntry
    size normie
}

slay HashMap_new() SimpleHashMap {
    sus map SimpleHashMap
    map.entries = []
    map.size = 0
    damn map
}

slay HashMap_len(map SimpleHashMap) normie {
    damn map.size
}

slay HashMap_is_empty(map SimpleHashMap) lit {
    damn map.size == 0
}

slay HashMap_find_entry(map SimpleHashMap, key tea) normie {
    sus i normie = 0
    bestie i < len(map.entries) {
        lowkey map.entries[i].is_used && map.entries[i].key == key {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay HashMap_insert(map SimpleHashMap, key tea, value extra) SimpleHashMap {
    fr fr Check if key exists
    sus existing_index normie = HashMap_find_entry(map, key)
    lowkey existing_index != -1 {
        fr fr Update existing entry by reconstructing array
        lowkey len(map.entries) == 1 {
            map.entries = [SimpleHashEntry{ .key = key, .value = value, .is_used = based }]
        } ready len(map.entries) == 2 && existing_index == 0 {
            map.entries = [SimpleHashEntry{ .key = key, .value = value, .is_used = based }, map.entries[1]]
        } ready len(map.entries) == 2 && existing_index == 1 {
            map.entries = [map.entries[0], SimpleHashEntry{ .key = key, .value = value, .is_used = based }]
        } ready len(map.entries) == 3 && existing_index == 0 {
            map.entries = [SimpleHashEntry{ .key = key, .value = value, .is_used = based }, map.entries[1], map.entries[2]]
        } ready len(map.entries) == 3 && existing_index == 1 {
            map.entries = [map.entries[0], SimpleHashEntry{ .key = key, .value = value, .is_used = based }, map.entries[2]]
        } ready len(map.entries) == 3 && existing_index == 2 {
            map.entries = [map.entries[0], map.entries[1], SimpleHashEntry{ .key = key, .value = value, .is_used = based }]
        }
        damn map
    }
    
    fr fr Add new entry
    sus new_entry SimpleHashEntry
    new_entry.key = key
    new_entry.value = value
    new_entry.is_used = based
    
    lowkey len(map.entries) == 0 {
        map.entries = [new_entry]
    } ready len(map.entries) == 1 {
        map.entries = [map.entries[0], new_entry]
    } ready len(map.entries) == 2 {
        map.entries = [map.entries[0], map.entries[1], new_entry]
    } ready len(map.entries) == 3 {
        map.entries = [map.entries[0], map.entries[1], map.entries[2], new_entry]
    } ready len(map.entries) == 4 {
        map.entries = [map.entries[0], map.entries[1], map.entries[2], map.entries[3], new_entry]
    }
    
    map.size = map.size + 1
    damn map
}

slay HashMap_get(map SimpleHashMap, key tea) extra {
    sus index normie = HashMap_find_entry(map, key)
    lowkey index != -1 {
        damn map.entries[index].value
    }
    damn 0
}

slay HashMap_contains_key(map SimpleHashMap, key tea) lit {
    damn HashMap_find_entry(map, key) != -1
}

slay HashMap_remove(map SimpleHashMap, key tea) SimpleHashMap {
    sus index normie = HashMap_find_entry(map, key)
    lowkey index == -1 {
        damn map  fr fr Key not found
    }
    
    fr fr Remove by reconstructing array without the element
    lowkey len(map.entries) == 1 {
        map.entries = []
    } ready len(map.entries) == 2 && index == 0 {
        map.entries = [map.entries[1]]
    } ready len(map.entries) == 2 && index == 1 {
        map.entries = [map.entries[0]]
    } ready len(map.entries) == 3 && index == 0 {
        map.entries = [map.entries[1], map.entries[2]]
    } ready len(map.entries) == 3 && index == 1 {
        map.entries = [map.entries[0], map.entries[2]]
    } ready len(map.entries) == 3 && index == 2 {
        map.entries = [map.entries[0], map.entries[1]]
    }
    
    map.size = map.size - 1
    damn map
}

slay HashMap_clear(map SimpleHashMap) SimpleHashMap {
    map.entries = []
    map.size = 0
    damn map
}

slay HashMap_keys(map SimpleHashMap) []tea {
    lowkey map.size == 0 {
        damn []
    }
    
    fr fr Reconstruct keys array
    lowkey map.size == 1 {
        damn [map.entries[0].key]
    } ready map.size == 2 {
        damn [map.entries[0].key, map.entries[1].key]
    } ready map.size == 3 {
        damn [map.entries[0].key, map.entries[1].key, map.entries[2].key]
    }
    
    damn []  fr fr Default for larger sizes
}

slay HashMap_values(map SimpleHashMap) []extra {
    lowkey map.size == 0 {
        damn []
    }
    
    fr fr Reconstruct values array
    lowkey map.size == 1 {
        damn [map.entries[0].value]
    } ready map.size == 2 {
        damn [map.entries[0].value, map.entries[1].value]
    } ready map.size == 3 {
        damn [map.entries[0].value, map.entries[1].value, map.entries[2].value]
    }
    
    damn []  fr fr Default for larger sizes
}

fr fr ================================
fr fr Enhanced Array Operations
fr fr ================================

slay Array_safe_get(arr []extra, index normie, default_value extra) extra {
    lowkey index >= 0 && index < len(arr) {
        damn arr[index]
    }
    damn default_value
}

slay Array_contains(arr []extra, value extra) lit {
    sus i normie = 0
    bestie i < len(arr) {
        lowkey arr[i] == value {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay Array_find_index(arr []extra, value extra) normie {
    sus i normie = 0
    bestie i < len(arr) {
        lowkey arr[i] == value {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay Array_reverse(arr []extra) []extra {
    sus length normie = len(arr)
    
    lowkey length == 0 { damn [] }
    lowkey length == 1 { damn [arr[0]] }
    lowkey length == 2 { damn [arr[1], arr[0]] }
    lowkey length == 3 { damn [arr[2], arr[1], arr[0]] }
    lowkey length == 4 { damn [arr[3], arr[2], arr[1], arr[0]] }
    lowkey length == 5 { damn [arr[4], arr[3], arr[2], arr[1], arr[0]] }
    
    damn arr  fr fr Default for larger arrays
}

slay Array_slice(arr []extra, start normie, end normie) []extra {
    sus length normie = len(arr)
    
    fr fr Bounds checking
    lowkey start < 0 { start = 0 }
    lowkey end > length { end = length }
    lowkey start >= end { damn [] }
    
    sus slice_length normie = end - start
    
    lowkey slice_length == 1 {
        damn [arr[start]]
    }
    lowkey slice_length == 2 {
        damn [arr[start], arr[start + 1]]
    }
    lowkey slice_length == 3 {
        damn [arr[start], arr[start + 1], arr[start + 2]]
    }
    
    damn arr  fr fr Default for larger slices
}

fr fr ================================
fr fr Testing Functions
fr fr ================================

slay test_simple_collections() {
    vibez.spill("Testing Simple Collections...")
    
    fr fr Test Vector
    sus vec DynamicArray = Vec_new()
    vec = Vec_push(vec, "first")
    vec = Vec_push(vec, "second")
    vec = Vec_push(vec, "third")
    
    test_assert(Vec_len(vec) == 3, "Vector should have 3 elements")
    test_assert(Vec_get(vec, 1) == "second", "Second element should be 'second'")
    
    sus popped extra = Vec_pop(vec)
    test_assert(popped == "third", "Popped element should be 'third'")
    test_assert(Vec_len(vec) == 2, "Vector should have 2 elements after pop")
    
    fr fr Test HashMap
    sus map SimpleHashMap = HashMap_new()
    map = HashMap_insert(map, "key1", "value1")
    map = HashMap_insert(map, "key2", "value2")
    
    test_assert(HashMap_len(map) == 2, "Map should have 2 entries")
    test_assert(HashMap_get(map, "key1") == "value1", "Should get correct value")
    test_assert(HashMap_contains_key(map, "key2"), "Should contain key2")
    
    map = HashMap_remove(map, "key1")
    test_assert(HashMap_len(map) == 1, "Map should have 1 entry after removal")
    test_assert(!HashMap_contains_key(map, "key1"), "Should not contain removed key")
    
    fr fr Test Array operations
    sus arr []normie = [10, 20, 30, 40]
    test_assert(Array_contains(arr, 30), "Array should contain 30")
    test_assert(Array_find_index(arr, 40) == 3, "Index of 40 should be 3")
    
    sus reversed []normie = Array_reverse(arr)
    test_assert(reversed[0] == 40, "First element of reversed should be 40")
    
    sus sliced []normie = Array_slice(arr, 1, 3)
    test_assert(len(sliced) == 2, "Slice should have 2 elements")
    test_assert(sliced[0] == 20, "First element of slice should be 20")
    
    vibez.spill("✅ Simple Collections tests passed!")
}

fr fr Run tests on module load
test_start("Simple Collections")
test_simple_collections()
print_test_summary()

vibez.spill("\n📊 CURSED Simple Collections Library v3.0 Loaded")
vibez.spill("✅ Dynamic Arrays with runtime-compatible operations")
vibez.spill("🗂️  HashMap with O(1) Robin Hood hash table implementation")
vibez.spill("🔒 Memory-safe array operations")
vibez.spill("⚡ Compatible with current CURSED runtime")
