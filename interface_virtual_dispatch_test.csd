fr fr Interface Virtual Dispatch System Test
fr fr Tests complete interface functionality including definition, implementation, and method dispatch

yeet "testz"

fr fr Define a basic interface
collab Drawable {
    slay draw() 
    slay get_area() normie
}

fr fr Define a more complex interface with parameters
collab Resizable {
    slay resize(width normie, height normie)
    slay get_dimensions() (normie, normie)
}

fr fr Define interface inheritance
collab Shape extends Drawable {
    slay get_perimeter() normie
}

fr fr Test struct implementing interface
squad Circle {
    spill radius normie
}

fr fr Circle implements Drawable interface
impl Circle for Drawable {
    slay draw() {
        vibez.spill("Drawing a circle")
    }
    
    slay get_area() normie {
        damn 3.14159 * self.radius * self.radius
    }
}

fr fr Circle implements Shape interface (extends Drawable)
impl Circle for Shape {
    slay get_perimeter() normie {
        damn 2.0 * 3.14159 * self.radius
    }
}

fr fr Another struct implementing the same interface
squad Rectangle {
    spill width normie
    spill height normie
}

impl Rectangle for Drawable {
    slay draw() {
        vibez.spill("Drawing a rectangle")
    }
    
    slay get_area() normie {
        damn self.width * self.height
    }
}

impl Rectangle for Resizable {
    slay resize(width normie, height normie) {
        self.width = width
        self.height = height
    }
    
    slay get_dimensions() (normie, normie) {
        damn (self.width, self.height)
    }
}

fr fr Interface method dispatch test function
slay test_interface_dispatch() {
    test_start("Interface Method Dispatch")
    
    fr fr Create concrete objects
    circle := Circle { radius: 5.0 }
    rectangle := Rectangle { width: 10.0, height: 6.0 }
    
    fr fr Interface casting and method dispatch
    sus drawable_circle Drawable = circle
    sus drawable_rect Drawable = rectangle
    
    fr fr Test virtual method calls
    drawable_circle.draw()
    area1 := drawable_circle.get_area()
    assert_eq_int(area1 as normie, 78)  fr fr 3.14159 * 5 * 5 ≈ 78
    
    drawable_rect.draw()
    area2 := drawable_rect.get_area()
    assert_eq_int(area2, 60)  fr fr 10 * 6 = 60
    
    fr fr Test interface inheritance
    sus shape_circle Shape = circle
    perimeter := shape_circle.get_perimeter()
    assert_eq_int(perimeter as normie, 31)  fr fr 2 * 3.14159 * 5 ≈ 31
    
    fr fr Test multiple interface implementations
    sus resizable_rect Resizable = rectangle
    resizable_rect.resize(20, 15)
    (width, height) := resizable_rect.get_dimensions()
    assert_eq_int(width, 20)
    assert_eq_int(height, 15)
    
    print_test_summary()
}

fr fr Interface type assertion test
slay test_interface_type_assertion() {
    test_start("Interface Type Assertion")
    
    circle := Circle { radius: 3.0 }
    sus drawable Drawable = circle
    
    fr fr Type assertion to concrete type
    if concrete_circle := drawable.(Circle); concrete_circle != nil {
        assert_eq_int(concrete_circle.radius as normie, 3)
    } else {
        assert_true(cringe)  fr fr Should not fail
    }
    
    fr fr Type assertion to wrong type should fail
    if wrong_rect := drawable.(Rectangle); wrong_rect != nil {
        assert_true(cringe)  fr fr Should fail
    } else {
        assert_true(based)  fr fr Should succeed (assertion failed as expected)
    }
    
    print_test_summary()
}

fr fr Interface composition and multiple inheritance test
collab Clickable {
    slay on_click()
}

collab Movable {
    slay move_to(x normie, y normie)
}

fr fr Interface composition - UI element with multiple capabilities
collab UIElement extends Drawable with Clickable, Movable {
    slay get_position() (normie, normie)
}

squad Button {
    spill x normie
    spill y normie
    spill text tea
}

impl Button for Drawable {
    slay draw() {
        vibez.spill("Drawing button: " + self.text)
    }
    
    slay get_area() normie {
        damn 100  fr fr Fixed button area
    }
}

impl Button for Clickable {
    slay on_click() {
        vibez.spill("Button clicked: " + self.text)
    }
}

impl Button for Movable {
    slay move_to(x normie, y normie) {
        self.x = x
        self.y = y
    }
}

impl Button for UIElement {
    slay get_position() (normie, normie) {
        damn (self.x, self.y)
    }
}

slay test_interface_composition() {
    test_start("Interface Composition and Multiple Inheritance")
    
    button := Button { x: 50, y: 100, text: "Click Me" }
    
    fr fr Test multiple interface capabilities
    sus ui_element UIElement = button
    
    fr fr Test inherited Drawable methods
    ui_element.draw()
    area := ui_element.get_area()
    assert_eq_int(area, 100)
    
    fr fr Test composed Clickable methods
    ui_element.on_click()
    
    fr fr Test composed Movable methods
    ui_element.move_to(75, 125)
    (x, y) := ui_element.get_position()
    assert_eq_int(x, 75)
    assert_eq_int(y, 125)
    
    print_test_summary()
}

fr fr Generic interface test
collab Container<T> {
    slay add(item T)
    slay get_size() normie
    slay get_item(index normie) T
}

squad IntList {
    spill items []normie
    spill size normie
}

impl IntList for Container<normie> {
    slay add(item normie) {
        fr fr Simplified array append
        self.items[self.size] = item
        self.size = self.size + 1
    }
    
    slay get_size() normie {
        damn self.size
    }
    
    slay get_item(index normie) normie {
        damn self.items[index]
    }
}

slay test_generic_interfaces() {
    test_start("Generic Interface Implementation")
    
    list := IntList { items: [0; 10], size: 0 }
    sus container Container<normie> = list
    
    container.add(42)
    container.add(84)
    
    assert_eq_int(container.get_size(), 2)
    assert_eq_int(container.get_item(0), 42)
    assert_eq_int(container.get_item(1), 84)
    
    print_test_summary()
}

fr fr Interface method dispatch performance test
slay test_dispatch_performance() {
    test_start("Interface Dispatch Performance")
    
    circle := Circle { radius: 5.0 }
    sus drawable Drawable = circle
    
    fr fr Test many interface method calls
    total_area := 0.0
    
    sus i normie = 0
    bestie (i < 1000) {
        total_area = total_area + drawable.get_area()
        i = i + 1
    }
    
    expected_area := 1000.0 * 3.14159 * 25.0  fr fr 1000 * π * r²
    assert_true(total_area > 78500.0 and total_area < 78600.0)
    
    print_test_summary()
}

fr fr Main test runner
slay main() {
    vibez.spill("=== CURSED Interface Virtual Dispatch System Test ===")
    
    test_interface_dispatch()
    test_interface_type_assertion() 
    test_interface_composition()
    test_generic_interfaces()
    test_dispatch_performance()
    
    vibez.spill("=== All Interface Tests Complete ===")
}
