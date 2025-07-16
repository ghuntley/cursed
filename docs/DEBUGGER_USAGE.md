# CURSED Debugger Usage Guide

The CURSED language provides comprehensive debugging capabilities through its integrated debugger with GDB and LLDB support.

## Features

- **Full DWARF v5 debug information generation**
- **Native GDB and LLDB integration**
- **Interactive debugging with breakpoints and watchpoints**
- **Source-level debugging with line mapping**
- **Variable inspection and modification**
- **Stack trace analysis**
- **Memory and register examination**
- **Multi-threaded debugging support**

## Installation Requirements

### GDB Support
```bash
# Install GDB (version 8.0 or later recommended)
sudo apt-get install gdb              # Ubuntu/Debian
sudo yum install gdb                  # CentOS/RHEL
brew install gdb                      # macOS

# Verify installation
gdb --version
```

### LLDB Support
```bash
# Install LLDB (version 10.0 or later recommended)
sudo apt-get install lldb             # Ubuntu/Debian
sudo yum install lldb                 # CentOS/RHEL
brew install llvm                     # macOS (includes LLDB)

# Verify installation
lldb --version
```

## Quick Start

### 1. Compile with Debug Information

```bash
# Compile CURSED program with debug symbols
cargo run --bin cursed -- compile --debug program.csd

# Or use the debugger to compile automatically
cargo run --bin cursed_debug program.csd
```

### 2. Start Debug Session

```bash
# Start interactive debugger
cargo run --bin cursed_debug program.csd

# Force specific debugger
cargo run --bin cursed_debug --gdb program.csd
cargo run --bin cursed_debug --lldb program.csd

# Attach to running process
cargo run --bin cursed_debug --attach 1234 program.csd

# Run debug script
cargo run --bin cursed_debug --script debug_commands.txt program.csd
```

## Interactive Debugger Commands

### Basic Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `help` | `h` | Show help information |
| `run [args]` | `r` | Run the program with optional arguments |
| `quit` | `q` | Exit debugger |

### Breakpoint Management

| Command | Alias | Description |
|---------|-------|-------------|
| `break <location>` | `b` | Set breakpoint at location |
| `break` | `b` | List all breakpoints |
| `delete <id>` | `d` | Delete breakpoint by ID |
| `enable <id>` | `en` | Enable breakpoint |
| `disable <id>` | `dis` | Disable breakpoint |

#### Breakpoint Locations

```bash
# Function name
break main
break factorial

# File and line
break main.csd:15
break src/lib.csd:42

# Address
break *0x400500

# Conditional breakpoint
break main if x > 10
```

### Execution Control

| Command | Alias | Description |
|---------|-------|-------------|
| `continue` | `c` | Continue execution |
| `step` | `s` | Step into (source line) |
| `next` | `n` | Step over (source line) |
| `finish` | `f` | Step out of current function |

### Stack and Frame Navigation

| Command | Alias | Description |
|---------|-------|-------------|
| `backtrace` | `bt` | Show stack trace |
| `frame [n]` | `fr` | Select frame (or show current) |
| `up` | | Move up one frame |
| `down` | | Move down one frame |

### Variable Inspection

| Command | Alias | Description |
|---------|-------|-------------|
| `print <expr>` | `p` | Print variable or expression |
| `watch <var>` | `w` | Watch variable for changes |
| `info variables` | `i var` | Show all variables in scope |
| `info locals` | `i loc` | Show local variables |
| `set <var> <value>` | | Set variable value |

#### Print Examples

```bash
# Print variable
print x
print factorial_result

# Print with format
print/x address_var          # Hexadecimal
print/d counter              # Decimal
print/c character_var        # Character

# Print complex expressions
print x + y * z
print array[index]
print struct_var.field_name
```

### Memory and Registers

| Command | Alias | Description |
|---------|-------|-------------|
| `info registers` | `i r` | Show all registers |
| `info threads` | `i th` | Show all threads |
| `disassemble` | `disas` | Show assembly code |
| `list [location]` | `l` | Show source code |

### Source Code Display

```bash
# List current location
list

# List specific function
list main
list factorial

# List file and line
list main.csd:10

# List with context
list main.csd:10,20
```

## Example Debug Session

### Sample CURSED Program (factorial.csd)

```cursed
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay main() normie {
    sus result normie = factorial(5)
    vibez.spill("Factorial of 5 is: ")
    vibez.spill(result)
    damn 0
}
```

### Debug Session

```bash
$ cargo run --bin cursed_debug factorial.csd
🐛 CURSED Debugger v1.0
📁 Source file: factorial.csd
🔧 Compiling factorial.csd with debug information...
✅ Compiled successfully: factorial_debug
🚀 Starting debug session for factorial_debug
Type 'help' for available commands

(cursed-debug) break main
🔴 Setting breakpoint at: main
Breakpoint 1 set at main

(cursed-debug) break factorial if n == 1
🔴 Setting breakpoint at: factorial if n == 1
Breakpoint 2 set at factorial with condition 'n == 1'

(cursed-debug) run
🏃 Running program with args: []
Breakpoint 1 hit at main() (factorial.csd:9)

(cursed-debug) step
👣 Stepping into...
factorial(n=5) (factorial.csd:2)

(cursed-debug) print n
🔍 Printing variable: n
$1 = 5

(cursed-debug) continue
▶️ Continuing execution...
Breakpoint 2 hit at factorial(n=1) (factorial.csd:3)

(cursed-debug) backtrace
📚 Stack trace:
#0  factorial (n=1) at factorial.csd:3
#1  factorial (n=2) at factorial.csd:5
#2  factorial (n=3) at factorial.csd:5
#3  factorial (n=4) at factorial.csd:5
#4  factorial (n=5) at factorial.csd:5
#5  main () at factorial.csd:9

(cursed-debug) print n
🔍 Printing variable: n
$2 = 1

(cursed-debug) continue
▶️ Continuing execution...
Factorial of 5 is: 120
Program exited normally (exit code: 0)

(cursed-debug) quit
👋 Debug session ended
```

## Advanced Features

### Conditional Breakpoints

```bash
# Break when variable equals value
break factorial if n == 1

# Break when condition is true
break process_data if count > 100

# Break with complex expressions
break main if strlen(name) > 10 && flag == based
```

### Watchpoints

```bash
# Watch variable for changes
watch global_counter

# Watch memory location
watch *0x7fff0000

# Watch with conditions
watch result if result > 1000
```

### Multi-threaded Debugging

```bash
# List all threads
info threads

# Switch to specific thread
thread 2

# Break in specific thread
break function_name thread 1

# Continue specific thread
thread 1
continue
```

### Memory Examination

```bash
# Examine memory at address
x/16xb 0x7fff0000        # 16 bytes in hex
x/8xw 0x400500           # 8 words in hex
x/4s 0x7fff1000          # 4 strings

# Examine variable memory
x/16xb &variable_name

# Show memory mappings
info proc mappings
```

### Advanced Scripting

Create a debug script file (`debug_script.txt`):

```bash
# Set multiple breakpoints
break main
break factorial
break process_result

# Set watchpoints
watch global_state

# Run program
run

# Custom commands for specific debugging
define print_state
    print current_value
    print state_flag
    print error_count
end

# Automated testing
commands 1
    print n
    continue
end
```

Run with script:

```bash
cargo run --bin cursed_debug --script debug_script.txt program.csd
```

## Troubleshooting

### Common Issues

#### 1. No Debug Information

```bash
Error: No debug information found
```

**Solution**: Compile with debug flags:
```bash
cargo run --bin cursed -- compile --debug program.csd
```

#### 2. Debugger Not Found

```bash
Error: No compatible debugger found
```

**Solution**: Install GDB or LLDB:
```bash
sudo apt-get install gdb lldb
```

#### 3. Breakpoint Not Hit

```bash
Warning: Breakpoint not resolved
```

**Solutions**:
- Check function/variable names for typos
- Ensure debug information is available
- Verify the code path is executed

#### 4. Variable Not Available

```bash
Error: No symbol "variable_name" in current context
```

**Solutions**:
- Check variable scope
- Ensure you're in the correct frame
- Verify variable name spelling

### Performance Tips

1. **Disable Optimization**: Use `--opt-level 0` for best debugging experience
2. **Minimal Breakpoints**: Too many breakpoints can slow execution
3. **Use Conditional Breakpoints**: More efficient than breaking and checking manually
4. **Watch Variables Sparingly**: Watchpoints can significantly impact performance

### Platform-Specific Notes

#### Linux
- Full GDB and LLDB support
- DWARF debug information works natively
- Core dump debugging available

#### macOS
- LLDB preferred over GDB
- May need to sign GDB for debugging
- Some GDB features limited

#### Windows
- Limited GDB support
- LLDB experimental
- Consider using Windows-specific debuggers

## Integration with IDEs

### VS Code

Install the CURSED debugger extension and configure `launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug CURSED Program",
            "type": "cursed",
            "request": "launch",
            "program": "${workspaceFolder}/program.csd",
            "debugger": "gdb",
            "stopOnEntry": false,
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

### Vim/Neovim

Use the CURSED LSP server with debugging support:

```vim
" Configure CURSED debugging
let g:cursed_debugger = 'gdb'
let g:cursed_debug_flags = ['--debug', '--opt-level', '0']

" Set breakpoint
nnoremap <F9> :CursedBreakpoint<CR>

" Start debugging
nnoremap <F5> :CursedDebug<CR>
```

### Emacs

Configure GUD mode for CURSED:

```elisp
;; Add CURSED debugger support
(add-to-list 'gud-debugger-list
  '(cursed-gdb "gdb" gud-gdb-marker-filter gud-gdb-find-file))

;; Key bindings
(global-set-key (kbd "<f9>") 'gud-break)
(global-set-key (kbd "<f5>") 'gud-run)
(global-set-key (kbd "<f10>") 'gud-next)
(global-set-key (kbd "<f11>") 'gud-step)
```

## Best Practices

### 1. Compile for Debugging
Always compile with debug information and minimal optimization:
```bash
cargo run --bin cursed -- compile --debug --opt-level 0 program.csd
```

### 2. Use Meaningful Variable Names
Debug-friendly code uses descriptive names:
```cursed
# Good
sus user_count normie = get_active_users()

# Poor
sus x normie = get_active_users()
```

### 3. Strategic Breakpoint Placement
- Set breakpoints at function entry points
- Use conditional breakpoints for loops
- Place breakpoints before critical operations

### 4. Leverage Stack Traces
Use `backtrace` to understand call hierarchy and identify the root cause of issues.

### 5. Combine Tools
Use both debugger and logging for comprehensive debugging:
```cursed
# Add debug logging
lowkey debug_enabled {
    vibez.spill("Entering function with param: " + param)
}
```

### 6. Test Edge Cases
Use the debugger to test boundary conditions:
```cursed
# Test with various inputs
factorial(0)  # Edge case
factorial(1)  # Base case  
factorial(10) # Normal case
```

## API Reference

### CURSED Debugger API

The debugger can be integrated programmatically:

```rust
use cursed::debug::{DwarfDebugGenerator, GdbIntegration, LldbIntegration};

// Generate debug information
let mut generator = DwarfDebugGenerator::new();
generator.generate_debug_info(&ast, "program.csd")?;
let debug_sections = generator.encode_debug_sections()?;

// Start GDB session
let mut gdb = GdbIntegration::new();
gdb.start_gdb("program_debug")?;
gdb.set_breakpoint("main")?;
gdb.run_program(&[])?;

// Start LLDB session
let mut lldb = LldbIntegration::new();
lldb.start_lldb("program_debug")?;
lldb.set_breakpoint("main")?;
lldb.run_program(&[])?;
```

## Contributing

To contribute to the CURSED debugger:

1. **Add New Features**: Extend debugger integration modules
2. **Improve DWARF Generation**: Enhance debug information completeness
3. **Platform Support**: Add support for additional platforms
4. **Documentation**: Improve usage examples and troubleshooting guides

### Development Setup

```bash
# Clone and build
git clone https://github.com/cursed/cursed.git
cd cursed
cargo build

# Run tests
cargo test debug

# Test debugger integration
cargo run --bin cursed_debug examples/factorial.csd
```

## References

- [DWARF Debugging Format](http://dwarfstd.org/)
- [GDB Documentation](https://www.gnu.org/software/gdb/documentation/)
- [LLDB Documentation](https://lldb.llvm.org/use/tutorial.html)
- [CURSED Language Specification](./LANGUAGE_SPEC.md)
