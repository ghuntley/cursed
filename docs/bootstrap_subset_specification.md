# CURSED Bootstrap Compiler Subset Specification

## Overview

The CURSED Bootstrap Compiler Subset is a minimal set of language features designed to enable self-hosting compilation. It includes only the essential features needed to implement a basic compiler, allowing the CURSED language to bootstrap itself from this reduced feature set.

## Design Goals

1. **Minimal Complexity**: Include only features absolutely necessary for compilation
2. **Self-Hosting**: Sufficient to implement a CURSED compiler in CURSED itself
3. **Maintainability**: Simple enough to implement and maintain reliably
4. **Extensibility**: Foundation for adding more advanced features later

## Included Language Features

### Core Language Elements

#### Tokens and Lexical Elements

**Essential Keywords:**
- `vibe` - Package declaration (equivalent to Go's `package`)
- `yeet` - Import statement (equivalent to Go's `import`)
- `slay` - Function definition (equivalent to Go's `func`)
- `sus` - Variable declaration (equivalent to Go's `var`)
- `facts` - Constant declaration (equivalent to Go's `const`)
- `lowkey` - If statement (equivalent to Go's `if`)
- `highkey` - Else clause (equivalent to Go's `else`)
- `bestie` - For loop (equivalent to Go's `for`)
- `yolo` - Return statement (equivalent to Go's `return`)
- `based` - Boolean true literal (equivalent to Go's `true`)
- `cap` - Nil/null literal (equivalent to Go's `nil`)
- `ghosted` - Break statement (equivalent to Go's `break`)
- `simp` - Continue statement (equivalent to Go's `continue`)

**Basic Types:**
- `normie` - 32-bit signed integer (equivalent to Go's `int32`)
- `thicc` - 64-bit signed integer (equivalent to Go's `int64`)
- `lit` - Boolean type (equivalent to Go's `bool`)
- `snack` - 32-bit floating point (equivalent to Go's `float32`)
- `meal` - 64-bit floating point (equivalent to Go's `float64`)
- String literals (double-quoted strings)

**Operators:**
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Assignment: `=`, `:=`, `+=`, `-=`, `*=`, `/=`

**Delimiters:**
- Parentheses: `(`, `)`
- Braces: `{`, `}`
- Brackets: `[`, `]` (for array literals and indexing)
- Punctuation: `,`, `;`, `:`, `.`

#### Expression Types

**Literal Expressions:**
- Integer literals: `42`, `-10`
- Float literals: `3.14`, `-2.5`
- String literals: `"hello world"`
- Boolean literals: `based` (true), `false` (implied by absence of true)

**Basic Expressions:**
- Identifiers: Variable and function names
- Infix expressions: Binary operations like `a + b`, `x == y`
- Prefix expressions: Unary operations like `-x`, `!flag`
- Parenthesized expressions: `(expression)`
- Function calls: `functionName(arg1, arg2)`
- Array literals: `[]normie{1, 2, 3}`
- Index expressions: `array[index]`
- Dot expressions: `module.function` (for standard library access)

#### Statement Types

**Declaration Statements:**
- Package declaration: `vibe packageName`
- Import statements: `yeet "module/path"`
- Variable declarations: `sus variableName = value`
- Constant declarations: `facts constantName = value`
- Function definitions: `slay functionName(params) { body }`

**Control Flow Statements:**
- If statements: `lowkey condition { body }`
- If-else statements: `lowkey condition { body } highkey { elseBody }`
- For loops: `bestie condition { body }`
- Return statements: `yolo value`
- Break statements: `ghosted`
- Continue statements: `simp`

**Other Statements:**
- Expression statements: Any expression followed by semicolon
- Block statements: `{ statements }`
- Assignment statements: `variable = value`

### Standard Library Access

The bootstrap subset allows access to essential standard library modules:

**vibez** - I/O Operations:
- `vibez.spill(message)` - Print to stdout (equivalent to `fmt.Println`)

**mathz** - Mathematical Operations:
- `mathz.add(a, b)` - Addition
- `mathz.sub(a, b)` - Subtraction  
- `mathz.mul(a, b)` - Multiplication
- `mathz.div(a, b)` - Division

**stringz** - String Operations:
- `stringz.len(s)` - String length
- `stringz.concat(a, b)` - String concatenation
- `stringz.substr(s, start, end)` - Substring

**timez** - Time Operations:
- `timez.now()` - Current timestamp
- `timez.sleep(duration)` - Sleep/pause execution

## Excluded Features

The following features are **not** included in the bootstrap subset:

### Advanced Type System
- Struct definitions (`squad`)
- Interface definitions (`collab`)
- Type aliases
- Basic generic types and type parameters (simplified constraints only)
- Method definitions
- Pointer types and operations

**Note**: Full generic constraint system and advanced features like higher-kinded types
are available in the complete compiler but simplified for bootstrap phase. See the
[Generics Guide](generics_guide.md) for complete generic programming capabilities.

### Concurrency
- Goroutines (`stan` keyword)
- Channels (`dm` type)
- Channel operations (`<-` operator)
- Select statements (`choose`)

### Advanced Control Flow
- Switch statements (`vibe_check`)
- Type switches
- Range-based for loops (`flex` keyword)
- Defer statements (`later`)

### Advanced Features
- Error handling mechanisms
- Complex expressions (type assertions, etc.)
- Slice operations beyond basic literals
- Map operations
- Complex standard library modules

## Example Bootstrap Program

Here's a complete example of a valid bootstrap program:

```cursed
vibe main

yeet "vibez"
yeet "mathz"

facts PI = 3.14159

slay calculateArea(radius snack) snack {
    yolo mathz.mul(PI, mathz.mul(radius, radius))
}

slay main() {
    sus radius snack = 5.0
    sus area = calculateArea(radius)
    
    vibez.spill("Circle area:")
    vibez.spill(area)
    
    bestie sus i = 0; i < 10; i = i + 1 {
        lowkey i % 2 == 0 {
            vibez.spill("Even:")
            vibez.spill(i)
        } highkey {
            vibez.spill("Odd:")
            vibez.spill(i)
        }
    }
}
```

## Validation and Compliance

### Validation Process

The bootstrap subset includes a validator that can check programs for compliance:

1. **Token Validation**: Ensures only allowed tokens are used
2. **Expression Validation**: Verifies expression types are permitted
3. **Statement Validation**: Confirms statement types are allowed
4. **Feature Validation**: Checks for use of excluded advanced features

### Configuration Options

**Strict Mode**: Enforces exact compliance with subset restrictions
**Lenient Mode**: Allows some flexibility for development
**Warning Mode**: Generates warnings for suboptimal but allowed patterns

### CLI Usage

```bash
# Validate a program against bootstrap subset
cursed bootstrap validate compiler.csd

# Compile using bootstrap mode
cursed bootstrap compile --strict compiler.csd

# Show subset information
cursed bootstrap info

# Show configuration options
cursed bootstrap config
```

## Implementation Requirements

### For Self-Hosting

A bootstrap CURSED compiler must be able to:

1. **Parse** all bootstrap subset syntax
2. **Type-check** basic types and operations
3. **Generate code** for essential operations
4. **Handle** standard library integration
5. **Support** separate compilation of modules
6. **Produce** executable output

### Minimal Compiler Components

1. **Lexer**: Tokenize bootstrap subset syntax
2. **Parser**: Build AST from tokens
3. **Type Checker**: Validate basic types
4. **Code Generator**: Produce target code
5. **Linker Integration**: Combine compiled modules
6. **Standard Library**: Essential runtime functions

## Rationale for Design Decisions

### Why These Features?

**Functions**: Essential for code organization and compiler implementation
**Variables**: Required for data storage and manipulation
**Control Flow**: Necessary for conditional logic and iteration
**Basic Types**: Sufficient for compiler data structures
**Arrays**: Needed for collections (token streams, AST nodes)
**Standard Library**: Required for I/O and basic operations

### Why These Exclusions?

**Structs**: Can be simulated with multiple variables
**Channels**: Not necessary for sequential compiler operation
**Generics**: Add complexity without essential benefit for bootstrap
**Pointers**: Memory management can be handled at higher level
**Advanced Control Flow**: Basic if/for covers essential needs

## Future Evolution

### Phase 1: Bootstrap (This Specification)
Minimal subset for self-hosting

### Phase 2: Enhanced Bootstrap
Add structs and interfaces for better organization

### Phase 3: Full Language
Add all advanced features (generics, concurrency, etc.)

### Migration Path
Programs written for earlier phases should continue to work in later phases, ensuring backward compatibility.

## Validation Examples

### Valid Bootstrap Program
```cursed
vibe main

slay add(a normie, b normie) normie {
    yolo a + b
}

slay main() {
    sus result = add(5, 3)
    vibez.spill(result)
}
```

### Invalid Bootstrap Program
```cursed
vibe main

squad Calculator {  // ❌ Structs not allowed
    name tea
}

slay main() {
    sus calc = Calculator{name: "calc"}  // ❌ Struct literals not allowed
}
```

## Conclusion

The CURSED Bootstrap Compiler Subset provides a carefully chosen minimal set of language features that enable self-hosting while maintaining simplicity. By restricting the feature set to only essential elements, it ensures that a bootstrap compiler can be implemented with manageable complexity, paving the way for full language implementation in subsequent phases.
