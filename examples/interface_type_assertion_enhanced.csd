vibe main;

// Basic errors interface
collab Error {
    slay error() tea;
}

// Enhanced type assertion error with more detailed information
squad EnhancedTypeAssertionError {
    expected tea,        // Expected type name
    actual tea,         // Actual type name
    source_line tea,    // Source line where the error occurred
    line normie,        // Line number in the source file
    column normie,      // Column number in the source file
    file tea,           // Source file name
    interfaces []tea    // List of interfaces implemented by the actual type
}

// Implement Error interface for EnhancedTypeAssertionError
slay (e EnhancedTypeAssertionError) error() tea {
    sus message = "Type assertion failed: expected " + e.expected + " but got " + e.actual;
    
    // Add source location information if available
    lowkey e.file != "" {
        message = message + "\n at " + e.file + ":" + e.line + ":" + e.column;
    }
    
    // Add source line if available
    lowkey e.source_line != "" {
        message = message + "\n in: " + e.source_line;
    }
    
    // Add interface information if available
    lowkey e.interfaces != cap && len(e.interfaces) > 0 {
        message = message + "\n implemented interfaces: " + strings.Join(e.interfaces, ", ");
    }
    
    return message;
}

// Basic Result type similar to Rust's Result
// Contains either a value or an error
squad Result<T, E> {
    value T,
    err E,
    isOk lit
}

// Helper to create a successful result
slay ok<T, E>(value T) Result<T, E> {
    return Result<T, E>{
        value: value,
        err: nofr as E,
        isOk: 1
    };
}

// Helper to create an error result
slay fail<T, E>(err E) Result<T, E> {
    return Result<T, E>{
        value: nofr as T,
        err: err,
        isOk: 0
    };
}

// Shape interface for geometric shapes
collab Shape {
    slay area() meal;
    slay name() tea;
}

// Drawable interface for things that can be drawn
collab Drawable {
    slay draw() tea;
}

// Circle implements Shape
squad Circle {
    radius meal,
}

slay (c Circle) area() meal {
    return 3.14159 * c.radius * c.radius;
}

slay (c Circle) name() tea {
    return "Circle";
}

slay (c Circle) draw() tea {
    return "Drawing a circle with radius " + c.radius;
}

// Rectangle implements Shape and Drawable with a diamond inheritance pattern
squad Rectangle {
    width meal,
    height meal
}

slay (r Rectangle) area() meal {
    return r.width * r.height;
}

slay (r Rectangle) name() tea {
    return "Rectangle";
}

slay (r Rectangle) draw() tea {
    return "Drawing a rectangle " + r.width + "x" + r.height;
}

// Triangle implements Shape but not Drawable
squad Triangle {
    base meal,
    height meal
}

slay (t Triangle) area() meal {
    return 0.5 * t.base * t.height;
}

slay (t Triangle) name() tea {
    return "Triangle";
}

// Function that tries to assert a drawable shape - demonstrates diamond inheritance pattern
slay drawShape(shape Shape) Result<tea, Error> {
    // Try to assert shape as Drawable, which could be Circle or Rectangle
    // This will use enhanced error reporting if it fails
    sus drawable = shape.(Drawable)?;
    
    // If we get here, the assertion succeeded
    // Draw the shape and return the result
    return ok<tea, Error>(drawable.draw());
}

// Function with nested assertions to demonstrate error propagation accuracy
slay processNestedShape(shape Shape) Result<tea, Error> {
    // First try to get the shape's name
    sus name = shape.name();
    
    // Try to draw the shape, this could fail with enhanced error reporting
    sus drawing = drawShape(shape)?;
    
    // If we get here, the shape was drawable
    // Return a combined result
    return ok<tea, Error>(name + ": " + drawing);
}

// Main function to demonstrate enhanced error reporting
slay main() {
    // Create different shapes
    sus circle = Circle{radius: 5.0};
    sus rectangle = Rectangle{width: 4.0, height: 6.0};
    sus triangle = Triangle{base: 3.0, height: 8.0};
    
    // Variable to store a shape interface
    sus shape Shape;
    
    // Try with Circle (implements both Shape and Drawable)
    shape = circle;
    vibez.spill("\nProcessing Circle:");
    sus circleResult = processNestedShape(shape);
    lowkey circleResult.isOk {
        vibez.spill("Success: " + circleResult.value);
    } no cap {
        vibez.spill("Error: " + circleResult.err.error());
    }
    
    // Try with Rectangle (implements both Shape and Drawable)
    shape = rectangle;
    vibez.spill("\nProcessing Rectangle:");
    sus rectResult = processNestedShape(shape);
    lowkey rectResult.isOk {
        vibez.spill("Success: " + rectResult.value);
    } no cap {
        vibez.spill("Error: " + rectResult.err.error());
    }
    
    // Try with Triangle (implements Shape but not Drawable)
    // This should fail with enhanced error reporting
    shape = triangle;
    vibez.spill("\nProcessing Triangle:");
    sus triangleResult = processNestedShape(shape);
    lowkey triangleResult.isOk {
        vibez.spill("Success: " + triangleResult.value);
    } no cap {
        vibez.spill("Error: " + triangleResult.err.error());
    }
    
    // Demonstrate direct type assertion with detailed error information
    vibez.spill("\nDirect type assertions with enhanced errors:");
    
    // This should succeed
    sus circle_drawable, ok1 = circle.(Drawable);
    lowkey ok1 {
        vibez.spill("Circle is Drawable: " + circle_drawable.draw());
    } no cap {
        vibez.spill("Circle is not Drawable");
    }
    
    // This should fail with detailed error information
    sus triangle_drawable, ok2 = triangle.(Drawable);
    lowkey ok2 {
        vibez.spill("Triangle is Drawable: " + triangle_drawable.draw());
    } no cap {
        vibez.spill("Triangle is not Drawable - would show enhanced error details");
    }
}