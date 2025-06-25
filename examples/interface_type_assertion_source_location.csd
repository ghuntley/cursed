vibe main;

// Basic errors interface
collab Error {
    slay error() tea;
}

// Type assertion error - returned when a type assertion fails
squad TypeAssertionError {
    expected tea,
    actual tea,
    file tea,
    line lit,
    column lit,
    source_line tea
}

// Implement Error interface for TypeAssertionError
slay (e TypeAssertionError) error() tea {
    return "Type assertion failed at " + e.file + ":" + e.line + "," + e.column + ": "
        + "expected " + e.expected + " but got " + e.actual + "\n"
        + "Source: " + e.source_line;
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

// Rectangle implements Shape
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

// Function that uses type assertion with ? operator
// It returns Result<meal, Error> and uses ? to propagate errors
slay calculateCircleArea(shape Shape) Result<meal, Error> {
    // Try to assert shape as Circle using ? operator
    // If assertion fails, ? will return early with the error that includes source location
    sus circle = shape.(Circle)?;
    
    // If we get here, the assertion succeeded
    // Calculate and return area with radius * 2 (diameter)
    return ok<meal, Error>(circle.radius * 2);
}

// Function that chains multiple assertions with ? operator and enhanced source location
slay processShape(shape Shape) Result<tea, Error> {
    // This will have precise source location in the error if it fails
    sus details = calculateCircleArea(shape)?;
    
    vibez.spill("Area details: " + details);
    
    // Return a formatted message
    return ok<tea, Error>("Shape has diameter: " + details);
}

// Main function to demonstrate enhanced source location in error reporting
slay main() {
    // Create different shapes
    sus circle = Circle{radius: 5.0};
    sus rectangle = Rectangle{width: 4.0, height: 6.0};
    
    // Variable to store a shape interface
    sus shape Shape;
    
    // Try with Circle
    shape = circle;
    vibez.spill("\nProcessing Circle:");
    sus circleResult = calculateCircleArea(shape);
    lowkey circleResult.isOk {
        vibez.spill("Circle calculation result: " + circleResult.value);
    } no cap {
        // This will include detailed source location
        vibez.spill("Error: " + circleResult.err.error());
    }
    
    // Process circle with more complex function
    sus circleProcessResult = processShape(circle);
    lowkey circleProcessResult.isOk {
        vibez.spill(circleProcessResult.value);
    } no cap {
        vibez.spill("Error processing circle: " + circleProcessResult.err.error());
    }
    
    // Try with Rectangle (should fail with source location information)
    shape = rectangle;
    vibez.spill("\nProcessing Rectangle:");
    sus rectResult = calculateCircleArea(shape);
    lowkey rectResult.isOk {
        vibez.spill("Rectangle calculation result: " + rectResult.value);
    } no cap {
        // This shows enhanced error with source location
        vibez.spill("Error: " + rectResult.err.error());
    }
    
    // Process rectangle with more complex function (should also fail)
    sus rectProcessResult = processShape(rectangle);
    lowkey rectProcessResult.isOk {
        vibez.spill(rectProcessResult.value);
    } no cap {
        // This shows the propagated error with its original source location
        vibez.spill("Error processing rectangle: " + rectProcessResult.err.error());
    }
}