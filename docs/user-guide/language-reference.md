# 📖 CURSED Language Reference

Complete syntax reference for the CURSED programming language.

## 📝 Table of Contents

- [Basic Syntax](#basic-syntax)
- [Data Types](#data-types)
- [Variables](#variables)
- [Functions](#functions)
- [Control Flow](#control-flow)
- [Error Handling](#error-handling)
- [Modules and Imports](#modules-and-imports)
- [Concurrency](#concurrency)
- [Advanced Features](#advanced-features)
- [Operators](#operators)
- [Comments](#comments)
- [Keywords](#keywords)

## 🔤 Basic Syntax

CURSED uses Gen Z slang keywords for a fun and modern programming experience:

```cursed
fr This is a comment
sus variable_name type = value    # Variable declaration
slay function_name() { }          # Function declaration
vibez.spill("Hello")             # Print statement
```

### File Extension
- CURSED source files use the `.csd` extension

### Case Sensitivity
- CURSED is case-sensitive
- Variables and functions typically use `snake_case`
- Types use `lowercase`

## 🏷️ Data Types

### Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| `drip` | Integer (64-bit signed) | `42`, `-123` |
| `meal` | Floating point (64-bit) | `3.14`, `-2.5` |
| `tea` | String (UTF-8) | `"Hello"`, `'World'` |
| `lit` | Boolean | `based` (true), `cap` (false) |

### Collection Types

```cursed
fr Arrays
sus numbers []drip = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Charlie"]

fr Maps (dictionaries)
sus scores map<tea, drip> = {
    "Alice": 95,
    "Bob": 87,
    "Charlie": 92
}

fr Sets
sus unique_numbers set<drip> = {1, 2, 3, 4, 5}
```

### Custom Types

```cursed
fr Structs
squad Person {
    name tea
    age drip
    email tea
}

fr Enums
sick Color {
    Red,
    Green,
    Blue,
    RGB(drip, drip, drip)
}

fr Interfaces
collab Drawable {
    slay draw()
    slay area() meal
}
```

## 📦 Variables

### Declaration

```cursed
fr Basic declaration
sus name tea = "Alice"
sus age drip = 25

fr Type inference
sus auto_string = "Inferred as tea"
sus auto_number = 42

fr Mutable vs immutable
sus mutable_var drip = 10    # Can be changed
lock immutable_var drip = 5  # Cannot be changed

fr Multiple assignment
sus x, y drip = 10, 20
sus a, b, c = 1, "hello", based
```

### Scope

```cursed
sus global_var drip = 100

slay example_function() {
    sus local_var drip = 50
    
    ready (based) {
        sus block_var drip = 25
        fr block_var is only accessible here
    }
    
    fr local_var is accessible here
    fr global_var is accessible everywhere
}
```

## ⚙️ Functions

### Basic Functions

```cursed
fr Simple function
slay greet() {
    vibez.spill("Hello!")
}

fr Function with parameters
slay greet_person(name tea) {
    vibez.spill("Hello,", name, "!")
}

fr Function with return value
slay add(a drip, b drip) drip {
    damn a + b
}

fr Multiple return values
slay divide_and_remainder(a drip, b drip) (drip, drip) {
    damn a / b, a % b
}
```

### Advanced Functions

```cursed
fr Default parameters
slay greet_with_title(name tea, title tea = "Mr.") {
    vibez.spill("Hello,", title, name)
}

fr Variadic functions
slay sum(numbers ...drip) drip {
    sus total drip = 0
    bestie (num in numbers) {
        total = total + num
    }
    damn total
}

fr Higher-order functions
slay apply_operation(a drip, b drip, op slay(drip, drip) drip) drip {
    damn op(a, b)
}

sus result drip = apply_operation(5, 3, slay(x drip, y drip) drip {
    damn x * y
})
```

### Generic Functions

```cursed
fr Generic function
slay max<T>(a T, b T) T {
    ready (a > b) {
        damn a
    }
    damn b
}

sus max_int drip = max<drip>(5, 10)
sus max_float meal = max<meal>(3.14, 2.71)
```

## 🔄 Control Flow

### Conditional Statements

```cursed
fr If-else statements
ready (condition) {
    fr Execute if true
} otherwise ready (other_condition) {
    fr Execute if other_condition is true
} otherwise {
    fr Execute if all conditions are false
}

fr Ternary operator
sus result drip = condition ? value_if_true : value_if_false
```

### Loops

```cursed
fr While loop
sus i drip = 0
bestie (i < 10) {
    vibez.spill("Count:", i)
    i = i + 1
}

fr For-each loop
sus numbers []drip = [1, 2, 3, 4, 5]
bestie (num in numbers) {
    vibez.spill("Number:", num)
}

fr For-each with index
bestie (index, num in numbers) {
    vibez.spill("Index:", index, "Number:", num)
}

fr Range loops
bestie (i in 0..10) {        # 0 to 9
    vibez.spill("i:", i)
}

bestie (i in 0..=10) {       # 0 to 10 (inclusive)
    vibez.spill("i:", i)
}
```

### Loop Control

```cursed
bestie (i in 0..100) {
    ready (i == 50) {
        break      # Exit loop
    }
    
    ready (i % 2 == 0) {
        continue   # Skip to next iteration
    }
    
    vibez.spill("Odd number:", i)
}
```

### Pattern Matching

```cursed
sick Result<T> {
    Ok(T),
    Error(tea)
}

sus result Result<drip> = Ok(42)

sick result {
    Ok(value) -> {
        vibez.spill("Success:", value)
    }
    Error(message) -> {
        vibez.spill("Error:", message)
    }
}

fr Pattern guards
sick value {
    x when x > 100 -> vibez.spill("Large number")
    x when x > 10 -> vibez.spill("Medium number")
    _ -> vibez.spill("Small number")
}
```

## ⚠️ Error Handling

### Error Types

```cursed
fr Function that can return an error
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero"
    }
    damn a / b
}
```

### Error Handling

```cursed
fr Try-catch with fam
sus result drip = divide(10, 2) fam {
    when "Division by zero" -> {
        vibez.spill("Cannot divide by zero!")
        damn 0
    }
    when error -> {
        vibez.spill("Unexpected error:", error)
        damn -1
    }
}

fr Propagate errors
slay risky_operation() yikes<tea> {
    sus value drip = divide(10, 0)?  # ? propagates error
    damn value * 2
}

fr Multiple error types
sick CustomError {
    NetworkError(tea),
    ParseError(tea),
    ValidationError(tea)
}

slay complex_operation() yikes<CustomError> {
    fr Function implementation
}
```

### Error Recovery

```cursed
fr Defer statements for cleanup
slay file_operation() yikes<tea> {
    sus file = open_file("data.txt")?
    defer file.close()  # Always executed, even on error
    
    fr Work with file
    sus content tea = file.read()?
    damn content
}
```

## 📦 Modules and Imports

### Import Statements

```cursed
fr Import entire module
yeet "mathz"
yeet "stringz"

fr Import specific functions
yeet "mathz" { sqrt, sin, cos }

fr Import with alias
yeet "very_long_module_name" as vmn

fr Import from relative path
yeet "./utils/helpers"
yeet "../shared/constants"
```

### Module Definition

```cursed
fr In file: math_utils.csd
module math_utils

fr Public functions (exported)
slay pub add(a drip, b drip) drip {
    damn a + b
}

fr Private functions (not exported)
slay calculate_internal() drip {
    damn 42
}

fr Public constants
lock pub PI meal = 3.14159265359

fr Public types
squad pub Point {
    x meal
    y meal
}
```

### Package System

```cursed
fr CursedPackage.toml
[package]
name = "my-package"
version = "1.0.0"

[dependencies]
mathz = "1.2.0"
stringz = "2.1.0"
networkz = { version = "3.0.0", features = ["tls"] }
```

## 🔄 Concurrency

### Goroutines

```cursed
yeet "concurrenz"

fr Start a goroutine
go {
    vibez.spill("Running in background")
}

fr Goroutine with parameters
go worker(worker_id) {
    vibez.spill("Worker", worker_id, "starting")
}

fr Wait for goroutines
sus wg WaitGroup = WaitGroup.new()
wg.add(2)

go {
    defer wg.done()
    fr Do work
}

go {
    defer wg.done()
    fr Do more work
}

wg.wait()  # Wait for all to complete
```

### Channels

```cursed
fr Create channels
sus ch chan<drip> = make_channel()
sus buffered_ch chan<drip> = make_channel(10)  # Buffered

fr Send and receive
go {
    ch <- 42      # Send
}

sus value drip = <-ch  # Receive

fr Select statement
sick {
    ch1 <- value -> {
        vibez.spill("Sent to ch1")
    }
    value := <-ch2 -> {
        vibez.spill("Received from ch2:", value)
    }
    timeout(1000) -> {
        vibez.spill("Timeout after 1 second")
    }
}
```

### Async/Await

```cursed
fr Async functions
slay async fetch_data(url tea) Promise<tea> {
    sus response = await http_get(url)
    damn response.body
}

fr Using async functions
slay main() {
    sus data tea = await fetch_data("https://api.example.com")
    vibez.spill("Data:", data)
}
```

## 🚀 Advanced Features

### Generics

```cursed
fr Generic types
squad Box<T> {
    value T
}

fr Generic functions with constraints
slay sort<T: Comparable>(items []T) []T {
    fr Sorting implementation
}

fr Multiple type parameters
slay map<K, V>(items []K, mapper slay(K) V) []V {
    sus result []V = []
    bestie (item in items) {
        result.push(mapper(item))
    }
    damn result
}
```

### Interfaces

```cursed
fr Define interface
collab Drawable {
    slay draw()
    slay area() meal
}

fr Implement interface
squad Circle {
    radius meal
}

impl Drawable for Circle {
    slay draw() {
        vibez.spill("Drawing circle with radius", self.radius)
    }
    
    slay area() meal {
        damn 3.14159 * self.radius * self.radius
    }
}
```

### Macros

```cursed
fr Define macro
macro debug_print(expr) {
    vibez.spill("DEBUG:", stringify(expr), "=", expr)
}

fr Use macro
sus x drip = 42
debug_print(x + 1)  # Outputs: DEBUG: x + 1 = 43
```

### Attributes

```cursured
fr Function attributes
[inline]
slay fast_function() drip {
    damn 42
}

[deprecated("Use new_function instead")]
slay old_function() {
    fr Old implementation
}

[test]
slay test_addition() {
    assert_eq(2 + 2, 4)
}
```

## 🔧 Operators

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `5 + 3` → `8` |
| `-` | Subtraction | `5 - 3` → `2` |
| `*` | Multiplication | `5 * 3` → `15` |
| `/` | Division | `15 / 3` → `5` |
| `%` | Modulo | `17 % 5` → `2` |
| `**` | Exponentiation | `2 ** 3` → `8` |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `5 == 5` → `based` |
| `!=` | Not equal | `5 != 3` → `based` |
| `<` | Less than | `3 < 5` → `based` |
| `<=` | Less or equal | `5 <= 5` → `based` |
| `>` | Greater than | `5 > 3` → `based` |
| `>=` | Greater or equal | `5 >= 5` → `based` |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&&` | Logical AND | `based && cap` → `cap` |
| `\|\|` | Logical OR | `based \|\| cap` → `based` |
| `!` | Logical NOT | `!based` → `cap` |

### Assignment Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `=` | Assignment | `x = 5` |
| `+=` | Add and assign | `x += 3` |
| `-=` | Subtract and assign | `x -= 2` |
| `*=` | Multiply and assign | `x *= 4` |
| `/=` | Divide and assign | `x /= 2` |
| `%=` | Modulo and assign | `x %= 3` |

### Bitwise Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&` | Bitwise AND | `5 & 3` → `1` |
| `\|` | Bitwise OR | `5 \| 3` → `7` |
| `^` | Bitwise XOR | `5 ^ 3` → `6` |
| `~` | Bitwise NOT | `~5` → `-6` |
| `<<` | Left shift | `5 << 1` → `10` |
| `>>` | Right shift | `10 >> 1` → `5` |

## 💬 Comments

```cursed
fr Single-line comment

/*
Multi-line comment
can span multiple lines
*/

fr fr Alternative single-line comment style

/**
 * Documentation comment
 * Used for generating documentation
 */
slay documented_function() {
    fr Function body
}
```

## 🔑 Keywords

### Core Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `sus` | Variable declaration | `sus x drip = 5` |
| `lock` | Constant declaration | `lock PI meal = 3.14` |
| `slay` | Function declaration | `slay add() { }` |
| `damn` | Return statement | `damn result` |
| `ready` | If statement | `ready (condition) { }` |
| `otherwise` | Else clause | `otherwise { }` |
| `bestie` | While/for loop | `bestie (x < 10) { }` |
| `break` | Break from loop | `break` |
| `continue` | Continue loop | `continue` |
| `sick` | Pattern matching | `sick value { }` |
| `when` | Pattern case | `when pattern -> { }` |

### Type Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `drip` | Integer type | `sus x drip` |
| `meal` | Float type | `sus y meal` |
| `tea` | String type | `sus s tea` |
| `lit` | Boolean type | `sus b lit` |
| `squad` | Struct definition | `squad Person { }` |
| `collab` | Interface definition | `collab Drawable { }` |
| `sick` | Enum definition | `sick Color { }` |

### Module Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `yeet` | Import statement | `yeet "mathz"` |
| `module` | Module declaration | `module utils` |
| `pub` | Public visibility | `slay pub function() { }` |

### Concurrency Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `go` | Start goroutine | `go { work() }` |
| `chan` | Channel type | `sus ch chan<drip>` |
| `async` | Async function | `slay async fetch() { }` |
| `await` | Await async result | `await fetch()` |

### Error Handling Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `yikes` | Error type/throw | `yikes "error message"` |
| `fam` | Error handling | `result fam { }` |
| `defer` | Defer execution | `defer cleanup()` |

### Literal Keywords

| Keyword | Purpose | Example |
|---------|---------|---------|
| `based` | Boolean true | `sus x lit = based` |
| `cap` | Boolean false | `sus y lit = cap` |
| `nil` | Null value | `sus ptr = nil` |

## 📏 Syntax Rules

### Naming Conventions
- Variables and functions: `snake_case`
- Types and interfaces: `PascalCase` or `lowercase`
- Constants: `UPPER_CASE` or `snake_case`
- Modules: `lowercase` or `snake_case`

### Code Style
- Use 4 spaces for indentation (not tabs)
- Place opening braces on the same line
- Use meaningful variable names
- Keep lines under 100 characters
- Use the built-in formatter: `cursed format`

### Best Practices
- Handle errors explicitly with `fam` blocks
- Use `defer` for cleanup operations
- Prefer composition over inheritance
- Write tests for your functions
- Document public APIs with comments
- Use type annotations for clarity

This reference covers the core CURSED language syntax. For more advanced topics and standard library documentation, see the [API Reference](../api/) and [Examples](../../examples/).
