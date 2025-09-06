# CURSED Language Specifications

CURSED is an esoteric programming language that follows Go-like grammar but uses Gen Z slang for keywords and tokens. The language is designed to be self-hosting via a bootstrapping compiler written in Rust.

## Specification Documents

- [Overview](overview.md): Core design principles and language philosophy
- [Lexical Structure](lexical.md): Tokens, keywords, and syntax structure
- [Types](types.md): Type system and primitive types
- [Grammar](grammar.md): Language grammar rules and syntax
- [Memory Management](memory_management.md): Memory management and garbage collection
- [Error Handling](error_handling.md): Error handling patterns and best practices
- [Concurrency](concurrency.md): Goroutines, channels, and concurrent programming
- [Compiler Stages](compiler_stages.md): Details of the bootstrapping process
- [Standard Library](stdlib.md): Core library functionality

## Recent Changes

### Pointer Syntax Evolution (2024)

CURSED has evolved its pointer syntax from the traditional `@` symbol to the **Among Us character `ඞ`** (U+0D9E), making it the world's first programming language to use this iconic internet culture symbol for pointer operations.

**Before (Legacy @ Syntax):**
```cursed
vibe main
yeet "vibez"

slay legacy_pointers() {
    sus x normie = 42
    sus ptr @normie = @x     // Old address-of syntax
    sus value normie = *ptr  // Dereference unchanged
    
    vibez.spill("Value: " + stringz.from_int(value))
}
```

**After (Modern ඞ Syntax):**
```cursed
vibe main
yeet "vibez"

slay modern_pointers() {
    sus x normie = 42
    sus ptr ඞnormie = ඞx     // New Among Us address-of syntax
    sus value normie = *ptr  // Dereference unchanged
    
    vibez.spill("Value: " + stringz.from_int(value))
}
```

**Cultural Impact:**

This groundbreaking syntax change establishes CURSED as the first programming language to incorporate the Among Us character into its core syntax. The `ඞ` character has become synonymous with internet culture and Gen Z communication, making CURSED programs instantly recognizable to modern developers.

**Technical Benefits:**

- **Visual distinctiveness**: The `ඞ` character is unmistakable and reduces confusion with other operators
- **Unicode compliance**: Properly implements Unicode U+0D9E character support
- **Parser efficiency**: The unique character avoids conflicts with existing operator precedence
- **Cultural relevance**: Aligns with CURSED's Gen Z-focused design philosophy

**Migration Guide:**

Legacy `@` syntax is deprecated and will be removed in future versions. All new code should use `ඞ` for address-of operations. See [POINTER_SYNTAX_CHANGE.md](../POINTER_SYNTAX_CHANGE.md) for comprehensive migration documentation.

This change reflects CURSED's commitment to incorporating contemporary Gen Z culture into programming language design while maintaining technical excellence. See the individual specification documents for updated syntax examples.

## Project Goals

1. Create a self-hosting compiler following the bootstrapping process
2. Maintain Go-like semantics while using Gen Z slang as syntax
3. Create a functional programming language that's both esoteric and practical
4. Learn compiler design and implementation techniques 