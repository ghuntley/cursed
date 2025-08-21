// CURSED Interface Dispatch Test
// Tests interface definition, implementation, and dynamic method dispatch

// Define Drawable interface
collab Drawable {
    slay draw() -> void
    slay get_area() -> normie
}

// Define Movable interface  
collab Movable {
    slay move(dx normie, dy normie) -> void
    slay get_position() -> (normie, normie)
}

// Rectangle struct implementing Drawable
squad Rectangle {
    width normie,
    height normie,
    x normie,
    y normie
}

// Implement Drawable for Rectangle
impl Drawable for Rectangle {
    slay draw() -> void {
        vibez.spill("Drawing rectangle {}x{} at ({}, {})", width, height, x, y)
    }
    
    slay get_area() -> normie {
        damn width * height
    }
}

// Implement Movable for Rectangle  
impl Movable for Rectangle {
    slay move(dx normie, dy normie) -> void {
        x = x + dx
        y = y + dy
    }
    
    slay get_position() -> (normie, normie) {
        damn (x, y)
    }
}

// Circle struct implementing both interfaces
squad Circle {
    radius normie,
    x normie, 
    y normie
}

impl Drawable for Circle {
    slay draw() -> void {
        vibez.spill("Drawing circle with radius {} at ({}, {})", radius, x, y)
    }
    
    slay get_area() -> normie {
        damn 3.14159 * radius * radius
    }
}

impl Movable for Circle {
    slay move(dx normie, dy normie) -> void {
        x = x + dx
        y = y + dy
    }
    
    slay get_position() -> (normie, normie) {
        damn (x, y)
    }
}

// Function that works with any Drawable object
slay draw_shape(shape Drawable) -> void {
    shape.draw()
    sus area normie = shape.get_area()
    vibez.spill("Area: {}", area)
}

// Function that works with any Movable object
slay move_shape(shape Movable, dx normie, dy normie) -> void {
    sus (old_x, old_y) = shape.get_position()
    vibez.spill("Moving from ({}, {}) by ({}, {})", old_x, old_y, dx, dy)
    shape.move(dx, dy)
    sus (new_x, new_y) = shape.get_position()
    vibez.spill("New position: ({}, {})", new_x, new_y)
}

// Function that works with objects implementing both interfaces
slay animate_shape(shape Drawable & Movable, dx normie, dy normie) -> void {
    vibez.spill("Before animation:")
    shape.draw()
    
    shape.move(dx, dy)
    
    vibez.spill("After animation:")
    shape.draw()
}

// Main function to test interface dispatch
slay main() -> void {
    vibez.spill("=== Interface Dispatch Test ===")
    
    // Create Rectangle instance
    sus rect Rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
        x: 0.0,
        y: 0.0
    }
    
    // Create Circle instance
    sus circle Circle = Circle {
        radius: 3.0,
        x: 2.0,
        y: 3.0
    }
    
    vibez.spill("\\n1. Testing direct method calls:")
    rect.draw()
    vibez.spill("Rectangle area: {}", rect.get_area())
    
    circle.draw()
    vibez.spill("Circle area: {}", circle.get_area())
    
    vibez.spill("\\n2. Testing interface method dispatch:")
    draw_shape(rect)
    draw_shape(circle)
    
    vibez.spill("\\n3. Testing movement interface:")
    move_shape(rect, 5.0, 3.0)
    move_shape(circle, -1.0, 2.0)
    
    vibez.spill("\\n4. Testing combined interface usage:")
    animate_shape(rect, 2.0, 1.0)
    animate_shape(circle, 1.0, -1.0)
    
    vibez.spill("\\n5. Testing polymorphic array:")
    sus shapes []Drawable = [rect, circle]
    bestie (shape in shapes) {
        shape.draw()
        sus area = shape.get_area()
        ready (area > 20.0) {
            vibez.spill("Large shape with area {}", area)
        } otherwise {
            vibez.spill("Small shape with area {}", area)
        }
    }
    
    vibez.spill("\\n=== Interface Dispatch Test Complete ===")
}
