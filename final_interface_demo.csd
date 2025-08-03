fr fr CURSED Interface System - Final Demonstration
fr fr This demonstrates all implemented interface features working together

fr fr 1. Basic Interface Definition
collab Drawable {
    slay draw()
    slay get_area() normie
}

fr fr 2. Generic Interface
collab Container<T> {
    slay add(item T)
    slay get(index normie) T
    slay size() normie
}

fr fr 3. Interface Inheritance
collab Shape extends Drawable {
    slay get_perimeter() normie
}

fr fr 4. Interface Composition  
collab Widget extends Drawable with Clickable {
    slay render()
}

collab Clickable {
    slay on_click()
}

fr fr 5. Struct Definitions
squad Circle {
    spill radius normie
}

squad Rectangle {
    spill width normie
    spill height normie
}

fr fr 6. Interface Implementations
impl Circle for Drawable {
    slay draw() {
        vibez.spill("Drawing circle")
    }
    
    slay get_area() normie {
        damn 78  fr fr π * r²
    }
}

impl Circle for Shape {
    slay get_perimeter() normie {
        damn 31  fr fr 2 * π * r
    }
}

impl Rectangle for Drawable {
    slay draw() {
        vibez.spill("Drawing rectangle")
    }
    
    slay get_area() normie {
        damn 60  fr fr width * height
    }
}

fr fr 7. Generic Interface Implementation
squad IntVector {
    spill data [100]normie
    spill count normie
}

impl IntVector for Container<normie> {
    slay add(item normie) {
        self.data[self.count] = item
        self.count = self.count + 1
    }
    
    slay get(index normie) normie {
        damn self.data[index]
    }
    
    slay size() normie {
        damn self.count
    }
}

fr fr 8. Interface Usage and Virtual Dispatch
slay demo_interfaces() {
    vibez.spill("=== Interface System Demo ===")
    
    fr fr Create objects
    circle := Circle { radius: 5 }
    rect := Rectangle { width: 10, height: 6 }
    
    fr fr Interface casting
    sus drawable1 Drawable = circle
    sus drawable2 Drawable = rect
    
    fr fr Virtual method dispatch
    drawable1.draw()  fr fr Calls Circle.draw()
    drawable2.draw()  fr fr Calls Rectangle.draw()
    
    fr fr Interface inheritance
    sus shape Shape = circle
    shape.draw()      fr fr Works via extends Drawable
    perimeter := shape.get_perimeter()
    
    fr fr Generic interfaces
    vector := IntVector { data: [0; 100], count: 0 }
    sus container Container<normie> = vector
    
    container.add(42)
    container.add(84)
    first := container.get(0)
    
    vibez.spill("=== Demo Complete ===")
}

fr fr Main entry point
slay main() {
    demo_interfaces()
    damn 0
}
