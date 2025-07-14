# Test interface inheritance hierarchies

# Base interface
collab Shape {
    slay area() meal
    slay perimeter() meal
}

# Interface extending Shape
collab Drawable : Shape {
    slay draw()
    slay set_color(color tea)
}

# Interface extending multiple interfaces
collab Interactive : Drawable, Clickable {
    slay handle_click(x normie, y normie)
    slay is_clickable() lit
}

# Generic interface with inheritance
collab Collection[T] : Iterable[T] {
    slay add(item T)
    slay remove(item T) lit
    slay contains(item T) lit
}

vibez.spill("Interface inheritance test complete!")
