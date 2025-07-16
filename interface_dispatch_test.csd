yeet "testz"

// Define interfaces for testing
collab Shape {
    slay area() meal
    slay perimeter() meal
}

collab Drawable {
    slay draw() 
}

// Define a struct that implements interfaces
vibe Rectangle {
    width meal
    height meal
}

// Implement Shape interface for Rectangle
slay Rectangle.area() meal {
    damn self.width * self.height
}

slay Rectangle.perimeter() meal {
    damn 2.0 * (self.width + self.height)
}

// Implement Drawable interface for Rectangle
slay Rectangle.draw() {
    vibez.spill("Drawing rectangle")
}

// Test interface dispatch
test_start("Interface Method Dispatch Tests")

// Create a rectangle instance
sus rect Rectangle = Rectangle { width: 5.0, height: 3.0 }

// Test direct method calls
sus area meal = rect.area()
assert_eq_float(area, 15.0)

sus perimeter meal = rect.perimeter()
assert_eq_float(perimeter, 16.0)

// Test interface casting and dispatch
sus shape Shape = rect
sus interface_area meal = shape.area()
assert_eq_float(interface_area, 15.0)

sus drawable Drawable = rect
drawable.draw()

test_start("Interface Inheritance Test")

// Define base interface
collab Printer {
    slay print(message tea)
}

// Define derived interface
collab ColorPrinter extends Printer {
    slay print_color(message tea, color tea)
}

// Implement ColorPrinter
vibe LaserPrinter {
    name tea
}

slay LaserPrinter.print(message tea) {
    vibez.spill("Printing: " + message)
}

slay LaserPrinter.print_color(message tea, color tea) {
    vibez.spill("Printing " + message + " in " + color)
}

// Test inheritance
sus printer LaserPrinter = LaserPrinter { name: "HP Laser" }
sus color_printer ColorPrinter = printer
color_printer.print("Hello")
color_printer.print_color("World", "red")

// Test base interface casting
sus base_printer Printer = printer
base_printer.print("Base interface call")

print_test_summary()
