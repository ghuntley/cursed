// Advanced interface test for CURSED language
yeet "testz"

// Interface inheritance
collab Shape {
    slay area() meal
}

collab ColoredShape extends Shape {
    slay color() tea
    slay set_color(color tea) cap
}

// Interface with default implementation
collab Printable {
    slay print() tea {
        damn "Default print implementation"
    }
    
    slay format() tea  // No default implementation
}

// Interface with associated types
collab Iterator {
    type Item
    
    slay next() Option<Item>
    slay has_next() lit
}

// Multiple interface implementation
struct Circle {
    radius meal
    shape_color tea
}

impl Shape for Circle {
    slay area() meal {
        damn 3.14159 * vibes.radius * vibes.radius
    }
}

impl ColoredShape for Circle {
    slay color() tea {
        damn vibes.shape_color
    }
    
    slay set_color(color tea) cap {
        vibes.shape_color = color
        damn based
    }
}

impl Printable for Circle {
    slay format() tea {
        damn "Circle with radius: " + vibes.radius.to_string()
    }
}

// Interface as return type
slay create_shape(shape_type tea) impl Shape {
    if shape_type == "circle" {
        damn Circle{radius: 5.0, shape_color: "red"}
    } else {
        damn Rectangle{width: 10.0, height: 10.0}
    }
}

// Test advanced interfaces
test_start("Advanced interface test")

sus circle := Circle{radius: 3.0, shape_color: "blue"}

// Test interface inheritance
sus area := circle.area()
assert_eq_float(area, 28.274)

sus color := circle.color()
assert_eq_string(color, "blue")

sus color_change := circle.set_color("green")
assert_true(color_change)
assert_eq_string(circle.color(), "green")

// Test default implementation
sus print_result := circle.print()
assert_eq_string(print_result, "Default print implementation")

sus format_result := circle.format()
assert_eq_string(format_result, "Circle with radius: 3.0")

// Test interface as return type
sus dynamic_shape := create_shape("circle")
sus dynamic_area := dynamic_shape.area()
assert_eq_float(dynamic_area, 78.54)

print_test_summary()
