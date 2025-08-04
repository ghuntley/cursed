# Getting Started with CURSED

Welcome to CURSED - a modern, self-hosting systems programming language designed for safety, performance, and developer experience.

## Installation

### Prerequisites

- **Rust** (for building the compiler)
- **LLVM** (for native compilation)
- **Git** (for cloning the repository)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build the compiler
cargo build --release

# Verify installation
cargo run --bin cursed -- --version
```

### Development Environment Setup

For the best development experience, install:

1. **VS Code Extension** (recommended)
2. **CURSED LSP Server** (included)
3. **Syntax Highlighting** (built-in)

## Your First CURSED Program

Create a file called `hello.csd`:

```cursed
vibez.spill("Hello, CURSED!")
```

Run it:

```bash
cargo run --bin cursed hello.csd
```

Output:
```
Hello, CURSED!
```

## Basic Program Structure

Every CURSED program consists of:

1. **Imports** (if needed)
2. **Function definitions**
3. **Main execution** (implicit)

```cursed
# Import modules
yeet "math"
yeet "string"

# Define a function
slay greet(name tea) {
    vibez.spill("Hello, " + name + "!")
}

# Main execution
greet("World")
```

## Compilation vs Interpretation

CURSED supports both modes:

### Interpretation (Development)
```bash
cargo run --bin cursed program.csd
```

### Native Compilation (Production)
```bash
cargo run --bin cursed -- compile program.csd
./program
```

## Language Characteristics

### Type Safety
```cursed
sus age normie = 25          # Integer
sus name tea = "Alice"       # String
sus active lit = based       # Boolean (true)
sus inactive lit = cap       # Boolean (false)
```

### Memory Safety
```cursed
# No manual memory management
# Automatic garbage collection
# Safe pointer operations
```

### Concurrency
```cursed
# Goroutines (lightweight threads)
yolo background_task()

# Channels for communication
sus ch chan normie = make(chan normie)
```

## Basic Syntax Rules

### Variables
```cursed
sus variable_name type = value
sus x normie = 42
sus message tea = "Hello"
```

### Comments
```cursed
# Single line comment
fr fr this is also a comment
no cap this is a comment too
on god this is also a comment
```

### Functions
```cursed
slay function_name(param1 type1, param2 type2) return_type {
    # Function body
    damn return_value
}
```

### Control Flow
```cursed
# If statements
lowkey condition {
    # do something
} highkey {
    # do something else
}

# Loops
bestie i := 0; i < 10; i++ {
    vibez.spill(i)
}
```

## Built-in Functions

### Output
```cursed
vibez.spill("Hello")        # Print string
vibez.spill(42)             # Print number
vibez.spill(based)          # Print boolean
```

### Type Conversion
```cursed
sus str tea = "123"
sus num normie = str.(normie)  # String to integer
```

## Next Steps

1. **Practice** with simple programs
2. **Read** the [Basic Syntax](02-basic-syntax.md) guide
3. **Explore** the [Variables and Types](03-variables-types.md) tutorial
4. **Join** the CURSED community

## Common Mistakes

### ❌ Wrong
```cursed
sus x = 42           # Missing type
print("Hello")       # Wrong function name
```

### ✅ Correct
```cursed
sus x normie = 42    # Explicit type
vibez.spill("Hello") # Correct function name
```

## Help and Resources

- **Documentation**: [docs/index.md](../index.md)
- **Examples**: [examples/](../../examples/)
- **FAQ**: [faq.md](../../faq.md)
- **Issues**: [GitHub Issues](https://github.com/ghuntley/cursed/issues)

---

Next: [Basic Syntax →](02-basic-syntax.md)
