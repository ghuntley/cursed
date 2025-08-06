# CURSED Grammar

This document specifies the grammar rules and syntax of the CURSED programming language. The grammar follows Go-like structure but uses Gen Z slang keywords.

## Program Structure

A CURSED program consists of one or more source files organized into packages. Each source file belongs to a single package and consists of package declarations, imports, and top-level declarations.

```
SourceFile       = PackageClause ";" { ImportDecl ";" } { TopLevelDecl ";" } .
PackageClause    = "vibe" PackageName .
PackageName      = identifier .
```

## Imports

Imports declare dependencies on other packages.

```
ImportDecl       = "yeet" ( ImportSpec | "(" { ImportSpec ";" } ")" ) .
ImportSpec       = [ identifier | "." ] ImportPath .
ImportPath       = string_lit .
```

Examples:

```
vibe main

yeet "vibez"
yeet "math"

yeet (
    "vibez"
    "math"
    "string"
)
```

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
```

### Types

```
TypeDecl         = "be_like" ( TypeSpec | "(" { TypeSpec ";" } ")" ) .
TypeSpec         = TypeName Type .
```

Example:

```
be_like Person squad {
    name tea
    age normie
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

```
IfStmt           = "lowkey" [ SimpleStmt ";" ] Expression Block [ "highkey" ( IfStmt | Block ) ] .
```

Example:

```
lowkey x > 0 {
    damn x
} highkey lowkey x < 0 {
    damn -x
} highkey {
    damn 0
}
```

Parentheses around the condition expression are optional:

```
lowkey (x > 0) {
    damn x
}
```

Both forms are valid in CURSED.

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
    lowkey done() {
        ghosted
    }
}

bestie _, val := flex items {
    process(val)
}
```

### While Statements

```
WhileStmt        = ( "periodt" | "flex" ) Expression Block .
```

Examples:

```
periodt x > 0 {
    x--
}

flex x < 100 {
    x = x * 2
}
```

**COMPATIBILITY**: Both `periodt` and `flex` are accepted for while statements.

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
    lowkey someCondition() {
        ghosted
    }
    lowkey otherCondition() {
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
PrimaryExpr      = Operand | Conversion | PrimaryExpr Selector | PrimaryExpr Index | PrimaryExpr Slice | PrimaryExpr TypeAssertion | PrimaryExpr Arguments .
```

### Operator Precedence

CURSED follows standard operator precedence (highest to lowest):

1. **Primary expressions**: `()`, `[]`, `.`, function calls
2. **Unary operators**: `!`, `-`, `+`, `*` (dereference), `&` (address-of)
3. **Multiplicative**: `*`, `/`, `%`
4. **Additive**: `+`, `-`
5. **Relational**: `<`, `<=`, `>`, `>=`
6. **Equality**: `==`, `!=`
7. **Logical AND**: `&&`
8. **Logical OR**: `||`
9. **Assignment**: `=`, `:=`, `+=`, `-=`, etc.

**CRITICAL**: Parsers MUST implement precedence climbing to ensure correct evaluation order.
Example: `2 + 3 * 4` must parse as `2 + (3 * 4)`, not `(2 + 3) * 4`.

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
SendStmt         = Channel "<-" Expression .
ReceiveExpr      = "<-" Channel .
CloseStmt        = "close" "(" Channel ")" .
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
// ch <- value, value := <-ch, close(ch)
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
    
    lowkey some_condition() {
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
    lowkey err != nah {
        damn "", err
    }
    damn "success", nah
}

result, err := doSomething()
lowkey err != nah {
    handleError(err)
}
``` 