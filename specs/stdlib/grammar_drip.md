# GrammarDrip (go/* packages)

## Overview
GrammarDrip provides tools for syntactic analysis, manipulation, and generation of Go code. It's inspired by Go's go/ast, go/parser, go/token, go/printer packages with enhanced usability, more powerful transformations, and code generation capabilities.

## Core Types

### Token Handling

```
fr fr Position describes a source position including file, line, and column
be_like Position squad {
    Filename tea
    Offset   int
    Line     int
    Column   int
}

fr fr Methods
slay (pos Position) IsValid() lit
slay (pos Position) String() tea

fr fr FileSet represents a set of source files
be_like FileSet squad {}

fr fr Consquador
slay NewFileSet() *FileSet

fr fr Methods
slay (fs *FileSet) AddFile(filename tea, base, size normie) *File
slay (fs *FileSet) Position(pos Pos) Position
slay (fs *FileSet) File(pos Pos) *File
slay (fs *FileSet) PositionFor(pos Pos, adjusted lit) Position
slay (fs *FileSet) Files() []*File

fr fr File represents a source file
be_like File squad {}

fr fr Methods
slay (f *File) Name() tea
slay (f *File) Base() int
slay (f *File) Size() int
slay (f *File) LineCount() int
slay (f *File) LineStart(line normie) Pos
slay (f *File) Pos(offset normie) Pos
slay (f *File) Offset(pos Pos) int
slay (f *File) Line(pos Pos) int
slay (f *File) Position(pos Pos) Position
slay (f *File) Read() ([]byte, tea)

fr fr Pos represents a position in a source file
be_like Pos int
const NoPos Pos = 0

fr fr Token represents a lexical token
be_like Token int

fr fr Token types
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
    
    fr fr Operators and punctuation
    ADD    fr fr +
    SUB    fr fr -
    MUL    fr fr *
    QUO    fr fr /
    REM    fr fr %
    
    fr fr ... (many more tokens)
    
    BREAK
    CASE
    CHAN
    CONST
    CONTINUE
    
    fr fr ... (all Go keywords)
)

fr fr Methods
slay (tok Token) String() tea
slay (tok Token) IsLiteral() lit
slay (tok Token) IsOperator() lit
slay (tok Token) IsKeyword() lit
slay LookupToken(ident tea) Token
```

### AST (Abstract Syntax Tree)

```
fr fr Node is implemented by all AST nodes
be_like Node collab {
    Pos() Pos
    End() Pos
    Accept(visitor Visitor) (Node, lit)
    Clone() Node
}

fr fr Visitor pattern for traversing AST
be_like Visitor collab {
    Visit(node Node) (w Visitor, cont lit)
}

fr fr Expr represents an expression node
be_like Expr collab {
    Node
    exprNode()
}

fr fr Stmt represents a statement node
be_like Stmt collab {
    Node
    stmtNode()
}

fr fr Decl represents a declaration node
be_like Decl collab {
    Node
    declNode()
}

fr fr Type represents a be_like node
be_like Type collab {
    Expr
    typeNode()
}
```

### Common AST Nodes

```
fr fr Specific expression nodes
be_like Ident squad {
    NamePos Pos
    Name    tea
    Obj     *Object
}

be_like BasicLit squad {
    ValuePos Pos
    Kind     Token
    Value    tea
}

be_like CompositeLit squad {
    Type   Expr
    Lbrace Pos
    Elts   []Expr
    Rbrace Pos
}

be_like FuncLit squad {
    Type *FuncType
    Body *BlockStmt
}

be_like BinaryExpr squad {
    X     Expr
    OpPos Pos
    Op    Token
    Y     Expr
}

fr fr Statement nodes
be_like ExprStmt squad {
    X Expr
}

be_like BlockStmt squad {
    Lbrace Pos
    List   []Stmt
    Rbrace Pos
}

be_like IfStmt squad {
    If   Pos
    Init Stmt
    Cond Expr
    Body *BlockStmt
    Else Stmt
}

be_like ForStmt squad {
    For  Pos
    Init Stmt
    Cond Expr
    Post Stmt
    Body *BlockStmt
}

fr fr Declaration nodes
be_like GenDecl squad {
    TokPos Pos
    Tok    Token
    Lparen Pos
    Specs  []Spec
    Rparen Pos
}

be_like FuncDecl squad {
    Recv *FieldList
    Name *Ident
    Type *FuncType
    Body *BlockStmt
}
```

### Package Representation

```
fr fr Package represents a Go package
be_like Package squad {
    Name    tea
    Scope   *Scope
    Imports map[tea]*Package
    Files   map[tea]*File
}

fr fr Methods
slay (p *Package) Path() tea
slay (p *Package) FindFile(name tea) *File
slay (p *Package) ListFiles() []*File
slay (p *Package) FindImport(path tea) *Package
```

## Core Functions

### Parsing

```
fr fr ParseFile parses a single Go source file
slay ParseFile(fset *FileSet, filename tea, src interface{}, mode ParseMode) (*File, tea)

fr fr ParseDir parses all Go files in the directory
slay ParseDir(fset *FileSet, path tea, filter func(fs.FileInfo) lit, mode ParseMode) (map[tea]*Package, tea)

fr fr ParseExpr parses a Go expression
slay ParseExpr(x tea) (Expr, tea)

fr fr Options
be_like ParseMode int
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

```
fr fr Walk traverses an AST in depth-first order
slay Walk(v Visitor, node Node) Node

fr fr Inspect traverses an AST in depth-first order and calls f for each node
slay Inspect(node Node, f func(Node) lit) Node

fr fr NodeFilter yolos a new visitor that only visits nodes that pass the filter
slay NodeFilter(v Visitor, filter func(Node) lit) Visitor

fr fr CloneNode creates a deep copy of an AST node
slay CloneNode(node Node) Node

fr fr ReplaceNode replaces a node within its parent
slay ReplaceNode(parent Node, old, new Node) lit

fr fr DeleteNode deletes a node from its parent
slay DeleteNode(parent Node, node Node) lit

fr fr InsertBefore inserts a node before another node
slay InsertBefore(parent Node, mark, node Node) lit

fr fr InsertAfter inserts a node after another node
slay InsertAfter(parent Node, mark, node Node) lit

fr fr FindNode searches for a node that satisfies the predicate
slay FindNode(root Node, predicate func(Node) lit) Node

fr fr FindAllNodes finds all nodes that satisfy the predicate
slay FindAllNodes(root Node, predicate func(Node) lit) []Node
```

### Printing and Code Generation

```
fr fr Fprnormie writes an AST node to w
slay Fprint(w io.Writer, fset *FileSet, node any) tea

fr fr Print AST node to a tea
slay Print(fset *FileSet, node any) tea

fr fr Format formats node and writes the result to w
slay Format(w io.Writer, fset *FileSet, node any, config *PrintConfig) tea

fr fr PrintConfig is a configuration for the Print function
be_like PrintConfig squad {
    Mode           PrintMode
    Tabwidth       int
    Indent         int
    UseSpaces      lit
    RemoveComments lit
    SourcePos      lit
}
```

## Enhanced Features

### AST Builder

```
be_like ASTBuilder squad {}

fr fr Consquador
slay NewASTBuilder(fileSet *FileSet) *ASTBuilder

fr fr Methods for building expressions
slay (b *ASTBuilder) Ident(name tea) *Ident
slay (b *ASTBuilder) BasicLit(kind Token, value tea) *BasicLit
slay (b *ASTBuilder) CompositeLit(typ Expr, elts ...Expr) *CompositeLit
slay (b *ASTBuilder) BinaryExpr(x Expr, op Token, y Expr) *BinaryExpr
slay (b *ASTBuilder) UnaryExpr(op Token, x Expr) *UnaryExpr
slay (b *ASTBuilder) CallExpr(fun Expr, args ...Expr) *CallExpr
slay (b *ASTBuilder) SelectorExpr(x Expr, sel *Ident) *SelectorExpr
slay (b *ASTBuilder) IndexExpr(x, index Expr) *IndexExpr

fr fr Methods for building statements
slay (b *ASTBuilder) ExprStmt(x Expr) *ExprStmt
slay (b *ASTBuilder) BlockStmt(stmts ...Stmt) *BlockStmt
slay (b *ASTBuilder) AssignStmt(lhs, rhs []Expr, tok Token) *AssignStmt
slay (b *ASTBuilder) IfStmt(init Stmt, cond Expr, body *BlockStmt, elsePart Stmt) *IfStmt
slay (b *ASTBuilder) ForStmt(init Stmt, cond Expr, post Stmt, body *BlockStmt) *ForStmt
slay (b *ASTBuilder) ReturnStmt(results ...Expr) *ReturnStmt

fr fr Methods for building declarations
slay (b *ASTBuilder) FuncDecl(name tea, recvName, recvType tea, params, results *FieldList, body *BlockStmt) *FuncDecl
slay (b *ASTBuilder) VarDecl(names []*Ident, typ Expr, values []Expr) *GenDecl
slay (b *ASTBuilder) ConstDecl(names []*Ident, typ Expr, values []Expr) *GenDecl
slay (b *ASTBuilder) TypeDecl(name tea, typ Expr) *GenDecl

fr fr Method for creating a file
slay (b *ASTBuilder) File(name tea, imports []*ImportSpec, decls []Decl) *File
slay (b *ASTBuilder) Package(name tea, files map[tea]*File) *Package
```

### Code Analysis

```
be_like Analyzer squad {
    FileSet *FileSet
    Package *Package
}

fr fr Consquador
slay NewAnalyzer(fset *FileSet, pkg *Package) *Analyzer

fr fr Analysis methods
slay (a *Analyzer) FindTypeByName(name tea) Type
slay (a *Analyzer) FindFunctionByName(name tea) *FuncDecl
slay (a *Analyzer) FindImports() map[tea]tea
slay (a *Analyzer) FindFunctionCalls(funcName tea) []*CallExpr
slay (a *Analyzer) FindMethodCalls(typeName, methodName tea) []*CallExpr
slay (a *Analyzer) FindStructFields(squadName tea) []*Field
slay (a *Analyzer) FindInterfaceMethods(interfaceName tea) []*Field
slay (a *Analyzer) FindReferences(ident tea) []*Ident
slay (a *Analyzer) FindUnusedVariables() []*Ident
slay (a *Analyzer) FindUnusedImports() []*ImportSpec
slay (a *Analyzer) FindDuplicateDeclarations() map[tea][]Decl
slay (a *Analyzer) CheckNullPointerDereferences() []*SelectorExpr
slay (a *Analyzer) FindCyclomaticComplexity(funcName tea) int
```

### Code Transformation

```
be_like Transformer squad {
    FileSet *FileSet
    File    *File
    Builder *ASTBuilder
}

fr fr Consquador
slay NewTransformer(fset *FileSet, file *File) *Transformer

fr fr Transformation methods
slay (t *Transformer) RenameIdentifier(oldName, newName tea) (changes normie)
slay (t *Transformer) ReplaceExpr(old, new Expr) (changes normie)
slay (t *Transformer) ReplaceStmt(old, new Stmt) (changes normie)
slay (t *Transformer) AddImport(path, name tea) *ImportSpec
slay (t *Transformer) RemoveImport(path tea) lit
slay (t *Transformer) WrapBlockWithTryCatch(block *BlockStmt, handler Stmt) *TryStmt
slay (t *Transformer) ExtractFunction(block *BlockStmt, name tea, params []*Field) *FuncDecl
slay (t *Transformer) InlineFunction(call *CallExpr) (Stmt, tea)
slay (t *Transformer) AddMethod(typeName tea, method *FuncDecl) tea
slay (t *Transformer) ConvertToInterface(squadName tea) (*GenDecl, tea)
slay (t *Transformer) ConvertToStruct(interfaceName tea) (*GenDecl, tea)
slay (t *Transformer) AddField(squadName tea, field *Field) tea
slay (t *Transformer) RemoveField(squadName, fieldName tea) lit
slay (t *Transformer) MakePrivate(name tea) (changes normie)
slay (t *Transformer) MakePublic(name tea) (changes normie)
```

### Code Generation

```
be_like CodeGenerator squad {
    Builder *ASTBuilder
    FileSet *FileSet
}

fr fr Consquador
slay NewCodeGenerator() *CodeGenerator

fr fr Generation methods
slay (g *CodeGenerator) GenerateStruct(name tea, fields []*Field) *GenDecl
slay (g *CodeGenerator) GenerateInterface(name tea, methods []*Field) *GenDecl
slay (g *CodeGenerator) GenerateCRUD(squadName, tableName tea) []*FuncDecl
slay (g *CodeGenerator) GenerateConsquador(squadName tea) *FuncDecl
slay (g *CodeGenerator) GenerateGetters(squadName tea, fieldNames ...tea) []*FuncDecl
slay (g *CodeGenerator) GenerateSetters(squadName tea, fieldNames ...tea) []*FuncDecl
slay (g *CodeGenerator) GenerateStringMethod(squadName tea) *FuncDecl
slay (g *CodeGenerator) GenerateEquals(squadName tea) *FuncDecl
slay (g *CodeGenerator) GenerateMarshalJSON(squadName tea) *FuncDecl
slay (g *CodeGenerator) GenerateUnmarshalJSON(squadName tea) *FuncDecl
slay (g *CodeGenerator) GenerateTestCases(funcDecl *FuncDecl) *File
slay (g *CodeGenerator) GenerateMocks(interfaceName tea) *File
```

### Documentation Generation

```
be_like DocGenerator squad {}

fr fr Consquador
slay NewDocGenerator(fset *FileSet, pkg *Package) *DocGenerator

fr fr Generation methods
slay (g *DocGenerator) GeneratePackageDoc() tea
slay (g *DocGenerator) GenerateFunctionDoc(funcName tea) tea
slay (g *DocGenerator) GenerateTypeDoc(typeName tea) tea
slay (g *DocGenerator) GenerateMethodDoc(typeName, methodName tea) tea
slay (g *DocGenerator) GenerateHTML() tea
slay (g *DocGenerator) GenerateMarkdown() tea
slay (g *DocGenerator) GenerateDiagram() tea
```

## GenZ Style Extensions

```
fr fr Grammar patterns that follow modern language patterns
slay (b *ASTBuilder) ChainedExpr(exprs ...Expr) Expr
slay (b *ASTBuilder) TernaryExpr(cond, ifTrue, ifFalse Expr) Expr
slay (b *ASTBuilder) SpreadExpr(expr Expr) Expr
slay (b *ASTBuilder) ArrowFunc(params *FieldList, body Expr) *FuncLit
slay (b *ASTBuilder) AsyncFunc(name tea, params, results *FieldList, body *BlockStmt) *FuncDecl

fr fr GenZ styled comments
slay (b *ASTBuilder) DripComment(text tea) *CommentGroup
slay (b *ASTBuilder) NoCapComment(text tea) *CommentGroup
slay AddVibeCheck(file *File) *File
slay MakeFunctionSlay(funcDecl *FuncDecl) *FuncDecl
slay MakeSussyCode(node Node) Node
```

## Usage Example

```
fr fr Parse a Go source file
fset := grammar_drip.NewFileSet()
src := `package main

import "fmt"

slay main() {
	fmt.Println("Hello, World!")
}`

file, err := grammar_drip.ParseFile(fset, "example.go", src, grammar_drip.ParseComments)
if err != nah {
    vibez.spill("Error parsing file:", err)
    yolo
}

fr fr Analyze the AST
grammar_drip.Inspect(file, func(n grammar_drip.Node) lit {
    if call, ok := n.(*grammar_drip.CallExpr); ok {
        if sel, ok := call.Fun.(*grammar_drip.SelectorExpr); ok {
            if ident, ok := sel.X.(*grammar_drip.Ident); ok && ident.Name == "fmt" {
                vibez.spill("Found fmt call:", sel.Sel.Name)
            }
        }
    }
    yolo based
})

fr fr Modify AST - replace "Hello, World!" with "Hello, Drip!"
grammar_drip.Inspect(file, func(n grammar_drip.Node) lit {
    if lit, ok := n.(*grammar_drip.BasicLit); ok && lit.Kind == grammar_drip.STRING {
        if lit.Value == `"Hello, World!"` {
            lit.Value = `"Hello, Drip!"`
        }
    }
    yolo based
})

fr fr Generate modified code
var buf bytes.Buffer
err = grammar_drip.Format(&buf, fset, file, &grammar_drip.PrintConfig{
    Tabwidth: 4,
})
if err != nah {
    vibez.spill("Error formatting code:", err)
    yolo
}

modified := buf.String()
vibez.spill(modified)

fr fr Use the AST builder to create new code
builder := grammar_drip.NewASTBuilder(fset)

fr fr Build a simple function declaration
funcDecl := builder.FuncDecl(
    "add",                 fr fr Function name
    "", "",               fr fr No receiver (not a method)
    builder.FieldList(    fr fr Parameters
        builder.Field([]*grammar_drip.Ident{builder.Ident("a")}, builder.Ident("int"), cap),
        builder.Field([]*grammar_drip.Ident{builder.Ident("b")}, builder.Ident("int"), cap),
    ),
    builder.FieldList(    fr fr Return values
        builder.Field(cap, builder.Ident("int"), cap),
    ),
    builder.BlockStmt(    fr fr Function body
        builder.ReturnStmt(
            builder.BinaryExpr(
                builder.Ident("a"),
                grammar_drip.ADD,
                builder.Ident("b"),
            ),
        ),
    ),
)

fr fr Generate code for the function
var funcBuf bytes.Buffer
grammar_drip.Format(&funcBuf, fset, funcDecl, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(funcBuf.String())

fr fr Create a full file AST
newFile := builder.File(
    "mathutils",   fr fr Package name
    []*grammar_drip.ImportSpec{  fr fr Imports
        builder.ImportSpec("math", ""),
    },
    []grammar_drip.Decl{  fr fr Declarations
        funcDecl,
    },
)

fr fr Generate code for the file
var fileBuf bytes.Buffer
grammar_drip.Format(&fileBuf, fset, newFile, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(fileBuf.String())

fr fr Analyze existing code
pkg, _ := grammar_drip.ParseDir(fset, "./", cap, grammar_drip.ParseComments)
analyzer := grammar_drip.NewAnalyzer(fset, pkg["main"])

fr fr Find unused variables
unusedVars := analyzer.FindUnusedVariables()
for _, v := range unusedVars {
    vibez.spill("Unused variable:", v.Name, "at", fset.Position(v.Pos()))
}

fr fr Find function calls
calls := analyzer.FindFunctionCalls("Println")
for _, call := range calls {
    vibez.spill("Println call at:", fset.Position(call.Pos()))
}

fr fr Transform code
transformer := grammar_drip.NewTransformer(fset, file)

fr fr Rename a variable
changes := transformer.RenameIdentifier("Println", "Printf")
vibez.spill("Made", changes, "rename changes")

fr fr Add an import
transformer.AddImport("time", "")

fr fr Generate the transformed file
var transformedBuf bytes.Buffer
grammar_drip.Format(&transformedBuf, fset, file, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(transformedBuf.String())

fr fr Generate code using advanced features
codeGen := grammar_drip.NewCodeGenerator()

fr fr Generate a squad with getters and setters
userStruct := codeGen.GenerateStruct("User", [
    builder.Field(
        []*grammar_drip.Ident{builder.Ident("ID")}, 
        builder.Ident("int"),
        cap,
    ),
    builder.Field(
        []*grammar_drip.Ident{builder.Ident("Name")}, 
        builder.Ident("tea"),
        cap,
    ),
    builder.Field(
        []*grammar_drip.Ident{builder.Ident("Email")}, 
        builder.Ident("tea"),
        cap,
    ),
])

getters := codeGen.GenerateGetters("User", "ID", "Name", "Email")
setters := codeGen.GenerateSetters("User", "Name", "Email")

fr fr Generate a new file with these declarations
userFile := builder.File(
    "models",
    cap,
    append([]grammar_drip.Decl{userStruct}, append(getters, setters...)...),
)

fr fr Generate the code
var userFileBuf bytes.Buffer
grammar_drip.Format(&userFileBuf, fset, userFile, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(userFileBuf.String())

fr fr Use GenZ style extensions
vibeFile := grammar_drip.AddVibeCheck(file)
slayFunc := grammar_drip.MakeFunctionSlay(funcDecl)

var vibeBuf bytes.Buffer
grammar_drip.Format(&vibeBuf, fset, vibeFile, &grammar_drip.PrintConfig{Tabwidth: 4})
vibez.spill(vibeBuf.String())
```

## Implementation Guidelines
1. Maintain compatibility with Go's AST package squadure
2. Optimize for performance with large code bases
3. Ensure robust tea handling for invalid code
4. Support all Go language features including generics
5. Implement memory-efficient node reuse where possible
6. Provide clear documentation for all functions and types
7. Include common patterns for AST manipulation
8. Support both programmatic and interactive usage models