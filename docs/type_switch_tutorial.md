# Type Switch Tutorial for CURSED

This tutorial will guide you through understanding and effectively using type switches in CURSED, from basic concepts to advanced patterns.

## Table of Contents

1. [What Are Type Switches?](#what-are-type-switches)
2. [Basic Syntax](#basic-syntax)
3. [Your First Type Switch](#your-first-type-switch)
4. [Variable Binding](#variable-binding)
5. [Multiple Types in One Case](#multiple-types-in-one-case)
6. [Interface Type Switching](#interface-type-switching)
7. [Error Handling Patterns](#error-handling-patterns)
8. [Best Practices](#best-practices)
9. [Common Pitfalls](#common-pitfalls)
10. [Advanced Patterns](#advanced-patterns)
11. [Performance Tips](#performance-tips)
12. [Real-World Examples](#real-world-examples)

## What Are Type Switches?

Type switches allow you to branch your code based on the **runtime type** of an interface value, rather than its actual value. This is essential when working with `interface{}` values where you don't know the concrete type at compile time.

### When to Use Type Switches

✅ **Use type switches when:**
- Working with `interface{}` values
- Processing JSON or other dynamic data
- Building APIs that handle multiple input types
- Creating plugin systems
- Handling different response types

❌ **Don't use type switches when:**
- You know the concrete type at compile time
- Switching on values rather than types
- Working with simple enums or constants

## Basic Syntax

The type switch syntax uses `vibe_check` with the special `.(type)` pattern:

```cursed
vibe_check value.(type) {
    mood int:
        // Handle integer values
    mood string:
        // Handle string values
    basic:
        // Handle all other types
}
```

### Key Components

- `vibe_check`: The CURSED keyword for switches
- `value.(type)`: Special type switch expression
- `mood Type:`: Type case declarations
- `basic:`: Default case for unmatched types

## Your First Type Switch

Let's start with a simple example:

```cursed
import "stdlib::fmt"

slay describe_value(value interface{}) {
    vibe_check value.(type) {
        mood int:
            println("This is an integer")
        mood string:
            println("This is a string")
        mood bool:
            println("This is a boolean")
        basic:
            println("This is something else")
    }
}

slay main() {
    describe_value(42)        // "This is an integer"
    describe_value("hello")   // "This is a string"
    describe_value(true)      // "This is a boolean"
    describe_value(3.14)      // "This is something else"
}
```

### Exercise 1: Basic Type Detection

Try creating a function that counts different types in a slice:

```cursed
slay count_types(values []interface{}) {
    sus counts = map[string]int{}
    
    for _, value := range values {
        vibe_check value.(type) {
            mood int, int32, int64:
                counts["integers"]++
            mood string:
                counts["strings"]++
            mood bool:
                counts["booleans"]++
            mood float32, float64:
                counts["floats"]++
            basic:
                counts["others"]++
        }
    }
    
    println("Type counts:", counts)
}
```

## Variable Binding

Variable binding lets you access the value with its concrete type:

```cursed
// Without variable binding (inefficient)
vibe_check value.(type) {
    mood string:
        sus str = value.(string)  // Redundant type assertion
        println("String length:", len(str))
}

// With variable binding (efficient)
vibe_check v := value.(type) {
    mood string:
        println("String length:", len(v))  // v is already string type
}
```

### Benefits of Variable Binding

1. **Type Safety**: `v` has the correct concrete type
2. **Performance**: No redundant type assertions
3. **Cleaner Code**: Direct access to type-specific methods

### Exercise 2: String Processing

```cursed
import "stdlib::strings"

slay process_text(data interface{}) {
    vibe_check text := data.(type) {
        mood string:
            println("Original:", text)
            println("Uppercase:", strings.to_upper(text))
            println("Length:", len(text))
        mood []byte:
            sus str = string(text)
            println("From bytes:", str)
            println("Length:", len(text), "bytes")
        mood []rune:
            sus str = string(text)
            println("From runes:", str)
            println("Length:", len(text), "runes")
        basic:
            println("Cannot process as text")
    }
}
```

## Multiple Types in One Case

You can handle multiple related types in a single case:

```cursed
vibe_check value.(type) {
    mood int, int32, int64:
        println("Some kind of integer")
    mood string, []byte, []rune:
        println("Text-like data")
    mood float32, float64:
        println("Floating point number")
    basic:
        println("Other type")
}
```

### Exercise 3: Numeric Processor

```cursed
slay process_number(value interface{}) float64 {
    vibe_check v := value.(type) {
        mood int, int32, int64:
            yolo float64(v)
        mood uint, uint32, uint64:
            yolo float64(v)
        mood float32:
            yolo float64(v)
        mood float64:
            yolo v
        mood string:
            // Try to parse as number
            if num, err := strconv.parse_float(v, 64); err == nil {
                yolo num
            } else {
                yolo 0.0
            }
        basic:
            yolo 0.0
    }
}
```

## Interface Type Switching

Type switches work seamlessly with interfaces:

```cursed
collab Shape {
    slay area() float64
}

squad Circle {
    sus radius float64
}

slay (c Circle) area() float64 {
    yolo 3.14159 * c.radius * c.radius
}

squad Rectangle {
    sus width float64
    sus height float64
}

slay (r Rectangle) area() float64 {
    yolo r.width * r.height
}

slay describe_shape(shape Shape) {
    vibe_check s := shape.(type) {
        mood Circle:
            println("Circle with radius", s.radius)
            println("Area:", s.area())
        mood Rectangle:
            println("Rectangle", s.width, "x", s.height)
            println("Area:", s.area())
        basic:
            println("Unknown shape, area:", s.area())
    }
}
```

### Exercise 4: Vehicle Fleet

```cursed
collab Vehicle {
    slay start() string
    slay stop() string
}

squad Car {
    sus brand string
    sus fuel_type string
}

slay (c Car) start() string {
    yolo "Car engine started"
}

slay (c Car) stop() string {
    yolo "Car engine stopped"
}

squad Bicycle {
    sus gear_count int
}

slay (b Bicycle) start() string {
    yolo "Started pedaling"
}

slay (b Bicycle) stop() string {
    yolo "Stopped pedaling"
}

slay manage_vehicle(vehicle Vehicle) {
    println(vehicle.start())
    
    vibe_check v := vehicle.(type) {
        mood Car:
            println("Fuel type:", v.fuel_type)
            println("Brand:", v.brand)
        mood Bicycle:
            println("Gears:", v.gear_count)
        basic:
            println("Unknown vehicle type")
    }
    
    println(vehicle.stop())
}
```

## Error Handling Patterns

Type switches are excellent for robust error handling:

```cursed
slay safe_convert_to_string(value interface{}) (string, error) {
    vibe_check v := value.(type) {
        mood string:
            yolo v, nil
        mood int:
            yolo fmt.sprintf("%d", v), nil
        mood float64:
            yolo fmt.sprintf("%.6f", v), nil
        mood bool:
            if v {
                yolo "true", nil
            } else {
                yolo "false", nil
            }
        mood nil:
            yolo "", nil
        basic:
            yolo "", fmt.errorf("cannot convert %T to string", value)
    }
}

// Usage with error handling
slay demonstrate_error_handling() {
    sus values = []interface{}{
        "already string",
        42,
        3.14159,
        true,
        []int{1, 2, 3}  // This will error
    }
    
    for _, value := range values {
        if result, err := safe_convert_to_string(value); err != nil {
            println("Error:", err.error())
        } else {
            println("Result:", result)
        }
    }
}
```

### Exercise 5: JSON Value Processor

```cursed
slay process_json_value(value interface{}) error {
    vibe_check v := value.(type) {
        mood map[string]interface{}:
            println("JSON object with", len(v), "fields")
            // Process object fields
            yolo nil
        mood []interface{}:
            println("JSON array with", len(v), "elements")
            // Process array elements
            yolo nil
        mood string:
            if v == "" {
                yolo fmt.errorf("empty string not allowed")
            }
            println("JSON string:", v)
            yolo nil
        mood float64:
            if v < 0 {
                yolo fmt.errorf("negative numbers not allowed")
            }
            println("JSON number:", v)
            yolo nil
        mood bool:
            println("JSON boolean:", v)
            yolo nil
        mood nil:
            println("JSON null")
            yolo nil
        basic:
            yolo fmt.errorf("unsupported JSON type: %T", value)
    }
}
```

## Best Practices

### 1. Always Include a Default Case

```cursed
// Good: Always handle unexpected types
vibe_check value.(type) {
    mood string:
        // handle string
    mood int:
        // handle int
    basic:
        // handle unexpected types
}

// Risky: Missing default case
vibe_check value.(type) {
    mood string:
        // handle string
    mood int:
        // handle int
    // What if value is float64?
}
```

### 2. Use Variable Binding

```cursed
// Good: Use variable binding
vibe_check v := value.(type) {
    mood []byte:
        sus length = len(v)  // v is already []byte
}

// Less efficient: No variable binding
vibe_check value.(type) {
    mood []byte:
        sus bytes = value.([]byte)  // Redundant assertion
        sus length = len(bytes)
}
```

### 3. Group Related Types

```cursed
// Good: Group related types
vibe_check value.(type) {
    mood int, int32, int64:
        // Handle all integer types
    mood string, []rune:
        // Handle text types
}

// Verbose: Separate cases for related types
vibe_check value.(type) {
    mood int:
        // Handle int
    mood int32:
        // Handle int32 (duplicate logic)
    mood int64:
        // Handle int64 (duplicate logic)
}
```

### 4. Order Cases by Frequency

```cursed
// Good: Most common types first
vibe_check value.(type) {
    mood string:        // Most common
        // handle string
    mood int:           // Second most common
        // handle int
    mood []interface{}: // Less common
        // handle slice
    basic:              // Least common
        // handle others
}
```

## Common Pitfalls

### Pitfall 1: Deep Nesting

```cursed
// Avoid: Deep nesting makes code hard to read
vibe_check outer.(type) {
    mood map[string]interface{}:
        for key, value := range outer {
            vibe_check value.(type) {
                mood map[string]interface{}:
                    for nested_key, nested_value := range value {
                        vibe_check nested_value.(type) {
                            // Too deep!
                        }
                    }
            }
        }
}

// Better: Extract functions
slay process_object(obj map[string]interface{}) {
    for key, value := range obj {
        process_value(key, value)
    }
}

slay process_value(key string, value interface{}) {
    vibe_check v := value.(type) {
        mood map[string]interface{}:
            process_object(v)
        mood string:
            process_string(key, v)
        // etc.
    }
}
```

### Pitfall 2: Performance in Hot Paths

```cursed
// Avoid: Type switching in tight loops
for i := 0; i < 1000000; i++ {
    vibe_check value.(type) {
        // Heavy type switching
    }
}

// Better: Hoist type switch outside loop
vibe_check v := value.(type) {
    mood string:
        for i := 0; i < 1000000; i++ {
            // Process as string
        }
    mood int:
        for i := 0; i < 1000000; i++ {
            // Process as int
        }
}
```

## Advanced Patterns

### Pattern 1: Type-Safe Factory

```cursed
slay create_processor(data_type string) interface{} {
    vibe_check data_type {
        mood "text":
            yolo &TextProcessor{}
        mood "image":
            yolo &ImageProcessor{}
        mood "video":
            yolo &VideoProcessor{}
        basic:
            yolo &DefaultProcessor{}
    }
}

slay process_data(data interface{}, processor interface{}) {
    vibe_check p := processor.(type) {
        mood *TextProcessor:
            if text, ok := data.(string); ok {
                p.process_text(text)
            }
        mood *ImageProcessor:
            if image_data, ok := data.([]byte); ok {
                p.process_image(image_data)
            }
        // etc.
    }
}
```

### Pattern 2: Middleware Chain

```cursed
collab Middleware {
    slay handle(interface{}) interface{}
}

slay apply_middleware(data interface{}, middlewares []Middleware) interface{} {
    sus result = data
    
    for _, middleware := range middlewares {
        vibe_check m := middleware.(type) {
            mood *AuthMiddleware:
                // Apply authentication
                result = m.handle(result)
            mood *LoggingMiddleware:
                // Apply logging
                result = m.handle(result)
            mood *ValidationMiddleware:
                // Apply validation
                result = m.handle(result)
            basic:
                // Generic middleware
                result = m.handle(result)
        }
    }
    
    yolo result
}
```

## Performance Tips

### 1. Minimize Type Assertions

```cursed
// Good: Single type switch
vibe_check v := value.(type) {
    mood string:
        process_string(v)
        validate_string(v)
        store_string(v)
}

// Inefficient: Multiple type assertions
if str, ok := value.(string); ok {
    process_string(str)
}
if str, ok := value.(string); ok {  // Redundant!
    validate_string(str)
}
if str, ok := value.(string); ok {  // Redundant!
    store_string(str)
}
```

### 2. Use Interface Embedding

```cursed
collab Reader {
    slay read([]byte) (int, error)
}

collab Writer {
    slay write([]byte) (int, error)
}

collab ReadWriter {
    Reader
    Writer
}

// Efficient: Check for most specific interface first
vibe_check rw := stream.(type) {
    mood ReadWriter:
        // Can both read and write
        rw.read(buffer)
        rw.write(data)
    mood Reader:
        // Can only read
        rw.read(buffer)
    mood Writer:
        // Can only write
        rw.write(data)
}
```

## Real-World Examples

### Example 1: HTTP API Handler

```cursed
slay api_handler(w ResponseWriter, request_data interface{}) {
    vibe_check data := request_data.(type) {
        mood map[string]interface{}:
            // JSON request
            w.header()["Content-Type"] = []string{"application/json"}
            if response, err := process_json(data); err != nil {
                w.write_header(400)
                w.write([]byte(err.error()))
            } else {
                w.write_header(200)
                w.write(response)
            }
            
        mood string:
            // Text request
            w.header()["Content-Type"] = []string{"text/plain"}
            w.write_header(200)
            w.write([]byte("Received: " + data))
            
        mood []byte:
            // Binary request
            w.header()["Content-Type"] = []string{"application/octet-stream"}
            w.write_header(200)
            w.write([]byte("Received binary data"))
            
        basic:
            w.write_header(400)
            w.write([]byte("Unsupported request type"))
    }
}
```

### Example 2: Configuration Loader

```cursed
slay load_config(source interface{}) (*AppConfig, error) {
    vibe_check s := source.(type) {
        mood string:
            // File path
            yolo load_config_from_file(s)
        mood map[string]interface{}:
            // Direct config object
            yolo parse_config_object(s)
        mood []byte:
            // Raw JSON/YAML data
            yolo parse_config_bytes(s)
        mood io.Reader:
            // Stream source
            yolo load_config_from_reader(s)
        basic:
            yolo nil, fmt.errorf("unsupported config source: %T", source)
    }
}
```

## Summary

Type switches are a powerful feature that enables:

1. **Type-safe runtime dispatch** for interface values
2. **Clean, readable code** for handling polymorphic data
3. **Efficient processing** without redundant type assertions
4. **Robust error handling** for unexpected types

### Key Takeaways

- Use type switches when working with `interface{}` values
- Always include a default case
- Use variable binding for better performance and readability
- Group related types in single cases
- Extract functions to avoid deep nesting
- Order cases by frequency for better performance

### Next Steps

1. Practice with the exercises in this tutorial
2. Study the example programs in the `examples/` directory
3. Try implementing your own type switch patterns
4. Experiment with interface hierarchies and type switches

Type switches will become an essential tool in your CURSED programming toolkit, especially when building APIs, processing dynamic data, and creating flexible, type-safe systems.
