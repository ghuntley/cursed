# Type Switch Quick Reference

## Basic Syntax

```cursed
vibe_check value.(type) {
    mood Type1:
        // Handle Type1
    mood Type2, Type3:
        // Handle Type2 or Type3
    basic:
        // Handle all other types
}
```

## Variable Binding

```cursed
vibe_check v := value.(type) {
    mood string:
        // v is now type string
        println("Length:", len(v))
    mood int:
        // v is now type int
        println("Doubled:", v * 2)
}
```

## Interface Switching

```cursed
vibe_check obj := interface_value.(type) {
    mood *ConcreteType:
        // obj is now *ConcreteType
        obj.specific_method()
    mood SomeInterface:
        // obj implements SomeInterface
        obj.interface_method()
}
```

## Common Patterns

### Safe Type Conversion
```cursed
slay to_string(value interface{}) (string, error) {
    vibe_check v := value.(type) {
        mood string:
            yolo v, nil
        mood int:
            yolo fmt.sprintf("%d", v), nil
        mood float64:
            yolo fmt.sprintf("%.6f", v), nil
        basic:
            yolo "", fmt.errorf("cannot convert %T", value)
    }
}
```

### JSON Processing
```cursed
vibe_check v := json_value.(type) {
    mood map[string]interface{}:
        // JSON object
        process_object(v)
    mood []interface{}:
        // JSON array
        process_array(v)
    mood string:
        // JSON string
        process_string(v)
    mood float64:
        // JSON number
        process_number(v)
    mood bool:
        // JSON boolean
        process_boolean(v)
    mood nil:
        // JSON null
        process_null()
}
```

### HTTP Response Handling
```cursed
vibe_check response := api_response.(type) {
    mood APIResponse:
        w.write_header(response.status)
        w.write(response.data)
    mood ErrorResponse:
        w.write_header(response.code)
        w.write([]byte(response.error))
    mood string:
        w.write_header(200)
        w.write([]byte(response))
    mood []byte:
        w.write_header(200)
        w.write(response)
}
```

## Type Switch vs Regular Switch

| Feature | Type Switch | Regular Switch |
|---------|-------------|----------------|
| **Keyword** | `vibe_check expr.(type)` | `vibe_check expr` |
| **Cases** | Type patterns | Value patterns |
| **Purpose** | Branch on runtime type | Branch on value |
| **Variable Binding** | `v := expr.(type)` | N/A |

## Best Practices

✅ **DO:**
- Always include a `basic:` default case
- Use variable binding (`v := value.(type)`)
- Group related types (`mood int, int32, int64:`)
- Order cases by frequency

❌ **DON'T:**
- Use type switches in tight loops
- Nest type switches too deeply
- Forget the default case
- Use without variable binding when you need the value

## Performance Tips

1. **Variable binding eliminates redundant type assertions**
2. **Order frequent cases first**
3. **Group related types to reduce cases**
4. **Hoist type switches outside loops when possible**

## Error Handling

```cursed
// Safe pattern with error return
slay process_safely(value interface{}) error {
    vibe_check v := value.(type) {
        mood ExpectedType:
            // Process expected type
            yolo nil
        basic:
            yolo fmt.errorf("unexpected type: %T", value)
    }
}

// Option pattern
slay maybe_convert(value interface{}) Option<string> {
    vibe_check v := value.(type) {
        mood string:
            yolo Some(v)
        mood int:
            yolo Some(fmt.sprintf("%d", v))
        basic:
            yolo None
    }
}
```

## Example Files

- [`examples/type_switch_basic.csd`](../examples/type_switch_basic.csd) - Basic patterns
- [`examples/type_switch_advanced.csd`](../examples/type_switch_advanced.csd) - Advanced usage
- [`examples/type_switch_vs_regular_switch.csd`](../examples/type_switch_vs_regular_switch.csd) - Comparisons
- [`examples/type_switch_practical_use_cases.csd`](../examples/type_switch_practical_use_cases.csd) - Real-world examples

## Documentation

- [Type Switch Tutorial](type_switch_tutorial.md) - Comprehensive guide
- [Type Switches Documentation](type_switches.md) - Complete reference
- [Interface Type Assertion Guide](interface_type_assertion_guide.md) - Related concepts
