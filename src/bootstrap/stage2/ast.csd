// Stage 2 AST - Abstract Syntax Tree nodes for CURSED minimal subset
// Written in CURSED using only basic features for bootstrapping

// Node interface for all AST nodes
interface Node {
    string() string
}

// Statement interface for statement nodes
interface Statement : Node {
    statement_node()
}

// Expression interface for expression nodes  
interface Expression : Node {
    expression_node()
}

// Program represents the root of the AST
struct Program {
    statements: []Statement
}

func (p *Program) string() string {
    result := ""
    for stmt in p.statements {
        result = result + stmt.string()
    }
    return result
}

// Import statement
struct ImportStatement {
    path: string
}

func (i *ImportStatement) statement_node() {}
func (i *ImportStatement) string() string {
    return "import \"" + i.path + "\""
}

// Function statement
struct FunctionStatement {
    name: string
    parameters: []Parameter
    return_type: string
    body: BlockStatement
}

func (f *FunctionStatement) statement_node() {}
func (f *FunctionStatement) string() string {
    params := ""
    for i, param in f.parameters {
        if i > 0 {
            params = params + ", "
        }
        params = params + param.name + ": " + param.type
    }
    
    result := "func " + f.name + "(" + params + ")"
    if f.return_type != "" {
        result = result + " " + f.return_type
    }
    result = result + " " + f.body.string()
    return result
}

// Parameter for function definitions
struct Parameter {
    name: string
    type: string
}

// Let statement (variable declaration)
struct LetStatement {
    name: string
    type: string
    value: Expression
}

func (l *LetStatement) statement_node() {}
func (l *LetStatement) string() string {
    result := "let " + l.name
    if l.type != "" {
        result = result + ": " + l.type
    }
    if l.value != nil {
        result = result + " = " + l.value.string()
    }
    return result
}

// Return statement
struct ReturnStatement {
    value: Expression
}

func (r *ReturnStatement) statement_node() {}
func (r *ReturnStatement) string() string {
    result := "return"
    if r.value != nil {
        result = result + " " + r.value.string()
    }
    return result
}

// Expression statement
struct ExpressionStatement {
    expression: Expression
}

func (e *ExpressionStatement) statement_node() {}
func (e *ExpressionStatement) string() string {
    return e.expression.string()
}

// Block statement
struct BlockStatement {
    statements: []Statement
}

func (b *BlockStatement) statement_node() {}
func (b *BlockStatement) string() string {
    result := "{"
    for stmt in b.statements {
        result = result + stmt.string() + ";"
    }
    result = result + "}"
    return result
}

// If statement
struct IfStatement {
    condition: Expression
    consequence: BlockStatement
    alternative: BlockStatement
}

func (i *IfStatement) statement_node() {}
func (i *IfStatement) string() string {
    result := "if " + i.condition.string() + " " + i.consequence.string()
    if i.alternative.statements != nil {
        result = result + " else " + i.alternative.string()
    }
    return result
}

// For statement
struct ForStatement {
    init: Statement
    condition: Expression
    update: Statement
    body: BlockStatement
}

func (f *ForStatement) statement_node() {}
func (f *ForStatement) string() string {
    result := "for "
    if f.init != nil {
        result = result + f.init.string() + "; "
    } else {
        result = result + "; "
    }
    if f.condition != nil {
        result = result + f.condition.string() + "; "
    } else {
        result = result + "; "
    }
    if f.update != nil {
        result = result + f.update.string()
    }
    result = result + " " + f.body.string()
    return result
}

// Struct statement
struct StructStatement {
    name: string
    fields: []StructField
}

func (s *StructStatement) statement_node() {}
func (s *StructStatement) string() string {
    result := "struct " + s.name + " {"
    for field in s.fields {
        result = result + field.name + ": " + field.type + ";"
    }
    result = result + "}"
    return result
}

// Struct field
struct StructField {
    name: string
    type: string
}

// Identifier expression
struct Identifier {
    value: string
}

func (i *Identifier) expression_node() {}
func (i *Identifier) string() string {
    return i.value
}

// Integer literal expression
struct IntegerLiteral {
    value: int
}

func (i *IntegerLiteral) expression_node() {}
func (i *IntegerLiteral) string() string {
    return string(i.value)
}

// String literal expression
struct StringLiteral {
    value: string
}

func (s *StringLiteral) expression_node() {}
func (s *StringLiteral) string() string {
    return "\"" + s.value + "\""
}

// Boolean literal expression
struct BooleanLiteral {
    value: bool
}

func (b *BooleanLiteral) expression_node() {}
func (b *BooleanLiteral) string() string {
    if b.value {
        return "true"
    }
    return "false"
}

// Prefix expression (!, -)
struct PrefixExpression {
    operator: string
    right: Expression
}

func (p *PrefixExpression) expression_node() {}
func (p *PrefixExpression) string() string {
    return "(" + p.operator + p.right.string() + ")"
}

// Infix expression (binary operators)
struct InfixExpression {
    left: Expression
    operator: string
    right: Expression
}

func (i *InfixExpression) expression_node() {}
func (i *InfixExpression) string() string {
    return "(" + i.left.string() + " " + i.operator + " " + i.right.string() + ")"
}

// Function call expression
struct CallExpression {
    function: Expression
    arguments: []Expression
}

func (c *CallExpression) expression_node() {}
func (c *CallExpression) string() string {
    args := ""
    for i, arg in c.arguments {
        if i > 0 {
            args = args + ", "
        }
        args = args + arg.string()
    }
    return c.function.string() + "(" + args + ")"
}

// Array literal expression
struct ArrayLiteral {
    elements: []Expression
}

func (a *ArrayLiteral) expression_node() {}
func (a *ArrayLiteral) string() string {
    elements := ""
    for i, elem in a.elements {
        if i > 0 {
            elements = elements + ", "
        }
        elements = elements + elem.string()
    }
    return "[" + elements + "]"
}

// Index expression
struct IndexExpression {
    left: Expression
    index: Expression
}

func (i *IndexExpression) expression_node() {}
func (i *IndexExpression) string() string {
    return "(" + i.left.string() + "[" + i.index.string() + "])"
}

// Assignment expression
struct AssignmentExpression {
    name: Identifier
    value: Expression
}

func (a *AssignmentExpression) expression_node() {}
func (a *AssignmentExpression) string() string {
    return a.name.string() + " = " + a.value.string()
}
