# Variables and Types

This guide covers CURSED's type system and variable handling, designed for safety and performance.

## Variable Declaration

### Basic Declaration
```cursed
sus variable_name type = value
```

### Examples
```cursed
sus name tea = "Alice"
sus age normie = 25
sus height drip = 5.8
sus is_active lit = based
```

### Short Declaration (Type Inference)
```cursed
name := "Bob"           # Inferred as tea
age := 30               # Inferred as normie
height := 6.0           # Inferred as drip
is_active := cap        # Inferred as lit
```

## Type System

### Integer Types

#### Signed Integers
```cursed
sus tiny_int smol = 127           # i8: -128 to 127
sus small_int mid = 32767         # i16: -32,768 to 32,767
sus normal_int normie = 2147483647 # i32: -2^31 to 2^31-1
sus large_int thicc = 9223372036854775807 # i64: -2^63 to 2^63-1
```

#### Unsigned Integers
```cursed
sus unsigned_byte byte = 255      # u8: 0 to 255
sus unsigned_rune rune = 65535    # u32: 0 to 2^32-1
```

#### Integer Literals
```cursed
sus decimal normie = 42
sus hex normie = 0x2A
sus octal normie = 0o52
sus binary normie = 0b101010
```

### Floating Point Types
```cursed
sus float_val drip = 3.14159      # f32: 32-bit float
sus double_val meal = 3.14159265359 # f64: 64-bit float
```

#### Float Literals
```cursed
sus scientific drip = 1.23e-4     # Scientific notation
sus no_decimal drip = 42.0        # Explicit decimal
sus decimal_only drip = .5        # Leading zero optional
```

### Character and String Types

#### Character Type
```cursed
sus char_val sip = 'A'           # Single character
sus unicode_char sip = '€'       # Unicode character
sus escape_char sip = '\n'       # Escape sequences
```

#### String Type
```cursed
sus basic_string tea = "Hello, World!"
sus empty_string tea = ""
sus unicode_string tea = "Hello, 世界!"
```

#### String Escape Sequences
```cursed
sus newline tea = "Line 1\nLine 2"
sus tab tea = "Column 1\tColumn 2"
sus quote tea = "She said \"Hello\""
sus backslash tea = "Path\\to\\file"
```

### Boolean Type
```cursed
sus is_true lit = based          # True
sus is_false lit = cap           # False
sus default_bool lit            # Defaults to cap
```

### Nil/Null Type
```cursed
sus null_value = cringe          # Nil/null value
```

## Composite Types

### Arrays

#### Fixed-Size Arrays
```cursed
sus numbers [5]normie = [1, 2, 3, 4, 5]
sus names [3]tea = ["Alice", "Bob", "Charlie"]
```

#### Dynamic Arrays (Slices)
```cursed
sus dynamic_numbers []normie = [1, 2, 3]
sus dynamic_names []tea = ["Alice", "Bob"]
```

#### Array Operations
```cursed
sus arr [3]normie = [10, 20, 30]

# Access elements
sus first normie = arr[0]
sus second normie = arr[1]

# Modify elements
arr[2] = 40

# Array length
sus length normie = len(arr)
```

### Slices

#### Slice Creation
```cursed
sus original []normie = [1, 2, 3, 4, 5]
sus slice_part []normie = original[1:4]  # [2, 3, 4]
sus from_start []normie = original[:3]   # [1, 2, 3]
sus to_end []normie = original[2:]       # [3, 4, 5]
```

#### Slice Operations
```cursed
sus numbers []normie = [1, 2, 3]

# Append elements
numbers = append(numbers, 4)
numbers = append(numbers, 5, 6, 7)

# Copy slices
sus copied []normie = make([]normie, len(numbers))
copy(copied, numbers)
```

### Tuples
```cursed
sus person (tea, normie) = ("Alice", 30)
sus coordinates (drip, drip, drip) = (1.0, 2.0, 3.0)

# Access tuple elements
sus name tea = person.0
sus age normie = person.1

# Tuple destructuring
sus (person_name, person_age) = person
```

### Structs
```cursed
# Define a struct
struct Person {
    name tea
    age normie
    email tea
}

# Create struct instance
sus alice Person = Person{
    name: "Alice",
    age: 30,
    email: "alice@example.com",
}

# Access struct fields
sus name tea = alice.name
alice.age = 31
```

### Maps
```cursed
# Create a map
sus ages map[tea]normie = make(map[tea]normie)

# Add elements
ages["Alice"] = 30
ages["Bob"] = 25

# Access elements
sus alice_age normie = ages["Alice"]

# Check if key exists
sus age, exists = ages["Charlie"]
lowkey exists {
    vibez.spill("Charlie's age:", age)
} highkey {
    vibez.spill("Charlie not found")
}
```

### Channels
```cursed
# Create channels
sus int_channel chan normie = make(chan normie)
sus string_channel chan tea = make(chan tea)

# Buffered channels
sus buffered_channel chan normie = make(chan normie, 10)
```

## Type Conversions

### Explicit Type Conversion
```cursed
sus str tea = "123"
sus num normie = str.(normie)        # String to integer

sus float_val drip = 3.14
sus int_val normie = float_val.(normie)  # Float to integer (truncates)

sus char_val sip = 'A'
sus char_code normie = char_val.(normie)  # Character to ASCII code
```

### Type Conversion Examples
```cursed
# Numeric conversions
sus a normie = 42
sus b drip = a.(drip)               # int to float
sus c smol = a.(smol)               # int to int8 (may overflow)

# String conversions
sus num normie = 42
sus str tea = num.(tea)             # int to string
sus bool_val lit = based
sus bool_str tea = bool_val.(tea)   # bool to string
```

### Safe Type Conversion
```cursed
slay safe_string_to_int(s tea) (normie, tea) {
    # Error handling for invalid conversions
    lowkey s == "" {
        damn 0, "Empty string"
    }
    
    # Try conversion
    result := s.(normie)
    damn result, ""
}
```

## Variable Scope

### Local Variables
```cursed
slay example_function() {
    sus local_var normie = 42      # Local to function
    
    lowkey based {
        sus block_var normie = 24   # Local to block
        vibez.spill(local_var)      # Can access outer scope
    }
    
    # vibez.spill(block_var)       # Error: block_var not in scope
}
```

### Package-Level Variables
```cursed
vibe example_package

sus package_var normie = 100       # Package-level variable

slay use_package_var() {
    vibez.spill(package_var)        # Can access package variable
}
```

## Constants

### Constant Declaration
```cursed
facts PI drip = 3.14159
facts MAX_USERS normie = 1000
facts APP_NAME tea = "MyApp"
```

### Constant Expressions
```cursed
facts SECONDS_IN_HOUR normie = 60 * 60
facts WELCOME_MSG tea = "Welcome to " + APP_NAME
```

## Zero Values

Every type has a zero value:

```cursed
sus zero_int normie              # 0
sus zero_float drip              # 0.0
sus zero_string tea              # ""
sus zero_bool lit                # cap
sus zero_pointer *normie         # cringe
sus zero_slice []normie          # cringe
sus zero_map map[tea]normie      # cringe
sus zero_channel chan normie     # cringe
```

## Advanced Type Features

### Type Aliases
```cursed
be_like UserID normie
be_like EmailAddress tea

sus user_id UserID = 12345
sus email EmailAddress = "user@example.com"
```

### Type Assertions
```cursed
sus value interface{} = 42
sus int_val normie = value.(normie)    # Type assertion

# Safe type assertion
sus int_val, ok = value.(normie)
lowkey ok {
    vibez.spill("Value is an integer:", int_val)
} highkey {
    vibez.spill("Value is not an integer")
}
```

### Generic Types
```cursed
# Generic function
slay max[T](a T, b T) T {
    lowkey a > b {
        damn a
    }
    damn b
}

# Usage
sus int_max normie = max[normie](10, 20)
sus float_max drip = max[drip](3.14, 2.71)
```

## Best Practices

### 1. Use Descriptive Names
```cursed
# Good
sus user_count normie = 100
sus is_authenticated lit = based
sus error_message tea = "Invalid input"

# Bad
sus n normie = 100
sus flag lit = based
sus msg tea = "Invalid input"
```

### 2. Initialize Variables
```cursed
# Good
sus counter normie = 0
sus message tea = ""
sus users []User = make([]User, 0)

# Avoid uninitialized variables (though they have zero values)
sus counter normie  # Will be 0, but be explicit
```

### 3. Use Appropriate Types
```cursed
# Good - use specific types
sus age smol = 25           # Age fits in i8
sus population thicc = 1000000  # Population needs i64

# Bad - using oversized types
sus age thicc = 25          # Wastes memory
```

### 4. Const for Immutable Values
```cursed
# Good
facts MAX_RETRY_ATTEMPTS normie = 3
facts DEFAULT_TIMEOUT drip = 30.0

# Bad - using variables for constants
sus max_retry_attempts normie = 3  # Should be const
```

## Common Type Errors

### Type Mismatch
```cursed
# ❌ Wrong
sus age normie = "25"

# ✅ Correct
sus age normie = 25
sus age_str tea = "25"
```

### Integer Overflow
```cursed
# ❌ Potential overflow
sus small_val smol = 200    # smol is i8, max is 127

# ✅ Correct
sus small_val smol = 100
sus large_val normie = 200
```

### Nil Pointer Dereference
```cursed
# ❌ Dangerous
sus ptr *normie = cringe
sus value normie = *ptr     # Runtime panic

# ✅ Safe
sus ptr *normie = cringe
lowkey ptr != cringe {
    sus value normie = *ptr
}
```

## Type System Benefits

### Memory Safety
- No buffer overflows
- No null pointer dereferences
- Garbage collection prevents memory leaks

### Performance
- Static typing enables optimization
- Zero-cost abstractions
- Efficient memory layout

### Developer Experience
- Clear type errors at compile time
- IDE support with type information
- Self-documenting code

---

Next: [Control Flow →](04-control-flow.md)
