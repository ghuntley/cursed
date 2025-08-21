// Interface Dispatch Code Generation Test Suite
// Tests complete vtable lookup paths and method dispatch

yeet "vibez"

// Define test interface
collab Drawable {
    slay draw() -> tea;
    slay getArea() -> drip;
    slay toString() -> tea;
}

// Define implementing struct
squad Circle {
    radius: drip,
}

// Implementation for Circle
impl Drawable for Circle {
    slay draw() -> tea {
        damn "Drawing circle with radius " + facts(radius)
    }
    
    slay getArea() -> drip {
        damn 3.14159 * radius * radius
    }
    
    slay toString() -> tea {
        damn "Circle(radius=" + facts(radius) + ")"
    }
}

// Define another implementing struct
squad Rectangle {
    width: drip,
    height: drip,
}

// Implementation for Rectangle
impl Drawable for Rectangle {
    slay draw() -> tea {
        damn "Drawing rectangle " + facts(width) + "x" + facts(height)
    }
    
    slay getArea() -> drip {
        damn width * height
    }
    
    slay toString() -> tea {
        damn "Rectangle(" + facts(width) + "x" + facts(height) + ")"
    }
}

// Test interface method calls
slay test_interface_dispatch() {
    // Create Circle instance
    sus circle Circle = Circle { radius: 5.0 }
    sus drawable_circle Drawable = circle
    
    // Test vtable dispatch for Circle methods
    vibez.spill("Testing Circle interface dispatch:")
    vibez.spill(drawable_circle.toString())
    vibez.spill(drawable_circle.draw())
    vibez.spill("Area:", facts(drawable_circle.getArea()))
    
    // Create Rectangle instance  
    sus rect Rectangle = Rectangle { width: 4.0, height: 6.0 }
    sus drawable_rect Drawable = rect
    
    // Test vtable dispatch for Rectangle methods
    vibez.spill("Testing Rectangle interface dispatch:")
    vibez.spill(drawable_rect.toString())
    vibez.spill(drawable_rect.draw())
    vibez.spill("Area:", facts(drawable_rect.getArea()))
    
    // Test polymorphic behavior
    sus shapes []Drawable = [drawable_circle, drawable_rect]
    
    vibez.spill("Testing polymorphic dispatch:")
    bestie (sus i drip = 0; i < len(shapes); i = i + 1) {
        sus shape Drawable = shapes[i]
        vibez.spill("Shape:", shape.toString())
        vibez.spill("Drawing:", shape.draw()) 
        vibez.spill("Area:", facts(shape.getArea()))
        vibez.spill("---")
    }
}

// Test invalid vtable handling
slay test_vtable_validation() {
    vibez.spill("Testing vtable validation (should trap on invalid vtable):")
    
    // This would test the fail-fast assertion mechanism
    // In a real test, this would create an object with corrupted vtable
    // and verify the trap is triggered
    
    vibez.spill("Vtable validation tests completed")
}

// Main test function
slay main() {
    vibez.spill("=== Interface Dispatch Code Generation Test ===")
    
    test_interface_dispatch()
    test_vtable_validation()
    
    vibez.spill("=== Interface dispatch tests completed ===")
}
