# CURSED Programming Language

CURSED is an esoteric programming language that combines Go-like grammar with Generation Z slang for keywords and tokens. It is implemented as a self-compiling compiler written in Rust.

## About CURSED

CURSED follows Go's practical design philosophy but replaces traditional programming keywords with contemporary Gen Z slang, creating a unique and entertaining programming experience while maintaining practical usability.

Example CURSED code:

```
vibe main

yeet "vibez"

slay main() {
    vibez.spill("Hello, World!")  fr fr This is a comment
    
    sus name tea = "bestie"
    vibez.spillf("Hey %s, what's good?", name)

    lowkey 1 < 2 {
        vibez.spill("This is based!")
    } highkey {
        vibez.spill("This is sus!")
    }
}
```

## Self-Compiling Compiler

CURSED is implemented following the bootstrapping compiler approach:

1. **Stage 0**: Bootstrap environment setup using Rust
2. **Stage 1**: Minimal bootstrap compiler in Rust
3. **Stage 2**: Full compiler written in CURSED
4. **Stage 3**: Self-compiled full compiler

## Project Structure

- `/src`: Compiler source code
- `/specs`: Language specifications and documentation
- `/examples`: Example CURSED programs
- `/tests`: Test suite for the compiler

## Getting Started

### Prerequisites

- Rust toolchain (1.54.0 or later)
- Cargo

### Building

```
make build
```

### Running Tests

```
make test
```

### Running the Compiler

```
make run ARGS="path/to/your/file.csd"
```

## Language Documentation

The complete language specifications are available in the `/specs` directory:

- [Overview](specs/overview.md)
- [Lexical Structure](specs/lexical.md)
- [Types](specs/types.md)
- [Grammar](specs/grammar.md)
- [Compiler Stages](specs/compiler_stages.md)
- [Standard Library](specs/stdlib.md)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 