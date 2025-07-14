# Test generic interfaces, inheritance, and method receivers

# Generic interface with type parameters
collab Comparable[T] {
    slay compare(other T) normie
    slay equals(other T) lit
}

# Interface inheritance 
collab Sortable : Comparable[T] {
    slay sort() 
}

# Generic interface with bounds
collab Container[T: Clone + Debug] {
    slay add(item T)
    slay get(index normie) T
    slay size() normie
}

# Method implementation with receiver
squad Person {
    name tea
    age normie
}

# Method with value receiver
slay (p Person) greet() {
    vibez.spill("Hello, I'm " + p.name)
}

# Method with pointer receiver  
slay (*p Person) set_age(new_age normie) {
    p.age = new_age
}

# Interface with multiple inheritance
collab Drawable : Shape, Colored {
    slay draw()
}

# Test interface compliance
vibez.spill("Interface generics and inheritance test complete!")
