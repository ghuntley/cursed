# CURSED REPL Guide

The CURSED REPL (Read-Eval-Print Loop) provides an interactive environment for developing and testing CURSED code. This guide covers all the features and commands available in the REPL.

## Getting Started

### Starting the REPL

```bash
# Basic REPL
cursed repl

# REPL with command history
cursed repl --history

# Verbose REPL with detailed output
cursed repl --verbose --history
```

### First Steps

When you start the REPL, you'll see:

```
🔥 CURSED REPL v0.1.0
Welcome to the most fire programming language! 🚀
Type :help for available commands or :exit to quit

📁 Working directory: /path/to/your/project

cursed> 
```

## Basic Usage

### Expressions

The REPL can evaluate expressions directly:

```cursed
cursed> 2 + 3
5

cursed> 42 * 1.5
63

cursed> "Hello" + " World"
"Hello World"

cursed> true && false
false
```

### Variables

Declare and use variables using CURSED syntax:

```cursed
cursed> facts age = 25
Variable age declared

cursed> sus score = 100.5
Variable score declared

cursed> age + 5
30

cursed> score * 2
201
```

### Functions

Define and call functions:

```cursed
cursed> slay greet(name) { println("Hello " + name) }
Function greet declared

cursed> greet("Taylor")
Hello Taylor

cursed> slay add(a, b) { periodt a + b }
Function add declared

cursed> add(10, 20)
30
```

### Multi-line Input

The REPL automatically detects when you need to continue typing:

```cursed
cursed> slay complex_function() {
...   >     facts x = 42
...   >     facts y = x * 2
...   >     periodt y
...   > }
Function complex_function declared
```

## Built-in Commands

All commands start with `:` (colon).

### Help and Information

- `:help` - Show all available commands
- `:help <command>` - Show help for specific command
- `:info` - Show REPL and project information
- `:version` - Show version information

### Session Management

- `:clear` - Clear the current session state
- `:vars` - List all variables in current session
- `:funcs` - List all functions in current session
- `:history [count]` - Show command history
- `:type <expression>` - Show the type of an expression

### File Operations

- `:load <file>` - Load and execute a CURSED file
- `:save <file>` - Save current session to a file

### Development Tools

- `:build [target]` - Build the current project
- `:test [pattern]` - Run project tests
- `:fmt [file]` - Format code in session or file
- `:lint [file]` - Run linter on session or project

### Workspace

- `:workspace` - Show workspace information

### Exit

- `:exit` - Exit the REPL
- `:quit` - Exit the REPL
- `:q` - Exit the REPL (shortcut)

## Advanced Features

### Syntax Highlighting

The REPL provides real-time syntax highlighting for CURSED code:

- **Keywords**: `slay`, `facts`, `sus`, `lowkey`, etc. (magenta)
- **Operators**: `+`, `-`, `*`, `/`, `=`, etc. (yellow)
- **Strings**: Text in quotes (green)
- **Numbers**: Numeric literals (cyan)
- **Comments**: Comment text (gray)

### Tab Completion

Press `Tab` to auto-complete:

- CURSED keywords
- Variable names from current session
- Function names from current session
- Built-in functions
- REPL commands
- File paths (for `:load` and `:save`)

### Command History

- Use ↑/↓ arrow keys to navigate command history
- Use `:history` to see recent commands
- History is saved between sessions (when `--history` is enabled)

### Error Handling

The REPL provides helpful error messages:

```cursed
cursed> facts = 42
🔥 Error: Invalid variable declaration syntax

cursed> unknown_function()
🔥 Error: Unknown function: unknown_function

cursed> facts x = 
🔥 Error: Parse error: Unexpected end of input
```

### Project Integration

When run in a project directory, the REPL provides additional features:

- `:build` - Build the project using CursedBuild.toml or Makefile
- `:test` - Run project tests
- Automatic discovery of project structure
- Integration with project dependencies

## Example Session

Here's a complete example session:

```cursed
🔥 CURSED REPL v0.1.0
Welcome to the most fire programming language! 🚀

cursed> facts name = "Taylor"
Variable name declared

cursed> facts age = 25
Variable age declared

cursed> slay introduce() {
...   >     println("Hi, I'm " + name)
...   >     println("I'm " + age + " years old")
...   > }
Function introduce declared

cursed> introduce()
Hi, I'm Taylor
I'm 25 years old

cursed> :vars
📋 Session Variables:
  name : string = "Taylor"
  age : int = 25

cursed> :funcs
🔧 Session Functions:
  introduce() { ... }

cursed> :type name
🔍 Type: string

cursed> age * 2
50

cursed> :save my_session.csd
✅ Session saved to: my_session.csd

cursed> :exit
👋 Thanks for using CURSED! Keep it fire! 🔥
```

## Tips and Tricks

### 1. Use Variables for Complex Expressions

Instead of retyping complex expressions:

```cursed
cursed> facts complex_calc = (42 * 3.14159) / 2.0
cursed> complex_calc + 10
```

### 2. Test Functions Incrementally

Build functions step by step:

```cursed
cursed> facts x = 10
cursed> facts y = 20
cursed> x + y  // Test the logic first
30
cursed> slay add(a, b) { periodt a + b }  // Then wrap in function
cursed> add(x, y)  // Test the function
30
```

### 3. Use :load for Larger Code

For multi-line code, save to a file and use `:load`:

```bash
# Edit in your favorite editor
vim test.csd

# Load in REPL
cursed> :load test.csd
```

### 4. Check Types When Debugging

Use `:type` to understand expression types:

```cursed
cursed> facts result = some_complex_function()
cursed> :type result
🔍 Type: float64
```

### 5. Save Your Work

Use `:save` regularly to preserve your session:

```cursed
cursed> :save backup_$(date +%Y%m%d).csd
```

## Keyboard Shortcuts

- **Ctrl+C**: Interrupt current input (returns to prompt)
- **Ctrl+D**: Exit REPL
- **↑/↓**: Navigate command history
- **Tab**: Auto-complete
- **Ctrl+L**: Clear screen (if supported by terminal)

## Configuration

The REPL can be configured through command-line options:

```bash
# Enable all features
cursed repl --history --verbose

# Disable syntax highlighting
cursed repl --no-syntax-highlighting

# Set custom timeout
cursed repl --timeout 60
```

## Troubleshooting

### REPL Won't Start

1. Check that CURSED is properly installed
2. Verify you have necessary permissions
3. Try running with `--verbose` for more information

### Syntax Highlighting Not Working

1. Check that your terminal supports ANSI colors
2. Try `cursed repl --no-colors` to disable colors
3. Update your terminal if needed

### Commands Not Working

1. Make sure commands start with `:`
2. Use `:help` to see available commands
3. Check spelling and syntax

### Performance Issues

1. Use `:clear` to reset session state
2. Avoid defining too many variables/functions in one session
3. Save and restart REPL if needed

## Getting Help

- Use `:help` in the REPL for quick reference
- Check the CURSED documentation for language features
- Report bugs or request features on GitHub

That's everything you need to know to be absolutely fire with the CURSED REPL! 🔥🚀
