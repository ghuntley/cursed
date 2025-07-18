# Basic Syntax

This guide covers the fundamental syntax elements of CURSED programming language.

## Keywords and Identifiers

### Reserved Keywords
```cursed
# Variable declaration
sus                    # Declare variable
damn                   # Return statement

# Control flow
lowkey                 # If statement
highkey                # Else statement
bestie                 # For loop
ready                  # Select statement
ghosted                # Break statement
simp                   # Continue statement

# Functions and types
slay                   # Function definition
vibe                   # Package declaration
yeet                   # Import statement
vibes                  # Export statement

# Concurrency
yolo                   # Goroutine spawn
chan                   # Channel type
make                   # Create channel/map

# Error handling
yikes                  # Panic/error
shook                  # Recover
fam                    # Error handling

# Literals
based                  # True boolean
cap                    # False boolean
cringe                 # Nil/null
```

### Identifiers
```cursed
# Valid identifiers
variable_name
functionName
CONSTANT_VALUE
_private_var
counter123

# Invalid identifiers
123invalid        # Cannot start with number
sus-name          # Cannot contain hyphens
class             # Reserved keyword
```

## Types

### Basic Types
```cursed
# Integer types
sus small_int smol = 42      # i8
sus medium_int mid = 1000    # i16
sus normal_int normie = 42   # i32 (default)
sus large_int thicc = 1000000 # i64

# Float types
sus float_val drip = 3.14    # f32
sus double_val meal = 3.14159 # f64

# Other types
sus char_val sip = 'A'       # Character
sus string_val tea = "Hello" # String
sus bool_val lit = based     # Boolean
sus byte_val byte = 255      # Unsigned 8-bit
```

### Composite Types
```cursed
# Arrays
sus numbers [5]normie = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Charlie"]

# Tuples
sus person (tea, normie) = ("Alice", 30)

# Structs
struct Person {
    name tea
    age normie
}
```

## Operators

### Arithmetic Operators
```cursed
sus a normie = 10
sus b normie = 3

sus sum normie = a + b        # Addition: 13
sus diff normie = a - b       # Subtraction: 7
sus product normie = a * b    # Multiplication: 30
sus quotient normie = a / b   # Division: 3
sus remainder normie = a % b  # Modulo: 1
```

### Comparison Operators
```cursed
sus x normie = 10
sus y normie = 20

sus equal lit = x == y        # Equality: cap
sus not_equal lit = x != y    # Inequality: based
sus less lit = x < y          # Less than: based
sus greater lit = x > y       # Greater than: cap
sus less_equal lit = x <= y   # Less or equal: based
sus greater_equal lit = x >= y # Greater or equal: cap
```

### Logical Operators
```cursed
sus a lit = based
sus b lit = cap

sus and_result lit = a && b   # Logical AND: cap
sus or_result lit = a || b    # Logical OR: based
sus not_result lit = !a       # Logical NOT: cap
```

### Assignment Operators
```cursed
sus x normie = 10

x += 5                        # Add and assign: x = 15
x -= 3                        # Subtract and assign: x = 12
x *= 2                        # Multiply and assign: x = 24
x /= 4                        # Divide and assign: x = 6
x %= 4                        # Modulo and assign: x = 2
```

### Increment/Decrement
```cursed
sus counter normie = 0

counter++                     # Post-increment
++counter                     # Pre-increment
counter--                     # Post-decrement
--counter                     # Pre-decrement
```

## Control Flow

### If Statements
```cursed
sus age normie = 18

lowkey age >= 18 {
    vibez.spill("You are an adult")
} highkey age >= 13 {
    vibez.spill("You are a teenager")
} highkey {
    vibez.spill("You are a child")
}
```

### Loops

#### For Loop (C-style)
```cursed
bestie i := 0; i < 10; i++ {
    vibez.spill(i)
}
```

#### For-Each Loop
```cursed
sus numbers []normie = [1, 2, 3, 4, 5]

bestie num <- numbers {
    vibez.spill(num)
}
```

#### While Loop
```cursed
sus i normie = 0
bestie i < 10 {
    vibez.spill(i)
    i++
}
```

### Break and Continue
```cursed
bestie i := 0; i < 10; i++ {
    lowkey i == 5 {
        simp  # Continue to next iteration
    }
    lowkey i == 8 {
        ghosted  # Break out of loop
    }
    vibez.spill(i)
}
```

## Functions

### Function Definition
```cursed
slay add(a normie, b normie) normie {
    damn a + b
}
```

### Function with Multiple Returns
```cursed
slay divide(a normie, b normie) (normie, tea) {
    lowkey b == 0 {
        damn 0, "Division by zero"
    }
    damn a / b, ""
}
```

### Function with No Return
```cursed
slay greet(name tea) {
    vibez.spill("Hello, " + name + "!")
}
```

## Variables

### Variable Declaration
```cursed
# Explicit type
sus name tea = "Alice"
sus age normie = 25

# Type inference
sus country := "USA"          # Inferred as tea
sus population := 331000000   # Inferred as normie
```

### Short Variable Declaration
```cursed
name := "Bob"
age := 30
active := based
```

### Multiple Variable Declaration
```cursed
sus a, b, c normie = 1, 2, 3
x, y, z := 10, 20, 30
```

## Comments

### Single Line Comments
```cursed
# This is a comment
fr fr This is also a comment
no cap This is a comment too
on god This is also a comment
```

### Multi-line Comments
```cursed
fr fr
This is a multi-line comment
that spans multiple lines
fr fr
```

## String Operations

### String Literals
```cursed
sus basic_string tea = "Hello, World!"
sus raw_string tea = `This is a raw string with "quotes"`
```

### String Concatenation
```cursed
sus first tea = "Hello"
sus second tea = "World"
sus combined tea = first + ", " + second + "!"
```

### String Interpolation
```cursed
sus name tea = "Alice"
sus age normie = 25
sus message tea = "My name is " + name + " and I am " + age.(tea) + " years old"
```

## Arrays and Slices

### Array Declaration
```cursed
sus fixed_array [5]normie = [1, 2, 3, 4, 5]
sus dynamic_array []normie = [1, 2, 3]
```

### Array Access
```cursed
sus numbers [3]normie = [10, 20, 30]
sus first normie = numbers[0]      # Access first element
numbers[1] = 25                    # Modify second element
```

### Slice Operations
```cursed
sus slice []normie = [1, 2, 3, 4, 5]
sus sub_slice []normie = slice[1:4]  # Elements 2, 3, 4
```

## Type Conversions

### Explicit Type Conversion
```cursed
sus str tea = "123"
sus num normie = str.(normie)      # String to integer

sus float_val drip = 3.14
sus int_val normie = float_val.(normie)  # Float to integer
```

### Type Assertions
```cursed
sus value interface{} = 42
sus int_val normie = value.(normie)    # Type assertion
```

## Error Handling

### Basic Error Handling
```cursed
slay divide(a normie, b normie) normie {
    lowkey b == 0 {
        yikes "Division by zero"
    }
    damn a / b
}
```

### Panic and Recover
```cursed
slay safe_divide(a normie, b normie) normie {
    defer {
        lowkey err := shook(); err != cringe {
            vibez.spill("Recovered from panic:", err)
        }
    }
    
    lowkey b == 0 {
        yikes "Division by zero"
    }
    damn a / b
}
```

## Best Practices

### 1. Use Descriptive Names
```cursed
# Good
sus user_count normie = 100
sus is_authenticated lit = based

# Bad
sus n normie = 100
sus flag lit = based
```

### 2. Consistent Formatting
```cursed
# Good
slay calculate_total(price drip, tax drip) drip {
    damn price * (1 + tax)
}

# Bad
slay calculate_total(price drip,tax drip)drip{
damn price*(1+tax)
}
```

### 3. Error Handling
```cursed
# Good
slay read_file(filename tea) (tea, tea) {
    # Check if file exists
    lowkey !file_exists(filename) {
        damn "", "File not found"
    }
    
    # Read file content
    content := read_file_content(filename)
    damn content, ""
}

# Bad
slay read_file(filename tea) tea {
    damn read_file_content(filename)  # No error handling
}
```

## Common Syntax Errors

### Missing Semicolons
```cursed
# CURSED doesn't require semicolons for statement termination
# These are equivalent:
sus x normie = 42
sus y normie = 24
```

### Incorrect Keyword Usage
```cursed
# ❌ Wrong
if condition {
    return value
}

# ✅ Correct
lowkey condition {
    damn value
}
```

### Type Mismatches
```cursed
# ❌ Wrong
sus x normie = "string"

# ✅ Correct
sus x normie = 42
sus y tea = "string"
```

---

Next: [Variables and Types →](03-variables-types.md)
