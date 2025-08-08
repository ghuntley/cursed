# Interface Method Dispatch Optimization Test
# Tests the enhanced vtable generation and method caching optimizations

yeet "testz"

# Define interface with multiple methods
collab Drawable {
    slay draw()
    slay area() drip
    slay perimeter() drip
}

# Define multiple structs implementing the interface
squad Rectangle {
    spill width drip
    spill height drip

    slay draw() {
        vibez.spill("Rectangle:", width, "x", height)
    }

    slay area() drip {
        damn width * height
    }

    slay perimeter() drip {
        damn 2 * (width + height)
    }
}

squad Circle {
    spill radius drip

    slay draw() {
        vibez.spill("Circle radius:", radius)
    }

    slay area() drip {
        damn 3 * radius * radius  # simplified pi * r^2
    }

    slay perimeter() drip {
        damn 6 * radius  # simplified 2 * pi * r
    }
}

squad Triangle {
    spill base drip
    spill height drip

    slay draw() {
        vibez.spill("Triangle base:", base, "height:", height)
    }

    slay area() drip {
        damn (base * height) / 2
    }

    slay perimeter() drip {
        damn base * 3  # simplified equilateral triangle
    }
}

# Test method dispatch optimization with multiple objects
test_start("Interface dispatch optimization test")

# Create multiple instances
sus rect Rectangle = Rectangle{width: 10, height: 5}
sus circle Circle = Circle{radius: 3}
sus triangle Triangle = Triangle{base: 4, height: 6}

# Test repeated method calls (should benefit from caching)
vibez.spill("=== Testing method call caching ===")

# Call same method multiple times on same object (cache hits)
rect.draw()
rect.draw()
rect.draw()

# Call different methods on same object (vtable optimization)
vibez.spill("Rectangle area:", rect.area())
vibez.spill("Rectangle perimeter:", rect.perimeter())

# Call same method on different objects (different vtables)
circle.draw()
triangle.draw()

# Verify areas are computed correctly
assert_eq_int(rect.area(), 50)
assert_eq_int(circle.area(), 27)  # 3 * 3 * 3 = 27
assert_eq_int(triangle.area(), 12)  # (4 * 6) / 2 = 12

# Verify perimeters
assert_eq_int(rect.perimeter(), 30)  # 2 * (10 + 5) = 30
assert_eq_int(circle.perimeter(), 18)  # 6 * 3 = 18
assert_eq_int(triangle.perimeter(), 12)  # 4 * 3 = 12

vibez.spill("=== All interface optimizations working ===")

print_test_summary()
