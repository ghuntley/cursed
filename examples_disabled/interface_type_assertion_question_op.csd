vibe main;

// Basic errors interface
collab Error {
    slay error() tea;
}

// Type assertion error - returned when a type assertion fails
squad TypeAssertionError {
    expected tea,
    actual tea
}

// Implement Error interface for TypeAssertionError
slay (e TypeAssertionError) error() tea {
    return "Type assertion failed: expected " + e.expected + " but got " + e.actual;
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

// Triangle implements Shape
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

// Function that uses type assertion with ? operator
// It returns Result<meal, Error> and uses ? to propagate errors
slay calculateCircleArea(shape Shape) Result<meal, Error> {
    // Try to assert shape as Circle using ? operator
    // If assertion fails, ? will return early with the error
    sus circle = shape.(Circle)?;
    
    // If we get here, the assertion succeeded
    // Calculate and return area with radius * 2 (diameter)
    return ok<meal, Error>(circle.radius * 2);
}

// Function that handles both Circle and Rectangle with ? operator
slay getShapeDetails(shape Shape) Result<tea, Error> {
    // Try asserting as Circle
    sus circle, circleOk = shape.(Circle);
    lowkey circleOk {
        return ok<tea, Error>("Circle with radius " + circle.radius);
    }
    
    // Try asserting as Rectangle with ? operator
    sus rect = shape.(Rectangle)?;
    
    // If we get here, it's a rectangle
    return ok<tea, Error>("Rectangle " + rect.width + "x" + rect.height);
}

// Function that chains multiple assertions with ? operator
slay processShape(shape Shape) Result<tea, Error> {
    sus details = getShapeDetails(shape)?;
    vibez.spill("Shape details: " + details);
    
    // We can also chain multiple ? operations
    sus area = shape.area();
    sus message = "Shape has area: " + area;
    
    return ok<tea, Error>(message);
}

// Main function to demonstrate ? operator with type assertions
slay main() {
    // Create different shapes
    sus circle = Circle{radius: 5.0};
    sus rectangle = Rectangle{width: 4.0, height: 6.0};
    sus triangle = Triangle{base: 3.0, height: 8.0};
    
    // Variable to store a shape interface
    sus shape Shape;
    
    // Try with Circle
    shape = circle;
    vibez.spill("\nProcessing Circle:");
    sus circleResult = calculateCircleArea(shape);
    lowkey circleResult.isOk {
        vibez.spill("Circle calculation result: " + circleResult.value);
    } no cap {
        vibez.spill("Error: " + circleResult.err.error());
    }
    
    // Process circle with more complex function
    sus circleProcessResult = processShape(circle);
    lowkey circleProcessResult.isOk {
        vibez.spill(circleProcessResult.value);
    } no cap {
        vibez.spill("Error processing circle: " + circleProcessResult.err.error());
    }
    
    // Try with Rectangle
    shape = rectangle;
    vibez.spill("\nProcessing Rectangle:");
    sus rectResult = calculateCircleArea(shape);
    lowkey rectResult.isOk {
        vibez.spill("Rectangle calculation result: " + rectResult.value);
    } no cap {
        vibez.spill("Error: " + rectResult.err.error());
    }
    
    // Process rectangle with more complex function
    sus rectProcessResult = processShape(rectangle);
    lowkey rectProcessResult.isOk {
        vibez.spill(rectProcessResult.value);
    } no cap {
        vibez.spill("Error processing rectangle: " + rectProcessResult.err.error());
    }
    
    // Try with Triangle
    shape = triangle;
    vibez.spill("\nProcessing Triangle:");
    sus triangleResult = calculateCircleArea(shape);
    lowkey triangleResult.isOk {
        vibez.spill("Triangle calculation result: " + triangleResult.value);
    } no cap {
        vibez.spill("Error: " + triangleResult.err.error());
    }
    
    // Process triangle with more complex function
    sus triangleProcessResult = processShape(triangle);
    lowkey triangleProcessResult.isOk {
        vibez.spill(triangleProcessResult.value);
    } no cap {
        vibez.spill("Error processing triangle: " + triangleProcessResult.err.error());
    }
}