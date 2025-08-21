# Test struct field access with verbose output
vibez.spill("=== Debug Struct Field Access ===")

# Define struct
squad Person {
    spill name tea
    spill age drip  
}

# Create struct instance with explicit verbose  
sus p Person = Person{name: "Alice", age: 30}
vibez.spill("Created person struct")

# Test simple field access
sus name_val = p.name  
vibez.spill("Accessed p.name:", name_val)

sus age_val = p.age
vibez.spill("Accessed p.age:", age_val)

vibez.spill("=== End Debug Test ===")
