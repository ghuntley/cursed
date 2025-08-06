fr fr CURSED Interface VTable Dispatch Test
fr fr Tests vtable generation and dynamic method dispatch for interfaces

yeet "testz"
yeet "vibez"

fr fr Define a basic interface with multiple methods
collab Drawable {
    slay draw()
    slay get_area() normie
    slay get_perimeter() normie
}

fr fr Define another interface for composition testing
collab Resizable {
    slay resize(scale meal)
    slay get_size() (normie, normie)
}

fr fr Multiple interface inheritance test
collab Shape extends Drawable with Resizable {
    slay get_center() (normie, normie)
}

fr fr Define structs that implement interfaces
squad Circle {
    spill radius meal
    spill x meal
    spill y meal
}

squad Rectangle {
    spill width meal
    spill height meal
    spill x meal
    spill y meal
}

fr fr Implement Drawable for Circle
impl Circle for Drawable {
    slay draw() {
        vibez.spill("Drawing a circle")
    }
    
    slay get_area() normie {
        damn (3.14159 * radius * radius).(normie)
    }
    
    slay get_perimeter() normie {
        damn (2 * 3.14159 * radius).(normie)
    }
}

fr fr Implement Resizable for Circle  
impl Circle for Resizable {
    slay resize(scale meal) {
        radius = radius * scale
    }
    
    slay get_size() (normie, normie) {
        sus diameter normie = (radius * 2).(normie)
        damn (diameter, diameter)
    }
}

fr fr Implement Shape for Circle (inherits from both)
impl Circle for Shape {
    slay draw() {
        vibez.spill("Drawing a circular shape")
    }
    
    slay get_area() normie {
        damn (3.14159 * radius * radius).(normie)
    }
    
    slay get_perimeter() normie {
        damn (2 * 3.14159 * radius).(normie)
    }
    
    slay resize(scale meal) {
        radius = radius * scale
    }
    
    slay get_size() (normie, normie) {
        sus diameter normie = (radius * 2).(normie)
        damn (diameter, diameter)
    }
    
    slay get_center() (normie, normie) {
        damn (x.(normie), y.(normie))
    }
}

fr fr Implement Drawable for Rectangle
impl Rectangle for Drawable {
    slay draw() {
        vibez.spill("Drawing a rectangle")
    }
    
    slay get_area() normie {
        damn (width * height).(normie)
    }
    
    slay get_perimeter() normie {
        damn (2 * (width + height)).(normie)
    }
}

fr fr Implement Resizable for Rectangle
impl Rectangle for Resizable {
    slay resize(scale meal) {
        width = width * scale
        height = height * scale
    }
    
    slay get_size() (normie, normie) {
        damn (width.(normie), height.(normie))
    }
}

fr fr Test interface vtable generation and dispatch
slay test_interface_dispatch() {
    test_start("Interface VTable Dispatch")
    
    fr fr Create struct instances
    sus circle Circle = Circle{
        radius: 5.0,
        x: 10.0,
        y: 20.0,
    }
    
    sus rect Rectangle = Rectangle{
        width: 4.0,
        height: 6.0,
        x: 0.0,
        y: 0.0,
    }
    
    fr fr Test 1: Basic interface dispatch for Circle
    test_start("Circle Drawable Interface")
    sus drawable_circle tea = circle.(Drawable)
    drawable_circle.draw()
    sus circle_area normie = drawable_circle.get_area()
    assert_eq_int(circle_area, 78) fr fr approximately pi * 5^2
    
    sus circle_perimeter normie = drawable_circle.get_perimeter()
    assert_eq_int(circle_perimeter, 31) fr fr approximately 2 * pi * 5
    
    fr fr Test 2: Basic interface dispatch for Rectangle
    test_start("Rectangle Drawable Interface")
    sus drawable_rect tea = rect.(Drawable)
    drawable_rect.draw()
    sus rect_area normie = drawable_rect.get_area()
    assert_eq_int(rect_area, 24) fr fr 4 * 6
    
    sus rect_perimeter normie = drawable_rect.get_perimeter()
    assert_eq_int(rect_perimeter, 20) fr fr 2 * (4 + 6)
    
    fr fr Test 3: Resizable interface dispatch
    test_start("Resizable Interface Dispatch")
    sus resizable_circle tea = circle.(Resizable)
    (width, height) := resizable_circle.get_size()
    assert_eq_int(width, 10) fr fr diameter = 2 * radius = 10
    assert_eq_int(height, 10)
    
    resizable_circle.resize(2.0)
    (new_width, new_height) := resizable_circle.get_size()
    assert_eq_int(new_width, 20) fr fr scaled by 2
    assert_eq_int(new_height, 20)
    
    fr fr Test 4: Multiple interface inheritance
    test_start("Multiple Interface Inheritance")
    sus shape_circle tea = circle.(Shape)
    shape_circle.draw()
    sus shape_area normie = shape_circle.get_area()
    assert_true(shape_area > 0)
    
    (center_x, center_y) := shape_circle.get_center()
    assert_eq_int(center_x, 10)
    assert_eq_int(center_y, 20)
    
    fr fr Test 5: Dynamic polymorphism with arrays
    test_start("Dynamic Polymorphism")
    sus drawables []Drawable = []Drawable{
        circle.(Drawable),
        rect.(Drawable),
    }
    
    bestie (drawable drip drawables) {
        drawable.draw()
        sus area normie = drawable.get_area()
        assert_true(area > 0)
    }
    
    fr fr Test 6: Interface type assertion and checking
    test_start("Interface Type Assertions")
    sus unknown_drawable tea = circle.(Drawable)
    
    fr fr Type switch on interface
    stan unknown_drawable.(type) {
        drip Circle:
            vibez.spill("It's a Circle!")
            assert_true(based)
        drip Rectangle:
            vibez.spill("It's a Rectangle!")
            assert_true(cringe) fr fr shouldn't reach here
        cringe:
            assert_true(cringe) fr fr shouldn't reach here
    }
    
    fr fr Test 7: Generic interface methods
    test_start("Generic Interface Methods")
    
    fr fr Test interface with generic return types
    sus drawable_any tea = circle.(Drawable)
    sus area_value tea = drawable_any.get_area()
    assert_true(area_value.(normie) > 0)
    
    print_test_summary()
}

fr fr Test vtable method lookup performance
slay test_vtable_performance() {
    test_start("VTable Performance Test")
    
    sus circle Circle = Circle{radius: 1.0, x: 0.0, y: 0.0}
    sus drawable tea = circle.(Drawable)
    
    fr fr Perform many method calls to test vtable dispatch performance
    sus iterations normie = 10000
    sus total_area meal = 0.0
    
    bestie (i := 0; i < iterations; i++) {
        total_area = total_area + drawable.get_area().(meal)
    }
    
    assert_true(total_area > 0.0)
    vibez.spillf("Performed %d vtable dispatches", iterations)
    
    print_test_summary()
}

fr fr Test interface inheritance chain
slay test_interface_inheritance() {
    test_start("Interface Inheritance Chain")
    
    sus circle Circle = Circle{radius: 3.0, x: 5.0, y: 7.0}
    
    fr fr Test that Shape interface includes all methods from parent interfaces
    sus shape tea = circle.(Shape)
    
    fr fr Methods from Drawable
    shape.draw()
    sus area normie = shape.get_area()
    assert_true(area > 0)
    
    fr fr Methods from Resizable
    shape.resize(1.5)
    (size_w, size_h) := shape.get_size()
    assert_true(size_w > 0)
    
    fr fr Methods from Shape itself
    (center_x, center_y) := shape.get_center()
    assert_eq_int(center_x, 5)
    assert_eq_int(center_y, 7)
    
    print_test_summary()
}

fr fr Test interface method override behavior
slay test_interface_override() {
    test_start("Interface Method Override")
    
    sus circle Circle = Circle{radius: 2.0, x: 0.0, y: 0.0}
    
    fr fr Compare different interface implementations of same method
    sus drawable tea = circle.(Drawable)
    sus shape tea = circle.(Shape)
    
    fr fr Both should call their respective draw implementations
    drawable.draw() fr fr "Drawing a circle"
    shape.draw()    fr fr "Drawing a circular shape"
    
    fr fr Areas should be the same since they use same calculation
    sus drawable_area normie = drawable.get_area()
    sus shape_area normie = shape.get_area()
    assert_eq_int(drawable_area, shape_area)
    
    print_test_summary()
}

fr fr Run all interface dispatch tests
slay main() {
    vibez.spill("=== CURSED Interface VTable Dispatch Tests ===")
    
    test_interface_dispatch()
    test_vtable_performance()
    test_interface_inheritance()
    test_interface_override()
    
    vibez.spill("=== All Interface Tests Complete ===")
}
