fr fr CURSED AST Implementation
fr fr Migrated from ast.zig to pure CURSED

yeet "testz"

fr fr Core AST node types
squad Expression {
    spill tag tea
    spill data normie
}

squad Statement {
    spill tag tea
    spill data normie
}

squad Type {
    spill tag tea
    spill data normie
}

fr fr Program structure
squad Program {
    spill statements []Statement
    spill imports []ImportStatement
    spill package PackageDeclaration
}

squad ImportStatement {
    spill path tea
    spill alias tea
    spill items []tea
}

squad PackageDeclaration {
    spill name tea
    spill version tea
}

fr fr Function-related structures
squad FunctionStatement {
    spill name tea
    spill parameters []Parameter
    spill type_parameters []TypeParameter
    spill return_type Type
    spill body []Statement
}

squad Parameter {
    spill name tea
    spill param_type Type
    spill is_mutable lit
    spill default_value Expression
}

squad TypeParameter {
    spill name tea
    spill constraints []Type
}

fr fr Variable and assignment structures
squad LetStatement {
    spill name tea
    spill var_type Type
    spill initializer Expression
    spill is_mutable lit
}

squad ShortDeclarationStatement {
    spill names []tea
    spill values []Expression
}

squad AssignmentStatement {
    spill target Expression
    spill operator tea
    spill value Expression
}

fr fr Control flow structures
squad IfStatement {
    spill condition Expression
    spill then_body []Statement
    spill else_body []Statement
}

squad WhileStatement {
    spill condition Expression
    spill body []Statement
}

squad ReturnStatement {
    spill value Expression
}

squad BreakStatement {
    fr fr Empty - just a marker
}

squad ContinueStatement {
    fr fr Empty - just a marker
}

fr fr Struct and interface structures
squad StructStatement {
    spill name tea
    spill fields []StructField
    spill type_parameters []TypeParameter
}

squad StructField {
    spill name tea
    spill field_type Type
    spill is_public lit
    spill default_value Expression
}

squad InterfaceStatement {
    spill name tea
    spill methods []InterfaceMethod
    spill type_parameters []TypeParameter
}

squad InterfaceMethod {
    spill name tea
    spill parameters []Parameter
    spill return_type Type
}

fr fr Concurrency structures
squad GoroutineStatement {
    spill body []Statement
}

squad ChannelExpression {
    spill element_type Type
    spill buffer_size normie
}

squad ChannelSendExpression {
    spill channel Expression
    spill value Expression
}

squad ChannelReceiveExpression {
    spill channel Expression
}

fr fr Expression structures
squad BinaryExpression {
    spill left Expression
    spill operator tea
    spill right Expression
}

squad UnaryExpression {
    spill operator tea
    spill operand Expression
}

squad CallExpression {
    spill function Expression
    spill arguments []Expression
}

squad MemberAccessExpression {
    spill object Expression
    spill member tea
}

squad ArrayAccessExpression {
    spill array Expression
    spill index Expression
}

squad ArrayExpression {
    spill elements []Expression
}

squad MapExpression {
    spill pairs []MapPair
}

squad MapPair {
    spill key Expression
    spill value Expression
}

squad StructLiteralExpression {
    spill struct_type Type
    spill fields []FieldInitializer
}

squad FieldInitializer {
    spill name tea
    spill value Expression
}

squad TupleExpression {
    spill elements []Expression
}

squad TupleAccessExpression {
    spill tuple Expression
    spill index normie
}

fr fr Pattern matching structures
squad MatchExpression {
    spill value Expression
    spill cases []MatchCase
}

squad MatchCase {
    spill pattern Expression
    spill guard Expression
    spill body Expression
}

squad TypeSwitchExpression {
    spill value Expression
    spill cases []TypeSwitchCase
}

squad TypeSwitchCase {
    spill type_pattern Type
    spill variable_name tea
    spill body []Statement
}

fr fr Type structures
squad PrimitiveType {
    spill name tea
}

squad ArrayType {
    spill element_type Type
    spill size normie
}

squad MapType {
    spill key_type Type
    spill value_type Type
}

squad ChannelType {
    spill element_type Type
    spill is_send_only lit
    spill is_receive_only lit
}

squad FunctionType {
    spill parameters []Type
    spill return_type Type
}

squad TupleType {
    spill elements []Type
}

squad IdentifierType {
    spill name tea
}

squad GenericType {
    spill base_type Type
    spill type_arguments []Type
}

fr fr Error handling structures
squad ErrorHandlingStatement {
    spill body []Statement
    spill error_variable tea
    spill recovery_body []Statement
}

squad YikesStatement {
    spill condition Expression
    spill body []Statement
    spill error_variable tea
}

squad FamStatement {
    spill body []Statement
    spill recovery_body []Statement
    spill error_variable tea
}

fr fr Literal expressions
squad LiteralExpression {
    spill value_type tea
    spill value tea
}

squad BooleanExpression {
    spill value lit
}

squad NumberExpression {
    spill value tea
    spill number_type tea
}

squad StringExpression {
    spill value tea
}

squad IdentifierExpression {
    spill name tea
}

fr fr Constructor functions for AST nodes
slay createProgram() Program {
    damn Program{
        statements: [],
        imports: [],
        package: PackageDeclaration{name: "", version: ""}
    }
}

slay createImportStatement(path tea) ImportStatement {
    damn ImportStatement{
        path: path,
        alias: "",
        items: []
    }
}

slay createPackageDeclaration(name tea) PackageDeclaration {
    damn PackageDeclaration{
        name: name,
        version: ""
    }
}

slay createFunctionStatement(name tea) FunctionStatement {
    damn FunctionStatement{
        name: name,
        parameters: [],
        type_parameters: [],
        return_type: createVoidType(),
        body: []
    }
}

slay createParameter(name tea, param_type Type) Parameter {
    damn Parameter{
        name: name,
        param_type: param_type,
        is_mutable: cringe,
        default_value: createNullExpression()
    }
}

slay createTypeParameter(name tea) TypeParameter {
    damn TypeParameter{
        name: name,
        constraints: []
    }
}

slay createLetStatement(name tea, is_mutable lit) LetStatement {
    damn LetStatement{
        name: name,
        var_type: createVoidType(),
        initializer: createNullExpression(),
        is_mutable: is_mutable
    }
}

slay createShortDeclarationStatement(names []tea, values []Expression) Statement {
    sus stmt Statement = Statement{
        tag: "ShortDeclaration",
        data: 0 fr fr Would point to ShortDeclarationStatement in full implementation
    }
    damn stmt
}

slay createAssignmentStatement(target Expression, operator tea, value Expression) Statement {
    sus stmt Statement = Statement{
        tag: "Assignment",
        data: 0 fr fr Would point to AssignmentStatement in full implementation
    }
    damn stmt
}

slay createReturnStatement(value Expression) Statement {
    sus stmt Statement = Statement{
        tag: "Return",
        data: 0 fr fr Would point to ReturnStatement in full implementation
    }
    damn stmt
}

slay createBreakStatement() Statement {
    sus stmt Statement = Statement{
        tag: "Break",
        data: 0
    }
    damn stmt
}

slay createContinueStatement() Statement {
    sus stmt Statement = Statement{
        tag: "Continue",
        data: 0
    }
    damn stmt
}

slay createIfStatement(condition Expression, then_body []Statement, else_body []Statement) Statement {
    sus stmt Statement = Statement{
        tag: "If",
        data: 0 fr fr Would point to IfStatement in full implementation
    }
    damn stmt
}

slay createWhileStatement(condition Expression, body []Statement) Statement {
    sus stmt Statement = Statement{
        tag: "While",
        data: 0 fr fr Would point to WhileStatement in full implementation
    }
    damn stmt
}

slay createStructStatement(name tea, fields []StructField) Statement {
    sus stmt Statement = Statement{
        tag: "Struct",
        data: 0 fr fr Would point to StructStatement in full implementation
    }
    damn stmt
}

slay createStructField(name tea, field_type Type) StructField {
    damn StructField{
        name: name,
        field_type: field_type,
        is_public: based,
        default_value: createNullExpression()
    }
}

slay createInterfaceStatement(name tea, methods []InterfaceMethod) Statement {
    sus stmt Statement = Statement{
        tag: "Interface",
        data: 0 fr fr Would point to InterfaceStatement in full implementation
    }
    damn stmt
}

slay createInterfaceMethod(name tea, parameters []Parameter, return_type Type) InterfaceMethod {
    damn InterfaceMethod{
        name: name,
        parameters: parameters,
        return_type: return_type
    }
}

slay createGoroutineStatement(body []Statement) Statement {
    sus stmt Statement = Statement{
        tag: "Goroutine",
        data: 0 fr fr Would point to GoroutineStatement in full implementation
    }
    damn stmt
}

slay createExpressionStatement(expr Expression) Statement {
    sus stmt Statement = Statement{
        tag: "Expression",
        data: 0 fr fr Would point to expression in full implementation
    }
    damn stmt
}

fr fr Expression constructors
slay createBinaryExpression(left Expression, operator tea, right Expression) Expression {
    sus expr Expression = Expression{
        tag: "Binary",
        data: 0 fr fr Would point to BinaryExpression in full implementation
    }
    damn expr
}

slay createUnaryExpression(operator tea, operand Expression) Expression {
    sus expr Expression = Expression{
        tag: "Unary",
        data: 0 fr fr Would point to UnaryExpression in full implementation
    }
    damn expr
}

slay createCallExpression(function Expression, arguments []Expression) Expression {
    sus expr Expression = Expression{
        tag: "Call",
        data: 0 fr fr Would point to CallExpression in full implementation
    }
    damn expr
}

slay createMemberAccessExpression(object Expression, member tea) Expression {
    sus expr Expression = Expression{
        tag: "MemberAccess",
        data: 0 fr fr Would point to MemberAccessExpression in full implementation
    }
    damn expr
}

slay createArrayAccessExpression(array Expression, index Expression) Expression {
    sus expr Expression = Expression{
        tag: "ArrayAccess",
        data: 0 fr fr Would point to ArrayAccessExpression in full implementation
    }
    damn expr
}

slay createArrayExpression(elements []Expression) Expression {
    sus expr Expression = Expression{
        tag: "Array",
        data: 0 fr fr Would point to ArrayExpression in full implementation
    }
    damn expr
}

slay createMapExpression(pairs []MapPair) Expression {
    sus expr Expression = Expression{
        tag: "Map",
        data: 0 fr fr Would point to MapExpression in full implementation
    }
    damn expr
}

slay createStructLiteralExpression(struct_type Type, fields []FieldInitializer) Expression {
    sus expr Expression = Expression{
        tag: "StructLiteral",
        data: 0 fr fr Would point to StructLiteralExpression in full implementation
    }
    damn expr
}

slay createTupleExpression(elements []Expression) Expression {
    sus expr Expression = Expression{
        tag: "Tuple",
        data: 0 fr fr Would point to TupleExpression in full implementation
    }
    damn expr
}

slay createMatchExpression(value Expression, cases []MatchCase) Expression {
    sus expr Expression = Expression{
        tag: "Match",
        data: 0 fr fr Would point to MatchExpression in full implementation
    }
    damn expr
}

slay createMatchCase(pattern Expression, body Expression) MatchCase {
    damn MatchCase{
        pattern: pattern,
        guard: createNullExpression(),
        body: body
    }
}

slay createBooleanExpression(value lit) Expression {
    sus expr Expression = Expression{
        tag: "Boolean",
        data: 0 fr fr Would contain boolean value in full implementation
    }
    damn expr
}

slay createNumberExpression(value tea) Expression {
    sus expr Expression = Expression{
        tag: "Number",
        data: 0 fr fr Would contain number value in full implementation
    }
    damn expr
}

slay createStringExpression(value tea) Expression {
    sus expr Expression = Expression{
        tag: "String",
        data: 0 fr fr Would contain string value in full implementation
    }
    damn expr
}

slay createIdentifierExpression(name tea) Expression {
    sus expr Expression = Expression{
        tag: "Identifier",
        data: 0 fr fr Would contain identifier name in full implementation
    }
    damn expr
}

slay createNullExpression() Expression {
    sus expr Expression = Expression{
        tag: "Null",
        data: 0
    }
    damn expr
}

fr fr Type constructors
slay createPrimitiveType(name tea) Type {
    sus type_obj Type = Type{
        tag: "Primitive",
        data: 0 fr fr Would point to PrimitiveType in full implementation
    }
    damn type_obj
}

slay createArrayType(element_type Type, size normie) Type {
    sus type_obj Type = Type{
        tag: "Array",
        data: 0 fr fr Would point to ArrayType in full implementation
    }
    damn type_obj
}

slay createMapType(key_type Type, value_type Type) Type {
    sus type_obj Type = Type{
        tag: "Map",
        data: 0 fr fr Would point to MapType in full implementation
    }
    damn type_obj
}

slay createChannelType(element_type Type) Type {
    sus type_obj Type = Type{
        tag: "Channel",
        data: 0 fr fr Would point to ChannelType in full implementation
    }
    damn type_obj
}

slay createFunctionType(parameters []Type, return_type Type) Type {
    sus type_obj Type = Type{
        tag: "Function",
        data: 0 fr fr Would point to FunctionType in full implementation
    }
    damn type_obj
}

slay createTupleType(elements []Type) Type {
    sus type_obj Type = Type{
        tag: "Tuple",
        data: 0 fr fr Would point to TupleType in full implementation
    }
    damn type_obj
}

slay createIdentifierType(name tea) Type {
    sus type_obj Type = Type{
        tag: "Identifier",
        data: 0 fr fr Would point to IdentifierType in full implementation
    }
    damn type_obj
}

slay createVoidType() Type {
    sus type_obj Type = Type{
        tag: "Void",
        data: 0
    }
    damn type_obj
}

fr fr Utility functions for AST manipulation
slay isExpressionStatement(stmt Statement) lit {
    damn stmt.tag == "Expression"
}

slay isFunctionStatement(stmt Statement) lit {
    damn stmt.tag == "Function"
}

slay isLetStatement(stmt Statement) lit {
    damn stmt.tag == "Let"
}

slay isReturnStatement(stmt Statement) lit {
    damn stmt.tag == "Return"
}

slay isBinaryExpression(expr Expression) lit {
    damn expr.tag == "Binary"
}

slay isCallExpression(expr Expression) lit {
    damn expr.tag == "Call"
}

slay isIdentifierExpression(expr Expression) lit {
    damn expr.tag == "Identifier"
}

slay isLiteralExpression(expr Expression) lit {
    damn expr.tag == "String" or expr.tag == "Number" or expr.tag == "Boolean"
}

slay isPrimitiveType(type_obj Type) lit {
    damn type_obj.tag == "Primitive"
}

slay isArrayType(type_obj Type) lit {
    damn type_obj.tag == "Array"
}

slay isMapType(type_obj Type) lit {
    damn type_obj.tag == "Map"
}

slay isChannelType(type_obj Type) lit {
    damn type_obj.tag == "Channel"
}

slay isFunctionType(type_obj Type) lit {
    damn type_obj.tag == "Function"
}

fr fr AST printing functions for debugging
slay printProgram(program Program) {
    vibez.spill("Program:")
    vibez.spill("  Package: " + program.package.name)
    
    vibez.spill("  Imports:")
    bestie i := 0; i < program.imports.length; i = i + 1 {
        vibez.spill("    " + program.imports[i].path)
    }
    
    vibez.spill("  Statements:")
    bestie i := 0; i < program.statements.length; i = i + 1 {
        printStatement(program.statements[i], "    ")
    }
}

slay printStatement(stmt Statement, indent tea) {
    vibez.spill(indent + "Statement: " + stmt.tag)
}

slay printExpression(expr Expression, indent tea) {
    vibez.spill(indent + "Expression: " + expr.tag)
}

slay printType(type_obj Type, indent tea) {
    vibez.spill(indent + "Type: " + type_obj.tag)
}

fr fr AST validation functions
slay validateProgram(program Program) lit {
    fr fr Basic validation - check all statements are valid
    bestie i := 0; i < program.statements.length; i = i + 1 {
        if !validateStatement(program.statements[i]) {
            damn cringe
        }
    }
    damn based
}

slay validateStatement(stmt Statement) lit {
    fr fr Basic statement validation
    damn stmt.tag != ""
}

slay validateExpression(expr Expression) lit {
    fr fr Basic expression validation
    damn expr.tag != ""
}

slay validateType(type_obj Type) lit {
    fr fr Basic type validation
    damn type_obj.tag != ""
}

fr fr AST transformation functions
slay cloneExpression(expr Expression) Expression {
    fr fr Simple clone - in full implementation would deep copy
    damn Expression{
        tag: expr.tag,
        data: expr.data
    }
}

slay cloneStatement(stmt Statement) Statement {
    fr fr Simple clone - in full implementation would deep copy
    damn Statement{
        tag: stmt.tag,
        data: stmt.data
    }
}

slay cloneType(type_obj Type) Type {
    fr fr Simple clone - in full implementation would deep copy
    damn Type{
        tag: type_obj.tag,
        data: type_obj.data
    }
}

fr fr Test functions for AST functionality
slay test_createBasicNodes() {
    test_start("Create Basic AST Nodes")
    
    sus program Program = createProgram()
    assert_true(program.statements.length == 0)
    
    sus expr Expression = createNumberExpression("42")
    assert_true(expr.tag == "Number")
    
    sus stmt Statement = createExpressionStatement(expr)
    assert_true(stmt.tag == "Expression")
    
    sus type_obj Type = createPrimitiveType("normie")
    assert_true(type_obj.tag == "Primitive")
    
    test_passed()
}

slay test_createFunction() {
    test_start("Create Function AST Node")
    
    sus func FunctionStatement = createFunctionStatement("test")
    assert_true(func.name == "test")
    assert_true(func.parameters.length == 0)
    assert_true(func.body.length == 0)
    
    test_passed()
}

slay test_createExpressions() {
    test_start("Create Expression AST Nodes")
    
    sus left Expression = createNumberExpression("42")
    sus right Expression = createNumberExpression("24")
    sus binary Expression = createBinaryExpression(left, "+", right)
    
    assert_true(binary.tag == "Binary")
    
    sus call Expression = createCallExpression(createIdentifierExpression("print"), [left])
    assert_true(call.tag == "Call")
    
    test_passed()
}

slay test_validateAST() {
    test_start("Validate AST Nodes")
    
    sus program Program = createProgram()
    assert_true(validateProgram(program))
    
    sus expr Expression = createNumberExpression("42")
    assert_true(validateExpression(expr))
    
    sus type_obj Type = createPrimitiveType("normie")
    assert_true(validateType(type_obj))
    
    test_passed()
}

slay test_cloneNodes() {
    test_start("Clone AST Nodes")
    
    sus original Expression = createStringExpression("hello")
    sus cloned Expression = cloneExpression(original)
    
    assert_true(cloned.tag == original.tag)
    assert_true(cloned.tag == "String")
    
    test_passed()
}

slay runASTTests() {
    test_createBasicNodes()
    test_createFunction()
    test_createExpressions()
    test_validateAST()
    test_cloneNodes()
    print_test_summary()
}

fr fr Entry point for testing
slay main() {
    runASTTests()
}
