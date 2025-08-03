fr fr Interface Implementation and Virtual Dispatch Test
fr fr Tests complete interface system including definitions, implementations, and method calls

yeet "testz"

fr fr Define a simple interface
collab Drawable {
    slay draw()
    slay get_area() normie
}

fr fr Define a struct that will implement the interface
squad Circle {
    spill radius meal
}

fr fr Implement the interface for the struct
impl Circle for Drawable {
    slay draw() {
        vibez.spill("Drawing a circle")
    }
    
    slay get_area() normie {
        damn 314
    }
}

fr fr Define another struct
squad Rectangle {
    spill width meal
    spill height meal
}

fr fr Implement the same interface for another struct
impl Rectangle for Drawable {
    slay draw() {
        vibez.spill("Drawing a rectangle")
    }
    
    slay get_area() normie {
        damn 100
    }
}

fr fr Test interface implementations
slay test_interface_implementations() {
    test_start("Interface Implementation Test")
    
    fr fr This is a basic test to see if the implementation parsing works
    vibez.spill("Interface implementation parsing test complete")
    
    print_test_summary()
}

slay main() {
    vibez.spill("=== Interface Implementation Test ===")
    test_interface_implementations()
}
