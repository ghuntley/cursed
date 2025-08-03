fr fr Advanced CURSED language features test for Zig codegen
fr fr Testing structs, interfaces, tuples, and memory management

fr fr Define a basic struct
squad Person {
    spill name tea
    spill age normie
    spill is_active lit
}

fr fr Define an interface
collab Drawable {
    slay draw() 
    slay get_area() normie
}

fr fr Define a struct that implements the interface
squad Circle {
    spill radius normie
    spill center_x normie
    spill center_y normie
}

fr fr Implement interface methods for Circle
slay Circle.draw() {
    vibez.spill("Drawing a circle")
}

slay Circle.get_area() normie {
    damn self.radius * self.radius * 3
}

fr fr Define another implementing struct
squad Rectangle {
    spill width normie
    spill height normie
}

slay Rectangle.draw() {
    vibez.spill("Drawing a rectangle")
}

slay Rectangle.get_area() normie {
    damn self.width * self.height
}

fr fr Test struct creation and field access
slay test_structs() {
    vibez.spill("Testing struct features")
    
    fr fr Create a person struct
    sus person drip = Person{
        name: "Alice",
        age: 30,
        is_active: based
    }
    
    fr fr Access struct fields
    vibez.spill(person.name)
    vibez.spill(person.age)
    vibez.spill(person.is_active)
}

fr fr Test tuple functionality
slay test_tuples() {
    vibez.spill("Testing tuple features")
    
    fr fr Create a tuple
    sus coords drip = (10, 20, 30)
    
    fr fr Access tuple elements
    vibez.spill(coords.0)
    vibez.spill(coords.1)
    vibez.spill(coords.2)
    
    fr fr Create a mixed-type tuple
    sus mixed drip = ("hello", 42, based)
    vibez.spill(mixed.0)
    vibez.spill(mixed.1)
    vibez.spill(mixed.2)
}

fr fr Test interface dispatch
slay test_interfaces() {
    vibez.spill("Testing interface features")
    
    fr fr Create shapes
    sus circle drip = Circle{
        radius: 5,
        center_x: 10,
        center_y: 20
    }
    
    sus rectangle drip = Rectangle{
        width: 10,
        height: 5
    }
    
    fr fr Test method calls
    circle.draw()
    vibez.spill(circle.get_area())
    
    rectangle.draw()
    vibez.spill(rectangle.get_area())
}

fr fr Test generic functionality (placeholder)
slay test_generics() {
    vibez.spill("Testing generic features (placeholder)")
    
    fr fr Generic array would be defined like:
    fr fr squad Array<T> {
    fr fr     spill data *T
    fr fr     spill size normie
    fr fr }
    
    fr fr For now, just test basic generic-like behavior
    sus numbers drip = [1, 2, 3, 4, 5]
    sus strings drip = ["hello", "world", "zig"]
    
    vibez.spill("Generic array test placeholder")
}

fr fr Test memory management
slay test_memory_management() {
    vibez.spill("Testing memory management")
    
    fr fr Create multiple structs to test GC
    sus person1 drip = Person{
        name: "Bob",
        age: 25,
        is_active: based
    }
    
    sus person2 drip = Person{
        name: "Carol",
        age: 35,
        is_active: cringe
    }
    
    fr fr Create some circles
    sus circle1 drip = Circle{
        radius: 3,
        center_x: 0,
        center_y: 0
    }
    
    sus circle2 drip = Circle{
        radius: 7,
        center_x: 5,
        center_y: 5
    }
    
    vibez.spill("Memory management test complete")
}

fr fr Main test function
slay main_character() {
    vibez.spill("Starting advanced CURSED features test")
    
    test_structs()
    test_tuples()
    test_interfaces()
    test_generics()
    test_memory_management()
    
    vibez.spill("All advanced features tested successfully!")
}
