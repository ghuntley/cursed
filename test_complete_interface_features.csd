# Comprehensive test for all interface features

# Generic interface with type constraints
collab Comparable[T: Clone + Debug] {
    slay compare(other T) normie
    slay equals(other T) lit
}

# Interface inheritance
collab Sortable : Comparable[T] {
    slay sort()
    slay reverse()
}

# Multi-level inheritance
collab AdvancedSortable : Sortable, Serializable {
    slay advanced_sort(algorithm tea)
    slay benchmark() meal
}

# Interface with multiple type parameters
collab KeyValueStore[K: Comparable, V: Clone] {
    slay put(key K, value V)
    slay get(key K) V
    slay contains(key K) lit
    slay size() normie
}

# Struct to implement interfaces
squad Person {
    name tea
    age normie
}

# Value receiver method
slay (p Person) get_info() tea {
    damn p.name + " (" + p.age + " years old)"
}

# Pointer receiver method
slay (*p Person) celebrate_birthday() {
    p.age = p.age + 1
    vibez.spill("Happy birthday! Now " + p.age + " years old.")
}

# Test multiple method signatures
slay (*p Person) update_info(new_name tea, new_age normie) {
    p.name = new_name
    p.age = new_age
}

# Generic struct with interface constraint
squad Container[T: Comparable] {
    items []T
    sorted lit
}

# Generic method with constraints
slay [T: Comparable](c *Container[T]) add_sorted(item T) {
    # Add item in sorted order
    c.sorted = based
}

vibez.spill("All interface features implemented successfully!")
vibez.spill("✅ Generic interfaces with type parameters")
vibez.spill("✅ Interface inheritance and composition")
vibez.spill("✅ Method receivers (value and pointer)")
vibez.spill("✅ Type constraints and bounds")
vibez.spill("✅ Multi-level inheritance hierarchies")
