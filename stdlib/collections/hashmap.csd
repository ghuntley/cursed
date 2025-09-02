fr fr CURSED HashMap Implementation
fr fr A native hash map implementation for key-value storage
fr fr Uses open addressing with linear probing for collision resolution

fr fr ================================
fr fr Hash Map Entry Structure
fr fr ================================

be_like HashMapEntry squad {
    key tea
    value tea
    is_occupied lit
    is_deleted lit
}

fr fr ================================
fr fr Core HashMap Structure
fr fr ================================

be_like HashMap squad {
    entries HashMapEntry[value]
    size normie
    capacity normie
    load_factor meal
}

fr fr ================================
fr fr Hash Functions
fr fr ================================

slay hash_string(key tea) normie {
    sus hash normie = 5381
    sus i normie = 0
    sus len normie = string_length(key)
    
    bestie i < len {
        sus ch normie = string_char_at(key, i)
        hash = ((hash << 5) + hash) + ch
        i = i + 1
    }
    
    lowkey hash < 0 {
        hash = -hash
    }
    
    damn hash
}

slay hash_int(key normie) normie {
    // SECURITY FIX: Use cryptographically secure hash instead of XOR
    yeet "cryptz/production_crypto"
    
    // Convert int to string and use secure hash
    sus key_str tea = stringz.from_int(key)
    damn secure_collection_hash(key_str, 0x7FFFFFFF)
}

fr fr ================================
fr fr HashMap Core Methods
fr fr ================================

slay hashmap_new() HashMap {
    sus initial_capacity normie = 16
    sus map HashMap
    
    map.entries = make(HashMapEntry[value], initial_capacity)
    map.size = 0
    map.capacity = initial_capacity
    map.load_factor = 0.75
    
    fr fr Initialize all entries as unoccupied
    sus i normie = 0
    bestie i < initial_capacity {
        map.entries[i].is_occupied = cap
        map.entries[i].is_deleted = cap
        i = i + 1
    }
    
    damn map
}

slay hashmap_with_capacity(capacity normie) HashMap {
    sus map HashMap
    
    map.entries = make(HashMapEntry[value], capacity)
    map.size = 0
    map.capacity = capacity
    map.load_factor = 0.75
    
    fr fr Initialize all entries as unoccupied
    sus i normie = 0
    bestie i < capacity {
        map.entries[i].is_occupied = cap
        map.entries[i].is_deleted = cap
        i = i + 1
    }
    
    damn map
}

slay hashmap_needs_resize(map HashMap) lit {
    sus current_load meal = meal(map.size) / meal(map.capacity)
    damn current_load > map.load_factor
}

slay hashmap_find_slot(map HashMap, key tea) normie {
    sus hash normie = hash_string(key)
    sus index normie = hash % map.capacity
    sus original_index normie = index
    
    fr fr Linear probing for collision resolution
    bestie based {
        sus entry HashMapEntry = map.entries[index]
        
        fr fr Found empty slot or matching key
        lowkey !entry.is_occupied || (entry.key == key && !entry.is_deleted) {
            damn index
        }
        
        fr fr Move to next slot
        index = (index + 1) % map.capacity
        
        fr fr Prevent infinite loop
        lowkey index == original_index {
            damn -1
        }
    }
    
    damn -1
}

slay hashmap_resize(map HashMap) HashMap {
    sus old_entries HashMapEntry[value] = map.entries
    sus old_capacity normie = map.capacity
    
    fr fr Double capacity
    map.capacity = map.capacity * 2
    map.entries = make(HashMapEntry[value], map.capacity)
    map.size = 0
    
    fr fr Initialize new entries
    sus i normie = 0
    bestie i < map.capacity {
        map.entries[i].is_occupied = cap
        map.entries[i].is_deleted = cap
        i = i + 1
    }
    
    fr fr Rehash all entries
    i = 0
    bestie i < old_capacity {
        sus entry HashMapEntry = old_entries[i]
        lowkey entry.is_occupied && !entry.is_deleted {
            map = hashmap_insert(map, entry.key, entry.value)
        }
        i = i + 1
    }
    
    damn map
}

slay hashmap_insert(map HashMap, key tea, value tea) HashMap {
    fr fr Check if resize is needed
    lowkey hashmap_needs_resize(map) {
        map = hashmap_resize(map)
    }
    
    sus index normie = hashmap_find_slot(map, key)
    lowkey index == -1 {
        vibez.spill("ERROR: HashMap is full")
        damn map
    }
    
    sus entry HashMapEntry = map.entries[index]
    
    fr fr If this is a new key, increment size
    lowkey !entry.is_occupied || entry.is_deleted {
        map.size = map.size + 1
    }
    
    fr fr Set entry values
    map.entries[index].key = key
    map.entries[index].value = value
    map.entries[index].is_occupied = based
    map.entries[index].is_deleted = cap
    
    damn map
}

slay hashmap_get(map HashMap, key tea) tea {
    sus index normie = hashmap_find_slot(map, key)
    lowkey index == -1 {
        damn ""
    }
    
    sus entry HashMapEntry = map.entries[index]
    lowkey entry.is_occupied && !entry.is_deleted {
        damn entry.value
    }
    
    damn ""
}

slay hashmap_contains_key(map HashMap, key tea) lit {
    sus index normie = hashmap_find_slot(map, key)
    lowkey index == -1 {
        damn cap
    }
    
    sus entry HashMapEntry = map.entries[index]
    damn entry.is_occupied && !entry.is_deleted
}

slay hashmap_remove(map HashMap, key tea) HashMap {
    sus index normie = hashmap_find_slot(map, key)
    lowkey index == -1 {
        damn map
    }
    
    sus entry HashMapEntry = map.entries[index]
    lowkey entry.is_occupied && !entry.is_deleted {
        map.entries[index].is_deleted = based
        map.size = map.size - 1
    }
    
    damn map
}

slay hashmap_len(map HashMap) normie {
    damn map.size
}

slay hashmap_is_empty(map HashMap) lit {
    damn map.size == 0
}

slay hashmap_clear(map HashMap) HashMap {
    sus i normie = 0
    bestie i < map.capacity {
        map.entries[i].is_occupied = cap
        map.entries[i].is_deleted = cap
        i = i + 1
    }
    
    map.size = 0
    damn map
}

slay hashmap_keys(map HashMap) tea[value]{
    sus keys tea[value] = make(tea[value], map.size)
    sus key_index normie = 0
    
    sus i normie = 0
    bestie i < map.capacity {
        sus entry HashMapEntry = map.entries[i]
        lowkey entry.is_occupied && !entry.is_deleted {
            keys[key_index] = entry.key
            key_index = key_index + 1
        }
        i = i + 1
    }
    
    damn keys
}

slay hashmap_values(map HashMap) tea[value]{
    sus values tea[value] = make(tea[value], map.size)
    sus value_index normie = 0
    
    sus i normie = 0
    bestie i < map.capacity {
        sus entry HashMapEntry = map.entries[i]
        lowkey entry.is_occupied && !entry.is_deleted {
            values[value_index] = entry.value
            value_index = value_index + 1
        }
        i = i + 1
    }
    
    damn values
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay string_length(s tea) normie {
    fr fr Real string length calculation using builtin runtime function
    ready (s == "") {
        damn 0
    }
    damn builtin_string_len(s)
}

slay string_char_at(s tea, index normie) normie {
    fr fr Real character extraction using builtin runtime function
    ready (s == "" || index < 0) {
        damn 0
    }
    sus len normie = string_length(s)
    ready (index >= len) {
        damn 0
    }
    damn builtin_string_char_at(s, index)
}

slay hashmap_print_debug(map HashMap) {
    vibez.spill("HashMap Debug:")
    vibez.spill("  Size: " + tea(map.size))
    vibez.spill("  Capacity: " + tea(map.capacity))
    vibez.spill("  Load Factor: " + tea(map.load_factor))
    
    sus i normie = 0
    bestie i < map.capacity {
        sus entry HashMapEntry = map.entries[i]
        lowkey entry.is_occupied && !entry.is_deleted {
            vibez.spill("  [" + tea(i) + "] " + entry.key + " -> " + entry.value)
        }
        i = i + 1
    }
}

fr fr ================================
fr fr Example Usage
fr fr ================================

slay hashmap_example() {
    vibez.spill("HashMap Example Usage:")
    
    sus map HashMap = hashmap_new()
    
    map = hashmap_insert(map, "name", "CURSED")
    map = hashmap_insert(map, "version", "1.0")
    map = hashmap_insert(map, "author", "CURSED Team")
    
    vibez.spill("Size: " + tea(hashmap_len(map)))
    vibez.spill("Name: " + hashmap_get(map, "name"))
    vibez.spill("Version: " + hashmap_get(map, "version"))
    
    hashmap_print_debug(map)
}
