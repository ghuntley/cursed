fr fr Advanced Interfaces Validation Test
yeet "testz"

fr fr Define basic interface
collab Drawable {
    slay draw() tea
    slay area() meal
}

fr fr Define interface with generic methods
collab Container<T> {
    slay add(item T)
    slay get(index normie) T
    slay size() normie
}

fr fr Define interface composition
collab Movable {
    slay move_to(x meal, y meal)
    slay get_position() (meal, meal)
}

collab DrawableMovable {
    collab Drawable
    collab Movable
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

squad DynamicArray<T> {
    spill items []T
    spill capacity normie
}

fr fr Implement Drawable for Circle
flex Circle => Drawable {
    slay draw() tea {
        damn "Drawing circle with radius " + radius.to_string()
    }
    
    slay area() meal {
        damn 3.14159 * radius * radius
    }
}

fr fr Implement Movable for Circle
flex Circle => Movable {
    slay move_to(new_x meal, new_y meal) {
        x = new_x
        y = new_y
    }
    
    slay get_position() (meal, meal) {
        damn (x, y)
    }
}

fr fr Implement DrawableMovable for Circle (interface composition)
flex Circle => DrawableMovable {}

fr fr Implement Drawable for Rectangle
flex Rectangle => Drawable {
    slay draw() tea {
        damn "Drawing rectangle " + width.to_string() + "x" + height.to_string()
    }
    
    slay area() meal {
        damn width * height
    }
}

fr fr Implement Movable for Rectangle
flex Rectangle => Movable {
    slay move_to(new_x meal, new_y meal) {
        x = new_x
        y = new_y
    }
    
    slay get_position() (meal, meal) {
        damn (x, y)
    }
}

fr fr Implement generic Container interface
flex DynamicArray<T> => Container<T> {
    slay add(item T) {
        items.push(item)
    }
    
    slay get(index normie) T {
        damn items[index]
    }
    
    slay size() normie {
        damn items.len()
    }
}

fr fr Test virtual dispatch with interfaces
slay draw_shape(shape Drawable) tea {
    damn shape.draw()
}

slay calculate_total_area(shapes []Drawable) meal {
    sus total meal = 0.0
    bestie shape in shapes {
        total = total + shape.area()
    }
    damn total
}

slay move_and_draw(obj DrawableMovable, x meal, y meal) tea {
    obj.move_to(x, y)
    damn obj.draw()
}

slay test_interfaces() {
    test_start("Basic Interface Implementation Test")
    
    fr fr Test basic interface implementation
    sus circle Circle = Circle{radius: 5.0, x: 0.0, y: 0.0}
    sus rect Rectangle = Rectangle{width: 4.0, height: 3.0, x: 0.0, y: 0.0}
    
    sus circle_desc tea = circle.draw()
    sus rect_desc tea = rect.draw()
    
    assert_true(circle_desc.contains("circle"))
    assert_true(rect_desc.contains("rectangle"))
    
    test_start("Virtual Dispatch Test")
    
    fr fr Test virtual dispatch through interface
    sus drawable_circle Drawable = circle
    sus drawable_rect Drawable = rect
    
    sus circle_area meal = drawable_circle.area()
    sus rect_area meal = drawable_rect.area()
    
    assert_eq_float(circle_area, 78.53975)
    assert_eq_float(rect_area, 12.0)
    
    test_start("Interface Function Parameters Test")
    
    fr fr Test passing interface implementations to functions
    sus circle_draw_result tea = draw_shape(circle)
    sus rect_draw_result tea = draw_shape(rect)
    
    assert_true(circle_draw_result.contains("circle"))
    assert_true(rect_draw_result.contains("rectangle"))
    
    test_start("Interface Arrays Test")
    
    fr fr Test arrays of interface implementations
    sus shapes []Drawable = [circle, rect]
    sus total_area meal = calculate_total_area(shapes)
    
    assert_eq_float(total_area, 90.53975)
    
    test_start("Interface Composition Test")
    
    fr fr Test interface composition
    sus movable_circle DrawableMovable = circle
    sus result tea = move_and_draw(movable_circle, 10.0, 20.0)
    sus (pos_x, pos_y) = movable_circle.get_position()
    
    assert_eq_float(pos_x, 10.0)
    assert_eq_float(pos_y, 20.0)
    assert_true(result.contains("circle"))
    
    test_start("Generic Interface Test")
    
    fr fr Test generic interface implementation
    sus int_array DynamicArray<normie> = DynamicArray<normie>{items: [], capacity: 10}
    sus container Container<normie> = int_array
    
    container.add(1)
    container.add(2)
    container.add(3)
    
    assert_eq_int(container.size(), 3)
    assert_eq_int(container.get(0), 1)
    assert_eq_int(container.get(1), 2)
    assert_eq_int(container.get(2), 3)
    
    print_test_summary()
}

test_interfaces()
