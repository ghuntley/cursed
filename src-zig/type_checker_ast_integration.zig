// P0 Sprint 1: AST Integration for Simple Type Checker
// Provides integration between SimpleTypeChecker and CURSED AST nodes

const std = @import("std");
const ast = @import("ast.zig");
const type_checker_simple = @import("type_checker_simple.zig");

const SimpleTypeChecker = type_checker_simple.SimpleTypeChecker;
const TypeInfo = type_checker_simple.TypeInfo;
const SimpleType = type_checker_simple.SimpleType;

// AST type checking visitor
pub const ASTTypeChecker = struct {
    type_checker: *SimpleTypeChecker,
    
    pub fn init(type_checker: *SimpleTypeChecker) ASTTypeChecker {
        return ASTTypeChecker{
            .type_checker = type_checker,
        };
    }
    
    // Check an entire program
    pub fn checkProgram(self: *ASTTypeChecker, program: *ast.Program) !void {
        for (program.statements.items) |statement| {
            _ = try self.checkStatement(statement);
        }
    }
    
    // Check a statement and return its type (if any)
    pub fn checkStatement(self: *ASTTypeChecker, statement: *ast.Statement) !?*TypeInfo {
        return switch (statement.*) {
            .VariableDeclaration => |*var_decl| try self.checkVariableDeclaration(var_decl),
            .Assignment => |*assignment| try self.checkAssignment(assignment),
            .ExpressionStatement => |*expr_stmt| try self.checkExpressionStatement(expr_stmt),
            .FunctionDeclaration => |*func_decl| try self.checkFunctionDeclaration(func_decl),
            .StructDeclaration => |*struct_decl| try self.checkStructDeclaration(struct_decl),
            .IfStatement => |*if_stmt| try self.checkIfStatement(if_stmt),
            .WhileLoop => |*while_loop| try self.checkWhileLoop(while_loop),
            .ForLoop => |*for_loop| try self.checkForLoop(for_loop),
            .ReturnStatement => |*ret_stmt| try self.checkReturnStatement(ret_stmt),
            .BlockStatement => |*block| try self.checkBlockStatement(block),
            else => null, // Other statement types not implemented yet
        };
    }
    
    // Check variable declaration
    fn checkVariableDeclaration(self: *ASTTypeChecker, var_decl: *ast.VariableDeclaration) !?*TypeInfo {
        const type_name = var_decl.type_annotation orelse "unknown";
        const is_mutable = var_decl.is_mutable;
        
        const var_type = try self.type_checker.checkVariableDeclaration(var_decl.name, type_name, is_mutable);
        
        // Check initializer if present
        if (var_decl.initializer) |init_expr| {
            const init_type = try self.checkExpression(init_expr);
            
            // Check that initializer type is compatible with declared type
            if (!self.type_checker.areTypesCompatible(init_type, var_type)) {
                try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                    self.type_checker.allocator,
                    "Cannot initialize variable '{s}' of type '{s}' with value of type '{s}'",
                    .{ var_decl.name, var_type.name, init_type.name }
                ));
            } else {
                try self.type_checker.symbol_table.setVariableInitialized(var_decl.name);
            }
        }
        
        return var_type;
    }
    
    // Check assignment statement
    fn checkAssignment(self: *ASTTypeChecker, assignment: *ast.Assignment) !?*TypeInfo {
        const value_type = try self.checkExpression(assignment.value);
        
        switch (assignment.target.*) {
            .Identifier => |ident| {
                try self.type_checker.checkAssignment(ident.name, value_type);
            },
            .FieldAccess => |field_access| {
                // Check field assignment
                const object_type = try self.checkExpression(field_access.object);
                _ = try self.type_checker.checkFieldAccess(object_type, field_access.field);
                // TODO: Check that field assignment is type compatible
            },
            .ArrayAccess => |array_access| {
                // Check array element assignment
                const array_type = try self.checkExpression(array_access.array);
                const index_type = try self.checkExpression(array_access.index);
                const element_type = try self.type_checker.checkArrayAccess(array_type, index_type);
                
                if (!self.type_checker.areTypesCompatible(value_type, element_type)) {
                    try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                        self.type_checker.allocator,
                        "Cannot assign value of type '{s}' to array element of type '{s}'",
                        .{ value_type.name, element_type.name }
                    ));
                }
            },
            else => {
                try self.type_checker.addError(.InvalidOperation, 
                    try self.type_checker.allocator.dupe(u8, "Invalid assignment target"));
            },
        }
        
        return value_type;
    }
    
    // Check expression statement
    fn checkExpressionStatement(self: *ASTTypeChecker, expr_stmt: *ast.ExpressionStatement) !?*TypeInfo {
        return try self.checkExpression(expr_stmt.expression);
    }
    
    // Check function declaration
    fn checkFunctionDeclaration(self: *ASTTypeChecker, func_decl: *ast.FunctionDeclaration) !?*TypeInfo {
        // Enter function scope
        try self.type_checker.enterScope();
        defer self.type_checker.exitScope();
        
        // Add parameters to scope
        for (func_decl.parameters.items) |param| {
            const param_type_name = param.type_annotation orelse "unknown";
            _ = try self.type_checker.checkVariableDeclaration(param.name, param_type_name, false);
        }
        
        // Check function body
        if (func_decl.body) |body| {
            _ = try self.checkStatement(body);
        }
        
        // TODO: Create and register function type with signature
        return null;
    }
    
    // Check struct declaration
    fn checkStructDeclaration(self: *ASTTypeChecker, struct_decl: *ast.StructDeclaration) !?*TypeInfo {
        var fields = std.ArrayList(struct { name: []const u8, type_name: []const u8 }).init(self.type_checker.allocator);
        defer fields.deinit();
        
        for (struct_decl.fields.items) |field| {
            const field_type_name = field.type_annotation orelse "unknown";
            try fields.append(.{ .name = field.name, .type_name = field_type_name });
        }
        
        try self.type_checker.registerStructType(struct_decl.name, fields.items);
        return self.type_checker.getTypeByName(struct_decl.name);
    }
    
    // Check if statement
    fn checkIfStatement(self: *ASTTypeChecker, if_stmt: *ast.IfStatement) !?*TypeInfo {
        const condition_type = try self.checkExpression(if_stmt.condition);
        
        // Check that condition is boolean
        if (!condition_type.base_type.isBoolean()) {
            try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                self.type_checker.allocator,
                "If condition must be boolean, got '{s}'",
                .{condition_type.name}
            ));
        }
        
        // Check then branch
        try self.type_checker.enterScope();
        defer self.type_checker.exitScope();
        _ = try self.checkStatement(if_stmt.then_branch);
        
        // Check else branch if present
        if (if_stmt.else_branch) |else_branch| {
            try self.type_checker.enterScope();
            defer self.type_checker.exitScope();
            _ = try self.checkStatement(else_branch);
        }
        
        return null;
    }
    
    // Check while loop
    fn checkWhileLoop(self: *ASTTypeChecker, while_loop: *ast.WhileLoop) !?*TypeInfo {
        const condition_type = try self.checkExpression(while_loop.condition);
        
        // Check that condition is boolean
        if (!condition_type.base_type.isBoolean()) {
            try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                self.type_checker.allocator,
                "While condition must be boolean, got '{s}'",
                .{condition_type.name}
            ));
        }
        
        // Check loop body
        try self.type_checker.enterScope();
        defer self.type_checker.exitScope();
        _ = try self.checkStatement(while_loop.body);
        
        return null;
    }
    
    // Check for loop
    fn checkForLoop(self: *ASTTypeChecker, for_loop: *ast.ForLoop) !?*TypeInfo {
        try self.type_checker.enterScope();
        defer self.type_checker.exitScope();
        
        // Check initializer
        if (for_loop.initializer) |init| {
            _ = try self.checkStatement(init);
        }
        
        // Check condition
        if (for_loop.condition) |condition| {
            const condition_type = try self.checkExpression(condition);
            if (!condition_type.base_type.isBoolean()) {
                try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                    self.type_checker.allocator,
                    "For loop condition must be boolean, got '{s}'",
                    .{condition_type.name}
                ));
            }
        }
        
        // Check increment
        if (for_loop.increment) |increment| {
            _ = try self.checkExpression(increment);
        }
        
        // Check body
        _ = try self.checkStatement(for_loop.body);
        
        return null;
    }
    
    // Check return statement
    fn checkReturnStatement(self: *ASTTypeChecker, ret_stmt: *ast.ReturnStatement) !?*TypeInfo {
        if (ret_stmt.value) |value| {
            return try self.checkExpression(value);
        }
        return self.type_checker.getTypeByName("cap"); // void
    }
    
    // Check block statement
    fn checkBlockStatement(self: *ASTTypeChecker, block: *ast.BlockStatement) !?*TypeInfo {
        try self.type_checker.enterScope();
        defer self.type_checker.exitScope();
        
        var last_type: ?*TypeInfo = null;
        for (block.statements.items) |statement| {
            last_type = try self.checkStatement(statement);
        }
        
        return last_type;
    }
    
    // Check expression and return its type
    pub fn checkExpression(self: *ASTTypeChecker, expression: *ast.Expression) !*TypeInfo {
        return switch (expression.*) {
            .Literal => |*literal| self.checkLiteral(literal),
            .Identifier => |*identifier| try self.checkIdentifier(identifier),
            .BinaryOperation => |*binary_op| try self.checkBinaryOperation(binary_op),
            .UnaryOperation => |*unary_op| try self.checkUnaryOperation(unary_op),
            .FunctionCall => |*func_call| try self.checkFunctionCall(func_call),
            .FieldAccess => |*field_access| try self.checkFieldAccess(field_access),
            .ArrayAccess => |*array_access| try self.checkArrayAccess(array_access),
            .ArrayLiteral => |*array_literal| try self.checkArrayLiteral(array_literal),
            .StructLiteral => |*struct_literal| try self.checkStructLiteral(struct_literal),
            else => self.type_checker.getTypeByName("unknown").?,
        };
    }
    
    // Check literal expression
    fn checkLiteral(self: *ASTTypeChecker, literal: *ast.Literal) *TypeInfo {
        return switch (literal.*) {
            .Integer => self.type_checker.getTypeByName("drip").?,
            .Float => self.type_checker.getTypeByName("vibes").?,
            .String => self.type_checker.getTypeByName("tea").?,
            .Boolean => self.type_checker.getTypeByName("lit").?,
            .Character => self.type_checker.getTypeByName("sip").?,
            else => self.type_checker.getTypeByName("unknown").?,
        };
    }
    
    // Check identifier expression
    fn checkIdentifier(self: *ASTTypeChecker, identifier: *ast.Identifier) !*TypeInfo {
        return self.type_checker.checkVariableAccess(identifier.name);
    }
    
    // Check binary operation
    fn checkBinaryOperation(self: *ASTTypeChecker, binary_op: *ast.BinaryOperation) !*TypeInfo {
        const left_type = try self.checkExpression(binary_op.left);
        const right_type = try self.checkExpression(binary_op.right);
        
        return self.type_checker.checkBinaryOperation(left_type, right_type, binary_op.operator);
    }
    
    // Check unary operation
    fn checkUnaryOperation(self: *ASTTypeChecker, unary_op: *ast.UnaryOperation) !*TypeInfo {
        const operand_type = try self.checkExpression(unary_op.operand);
        
        return switch (unary_op.operator[0]) {
            '-' => blk: {
                if (!operand_type.base_type.isNumeric()) {
                    try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                        self.type_checker.allocator,
                        "Cannot apply unary minus to type '{s}'",
                        .{operand_type.name}
                    ));
                }
                break :blk operand_type;
            },
            '!' => blk: {
                if (!operand_type.base_type.isBoolean()) {
                    try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                        self.type_checker.allocator,
                        "Cannot apply logical not to type '{s}'",
                        .{operand_type.name}
                    ));
                }
                break :blk self.type_checker.getTypeByName("lit").?;
            },
            else => operand_type,
        };
    }
    
    // Check function call
    fn checkFunctionCall(self: *ASTTypeChecker, func_call: *ast.FunctionCall) !*TypeInfo {
        var arg_types = std.ArrayList(*TypeInfo).init(self.type_checker.allocator);
        defer arg_types.deinit();
        
        for (func_call.arguments.items) |arg| {
            const arg_type = try self.checkExpression(arg);
            try arg_types.append(arg_type);
        }
        
        return self.type_checker.checkFunctionCall(func_call.function_name, arg_types.items);
    }
    
    // Check field access
    fn checkFieldAccess(self: *ASTTypeChecker, field_access: *ast.FieldAccess) !*TypeInfo {
        const object_type = try self.checkExpression(field_access.object);
        return self.type_checker.checkFieldAccess(object_type, field_access.field);
    }
    
    // Check array access
    fn checkArrayAccess(self: *ASTTypeChecker, array_access: *ast.ArrayAccess) !*TypeInfo {
        const array_type = try self.checkExpression(array_access.array);
        const index_type = try self.checkExpression(array_access.index);
        return self.type_checker.checkArrayAccess(array_type, index_type);
    }
    
    // Check array literal
    fn checkArrayLiteral(self: *ASTTypeChecker, array_literal: *ast.ArrayLiteral) !*TypeInfo {
        if (array_literal.elements.items.len == 0) {
            // Empty array - return generic array type
            return self.type_checker.getTypeByName("unknown").?;
        }
        
        // Check all elements have the same type
        const first_element_type = try self.checkExpression(array_literal.elements.items[0]);
        
        for (array_literal.elements.items[1..]) |element| {
            const element_type = try self.checkExpression(element);
            if (!self.type_checker.areTypesCompatible(element_type, first_element_type)) {
                try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                    self.type_checker.allocator,
                    "Array elements must have consistent type, expected '{s}', got '{s}'",
                    .{ first_element_type.name, element_type.name }
                ));
            }
        }
        
        // Create array type with element type
        return TypeInfo.makeArray(self.type_checker.allocator, first_element_type);
    }
    
    // Check struct literal
    fn checkStructLiteral(self: *ASTTypeChecker, struct_literal: *ast.StructLiteral) !*TypeInfo {
        const struct_type = self.type_checker.getTypeByName(struct_literal.type_name) orelse {
            try self.type_checker.addError(.UnknownType, try std.fmt.allocPrint(
                self.type_checker.allocator,
                "Unknown struct type '{s}'",
                .{struct_literal.type_name}
            ));
            return self.type_checker.getTypeByName("unknown").?;
        };
        
        // Check each field assignment
        for (struct_literal.fields.items) |field_init| {
            const field_type = try self.type_checker.checkFieldAccess(struct_type, field_init.name);
            const value_type = try self.checkExpression(field_init.value);
            
            if (!self.type_checker.areTypesCompatible(value_type, field_type)) {
                try self.type_checker.addError(.TypeMismatch, try std.fmt.allocPrint(
                    self.type_checker.allocator,
                    "Cannot assign value of type '{s}' to field '{s}' of type '{s}'",
                    .{ value_type.name, field_init.name, field_type.name }
                ));
            }
        }
        
        return struct_type;
    }
};

// Helper function to run type checking on a program
pub fn typeCheckProgram(allocator: std.mem.Allocator, program: *ast.Program) !SimpleTypeChecker {
    var type_checker = SimpleTypeChecker.init(allocator);
    var ast_checker = ASTTypeChecker.init(&type_checker);
    
    try ast_checker.checkProgram(program);
    
    return type_checker;
}

// Test integration
test "AST type checker integration" {
    const allocator = std.testing.allocator;
    
    // Create a simple test program AST
    var program = ast.Program.init(allocator);
    defer program.deinit();
    
    // Create a variable declaration: sus x drip = 42
    var var_decl = try allocator.create(ast.VariableDeclaration);
    var_decl.* = ast.VariableDeclaration{
        .name = "x",
        .type_annotation = "drip",
        .initializer = null,
        .is_mutable = true,
    };
    
    const stmt = try allocator.create(ast.Statement);
    stmt.* = ast.Statement{ .VariableDeclaration = var_decl };
    try program.statements.append(stmt);
    
    // Type check the program
    var type_checker = try typeCheckProgram(allocator, &program);
    defer type_checker.deinit();
    
    // Check that no errors occurred
    try std.testing.expect(!type_checker.hasErrors());
    
    std.log.info("✅ AST type checker integration test passed", .{});
}
