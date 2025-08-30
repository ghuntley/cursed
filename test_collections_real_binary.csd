yeet "vibez"
yeet "collections"

slay main_character() {
    vibez.spill("=== Collections Real Test ===")
    
    fr fr Create and populate vector
    sus vec drip = collections.Vec_new()
    vibez.spill("Created new vector")
    
    vec = collections.Vec_push(vec, 42)
    vibez.spill("Pushed 42 to vector")
    
    vec = collections.Vec_push(vec, 99)
    vibez.spill("Pushed 99 to vector")
    
    sus len drip = collections.Vec_len(vec)
    vibez.spill("Vector length: " + len.(tea))
    
    sus first drip = collections.Vec_get(vec, 0)
    vibez.spill("Element at index 0: " + first.(tea))
    
    sus second drip = collections.Vec_get(vec, 1)
    vibez.spill("Element at index 1: " + second.(tea))
    
    fr fr Test hashmap
    sus map drip = collections.Map_new()
    map = collections.Map_insert(map, "key1", 123)
    sus value drip = collections.Map_get(map, "key1")
    vibez.spill("Map value for 'key1': " + value.(tea))
    
    sus map_len drip = collections.Map_len(map)
    vibez.spill("Map size: " + map_len.(tea))
    
    vibez.spill("=== Collections test completed ===")
}
