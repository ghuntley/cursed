# Complete interface method dispatch test
collab Drawable {
    slay draw()
    slay area() normie
    slay perimeter() normie
}

collab Named {
    slay get_name() tea
    slay set_name(new_name tea)
}

squad Circle {
    spill radius normie
    spill name tea
    
    slay draw() {
        vibez.spill("🔵 Drawing circle:", this.name, "radius =", this.radius)
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
    
    slay set_name(new_name tea) {
        vibez.spill("Setting circle name to:", new_name)
    }
}

squad Rectangle {
    spill width normie
    spill height normie
    spill name tea
    
    slay draw() {
        vibez.spill("⬜ Drawing rectangle:", this.name, "size:", this.width, "x", this.height)
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
    
    slay set_name(new_name tea) {
        vibez.spill("Setting rectangle name to:", new_name)
    }
}

# Polymorphic functions
slay describe_shape(shape Drawable) {
    shape.draw()
    vibez.spill("  📏 Area:", shape.area())
    vibez.spill("  📐 Perimeter:", shape.perimeter())
}

slay print_name(obj Named) {
    vibez.spill("  🏷️  Name:", obj.get_name())
}

slay modify_name(obj Named, new_name tea) {
    obj.set_name(new_name)
}

# Create test instances
sus circle Circle = Circle{radius: 5.0, name: "BigCircle"}
sus rect Rectangle = Rectangle{width: 4.0, height: 3.0, name: "SmallRect"}

vibez.spill("=== 🧪 Testing Direct Method Calls ===")
circle.draw()
vibez.spill("Circle area:", circle.area())
vibez.spill("Circle perimeter:", circle.perimeter())
vibez.spill("Circle name:", circle.get_name())

vibez.spill()
rect.draw()
vibez.spill("Rectangle area:", rect.area())
vibez.spill("Rectangle perimeter:", rect.perimeter())
vibez.spill("Rectangle name:", rect.get_name())

vibez.spill()
vibez.spill("=== 🔄 Testing Polymorphic Interface Calls ===")
describe_shape(circle)
print_name(circle)

vibez.spill()
describe_shape(rect)
print_name(rect)

vibez.spill()
vibez.spill("=== ✏️ Testing Method Calls with Parameters ===")
modify_name(circle, "RenamedCircle")
modify_name(rect, "RenamedRectangle")

vibez.spill()
vibez.spill("=== ✅ Interface Method Dispatch Complete! ===")
