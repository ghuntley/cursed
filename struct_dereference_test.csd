# Test struct field dereferencing with pointers
vibez.spill("=== Struct Dereferencing Test ===")

# Basic struct definition
squad Point {
    spill x drip
    spill y drip
}

# Create a struct
sus p Point = Point{x: 10, y: 20}
vibez.spill("Original point:", p.x, p.y)

# Try pointer dereference syntax (this might be broken)
# *p should dereference the pointer to the struct
vibez.spill("Testing pointer dereference...")

# Address-of and dereference operations
sus ptr_p = &p  # Get pointer to p
vibez.spill("Got pointer to p")

# Try to dereference and access fields
# This is where dereferencing might be broken
sus deref_x = (*ptr_p).x
sus deref_y = (*ptr_p).y
vibez.spill("Dereferenced values:", deref_x, deref_y)

vibez.spill("=== Test complete ===")
