# CURSED Language Reference

Complete reference for CURSED language syntax, semantics, and features.

## Table of Contents

1. [Lexical Structure](#lexical-structure)
2. [Data Types](#data-types)
3. [Variables and Constants](#variables-and-constants)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Pattern Matching](#pattern-matching)
7. [Structs and Enums](#structs-and-enums)
8. [Modules and Imports](#modules-and-imports)
9. [Error Handling](#error-handling)
10. [Concurrency](#concurrency)
11. [Memory Management](#memory-management)

## Lexical Structure

### Keywords
```cursed
# Variable declarations
sus          # Variable declaration
spill        # Struct field declaration

# Function definitions
slay         # Function definition
damn         # Return statement

# Control flow
ready        # If condition
otherwise    # Else clause
bestie       # While loop
sick         # Pattern matching switch
when         # Pattern matching case

# Literals
based        # Boolean true
cringe       # Boolean false

# Module system
yeet         # Import statement
squad        # Struct definition
collab       # Interface definition

# Error handling
yikes        # Error return
fam          # Error handling
shook        # Panic/abort

# Concurrency
vibe         # Goroutine spawn
chan         # Channel type
```

### Identifiers
- Start with letter or underscore
- Followed by letters, digits, or underscores
- Case-sensitive
- No reserved word conflicts

```cursed
# Valid identifiers
sus valid_name drip = 42
sus camelCase drip = 100
sus PascalCase drip = 200
sus _private drip = 300
```

### Comments
```cursed
# Single line comment

# Multi-line comments are multiple single-line comments
# This is line 1
# This is line 2
```

## Data Types

### Primitive Types

#### Integer Types
```cursed
# Signed integers
sus small drip = 42           # i32 (default integer)
sus big i64 = 1234567890

# Unsigned integers  
sus positive u32 = 100
sus large u64 = 18446744073709551615
```

#### Floating Point
```cursed
sus price f32 = 19.99
sus precise f64 = 3.141592653589793
```

#### Boolean
```cursed
sus is_active lit = based     # true
sus is_disabled lit = cringe  # false
```

#### String
```cursed
sus name tea = "Alice"
sus multiline tea = "Line 1\nLine 2"
sus escaped tea = "Quote: \"Hello\""
```

#### Character
```cursed
sus letter rune = 'A'
sus emoji rune = '🔥'
```

### Composite Types

#### Arrays
```cursed
# Fixed-size arrays
sus numbers [5]drip = [1, 2, 3, 4, 5]
sus names [3]tea = ["Alice", "Bob", "Charlie"]

# Dynamic arrays (slices)
sus dynamic []drip = [1, 2, 3]
dynamic = append(dynamic, 4)  # [1, 2, 3, 4]
```

#### Slices
```cursed
sus full []drip = [1, 2, 3, 4, 5]
sus slice []drip = full[1:4]  # [2, 3, 4]
sus prefix []drip = full[:3]  # [1, 2, 3]
sus suffix []drip = full[2:]  # [3, 4, 5]
```

#### Maps/Dictionaries
```cursed
sus scores map[tea]drip = {
    "Alice": 95,
    "Bob": 87,
    "Charlie": 92
}

# Access and modify
vibez.spill(scores["Alice"])  # 95
scores["David"] = 89
```

## Variables and Constants

### Variable Declaration
```cursed
# Basic declaration
sus count drip = 0
sus message tea = "Hello"

# Type inference
sus auto_int = 42        # Inferred as drip
sus auto_string = "hi"   # Inferred as tea

# Uninitialized variables
sus later drip
later = 100

# Multiple declarations
sus x drip = 1
sus y drip = 2
sus z drip = x + y
```

### Constants
```cursed
# Compile-time constants
const PI f64 = 3.141592653589793
const MAX_USERS drip = 1000
const APP_NAME tea = "CURSED App"
```

### Type Annotations
```cursed
# Explicit typing
sus age: drip = 25
sus name: tea = "Bob"
sus scores: []drip = [90, 85, 92]
```

## Functions

### Function Definition
```cursed
# Basic function
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

# Multiple parameters
slay add(x drip, y drip) drip {
    damn x + y
}

# No return value (void)
slay print_info(name tea, age drip) {
    vibez.spill("Name:", name)
    vibez.spill("Age:", age)
}
```

### Generic Functions
```cursed
# Generic function with type parameter
slay swap<T>(a T, b T) (T, T) {
    damn (b, a)
}

# Usage
sus (x, y) = swap(1, 2)        # x=2, y=1
sus (p, q) = swap("a", "b")    # p="b", q="a"
```

### Function Types and Higher-Order Functions
```cursed
# Function type alias
type Predicate<T> = slay(T) lit

# Higher-order function
slay filter<T>(items []T, pred Predicate<T>) []T {
    sus result []T = []
    for item in items {
        ready (pred(item)) {
            result = append(result, item)
        }
    }
    damn result
}

# Usage
slay is_positive(x drip) lit {
    damn x > 0
}

sus numbers []drip = [-2, -1, 0, 1, 2]
sus positive []drip = filter(numbers, is_positive)
```

### Closures
```cursed
# Closure with captured variables
slay make_adder(x drip) slay(drip) drip {
    damn slay(y drip) drip {
        damn x + y  # Captures x from outer scope
    }
}

sus add_five = make_adder(5)
sus result drip = add_five(10)  # 15
```

## Control Flow

### Conditional Statements
```cursed
# If-else
sus score drip = 85
ready (score >= 90) {
    vibez.spill("Excellent!")
} otherwise ready (score >= 80) {
    vibez.spill("Good!")
} otherwise ready (score >= 70) {
    vibez.spill("Fair")
} otherwise {
    vibez.spill("Needs improvement")
}

# Ternary-like expression
sus grade tea = ready (score >= 80) "Pass" otherwise "Fail"
```

### Loops
```cursed
# While loop
sus i drip = 0
bestie (i < 5) {
    vibez.spill("Iteration:", i)
    i = i + 1
}

# For loop (range)
for i in 0..5 {
    vibez.spill("Number:", i)
}

# For loop (array iteration)
sus items []tea = ["apple", "banana", "cherry"]
for item in items {
    vibez.spill("Item:", item)
}

# For loop with index
for i, item in items {
    vibez.spill("Index:", i, "Item:", item)
}

# Break and continue
for i in 0..10 {
    ready (i == 3) {
        continue  # Skip 3
    }
    ready (i == 7) {
        break     # Stop at 7
    }
    vibez.spill(i)
}
```

## Pattern Matching

### Basic Pattern Matching
```cursed
enum Status {
    Pending,
    Processing,
    Complete,
    Failed(tea)  # Enum with associated data
}

sus current Status = Processing

sick (current) {
    when Pending -> vibez.spill("Waiting...")
    when Processing -> vibez.spill("Working...")
    when Complete -> vibez.spill("Done!")
    when Failed(error) -> vibez.spill("Error:", error)
}
```

### Advanced Pattern Matching
```cursed
# Pattern matching with guards
sus value drip = 42
sick (value) {
    when 0 -> vibez.spill("Zero")
    when x ready (x > 0 && x < 10) -> vibez.spill("Small positive")
    when x ready (x >= 10 && x < 100) -> vibez.spill("Medium positive")
    when x ready (x >= 100) -> vibez.spill("Large positive")
    when _ -> vibez.spill("Negative")
}

# Array pattern matching
sus numbers []drip = [1, 2, 3]
sick (numbers) {
    when [] -> vibez.spill("Empty array")
    when [x] -> vibez.spill("Single element:", x)
    when [first, ...rest] -> {
        vibez.spill("First:", first)
        vibez.spill("Rest:", rest)
    }
}

# Struct pattern matching
squad Point { spill x drip; spill y drip }
sus point Point = Point{x: 0, y: 5}

sick (point) {
    when Point{x: 0, y: 0} -> vibez.spill("Origin")
    when Point{x: 0, y} -> vibez.spill("On Y-axis at", y)
    when Point{x, y: 0} -> vibez.spill("On X-axis at", x)
    when Point{x, y} ready (x == y) -> vibez.spill("On diagonal")
    when Point{x, y} -> vibez.spill("Point at", x, ",", y)
}
```

## Structs and Enums

### Struct Definition
```cursed
# Basic struct
squad Person {
    spill name tea
    spill age drip
    spill email tea
}

# Struct with methods
squad Rectangle {
    spill width drip
    spill height drip
}

# Method implementation
impl Rectangle {
    slay area(self) drip {
        damn self.width * self.height
    }
    
    slay perimeter(self) drip {
        damn 2 * (self.width + self.height)
    }
    
    slay scale(self mut, factor drip) {
        self.width = self.width * factor
        self.height = self.height * factor
    }
}
```

### Generic Structs
```cursed
# Generic struct
squad Container<T> {
    spill value T
    spill name tea
}

# Usage
sus int_container Container<drip> = Container{
    value: 42,
    name: "Number container"
}

sus string_container Container<tea> = Container{
    value: "Hello",
    name: "String container"
}
```

### Enums
```cursed
# Simple enum
enum Color {
    Red,
    Green,
    Blue,
    Custom(r u8, g u8, b u8)  # Enum with data
}

# Enum with methods
impl Color {
    slay to_hex(self) tea {
        sick (self) {
            when Red -> damn "#FF0000"
            when Green -> damn "#00FF00"
            when Blue -> damn "#0000FF"
            when Custom(r, g, b) -> {
                damn "#" + hex(r) + hex(g) + hex(b)
            }
        }
    }
}
```

## Modules and Imports

### Module Structure
```
project/
├── main.csd
├── utils.csd
└── models/
    ├── user.csd
    └── order.csd
```

### Import System
```cursed
# Import standard library modules
yeet "mathz"        # Math functions
yeet "stringz"      # String utilities
yeet "testz"        # Testing framework

# Import local modules
yeet "utils"        # ./utils.csd
yeet "models/user"  # ./models/user.csd

# Aliased imports
yeet "mathz" as math
yeet "stringz" as str

# Selective imports
yeet "mathz" { abs_normie, max_normie }
```

### Module Exports
```cursed
# utils.csd - Export functions and types
slay public_function() tea {
    damn "This is exported"
}

# Private function (not exported)
slay private_helper() tea {
    damn "This is internal"
}

# Export struct
squad PublicStruct {
    spill data tea
}
```

## Error Handling

### Error Types
```cursed
# Function that can return an error
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero error"
    }
    damn a / b
}

# Result type for fallible operations
type Result<T, E> = enum {
    Ok(T),
    Err(E)
}
```

### Error Handling Patterns
```cursed
# Try-catch style error handling
sus result drip = divide(10, 2) fam {
    when "Division by zero error" -> {
        vibez.spill("Cannot divide by zero!")
        damn 0  # Default value
    }
    when other -> {
        vibez.spill("Unexpected error:", other)
        shook   # Panic
    }
}

# Error propagation
slay calculate() yikes<tea> {
    sus x drip = divide(20, 4)?  # Propagate error if any
    sus y drip = divide(x, 2)?   # Propagate error if any
    damn y
}

# Panic on error
sus value drip = divide(10, 0).unwrap()  # Panics if error
```

## Concurrency

### Goroutines
```cursed
yeet "concurrenz"

# Spawn a goroutine
vibe {
    vibez.spill("Running in goroutine")
    sleep(1000)  # Sleep for 1 second
    vibez.spill("Goroutine finished")
}

# Goroutine with parameters
slay worker(id drip, work_channel chan<tea>) {
    bestie (based) {
        sus work tea = <-work_channel
        ready (work == "stop") {
            break
        }
        vibez.spill("Worker", id, "processing:", work)
    }
}

# Spawn multiple workers
for i in 0..5 {
    vibe worker(i, work_chan)
}
```

### Channels
```cursed
# Create channels
sus messages chan<tea> = make_channel()
sus numbers chan<drip> = make_channel_buffered(10)

# Send and receive
vibe {
    messages <- "Hello"      # Send
    messages <- "World"
}

sus msg1 tea = <-messages   # Receive (blocking)
sus msg2 tea = <-messages

# Channel operations with select
select {
    when msg <- messages -> {
        vibez.spill("Received message:", msg)
    }
    when numbers <- 42 -> {
        vibez.spill("Sent number 42")
    }
    when timeout(1000) -> {
        vibez.spill("Timeout after 1 second")
    }
    default -> {
        vibez.spill("No channel operation ready")
    }
}
```

### Synchronization
```cursed
yeet "concurrenz"

# Mutex for shared state
sus counter Mutex<drip> = Mutex.new(0)

slay increment_counter() {
    sus guard = counter.lock()
    guard.value = guard.value + 1
    # Automatically unlocked when guard goes out of scope
}

# WaitGroup for coordination
sus wg WaitGroup = WaitGroup.new()

for i in 0..10 {
    wg.add(1)
    vibe {
        defer wg.done()
        increment_counter()
    }
}

wg.wait()  # Wait for all goroutines to finish
```

## Memory Management

### Stack vs Heap Allocation
```cursed
# Stack allocation (automatic)
sus local_array [100]drip = [0; 100]  # Stack allocated

# Heap allocation (manual)
sus heap_array []drip = make_slice(1000)  # Heap allocated

# Reference counting for shared ownership
sus shared Rc<tea> = Rc.new("shared data")
sus another_ref = shared.clone()
```

### Memory Safety Features
```cursed
# Automatic memory management
slay process_data() {
    sus data []drip = [1, 2, 3, 4, 5]
    # Memory automatically freed when function exits
}

# Explicit cleanup with defer
slay file_operation() yikes<tea> {
    sus file = open_file("data.txt")?
    defer file.close()  # Always called before function exit
    
    # Process file...
    ready (some_condition) {
        yikes "Processing failed"  # defer still runs
    }
    
    damn "Success"
}
```

### Ownership and Borrowing
```cursed
# Move semantics
sus data []drip = [1, 2, 3]
sus moved_data = data      # data is now invalid
# vibez.spill(data)       # Compile error: use after move

# Borrowing (references)
slay process_slice(items &[]drip) {
    for item in items {
        vibez.spill(item)
    }
}

sus numbers []drip = [1, 2, 3]
process_slice(&numbers)    # Borrow, not move
vibez.spill(numbers)       # Still valid
```

This completes the core language reference. For more advanced topics, see:

- [Standard Library Reference](stdlib/)
- [Advanced Features Guide](advanced-features.md)
- [Performance Best Practices](../deployment/performance.md)
- [Memory Safety Guide](memory-safety.md)
