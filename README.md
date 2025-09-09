# CURSED 💀

> An esoteric programming language that combines Go-like semantics with Gen Z slang keywords, featuring the world's first use of Among Us `ඞ` characters in pointer syntax.

CURSED is a statically typed, garbage-collected programming language designed to make coding more expressive and culturally relevant while maintaining practical functionality. It's self-hosting through a multi-stage bootstrap compiler written in Zig.

## Features

- **Gen Z Slang Keywords**: Core programming constructs use contemporary slang (`slay` for function, `sus` for variable, `vibe` for package, `stan` for goroutine)
- **Among Us Pointer Syntax**: First programming language to use `ඞ` (U+0D9E) for pointer operations
- **Go-like Semantics**: Familiar control flow and typing for experienced developers
- **Built-in Concurrency**: Goroutines (`stan`) and channels (`dm`) for concurrent programming
- **Advanced Memory Management**: Generational garbage collector with performance monitoring
- **Comprehensive Error Handling**: `yikes`/`fam`/`shook` error system with panic recovery

## Quick Start

### Hello World

```cursed
vibe main
yeet "vibez"

slay main() {
    vibez.spill("Hello, World!")
}
```

### Variables and Types

```cursed
vibe example
yeet "vibez", "stringz"

slay demo() {
    sus age normie = 25          // 32-bit integer
    sus name tea = "Alice"       // string
    sus is_cool lit = based      // boolean (true)
    sus height snack = 5.8       // 32-bit float
    
    // Pointer operations with Among Us syntax
    sus ptr ඞnormie = ඞage       // pointer to age
    sus value normie = *ptr      // dereference
    
    vibez.spill("Name:", name)
}
```

### Functions and Control Flow

```cursed
slay calculate(x normie, y normie) normie {
    ready x > y {
        damn x + y
    } otherwise {
        damn x - y
    }
}

slay loop_example() {
    bestie i := 0; i < 10; i++ {
        ready i % 2 == 0 {
            vibez.spill("Even:", i)
        }
    }
}
```

### Concurrency

```cursed
slay worker_example() {
    sus ch dm<normie>           // channel
    
    // Spawn goroutine
    stan {
        dm_send(ch, 42)
    }
    
    // Receive from channel
    result := dm_recv(ch)
    vibez.spill("Received:", result)
}
```

### Error Handling

```cursed
slay divide(a normie, b normie) normie yikes {
    ready b == 0 {
        yikes "Division by zero"
    }
    damn a / b
}

slay safe_division() {
    fam {
        result := divide(10, 0) shook
        vibez.spill("Result:", result)
    } sus error {
        vibez.spill("Error:", error.message())
    }
}
```

## Language Overview

### Keywords Mapping

| Traditional | CURSED | Usage |
|-------------|--------|-------|
| package | `vibe` | Package declaration |
| import | `yeet` | Import packages |
| func | `slay` | Function definition |
| var | `sus` | Variable declaration |
| const | `facts` | Constant declaration |
| if | `ready` | Conditional |
| else | `otherwise` | Alternative branch |
| for | `bestie` | Loop |
| while | `periodt` | While loop |
| return | `damn` | Return statement |
| true | `based` | Boolean true |
| false | `cringe` | Boolean false |
| nil | `nah` | Null value |
| go | `stan` | Spawn goroutine |

### Types

| Type | Description |
|------|-------------|
| `lit` | Boolean |
| `normie` | 32-bit signed integer |
| `smol` | 8-bit signed integer |
| `mid` | 16-bit signed integer |
| `thicc` | 64-bit signed integer |
| `snack` | 32-bit float |
| `meal` | 64-bit float |
| `tea` | String |
| `sip` | Character |
| `ඞT` | Pointer to type T |
| `dm<T>` | Channel of type T |

## Installation

### Prerequisites

- Zig 0.13+ (compiler implementation)
- Git

The CURSED compiler is now implemented in Zig with built-in LLVM support, eliminating external LLVM dependencies and enabling cross-platform compilation including Windows.

### Building from Source

```bash
git clone https://github.com/ghuntley/cursed
cd cursed
zig build
```

### Running Programs

```bash
# Interpret and run a CURSED program
./zig-out/bin/cursed-compiler example.💀

# Compile to native executable
./zig-out/bin/cursed-compiler --compile example.💀

# Generate LLVM IR
./zig-out/bin/cursed-compiler --emit-ir example.💀

# Debug mode
./zig-out/bin/cursed-compiler --debug --verbose example.💀
```

## Project Structure

```
├── src-zig/            # Zig compiler implementation source
├── runtime/            # Runtime library and garbage collector  
├── stdlib/             # Standard library modules
├── specs/              # Language specifications
├── test_suite/         # Comprehensive tests (including LeetCode suite)
├── tools/              # Development tools
└── build.zig           # Zig build configuration
```

## Development Status

CURSED has evolved through multiple implementation phases:

1. ✅ **Stage 0**: Environment setup and language design
2. ✅ **Stage 1**: Zig-native compiler with LLVM backend  
3. 🚧 **Stage 2**: Advanced features and optimization
4. 🔮 **Stage 3**: Self-hosting compiler in CURSED

**Current Status**: Full compiler implemented in Zig with comprehensive language support, built-in LLVM backend, and extensive test suite including 17+ LeetCode problems.

## Contributing

We welcome contributions! Please see:

- Language specifications in [`specs/`](specs/)
- Open issues for good first contributions
- Development roadmap in project milestones

### Development Guidelines

1. Follow the language specifications in `specs/`
2. Add tests for new features
3. Update documentation
4. Use conventional commit messages

## Documentation

- [Language Specifications](specs/) - Complete language documentation
- [Grammar Reference](specs/grammar.md) - Syntax and grammar rules
- [Type System](specs/types.md) - Type system details
- [Concurrency Model](specs/concurrency.md) - Goroutines and channels
- [Memory Management](specs/memory_management.md) - GC and memory model
- [Error Handling](specs/error_handling.md) - Error patterns and recovery

## Examples

### LeetCode Test Suite

The [`test_suite/leetcode_comprehensive_suite/`](test_suite/leetcode_comprehensive_suite/) contains **17+ LeetCode problems** implemented in CURSED, demonstrating:

- **Complex algorithms**: Binary search, dynamic programming, backtracking
- **Data structures**: Linked lists and trees with `ඞ` pointers  
- **String manipulation**: Anagram detection, palindrome validation
- **Array operations**: Two Sum, 3Sum, Product Except Self
- **Meme implementations**: FizzBuzz with "Sus" and "Impostor"

```cursed
// LeetCode #206: Reverse Linked List with Among Us pointers
slay reverse_list(head ඞListNode) ඞListNode {
    sus prev ඞListNode = nah
    sus current ඞListNode = head
    
    periodt current != nah {
        sus next_temp ඞListNode = current.next
        current.next = prev
        prev = current
        current = next_temp
    }
    
    damn prev
}
```

### Additional Examples

- Basic syntax and language features
- Concurrency patterns with goroutines and channels
- Memory management demonstrations
- Error handling patterns

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by Go's simplicity and concurrency model
- Gen Z linguistic creativity and internet culture
- The Among Us community for the iconic `ඞ` character

---

*"Stay based, avoid cringe, and always handle your yikes responsibly."* 💀✨
