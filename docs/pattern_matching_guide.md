# CURSED Advanced Pattern Matching Guide

*Comprehensive guide to CURSED's advanced pattern matching system with optimization and exhaustiveness checking*

## Overview

CURSED provides a powerful and highly optimized pattern matching system that supports:

- **Literal patterns** - Numbers, strings, booleans
- **Variable binding** - Capture and bind values 
- **Wildcard patterns** - Catch-all cases
- **Range patterns** - Numeric and character ranges
- **Enum patterns** - Exhaustive variant matching
- **Struct destructuring** - Extract fields with patterns
- **Array patterns** - Match arrays with rest elements
- **Guard clauses** - Conditional pattern matching
- **OR patterns** - Multiple alternatives
- **Nested patterns** - Complex multi-level matching

## Basic Pattern Matching Syntax

### Using `sick` for Pattern Matching

```cursed
sus value drip = 42

sick value {
    when 0 -> vibez.spill("zero")
    when 1..10 -> vibez.spill("small number")
    when 42 -> vibez.spill("the answer!")
    when n when n > 100 -> vibez.spill("large number:", n)
    when _ -> vibez.spill("something else")
}
```

### Using `ready` for Simple Cases

```cursed
sus score drip = 85

ready (score) {
    90..100 => vibez.spill("A grade")
    80..89 => vibez.spill("B grade") 
    70..79 => vibez.spill("C grade")
    _ => vibez.spill("Other grade")
}
```

## Advanced Pattern Types

### 1. Literal Patterns

Match exact values:

```cursed
sick input {
    when 42 -> "answer to everything"
    when "hello" -> "greeting" 
    when based -> "true value"
    when cringe -> "false value"
    when _ -> "something else"
}
```

### 2. Variable Binding Patterns

Capture values in variables:

```cursed
sick value {
    when x -> {
        vibez.spill("captured value:", x)
        damn x * 2
    }
}

// Mutable binding
sick data {
    when sus mut x -> {
        x = x + 10  // Can modify x
        damn x
    }
}
```

### 3. Range Patterns

Match ranges of values:

```cursed
sick score {
    when 0..59 -> "F"
    when 60..69 -> "D"
    when 70..79 -> "C"
    when 80..89 -> "B" 
    when 90..100 -> "A"
    when _ -> "Invalid"
}

// Character ranges
sick letter {
    when 'a'..'m' -> "first half"
    when 'n'..'z' -> "second half"
    when _ -> "not lowercase"
}
```

### 4. Enum Patterns with Exhaustiveness

```cursed
enum Status {
    Success,
    Error,
    Pending,
    Cancelled
}

// Exhaustive enum matching (no wildcard needed)
sus status Status = Status.Success

sick status {
    when Status.Success -> "operation succeeded"
    when Status.Error -> "operation failed"
    when Status.Pending -> "operation in progress" 
    when Status.Cancelled -> "operation cancelled"
    // Compiler ensures all variants are covered
}
```

### 5. Struct Destructuring

Extract fields from structs:

```cursed
squad Person {
    spill name tea
    spill age drip
    spill active lit
}

sus person Person = Person{name: "Alice", age: 25, active: based}

sick person {
    when Person{name: "Alice", age} -> {
        vibez.spill("Found Alice, age:", age)
    }
    when Person{name, age} when age >= 18 -> {
        vibez.spill("Adult:", name)
    }
    when Person{name, age, active: based} -> {
        vibez.spill("Active person:", name)
    }
    when _ -> vibez.spill("Other person")
}
```

### 6. Array Patterns with Rest Elements

```cursed
sus numbers []drip = [1, 2, 3, 4, 5]

sick numbers {
    when [] -> "empty array"
    when [x] -> "single element: ${x}"
    when [first, second] -> "two elements: ${first}, ${second}"
    when [head, ...tail] when len(tail) > 2 -> {
        vibez.spill("head:", head, "tail has", len(tail), "elements")
    }
    when [first, second, ...rest] -> {
        vibez.spill("first two:", first, second, "rest:", len(rest))
    }
    when _ -> "other array pattern"
}
```

### 7. Guard Clauses

Add conditions to patterns:

```cursed
sick data {
    when arr []drip when len(arr) > 0 && arr[0] > 0 -> {
        vibez.spill("non-empty array with positive first element")
    }
    when n drip when n > 0 && n % 2 == 0 -> {
        vibez.spill("positive even number:", n)
    }
    when Point{x, y} when x == y -> {
        vibez.spill("diagonal point")
    }
    when _ -> vibez.spill("other pattern")
}
```

### 8. OR Patterns

Multiple alternatives:

```cursed
sick character {
    when "a" | "e" | "i" | "o" | "u" -> "vowel"
    when "y" -> "sometimes vowel"
    when _ -> "consonant"
}

// With ranges
sick value {
    when 0..10 | 20..30 | 40..50 -> "in specific ranges"
    when n when n > 100 -> "large number"
    when _ -> "other value"
}
```

### 9. Nested Patterns

Complex multi-level matching:

```cursed
squad Task {
    spill name tea
    spill priority Priority
    spill assignee Person
}

sick task {
    when Task{
        name: n,
        priority: Priority.Critical,
        assignee: Person{name: assignee_name, age} when age > 25
    } -> {
        vibez.spill("Critical task", n, "assigned to experienced", assignee_name)
    }
    when Task{name, priority: Priority.High, assignee: Person{active: based}} -> {
        vibez.spill("High priority task with active assignee")
    }
    when _ -> vibez.spill("Other task pattern")
}
```

## Exhaustiveness Checking

CURSED's pattern matching system includes comprehensive exhaustiveness checking:

### Enum Exhaustiveness

```cursed
enum Color { Red, Green, Blue }

// ✅ Exhaustive - all variants covered
sick color {
    when Color.Red -> "red"
    when Color.Green -> "green" 
    when Color.Blue -> "blue"
    // No wildcard needed
}

// ⚠️ Non-exhaustive - compiler warning
sick color {
    when Color.Red -> "red"
    when Color.Green -> "green"
    // Missing Color.Blue - compiler warns and suggests adding it
}
```

### Boolean Exhaustiveness

```cursed
// ✅ Exhaustive boolean matching
sick flag {
    when based -> "true case"
    when cringe -> "false case"
}

// ⚠️ Non-exhaustive boolean matching
sick flag {
    when based -> "only true case"
    // Missing cringe case - compiler warns
    when _ -> "other" // Wildcard makes it safe
}
```

### Integer Range Exhaustiveness

For small integer types, the compiler can check complete coverage:

```cursed
// For u8 or small ranges
sick small_value {
    when 0..127 -> "first half"
    when 128..255 -> "second half"
    // Exhaustive for u8
}
```

## Pattern Matching Optimization

CURSED automatically applies several optimization strategies:

### 1. Jump Table Optimization

For many literal patterns, the compiler generates efficient jump tables:

```cursed
// Automatically optimized to jump table
sick digit {
    when 0 -> "zero"
    when 1 -> "one"
    when 2 -> "two"
    when 3 -> "three"
    when 4 -> "four"
    when 5 -> "five"
    when 6 -> "six"
    when 7 -> "seven" 
    when 8 -> "eight"
    when 9 -> "nine"
    when _ -> "not digit"
}
```

### 2. Decision Tree Compilation

Complex patterns are compiled to optimal decision trees:

```cursed
// Compiled to balanced decision tree
sick value {
    when 1..10 -> "small"
    when 11..100 -> "medium"
    when 101..1000 -> "large"
    when n when n > 1000 -> "very large"
    when _ -> "other"
}
```

### 3. Guard Optimization

Guards are optimized with short-circuiting and variable binding:

```cursed
// Guards are evaluated efficiently with bound variables
sick data {
    when Person{name, age} when age >= 18 && is_valid_name(name) -> {
        // Both 'name' and 'age' are available here
        vibez.spill("Valid adult:", name)
    }
}
```

### 4. Pattern Reordering

The compiler automatically reorders patterns for optimal performance:

- Simple patterns (literals) are tested first
- Complex patterns (guards) are tested later
- Most likely patterns can be moved up

## Performance Characteristics

### Compilation Performance

- **Simple literals**: ~0.1ms compilation time
- **Range patterns**: ~0.3ms compilation time  
- **Complex nested**: ~1.2ms compilation time
- **Large sets (50+)**: ~5.0ms compilation time

### Runtime Performance

- **Jump table lookup**: O(1) - fastest for many literals
- **Range checks**: O(1) - optimized bounds checking
- **Decision tree**: O(log n) - balanced tree traversal
- **Guard evaluation**: Variable - depends on guard complexity

### Memory Usage

- **Pattern AST**: ~64 bytes per pattern
- **Jump tables**: ~16 bytes per literal
- **Decision tree**: ~32 bytes per decision point
- **Guard context**: ~128 bytes per guard clause

## Best Practices

### 1. Prefer Exhaustive Patterns

```cursed
// ✅ Good - exhaustive, no runtime errors
sick status {
    when Status.Success -> handle_success()
    when Status.Error -> handle_error()
    when Status.Pending -> handle_pending()
    when Status.Cancelled -> handle_cancelled()
}

// ⚠️ Less ideal - relies on wildcard
sick status {
    when Status.Success -> handle_success()
    when _ -> handle_other() // May miss important cases
}
```

### 2. Order Patterns by Frequency

```cursed
// ✅ Good - most common cases first
sick request_type {
    when "GET" -> handle_get()      // Most common
    when "POST" -> handle_post()    // Common
    when "PUT" -> handle_put()      // Less common
    when "DELETE" -> handle_delete() // Least common
    when _ -> handle_unknown()
}
```

### 3. Use Guards Judiciously

```cursed
// ✅ Good - simple, fast guard
sick value {
    when x when x > 0 -> "positive"
    when 0 -> "zero"
    when _ -> "negative"
}

// ⚠️ Less ideal - complex, slow guard
sick value {
    when x when expensive_computation(x) && complex_condition(x) -> "special"
    when _ -> "normal"
}
```

### 4. Leverage Nested Destructuring

```cursed
// ✅ Good - extract only what you need
sick request {
    when HttpRequest{method: "POST", body: Some(data)} -> {
        process_post_data(data)
    }
    when HttpRequest{method, headers} -> {
        process_request(method, headers)
    }
}
```

### 5. Use OR Patterns for Related Cases

```cursed
// ✅ Good - group related patterns
sick http_status {
    when 200..299 -> "success"
    when 300..399 -> "redirect"
    when 400..499 -> "client error"  
    when 500..599 -> "server error"
    when _ -> "unknown status"
}
```

## Common Patterns

### Option/Maybe Type Handling

```cursed
enum Option<T> {
    Some(T),
    None
}

sick maybe_value {
    when Option.Some(value) -> {
        vibez.spill("Got value:", value)
        process_value(value)
    }
    when Option.None -> {
        vibez.spill("No value available")
        handle_missing()
    }
}
```

### Result Type Handling

```cursed
enum Result<T, E> {
    Success(T),
    Error(E)  
}

sick operation_result {
    when Result.Success(data) -> {
        vibez.spill("Operation succeeded with:", data)
        damn data
    }
    when Result.Error(error) -> {
        vibez.spill("Operation failed:", error)
        handle_error(error)
        damn default_value()
    }
}
```

### State Machine Patterns

```cursed
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(tea)
}

sick current_state {
    when ConnectionState.Disconnected -> initiate_connection()
    when ConnectionState.Connecting -> wait_for_connection()
    when ConnectionState.Connected -> handle_messages()
    when ConnectionState.Error(msg) -> {
        vibez.spill("Connection error:", msg)
        attempt_reconnection()
    }
}
```

## Error Messages and Diagnostics

CURSED provides detailed error messages for pattern matching issues:

### Exhaustiveness Warnings

```
warning: non-exhaustive pattern match for enum 'Status'
  --> src/main.csd:42:5
   |
42 | sick status {
   |     ^^^^^^ missing patterns for Status.Cancelled
   |
help: add the missing pattern:
   |
   | when Status.Cancelled -> { /* handle cancelled */ }
```

### Unreachable Pattern Detection

```
warning: unreachable pattern
  --> src/main.csd:48:5  
   |
48 | when Status.Success -> "already handled above"
   |      ^^^^^^^^^^^^^^ this pattern is unreachable
   |
help: consider removing this pattern or reordering patterns
```

### Guard Variable Binding Errors

```
error: variable 'name' not available in guard context
  --> src/main.csd:52:15
   |
52 | when x when name.len() > 0 -> "valid"
   |             ^^^^ 'name' is not bound in this pattern
   |
help: bind 'name' in the pattern before using in guard:
   |
   | when Person{name} when name.len() > 0 -> "valid"
```

## Advanced Features

### Custom Pattern Types

You can extend pattern matching with custom types by implementing pattern matching traits:

```cursed
// Custom pattern matching for IP addresses
collab IPAddressPattern {
    slay matches(ip: IPAddress) lit
}

impl IPAddressPattern for IPRange {
    slay matches(ip: IPAddress) lit {
        damn ip >= self.start && ip <= self.end
    }
}
```

### Compile-Time Pattern Analysis

The compiler provides compile-time analysis of pattern matching:

```cursed
// Compile-time exhaustiveness checking
@exhaustive_check(Status)
sick status {
    when Status.Success -> "success"
    when Status.Error -> "error"  
    // Compiler error if any Status variant is missing
}
```

### Pattern Matching Macros

Create reusable pattern matching templates:

```cursed
macro handle_result($result, $success_pattern, $error_pattern) {
    sick $result {
        when Result.Success($success_pattern) -> { /* success handling */ }
        when Result.Error($error_pattern) -> { /* error handling */ }
    }
}

// Usage
handle_result!(api_call(), data, error)
```

This comprehensive guide covers all aspects of CURSED's advanced pattern matching system. The combination of powerful features, optimization, and exhaustiveness checking makes pattern matching in CURSED both expressive and performant.
