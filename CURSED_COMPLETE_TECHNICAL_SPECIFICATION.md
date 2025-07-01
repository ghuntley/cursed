# CURSED Language Complete Technical Specification

## Executive Summary

CURSED is an esoteric programming language that combines Go-like semantics with Gen Z slang keywords. It follows a self-hosting compiler approach, targets LLVM IR for code generation, and provides a comprehensive standard library. This document provides the complete technical specification extracted from all specification files.

## 1. Language Overview & Philosophy

### 1.1 Core Design Principles
- **Syntactic Familiarity with a Twist**: Go-like grammar with Gen Z slang keywords
- **Self-Hosting**: Multi-stage bootstrapping process starting with Rust
- **Pragmatic Absurdity**: Esoteric by design but fully functional
- **Compilation Efficiency**: Produces efficient code with readable source
- **Static Typing**: Type inference with garbage collection
- **Concurrency Support**: Built-in primitives for concurrent programming

### 1.2 Language Characteristics
- **Encoding**: UTF-8 source code, no BOM
- **Modules**: Package-based code organization
- **Expression-Based**: Most constructs yield values
- **Target**: LLVM IR for performance and portability

## 2. Lexical Structure

### 2.1 Character Sets and Encoding
- Unicode text encoded in UTF-8
- Whitespace characters separate tokens but are otherwise ignored

### 2.2 Comments
```cursed
fr fr This is a line comment

no cap
This is a block comment
that spans multiple lines
on god
```

### 2.3 Identifiers
- Sequence of letters, digits, and underscores
- First character cannot be a digit
- Case-sensitive

### 2.4 Keywords Mapping
| Go Keyword | CURSED Keyword | Function |
|------------|----------------|----------|
| package    | vibe          | Package declaration |
| import     | yeet          | Import declaration |
| func       | slay          | Function declaration |
| return     | yolo          | Return statement |
| var        | sus           | Variable declaration |
| const      | facts         | Constant declaration |
| if         | lowkey        | Conditional statement |
| else       | highkey       | Else clause |
| for        | bestie        | For loop |
| while      | periodt       | While loop |
| switch     | vibe_check    | Switch statement |
| case       | mood          | Case clause |
| default    | basic         | Default clause |
| break      | ghosted       | Break statement |
| continue   | simp          | Continue statement |
| type       | be_like       | Type declaration |
| struct     | squad         | Struct type |
| interface  | collab        | Interface type |
| map        | tea           | Map type |
| chan       | dm            | Channel type |
| go         | stan          | Goroutine creation |
| range      | flex          | Range clause |
| defer      | later         | Defer statement |
| true       | based         | Boolean true |
| false      | sus           | Boolean false |
| nil        | cap           | Nil/null value |

### 2.5 Operators and Punctuation
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Bitwise: `&`, `|`, `^`, `<<`, `>>`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Assignment: `=`, `:=`, `+=`, `-=`, etc.
- Logical: `&&`, `||`, `!`
- Channel: `<-`
- Delimiters: `()`, `[]`, `{}`, `,`, `;`, `.`, `:`

### 2.6 Literals
#### 2.6.1 Integer Literals
- Decimal: `123`
- Octal: `0o173`
- Hexadecimal: `0xAB`
- Binary: `0b1010`

#### 2.6.2 Floating-Point Literals
- `3.14159`
- `1.0e10`
- `.5`
- `1.`

#### 2.6.3 String Literals
- Quoted: `"hello world"`
- Raw: `` `multiline string` ``
- Escape sequences: `\n`, `\t`, `\\`, `\'`, `\"`

#### 2.6.4 Boolean and Nil Literals
- `based` (true)
- `sus` (false)
- `cap` (nil)

## 3. Type System

### 3.1 Basic Types
| Type Category | CURSED Types | Description | Size |
|---------------|--------------|-------------|------|
| Boolean       | `lit`        | True/false values | 1 bit |
| Numeric       | `smol`       | 8-bit signed integer | 8 bits |
|               | `mid`        | 16-bit signed integer | 16 bits |
|               | `normie`     | 32-bit signed integer | 32 bits |
|               | `thicc`      | 64-bit signed integer | 64 bits |
|               | `snack`      | 32-bit floating point | 32 bits |
|               | `meal`       | 64-bit floating point | 64 bits |
|               | `byte`       | Alias for uint8 | 8 bits |
|               | `rune`       | Alias for int32 (Unicode) | 32 bits |
| String        | `tea`        | UTF-8 encoded text | Variable |
| Character     | `sip`        | Single Unicode character | 32 bits |
| Complex       | `extra`      | Complex number | 128 bits |

### 3.2 Composite Types
| Type Category | Syntax | Description |
|---------------|--------|-------------|
| Array       | `[n]T`   | Fixed-size array of n elements |
| Slice       | `[]T`    | Dynamic array |
| Map         | `tea[K]V` | Hash map from K to V |
| Struct      | `squad`  | Collection of fields |
| Interface   | `collab` | Set of method signatures |
| Pointer     | `@T`     | Pointer to type T |
| Function    | `slay`   | Function type |
| Channel     | `dm<T>`  | Channel of type T |

### 3.3 Zero Values
| Type | Zero Value |
|------|------------|
| `lit` | `sus` (false) |
| Numeric types | `0` |
| `tea` | `""` (empty string) |
| `sip` | `\0` (null character) |
| Pointers | `cap` (nil) |
| Slices | `cap` (nil) |
| Maps | `cap` (nil) |
| Channels | `cap` (nil) |
| Structs | Each field has zero value |
| Arrays | Each element has zero value |

### 3.4 Type Inference
Variables can be declared with type inference using `:=`:
```cursed
x := 10        fr fr x is normie
y := "hello"   fr fr y is tea
z := based     fr fr z is lit
c := 'a'       fr fr c is sip
```

### 3.5 Generic Types
Generic types are supported using square brackets:
```cursed
be_like Stack[T] squad {
    items []T
    size normie
}

slay push[T](s @Stack[T], item T) {
    s.items = append(s.items, item)
    s.size++
}
```

### 3.6 Character Type Operations
```cursed
sus c sip = 'a'
sus is_upper lit = c.is_uppercase()
sus is_lower lit = c.is_lowercase()
sus is_digit lit = c.is_digit()
sus is_alpha lit = c.is_alpha()
sus is_alnum lit = c.is_alnum()
sus as_upper sip = c.to_uppercase()
sus as_lower sip = c.to_lowercase()
sus as_int normie = normie(c)
```

## 4. Grammar Rules

### 4.1 Program Structure
```
SourceFile       = PackageClause ";" { ImportDecl ";" } { TopLevelDecl ";" } .
PackageClause    = "vibe" PackageName .
PackageName      = identifier .
```

### 4.2 Imports
```
ImportDecl       = "yeet" ( ImportSpec | "(" { ImportSpec ";" } ")" ) .
ImportSpec       = [ identifier | "." ] ImportPath .
ImportPath       = string_lit .
```

Example:
```cursed
vibe main

yeet (
    "fmt"
    tea "strings"
)
```

### 4.3 Declarations
```
Declaration      = ConstDecl | TypeDecl | VarDecl | FuncDecl .
TopLevelDecl     = Declaration | MethodDecl .
```

#### 4.3.1 Constants
```
ConstDecl        = "facts" ( ConstSpec | "(" { ConstSpec ";" } ")" ) .
ConstSpec        = IdentifierList [ Type ] "=" ExpressionList .
```

#### 4.3.2 Variables
```
VarDecl          = "sus" ( VarSpec | "(" { VarSpec ";" } ")" ) .
VarSpec          = IdentifierList ( Type [ "=" ExpressionList ] | "=" ExpressionList ) .
```

#### 4.3.3 Types
```
TypeDecl         = "be_like" ( TypeSpec | "(" { TypeSpec ";" } ")" ) .
TypeSpec         = TypeName Type .
```

#### 4.3.4 Functions
```
FuncDecl         = "slay" FunctionName [ TypeParameters ] Signature [ FunctionBody ] .
FunctionName     = identifier .
FunctionBody     = Block .
```

### 4.4 Statements
```
Statement        = Declaration | SimpleStmt |
                   IfStmt | SwitchStmt | ForStmt | 
                   Block | ReturnStmt | BreakStmt | ContinueStmt .
```

#### 4.4.1 If Statements
```
IfStmt           = "lowkey" [ SimpleStmt ";" ] Expression Block [ "highkey" ( IfStmt | Block ) ] .
```

Both parenthesized and non-parenthesized conditions are valid:
```cursed
lowkey x > 0 {
    yolo x
}

lowkey (x > 0) {
    yolo x
}
```

#### 4.4.2 Switch Statements
```
SwitchStmt       = ExprSwitchStmt | TypeSwitchStmt .
ExprSwitchStmt   = "vibe_check" [ SimpleStmt ";" ] [ Expression ] "{" { ExprCaseClause } "}" .
ExprCaseClause   = ExprSwitchCase ":" StatementList .
ExprSwitchCase   = "mood" ExpressionList | "basic" .
```

#### 4.4.3 For Statements
```
ForStmt          = "bestie" [ Condition | ForClause | RangeClause ] Block .
Condition        = Expression .
ForClause        = [ InitStmt ] ";" [ Condition ] ";" [ PostStmt ] .
RangeClause      = [ ExpressionList "=" | IdentifierList ":=" ] "flex" Expression .
```

#### 4.4.4 While Statements
```
WhileStmt        = "periodt" Expression Block .
```

#### 4.4.5 Return Statements
```
ReturnStmt       = "yolo" [ ExpressionList ] .
```

#### 4.4.6 Break and Continue
```
BreakStmt        = "ghosted" [ Label ] .
ContinueStmt     = "simp" [ Label ] .
```

### 4.5 Expressions
```
Expression       = UnaryExpr | Expression binary_op Expression .
UnaryExpr        = PrimaryExpr | unary_op UnaryExpr .
PrimaryExpr      = Operand | Conversion | PrimaryExpr Selector | PrimaryExpr Index | PrimaryExpr Slice | PrimaryExpr TypeAssertion | PrimaryExpr Arguments .
```

### 4.6 Concurrency
```
GoStmt           = "stan" Expression .
SendStmt         = Channel "<-" Expression .
Channel          = Expression .
```

### 4.7 Defer Statements
```
DeferStmt        = "later" Expression .
```

## 5. Compilation Pipeline

### 5.1 Bootstrap Process
The compiler follows a 4-stage bootstrapping process:

1. **Stage 0**: Bootstrap Environment Setup (Rust-based)
2. **Stage 1**: Minimal Bootstrap Compiler
3. **Stage 2**: Full Compiler in CURSED
4. **Stage 3**: Self-Compiled Full Compiler

### 5.2 Compilation Stages
1. **Lexical Analysis**: Source code → Token stream
2. **Preprocessing**: Enhanced tokens with generic syntax support
3. **Parsing**: Token stream → Abstract Syntax Tree (AST)
4. **Semantic Analysis**: Type checking and validation
5. **Intermediate Representation**: AST → IR
6. **Optimization**: Various optimization passes
7. **Code Generation**: IR → LLVM IR
8. **Linking**: Link with runtime libraries

### 5.3 LLVM IR Target
- Compiles to LLVM Intermediate Representation
- Leverages LLVM's optimization passes
- Enables cross-platform native code generation
- Supports interoperability with other LLVM-based languages

### 5.4 Name Mangling
Single module compilation strategy with name mangling:
- Format: `_<package_name>_<symbol_name>`
- Example: `_myutils_DoThing`
- Private symbols also mangled to avoid conflicts

## 6. Preprocessor

### 6.1 Generic Syntax Support
The preprocessor handles:
1. Generic type declarations: `be_like Box[T] squad { ... }`
2. Generic function declarations: `slay foo[T](x normie) T { ... }`
3. Generic function calls: `foo[normie](42)`
4. Nested generic types: `be_like Pair[K, V[T]] squad { ... }`

### 6.2 Token Processing
- Buffers tokens for pattern analysis
- Adds metadata for generic constructs
- Provides detailed error messages for malformed syntax

## 7. Tree-Sitter Grammar

### 7.1 Components
- **Rules**: Syntactic patterns
- **Extras**: Whitespace and comments
- **Precedence**: Operator precedence
- **Conflict Resolution**: Parsing conflicts

### 7.2 Query Files
- `highlights.scm`: Syntax highlighting
- `folds.scm`: Code folding
- `indents.scm`: Auto-indentation
- `locals.scm`: Variable scoping
- `injections.scm`: Language injection
- `textobjects.scm`: Navigation and selection

## 8. Standard Library Specification

### 8.1 Core Packages

#### 8.1.1 `vibez` (fmt)
Formatted I/O functions:
- `spill(args ...collab{})`: Print with newline
- `spillf(format tea, args ...collab{})`: Formatted print
- `spillstr(format tea, args ...collab{})`: Format to string
- `scan(args ...collab{})`: Scan input
- `scanln(args ...collab{})`: Scan line

#### 8.1.2 `core` (builtin)
Fundamental types and functions:
- Type conversions: `lit()`, `normie()`, `tea()`, etc.
- Collection operations: `append()`, `cap()`, `len()`, `make()`
- Memory operations: `new()`
- Error handling: `shook()`, `unbothered()`

#### 8.1.3 `dropz` (io)
Basic I/O primitives with interfaces:
- `Reader`: Reading bytes
- `Writer`: Writing bytes
- `Closer`: Closing resources

#### 8.1.4 `vibe_life` (os)
Operating system functionality:
- Command-line arguments: `Args`
- Environment variables: `Getenv()`, `Setenv()`
- Process control: `Exit()`
- File operations: `Create()`, `Open()`

#### 8.1.5 `stringz` (strings)
String manipulation:
- `Contains()`, `Count()`, `HasPrefix()`, `HasSuffix()`
- `Join()`, `Split()`, `ToLower()`, `ToUpper()`
- `Trim()`

#### 8.1.6 `mathz` (math)
Mathematical functions:
- `Abs()`, `Ceil()`, `Floor()`, `Max()`, `Min()`
- `Pow()`, `Sqrt()`
- Constants: `Pi`, `E`

#### 8.1.7 `timez` (time)
Time-related functionality:
- `Time`, `Duration` types
- `Now()`, `Sleep()`, `Since()`, `Until()`

#### 8.1.8 `concurrenz` (sync)
Synchronization primitives:
- `Mutex`, `RWMutex`
- `WaitGroup`, `Cond`, `Once`, `Pool`

### 8.2 Advanced Packages

#### 8.2.1 `oglogging` (log)
Logging facility:
- `Logger` type with configurable output
- Standard functions: `spill()`, `spillf()`
- Fatal logging: `fatal()`, `fatalf()`
- Panic logging: `shook()`, `shookf()`
- Flags: `Ldate`, `Ltime`, `Lmicroseconds`, etc.

#### 8.2.2 `cryptz` (crypto)
Cryptographic primitives:
- Hash functions: SHA256, SHA512, Blake3, HMAC
- Symmetric encryption: AES, GCM
- Asymmetric cryptography: RSA, ECDSA, Ed25519
- Password hashing and random generation

#### 8.2.3 `vibe_net` (net)
Network I/O:
- IP addressing: `IPVibe`, `IPNetVibe`, `IPMaskVibe`
- Connections: `ConnVibe`, `TCPConnVibe`, `UDPConnVibe`
- Listeners: `ListenerVibe`, `TCPListenerVibe`
- DNS resolution and network interfaces
- Enhanced features: Connection pools, circuit breakers, rate limiters

#### 8.2.4 `slay_io` (bufio)
Buffered I/O operations:
- `SlayReader`: Buffered reading with peek, readline
- `SlayWriter`: Buffered writing with flush
- `SlayScanner`: Token scanning with split functions
- `SlayReadWriter`: Combined reader/writer

#### 8.2.5 `syslog_era` (syslog)
Syslog client implementation:
- RFC 5424 compliant
- Facility and severity levels
- TCP/UDP/TLS transport
- Integration with logging packages

### 8.3 Additional Specialized Packages
The standard library includes 50+ specialized packages covering:
- ASN.1 encoding (`asn1_mood`)
- Atomic operations (`atomic_drip`)
- Big integers (`big_mood`)
- Binary data (`binary_drip`)
- CSV processing (`csv_mood`)
- HTML processing (`htmlrizzler`)
- HTTP client/server (`glowup_http`)
- JSON encoding (`json_tea`)
- Regular expressions (`regex_vibez`)
- SQL database (`sql_slay`)
- Template processing (`rizz_template`)
- Testing framework (`test_vibes`)
- And many more...

## 9. Memory Management

### 9.1 Garbage Collection
- Automatic memory management
- GC integration with LLVM IR generation
- Stack maps for live pointer tracking
- Support for concurrent collection

### 9.2 Memory Layout
- Local variables: Stack allocation using LLVM `alloca`
- Global variables: LLVM global variables
- Heap allocation: GC-managed
- Strings and slices: Runtime-managed structures

## 10. Concurrency Model

### 10.1 Goroutines (`stan`)
- Lightweight threads managed by runtime
- LLVM calls to runtime scheduler
- Cooperative scheduling

### 10.2 Channels (`dm`)
- Type-safe message passing
- Buffered and unbuffered channels
- Select statements for multiplexing
- Runtime-implemented with synchronization primitives

## 11. Error Handling

### 11.1 Error Patterns
Following Go's error handling model:
```cursed
result, err := doSomething()
lowkey err != cap {
    handleError(err)
}
```

### 11.2 Panic and Recovery
- `shook()`: Trigger panic
- `unbothered()`: Recover from panic
- Similar to Go's panic/recover mechanism

## 12. Module System

### 12.1 Package Declaration
```cursed
vibe PackageName
```

### 12.2 Import System
```cursed
yeet "path/to/package"        // Standard import
yeet alias "path/to/package"  // Aliased import
```

### 12.3 Visibility Rules
- Uppercase first letter: Exported (public)
- Lowercase first letter: Private to package
- Qualified access: `packageName.ExportedSymbol`

## 13. Implementation Requirements

### 13.1 Runtime Requirements
- Garbage collector implementation
- Goroutine scheduler
- Channel operations
- Standard library runtime support

### 13.2 Compiler Requirements
- LLVM IR generation
- Type checking and inference
- Generic type instantiation
- Module system implementation
- Error reporting and diagnostics

### 13.3 Performance Requirements
- Efficient code generation
- Minimal runtime overhead
- Optimized standard library implementations
- Fast compilation times

## 14. Testing Strategy

### 14.1 Test Types
- Unit tests for individual components
- Integration tests for component interactions
- End-to-end compilation tests
- Regression tests
- Compliance tests against specification
- Self-hosting tests

### 14.2 Test Infrastructure
Built-in testing framework (`test_vibes`) with:
- Test discovery and execution
- Benchmarking capabilities
- Coverage analysis
- Property-based testing support

## 15. Development Phases

### 15.1 Phase Timeline
| Phase | Duration | Key Deliverables |
|-------|----------|------------------|
| Stage 0 | 2-4 weeks | Project setup, lexer, parser, AST |
| Stage 1 | 2-3 months | Minimal compiler, basic runtime |
| Stage 2 | 3-6 months | Full compiler in CURSED, stdlib |
| Stage 3 | 1-2 months | Self-compiled, complete toolchain |

### 15.2 Implementation Priorities
1. Core language features (types, functions, control flow)
2. Basic I/O and string operations
3. Concurrency primitives
4. Standard library expansion
5. Advanced features and optimizations

## 16. Specification Gaps and Ambiguities

### 16.1 Identified Gaps
1. **Generic Type Constraints**: Specification doesn't detail type constraints for generics
2. **Method Resolution**: Interface method resolution order not fully specified
3. **Package Versioning**: No versioning scheme for packages
4. **Build System**: Package management and build system not fully defined
5. **Reflection**: Runtime reflection capabilities not specified
6. **Unsafe Operations**: No unsafe pointer operations defined
7. **CGO Equivalent**: C interoperability mechanism not specified

### 16.2 Ambiguities Requiring Clarification
1. **Generic Type Inference**: How much type inference is supported for generics
2. **Channel Semantics**: Exact buffering and closing semantics
3. **GC Tuning**: Garbage collection configuration options
4. **Cross-Compilation**: Cross-platform compilation support
5. **Debugging**: Debug information format and tooling
6. **Profiling**: Performance profiling infrastructure

## 17. Implementation Mapping

### 17.1 Specification to Implementation Matrix
| Specification Area | Implementation Module | Priority | Complexity |
|-------------------|----------------------|----------|------------|
| Lexical Analysis | `lexer.rs` | High | Medium |
| Parser | `parser.rs` | High | High |
| Type System | `type_checker.rs` | High | High |
| AST | `ast.rs` | High | Medium |
| Code Generation | `codegen.rs` | High | High |
| Runtime | `runtime/` | High | High |
| Standard Library | `stdlib/` | Medium | High |
| Preprocessor | `preprocessor.rs` | Medium | Medium |
| Error Handling | `error.rs` | High | Medium |
| Module System | `module.rs` | High | Medium |

### 17.2 Critical Implementation Dependencies
1. **LLVM Integration**: Core dependency for code generation
2. **Garbage Collector**: Custom GC implementation required
3. **Runtime Library**: Goroutine scheduler and channel implementation
4. **Standard Library**: Comprehensive stdlib implementation
5. **Type System**: Advanced type inference and generic support

## 18. Success Criteria

### 18.1 Functional Requirements
- [ ] Complete lexical analysis of all tokens
- [ ] Full grammar parsing with error recovery
- [ ] Type checking with inference and generics
- [ ] LLVM IR generation for all language constructs
- [ ] Garbage collection with minimal pause times
- [ ] Concurrent execution with goroutines and channels
- [ ] Complete standard library implementation
- [ ] Self-hosting capability
- [ ] Cross-platform compilation

### 18.2 Performance Requirements
- Compilation speed: < 10x slower than Go
- Runtime performance: < 2x slower than Go
- Memory usage: Comparable to Go
- Startup time: < 100ms for typical programs

### 18.3 Quality Requirements
- 100% specification compliance
- Comprehensive test coverage (>90%)
- Documentation for all public APIs
- Error messages with actionable guidance
- IDE integration support

## Conclusion

This technical specification provides a comprehensive foundation for implementing the CURSED programming language. The specification covers all aspects from lexical structure to runtime behavior, providing exact requirements for a complete implementation. The identified gaps and ambiguities should be addressed during the implementation process to ensure a robust and fully-featured language.

The multi-stage bootstrapping approach, combined with the detailed standard library specification and LLVM IR target, provides a clear path to creating a self-hosting, performant programming language that successfully combines Go's pragmatism with Gen Z's linguistic creativity.
