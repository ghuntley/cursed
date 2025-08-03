fr fr CURSED Interface Implementation System Test
fr fr Tests complete interface functionality including collab and implementation parsing

yeet "testz"

fr fr Test 1: Basic Interface Definition
test_start("Basic Interface Definition")

collab Drawable {
    slay draw()
    slay get_area() normie
}

fr fr Test 2: Interface with Generic Parameters
collab Container<T> {
    slay add(item T)
    slay get_size() normie
    slay get_item(index normie) T
}

fr fr Test 3: Interface Inheritance 
collab Shape extends Drawable {
    slay get_perimeter() normie
}

fr fr Test 4: Interface Composition
collab UIElement extends Drawable with Clickable {
    slay get_position() (normie, normie)
}

fr fr Supporting interfaces for composition test
collab Clickable {
    slay on_click()
}

fr fr Test 5: Struct Definition
squad Circle {
    spill radius normie
}

squad Rectangle {
    spill width normie
    spill height normie
}

fr fr Test 6: Basic Interface Implementation
impl Circle for Drawable {
    slay draw() {
        vibez.spill("Drawing a circle")
    }
    
    slay get_area() normie {
        damn 78  fr fr Simplified π * r² for r=5
    }
}

impl Rectangle for Drawable {
    slay draw() {
        vibez.spill("Drawing a rectangle")
    }
    
    slay get_area() normie {
        damn 60  fr fr width * height = 10 * 6
    }
}

fr fr Test 7: Interface Inheritance Implementation
impl Circle for Shape {
    slay get_perimeter() normie {
        damn 31  fr fr Simplified 2 * π * r for r=5
    }
}

fr fr Test 8: Generic Interface Implementation
squad IntList {
    spill items [10]normie
    spill size normie
}

impl IntList for Container<normie> {
    slay add(item normie) {
        fr fr Simplified array append - just store in next slot
        self.size = self.size + 1
    }
    
    slay get_size() normie {
        damn self.size
    }
    
    slay get_item(index normie) normie {
        damn self.items[index]
    }
}

fr fr Test 9: Interface Usage and Virtual Dispatch
slay test_virtual_dispatch() {
    test_start("Virtual Dispatch Test")
    
    fr fr Create concrete objects
    circle := Circle { radius: 5 }
    rectangle := Rectangle { width: 10, height: 6 }
    
    fr fr Interface casting and virtual method calls
    sus drawable_circle Drawable = circle
    sus drawable_rect Drawable = rectangle
    
    fr fr Test virtual method dispatch
    drawable_circle.draw()
    area1 := drawable_circle.get_area()
    assert_eq_int(area1, 78)
    
    drawable_rect.draw()
    area2 := drawable_rect.get_area()
    assert_eq_int(area2, 60)
    
    fr fr Test interface inheritance
    sus shape_circle Shape = circle
    perimeter := shape_circle.get_perimeter()
    assert_eq_int(perimeter, 31)
    
    fr fr Test method calls through inheritance
    shape_circle.draw()  fr fr Should work via extends Drawable
    inherited_area := shape_circle.get_area()
    assert_eq_int(inherited_area, 78)
    
    print_test_summary()
}

fr fr Test 10: Generic Interface Usage
slay test_generic_interface() {
    test_start("Generic Interface Test")
    
    int_list := IntList { items: [0; 10], size: 0 }
    sus container Container<normie> = int_list
    
    container.add(42)
    container.add(84)
    
    size := container.get_size()
    assert_eq_int(size, 2)
    
    first_item := container.get_item(0)
    assert_eq_int(first_item, 42)
    
    print_test_summary()
}

fr fr Test 11: Interface Type Assertion
slay test_type_assertion() {
    test_start("Interface Type Assertion Test")
    
    circle := Circle { radius: 3 }
    sus drawable Drawable = circle
    
    fr fr Type assertion to concrete type
    if concrete_circle := drawable.(Circle); concrete_circle != cap {
        assert_eq_int(concrete_circle.radius, 3)
        assert_true(based)
    } else {
        assert_true(cringe)  fr fr Should not fail
    }
    
    print_test_summary()
}

fr fr Main test runner
slay main() {
    vibez.spill("=== CURSED Interface Implementation System Test ===")
    
    fr fr Basic functionality tests
    test_start("Interface Definition Parsing")
    assert_true(based)  fr fr If we get here, parsing succeeded
    print_test_summary()
    
    fr fr Advanced functionality tests
    test_virtual_dispatch()
    test_generic_interface()
    test_type_assertion()
    
    vibez.spill("=== All Interface System Tests Complete ===")
    damn 0
}
