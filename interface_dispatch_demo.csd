fr fr Interface Dispatch System Demonstration

vibez.spill("=== CURSED Interface Dispatch Demo ===")

fr fr 1. Define an interface with the 'collab' keyword
collab Drawable {
    slay draw()
    slay area() normie
}

fr fr 2. Define a struct implementing the interface with 'squad' keyword
squad Circle {
    spill radius normie
    
    slay draw() {
        vibez.spill("Drawing a circle with radius:", radius)
    }
    
    slay area() normie {
        damn 3.14159 * radius * radius
    }
}

fr fr 3. Create an interface instance
sus shape Drawable = Circle{radius: 5}
vibez.spill("Created shape:", shape)

fr fr 4. Call interface methods (this should be dispatched)
vibez.spill("Calling shape.draw():")
shape.draw()

vibez.spill("Interface dispatch demonstration complete!")
