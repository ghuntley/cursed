# Test direct struct field access
vibez.spill("=== Direct Struct Field Access Test ===")

# Define struct
squad Person {
    spill name tea
    spill age drip
}

# Create struct instance
sus p Person = Person{name: "Alice", age: 30}
vibez.spill("Created person:", p)

# Test direct field access in print statements  
vibez.spill("Direct name access:", p.name)
vibez.spill("Direct age access:", p.age)

# Test field access in expressions
ready (p.age > 25) {
    vibez.spill("Person is older than 25")
} otherwise {
    vibez.spill("Person is 25 or younger")
}

vibez.spill("=== Test Complete ===")
