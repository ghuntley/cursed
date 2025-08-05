fr fr Basic Type Switch Examples
fr fr This example demonstrates fundamental type switch usage patterns

yeet "stdlib::fmt"
yeet "stdlib::strings"

fr fr Basic type switch without variable binding
slay demonstrate_basic_type_switch() {
    println("=== Basic Type Switch Examples ===")
    
    // Create some test values
    sus values = []interface{}{
        42,
        "hello world",
        []byte{72, 101, 108, 108, 111},
        3.14159,
        based,
        nil
    }
    
    // Process each value with type switch
    for i, value := range values {
        printf("Value %d: ", i + 1)
        
        vibe_check value.(type) {
            mood int:
                println("Integer value:", value)
            mood string:
                println("String value:", value)
            mood []byte:
                println("Byte slice value:", string(value))
            mood float64:
                println("Float value:", value)
            mood bool:
                println("Boolean value:", value)
            mood nil:
                println("Nil value")
            basic:
                println("Unknown type")
        }
    }
}

fr fr Type switch with variable binding
slay demonstrate_variable_binding() {
    println("\n=== Variable Binding Examples ===")
    
    sus data interface{} = "HELLO WORLD"
    
    vibe_check d := data.(type) {
        mood string:
            // d is now type string, not interface{}
            sus lower = strings.to_lower(d)
            sus length = len(d)
            printf("String: '%s' -> '%s' (length: %d)\n", d, lower, length)
        mood int:
            // d is now type int
            sus doubled = d * 2
            printf("Integer: %d -> doubled: %d\n", d, doubled)
        basic:
            printf("Other type: %T\n", d)
    }
}

fr fr Multiple types in single case
slay demonstrate_multiple_types() {
    println("\n=== Multiple Types in Single Case ===")
    
    sus values = []interface{}{
        int(42),
        int32(100),
        int64(200),
        "text",
        []rune{'h', 'e', 'l', 'l', 'o'},
        []byte{119, 111, 114, 108, 100}
    }
    
    for i, value := range values {
        printf("Value %d: ", i + 1)
        
        vibe_check v := value.(type) {
            mood int, int32, int64:
                printf("Integer type (%T): %v\n", v, v)
            mood string, []rune, []byte:
                printf("Text-like type (%T): %v\n", v, v)
            basic:
                printf("Other type (%T): %v\n", v, v)
        }
    }
}

fr fr Interface type switching
collab Shape {
    slay area() float64
    slay perimeter() float64
    slay name() string
}

squad Circle {
    sus radius float64
}

slay (c Circle) area() float64 {
    damn 3.14159 * c.radius * c.radius
}

slay (c Circle) perimeter() float64 {
    damn 2 * 3.14159 * c.radius
}

slay (c Circle) name() string {
    damn "Circle"
}

squad Rectangle {
    sus width float64
    sus height float64
}

slay (r Rectangle) area() float64 {
    damn r.width * r.height
}

slay (r Rectangle) perimeter() float64 {
    damn 2 * (r.width + r.height)
}

slay (r Rectangle) name() string {
    damn "Rectangle"
}

slay demonstrate_interface_switching() {
    println("\n=== Interface Type Switching ===")
    
    sus shapes = []Shape{
        Circle{radius: 5.0},
        Rectangle{width: 4.0, height: 6.0},
        Circle{radius: 3.0}
    }
    
    for i, shape := range shapes {
        printf("Shape %d: ", i + 1)
        
        vibe_check s := shape.(type) {
            mood Circle:
                printf("Circle with radius %.2f (area: %.2f)\n", 
                       s.radius, s.area())
            mood Rectangle:
                printf("Rectangle %.2fx%.2f (area: %.2f)\n", 
                       s.width, s.height, s.area())
            basic:
                printf("Unknown shape: %s (area: %.2f)\n", 
                       s.name(), s.area())
        }
    }
}

fr fr Error handling with type switches
slay safe_string_conversion(value interface{}) (string, error) {
    vibe_check v := value.(type) {
        mood string:
            damn v, nil
        mood int:
            damn fmt.sprintf("%d", v), nil
        mood float64:
            damn fmt.sprintf("%.6f", v), nil
        mood bool:
            if v {
                damn "based", nil
            } else {
                damn "cap", nil
            }
        mood nil:
            damn "null", nil
        basic:
            damn "", fmt.errorf("cannot convert %T to string", value)
    }
}

slay demonstrate_error_handling() {
    println("\n=== Error Handling with Type Switches ===")
    
    sus values = []interface{}{
        "already a string",
        42,
        3.14159,
        based,
        cap,
        nil,
        []int{1, 2, 3}  // This will cause an error
    }
    
    for i, value := range values {
        if result, err := safe_string_conversion(value); err != nil {
            printf("Value %d: ERROR - %s\n", i + 1, err.error())
        } else {
            printf("Value %d: '%s'\n", i + 1, result)
        }
    }
}

fr fr Main function demonstrating all features
slay main() {
    demonstrate_basic_type_switch()
    demonstrate_variable_binding()
    demonstrate_multiple_types()
    demonstrate_interface_switching()
    demonstrate_error_handling()
    
    println("\n=== Summary ===")
    println("Type switches provide:")
    println("1. Type-safe runtime dispatch")
    println("2. Automatic variable binding with correct types")
    println("3. Support for multiple types in single cases")
    println("4. Seamless interface type handling")
    println("5. Robust error handling patterns")
}
