// Basic interface test for CURSED language
yeet "testz"

// Basic interface definition
collab Drawable {
    slay draw() tea
    slay area() meal
}

// Interface with parameters
collab Resizable {
    slay resize(width meal, height meal) cap
}

// Struct implementing interface
struct Rectangle {
    width meal
    height meal
}

// Interface implementation
impl Drawable for Rectangle {
    slay draw() tea {
        damn "Drawing rectangle"
    }
    
    slay area() meal {
        damn vibes.width * vibes.height
    }
}

impl Resizable for Rectangle {
    slay resize(width meal, height meal) cap {
        vibes.width = width
        vibes.height = height
        damn based
    }
}

// Function accepting interface
slay draw_shape(shape impl Drawable) tea {
    damn shape.draw()
}

// Test basic interfaces
test_start("Basic interface test")

sus rect := Rectangle{width: 10.0, height: 20.0}

// Test interface method calls
sus drawing := rect.draw()
assert_eq_string(drawing, "Drawing rectangle")

sus area := rect.area()
assert_eq_float(area, 200.0)

// Test interface parameter
sus result := draw_shape(rect)
assert_eq_string(result, "Drawing rectangle")

// Test interface mutation
sus resize_result := rect.resize(15.0, 25.0)
assert_true(resize_result)
assert_eq_float(rect.width, 15.0)
assert_eq_float(rect.height, 25.0)

print_test_summary()
