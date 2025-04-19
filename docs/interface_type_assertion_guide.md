# Interface Type Assertion Guide

## Overview

Interface type assertions allow you to check if an interface value holds a specific concrete type and safely convert it back to that type. This is a crucial feature for working with interfaces effectively in CURSED.

## Syntax

```cursed
// Basic syntax
concreteValue, ok = interfaceValue.(ConcreteType)

// Checking only
_, hasType = interfaceValue.(ConcreteType)
```

## Usage Patterns

### Basic Type Assertion

```cursed
collab Stringer {
    toString() tea;
}

squad Person {
    name tea,
    age lit
}

slay (p Person) toString() tea {
    return p.name
}

slay processStringer(s Stringer) {
    // Type assertion to check and convert
    sus person, ok = s.(Person)
    
    if ok {
        // Now we can access Person-specific fields
        vibez.spill("Person: " + person.name + ", age: " + person.age)
    } else {
        // Just use the interface method
        vibez.spill("Unknown stringer: " + s.toString())
    }
}
```

### Type Switch Pattern

This pattern uses multiple type assertions to handle different concrete types differently:

```cursed
slay processValue(val any) {
    // Try to assert as different types
    sus str, isString = val.(tea)
    sus num, isInt = val.(lit)
    sus flt, isFloat = val.(meal)
    
    if isString {
        return "String: " + str
    } else if isInt {
        return "Integer: " + num
    } else if isFloat {
        return "Float: " + flt
    }
    
    return "Unknown type"
}
```

## Best Practices

### 1. Always Check the Success Flag

Always check the success flag (`ok` in the examples) before using the converted value to avoid null pointer errors:

```cursed
// Good practice
sus value, ok = interfaceValue.(ConcreteType)
if ok {
    // Safe to use value here
}

// Bad practice - may cause runtime errors
sus value = interfaceValue.(ConcreteType) // Don't do this!
value.someMethod() // May crash if assertion failed
```

### 2. Use Type Assertions Strategically

Type assertions should be used strategically, not as a replacement for proper interface design. Consider if you can solve the problem using interfaces instead:

```cursed
// Instead of multiple type assertions:
sus circle, isCircle = shape.(Circle)
sus rect, isRect = shape.(Rectangle)

// Consider adding a common method to the interface:
collab Shape {
    area() meal;
    name() tea; // Add method to identify the shape type
}
```

### 3. Group Related Type Assertions

Group related type assertions together to improve readability and performance:

```cursed
// Process different reader types
slay processReader(reader any) {
    // Group related type assertions
    sus fileReader, isFileReader = reader.(FileReader)
    sus networkReader, isNetworkReader = reader.(NetworkReader)
    sus bufferReader, isBufferReader = reader.(BufferReader)
    
    if isFileReader {
        // Handle file reader
    } else if isNetworkReader {
        // Handle network reader
    } else if isBufferReader {
        // Handle buffer reader
    }
}
```

## Error Handling

When a type assertion fails, the first returned value will be the zero value for the target type (nil for reference types). Always use the boolean flag to check for success.

```cursed
slay safeGetValue(val any) tea {
    sus str, ok = val.(tea)
    if !ok {
        return "Value is not a string"
    }
    return str
}
```

## Performance Considerations

1. **Caching**: The system caches type IDs to make repeated assertions more efficient.

2. **Multiple Assertions**: When possible, perform type assertions once and store the result rather than repeatedly asserting the same type.

```cursed
// Less efficient - repeated assertions
periodt i := 0; i < items.length; i++ {
    sus _, isSpecialType = items[i].(SpecialType)
    if isSpecialType {
        // Process special item
    }
}

// More efficient - single pass segregation
sus specialItems = make(tea[]SpecialType, 0)

periodt i := 0; i < items.length; i++ {
    sus specialItem, ok = items[i].(SpecialType)
    if ok {
        specialItems = append(specialItems, specialItem)
    }
}

// Now process all special items
periodt i := 0; i < specialItems.length; i++ {
    // Process without need for further assertions
}
```

## Common Patterns

### Filtering Collections by Type

```cursed
slay filterByType(items tea[]any) tea[]Person {
    sus result = make(tea[]Person, 0)
    
    periodt i := 0; i < items.length; i++ {
        sus person, ok = items[i].(Person)
        if ok {
            result = append(result, person)
        }
    }
    
    return result
}
```

### Type-Based Dispatch

```cursed
slay dispatch(handler any, message tea) tea {
    // Try different handler types
    sus textHandler, isTextHandler = handler.(TextHandler)
    sus jsonHandler, isJsonHandler = handler.(JsonHandler)
    sus binaryHandler, isBinaryHandler = handler.(BinaryHandler)
    
    if isTextHandler {
        return textHandler.processText(message)
    } else if isJsonHandler {
        return jsonHandler.processJson(message)
    } else if isBinaryHandler {
        return binaryHandler.processBinary(message)
    }
    
    return "Unknown handler type"
}
```

## Debugging Type Assertions

When a type assertion fails unexpectedly, enable debug mode to get detailed information about the actual and expected types.

```bash
# Enable debug logging for type assertions
export CURSED_DEBUG=1
./your_program
```

This will log detailed information about type assertions, including the expected type, actual type ID, and whether the assertion succeeded or failed.