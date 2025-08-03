const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast_simple.zig");
const lexer = @import("lexer.zig");
const error_handling = @import("error_handling.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const CursedError = error_handling.CursedError;
const safeDupeString = error_handling.safeDupeString;

// Forward declaration for struct support
pub const StructInstance = struct {
    type_name: []const u8,
    fields: HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, type_name: []const u8) CursedError!StructInstance {
        const type_name_copy = try safeDupeString(allocator, type_name);
        
        return StructInstance{
            .type_name = type_name_copy,
            .fields = HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *StructInstance) void {
        self.allocator.free(self.type_name);
        self.fields.deinit();
    }
    
    pub fn setField(self: *StructInstance, name: []const u8, value: Value) !void {
        const field_name = try self.allocator.dupe(u8, name);
        try self.fields.put(field_name, value);
    }
    
    pub fn getField(self: *StructInstance, name: []const u8) ?Value {
        return self.fields.get(name);
    }
};

pub const InterpreterError = error{
    UndefinedVariable,
    UndefinedFunction,
    UndefinedStruct,
    UndefinedField,
    TypeMismatch,
    DivisionByZero,
    RuntimeError,
    OutOfMemory,
    InvalidExpression,
    InvalidStatement,
    InvalidStructField,
};

pub const Value = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Struct: StructInstance,
    Array: []Value,
    Null,

    pub fn toString(self: Value, allocator: Allocator) ![]u8 {
        switch (self) {
            .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
            .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
            .String => |str| return allocator.dupe(u8, str),
            .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "lies"),
            .Character => |char| return std.fmt.allocPrint(allocator, "{c}", .{char}),
            .Struct => |struct_inst| return std.fmt.allocPrint(allocator, "{}{{ ... }}", .{struct_inst.type_name}),
            .Array => |arr| return std.fmt.allocPrint(allocator, "[{} items]", .{arr.len}),
            .Null => return allocator.dupe(u8, "cap"),
        }
    }

    pub fn toBool(self: Value) bool {
        switch (self) {
            .Boolean => |bool_val| return bool_val,
            .Integer => |int| return int != 0,
            .Float => |float| return float != 0.0,
            .String => |str| return str.len > 0,
            .Character => |char| return char != 0,
            .Struct => return true, // Structs are always truthy
            .Array => |arr| return arr.len > 0,
            .Null => return false,
        }
    }

    pub fn isNumber(self: Value) bool {
        return switch (self) {
            .Integer, .Float => true,
            else => false,
        };
    }

    pub fn toNumber(self: Value) InterpreterError!f64 {
        switch (self) {
            .Integer => |int| return @as(f64, @floatFromInt(int)),
            .Float => |float| return float,
            else => return InterpreterError.TypeMismatch,
        }
    }
};

pub const Environment = struct {
    variables: HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    parent: ?*Environment,
    allocator: Allocator,

    pub fn init(allocator: Allocator, parent: ?*Environment) Environment {
        return Environment{
            .variables = HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .parent = parent,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Environment) void {
        self.variables.deinit();
    }

    pub fn define(self: *Environment, name: []const u8, value: Value) !void {
        try self.variables.put(name, value);
    }

    pub fn get(self: *Environment, name: []const u8) InterpreterError!Value {
        if (self.variables.get(name)) |value| {
            return value;
        }
        
        if (self.parent) |parent| {
            return parent.get(name);
        }
        
        return InterpreterError.UndefinedVariable;
    }

    pub fn set(self: *Environment, name: []const u8, value: Value) InterpreterError!void {
        if (self.variables.contains(name)) {
            try self.variables.put(name, value);
            return;
        }
        
        if (self.parent) |parent| {
            try parent.set(name, value);
            return;
        }
        
        return InterpreterError.UndefinedVariable;
    }
};

// Basic parsing structures for the simple AST interpreter
pub const ParsedExpression = struct {
    tag: Expression,
    data: union {
        identifier: []const u8,
        integer: i64,
        float: f64,
        string: []const u8,
        boolean: bool,
        character: u8,
        binary: BinaryExpr,
        call: CallExpr,
    },
};

pub const BinaryExpr = struct {
    left: *ParsedExpression,
    operator: []const u8,
    right: *ParsedExpression,
};

pub const CallExpr = struct {
    function: *ParsedExpression,
    arguments: ArrayList(ParsedExpression),
};

pub const ParsedStatement = struct {
    tag: Statement,
    data: union {
        expression: ParsedExpression,
        variable: VarStmt,
        function: FuncStmt,
        return_stmt: RetStmt,
        if_stmt: IfStmt,
        while_stmt: WhileStmt,
        assignment: AssignStmt,
    },
};

pub const VarStmt = struct {
    name: []const u8,
    value: ?ParsedExpression,
};

pub const FuncStmt = struct {
    name: []const u8,
    parameters: ArrayList([]const u8),
    body: ArrayList(ParsedStatement),
};

pub const RetStmt = struct {
    value: ?ParsedExpression,
};

pub const IfStmt = struct {
    condition: ParsedExpression,
    then_branch: ArrayList(ParsedStatement),
    else_branch: ?ArrayList(ParsedStatement),
};

pub const WhileStmt = struct {
    condition: ParsedExpression,
    body: ArrayList(ParsedStatement),
};

pub const AssignStmt = struct {
    name: []const u8,
    value: ParsedExpression,
};

pub const StructType = struct {
    name: []const u8,
    fields: ArrayList(FieldDefinition),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, name: []const u8) CursedError!StructType {
        const name_copy = try safeDupeString(allocator, name);
        
        return StructType{
            .name = name_copy,
            .fields = ArrayList(FieldDefinition).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *StructType) void {
        self.allocator.free(self.name);
        self.fields.deinit();
    }
    
    pub fn addField(self: *StructType, name: []const u8, field_type: []const u8) !void {
        const field = FieldDefinition{
            .name = try self.allocator.dupe(u8, name),
            .field_type = try self.allocator.dupe(u8, field_type),
        };
        try self.fields.append(field);
    }
};

pub const FieldDefinition = struct {
    name: []const u8,
    field_type: []const u8,
};

pub const SimpleInterpreter = struct {
    environment: Environment,
    functions: HashMap([]const u8, FuncStmt, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    struct_types: HashMap([]const u8, StructType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub fn init(allocator: Allocator) SimpleInterpreter {
        return SimpleInterpreter{
            .environment = Environment.init(allocator, null),
            .functions = HashMap([]const u8, FuncStmt, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .struct_types = HashMap([]const u8, StructType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *SimpleInterpreter) void {
        self.environment.deinit();
        self.functions.deinit();
        self.struct_types.deinit();
    }

    pub fn execute(self: *SimpleInterpreter, tokens: []const lexer.Token) InterpreterError!void {
        // For now, implement a very basic execution that handles simple programs
        // This will parse tokens directly into simple expressions and execute them
        
        var i: usize = 0;
        while (i < tokens.len) {
            const token = tokens[i];
            // Debug: std.debug.print("Processing token: {any} = '{s}'\n", .{ token.kind, token.lexeme });
            
            // Skip whitespace and comments
            if (token.kind == .LineComment or token.kind == .BlockComment or token.kind == .Newline) {
                i += 1;
                continue;
            }
            
            // Handle vibez.spill statements
            if (token.kind == .Identifier and std.mem.eql(u8, token.lexeme, "vibez")) {
                i = try self.executeVibesSpill(tokens, i);
                continue;
            }
            
            // Handle struct declarations (squad)
            if (token.kind == .Squad) {
                i = try self.executeStructDeclaration(tokens, i);
                continue;
            }
            
            // Handle variable declarations (sus)
            if (token.kind == .Sus) {
                i = try self.executeVariableDeclaration(tokens, i);
                continue;
            }
            
            // Handle function calls
            if (token.kind == .Identifier) {
                // Check if next token is '(' - this would be a function call
                if (i + 1 < tokens.len and tokens[i + 1].kind == .LeftParen) {
                    i = try self.executeFunctionCall(tokens, i);
                    continue;
                }
            }
            
            i += 1;
        }
    }
    
    fn executeVibesSpill(self: *SimpleInterpreter, tokens: []const lexer.Token, start: usize) InterpreterError!usize {
        var i = start;
        
        // Skip "vibez"
        i += 1;
        
        // Skip "."
        if (i < tokens.len and tokens[i].kind == .Dot) {
            i += 1;
        }
        
        // Skip "spill" 
        if (i < tokens.len and tokens[i].kind == .Spill) {
            i += 1;
        }
        
        // Skip "("
        if (i < tokens.len and tokens[i].kind == .LeftParen) {
            i += 1;
        }
        
        // Get the argument
        if (i < tokens.len) {
            const value = try self.evaluateSimpleExpression(tokens, &i);
            const str = try value.toString(self.allocator);
            defer self.allocator.free(str);
            
            std.debug.print("{s}\n", .{str});
        }
        
        // Skip ")"
        if (i < tokens.len and tokens[i].kind == .RightParen) {
            i += 1;
        }
        
        return i;
    }
    
    fn executeVariableDeclaration(self: *SimpleInterpreter, tokens: []const lexer.Token, start: usize) InterpreterError!usize {
        var i = start;
        
        // Skip "sus"
        i += 1;
        
        // Get variable name
        if (i >= tokens.len or tokens[i].kind != .Identifier) {
            return InterpreterError.InvalidStatement;
        }
        const var_name = tokens[i].lexeme;
        i += 1;
        
        // Skip type annotation if present (drip, tea, etc.)
        if (i < tokens.len and (tokens[i].kind == .Normie or tokens[i].kind == .Tea or tokens[i].kind == .Lit)) {
            i += 1;
        }
        
        // Skip "="
        if (i < tokens.len and tokens[i].kind == .Equal) {
            i += 1;
        }
        
        // Get the value
        const value = try self.evaluateSimpleExpression(tokens, &i);
        try self.environment.define(var_name, value);
        
        return i;
    }
    
    fn executeStructDeclaration(self: *SimpleInterpreter, tokens: []const lexer.Token, start: usize) InterpreterError!usize {
        var i = start;
        
        // Skip "squad"
        i += 1;
        
        // Get struct name
        if (i >= tokens.len or tokens[i].kind != .Identifier) {
            return InterpreterError.InvalidStatement;
        }
        const struct_name = tokens[i].lexeme;
        i += 1;
        
        // Skip opening brace
        if (i >= tokens.len or tokens[i].kind != .LeftBrace) {
            return InterpreterError.InvalidStatement;
        }
        i += 1;
        
        // Create struct type
        var struct_type = StructType.init(self.allocator, struct_name);
        
        // Parse fields
        while (i < tokens.len and tokens[i].kind != .RightBrace) {
            // Skip newlines and spill visibility modifier
            if (tokens[i].kind == .Newline or tokens[i].kind == .Spill) {
                i += 1;
                continue;
            }
            
            // Get field name
            if (tokens[i].kind == .Identifier) {
                const field_name = tokens[i].lexeme;
                i += 1;
                
                // Get field type
                if (i < tokens.len and (tokens[i].kind == .Normie or tokens[i].kind == .Tea or 
                                      tokens[i].kind == .Lit or tokens[i].kind == .Meal or 
                                      tokens[i].kind == .Identifier)) {
                    const field_type = tokens[i].lexeme;
                    i += 1;
                    
                    try struct_type.addField(field_name, field_type);
                }
                
                // Skip optional comma
                if (i < tokens.len and tokens[i].kind == .Comma) {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        
        // Skip closing brace
        if (i < tokens.len and tokens[i].kind == .RightBrace) {
            i += 1;
        }
        
        // Register struct type
        try self.struct_types.put(struct_name, struct_type);
        
        return i;
    }
    
    fn executeFunctionCall(self: *SimpleInterpreter, tokens: []const lexer.Token, start: usize) InterpreterError!usize {
        _ = self;
        _ = tokens;
        // For now, just skip function calls
        return start + 1;
    }
    
    fn evaluateSimpleExpression(self: *SimpleInterpreter, tokens: []const lexer.Token, i: *usize) InterpreterError!Value {
        if (i.* >= tokens.len) {
            return Value.Null;
        }
        
        const token = tokens[i.*];
        i.* += 1;
        
        switch (token.kind) {
            .Number => {
                // Try to parse as integer first, then float
                if (std.fmt.parseInt(i64, token.lexeme, 10)) |int_val| {
                    return Value{ .Integer = int_val };
                } else |_| {
                    if (std.fmt.parseFloat(f64, token.lexeme)) |float_val| {
                        return Value{ .Float = float_val };
                    } else |_| {
                        return Value{ .Integer = 0 };
                    }
                }
            },
            .StringLiteral => {
                // Remove quotes from string literal
                const content = if (token.lexeme.len >= 2) 
                    token.lexeme[1..token.lexeme.len-1] 
                else 
                    token.lexeme;
                return Value{ .String = content };
            },
            .Based => return Value{ .Boolean = true },
            .Lies => return Value{ .Boolean = false },
            .Identifier => {
                // Check if this is a struct literal (Identifier followed by '{')
                if (i.* < tokens.len and tokens[i.*].kind == .LeftBrace) {
                    const struct_name = token.lexeme;
                    return try self.evaluateStructLiteral(tokens, i, struct_name);
                }
                
                // Look up variable
                return self.environment.get(token.lexeme) catch Value.Null;
            },
            else => return Value.Null,
        }
    }
    
    fn evaluateStructLiteral(self: *SimpleInterpreter, tokens: []const lexer.Token, i: *usize, struct_name: []const u8) InterpreterError!Value {
        // Check if struct type exists
        if (!self.struct_types.contains(struct_name)) {
            return InterpreterError.UndefinedStruct;
        }
        
        // Skip opening brace
        i.* += 1;
        
        // Create struct instance
        var struct_instance = StructInstance.init(self.allocator, struct_name);
        
        // Parse field assignments
        while (i.* < tokens.len and tokens[i.*].kind != .RightBrace) {
            // Skip newlines
            if (tokens[i.*].kind == .Newline) {
                i.* += 1;
                continue;
            }
            
            // Get field name
            if (tokens[i.*].kind == .Identifier) {
                const field_name = tokens[i.*].lexeme;
                i.* += 1;
                
                // Skip colon
                if (i.* < tokens.len and tokens[i.*].kind == .Colon) {
                    i.* += 1;
                }
                
                // Get field value
                const field_value = try self.evaluateSimpleExpression(tokens, i);
                try struct_instance.setField(field_name, field_value);
                
                // Skip optional comma
                if (i.* < tokens.len and tokens[i.*].kind == .Comma) {
                    i.* += 1;
                }
            } else {
                i.* += 1;
            }
        }
        
        // Skip closing brace
        if (i.* < tokens.len and tokens[i.*].kind == .RightBrace) {
            i.* += 1;
        }
        
        return Value{ .Struct = struct_instance };
    }
};

test "simple interpreter basic" {
    const allocator = std.testing.allocator;
    
    var interpreter = SimpleInterpreter.init(allocator);
    defer interpreter.deinit();
    
    // Test basic value operations
    const int_val = Value{ .Integer = 42 };
    const str = try int_val.toString(allocator);
    defer allocator.free(str);
    
    try std.testing.expect(std.mem.eql(u8, str, "42"));
}
