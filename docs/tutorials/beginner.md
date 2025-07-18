# CURSED Programming Language - Beginner Tutorial

## Table of Contents
1. [Installation](#installation)
2. [First Program](#first-program)
3. [Basic Syntax](#basic-syntax)
4. [Variables and Types](#variables-and-types)
5. [Control Flow](#control-flow)
6. [Functions](#functions)
7. [Basic Data Structures](#basic-data-structures)
8. [Error Handling](#error-handling)

## Installation

### Prerequisites
- Rust (latest stable version)
- LLVM development libraries
- Git

### Installing CURSED
```bash
# Clone the repository
git clone https://github.com/cursed-lang/cursed
cd cursed

# Build the compiler
cargo build --release

# Add to PATH (optional)
export PATH=$PATH:$(pwd)/target/release
```

### Verify Installation
```bash
# Check compiler version
cargo run --bin cursed --version

# Run a simple test
echo 'vibez.spill("Hello, CURSED!")' > hello.csd
cargo run --bin cursed hello.csd
```

## First Program

Let's start with the traditional "Hello, World!" program:

**File: `hello_world.csd`**
```cursed
vibez.spill("Hello, World!")
```

### Running Your Program

#### Interpretation Mode (Development)
```bash
cargo run --bin cursed hello_world.csd
```

#### Compilation Mode (Production)
```bash
# Compile to native executable
cargo run --bin cursed -- compile hello_world.csd

# Run the compiled executable
./hello_world
```

### Understanding the Code
- `vibez.spill()` is CURSED's print function
- Strings are enclosed in double quotes
- No semicolons required at the end of statements

## Basic Syntax

### Comments
```cursed
# This is a single-line comment
# There are no multi-line comments in CURSED
```

### Keywords
CURSED uses unique keywords that reflect its vibe:
- `sus` - variable declaration
- `slay` - function definition
- `damn` - return statement
- `vibez` - standard library namespace
- `yeet` - import statement
- `based` - true
- `cap` - false
- `cringe` - null/nil

### Case Sensitivity
CURSED is case-sensitive:
```cursed
sus myVar tea = "hello"  # Valid
sus myvar tea = "world"  # Different variable
```

## Variables and Types

### Variable Declaration
```cursed
# Basic variable declaration
sus name tea = "Alice"
sus age normie = 25
sus isActive lit = based

# Short variable declaration (type inferred)
count := 42
pi := 3.14159
```

### Basic Types

#### Integers
```cursed
sus smallInt smol = 100      # i8
sus mediumInt mid = 1000     # i16
sus normalInt normie = 10000  # i32 (default)
sus bigInt thicc = 1000000   # i64
sus byteVal byte = 255       # u8
```

#### Floating Point
```cursed
sus smallFloat drip = 3.14   # f32
sus bigFloat meal = 3.14159  # f64
```

#### Booleans
```cursed
sus isTrue lit = based       # true
sus isFalse lit = cap        # false
```

#### Strings
```cursed
sus greeting tea = "Hello"
sus name tea = "World"
sus combined tea = greeting + ", " + name + "!"
```

#### Characters
```cursed
sus letter sip = 'A'
sus digit sip = '5'
sus newline sip = '\n'
```

### Type Conversions
```cursed
sus number normie = 42
sus floatNum meal = number.(meal)  # Convert to float
sus smallNum smol = number.(smol)  # Convert to small int
```

## Control Flow

### If Statements
```cursed
sus score normie = 85

lowkey score >= 90 {
    vibez.spill("Excellent!")
} ghostly score >= 80 {
    vibez.spill("Good job!")
} vibes {
    vibez.spill("Keep trying!")
}
```

### For Loops

#### Traditional For Loop
```cursed
bestie i := 0; i < 10; i++ {
    vibez.spill("Count: " + i)
}
```

#### For-In Loop
```cursed
sus numbers := [1, 2, 3, 4, 5]
bestie num in numbers {
    vibez.spill("Number: " + num)
}
```

### While Loops
```cursed
sus counter normie = 0
bestie counter < 5 {
    vibez.spill("Counter: " + counter)
    counter++
}
```

### Break and Continue
```cursed
bestie i := 0; i < 10; i++ {
    lowkey i == 5 {
        simp  # continue
    }
    lowkey i == 8 {
        ghosted  # break
    }
    vibez.spill("i = " + i)
}
```

## Functions

### Function Declaration
```cursed
slay greet(name tea) {
    vibez.spill("Hello, " + name + "!")
}

# Call the function
greet("Alice")
```

### Functions with Return Values
```cursed
slay add(a normie, b normie) normie {
    damn a + b
}

sus result normie = add(5, 3)
vibez.spill("Result: " + result)
```

### Multiple Return Values
```cursed
slay divide(a normie, b normie) (normie, tea) {
    lowkey b == 0 {
        damn 0, "Division by zero"
    }
    damn a / b, ""
}

sus quotient normie
sus errorMsg tea
(quotient, errorMsg) = divide(10, 2)
```

### Function Parameters
```cursed
# By value (default)
slay modifyValue(x normie) {
    x = 100  # Original value unchanged
}

# By reference (for arrays, maps, etc.)
slay modifyArray(arr [normie]) {
    arr[0] = 999  # Original array is modified
}
```

## Basic Data Structures

### Arrays
```cursed
# Fixed-size arrays
sus numbers [5]normie = [1, 2, 3, 4, 5]
sus names [3]tea = ["Alice", "Bob", "Charlie"]

# Dynamic arrays (slices)
sus dynamicNumbers []normie = [1, 2, 3]
dynamicNumbers = append(dynamicNumbers, 4)

# Accessing elements
vibez.spill("First number: " + numbers[0])
vibez.spill("Array length: " + len(numbers))
```

### Strings
```cursed
sus message tea = "Hello, World!"
sus firstChar sip = message[0]  # 'H'
sus length normie = len(message)  # 13

# String concatenation
sus greeting tea = "Hello"
sus name tea = "Alice"
sus fullGreeting tea = greeting + ", " + name + "!"
```

### Tuples
```cursed
# Creating tuples
sus person := ("Alice", 25, based)
sus coordinates := (10.5, 20.3)

# Accessing tuple elements
sus name tea = person.0
sus age normie = person.1
sus isActive lit = person.2

# Tuple destructuring
sus (userName, userAge, userActive) := person
```

## Error Handling

### Basic Error Handling
```cursed
slay safeDivide(a normie, b normie) (normie, tea) {
    lowkey b == 0 {
        damn 0, "Cannot divide by zero"
    }
    damn a / b, ""
}

sus result normie
sus error tea
(result, error) = safeDivide(10, 0)

lowkey error != "" {
    vibez.spill("Error: " + error)
} vibes {
    vibez.spill("Result: " + result)
}
```

### Using Error Types
```cursed
yeet "error_drip"  # Import error handling module

slay processFile(filename tea) error_drip.ErrorType {
    lowkey filename == "" {
        damn error_drip.new_error("Filename cannot be empty")
    }
    
    # Process file...
    damn cringe  # No error
}

sus err := processFile("data.txt")
lowkey err != cringe {
    vibez.spill("Error processing file: " + err.message())
}
```

## Practice Exercises

### Exercise 1: Calculator
Create a simple calculator that performs basic arithmetic operations:

```cursed
slay add(a normie, b normie) normie {
    damn a + b
}

slay subtract(a normie, b normie) normie {
    damn a - b
}

slay multiply(a normie, b normie) normie {
    damn a * b
}

slay divide(a normie, b normie) (normie, tea) {
    lowkey b == 0 {
        damn 0, "Division by zero"
    }
    damn a / b, ""
}

# Test the calculator
sus a normie = 10
sus b normie = 3

vibez.spill("Addition: " + add(a, b))
vibez.spill("Subtraction: " + subtract(a, b))
vibez.spill("Multiplication: " + multiply(a, b))

sus result normie
sus error tea
(result, error) = divide(a, b)
lowkey error != "" {
    vibez.spill("Error: " + error)
} vibes {
    vibez.spill("Division: " + result)
}
```

### Exercise 2: Array Operations
Write functions to work with arrays:

```cursed
slay findMax(numbers []normie) normie {
    lowkey len(numbers) == 0 {
        damn 0
    }
    
    sus max normie = numbers[0]
    bestie i := 1; i < len(numbers); i++ {
        lowkey numbers[i] > max {
            max = numbers[i]
        }
    }
    damn max
}

slay sum(numbers []normie) normie {
    sus total normie = 0
    bestie num in numbers {
        total += num
    }
    damn total
}

# Test the functions
sus numbers []normie = [5, 2, 8, 1, 9, 3]
vibez.spill("Max: " + findMax(numbers))
vibez.spill("Sum: " + sum(numbers))
```

### Exercise 3: String Processing
Create a function to count words in a string:

```cursed
yeet "stringz"  # Import string utilities

slay countWords(text tea) normie {
    lowkey text == "" {
        damn 0
    }
    
    sus words []tea = stringz.split(text, " ")
    sus count normie = 0
    
    bestie word in words {
        lowkey stringz.trim(word) != "" {
            count++
        }
    }
    
    damn count
}

# Test the function
sus text tea = "Hello world from CURSED"
vibez.spill("Word count: " + countWords(text))
```

## Next Steps

Congratulations! You've completed the beginner tutorial. You should now understand:
- Basic CURSED syntax and keywords
- Variable declaration and types
- Control flow structures
- Function definition and usage
- Basic data structures
- Error handling fundamentals

### What's Next?
- [Intermediate Tutorial](intermediate.md) - Learn about modules, structs, and advanced features
- [Advanced Tutorial](advanced.md) - Master concurrency, generics, and performance optimization
- [API Documentation](../api/) - Explore the complete standard library
- [Examples](../../examples/) - See real-world CURSED applications

### Additional Resources
- [CURSED Language Specification](../spec.md)
- [Common Patterns](../patterns.md)
- [Performance Guide](../performance.md)
- [Migration Guides](../migration/) - Coming from other languages

---

*Happy coding with CURSED! 🔥*
