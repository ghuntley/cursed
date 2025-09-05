fr fr Test all newly migrated stdlib modules

yeet "vibez"
yeet "collections" 
yeet "json"
yeet "regex"
yeet "memory"

main_character() {
    vibez.spill("=== Testing All New Stdlib Modules ===")
    vibez.spill("")
    
    fr fr Test collections module
    vibez.spill("1. Testing collections module:")
    sus vec = collections.Vec_new()
    vec = collections.Vec_push(vec, 42)
    vec = collections.Vec_push(vec, 99)
    sus len = collections.Vec_len(vec)
    sus first = collections.Vec_get(vec, 0)
    vibez.spill("  Vector length: " + len.(tea))
    vibez.spill("  First element: " + first.(tea))
    vibez.spill("")
    
    fr fr Test json module
    vibez.spill("2. Testing json module:")
    sus json_text = "{\"name\":\"CURSED\",\"version\":1}"
    sus is_valid = json.validate(json_text)
    vibez.spill("  JSON validation: " + is_valid.(tea))
    sus parsed = json.parse(json_text)
    vibez.spill("  Parsed JSON: " + parsed.(tea))
    vibez.spill("")
    
    fr fr Test regex module
    vibez.spill("3. Testing regex module:")
    sus text = "The number is 123 and another is 456"
    sus match = regex.find("\\d+", text)
    vibez.spill("  First number found: " + match.(tea))
    sus replaced = regex.replace("\\d+", text, "XXX")
    vibez.spill("  With replacement: " + replaced.(tea))
    vibez.spill("")
    
    fr fr Test memory module
    vibez.spill("4. Testing memory module:")
    sus addr = memory.malloc(1024)
    vibez.spill("  Allocated memory at: " + addr.(tea))
    memory.memset(addr, 0, 1024)
    vibez.spill("  Memory cleared successfully")
    memory.free(addr)
    vibez.spill("  Memory freed successfully")
    vibez.spill("")
    
    vibez.spill("=== All modules tested successfully! ===")
    
    no_cap 0
}
