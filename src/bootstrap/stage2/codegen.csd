// Stage 2 Code Generator - Written in CURSED minimal subset
// Generates LLVM IR and compiles to executable

import "std/io"
import "std/os"
import "std/string"

// CodeGenerator generates LLVM IR and executable code
struct CodeGenerator {
    module_name: string
    output_path: string
    llvm_ir: string
    errors: []string
}

// Create new code generator
func new_codegen() CodeGenerator {
    return CodeGenerator{
        module_name: "main",
        output_path: "",
        llvm_ir: "",
        errors: []string{},
    }
}

// Generate code from AST
func (c *CodeGenerator) generate(program: *Program, output_file: string) bool {
    c.output_path = output_file
    c.llvm_ir = ""
    c.errors = []string{}
    
    // Generate LLVM IR header
    c.emit_header()
    
    // Generate code for each statement
    for stmt in program.statements {
        c.generate_statement(stmt)
    }
    
    // Generate main function wrapper if needed
    c.emit_main_wrapper()
    
    if len(c.errors) > 0 {
        return false
    }
    
    // Write LLVM IR to file
    ir_file := output_file + ".ll"
    success := io.write_file(ir_file, c.llvm_ir)
    if !success {
        c.add_error("Failed to write LLVM IR file: " + ir_file)
        return false
    }
    
    // Compile LLVM IR to executable
    return c.compile_to_executable(ir_file, output_file)
}

// Emit LLVM IR header
func (c *CodeGenerator) emit_header() {
    c.llvm_ir = c.llvm_ir + "; ModuleID = '" + c.module_name + "'\n"
    c.llvm_ir = c.llvm_ir + "target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n"
    c.llvm_ir = c.llvm_ir + "target triple = \"x86_64-unknown-linux-gnu\"\n\n"
    
    // Declare external functions
    c.llvm_ir = c.llvm_ir + "declare i32 @printf(i8*, ...)\n"
    c.llvm_ir = c.llvm_ir + "declare i8* @malloc(i64)\n"
    c.llvm_ir = c.llvm_ir + "declare void @free(i8*)\n"
    c.llvm_ir = c.llvm_ir + "declare i32 @exit(i32)\n\n"
}

// Generate statement code
func (c *CodeGenerator) generate_statement(stmt: Statement) {
    if import_stmt, ok := stmt.(*ImportStatement); ok {
        c.generate_import(import_stmt)
    } else if func_stmt, ok := stmt.(*FunctionStatement); ok {
        c.generate_function(func_stmt)
    } else if let_stmt, ok := stmt.(*LetStatement); ok {
        c.generate_let(let_stmt)
    } else if return_stmt, ok := stmt.(*ReturnStatement); ok {
        c.generate_return(return_stmt)
    } else if expr_stmt, ok := stmt.(*ExpressionStatement); ok {
        c.generate_expression(expr_stmt.expression)
    } else if if_stmt, ok := stmt.(*IfStatement); ok {
        c.generate_if(if_stmt)
    } else if for_stmt, ok := stmt.(*ForStatement); ok {
        c.generate_for(for_stmt)
    } else if struct_stmt, ok := stmt.(*StructStatement); ok {
        c.generate_struct(struct_stmt)
    } else if block_stmt, ok := stmt.(*BlockStatement); ok {
        c.generate_block(block_stmt)
    } else {
        c.add_error("Unknown statement type")
    }
}

// Generate import statement
func (c *CodeGenerator) generate_import(stmt: *ImportStatement) {
    // For stage 2, imports are mostly handled at link time
    // Add comment for now
    c.llvm_ir = c.llvm_ir + "; import \"" + stmt.path + "\"\n"
}

// Generate function statement
func (c *CodeGenerator) generate_function(stmt: *FunctionStatement) {
    // Determine return type
    ret_type := "void"
    if stmt.return_type != "" {
        ret_type = c.llvm_type(stmt.return_type)
    }
    
    // Build parameter list
    params := ""
    for i, param in stmt.parameters {
        if i > 0 {
            params = params + ", "
        }
        params = params + c.llvm_type(param.type) + " %" + param.name
    }
    
    // Function header
    c.llvm_ir = c.llvm_ir + "define " + ret_type + " @" + stmt.name + "(" + params + ") {\n"
    c.llvm_ir = c.llvm_ir + "entry:\n"
    
    // Generate function body
    c.generate_block(&stmt.body)
    
    // Ensure function ends with return
    if ret_type == "void" {
        c.llvm_ir = c.llvm_ir + "  ret void\n"
    }
    
    c.llvm_ir = c.llvm_ir + "}\n\n"
}

// Generate let statement (variable declaration)
func (c *CodeGenerator) generate_let(stmt: *LetStatement) {
    if stmt.value != nil {
        // Allocate stack space
        llvm_type := c.llvm_type(stmt.type)
        c.llvm_ir = c.llvm_ir + "  %" + stmt.name + " = alloca " + llvm_type + "\n"
        
        // Generate initializer expression
        value_reg := c.generate_expression(stmt.value)
        
        // Store value
        c.llvm_ir = c.llvm_ir + "  store " + llvm_type + " " + value_reg + ", " + llvm_type + "* %" + stmt.name + "\n"
    } else {
        // Just allocate uninitialized
        llvm_type := c.llvm_type(stmt.type)
        c.llvm_ir = c.llvm_ir + "  %" + stmt.name + " = alloca " + llvm_type + "\n"
    }
}

// Generate return statement
func (c *CodeGenerator) generate_return(stmt: *ReturnStatement) {
    if stmt.value != nil {
        value_reg := c.generate_expression(stmt.value)
        c.llvm_ir = c.llvm_ir + "  ret i32 " + value_reg + "\n"
    } else {
        c.llvm_ir = c.llvm_ir + "  ret void\n"
    }
}

// Generate if statement
func (c *CodeGenerator) generate_if(stmt: *IfStatement) {
    // Generate condition
    cond_reg := c.generate_expression(stmt.condition)
    
    // Create basic blocks
    then_label := c.new_label("if.then")
    else_label := c.new_label("if.else")
    end_label := c.new_label("if.end")
    
    // Branch based on condition
    c.llvm_ir = c.llvm_ir + "  br i1 " + cond_reg + ", label %" + then_label + ", label %"
    if stmt.alternative.statements != nil {
        c.llvm_ir = c.llvm_ir + else_label + "\n"
    } else {
        c.llvm_ir = c.llvm_ir + end_label + "\n"
    }
    
    // Then block
    c.llvm_ir = c.llvm_ir + then_label + ":\n"
    c.generate_block(&stmt.consequence)
    c.llvm_ir = c.llvm_ir + "  br label %" + end_label + "\n"
    
    // Else block (if present)
    if stmt.alternative.statements != nil {
        c.llvm_ir = c.llvm_ir + else_label + ":\n"
        c.generate_block(&stmt.alternative)
        c.llvm_ir = c.llvm_ir + "  br label %" + end_label + "\n"
    }
    
    // End block
    c.llvm_ir = c.llvm_ir + end_label + ":\n"
}

// Generate for statement
func (c *CodeGenerator) generate_for(stmt: *ForStatement) {
    // Generate init statement
    if stmt.init != nil {
        c.generate_statement(stmt.init)
    }
    
    // Create basic blocks
    cond_label := c.new_label("for.cond")
    body_label := c.new_label("for.body")
    update_label := c.new_label("for.update")
    end_label := c.new_label("for.end")
    
    // Jump to condition
    c.llvm_ir = c.llvm_ir + "  br label %" + cond_label + "\n"
    
    // Condition block
    c.llvm_ir = c.llvm_ir + cond_label + ":\n"
    if stmt.condition != nil {
        cond_reg := c.generate_expression(stmt.condition)
        c.llvm_ir = c.llvm_ir + "  br i1 " + cond_reg + ", label %" + body_label + ", label %" + end_label + "\n"
    } else {
        c.llvm_ir = c.llvm_ir + "  br label %" + body_label + "\n"
    }
    
    // Body block
    c.llvm_ir = c.llvm_ir + body_label + ":\n"
    c.generate_block(&stmt.body)
    c.llvm_ir = c.llvm_ir + "  br label %" + update_label + "\n"
    
    // Update block
    c.llvm_ir = c.llvm_ir + update_label + ":\n"
    if stmt.update != nil {
        c.generate_statement(stmt.update)
    }
    c.llvm_ir = c.llvm_ir + "  br label %" + cond_label + "\n"
    
    // End block
    c.llvm_ir = c.llvm_ir + end_label + ":\n"
}

// Generate struct statement
func (c *CodeGenerator) generate_struct(stmt: *StructStatement) {
    // Generate struct type definition
    field_types := ""
    for i, field in stmt.fields {
        if i > 0 {
            field_types = field_types + ", "
        }
        field_types = field_types + c.llvm_type(field.type)
    }
    
    c.llvm_ir = c.llvm_ir + "%" + stmt.name + " = type { " + field_types + " }\n"
}

// Generate block statement
func (c *CodeGenerator) generate_block(stmt: *BlockStatement) {
    for s in stmt.statements {
        c.generate_statement(s)
    }
}

// Generate expression and return register name
func (c *CodeGenerator) generate_expression(expr: Expression) string {
    if ident, ok := expr.(*Identifier); ok {
        return c.generate_identifier(ident)
    } else if int_lit, ok := expr.(*IntegerLiteral); ok {
        return c.generate_integer_literal(int_lit)
    } else if str_lit, ok := expr.(*StringLiteral); ok {
        return c.generate_string_literal(str_lit)
    } else if bool_lit, ok := expr.(*BooleanLiteral); ok {
        return c.generate_boolean_literal(bool_lit)
    } else if prefix_expr, ok := expr.(*PrefixExpression); ok {
        return c.generate_prefix_expression(prefix_expr)
    } else if infix_expr, ok := expr.(*InfixExpression); ok {
        return c.generate_infix_expression(infix_expr)
    } else if call_expr, ok := expr.(*CallExpression); ok {
        return c.generate_call_expression(call_expr)
    } else if array_lit, ok := expr.(*ArrayLiteral); ok {
        return c.generate_array_literal(array_lit)
    } else if index_expr, ok := expr.(*IndexExpression); ok {
        return c.generate_index_expression(index_expr)
    } else if assign_expr, ok := expr.(*AssignmentExpression); ok {
        return c.generate_assignment_expression(assign_expr)
    } else {
        c.add_error("Unknown expression type")
        return "%error"
    }
}

// Generate identifier
func (c *CodeGenerator) generate_identifier(expr: *Identifier) string {
    // Load value from variable
    reg := c.new_register()
    c.llvm_ir = c.llvm_ir + "  " + reg + " = load i32, i32* %" + expr.value + "\n"
    return reg
}

// Generate integer literal
func (c *CodeGenerator) generate_integer_literal(expr: *IntegerLiteral) string {
    return string(expr.value)
}

// Generate string literal
func (c *CodeGenerator) generate_string_literal(expr: *StringLiteral) string {
    // For now, return simple string representation
    // In full implementation, would create global string constants
    reg := c.new_register()
    c.llvm_ir = c.llvm_ir + "  " + reg + " = alloca [" + string(len(expr.value) + 1) + " x i8]\n"
    return reg
}

// Generate boolean literal
func (c *CodeGenerator) generate_boolean_literal(expr: *BooleanLiteral) string {
    if expr.value {
        return "1"
    }
    return "0"
}

// Generate prefix expression
func (c *CodeGenerator) generate_prefix_expression(expr: *PrefixExpression) string {
    right_reg := c.generate_expression(expr.right)
    reg := c.new_register()
    
    if expr.operator == "-" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = sub i32 0, " + right_reg + "\n"
    } else if expr.operator == "!" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = icmp eq i32 " + right_reg + ", 0\n"
    }
    
    return reg
}

// Generate infix expression
func (c *CodeGenerator) generate_infix_expression(expr: *InfixExpression) string {
    left_reg := c.generate_expression(expr.left)
    right_reg := c.generate_expression(expr.right)
    reg := c.new_register()
    
    if expr.operator == "+" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = add i32 " + left_reg + ", " + right_reg + "\n"
    } else if expr.operator == "-" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = sub i32 " + left_reg + ", " + right_reg + "\n"
    } else if expr.operator == "*" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = mul i32 " + left_reg + ", " + right_reg + "\n"
    } else if expr.operator == "/" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = sdiv i32 " + left_reg + ", " + right_reg + "\n"
    } else if expr.operator == "==" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = icmp eq i32 " + left_reg + ", " + right_reg + "\n"
    } else if expr.operator == "!=" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = icmp ne i32 " + left_reg + ", " + right_reg + "\n"
    } else if expr.operator == "<" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = icmp slt i32 " + left_reg + ", " + right_reg + "\n"
    } else if expr.operator == ">" {
        c.llvm_ir = c.llvm_ir + "  " + reg + " = icmp sgt i32 " + left_reg + ", " + right_reg + "\n"
    } else {
        c.add_error("Unknown operator: " + expr.operator)
    }
    
    return reg
}

// Generate call expression
func (c *CodeGenerator) generate_call_expression(expr: *CallExpression) string {
    // Generate function name
    func_name := ""
    if ident, ok := expr.function.(*Identifier); ok {
        func_name = ident.value
    } else {
        c.add_error("Invalid function call")
        return "%error"
    }
    
    // Generate arguments
    args := ""
    for i, arg in expr.arguments {
        if i > 0 {
            args = args + ", "
        }
        arg_reg := c.generate_expression(arg)
        args = args + "i32 " + arg_reg
    }
    
    // Generate call
    reg := c.new_register()
    c.llvm_ir = c.llvm_ir + "  " + reg + " = call i32 @" + func_name + "(" + args + ")\n"
    
    return reg
}

// Generate array literal
func (c *CodeGenerator) generate_array_literal(expr: *ArrayLiteral) string {
    // Simplified array implementation
    reg := c.new_register()
    size := len(expr.elements)
    c.llvm_ir = c.llvm_ir + "  " + reg + " = alloca [" + string(size) + " x i32]\n"
    
    // Initialize elements
    for i, elem in expr.elements {
        elem_reg := c.generate_expression(elem)
        ptr_reg := c.new_register()
        c.llvm_ir = c.llvm_ir + "  " + ptr_reg + " = getelementptr [" + string(size) + " x i32], [" + string(size) + " x i32]* " + reg + ", i32 0, i32 " + string(i) + "\n"
        c.llvm_ir = c.llvm_ir + "  store i32 " + elem_reg + ", i32* " + ptr_reg + "\n"
    }
    
    return reg
}

// Generate index expression
func (c *CodeGenerator) generate_index_expression(expr: *IndexExpression) string {
    array_reg := c.generate_expression(expr.left)
    index_reg := c.generate_expression(expr.index)
    
    ptr_reg := c.new_register()
    val_reg := c.new_register()
    
    c.llvm_ir = c.llvm_ir + "  " + ptr_reg + " = getelementptr i32, i32* " + array_reg + ", i32 " + index_reg + "\n"
    c.llvm_ir = c.llvm_ir + "  " + val_reg + " = load i32, i32* " + ptr_reg + "\n"
    
    return val_reg
}

// Generate assignment expression
func (c *CodeGenerator) generate_assignment_expression(expr: *AssignmentExpression) string {
    value_reg := c.generate_expression(expr.value)
    c.llvm_ir = c.llvm_ir + "  store i32 " + value_reg + ", i32* %" + expr.name.value + "\n"
    return value_reg
}

// Convert CURSED type to LLVM type
func (c *CodeGenerator) llvm_type(cursed_type: string) string {
    if cursed_type == "int" {
        return "i32"
    } else if cursed_type == "bool" {
        return "i1"
    } else if cursed_type == "string" {
        return "i8*"
    } else if cursed_type == "" {
        return "void"
    } else {
        return "%" + cursed_type  // User-defined type
    }
}

// Generate main wrapper if no main function exists
func (c *CodeGenerator) emit_main_wrapper() {
    // Check if main function already exists
    if !string.contains(c.llvm_ir, "@main(") {
        c.llvm_ir = c.llvm_ir + "define i32 @main() {\n"
        c.llvm_ir = c.llvm_ir + "entry:\n"
        c.llvm_ir = c.llvm_ir + "  ret i32 0\n"
        c.llvm_ir = c.llvm_ir + "}\n"
    }
}

// Compile LLVM IR to executable
func (c *CodeGenerator) compile_to_executable(ir_file: string, output_file: string) bool {
    // Use clang to compile LLVM IR to executable
    cmd := "clang -o " + output_file + " " + ir_file
    exit_code := os.system(cmd)
    
    if exit_code != 0 {
        c.add_error("Failed to compile LLVM IR to executable")
        return false
    }
    
    return true
}

// Generate new register name
func (c *CodeGenerator) new_register() string {
    static register_counter := 0
    register_counter = register_counter + 1
    return "%r" + string(register_counter)
}

// Generate new label name
func (c *CodeGenerator) new_label(prefix: string) string {
    static label_counter := 0
    label_counter = label_counter + 1
    return prefix + "." + string(label_counter)
}

// Add error
func (c *CodeGenerator) add_error(msg: string) {
    c.errors = append(c.errors, msg)
}

// Check if codegen has errors
func (c *CodeGenerator) has_errors() bool {
    return len(c.errors) > 0
}

// Get codegen errors
func (c *CodeGenerator) get_errors() []string {
    return c.errors
}
