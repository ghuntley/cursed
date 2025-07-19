vibe main;

fr fr Basic errors interface
collab Error {
    slay error() tea;
}

fr fr Enhanced type assertion error with file system source location
squad TypeAssertionError {
    expected tea,
    actual tea,
    file tea,
    line lit,
    column lit,
    source_line tea,
    source_file_content tea,  // New: store file content for context
    context_range tea[]lit   // New: store line ranges for context
}

fr fr Implement Error interface for TypeAssertionError
slay (e TypeAssertionError) error() tea {
    return "Type assertion failed at " + e.file + ":" + e.line + "," + e.column + ": "
        + "expected " + e.expected + " but got " + e.actual + "\n\n"
        + "Source context:\n" + format_source_with_marker(e.source_file_content, e.line, e.column, e.context_range);
}

fr fr Helper function to format source code with line markers
slay format_source_with_marker(source tea, line lit, column lit, context_range tea[]lit) tea {
    sus lines = split_lines(source);
    sus result = "";
    
    // Get line range
    sus start_line = 0;
    sus end_line = lines.length - 1;
    
    lowkey context_range.length == 2 {
        start_line = context_range[0];
        end_line = context_range[1];
    }
    
    // Format each line in the range
    periodt i := start_line; i <= end_line; i++ {
        lowkey i < lines.length {
            sus prefix = i == line ? ">" : " ";
            sus line_num = i + 1; // Convert to 1-indexed for display
            sus line_content = lines[i];
            
            // Add the line with prefix
            result = result + prefix + " " + pad_right(line_num, 4) + " | " + line_content + "\n";
            
            // Add marker line for the error location
            lowkey i == line {
                sus marker = "  " + "     " + "| " + string_repeat(" ", column) + "^\n";
                result = result + marker;
            }
        }
    }
    
    return result;
}

fr fr Helper to pad a number with spaces on the right
slay pad_right(num lit, width lit) tea {
    sus str = num + "";
    periodt str.length < width {
        str = str + " ";
    }
    return str;
}

fr fr Helper to split a string into lines
slay split_lines(text tea) tea[] {
    sus result = make(tea[], 0);
    sus line = "";
    
    periodt i := 0; i < text.length; i++ {
        sus ch = text[i];
        lowkey ch == "\n" {
            result = append(result, line);
            line = "";
        } no cap {
            line = line + ch;
        }
    }
    
    // Add the last line if it's not empty
    lowkey line.length > 0 {
        result = append(result, line);
    }
    
    return result;
}

fr fr Helper to repeat a string
slay string_repeat(str tea, count lit) tea {
    sus result = "";
    periodt i := 0; i < count; i++ {
        result = result + str;
    }
    return result;
}

fr fr Basic Result type similar to Rust's Result
fr fr Contains either a value or an error
squad Result<T, E> {
    value T,
    err E,
    isOk lit
}

fr fr Helper to create a successful result
slay ok<T, E>(value T) Result<T, E> {
    return Result<T, E>{
        value: value,
        err: nofr as E,
        isOk: 1
    };
}

fr fr Helper to create an error result
slay fail<T, E>(err E) Result<T, E> {
    return Result<T, E>{
        value: nofr as T,
        err: err,
        isOk: 0
    };
}

fr fr Shape interface for geometric shapes
collab Shape {
    slay area() meal;
    slay name() tea;
}

fr fr Circle implements Shape
squad Circle {
    radius meal,
}

slay (c Circle) area() meal {
    return 3.14159 * c.radius * c.radius;
}

slay (c Circle) name() tea {
    return "Circle";
}

fr fr Rectangle implements Shape
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

fr fr Function that uses type assertion with ? operator and filesystem source location
fr fr It returns Result<meal, Error> and uses ? to propagate errors
slay calculateCircleArea(shape Shape) Result<meal, Error> {
    // Try to assert shape as Circle using ? operator
    // If assertion fails, ? will return early with the error that includes source location
    sus circle = shape.(Circle)?;
    
    // If we get here, the assertion succeeded
    // Calculate and return area with radius * 2 (diameter)
    return ok<meal, Error>(circle.radius * 2);
}

fr fr Function that chains multiple assertions with ? operator and enhanced source location
slay processShape(shape Shape) Result<tea, Error> {
    // This will have precise source location in the error if it fails
    sus details = calculateCircleArea(shape)?;
    
    vibez.spill("Area details: " + details);
    
    // Return a formatted message
    return ok<tea, Error>("Shape has diameter: " + details);
}

fr fr Helper function to verify the shape is a Circle before processing
slay verifyCircle(shape Shape) Result<Circle, Error> {
    // This line will have exact file, line, and column information in errors
    return ok<Circle, Error>(shape.(Circle)?);
}

fr fr Main function to demonstrate filesystem-enhanced source location in error reporting
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
        // This will include detailed source location with file context
        vibez.spill("Error: " + circleResult.err.error());
    }
    
    // Process circle with more complex function
    sus circleProcessResult = processShape(circle);
    lowkey circleProcessResult.isOk {
        vibez.spill(circleProcessResult.value);
    } no cap {
        vibez.spill("Error processing circle: " + circleProcessResult.err.error());
    }
    
    // Try with Rectangle (should fail with enhanced filesystem source location)
    shape = rectangle;
    vibez.spill("\nProcessing Rectangle:");
    
    // First verify it's a circle (will fail)
    sus verified = verifyCircle(shape);
    lowkey verified.isOk {
        vibez.spill("Verified circle: " + verified.value.radius);
    } no cap {
        // This shows enhanced error with source location and file context
        vibez.spill("Error: " + verified.err.error());
    }
    
    // This will also fail but with a different source location
    sus rectResult = calculateCircleArea(shape);
    lowkey rectResult.isOk {
        vibez.spill("Rectangle calculation result: " + rectResult.value);
    } no cap {
        // This shows enhanced error with source location and file context
        vibez.spill("Error: " + rectResult.err.error());
    }
}