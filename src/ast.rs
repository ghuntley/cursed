//! Abstract Syntax Tree for CURSED language

use crate::error::CursedError;

/// Root program node
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub imports: Vec<ImportStatement>,
    pub package: Option<PackageDeclaration>,
}

/// Import statement
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
    pub alias: Option<String>,
    pub items: Vec<String>,
}

/// Package declaration
#[derive(Debug, Clone)]
pub struct PackageDeclaration {
    pub name: String,
    pub version: Option<String>,
}

/// Statement types
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Let(LetStatement),
    Assignment(AssignmentStatement),
    Return(ReturnStatement),
    If(IfStatement),
    Function(FunctionStatement),
    While(WhileStatement),
    For(ForStatement),
    ForIn(ForInStatement),
    Switch(SwitchStatement),
    Goroutine(GoroutineStatement),
    Channel(ChannelStatement),
    Select(SelectStatement),
    Struct(StructStatement),
    Interface(InterfaceStatement),
    Panic(PanicStatement),
    Catch(CatchStatement),
    Defer(DeferStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Increment(IncrementStatement),
    Decrement(DecrementStatement),
}

/// Expression types
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Character(char),
    Binary(BinaryExpression),
    Call(CallExpression),
    MemberAccess(MemberAccessExpression),
    Literal(Literal),
    Unary(UnaryExpression),
    Array(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
    ChannelSend(ChannelSendExpression),
    ChannelReceive(ChannelReceiveExpression),
    ChannelCreation(ChannelCreationExpression),
    StructLiteral(StructLiteralExpression),
    Lambda(LambdaExpression),
    Tuple(TupleExpression),
    TupleAccess(TupleAccessExpression),
    ArrayAccess(ArrayAccessExpression),
}

/// Binary expression
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

/// Function call expression
#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

/// Member access expression (e.g., object.property)
#[derive(Debug, Clone)]
pub struct MemberAccessExpression {
    pub object: Box<Expression>,
    pub property: String,
}

/// Struct literal expression (e.g., Person { name: "Alice", age: 30 })
#[derive(Debug, Clone)]
pub struct StructLiteralExpression {
    pub struct_name: String,
    pub fields: Vec<StructFieldAssignment>,
}

/// Struct field assignment in a struct literal
#[derive(Debug, Clone)]
pub struct StructFieldAssignment {
    pub field_name: String,
    pub value: Expression,
}

/// Lambda expression (anonymous function)
#[derive(Debug, Clone)]
pub struct LambdaExpression {
    pub parameters: Vec<String>,
    pub body: Box<Expression>,
}

/// Tuple expression (e.g., (1, "hello", based))
#[derive(Debug, Clone)]
pub struct TupleExpression {
    pub elements: Vec<Expression>,
}

/// Tuple access expression (e.g., tuple.0, tuple.1)
#[derive(Debug, Clone)]
pub struct TupleAccessExpression {
    pub tuple: Box<Expression>,
    pub index: usize,
}

/// Array access expression (e.g., array[0], array[index])
#[derive(Debug, Clone)]
pub struct ArrayAccessExpression {
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}

/// Visibility level for symbols
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,    // pub - accessible from other modules
    Private,   // private (default) - only accessible within current module
    Package,   // pkg - accessible within current package
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}

/// Let statement target (single variable or tuple destructuring)
#[derive(Debug, Clone)]
pub enum LetTarget {
    Single(String),
    Tuple(Vec<String>),
}

impl LetTarget {
    /// Get the primary name for compatibility (returns first name for tuples)
    pub fn primary_name(&self) -> String {
        match self {
            LetTarget::Single(name) => name.clone(),
            LetTarget::Tuple(names) => names.first().cloned().unwrap_or_default(),
        }
    }
}

/// Let statement
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub target: LetTarget,
    pub value: Expression,
    pub var_type: Option<Type>, // Type annotation for the variable
    pub visibility: Visibility,
}

/// Assignment statement  
#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub target: AssignmentTarget,
    pub value: Expression,
}

/// Assignment target (single variable or tuple destructuring)
#[derive(Debug, Clone)]
pub enum AssignmentTarget {
    Single(String),
    Tuple(Vec<String>),
}

/// Return statement
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

/// If statement
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Vec<Statement>,
    pub else_branch: Option<Vec<Statement>>,
}

/// Generic type parameter
#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub name: String,
    pub bounds: Vec<String>, // trait bounds like Clone, Debug, etc.
}

/// Where clause for additional generic constraints
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub constraints: Vec<TypeConstraint>,
}

/// Type constraint for where clauses
#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub type_name: String,
    pub bounds: Vec<String>,
}

/// Function statement
#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>, // Generic type parameters
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub return_type: Option<Type>,
    pub where_clause: Option<WhereClause>, // Where clause for constraints
    pub visibility: Visibility,
}

/// While statement
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

/// For statement
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Vec<Statement>,
}

/// For-in statement
#[derive(Debug, Clone)]
pub struct ForInStatement {
    pub variable: String,
    pub iterable: Expression,
    pub body: Vec<Statement>,
}

/// Switch statement (vibe_check keyword)
#[derive(Debug, Clone)]
pub struct SwitchStatement {
    pub expression: Expression,
    pub cases: Vec<SwitchCase>,
    pub default_case: Option<Vec<Statement>>,
}

/// Switch case (mood keyword)
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub pattern: Expression,
    pub body: Vec<Statement>,
}

/// Goroutine statement
#[derive(Debug, Clone)]
pub struct GoroutineStatement {
    pub expression: Expression,
}

/// Channel statement  
#[derive(Debug, Clone)]
pub struct ChannelStatement {
    pub name: String,
    pub buffer_size: Option<Expression>,
}

/// Channel send expression (channel <- value)
#[derive(Debug, Clone)]
pub struct ChannelSendExpression {
    pub channel: Box<Expression>,
    pub value: Box<Expression>,
}

/// Channel receive expression (<-channel)
#[derive(Debug, Clone)]
pub struct ChannelReceiveExpression {
    pub channel: Box<Expression>,
}

/// Channel creation expression (dm type())
#[derive(Debug, Clone)]
pub struct ChannelCreationExpression {
    pub element_type: Type,
    pub capacity: Option<Box<Expression>>,
}

/// Select statement for channel multiplexing
#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub cases: Vec<SelectCase>,
    pub default_case: Option<Vec<Statement>>,
}

/// Select case (mood keyword with channel operations)
#[derive(Debug, Clone)]
pub struct SelectCase {
    pub operation: Box<Expression>, // ChannelSend or ChannelReceive
    pub body: Vec<Statement>,
}

/// Struct statement (squad keyword)
#[derive(Debug, Clone)]
pub struct StructStatement {
    pub name: String,
    pub fields: Vec<StructField>,
    pub visibility: Visibility,
}

/// Struct field definition
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: Option<Type>,
    pub visibility: Visibility,
}

/// Interface statement (collab keyword)
#[derive(Debug, Clone)]
pub struct InterfaceStatement {
    pub name: String,
    pub methods: Vec<MethodSignature>,
    pub visibility: Visibility,
}

/// Method signature for interfaces
#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

/// Parameter definition
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<Type>,
}

/// Unary expression
#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
    Plus,
}

/// Literal values
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Nil,
}

/// Panic statement for error throwing (yeet_error message)
#[derive(Debug, Clone)]
pub struct PanicStatement {
    pub message: Box<Expression>,
}

impl PanicStatement {
    pub fn new(message: Box<Expression>) -> Self {
        Self { message }
    }
}

/// Catch statement for error handling (catch { ... })
#[derive(Debug, Clone)]
pub struct CatchStatement {
    pub protected_block: Vec<Statement>,
    pub recovery_block: Option<Vec<Statement>>,
    pub error_variable: Option<String>,
}

impl CatchStatement {
    pub fn new(protected_block: Vec<Statement>) -> Self {
        Self {
            protected_block,
            recovery_block: None,
            error_variable: None,
        }
    }
    
    pub fn with_recovery(mut self, recovery_block: Vec<Statement>) -> Self {
        self.recovery_block = Some(recovery_block);
        self
    }
    
    pub fn with_error_var(mut self, error_variable: String) -> Self {
        self.error_variable = Some(error_variable);
        self
    }
}

/// Defer statement for deferred execution (later expression)
#[derive(Debug, Clone)]
pub struct DeferStatement {
    pub expression: Box<Expression>,
}

impl DeferStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

/// Break statement (ghosted)
#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub label: Option<String>,
}

impl BreakStatement {
    pub fn new() -> Self {
        Self { label: None }
    }
    
    pub fn with_label(label: String) -> Self {
        Self { label: Some(label) }
    }
}

impl Default for BreakStatement {
    fn default() -> Self {
        Self::new()
    }
}

/// Continue statement (simp)
#[derive(Debug, Clone)]
pub struct ContinueStatement {
    pub label: Option<String>,
}

impl ContinueStatement {
    pub fn new() -> Self {
        Self { label: None }
    }
    
    pub fn with_label(label: String) -> Self {
        Self { label: Some(label) }
    }
}

impl Default for ContinueStatement {
    fn default() -> Self {
        Self::new()
    }
}

/// Increment statement (++variable or variable++)
#[derive(Debug, Clone)]
pub struct IncrementStatement {
    pub variable: String,
    pub is_prefix: bool,
}

impl IncrementStatement {
    pub fn new(variable: String, is_prefix: bool) -> Self {
        Self { variable, is_prefix }
    }
    
    pub fn prefix(variable: String) -> Self {
        Self::new(variable, true)
    }
    
    pub fn postfix(variable: String) -> Self {
        Self::new(variable, false)
    }
}

/// Decrement statement (--variable or variable--)
#[derive(Debug, Clone)]
pub struct DecrementStatement {
    pub variable: String,
    pub is_prefix: bool,
}

impl DecrementStatement {
    pub fn new(variable: String, is_prefix: bool) -> Self {
        Self { variable, is_prefix }
    }
    
    pub fn prefix(variable: String) -> Self {
        Self::new(variable, true)
    }
    
    pub fn postfix(variable: String) -> Self {
        Self::new(variable, false)
    }
}

/// Type annotations
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    Function(Vec<Type>, Box<Type>),
    Array(Box<Type>, Option<usize>),  // Array type with optional size [N]T
    Slice(Box<Type>),                 // Slice type []T
    Custom(String),
    // CURSED-specific types
    Normie,              // Standard/basic integer type (normie)
    Tea,                 // String/information type (tea)
    Lit,                 // Boolean/truth type (lit)
    Sip,                 // Character type (sip)
    Squad(Box<Type>),    // Array/collection type (squad)
    Collab(String),      // Interface type (collab)
    Dm(Box<Type>),       // Channel type (dm<T>)
    Tuple(Vec<Type>),    // Tuple type (tuple)
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Integer => write!(f, "normie"),
            Type::Float => write!(f, "tea"), 
            Type::String => write!(f, "tea"),
            Type::Boolean => write!(f, "lit"),
            Type::Void => write!(f, "void"),
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Array(inner, size) => {
                if let Some(size) = size {
                    write!(f, "[{}]{}", size, inner)
                } else {
                    write!(f, "[]{}", inner)
                }
            }
            Type::Slice(inner) => write!(f, "[]{}", inner),
            Type::Custom(name) => write!(f, "{}", name),
            Type::Normie => write!(f, "normie"),
            Type::Tea => write!(f, "tea"),
            Type::Lit => write!(f, "lit"),
            Type::Sip => write!(f, "sip"),
            Type::Squad(inner) => write!(f, "squad<{}>", inner),
            Type::Collab(name) => write!(f, "collab<{}>", name),
            Type::Dm(inner) => write!(f, "dm<{}>", inner),
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl Type {
    /// Convert Type to string for backward compatibility
    pub fn to_string_compat(&self) -> String {
        format!("{}", self)
    }
}

/// AST visitor trait for traversing the AST
pub trait AstVisitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_statement(&mut self, statement: &Statement) -> T;
    fn visit_expression(&mut self, expression: &Expression) -> T;
}

impl Default for Program {
    fn default() -> Self {
        Self {
            statements: vec![],
            imports: vec![],
            package: None,
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Parse a program from source code (simplified implementation)
pub fn parse_program(source: &str) -> Result<Program, CursedError> {
    // This is a simplified implementation for compatibility
    // Real parsing would use the lexer and parser modules
    let lexer = crate::lexer::Lexer::new(source.to_string());
    let mut parser = crate::parser::Parser::new(lexer)?;
    parser.parse_program()
}
