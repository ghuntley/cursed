yeet "stdlib/collections/hashmap.csd"

slay main() {
    vibez.spill("Testing CURSED HashMap Implementation")
    
    fr fr Create a new HashMap
    sus map HashMap = hashmap_new()
    
    fr fr Test basic operations
    map = hashmap_insert(map, "name", "CURSED")
    map = hashmap_insert(map, "version", "1.0")
    map = hashmap_insert(map, "type", "Language")
    
    fr fr Test retrieval
    vibez.spill("Name: " + hashmap_get(map, "name"))
    vibez.spill("Version: " + hashmap_get(map, "version"))
    vibez.spill("Type: " + hashmap_get(map, "type"))
    
    fr fr Test size
    vibez.spill("Size: " + tea(hashmap_len(map)))
    
    fr fr Test contains
    lowkey hashmap_contains_key(map, "name") {
        vibez.spill("Found key: name")
    }
    
    lowkey !hashmap_contains_key(map, "nonexistent") {
        vibez.spill("Key 'nonexistent' not found (correct)")
    }
    
    fr fr Test remove
    map = hashmap_remove(map, "version")
    vibez.spill("After removing version, size: " + tea(hashmap_len(map)))
    
    fr fr Test clear
    map = hashmap_clear(map)
    vibez.spill("After clear, size: " + tea(hashmap_len(map)))
    vibez.spill("Is empty: " + tea(hashmap_is_empty(map)))
    
    vibez.spill("HashMap test completed successfully!")
}

main()
