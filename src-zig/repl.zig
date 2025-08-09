const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const type_system_runtime = @import("type_system_runtime.zig");
const module_loader = @import("module_loader.zig");

// Import Variable and VariableStore from main_unified.zig
const main = @import("main_unified.zig");
const Variable = main.Variable;
const VariableStore = main.VariableStore;
const FunctionStore = main.FunctionStore;
const StructStore = main.StructStore;

/// CURSED REPL session manager
pub const ReplSession = struct {
    variables: VariableStore,
    functions: FunctionStore,
    structs: StructStore,
    history: ArrayList([]const u8),
    allocator: Allocator,
    verbose: bool,
    line_number: u32,
    
    pub fn init(allocator: Allocator, verbose: bool) ReplSession {
        return ReplSession{
            .variables = VariableStore.init(allocator),
            .functions = FunctionStore.init(allocator),
            .structs = StructStore.init(allocator),
            .history = ArrayList([]const u8).init(allocator),
            .allocator = allocator,
            .verbose = verbose,
            .line_number = 1,
        };
    }
    
    pub fn deinit(self: *ReplSession) void {
        // Clean up variables
        var var_iter = self.variables.iterator();
        while (var_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit(self.allocator);
        }
        self.variables.deinit();
        
        // Clean up functions
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit(self.allocator);
        }
        self.functions.deinit();
        
        // Clean up structs
        var struct_iter = self.structs.iterator();
        while (struct_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit(self.allocator);
        }
        self.structs.deinit();
        
        // Clean up history
        for (self.history.items) |line| {
            self.allocator.free(line);
        }
        self.history.deinit();
    }
    
    /// Evaluate a CURSED expression in the REPL context
    pub fn evaluate(self: *ReplSession, input: []const u8) !?Variable {
        const trimmed = std.mem.trim(u8, input, " \t\r\n");
        if (trimmed.len == 0) return null;
        
        // Add to history
        const history_line = try self.allocator.dupe(u8, trimmed);
        try self.history.append(history_line);
        
        // Try to parse and evaluate the input
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        // First try: parse as a complete statement
        if (try self.tryParseStatement(arena_allocator, trimmed)) |result| {
            return result;
        }
        
        // Second try: parse as an expression
        if (try self.tryParseExpression(arena_allocator, trimmed)) |result| {
            return result;
        }
        
        // If both fail, return an error
        return error.ParseError;
    }
    
    /// Try to parse input as a complete statement
    fn tryParseStatement(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !?Variable {
        // Handle variable declarations: sus x drip = 42
        if (std.mem.startsWith(u8, input, "sus ")) {
            return try self.handleVariableDeclaration(arena_allocator, input);
        }
        
        // Handle function definitions: slay func_name(params) type { body }
        if (std.mem.startsWith(u8, input, "slay ")) {
            return try self.handleFunctionDefinition(arena_allocator, input);
        }
        
        // Handle print statements: vibez.spill(...)
        if (std.mem.indexOf(u8, input, "vibez.spill(")) |_| {
            return try self.handlePrintStatement(arena_allocator, input);
        }
        
        // Handle module imports: yeet "module_name"
        if (std.mem.startsWith(u8, input, "yeet ")) {
            return try self.handleModuleImport(arena_allocator, input);
        }
        
        // Handle assignments: var = value
        if (std.mem.indexOf(u8, input, "=")) |equals_pos| {
            return try self.handleAssignment(arena_allocator, input, equals_pos);
        }
        
        return null;
    }
    
    /// Try to parse input as an expression
    fn tryParseExpression(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !?Variable {
        // Use the existing expression evaluation from main_unified.zig
        const main_module = @import("main_unified.zig");
        
        // Create a temporary variable to hold the result
        var result = main_module.evaluateExpression(&self.variables, &self.functions, arena_allocator, input, self.verbose) catch {
            return null;
        };
        
        // Clone the result so it persists beyond the arena
        return try result.clone(self.allocator);
    }
    
    /// Handle variable declarations
    fn handleVariableDeclaration(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        _ = arena_allocator;
        
        // Parse: sus variable_name type = value
        const declaration = std.mem.trim(u8, input[4..], " \t"); // Remove "sus "
        
        if (std.mem.indexOf(u8, declaration, "=")) |equals_pos| {
            const left_side = std.mem.trim(u8, declaration[0..equals_pos], " \t");
            const value_expr = std.mem.trim(u8, declaration[equals_pos + 1..], " \t");
            
            // Parse variable name and type
            var parts = std.mem.splitScalar(u8, left_side, ' ');
            const var_name = parts.next() orelse return error.InvalidSyntax;
            const var_type = parts.next(); // Optional type annotation
            
            if (var_type != null and self.verbose) {
                print("  📝 Variable type: {s}\n", .{var_type.?});
            }
            
            // Evaluate the value expression
            const main_module = @import("main_unified.zig");
            const value = try main_module.evaluateExpression(&self.variables, &self.functions, self.allocator, value_expr, self.verbose);
            
            // Store the variable
            const var_name_copy = try self.allocator.dupe(u8, var_name);
            try self.variables.put(var_name_copy, value);
            
            if (self.verbose) {
                print("  ✅ Variable declared: {s} = {any}\n", .{ var_name, value });
            }
            
            return value;
        }
        
        return error.InvalidSyntax;
    }
    
    /// Handle function definitions
    fn handleFunctionDefinition(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        _ = arena_allocator;
        
        // For now, just acknowledge the function definition
        // Full function parsing would require more complex AST handling
        const func_text = std.mem.trim(u8, input[5..], " \t"); // Remove "slay "
        
        if (std.mem.indexOf(u8, func_text, "(")) |paren_pos| {
            const func_name = std.mem.trim(u8, func_text[0..paren_pos], " \t");
            print("  📝 Function definition: {s}\n", .{func_name});
            
            // Store a placeholder function for now
            const name_copy = try self.allocator.dupe(u8, func_name);
            const func_def = main.FunctionDefinition.init(self.allocator, name_copy);
            try self.functions.put(name_copy, func_def);
            
            const result_str = try std.fmt.allocPrint(self.allocator, "Function '{s}' defined", .{func_name});
            return Variable{ .String = main.ManagedString.fromOwned(result_str) };
        }
        
        return error.InvalidSyntax;
    }
    
    /// Handle print statements
    fn handlePrintStatement(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        const main_module = @import("main_unified.zig");
        
        // Execute the print statement using the existing handler
        try main_module.handleVibesSpill(&self.variables, &self.functions, arena_allocator, input, 0, self.verbose);
        
        return Variable{ .String = main.ManagedString.fromLiteral("") };
    }
    
    /// Handle module imports
    fn handleModuleImport(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        _ = arena_allocator;
        
        // Parse: yeet "module_name"
        const import_text = std.mem.trim(u8, input[5..], " \t"); // Remove "yeet "
        
        if (import_text.len >= 2 and import_text[0] == '"' and import_text[import_text.len - 1] == '"') {
            const module_name = import_text[1..import_text.len - 1];
            
            // Try to load the module using existing module loader
            const simple_resolver = @import("simple_import_resolver.zig");
            _ = simple_resolver.resolveImport(self.allocator, module_name, "stdlib", self.verbose) catch |err| {
                if (self.verbose) {
                    print("  ⚠️  Module import warning: {any}\n", .{err});
                }
                const result_str = try std.fmt.allocPrint(self.allocator, "Module '{s}' not found", .{module_name});
                return Variable{ .String = main.ManagedString.fromOwned(result_str) };
            };
            
            const result_str = try std.fmt.allocPrint(self.allocator, "Module '{s}' imported", .{module_name});
            return Variable{ .String = main.ManagedString.fromOwned(result_str) };
        }
        
        return error.InvalidSyntax;
    }
    
    /// Handle assignments
    fn handleAssignment(self: *ReplSession, arena_allocator: Allocator, input: []const u8, equals_pos: usize) !Variable {
        const var_name = std.mem.trim(u8, input[0..equals_pos], " \t");
        const value_expr = std.mem.trim(u8, input[equals_pos + 1..], " \t");
        
        // Evaluate the value expression
        const main_module = @import("main_unified.zig");
        const value = try main_module.evaluateExpression(&self.variables, &self.functions, arena_allocator, value_expr, self.verbose);
        
        // Clone the value to persist beyond arena
        const persistent_value = try value.clone(self.allocator);
        
        // Update or create the variable
        if (self.variables.getPtr(var_name)) |existing| {
            existing.deinit(self.allocator);
            existing.* = persistent_value;
        } else {
            const var_name_copy = try self.allocator.dupe(u8, var_name);
            try self.variables.put(var_name_copy, persistent_value);
        }
        
        if (self.verbose) {
            print("  ✅ Assignment: {s} = {any}\n", .{ var_name, persistent_value });
        }
        
        return try persistent_value.clone(self.allocator);
    }
    
    /// Show variables in the current session
    pub fn showVariables(self: *ReplSession) void {
        if (self.variables.count() == 0) {
            print("  No variables defined\n", .{});
            return;
        }
        
        print("  Current variables:\n", .{});
        var iter = self.variables.iterator();
        while (iter.next()) |entry| {
            const var_str = entry.value_ptr.toString(self.allocator) catch "???";
            defer if (!std.mem.eql(u8, var_str, "???")) self.allocator.free(var_str);
            print("    {s} = {s}\n", .{ entry.key_ptr.*, var_str });
        }
    }
    
    /// Show command history
    pub fn showHistory(self: *ReplSession) void {
        if (self.history.items.len == 0) {
            print("  No command history\n", .{});
            return;
        }
        
        print("  Command history:\n", .{});
        for (self.history.items, 0..) |line, i| {
            print("  {:3}: {s}\n", .{ i + 1, line });
        }
    }
    
    /// Clear the screen
    pub fn clearScreen(self: *ReplSession) void {
        _ = self;
        // ANSI escape sequence to clear screen and move cursor to top-left
        print("\x1B[2J\x1B[1;1H", .{});
    }
};

/// CURSED REPL implementation
pub fn runRepl(allocator: Allocator, verbose: bool) !void {
    var session = ReplSession.init(allocator, verbose);
    defer session.deinit();
    
    // Print welcome message
    printWelcome();
    
    // Main REPL loop
    var stdin = std.io.getStdIn().reader();
    
    while (true) {
        // Print prompt
        print("{s}", .{"cursed> "});
        
        // Read input
        var input_buffer: [1024]u8 = undefined;
        const input = (try stdin.readUntilDelimiterOrEof(input_buffer[0..], '\n')) orelse break;
        
        // Handle special commands
        if (handleSpecialCommand(&session, input)) |should_exit| {
            if (should_exit) break;
            continue;
        }
        
        // Evaluate the input
        if (session.evaluate(input)) |result| {
            if (result) |value| {
                const value_str = value.toString(allocator) catch "???";
                defer if (!std.mem.eql(u8, value_str, "???")) allocator.free(value_str);
                
                // Only print non-empty results
                if (!std.mem.eql(u8, value_str, "")) {
                    print("{s}\n", .{value_str});
                }
                
                // Clean up the result
                var temp_value = value;
                temp_value.deinit(allocator);
            }
        } else |err| {
            switch (err) {
                error.ParseError => print("Error: Invalid syntax\n", .{}),
                else => print("Error: {any}\n", .{err}),
            }
        }
        
        session.line_number += 1;
    }
    
    print("Goodbye!\n", .{});
}

/// Handle special REPL commands
fn handleSpecialCommand(session: *ReplSession, input: []const u8) ?bool {
    const trimmed = std.mem.trim(u8, input, " \t\r\n");
    
    if (std.mem.eql(u8, trimmed, ":quit") or std.mem.eql(u8, trimmed, ":exit") or std.mem.eql(u8, trimmed, ":q")) {
        return true; // Signal to exit
    }
    
    if (std.mem.eql(u8, trimmed, ":help") or std.mem.eql(u8, trimmed, ":h")) {
        printHelp();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":vars") or std.mem.eql(u8, trimmed, ":variables")) {
        session.showVariables();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":history") or std.mem.eql(u8, trimmed, ":hist")) {
        session.showHistory();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":clear") or std.mem.eql(u8, trimmed, ":cls")) {
        session.clearScreen();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":version")) {
        print("CURSED REPL v1.0.0 (Zig implementation)\n", .{});
        return false;
    }
    
    if (std.mem.startsWith(u8, trimmed, ":")) {
        print("Unknown command: {s}. Type :help for available commands.\n", .{trimmed});
        return false;
    }
    
    return null; // Not a special command
}

/// Print welcome message
fn printWelcome() void {
    print("🔥 CURSED REPL v1.0.0\n", .{});
    print("Interactive CURSED language shell\n", .{});
    print("Type :help for help, :quit to exit\n", .{});
    print("\n", .{});
}

/// Print help message
fn printHelp() void {
    print("\n", .{});
    print("CURSED REPL Commands:\n", .{});
    print("  :help, :h         - Show this help message\n", .{});
    print("  :quit, :exit, :q  - Exit the REPL\n", .{});
    print("  :vars, :variables - Show current variables\n", .{});
    print("  :history, :hist   - Show command history\n", .{});
    print("  :clear, :cls      - Clear the screen\n", .{});
    print("  :version          - Show version information\n", .{});
    print("\n", .{});
    print("CURSED Language Features:\n", .{});
    print("  Variables:  sus x drip = 42\n", .{});
    print("  Functions:  slay add(a drip, b drip) drip {{ damn a + b }}\n", .{});
    print("  Arrays:     sus arr []drip = [1, 2, 3]\n", .{});
    print("  Print:      vibez.spill(\"Hello, world!\")\n", .{});
    print("  Import:     yeet \"stdlib_module\"\n", .{});
    print("  Control:    ready (condition) {{ ... }}\n", .{});
    print("              bestie (condition) {{ ... }}\n", .{});
    print("\n", .{});
}
