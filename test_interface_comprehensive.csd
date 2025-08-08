# Comprehensive interface method dispatch test
collab Drawable {
    slay draw()
    slay area() normie
    slay perimeter() normie
}

collab Named {
    slay get_name() tea
}

squad Circle {
    spill radius normie
    spill name tea
    
    slay draw() {
        vibez.spill("🔵 Drawing circle:", this.name, "with radius", this.radius)
    }
    
    slay area() normie {
        damn 3.14159 * this.radius * this.radius
    }
    
    slay perimeter() normie {
        damn 2.0 * 3.14159 * this.radius
    }
    
    slay get_name() tea {
        damn this.name
    }
}

squad Rectangle {
    spill width normie
    spill height normie
    spill name tea
    
    slay draw() {
        vibez.spill("⬜ Drawing rectangle:", this.name, this.width, "x", this.height)
    }
    
    slay area() normie {
        damn this.width * this.height
    }
    
    slay perimeter() normie {
        damn 2.0 * (this.width + this.height)
    }
    
    slay get_name() tea {
        damn this.name
    }
}

# Test polymorphic functions
slay describe_drawable(shape Drawable) {
    shape.draw()
    vibez.spill("  Area:", shape.area())
    vibez.spill("  Perimeter:", shape.perimeter())
}

slay show_name(obj Named) {
    vibez.spill("  Name:", obj.get_name())
}

# Create instances
sus circle Circle = Circle{radius: 5.0, name: "MyCircle"}
sus rect Rectangle = Rectangle{width: 4.0, height: 3.0, name: "MyRect"}

vibez.spill("=== Direct method calls ===")
circle.draw()
vibez.spill("Circle area:", circle.area())
vibez.spill("Circle perimeter:", circle.perimeter())

rect.draw()
vibez.spill("Rectangle area:", rect.area())
vibez.spill("Rectangle perimeter:", rect.perimeter())

vibez.spill("\n=== Polymorphic interface calls ===")
describe_drawable(circle)
describe_drawable(rect)

vibez.spill("\n=== Named interface calls ===")
show_name(circle)
show_name(rect)

vibez.spill("\n=== Mixed usage ===")
# Test that polymorphism preserves the actual type
sus shapes = [circle, rect]
# Note: This is conceptual - array of interfaces not fully implemented yet
