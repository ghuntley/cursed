# Simple struct test to identify issues
vibez.spill("Testing basic struct creation and field access")

# Simple struct definition  
squad Person {
    spill name tea
    spill age drip
}

# Create struct instance
sus p Person = Person{name: "Alice", age: 30}

# Test field access
vibez.spill("Person name:", p.name) 
vibez.spill("Person age:", p.age)

vibez.spill("Test complete")
