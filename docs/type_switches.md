# Type Switches in CURSED

Type switches are a powerful feature in CURSED that allow you to branch based on the runtime type of an interface value. They provide type-safe runtime dispatch and are essential for working with interface types effectively.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [Variable Binding](#variable-binding)
- [Type Switch vs Regular Switch](#type-switch-vs-regular-switch)
- [Interface Integration](#interface-integration)
- [Error Handling](#error-handling)
- [Practical Use Cases](#practical-use-cases)
- [Advanced Patterns](#advanced-patterns)
- [Performance Considerations](#performance-considerations)

## Basic Syntax

Type switches use the `vibe_check` keyword with the special `.(type)` syntax:

```cursed
vibe_check value.(type) {
    mood int:
        println("It's an integer!")
    mood string:
        println("It's a string!")
    mood []byte:
        println("It's a byte slice!")
    basic:
        println("Unknown type")
}
```

### Multiple Types in One Case

You can handle multiple types in a single case using comma separation:

```cursed
vibe_check value.(type) {
    mood int, int32, int64:
        println("It's some kind of integer")
    mood string, []rune:
        println("It's text-like data")
    basic:
        println("Something else")
}
```

## Variable Binding

Type switches can bind the value to a new variable with the correct type:

```cursed
vibe_check v := value.(type) {
    mood int:
        // v is now type int, not interface{}
        sus doubled = v * 2
        println(doubled)
    mood string:
        // v is now type string
        sus upper = strings.to_upper(v)
        println(upper)
    basic:
        // v remains interface{} type
        println("Unknown value:", v)
}
```

### Benefits of Variable Binding

- **Type Safety**: The bound variable has the correct concrete type
- **No Type Assertions**: Direct access to type-specific methods and operations
- **Better Performance**: Eliminates runtime type assertions in case bodies

## Type Switch vs Regular Switch

| Feature | Type Switch | Regular Switch |
|---------|-------------|----------------|
| **Purpose** | Branch on runtime type | Branch on value |
| **Syntax** | `vibe_check expr.(type)` | `vibe_check expr` |
| **Cases** | Type patterns (`mood int:`) | Value patterns (`mood 42:`) |
| **Variable Binding** | `v := expr.(type)` | N/A |
| **Interface Handling** | Native support | Requires type assertions |

### Regular Switch Example

```cursed
vibe_check status {
    mood "active":
        println("System is active")
    mood "inactive":
        println("System is inactive")
    basic:
        println("Unknown status")
}
```

### Type Switch Example

```cursed
vibe_check data.(type) {
    mood int:
        println("Numeric data")
    mood string:
        println("Text data")
    basic:
        println("Other data type")
}
```

## Interface Integration

Type switches work seamlessly with interface types and type assertions:

### Basic Interface Switching

```cursed
collab Shape {
    slay area() float64
    slay perimeter() float64
}

squad Circle {
    sus radius float64
}

slay (c Circle) area() float64 {
    yolo 3.14159 * c.radius * c.radius
}

slay (c Circle) perimeter() float64 {
    yolo 2 * 3.14159 * c.radius
}

slay handle_shape(shape Shape) {
    vibe_check s := shape.(type) {
        mood Circle:
            println("Circle with radius:", s.radius)
        mood Rectangle:
            println("Rectangle with dimensions:", s.width, s.height)
        basic:
            println("Unknown shape type")
    }
}
```

### Interface Hierarchy

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

slay process_io(io interface{}) {
    vibe_check stream := io.(type) {
        mood Reader:
            // Can only read
            sus buffer = make([]byte, 1024)
            stream.read(buffer)
        mood Writer:
            // Can only write
            stream.write([]byte("Hello"))
        mood ReadWriter:
            // Can both read and write
            sus buffer = make([]byte, 1024)
            stream.read(buffer)
            stream.write([]byte("Response"))
        basic:
            println("Not an I/O interface")
    }
}
```

## Error Handling

Type switches provide robust error handling patterns:

### Safe Type Conversion

```cursed
slay safe_convert_to_int(value interface{}) (int, error) {
    vibe_check v := value.(type) {
        mood int:
            yolo v, nil
        mood string:
            // Try to parse string as int
            if num, err := strconv.parse_int(v); err == nil {
                yolo num, nil
            } else {
                yolo 0, fmt.errorf("invalid number format: %s", v)
            }
        mood float64:
            yolo int(v), nil
        basic:
            yolo 0, fmt.errorf("cannot convert %T to int", value)
    }
}
```

### Error Propagation with Type Switches

```cursed
slay process_data(data interface{}) error {
    vibe_check d := data.(type) {
        mood []byte:
            if len(d) == 0 {
                yolo fmt.errorf("empty byte slice")
            }
            // Process byte data
        mood string:
            if d == "" {
                yolo fmt.errorf("empty string")
            }
            // Process string data
        mood nil:
            yolo fmt.errorf("nil data provided")
        basic:
            yolo fmt.errorf("unsupported data type: %T", data)
    }
    yolo nil
}
```

## Practical Use Cases

### 1. JSON Processing

```cursed
slay process_json_value(value interface{}) {
    vibe_check v := value.(type) {
        mood map[string]interface{}:
            println("JSON object with", len(v), "keys")
            for key, val := range v {
                println("  ", key, ":", val)
            }
        mood []interface{}:
            println("JSON array with", len(v), "elements")
            for i, item := range v {
                println("  [", i, "]:", item)
            }
        mood string:
            println("JSON string:", v)
        mood float64:
            println("JSON number:", v)
        mood bool:
            println("JSON boolean:", v)
        mood nil:
            println("JSON null")
        basic:
            println("Unknown JSON type")
    }
}
```

### 2. HTTP Handler

```cursed
collab ResponseWriter {
    slay write([]byte) (int, error)
    slay header() map[string][]string
    slay write_header(int)
}

slay api_handler(w ResponseWriter, data interface{}) {
    vibe_check response := data.(type) {
        mood string:
            w.header()["Content-Type"] = []string{"text/plain"}
            w.write([]byte(response))
        mood map[string]interface{}:
            w.header()["Content-Type"] = []string{"application/json"}
            if json_data, err := json.marshal(response); err == nil {
                w.write(json_data)
            } else {
                w.write_header(500)
                w.write([]byte("JSON encoding error"))
            }
        mood []byte:
            w.header()["Content-Type"] = []string{"application/octet-stream"}
            w.write(response)
        mood error:
            w.write_header(500)
            w.write([]byte(response.error()))
        basic:
            w.write_header(400)
            w.write([]byte("Unsupported response type"))
    }
}
```

### 3. Configuration Processing

```cursed
squad DatabaseConfig {
    sus host string
    sus port int
    sus database string
}

squad RedisConfig {
    sus addr string
    sus password string
    sus db int
}

slay setup_storage(config interface{}) {
    vibe_check cfg := config.(type) {
        mood DatabaseConfig:
            println("Setting up database connection to", cfg.host)
            // Initialize database connection
        mood RedisConfig:
            println("Setting up Redis connection to", cfg.addr)
            // Initialize Redis connection
        mood map[string]interface{}:
            // Handle generic config map
            if db_host, ok := cfg["database_host"]; ok {
                println("Generic database config detected")
            }
        basic:
            panic("Unsupported configuration type")
    }
}
```

### 4. Plugin System

```cursed
collab Plugin {
    slay name() string
    slay version() string
    slay execute(interface{}) interface{}
}

collab AudioPlugin {
    Plugin
    slay process_audio([]float32) []float32
}

collab VideoPlugin {
    Plugin
    slay process_video([][]byte) [][]byte
}

slay load_plugin(plugin Plugin, data interface{}) interface{} {
    vibe_check p := plugin.(type) {
        mood AudioPlugin:
            vibe_check audio_data := data.(type) {
                mood []float32:
                    yolo p.process_audio(audio_data)
                basic:
                    yolo fmt.errorf("AudioPlugin requires []float32 data")
            }
        mood VideoPlugin:
            vibe_check video_data := data.(type) {
                mood [][]byte:
                    yolo p.process_video(video_data)
                basic:
                    yolo fmt.errorf("VideoPlugin requires [][]byte data")
            }
        basic:
            // Use generic plugin interface
            yolo p.execute(data)
    }
}
```

## Advanced Patterns

### Nested Type Switches

```cursed
slay process_nested_data(outer interface{}) {
    vibe_check o := outer.(type) {
        mood map[string]interface{}:
            if inner, exists := o["data"]; exists {
                vibe_check i := inner.(type) {
                    mood []interface{}:
                        println("Found array with", len(i), "elements")
                    mood map[string]interface{}:
                        println("Found nested object")
                    basic:
                        println("Found primitive value:", i)
                }
            }
        mood []interface{}:
            for idx, item := range o {
                vibe_check item.(type) {
                    mood map[string]interface{}:
                        println("Array item", idx, "is an object")
                    basic:
                        println("Array item", idx, "is primitive")
                }
            }
        basic:
            println("Top level is primitive")
    }
}
```

### Type Switch with Channels

```cursed
slay message_processor(ch <-chan interface{}) {
    lowkey {
        vibe_check msg := <-ch.(type) {
            mood string:
                println("Text message:", msg)
            mood []byte:
                println("Binary message, length:", len(msg))
            mood error:
                println("Error message:", msg.error())
                bestie
            mood nil:
                println("Channel closed")
                yolo
            basic:
                println("Unknown message type")
        }
        yolo  // Yield to scheduler
    }
}
```

### Type-Safe Factory Pattern

```cursed
collab Serializable {
    slay serialize() []byte
    slay deserialize([]byte) error
}

slay create_serializer(format string) Serializable {
    vibe_check format {
        mood "json":
            yolo &JsonSerializer{}
        mood "xml":
            yolo &XmlSerializer{}
        mood "binary":
            yolo &BinarySerializer{}
        basic:
            yolo &DefaultSerializer{}
    }
}

slay serialize_data(data interface{}, format string) []byte {
    serializer := create_serializer(format)
    
    vibe_check d := data.(type) {
        mood Serializable:
            // Object can serialize itself
            yolo d.serialize()
        basic:
            // Use external serializer
            serializer.set_data(d)
            yolo serializer.serialize()
    }
}
```

## Performance Considerations

### Runtime Performance

- **Hash-based type checking**: O(1) type identification using FNV-1a hash
- **Branch prediction**: Modern CPUs optimize type switch branches effectively
- **Memory efficiency**: No heap allocations for type checking
- **Cache locality**: Type IDs are small integers that cache well

### Best Practices

1. **Order cases by frequency**: Put most common cases first
2. **Use variable binding**: Avoid redundant type assertions in case bodies
3. **Combine related types**: Use multi-type cases when possible
4. **Avoid deep nesting**: Prefer composition over deeply nested type switches

### Performance Example

```cursed
// Good: Variable binding eliminates repeated assertions
slay process_efficiently(data interface{}) {
    vibe_check d := data.(type) {
        mood string:
            sus upper = strings.to_upper(d)  // d is already string type
            sus length = len(d)              // No assertion needed
            yolo fmt.sprintf("%s (%d chars)", upper, length)
        mood int:
            yolo fmt.sprintf("Number: %d", d * 2)  // d is already int type
    }
}

// Less efficient: Multiple type assertions
slay process_inefficiently(data interface{}) {
    vibe_check data.(type) {
        mood string:
            sus str = data.(string)          // Redundant assertion
            sus upper = strings.to_upper(str)
            sus length = len(str)
            yolo fmt.sprintf("%s (%d chars)", upper, length)
        mood int:
            sus num = data.(int)             // Redundant assertion
            yolo fmt.sprintf("Number: %d", num * 2)
    }
}
```

## Common Pitfalls and Solutions

### Pitfall 1: Missing Default Case

```cursed
// Risky: No default case
vibe_check value.(type) {
    mood string:
        println("String value")
    mood int:
        println("Integer value")
    // What if value is float64?
}

// Better: Always include default
vibe_check value.(type) {
    mood string:
        println("String value")
    mood int:
        println("Integer value")
    basic:
        println("Unexpected type:", fmt.sprintf("%T", value))
}
```

### Pitfall 2: Forgetting Variable Binding

```cursed
// Inefficient: No variable binding
vibe_check data.(type) {
    mood []byte:
        sus length = len(data.([]byte))  // Redundant assertion
}

// Better: Use variable binding
vibe_check d := data.(type) {
    mood []byte:
        sus length = len(d)  // d is already []byte
}
```

### Pitfall 3: Type Switch in Hot Paths

```cursed
// Avoid in performance-critical code
for i := 0; i < 1000000; i++ {
    vibe_check value.(type) {
        // Heavy type switching in tight loop
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

## Conclusion

Type switches are a powerful feature that makes working with interface types safe, efficient, and expressive. They provide:

- **Type Safety**: Compile-time guarantees about type handling
- **Performance**: Efficient runtime dispatch without reflection overhead
- **Expressiveness**: Clean, readable code for complex type handling
- **Integration**: Seamless work with interfaces and type assertions

Use type switches when you need to handle multiple possible types of interface values, especially when each type requires different processing logic. They're essential for building robust, type-safe systems that work with dynamic data.
