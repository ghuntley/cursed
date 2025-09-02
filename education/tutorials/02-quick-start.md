# CURSED in 30 Minutes

Learn the essentials of CURSED programming in just 30 minutes. This hands-on tutorial covers everything you need to start building real applications.

## Your First CURSED Program

```cursed
# hello.csd
yeet "vibez"

vibez.spill("Hello, CURSED! 🔥")
```

```bash
cursed-zig hello.csd
# Output: Hello, CURSED! 🔥
```

## Variables & Types

CURSED has expressive type names that make code readable:

```cursed
# Variables
sus name tea = "CURSED Developer"    # String
sus age drip = 25                    # Integer
sus score lit = based                # Boolean (true)
sus active lit = cringe              # Boolean (false)

vibez.spill("Name:", name, "Age:", age)
```

**Key Points:**
- `sus` declares variables
- `tea` = string, `drip` = integer, `lit` = boolean
- `based` = true, `cringe` = false

## Functions

Functions use the `slay` keyword and `damn` for returns:

```cursed
# Simple function
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

# Function with multiple parameters
slay add(a drip, b drip) drip {
    damn a + b
}

# Using functions
sus message tea = greet("World")
sus sum drip = add(10, 20)
vibez.spill(message)  # Hello, World!
vibez.spill("Sum:", sum)  # Sum: 30
```

## Control Flow

### If Statements
```cursed
sus score drip = 85

ready (score >= 90) {
    vibez.spill("Excellent!")
} otherwise ready (score >= 70) {
    vibez.spill("Good job!")
} otherwise {
    vibez.spill("Keep trying!")
}
```

### Loops
```cursed
# While loop
sus count drip = 0
bestie (count < 5) {
    vibez.spill("Count:", count)
    count = count + 1
}

# For-each style loop (using arrays)
sus numbers []drip = [1, 2, 3, 4, 5]
# More on arrays below...
```

## Arrays & Collections

```cursed
yeet "arrayz"  # Import array utilities

# Create arrays
sus fruits []tea = ["apple", "banana", "orange"]
sus numbers []drip = [1, 2, 3, 4, 5]

# Access elements
vibez.spill("First fruit:", fruits[0])
vibez.spill("Array length:", arrayz.len(fruits))

# Add elements
arrayz.push(fruits, "grape")
vibez.spill("Updated fruits:", fruits)
```

## Structs

Define custom data types with `squad`:

```cursed
# Define a struct
squad Person {
    name tea,
    age drip,
    active lit
}

# Create instances
sus person Person = Person{
    name: "Alice",
    age: 30,
    active: based
}

# Access fields
vibez.spill("Person:", person.name, "age", person.age)
```

## Error Handling

CURSED has elegant error handling with `yikes`/`fam`:

```cursed
yeet "mathz"

slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero!"
    }
    damn a / b
}

# Handle errors
sus result drip = divide(10, 2) fam {
    when "Division by zero!" -> {
        vibez.spill("Error: Cannot divide by zero")
        damn 0
    }
    when _ -> {
        vibez.spill("Unknown error")
        damn 0
    }
}

vibez.spill("Result:", result)
```

## Concurrency

CURSED makes concurrent programming simple:

```cursed
yeet "concurrenz"

# Create a channel
sus ch chan<drip> = make_channel()

# Goroutine (concurrent function)
go {
    ch <- 42
    vibez.spill("Sent value to channel")
}

# Receive from channel
sus value drip = <-ch
vibez.spill("Received:", value)
```

## Modules & Imports

Organize code with modules:

```cursed
# Import standard library modules
yeet "vibez"    # I/O operations
yeet "mathz"    # Math functions
yeet "stringz"  # String utilities
yeet "filez"    # File operations

# Use imported functions
sus result drip = mathz.abs_normie(-42)
sus upper tea = stringz.to_upper("hello")
vibez.spill("Absolute value:", result)
vibez.spill("Uppercase:", upper)
```

## Your First Complete Program

Let's build a number guessing game:

```cursed
# guessing_game.csd
yeet "vibez"
yeet "mathz"

squad Game {
    secret drip,
    attempts drip
}

slay new_game() Game {
    damn Game{
        secret: mathz.random_range(1, 100),
        attempts: 0
    }
}

slay check_guess(game Game, guess drip) tea {
    game.attempts = game.attempts + 1
    
    ready (guess == game.secret) {
        damn "Congratulations! You won in " + 
             string(game.attempts) + " attempts!"
    } otherwise ready (guess < game.secret) {
        damn "Too low! Try again."
    } otherwise {
        damn "Too high! Try again."
    }
}

# Main game loop
sus game Game = new_game()
vibez.spill("Welcome to the Number Guessing Game!")
vibez.spill("I'm thinking of a number between 1 and 100.")

bestie (based) {
    vibez.spill("Enter your guess:")
    sus input tea = vibez.read_line()
    sus guess drip = parse_int(input) fam {
        when _ -> {
            vibez.spill("Please enter a valid number!")
            continue
        }
    }
    
    sus result tea = check_guess(game, guess)
    vibez.spill(result)
    
    ready (guess == game.secret) {
        break
    }
}
```

## Standard Library Highlights

CURSED includes 50+ standard library modules:

```cursed
# File operations
yeet "filez"
sus content tea = filez.read_file("config.txt")

# JSON handling
yeet "jsonz"
sus data = jsonz.parse("{\"name\": \"CURSED\"}")

# Web requests
yeet "networkz"
sus response tea = networkz.get("https://api.example.com/data")

# Time and dates
yeet "timez"
sus now = timez.now()
vibez.spill("Current time:", timez.format(now))
```

## Building & Running

### Interpreter Mode (Development)
```bash
cursed-zig program.csd
```

### Compile to Binary (Production)
```bash
cursed-zig --compile program.csd
./program
```

### Cross-Compilation
```bash
cursed-zig --compile --target=x86_64-linux program.csd
cursed-zig --compile --target=wasm32-wasi program.csd
```

## Next Steps

🎉 **Congratulations!** You've learned CURSED fundamentals in 30 minutes.

**Continue your journey:**
- [Build Your First Web App](./03-first-app.md)
- [Explore Standard Library](../reference/stdlib/)
- [Advanced Features Tutorial](./advanced/)
- [Join the Community](../community/discord.md)

## Quick Reference Card

```cursed
# Variables
sus name tea = "value"
sus count drip = 42
sus active lit = based

# Functions
slay func_name(param type) return_type {
    damn result
}

# Control Flow
ready (condition) { }
otherwise { }
bestie (condition) { }

# Error Handling
yikes "error message"
result fam { when error -> action }

# Concurrency
go { /* concurrent code */ }
sus ch chan<type> = make_channel()

# Imports
yeet "module_name"
```

**Ready to build something amazing?** 🚀
