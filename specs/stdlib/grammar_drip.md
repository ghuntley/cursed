# GrammarDrip (go/* packages)

## Overview
GrammarDrip provides tools for syntactic analysis, manipulation, and generation of Go code. It's inspired by Go's go/ast, go/parser, go/token, go/printer packages with enhanced usability, more powerful transformations, and code generation capabilities.

## Core Types

### Token Handling

```go
// Position describes a source position including file, line, and column
type Position struct {
    Filename string
    Offset   int
    Line     int
    Column   int
}

// Methods
func (pos Position) IsValid() bool
func (pos Position) String() string

// FileSet represents a set of source files
type FileSet struct {}

// Constructor
func NewFileSet() *FileSet

// Methods
func (fs *FileSet) AddFile(filename string, base, size int) *File
func (fs *FileSet) Position(pos Pos) Position
func (fs *FileSet) File(pos Pos) *File
func (fs *FileSet) PositionFor(pos Pos, adjusted bool) Position
func (fs *FileSet) Files() []*File

// File represents a source file
type File struct {}

// Methods
func (f *File) Name() string
func (f *File) Base() int
func (f *File) Size() int
func (f *File) LineCount() int
func (f *File) LineStart(line int) Pos
func (f *File) Pos(offset int) Pos
func (f *File) Offset(pos Pos) int
func (f *File) Line(pos Pos) int
func (f *File) Position(pos Pos) Position
func (f *File) Read() ([]byte, error)

// Pos represents a position in a source file
type Pos int
const NoPos Pos = 0

// Token represents a lexical token
type Token int

// Token types
const (
    ILLEGAL Token = iota
    EOF
    COMMENT
    IDENT
    INT
    FLOAT
    IMAG
    CHAR
    STRING
    
    // Operators and punctuation
    ADD    // +
    SUB    // -
    MUL    // *
    QUO    // /
    REM    // %
    
    // ... (many more tokens)
    
    BREAK
    CASE
    CHAN
    CONST
    CONTINUE
    
    // ... (all Go keywords)
)

// Methods
func (tok Token) String() string
func (tok Token) IsLiteral() bool
func (tok Token) IsOperator() bool
func (tok Token) IsKeyword() bool
func LookupToken(ident string) Token
```

### AST (Abstract Syntax Tree)

```go
// Node is implemented by all AST nodes
type Node interface {
    Pos() Pos
    End() Pos
    Accept(visitor Visitor) (Node, bool)
    Clone() Node
}

// Visitor pattern for traversing AST
type Visitor interface {
    Visit(node Node) (w Visitor, cont bool)
}

// Expr represents an expression node
type Expr interface {
    Node
    exprNode()
}

// Stmt represents a statement node
type Stmt interface {
    Node
    stmtNode()
}

// Decl represents a declaration node
type Decl interface {
    Node
    declNode()
}

// Type represents a type node
type Type interface {
    Expr
    typeNode()
}
```

### Common AST Nodes

```go
// Specific expression nodes
type Ident struct {
    NamePos Pos
    Name    string
    Obj     *Object
}

type BasicLit struct {
    ValuePos Pos
    Kind     Token
    Value    string
}

type CompositeLit struct {
    Type   Expr
    Lbrace Pos
    Elts   []Expr
    Rbrace Pos
}

type FuncLit struct {
    Type *FuncType
    Body *BlockStmt
}

type BinaryExpr struct {
    X     Expr
    OpPos Pos
    Op    Token
    Y     Expr
}

// Statement nodes
type ExprStmt struct {
    X Expr
}

type BlockStmt struct {
    Lbrace Pos
    List   []Stmt
    Rbrace Pos
}

type IfStmt struct {
    If   Pos
    Init Stmt
    Cond Expr
    Body *BlockStmt
    Else Stmt
}

type ForStmt struct {
    For  Pos
    Init Stmt
    Cond Expr
    Post Stmt
    Body *BlockStmt
}

// Declaration nodes
type GenDecl struct {
    TokPos Pos
    Tok    Token
    Lparen Pos
    Specs  []Spec
    Rparen Pos
}

type FuncDecl struct {
    Recv *FieldList
    Name *Ident
    Type *FuncType
    Body *BlockStmt
}
```

### Package Representation

```go
// Package represents a Go package
type Package struct {
    Name    string
    Scope   *Scope
    Imports map[string]*Package
    Files   map[string]*File
}

// Methods
func (p *Package) Path() string
func (p *Package) FindFile(name string) *File
func (p *Package) ListFiles() []*File
func (p *Package) FindImport(path string) *Package
```

## Core Functions

### Parsing

```go
// ParseFile parses a single Go source file
func ParseFile(fset *FileSet, filename string, src interface{}, mode ParseMode) (*File, error)

// ParseDir parses all Go files in the directory
func ParseDir(fset *FileSet, path string, filter func(fs.FileInfo) bool, mode ParseMode) (map[string]*Package, error)

// ParseExpr parses a Go expression
func ParseExpr(x string) (Expr, error)

// Options
type ParseMode int
const (
    ParseComments ParseMode = 1 << iota
    ParseDeclarations
    ParseImports
    ParseStdLib
    SkipFuncBodies
    Trace
    AllErrors
)
```

### AST Manipulation

```go
// Walk traverses an AST in depth-first order
func Walk(v Visitor, node Node) Node

// Inspect traverses an AST in depth-first order and calls f for each node
func Inspect(node Node, f func(Node) bool) Node

// NodeFilter returns a new visitor that only visits nodes that pass the filter
func NodeFilter(v Visitor, filter func(Node) bool) Visitor

// CloneNode creates a deep copy of an AST node
func CloneNode(node Node) Node

// ReplaceNode replaces a node within its parent
func ReplaceNode(parent Node, old, new Node) bool

// DeleteNode deletes a node from its parent
func DeleteNode(parent Node, node Node) bool

// InsertBefore inserts a node before another node
func InsertBefore(parent Node, mark, node Node) bool

// InsertAfter inserts a node after another node
func InsertAfter(parent Node, mark, node Node) bool

// FindNode searches for a node that satisfies the predicate
func FindNode(root Node, predicate func(Node) bool) Node

// FindAllNodes finds all nodes that satisfy the predicate
func FindAllNodes(root Node, predicate func(Node) bool) []Node
```

### Printing and Code Generation

```go
// Fprint writes an AST node to w
func Fprint(w io.Writer, fset *FileSet, node any) error

// Print AST node to a string
func Print(fset *FileSet, node any) string

// Format formats node and writes the result to w
func Format(w io.Writer, fset *FileSet, node any, config *PrintConfig) error

// PrintConfig is a configuration for the Print function
type PrintConfig struct {
    Mode           PrintMode
    Tabwidth       int
    Indent         int
    UseSpaces      bool
    RemoveComments bool
    SourcePos      bool
}
```

## Enhanced Features

### AST Builder

```go
type ASTBuilder struct {}

// Constructor
func NewASTBuilder(fileSet *FileSet) *ASTBuilder

// Methods for building expressions
func (b *ASTBuilder) Ident(name string) *Ident
func (b *ASTBuilder) BasicLit(kind Token, value string) *BasicLit
func (b *ASTBuilder) CompositeLit(typ Expr, elts ...Expr) *CompositeLit
func (b *ASTBuilder) BinaryExpr(x Expr, op Token, y Expr) *BinaryExpr
func (b *ASTBuilder) UnaryExpr(op Token, x Expr) *UnaryExpr
func (b *ASTBuilder) CallExpr(fun Expr, args ...Expr) *CallExpr
func (b *ASTBuilder) SelectorExpr(x Expr, sel *Ident) *SelectorExpr
func (b *ASTBuilder) IndexExpr(x, index Expr) *IndexExpr

// Methods for building statements
func (b *ASTBuilder) ExprStmt(x Expr) *ExprStmt
func (b *ASTBuilder) BlockStmt(stmts ...Stmt) *BlockStmt
func (b *ASTBuilder) AssignStmt(lhs, rhs []Expr, tok Token) *AssignStmt
func (b *ASTBuilder) IfStmt(init Stmt, cond Expr, body *BlockStmt, elsePart Stmt) *IfStmt
func (b *ASTBuilder) ForStmt(init Stmt, cond Expr, post Stmt, body *BlockStmt) *ForStmt
func (b *ASTBuilder) ReturnStmt(results ...Expr) *ReturnStmt

// Methods for building declarations
func (b *ASTBuilder) FuncDecl(name string, recvName, recvType string, params, results *FieldList, body *BlockStmt) *FuncDecl
func (b *ASTBuilder) VarDecl(names []*Ident, typ Expr, values []Expr) *GenDecl
func (b *ASTBuilder) ConstDecl(names []*Ident, typ Expr, values []Expr) *GenDecl
func (b *ASTBuilder) TypeDecl(name string, typ Expr) *GenDecl

// Method for creating a file
func (b *ASTBuilder) File(name string, imports []*ImportSpec, decls []Decl) *File
func (b *ASTBuilder) Package(name string, files map[string]*File) *Package
```

### Code Analysis

```go
type Analyzer struct {
    FileSet *FileSet
    Package *Package
}

// Constructor
func NewAnalyzer(fset *FileSet, pkg *Package) *Analyzer

// Analysis methods
func (a *Analyzer) FindTypeByName(name string) Type
func (a *Analyzer) FindFunctionByName(name string) *FuncDecl
func (a *Analyzer) FindImports() map[string]string
func (a *Analyzer) FindFunctionCalls(funcName string) []*CallExpr
func (a *Analyzer) FindMethodCalls(typeName, methodName string) []*CallExpr
func (a *Analyzer) FindStructFields(structName string) []*Field
func (a *Analyzer) FindInterfaceMethods(interfaceName string) []*Field
func (a *Analyzer) FindReferences(ident string) []*Ident
func (a *Analyzer) FindUnusedVariables() []*Ident
func (a *Analyzer) FindUnusedImports() []*ImportSpec
func (a *Analyzer) FindDuplicateDeclarations() map[string][]Decl
func (a *Analyzer) CheckNullPointerDereferences() []*SelectorExpr
func (a *Analyzer) FindCyclomaticComplexity(funcName string) int
```

### Code Transformation

```go
type Transformer struct {
    FileSet *FileSet
    File    *File
    Builder *ASTBuilder
}

// Constructor
func NewTransformer(fset *FileSet, file *File) *Transformer

// Transformation methods
func (t *Transformer) RenameIdentifier(oldName, newName string) (changes int)
func (t *Transformer) ReplaceExpr(old, new Expr) (changes int)
func (t *Transformer) ReplaceStmt(old, new Stmt) (changes int)
func (t *Transformer) AddImport(path, name string) *ImportSpec
func (t *Transformer) RemoveImport(path string) bool
func (t *Transformer) WrapBlockWithTryCatch(block *BlockStmt, handler Stmt) *TryStmt
func (t *Transformer) ExtractFunction(block *BlockStmt, name string, params []*Field) *FuncDecl
func (t *Transformer) InlineFunction(call *CallExpr) (Stmt, error)
func (t *Transformer) AddMethod(typeName string, method *FuncDecl) error
func (t *Transformer) ConvertToInterface(structName string) (*GenDecl, error)
func (t *Transformer) ConvertToStruct(interfaceName string) (*GenDecl, error)
func (t *Transformer) AddField(structName string, field *Field) error
func (t *Transformer) RemoveField(structName, fieldName string) bool
func (t *Transformer) MakePrivate(name string) (changes int)
func (t *Transformer) MakePublic(name string) (changes int)
```

### Code Generation

```go
type CodeGenerator struct {
    Builder *ASTBuilder
    FileSet *FileSet
}

// Constructor
func NewCodeGenerator() *CodeGenerator

// Generation methods
func (g *CodeGenerator) GenerateStruct(name string, fields []*Field) *GenDecl
func (g *CodeGenerator) GenerateInterface(name string, methods []*Field) *GenDecl
func (g *CodeGenerator) GenerateCRUD(structName, tableName string) []*FuncDecl
func (g *CodeGenerator) GenerateConstructor(structName string) *FuncDecl
func (g *CodeGenerator) GenerateGetters(structName string, fieldNames ...string) []*FuncDecl
func (g *CodeGenerator) GenerateSetters(structName string, fieldNames ...string) []*FuncDecl
func (g *CodeGenerator) GenerateStringMethod(structName string) *FuncDecl
func (g *CodeGenerator) GenerateEquals(structName string) *FuncDecl
func (g *CodeGenerator) GenerateMarshalJSON(structName string) *FuncDecl
func (g *CodeGenerator) GenerateUnmarshalJSON(structName string) *FuncDecl
func (g *CodeGenerator) GenerateTestCases(funcDecl *FuncDecl) *File
func (g *CodeGenerator) GenerateMocks(interfaceName string) *File
```

### Documentation Generation

```go
type DocGenerator struct {}

// Constructor
func NewDocGenerator(fset *FileSet, pkg *Package) *DocGenerator

// Generation methods
func (g *DocGenerator) GeneratePackageDoc() string
func (g *DocGenerator) GenerateFunctionDoc(funcName string) string
func (g *DocGenerator) GenerateTypeDoc(typeName string) string
func (g *DocGenerator) GenerateMethodDoc(typeName, methodName string) string
func (g *DocGenerator) GenerateHTML() string
func (g *DocGenerator) GenerateMarkdown() string
func (g *DocGenerator) GenerateDiagram() string
```

## GenZ Style Extensions

```go
// Grammar patterns that follow modern language patterns
func (b *ASTBuilder) ChainedExpr(exprs ...Expr) Expr
func (b *ASTBuilder) TernaryExpr(cond, ifTrue, ifFalse Expr) Expr
func (b *ASTBuilder) SpreadExpr(expr Expr) Expr
func (b *ASTBuilder) ArrowFunc(params *FieldList, body Expr) *FuncLit
func (b *ASTBuilder) AsyncFunc(name string, params, results *FieldList, body *BlockStmt) *FuncDecl

// GenZ styled comments
func (b *ASTBuilder) DripComment(text string) *CommentGroup
func (b *ASTBuilder) NoCapComment(text string) *CommentGroup
func AddVibeCheck(file *File) *File
func MakeFunctionSlay(funcDecl *FuncDecl) *FuncDecl
func MakeSussyCode(node Node) Node
```

## Usage Example

```go
// Parse a Go source file
fset := grammar_drip.NewFileSet()
src := `package main

import "fmt"

func main() {
	fmt.Println("Hello, World!")
}`

file, err := grammar_drip.ParseFile(fset, "example.go", src, grammar_drip.ParseComments)
if err != nil {
    vibez.spill("Error parsing file:", err)
    return
}

// Analyze the AST
grammar_drip.Inspect(file, func(n grammar_drip.Node) bool {
    if call, ok := n.(*grammar_drip.CallExpr); ok {
        if sel, ok := call.Fun.(*grammar_drip.SelectorExpr); ok {
            if ident, ok := sel.X.(*grammar_drip.Ident); ok && ident.Name == "fmt" {
                vibez.spill("Found fmt call:", sel.Sel.Name)
            }
        }
    }
    return true
})

// Modify AST - replace "Hello, World!" with "Hello, Drip!"
grammar_drip.Inspect(file, func(n grammar_drip.Node) bool {
    if lit, ok := n.(*grammar_drip.BasicLit); ok && lit.Kind == grammar_drip.STRING {
        if lit.Value == `"Hello, World!"` {
            lit.Value = `"Hello, Drip!"`
        }
    }
    return true
})

// Generate modified code
var buf bytes.Buffer
err = grammar_drip.Format(&buf, fset, file, &grammar_drip.PrintConfig{
    Tabwidth: 4,
})
if err != nil {
    vibez.spill("Error formatting code:", err)
    return
}

modified := buf.String()
vibez.spill(modified)

// Use the AST builder to create new code
builder := grammar_drip.NewASTBuilder(fset)

// Build a simple function declaration
funcDecl := builder.FuncDecl(
    "add",                 // Function name
    "", "",               // No receiver (not a method)
    builder.FieldList(    // Parameters
        builder.Field([]*grammar_drip.Ident{builder.Ident("a")}, builder.Ident("int"), nil),
        builder.Field([]*grammar_drip.Ident{builder.Ident("b")}, builder.Ident("int"), nil),
    ),
    builder.FieldList(    // Return values
        builder.Field(nil, builder.Ident("int"), nil),
    ),
    builder.BlockStmt(    // Function body
        builder.ReturnStmt(
            builder.BinaryExpr(
                builder.Ident("a"),
                grammar_drip.ADD,
                builder.Ident("b"),
            ),
        ),
    ),
)

// Generate code for the function
var funcBuf bytes.Buffer
grammar_drip.Format(&funcBuf, fset, funcDecl, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(funcBuf.String())

// Create a full file AST
newFile := builder.File(
    "mathutils",   // Package name
    []*grammar_drip.ImportSpec{  // Imports
        builder.ImportSpec("math", ""),
    },
    []grammar_drip.Decl{  // Declarations
        funcDecl,
    },
)

// Generate code for the file
var fileBuf bytes.Buffer
grammar_drip.Format(&fileBuf, fset, newFile, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(fileBuf.String())

// Analyze existing code
pkg, _ := grammar_drip.ParseDir(fset, "./", nil, grammar_drip.ParseComments)
analyzer := grammar_drip.NewAnalyzer(fset, pkg["main"])

// Find unused variables
unusedVars := analyzer.FindUnusedVariables()
for _, v := range unusedVars {
    vibez.spill("Unused variable:", v.Name, "at", fset.Position(v.Pos()))
}

// Find function calls
calls := analyzer.FindFunctionCalls("Println")
for _, call := range calls {
    vibez.spill("Println call at:", fset.Position(call.Pos()))
}

// Transform code
transformer := grammar_drip.NewTransformer(fset, file)

// Rename a variable
changes := transformer.RenameIdentifier("Println", "Printf")
vibez.spill("Made", changes, "rename changes")

// Add an import
transformer.AddImport("time", "")

// Generate the transformed file
var transformedBuf bytes.Buffer
grammar_drip.Format(&transformedBuf, fset, file, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(transformedBuf.String())

// Generate code using advanced features
codeGen := grammar_drip.NewCodeGenerator()

// Generate a struct with getters and setters
userStruct := codeGen.GenerateStruct("User", [
    builder.Field(
        []*grammar_drip.Ident{builder.Ident("ID")}, 
        builder.Ident("int"),
        nil,
    ),
    builder.Field(
        []*grammar_drip.Ident{builder.Ident("Name")}, 
        builder.Ident("string"),
        nil,
    ),
    builder.Field(
        []*grammar_drip.Ident{builder.Ident("Email")}, 
        builder.Ident("string"),
        nil,
    ),
])

getters := codeGen.GenerateGetters("User", "ID", "Name", "Email")
setters := codeGen.GenerateSetters("User", "Name", "Email")

// Generate a new file with these declarations
userFile := builder.File(
    "models",
    nil,
    append([]grammar_drip.Decl{userStruct}, append(getters, setters...)...),
)

// Generate the code
var userFileBuf bytes.Buffer
grammar_drip.Format(&userFileBuf, fset, userFile, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(userFileBuf.String())

// Use GenZ style extensions
vibeFile := grammar_drip.AddVibeCheck(file)
slayFunc := grammar_drip.MakeFunctionSlay(funcDecl)

var vibeBuf bytes.Buffer
grammar_drip.Format(&vibeBuf, fset, vibeFile, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(vibeBuf.String())
```

## Implementation Guidelines
1. Maintain compatibility with Go's AST package structure
2. Optimize for performance with large code bases
3. Ensure robust error handling for invalid code
4. Support all Go language features including generics
5. Implement memory-efficient node reuse where possible
6. Provide clear documentation for all functions and types
7. Include common patterns for AST manipulation
8. Support both programmatic and interactive usage models