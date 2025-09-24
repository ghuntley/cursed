# CURSED Grammar

This document specifies the grammar rules and syntax of the CURSED programming language. The grammar follows Go-like structure but uses Gen Z slang keywords.

## Lexical Elements

### Identifiers

```
identifier       = unicode_letter { unicode_letter | unicode_digit } .
unicode_letter   = /* any Unicode code point categorized as "Letter" or emoji */ .
unicode_digit    = /* any Unicode code point categorized as "Number, decimal digit" */ .
```

**CURSED ENHANCEMENT:** Identifiers support full Unicode including emoji characters for maximum expressiveness! 🔥

Examples of valid identifiers:
- Traditional: `myVariable`, `_private`, `MAX_VALUE`
- Unicode: `café`, `用户名`, `тест`
- **Emoji**: `🔥`, `💻data`, `🚀speed`, `😂count`

## Program Structure

A CURSED program consists of one or more source files organized into packages. Each source file belongs to a single package and consists of package declarations, imports, and top-level declarations.

```
SourceFile       = PackageClause ";" { ImportDecl ";" } { TopLevelDecl ";" } .
PackageClause    = "vibe" PackageName .
PackageName      = identifier .
```

## Imports

**CANONICAL SYNTAX:** Imports declare dependencies on other packages using the `yeet` keyword.

```
ImportDecl       = "yeet" ( ImportSpec | "(" { ImportSpec ";" } ")" | ImportList ) .
ImportSpec       = [ identifier | "." ] ImportPath .
ImportPath       = string_lit .
ImportList       = ImportPath { "," ImportPath } .
```

**PARSING RULES:**

1. **Single import**: `yeet "module_name"`
2. **Multiple imports (comma-separated)**: `yeet "module1", "module2", "module3"`
3. **Grouped imports (parenthesized)**: `yeet ( "module1"; "module2"; "module3" )`
4. **Aliased import**: `yeet "module_name" as alias_name`
5. **Specific imports**: `yeet "module" { symbol1, symbol2 }`

Examples:

```
vibe main

// Single import
yeet "vibez"
yeet "mathz"

// Comma-separated imports  
yeet "vibez", "mathz", "stringz"

// Grouped imports (semicolon-separated)
yeet (
    "vibez"
    "mathz"
    "stringz"  
)

// Aliased import
yeet "very_long_module_name" as short

// Specific symbol imports
yeet "mathz" { sin, cos, tan }
```

**IMPLEMENTATION REQUIREMENT:** All parsers MUST support all four import forms with consistent precedence and error handling.

## Declarations and Scope

Declarations introduce new identifiers and bind them to constants, types, variables, functions, or packages.

```
Declaration      = ConstDecl | TypeDecl | VarDecl | FuncDecl .
TopLevelDecl     = Declaration | MethodDecl .
```

### Constants

```
ConstDecl        = "facts" ( ConstSpec | "(" { ConstSpec ";" } ")" ) .
ConstSpec        = IdentifierList [ Type ] "=" ExpressionList .
```

Example:

```
facts (
    PI = 3.14159
    E = 2.71828
)
```

### Variables

```
VarDecl          = "sus" ( VarSpec | "(" { VarSpec ";" } ")" ) .
VarSpec          = IdentifierList ( Type [ "=" ExpressionList ] | "=" ExpressionList ) .
```

Example:

```
sus name tea = "World"
sus age, height = 25, 180.5

fr fr Pointer variable declarations
sus ptr ඞnormie        // Uninitialized pointer
sus ptr2 ඞnormie = ඞage  // Pointer to age variable
```

### Types

```
TypeDecl         = "be_like" ( TypeSpec | "(" { TypeSpec ";" } ")" ) .
TypeSpec         = TypeName Type .
Type             = BasicType | PointerType | ArrayType | SliceType | MapType | 
                   StructType | InterfaceType | FunctionType | ChannelType .
BasicType        = "normie" | "tea" | "lit" | "smol" | "mid" | "thicc" | 
                   "snack" | "meal" | "byte" | "rune" | "sip" | "extra" | 
                   "cap" | "yikes" | identifier .
PointerType      = "ඞ" Type .
ArrayType        = Type "[" Expression "]" .
SliceType        = Type "[" "value" "]" | Type "[" "]" .
MapType          = "map" "[" Type "]" Type .
StructType       = "squad" "{" { FieldDecl ";" } "}" .
InterfaceType    = "collab" "{" { MethodSpec ";" } "}" .
FunctionType     = "slay" "(" [ ParameterList ] ")" [ Type ] .
ChannelType      = "dm" "<" Type ">" [ "[" Expression "]" ] .
```

Examples:

```
be_like Person squad {
    name tea
    age normie
    address ඞAddress    // Pointer to Address
}

be_like Node squad {
    data normie
    next ඞNode         // Pointer to next node
}
```

### Functions

```
FuncDecl         = "slay" FunctionName [ TypeParameters ] Signature [ FunctionBody ] .
FunctionName     = identifier .
FunctionBody     = Block .
```

Example:

```
slay add(x, y normie) normie {
    damn x + y
}
```

## Statements

Statements control execution.

```
Statement        = Declaration | SimpleStmt |
                   IfStmt | SwitchStmt | ForStmt | 
                   Block | ReturnStmt | BreakStmt | ContinueStmt .
```

### Simple Statements

```
SimpleStmt       = EmptyStmt | ExpressionStmt | Assignment | ShortVarDecl |
                   IncDecStmt .
```

### If Statements

**CANONICAL KEYWORDS:**
```
IfStmt           = "ready" [ SimpleStmt ";" ] Expression Block [ "otherwise" ( IfStmt | Block ) ] .
```

Example:

```
ready x > 0 {
    damn x
} otherwise ready x < 0 {
    damn -x
} otherwise {
    damn 0
}
```

**DEPRECATED (remove in v2.0):**
```
// DEPRECATED: lowkey/highkey - use ready/otherwise  
ready x > 0 { ... } otherwise { ... }
```

Parentheses around the condition expression are optional:

```
ready (x > 0) {
    damn x
}
```

### Switch Statements

```
SwitchStmt       = ExprSwitchStmt | TypeSwitchStmt .
ExprSwitchStmt   = "vibe_check" [ SimpleStmt ";" ] [ Expression ] "{" { ExprCaseClause } "}" .
ExprCaseClause   = ExprSwitchCase ":" StatementList .
ExprSwitchCase   = "mood" ExpressionList | "basic" .
```

Example:

```
vibe_check day {
    mood "Monday", "Tuesday":
        vibez.spill("Start of week")
    mood "Friday":
        vibez.spill("End of week")
    basic:
        vibez.spill("Mid-week")
}
```

### For Statements

**CANONICAL KEYWORDS:**
```
ForStmt          = "bestie" [ Condition | ForClause | RangeClause ] Block .
Condition        = Expression .
ForClause        = [ InitStmt ] ";" [ Condition ] ";" [ PostStmt ] .
RangeClause      = [ ExpressionList "=" | IdentifierList ":=" ] "flex" Expression .
```

Examples:

```
bestie i := 0; i < 10; i++ {
    vibez.spill(i)
}

bestie x < 100 {
    x = x * 2
}

bestie {
    doSomething()
    ready done() {
        ghosted
    }
}

bestie _, val := flex items {
    process(val)
}
```

### While Statements

**CANONICAL KEYWORD:**
```
WhileStmt        = "periodt" Expression Block .
```

Example:

```
periodt x > 0 {
    x--
}
```

**DEPRECATED (remove in v2.0):**
```
// DEPRECATED: flex for while loops - use periodt
flex x < 100 { ... }
```

### Return Statements

```
ReturnStmt       = "damn" [ ExpressionList ] .
```

The `damn` keyword is used to return values from functions.

**CANONICAL**: `damn` is the canonical return keyword. `yolo` is deprecated and MAY be removed in future versions.
**PARSER REQUIREMENT**: New parsers SHOULD prefer `damn` and MAY emit warnings for `yolo`.

Examples:

```
damn x + y
damn "Hello World"
damn  # return with no value
```

### Break and Continue Statements

```
BreakStmt        = "ghosted" [ Label ] .
ContinueStmt     = "simp" [ Label ] .
```

Example:

```
bestie {
    ready someCondition() {
        ghosted
    }
    ready otherCondition() {
        simp
    }
}
```

## Expressions

Expressions compute values.

```
Expression       = LogicalOrExpr .
LogicalOrExpr    = LogicalAndExpr { "||" LogicalAndExpr } .
LogicalAndExpr   = EqualityExpr { "&&" EqualityExpr } .
EqualityExpr     = RelationalExpr { ( "==" | "!=" ) RelationalExpr } .
RelationalExpr   = AdditiveExpr { ( "<" | "<=" | ">" | ">=" ) AdditiveExpr } .
AdditiveExpr     = MultiplicativeExpr { ( "+" | "-" ) MultiplicativeExpr } .
MultiplicativeExpr = UnaryExpr { ( "*" | "/" | "%" ) UnaryExpr } .
UnaryExpr        = PrimaryExpr | unary_op UnaryExpr .
unary_op         = "!" | "-" | "+" | "*" | "ඞ" .
PrimaryExpr      = Operand | Conversion | PrimaryExpr Selector | PrimaryExpr Index | PrimaryExpr Slice | PrimaryExpr TypeAssertion | PrimaryExpr Arguments .
```

### Operator Precedence

CURSED follows standard operator precedence (highest to lowest):

1. **Primary expressions**: `()`, `[]`, `.`, function calls
2. **Unary operators**: `!`, `-`, `+`, `*` (dereference), `ඞ` (address-of)
3. **Multiplicative**: `*`, `/`, `%`
4. **Additive**: `+`, `-`
5. **Relational**: `<`, `<=`, `>`, `>=`
6. **Equality**: `==`, `!=`
7. **Logical AND**: `&&`
8. **Logical OR**: `||`
9. **Assignment**: `=`, `:=`, `+=`, `-=`, etc.

**CRITICAL**: Parsers MUST implement precedence climbing to ensure correct evaluation order.
Example: `2 + 3 * 4` must parse as `2 + (3 * 4)`, not `(2 + 3) * 4`.

### Pointer Operations

The CURSED language uses the Among Us character `ඞ` (U+0D9E) for pointer operations:

- **Address-of operator** (`ඞ`): Gets the memory address of a variable
- **Dereference operator** (`*`): Accesses the value stored at a pointer address

**Examples:**

```cursed
vibe main
yeet "vibez"

slay demonstrate_pointers() {
    // Basic pointer operations
    sus x normie = 42
    sus ptr ඞnormie = ඞx        // Get address of x
    sus value normie = *ptr     // Dereference pointer
    
    // Pointer arithmetic and operations
    sus data normie[5] = {1, 2, 3, 4, 5}
    sus arr_ptr ඞnormie = ඞdata[0]  // Pointer to first element
    
    // Multiple levels of indirection
    sus ptr_to_ptr ඞඞnormie = ඞptr
    sus original_value normie = **ptr_to_ptr
    
    vibez.spill("Original value: " + stringz.from_int(original_value))
}
```

**Syntax Evolution:**

CURSED previously used the `@` symbol for address-of operations but evolved to use the Among Us character `ඞ` to better reflect modern internet culture:

- **Legacy (deprecated):** `sus ptr @normie = @x`
- **Current:** `sus ptr ඞnormie = ඞx`

**Parser Requirements:**

- The `ඞ` character (U+0D9E) MUST be recognized as a valid unary operator
- Pointer type declarations use `ඞType` syntax
- Multiple levels of pointer indirection are supported: `ඞඞnormie`, `ඞඞඞnormie`, etc.

### Primary Expressions

```
Operand          = Literal | OperandName | "(" Expression ")" .
Literal          = BasicLit | CompositeLit | FunctionLit .
BasicLit         = int_lit | float_lit | string_lit | bool_lit | nil_lit | char_lit .
OperandName      = identifier | QualifiedIdentifier .
QualifiedIdentifier = identifier "." identifier .
```

### Method Calls and Selectors

```
Selector         = "." identifier .
Index            = "[" Expression "]" .
Slice            = "[" [ Expression ] ":" [ Expression ] [ ":" Expression ] "]" .
TypeAssertion    = ".(" Type ")" .
```

Example:

```
x.field
arr[i]
slice[i:j]
value.(tea)
```

### Function Calls

```
Arguments        = "(" [ ( ExpressionList [ "," ] ) | Type [ "," ExpressionList [ "," ] ] ] ")" .
ExpressionList   = Expression { "," Expression } .
```

Examples:

```
vibez.spill("Hello, world!")
add(1, 2)
math.pow(2, 3)

fr fr Pointer operations
sus x normie = 42
sus ptr ඞnormie = ඞx    // Get address of x
sus value normie = *ptr  // Dereference pointer
```

## Goroutines and Channels

CURSED provides Go-style goroutines and channels for concurrent programming.

### Goroutine Syntax

```
GoStmt           = "stan" Expression .
```

The `stan` keyword spawns a new goroutine:

```
stan doSomething()                    // Spawn goroutine
stan worker(ch, data)                 // Spawn with parameters
stan {                                // Anonymous goroutine
    processBatch(data)
}
```

### Channel Syntax

```
ChannelType      = "dm" "<" Type ">" [ "[" Expression "]" ] .
SendStmt         = "dm_send" "(" Channel "," Expression ")" .
ReceiveExpr      = "dm_recv" "(" Channel ")" .
CloseStmt        = "dm_close" "(" Channel ")" .
```

### Channel Operations

```
sus ch dm<normie>                     // Unbuffered channel declaration
sus buffered dm<tea>[10]              // Buffered channel declaration

dm_send(ch, value)                    // Send operation (blocking) - CANONICAL
value := dm_recv(ch)                  // Receive operation (blocking) - CANONICAL  
value, ok := dm_recv(ch)              // Receive with closed check - CANONICAL

dm_close(ch)                          // Close channel - CANONICAL

// Legacy Go-style syntax (DEPRECATED - remove in future versions):
// dm_send(ch, value, value := dm_recv(ch), close(ch)
// PARSER REQUIREMENT: New parsers SHOULD NOT implement legacy syntax.
```

### Select Statements

```
SelectStmt       = "ready" "{" { SelectCase } "}" .
SelectCase       = "mood" ( SendStmt | ReceiveStmt ) ":" StatementList |
                   "basic" ":" StatementList .
SendStmt         = "dm_send" "(" Channel "," Expression ")" .
ReceiveStmt      = [ ExpressionList "=" | IdentifierList ":=" ] "dm_recv" "(" Channel ")" .
```

The `ready` keyword is used for select statements that allow non-blocking operation on multiple channels.

Examples:

```
slay select_basic_example() lit {
    # Basic select with default case
    ready {
        basic:
            vibez.spill("Default case executed")
    }
    damn based
}

slay select_with_channels() lit {
    sus ch1 dm<normie>
    sus ch2 dm<tea>
    sus timeout dm<lit>
    
    ready {
        mood dm_send(ch1, 42):
            vibez.spill("Send to ch1 succeeded")
        mood result := dm_recv(ch2):
            vibez.spill("Received from ch2: " + result)
        mood dm_recv(timeout):
            vibez.spill("Timeout occurred")
        basic:
            vibez.spill("No operations ready")
    }
    damn based
}

slay select_multiple_operations() lit {
    sus input_ch dm<tea>
    sus output_ch dm<normie>
    sus done_ch dm<lit>
    
    ready {
        mood msg := dm_recv(input_ch):
            vibez.spill("Processing: " + msg)
        mood dm_send(output_ch, 100):
            vibez.spill("Sent result")
        mood dm_recv(done_ch):
            vibez.spill("Done signal received")
        basic:
            vibez.spill("No channel operations ready")
    }
    damn based
}
```

### Goroutine Lifecycle

```
// Goroutine spawning
stan worker(data)

// Goroutine synchronization
sus done dm<lit>
stan {
    doWork()
    dm_send(done, based)
}
dm_recv(done)                         // Wait for completion
```

## Defer Statements

```
DeferStmt        = "later" Expression .
```

The `later` keyword is used to defer execution of an expression until the function returns.

Examples:

```
slay cleanup_example() lit {
    vibez.spill("Function start")
    
    # Basic defer statement
    later vibez.spill("This executes at function end")
    
    # Resource cleanup pattern
    sus file := open("data.txt")
    later file.close()
    
    # Multiple defers execute in LIFO order
    later vibez.spill("Third defer")
    later vibez.spill("Second defer")
    later vibez.spill("First defer")
    
    vibez.spill("Function body")
    damn based
}

slay defer_with_early_return() lit {
    later vibez.spill("Cleanup happens even with early return")
    
    ready some_condition() {
        damn based  # defer still executes
    }
    
    vibez.spill("Normal path")
    damn based
}
```

## Error Handling

CURSED follows Go's error handling pattern:

```
slay doSomething() (tea, Error) {
    ready err != nah {
        damn "", err
    }
    damn "success", nah
}

result, err := doSomething()
ready err != nah {
    handleError(err)
}
``` 