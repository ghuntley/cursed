# Getting Started with CURSED

Welcome to CURSED, a modern systems programming language with Gen Z-inspired syntax and powerful performance features.

## Installation

### Prerequisites
- Zig 0.11+ (development environment)
- Linux/macOS (Windows support via WSL)
- 4GB+ RAM for compilation

### Quick Install
```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed
cd cursed

# Build the compiler
zig build

# Verify installation
./zig-out/bin/cursed-zig --version
```

## Your First CURSED Program

Create a file called `hello.csd`:

```cursed
# hello.csd - Your first CURSED program
vibez.spill("Hello, world!")
```

Run it:
```bash
./zig-out/bin/cursed-zig hello.csd
```

Output:
```
Hello, world!
```

## Basic Syntax Tour

### Variables
```cursed
# Integer variables
sus age drip = 25
sus count drip = 42

# String variables  
sus name tea = "Alex"
sus message tea = "Welcome to CURSED!"

# Boolean variables
sus is_cool lit = based    # true
sus is_boring lit = cringe # false
```

### Functions
```cursed
# Function definition
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

# Function with multiple parameters
slay add(x drip, y drip) drip {
    damn x + y
}

# Using functions
sus greeting tea = greet("CURSED")
sus sum drip = add(10, 20)
vibez.spill(greeting)  # Hello, CURSED!
vibez.spill(sum)       # 30
```

### Control Flow
```cursed
# If statements
sus score drip = 85
ready (score >= 90) {
    vibez.spill("A grade!")
} otherwise ready (score >= 80) {
    vibez.spill("B grade!")
} otherwise {
    vibez.spill("Keep trying!")
}

# While loops
sus counter drip = 0
bestie (counter < 5) {
    vibez.spill("Count: ", counter)
    counter = counter + 1
}
```

### Arrays
```cursed
# Array creation
sus numbers []drip = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Charlie"]

# Array access
vibez.spill(numbers[0])    # 1
vibez.spill(names[1])      # Bob

# Array length
yeet "arrayz"
vibez.spill(len(numbers))  # 5
```

### Standard Library
```cursed
# Math operations
yeet "mathz"
sus absolute drip = abs_normie(-42)    # 42
sus maximum drip = max_normie(10, 20)  # 20

# String operations
yeet "stringz"
sus length drip = len_tea("hello")     # 5
sus upper tea = upper_tea("hello")     # HELLO

# Testing framework
yeet "testz"
test_start("My first test")
assert_eq_int(2 + 2, 4)
assert_eq_string("hello", "hello")
print_test_summary()
```

## Project Structure

### Basic Project Layout
```
my-cursed-project/
├── src/
│   ├── main.csd
│   ├── utils.csd
│   └── models/
│       └── user.csd
├── tests/
│   └── test_main.csd
├── CursedPackage.toml
└── README.md
```

### Package Configuration
```toml
# CursedPackage.toml
[package]
name = "my-project"
version = "0.1.0"
author = "Your Name"

[dependencies]
# External dependencies go here

[build]
target = "native"
optimize = true
```

## Development Workflow

### Basic Commands
```bash
# Run your program
./zig-out/bin/cursed-zig src/main.csd

# Compile to native binary
./zig-out/bin/cursed-zig --compile src/main.csd

# Type checking only
./zig-out/bin/cursed-zig check src/main.csd

# Format code
./zig-out/bin/cursed-zig format src/main.csd

# Memory safety check
valgrind ./zig-out/bin/cursed-zig src/main.csd
```

### Testing
```cursed
# test_example.csd
yeet "testz"

test_start("Math operations")
assert_eq_int(2 + 3, 5)
assert_eq_int(10 - 4, 6)
assert_true(5 > 3)
assert_false(2 > 5)

test_start("String operations")
assert_eq_string("hello" + " world", "hello world")
assert_true(len_tea("test") == 4)

print_test_summary()
```

Run tests:
```bash
./zig-out/bin/cursed-zig test_example.csd
```

## Common Patterns

### Error Handling
```cursed
# Using built-in error handling
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero"
    }
    damn a / b
}

# Handle errors
sus result drip = divide(10, 2) fam {
    when "Division by zero" -> {
        vibez.spill("Cannot divide by zero!")
        damn 0
    }
    when other -> {
        vibez.spill("Unexpected error:", other)
        shook  # Panic
    }
}
```

### Pattern Matching
```cursed
# Enum definition
enum Color { Red, Green, Blue }

# Pattern matching
sus color Color = Red
sick (color) {
    when Red -> vibez.spill("Stop!")
    when Green -> vibez.spill("Go!")
    when Blue -> vibez.spill("Caution!")
}
```

### Struct Definition
```cursed
# Define a struct
squad Person {
    spill name tea
    spill age drip
    spill email tea
}

# Create and use struct
sus person Person = Person{
    name: "Alice",
    age: 30,
    email: "alice@example.com"
}

vibez.spill("Name:", person.name)
vibez.spill("Age:", person.age)
```

## Next Steps

1. **[Language Reference](language-reference.md)** - Complete syntax documentation
2. **[Standard Library](stdlib/)** - Explore available modules
3. **[Examples](../examples/)** - Real-world code examples
4. **[Best Practices](best-practices.md)** - Writing efficient CURSED code

## Getting Help

- **Documentation**: Check the [FAQ](../support/faq.md) for common questions
- **Troubleshooting**: See [troubleshooting guide](../support/troubleshooting.md)
- **Community**: Visit our [GitHub discussions](https://github.com/ghuntley/cursed/discussions)
- **Issues**: Report bugs on [GitHub Issues](https://github.com/ghuntley/cursed/issues)

Welcome to the CURSED community! 🔥
