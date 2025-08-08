# Debug method call issues
squad Circle {
    spill radius normie
    
    slay area() normie {
        damn 3.14159 * this.radius * this.radius
    }
    
    slay perimeter() normie {
        damn 2.0 * 3.14159 * this.radius
    }
    
    slay get_name() tea {
        damn "MyCircle"
    }
}

sus circle Circle = Circle{radius: 5.0}

vibez.spill("Testing direct calls:")
vibez.spill("Area:", circle.area())

# Test if perimeter method exists
vibez.spill("Perimeter:", circle.perimeter())

# Test if get_name method exists  
vibez.spill("Name:", circle.get_name())
