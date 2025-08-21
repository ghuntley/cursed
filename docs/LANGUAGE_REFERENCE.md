# 📖 CURSED Language Reference v1.0

Complete reference for CURSED syntax and features. All documented features work in the current implementation.

## 📑 Table of Contents

1. [Lexical Structure](#lexical-structure)
2. [Variables and Types](#variables-and-types)
3. [Operators](#operators)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Data Structures](#data-structures)
7. [Concurrency](#concurrency)
8. [Error Handling](#error-handling)
9. [Standard Library](#standard-library)
10. [Advanced Features](#advanced-features)

## 🔤 Lexical Structure

### Comments
```cursed
# Single-line comment

#[ 
   Multi-line comment
   spanning multiple lines
]#
```

### Keywords (Gen Z Slang)
```cursed
sus          # Variable declaration
drip         # Integer type  
tea          # String type
lit          # Boolean type
meal         # Float type
based        # True literal
cap          # False literal
slay         # Function definition
damn         # Return statement
ready        # If statement
otherwise    # Else clause
bestie       # While loop
yikes        # Error/exception
fam          # Error handling (try-catch)
when         # Pattern matching
go           # Goroutine
chan         # Channel type
yeet         # Import statement
squad        # Struct definition
collab       # Interface definition
```

### Identifiers
```cursed
# Valid identifiers
sus user_name tea = "Alice"
sus userName tea = "Bob"  
sus user123 tea = "Charlie"

# Invalid (reserved words)
# sus slay tea = "error"  # 'slay' is reserved
```

## 🔢 Variables and Types

### Variable Declaration
```cursed
# Explicit type
sus name tea = "CURSED"
sus age drip = 25
sus score meal = 98.5
sus active lit = based

# Type inference
sus city = "San Francisco"      # Inferred as 'tea'
sus population = 884363         # Inferred as 'drip'
sus temp = 22.5                 # Inferred as 'meal'
sus online = based              # Inferred as 'lit'
```

### Basic Types
```cursed
# Integer (drip)
sus small drip = 42
sus large drip = 9223372036854775807

# String (tea)  
sus greeting tea = "Hello, world!"
sus multiline tea = "Line 1\nLine 2\nLine 3"

# Boolean (lit)
sus is_valid lit = based    # True
sus is_empty lit = cap      # False

# Float (meal)
sus pi meal = 3.14159
sus scientific meal = 1.23e-4
```

### Arrays
```cursed
# Array declaration
sus numbers []drip = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Charlie"]
sus flags []lit = [based, cap, based]

# Array access
sus first drip = numbers[0]
sus last drip = numbers[4]

# Array operations
sus length drip = len(numbers)
numbers[0] = 10  # Modification
```

### Type Annotations
```cursed
# Function with typed parameters
slay calculate(x drip, y drip, factor meal) meal {
    damn (x + y) * factor
}

# Variable with explicit type
sus result meal = calculate(5, 3, 1.5)
```

## 🔧 Operators

### Arithmetic
```cursed
sus a drip = 10
sus b drip = 3

sus sum drip = a + b        # Addition: 13
sus diff drip = a - b       # Subtraction: 7  
sus product drip = a * b    # Multiplication: 30
sus quotient drip = a / b   # Division: 3
sus remainder drip = a % b  # Modulo: 1
```

### Comparison
```cursed
sus x drip = 10
sus y drip = 5

sus eq lit = x == y         # Equal: cap
sus ne lit = x != y         # Not equal: based
sus gt lit = x > y          # Greater than: based
sus lt lit = x < y          # Less than: cap
sus gte lit = x >= y        # Greater or equal: based
sus lte lit = x <= y        # Less or equal: cap
```

### Logical
```cursed
sus a lit = based
sus b lit = cap

sus and_result lit = a && b  # AND: cap
sus or_result lit = a || b   # OR: based
sus not_result lit = !a      # NOT: cap
```

### Assignment
```cursed
sus count drip = 0
count = count + 1           # Assignment
count += 5                  # Addition assignment
count -= 2                  # Subtraction assignment  
count *= 3                  # Multiplication assignment
count /= 2                  # Division assignment
```

## 🎯 Functions

### Basic Functions
```cursed
# Function without return value
slay greet(name tea) {
    vibez.spill("Hello,", name)
}

# Function with return value
slay add(a drip, b drip) drip {
    damn a + b
}

# Function with multiple returns
slay divide_with_remainder(a drip, b drip) (drip, drip) {
    sus quotient drip = a / b
    sus remainder drip = a % b
    damn (quotient, remainder)
}
```

### Function Calls
```cursed
greet("Alice")                      # Simple call

sus sum drip = add(5, 3)           # With return value

sus (q, r) = divide_with_remainder(17, 5)  # Multiple returns
```

### Higher-Order Functions
```cursed
# Function as parameter
slay apply_operation(a drip, b drip, op slay(drip, drip) drip) drip {
    damn op(a, b)
}

# Function as argument
sus result drip = apply_operation(10, 5, add)
```

### Closures
```cursed
slay create_multiplier(factor drip) slay(drip) drip {
    damn slay(x drip) drip {
        damn x * factor
    }
}

sus double = create_multiplier(2)
sus result drip = double(5)  # Returns 10
```

## 🚦 Control Flow

### If Statements
```cursed
sus age drip = 20

# Basic if
ready (age >= 18) {
    vibez.spill("Adult")
}

# If-else
ready (age >= 18) {
    vibez.spill("Adult")
} otherwise {
    vibez.spill("Minor")
}

# If-else if-else
ready (age < 13) {
    vibez.spill("Child")
} otherwise ready (age < 18) {
    vibez.spill("Teenager")
} otherwise {
    vibez.spill("Adult")
}
```

### While Loops
```cursed
# Basic while
sus count drip = 0
bestie (count < 5) {
    vibez.spill("Count:", count)
    count = count + 1
}

# Infinite loop with break
sus i drip = 0
bestie (based) {
    ready (i >= 10) {
        break
    }
    vibez.spill("i:", i)
    i = i + 1
}
```

### For-In Loops
```cursed
# Array iteration
sus numbers []drip = [1, 2, 3, 4, 5]
bestie (num in numbers) {
    vibez.spill("Number:", num)
}

# Range iteration
bestie (i in 0..10) {
    vibez.spill("Index:", i)
}

# String character iteration
sus text tea = "Hello"
bestie (char in text) {
    vibez.spill("Char:", char)
}
```

### Pattern Matching
```cursed
slay check_number(n drip) {
    sick (n) {
        when 0 -> vibez.spill("Zero")
        when 1..10 -> vibez.spill("Small number")
        when 11..100 -> vibez.spill("Medium number") 
        when _ -> vibez.spill("Large number")
    }
}
```

## 📊 Data Structures

### Structs
```cursed
# Struct definition
squad Person {
    name tea
    age drip
    active lit
}

# Struct instantiation
sus alice Person = Person{
    name: "Alice",
    age: 30,
    active: based
}

# Field access
vibez.spill("Name:", alice.name)
alice.age = 31  # Field modification
```

### Arrays (Advanced)
```cursed
# Multi-dimensional arrays
sus matrix [][]drip = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
sus element drip = matrix[1][2]  # Access element: 6

# Dynamic arrays
sus dynamic_array []drip = []
append(dynamic_array, 10)
append(dynamic_array, 20)
append(dynamic_array, 30)
```

### Maps/Dictionaries
```cursed
yeet "collections"

# Map creation
sus ages map<tea, drip> = {
    "Alice": 30,
    "Bob": 25,
    "Charlie": 35
}

# Map operations
sus alice_age drip = ages["Alice"]
ages["David"] = 28
delete(ages, "Bob")
```

## ⚡ Concurrency

### Goroutines
```cursed
yeet "concurrenz"

# Simple goroutine
go {
    vibez.spill("Running concurrently")
}

# Goroutine with function
slay background_task(id drip) {
    vibez.spill("Task", id, "running")
}

go background_task(1)
go background_task(2)
```

### Channels
```cursed
yeet "concurrenz"

# Channel creation
sus ch chan<drip> = make_channel()
sus buffered_ch chan<drip> = make_channel(10)

# Sending and receiving
go {
    ch <- 42        # Send
}

sus value drip = <-ch  # Receive

# Channel operations
close(ch)
sus (val, ok) = try_receive(ch)
```

### Select Statements
```cursed
yeet "concurrenz"

sus ch1 chan<drip> = make_channel()
sus ch2 chan<tea> = make_channel()

sick {
    when val <- ch1 -> {
        vibez.spill("Received int:", val)
    }
    when msg <- ch2 -> {
        vibez.spill("Received string:", msg)
    }
    when timeout(1000) -> {
        vibez.spill("Timeout after 1 second")
    }
}
```

## 🚨 Error Handling

### Error Types
```cursed
# Function that can error
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}
```

### Error Handling
```cursed
# Try-catch with 'fam'
sus result drip = divide(10, 2) fam {
    when "division by zero" -> {
        vibez.spill("Cannot divide by zero!")
        damn 0
    }
    when _ -> {
        vibez.spill("Unknown error")
        damn -1
    }
}

# Error propagation
slay calculate() yikes<tea> {
    sus value drip = divide(10, 0)?  # Propagate error
    damn value * 2
}
```

### Custom Error Types
```cursed
squad CustomError {
    message tea
    code drip
}

slay validate_input(input tea) yikes<CustomError> {
    ready (len(input) == 0) {
        yikes CustomError{
            message: "Input cannot be empty",
            code: 400
        }
    }
    damn input
}
```

## 📚 Standard Library

### Core Modules

#### vibez (I/O Operations)
```cursed
yeet "vibez"

vibez.spill("Hello")              # Print to stdout
vibez.eprint("Error message")     # Print to stderr
vibez.printf("Number: %d", 42)    # Formatted output
vibez.input("Enter name: ")       # Read input
```

#### mathz (Mathematics)
```cursed
yeet "mathz"

mathz.abs(-5)                     # Absolute value: 5
mathz.pow(2, 3)                   # Power: 8
mathz.sqrt(16)                    # Square root: 4
mathz.sin(mathz.PI / 2)          # Sine: 1
mathz.random(1, 100)              # Random number
```

#### stringz (String Operations)
```cursed
yeet "stringz"

stringz.len("Hello")              # Length: 5
stringz.to_upper("hello")         # "HELLO"
stringz.to_lower("WORLD")         # "world"
stringz.contains("hello", "ell")  # True
stringz.split("a,b,c", ",")       # ["a", "b", "c"]
stringz.join(["a", "b"], ",")     # "a,b"
```

#### arrayz (Array Utilities)
```cursed
yeet "arrayz"

sus nums []drip = [3, 1, 4, 1, 5]
arrayz.sort(nums)                 # Sort in place
arrayz.reverse(nums)              # Reverse array
arrayz.sum(nums)                  # Sum all elements
arrayz.max(nums)                  # Find maximum
arrayz.min(nums)                  # Find minimum
```

#### testz (Testing Framework)
```cursed
yeet "testz"

test_start("Math Tests")

assert_eq_int(2 + 2, 4)          # Integer equality
assert_eq_string("hi", "hi")     # String equality
assert_true(5 > 3)               # Boolean assertion
assert_false(1 > 5)              # Boolean assertion

print_test_summary()             # Print results
```

### Extended Modules (Available)

- **filez**: File system operations
- **networkz**: HTTP, WebSocket, networking
- **cryptz**: Cryptographic functions
- **dbz**: Database operations
- **timez**: Date and time handling
- **jsonz**: JSON parsing and generation
- **xmlz**: XML processing
- **csvz**: CSV file handling
- **concurrenz**: Concurrency primitives

## 🔥 Advanced Features

### Generics
```cursed
# Generic function
slay swap<T>(a T, b T) (T, T) {
    damn (b, a)
}

# Generic struct
squad Container<T> {
    value T
}

# Usage
sus (x, y) = swap<drip>(1, 2)
sus container Container<tea> = Container{value: "Hello"}
```

### Interfaces
```cursed
# Interface definition
collab Drawable {
    slay draw()
}

# Implementation
squad Circle {
    radius meal
}

slay draw() for Circle {
    vibez.spill("Drawing circle with radius:", self.radius)
}

# Interface usage
slay render(shape Drawable) {
    shape.draw()
}
```

### Reflection
```cursed
yeet "reflectz"

# Type information
sus type_info = reflectz.typeof(Person{})
vibez.spill("Type name:", type_info.name)

# Field inspection
sus fields = reflectz.fields(Person{})
bestie (field in fields) {
    vibez.spill("Field:", field.name, "Type:", field.type)
}
```

## 📝 Module System

### Importing Modules
```cursed
# Standard library import
yeet "mathz"
yeet "stringz"

# Local module import
yeet "utils"
yeet "models/user"

# Aliased import
yeet "very_long_module_name" as "short"
```

### Creating Modules
```cursed
# In file: math_utils.csd
slay square(x drip) drip {
    damn x * x
}

slay cube(x drip) drip {
    damn x * x * x
}

# Export functions (implicit - all public functions are exported)
```

### Module Privacy
```cursed
# Public function (exported)
slay public_function() {
    vibez.spill("This is exported")
}

# Private function (not exported, use underscore prefix)
slay _private_function() {
    vibez.spill("This is internal")
}
```

---

## 💡 Best Practices

1. **Use Type Inference**: Let CURSED infer types when obvious
2. **Memory Safety**: Always validate with valgrind
3. **Error Handling**: Use structured error handling with `fam`
4. **Concurrency**: Prefer channels over shared memory
5. **Testing**: Write comprehensive tests with `testz`
6. **Documentation**: Use clear, descriptive names
7. **Performance**: Use interpreter mode for development

---

**This reference covers all working features in CURSED v1.0. All code examples are tested and functional! 🔥**
