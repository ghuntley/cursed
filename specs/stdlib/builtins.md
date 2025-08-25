# Built-in Functions

## Overview
Built-in functions are globally available functions that don't require importing any modules. They provide fundamental operations for data manipulation, memory management, and basic I/O.

## Core Built-ins

### Memory Management

```cursed
fr fr Creates a new instance of T and returns a pointer to it
slay new<T>() *T

fr fr Allocates memory for a slice, map, or channel
slay make<T>(size normie) T                    // For slices and maps
slay make<T>(size normie, capacity normie) T   // For slices with capacity
slay make<T>(buffer_size normie) T             // For channels
```

### Collection Operations

```cursed
fr fr Returns the length of a string, slice, array, map, or channel
slay len<T>(collection T) normie

fr fr Returns the capacity of a slice or channel
slay cap<T>(collection T) normie

fr fr Appends elements to a slice and returns the new slice
slay append<T>(slice []T, elements ...T) []T

fr fr Deletes an element from a map
slay delete<K, V>(map[K]V, key K)

fr fr Copies elements from source to destination slice
slay copy<T>(dest []T, src []T) normie
```

### Channel Operations

```cursed
fr fr Closes a channel
slay close<T>(channel dm<T>)

fr fr Checks if a channel is closed (returns value, ok)
slay receive<T>(channel dm<T>) (T, lit)
```

### Error Handling

```cursed
fr fr Triggers a panic with the given message
slay panic(message interface{})

fr fr Recovers from a panic and returns the panic value
slay recover() interface{}
```

### Basic I/O

```cursed
fr fr Prints values to stdout without newline
slay print(values ...interface{})

fr fr Prints values to stdout with newline
slay println(values ...interface{})
```

### Type Operations

```cursed
fr fr Type assertion (returns value, ok)
slay assert<T>(value interface{}) (T, lit)

fr fr Returns the type of a value as a string
slay typeof(value interface{}) tea
```

### Advanced Operations

```cursed
fr fr Performs a deep copy of a value
slay deepcopy<T>(value T) T

fr fr Checks if two values are equal (deep comparison)
slay equals<T>(a T, b T) lit

fr fr Compares two values (-1, 0, or 1)
slay compare<T>(a T, b T) normie
```

## Usage Examples

```cursed
fr fr Memory allocation
ptr := new<normie>()
*ptr = 42

fr fr Slice operations
slice := make<[]normie>(10, 20)
slice = append(slice, 1, 2, 3)
vibez.spill("Length:", len(slice), "Capacity:", cap(slice))

fr fr Map operations
m := make<map[tea]normie>()
m["key"] = 42
delete(m, "key")

fr fr Channel operations
ch := make<dm<normie>>(10)
dm_send(ch, 42
value, ok := receive(ch)
close(ch)

fr fr Error handling
defer slay() {
    ready recovered := recover(); recovered != cringe {
        vibez.spill("Recovered from panic:", recovered)
    }
}()
panic("Something went wrong!")

fr fr Type operations
sus value interface{} = 42
number, ok := assert<normie>(value)
vibez.spill("Type:", typeof(value))
```

## Implementation Guidelines

1. **Thread Safety**: All built-ins must be thread-safe
2. **Performance**: Built-ins should be optimized for common use cases
3. **Error Handling**: Provide clear error messages for invalid operations
4. **Type Safety**: Ensure type parameters are validated at compile time
5. **Memory Management**: Proper cleanup and GC integration
6. **Compatibility**: Maintain Go-like semantics where appropriate
