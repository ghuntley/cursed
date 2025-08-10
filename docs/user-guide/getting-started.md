# 🚀 Getting Started with CURSED

Welcome to CURSED! This guide will get you up and running with the Gen Z programming language in minutes.

## 📥 Installation

### Quick Install (Recommended)

```bash
# Linux/macOS
curl -sSf https://install.cursedlang.org | sh

# Windows (PowerShell)
iwr https://install.cursedlang.org/windows | iex
```

### Alternative Installation Methods

#### Package Managers
```bash
# Homebrew (macOS)
brew install cursed

# Ubuntu/Debian
sudo apt install cursed

# Arch Linux
sudo pacman -S cursed

# Windows
winget install cursed
```

#### Manual Installation
1. Download from [GitHub Releases](https://github.com/ghuntley/cursed/releases)
2. Extract the archive
3. Add the `bin/` directory to your PATH

#### Build from Source
```bash
git clone https://github.com/ghuntley/cursed.git
cd cursed
direnv allow  # Loads development environment
zig build     # 0.1-0.2 second builds!
```

## ✅ Verify Installation

```bash
cursed --version
# Output: CURSED v1.0.0

cursed --help
# Shows command help
```

## 👋 Your First CURSED Program

Create a file called `hello.csd`:

```cursed
fr Hello World in CURSED
sus greeting tea = "Hello, world!"
vibez.spill(greeting)
```

Run it:
```bash
cursed hello.csd
# Output: Hello, world!
```

## 🎯 Basic Syntax Tour

### Variables
```cursed
sus name tea = "Alice"           # String
sus age drip = 25               # Integer
sus height meal = 5.6           # Float
sus is_awesome lit = based      # Boolean (true)
sus is_boring lit = cap         # Boolean (false)
```

### Arrays
```cursed
sus numbers []drip = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Charlie"]

vibez.spill("First number:", numbers[0])
vibez.spill("Array length:", len(numbers))
```

### Functions
```cursed
slay greet(name tea) {
    vibez.spill("Hey", name, "!")
}

slay add(a drip, b drip) drip {
    damn a + b                  # Return value
}

greet("Alice")                  # Call function
sus sum drip = add(5, 3)       # Use return value
```

### Control Flow
```cursed
fr Conditional statements
ready (age >= 18) {
    vibez.spill("You can vote!")
} otherwise ready (age >= 16) {
    vibez.spill("You can drive!")
} otherwise {
    vibez.spill("Keep growing!")
}

fr Loops
sus i drip = 0
bestie (i < 5) {
    vibez.spill("Count:", i)
    i = i + 1
}

fr For-each style loops
bestie (name in names) {
    vibez.spill("Hello", name)
}
```

### Error Handling
```cursed
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Cannot divide by zero"
    }
    damn a / b
}

fr Handle errors
sus result drip = divide(10, 2) fam {
    when "Cannot divide by zero" -> {
        vibez.spill("Division error!")
        damn 0
    }
}
```

## 📚 Standard Library Basics

### String Operations
```cursed
yeet "stringz"

sus text tea = "Hello World"
sus upper tea = stringz.to_upper(text)
sus words []tea = stringz.split(text, " ")
sus joined tea = stringz.join(words, "-")

vibez.spill("Upper:", upper)
vibez.spill("Words:", words)
vibez.spill("Joined:", joined)
```

### Math Operations
```cursed
yeet "mathz"

sus pi meal = mathz.PI
sus sqrt_result meal = mathz.sqrt(16.0)
sus sin_result meal = mathz.sin(pi / 2)

vibez.spill("Pi:", pi)
vibez.spill("Square root of 16:", sqrt_result)
vibez.spill("Sin(π/2):", sin_result)
```

### Array Operations
```cursed
yeet "arrayz"

sus numbers []drip = [1, 2, 3, 4, 5]
sus doubled []drip = arrayz.map(numbers, slay(x drip) drip { damn x * 2 })
sus sum drip = arrayz.reduce(numbers, 0, slay(acc drip, x drip) drip { damn acc + x })

vibez.spill("Original:", numbers)
vibez.spill("Doubled:", doubled)
vibez.spill("Sum:", sum)
```

## 🧪 Testing Your Code

Create `test_hello.csd`:

```cursed
yeet "testz"

slay test_addition() {
    sus result drip = 2 + 2
    testz.assert_eq_int(result, 4)
    testz.assert_true(result > 0)
}

slay test_strings() {
    sus greeting tea = "Hello"
    testz.assert_eq_string(greeting, "Hello")
    testz.assert_true(len(greeting) == 5)
}

fr Run tests
testz.test_start("Basic Tests")
test_addition()
test_strings()
testz.print_test_summary()
```

Run tests:
```bash
cursed test_hello.csd
# Output: Test results with pass/fail counts
```

## ⚡ Interactive Development

Start the REPL:
```bash
cursed repl
```

Try commands interactively:
```cursed
CURSED> sus x drip = 42
CURSED> vibez.spill("Value:", x)
Value: 42
CURSED> sus doubled drip = x * 2  
CURSED> vibez.spill("Doubled:", doubled)
Doubled: 84
CURSED> exit
```

## 🔧 Development Tools

### Code Formatting
```bash
# Format a single file
cursed format hello.csd

# Format entire directory
cursed format src/

# Check if files are formatted
cursed format --check src/
```

### Code Linting
```bash
# Lint a file
cursed lint hello.csd

# Lint with JSON output
cursed lint --format json src/

# Auto-fix issues
cursed lint --fix src/
```

### Type Checking
```bash
# Type check without running
cursed check hello.csd

# Type check with verbose output
cursed check --verbose hello.csd
```

## 🏗️ Compilation

### Interpret vs Compile
```bash
# Interpret (default, faster for development)
cursed hello.csd

# Compile to native binary
cursed hello.csd --compile
./hello

# Compile with optimizations
cursed hello.csd --compile --optimize=3
```

### Cross-Compilation
```bash
# Compile for different platforms
cursed hello.csd --compile --target=aarch64-macos
cursed hello.csd --compile --target=x86_64-windows
cursed hello.csd --compile --target=wasm32
```

### Debug Information
```bash
# Compile with debug info for GDB/LLDB
cursed hello.csd --compile --debug-info

# Debug with GDB
gdb ./hello
(gdb) break main
(gdb) run
```

## 🎨 IDE Setup

### VS Code
1. Install the CURSED extension:
   ```bash
   code --install-extension cursed-lang.cursed-vscode
   ```
2. Open a `.csd` file to get syntax highlighting and IntelliSense

### Vim/Neovim
Add to your `.vimrc` or `init.vim`:
```vim
Plug 'cursed-lang/vim-cursed'

" LSP configuration (Neovim only)
lua << EOF
require'lspconfig'.cursed.setup{}
EOF
```

### Any Editor with LSP Support
The CURSED Language Server provides:
- Syntax highlighting
- Code completion
- Error diagnostics
- Hover information
- Go to definition
- Code formatting

Start the LSP server:
```bash
cursed-lsp --stdio
```

## 📂 Project Structure

For larger projects, use this structure:
```
my-cursed-project/
├── src/
│   ├── main.csd          # Entry point
│   ├── utils.csd         # Utility functions
│   └── models/           # Data models
├── test/
│   ├── test_main.csd     # Tests
│   └── test_utils.csd
├── docs/                 # Documentation
├── CursedPackage.toml    # Package configuration
└── README.md
```

Create `CursedPackage.toml`:
```toml
[package]
name = "my-project"
version = "0.1.0"
description = "My awesome CURSED project"
authors = ["Your Name <you@example.com>"]

[dependencies]
# Add dependencies here

[build]
entry_point = "src/main.csd"
optimize = true
target = "native"
```

## 🚀 Next Steps

Now that you have the basics, explore:

1. **[Language Reference](language-reference.md)** - Complete syntax guide
2. **[Standard Library](../api/)** - 50+ module documentation  
3. **[Examples](../../examples/)** - Real-world code examples
4. **[Migration Guides](../migration/)** - From other languages
5. **[Advanced Features](advanced-features.md)** - Generics, concurrency, etc.

## ❓ Getting Help

- 📖 **Documentation**: This guide and API reference
- 💬 **Discord**: [discord.gg/cursed-lang](https://discord.gg/cursed-lang)
- 🐛 **Issues**: [GitHub Issues](https://github.com/ghuntley/cursed/issues)
- 💡 **Discussions**: [GitHub Discussions](https://github.com/ghuntley/cursed/discussions)

Welcome to the CURSED community! 🔥
