yeet "testz"

// Simple interface test
collab Shape {
    slay area()
    slay perimeter()
}

// Simple struct
vibe Rectangle {
    width normie
    height normie
}

// Method implementations
slay Rectangle.area() normie {
    damn self.width * self.height
}

slay Rectangle.perimeter() normie {
    damn 2 * (self.width + self.height)
}

test_start("Simple Interface Test")

// Create rectangle
sus rect Rectangle = Rectangle { width: 5, height: 3 }

// Test method calls
sus area normie = rect.area()
assert_eq_int(area, 15)

print_test_summary()
