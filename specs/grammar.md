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

Example:

```
vibe main

yeet (
    "fmt"
    tea "strings"
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
    yolo x + y
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
    yolo x
} highkey lowkey x < 0 {
    yolo -x
} highkey {
    yolo 0
}
```

Parentheses around the condition expression are optional:

```
lowkey (x > 0) {
    yolo x
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
        print("Start of week")
    mood "Friday":
        print("End of week")
    basic:
        print("Mid-week")
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
    print(i)
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
WhileStmt        = "periodt" Expression Block .
```

Example:

```
periodt x > 0 {
    x--
}
```

### Return Statements

```
ReturnStmt       = "yolo" [ ExpressionList ] .
```

Example:

```
yolo x + y
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
Expression       = UnaryExpr | Expression binary_op Expression .
UnaryExpr        = PrimaryExpr | unary_op UnaryExpr .
PrimaryExpr      = Operand | Conversion | PrimaryExpr Selector | PrimaryExpr Index | PrimaryExpr Slice | PrimaryExpr TypeAssertion | PrimaryExpr Arguments .
```

### Primary Expressions

```
Operand          = Literal | OperandName | "(" Expression ")" .
Literal          = BasicLit | CompositeLit | FunctionLit .
BasicLit         = int_lit | float_lit | string_lit .
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

Example:

```
fmt.Println("Hello, world!")
add(1, 2)
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

ch <- value                           // Send operation (blocking)
value := <-ch                         // Receive operation (blocking)
value, ok := <-ch                     // Receive with closed check
close(ch)                             // Close channel
```

### Select Statements

```
SelectStmt       = "ready" "{" { SelectCase } "}" .
SelectCase       = "mood" ( SendStmt | ReceiveStmt ) ":" StatementList |
                   "basic" ":" StatementList .
ReceiveStmt      = [ ExpressionList "=" | IdentifierList ":=" ] ReceiveExpr .
```

Example:

```
ready {
    mood ch1 <- value:
        // Send succeeded
    mood result := <-ch2:
        // Receive succeeded
    mood <-timeout:
        // Timeout occurred
    basic:
        // No operations ready
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
    done <- based
}
<-done                                // Wait for completion
```

## Defer Statements

```
DeferStmt        = "later" Expression .
```

Example:

```
later file.Close()
```

## Error Handling

CURSED follows Go's error handling pattern:

```
slay doSomething() (tea, Error) {
    lowkey err != cringe {
        yolo "", err
    }
    yolo "success", cringe
}

result, err := doSomething()
lowkey err != cringe {
    handleError(err)
}
``` 