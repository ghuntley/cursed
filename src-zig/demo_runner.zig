//! Demo Runner for CURSED Error Handling and Concurrency Features
//! Executes the CURSED demo files and shows the features in action

const std = @import("std");
const Allocator = std.mem.Allocator;
const integration = @import("error_concurrency_integration.zig");

/// Simple CURSED code parser for demo purposes
pub const DemoParser = struct {
    const Self = @This();
    
    allocator: Allocator,
    interpreter: *EnhancedInterpreter,
    
    pub const EnhancedInterpreter = struct {
        allocator: Allocator,
        runtime: *integration.UnifiedRuntime,
        variables: std.StringHashMap(integration.InterpreterValue),
        functions: std.StringHashMap(DemoFunction),
        
        pub const DemoFunction = struct {
            name: []const u8,
            params: [][]const u8,
            body: []const u8,
        };
        
        pub fn init(allocator: Allocator) !*EnhancedInterpreter {
        _ = allocator;
            const interp = try allocator.create(EnhancedInterpreter);
            interp.* = EnhancedInterpreter{
                .allocator = allocator,
                .runtime = try integration.UnifiedRuntime.init(allocator),
                .variables = std.StringHashMap(integration.InterpreterValue){},
                .functions = std.StringHashMap(DemoFunction){},
            };
            return interp;
        }
        
        pub fn deinit(self: *EnhancedInterpreter) void {
            self.variables.deinit(self.allocator);
            self.functions.deinit(self.allocator);
            self.runtime.deinit(self.allocator);
            self.allocator.destroy(self);
        }
        
        pub fn executeCommand(self: *EnhancedInterpreter, command: []const u8) !void {
            const trimmed = std.mem.trim(u8, command, " \t\r\n");
            
            if (trimmed.len == 0 or trimmed[0] == '#') {
                return; // Skip empty lines and comments
            }
            
            // Handle different command types
            if (std.mem.startsWith(u8, trimmed, "vibez.spill(")) {
                try self.handleSpillCommand(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "yikes ")) {
                try self.handleYikesCommand(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "fam {")) {
                try self.handleFamCommand(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "stan ")) {
                try self.handleStanCommand(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "sus ")) {
                try self.handleVariableDeclaration(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "slay ")) {
                try self.handleFunctionDefinition(trimmed);
            } else if (std.mem.indexOf(u8, trimmed, "()") != null) {
                try self.handleFunctionCall(trimmed);
            } else {
                std.debug.print("// Unknown command: {s}\n", .{trimmed});
            }
        }
        
        fn handleSpillCommand(self: *EnhancedInterpreter, command: []const u8) !void {
            // Extract content between parentheses
            const start = std.mem.indexOf(u8, command, "(") orelse return;
            const end = std.mem.lastIndexOf(u8, command, ")") orelse return;
            const content = command[start + 1 .. end];
            
            // Simple argument parsing
            var args = std.mem.split(u8, content, ",");
            std.debug.print("[CURSED OUTPUT] ");
            
            var first = true;
            while (args.next()) |arg| {
                if (!first) std.debug.print(" ");
                first = false;
                
                const trimmed_arg = std.mem.trim(u8, arg, " \t");
                
                // Handle string literals
                if (trimmed_arg.len >= 2 and trimmed_arg[0] == '"' and trimmed_arg[trimmed_arg.len - 1] == '"') {
                    const str_content = trimmed_arg[1 .. trimmed_arg.len - 1];
                    std.debug.print("{s}", .{str_content});
                } else if (self.variables.get(trimmed_arg)) |value| {
                    std.debug.print("{s}", .{value});
                } else {
                    std.debug.print("{s}", .{trimmed_arg});
                }
            }
            std.debug.print("\n");
        }
        
        fn handleYikesCommand(self: *EnhancedInterpreter, command: []const u8) !void {
            // Extract error message
            const start = std.mem.indexOf(u8, command, "\"") orelse return;
            const end = std.mem.lastIndexOf(u8, command, "\"") orelse return;
            const message = command[start + 1 .. end];
            
            const error_result = try integration.executeYikesStatement(
                self.runtime,
                message,
                .Runtime,
                1000
            );
            
            std.debug.print("[CURSED ERROR] Created: {s}\n", .{error_result});
        }
        
        fn handleFamCommand(self: *EnhancedInterpreter, command: []const u8) !void {
            _ = command;
            std.debug.print("[CURSED FAM] Entering recovery block\n");
            
            // Simulate fam block execution
            const fam_result = integration.executeFamStatement(
                self.runtime,
                struct {
                    fn tryBlock() integration.InterpreterValue {
                        return integration.InterpreterValue{ .Integer = 42 };
                    }
                }.tryBlock,
                struct {
                    fn catchBlock(error_obj: *integration.advanced_error_handling.CursedError) integration.InterpreterValue {
                        std.debug.print("[CURSED FAM] Caught error: {s}\n", .{error_obj.*});
                        return integration.InterpreterValue{ .String = "Recovered" };
                    }
                }.catchBlock
            );
            
            std.debug.print("[CURSED FAM] Result: {s}\n", .{fam_result});
        }
        
        fn handleStanCommand(self: *EnhancedInterpreter, command: []const u8) !void {
            _ = command;
            std.debug.print("[CURSED STAN] Spawning goroutine\n");
            
            var counter: i32 = 0;
            const goroutine_result = try integration.executeStanStatement(
                self.runtime,
                struct {
                    fn entry(ctx: ?*anyopaque) void {
                        if (ctx) |c| {
                            const counter_ptr = @as(*i32, @ptrCast(@alignCast(c)));
                            counter_ptr.* = 42;
                            std.debug.print("[CURSED GOROUTINE] Executed, set counter to {s}\n", .{counter_ptr.*});
                        }
                    }
                }.entry,
                &counter
            );
            
            std.debug.print("[CURSED STAN] Goroutine ID: {s}\n", .{goroutine_result});
            
            // Give goroutine time to execute
            std.time.sleep(10_000_000); // 10ms
            std.debug.print("[CURSED STAN] Counter after goroutine: {s}\n", .{counter});
        }
        
        fn handleVariableDeclaration(self: *EnhancedInterpreter, command: []const u8) !void {
            // Simple variable parsing: sus name type = value
            var parts = std.mem.split(u8, command, " ");
            _ = parts.next(); // Skip "sus"
            const name = parts.next() orelse return;
            const type_part = parts.next() orelse return;
            _ = type_part; // Skip type for now
            _ = parts.next(); // Skip "="
            const value_part = parts.next() orelse return;
            
            var value: integration.InterpreterValue = undefined;
            if (std.mem.startsWith(u8, value_part, "\"")) {
                // String literal
                const str_content = std.mem.trim(u8, value_part, "\"");
                value = integration.InterpreterValue{ .String = try self.allocator.dupe(u8, str_content) };
            } else if (std.fmt.parseInt(i64, value_part, 10)) |int_val| {
                value = integration.InterpreterValue{ .Integer = int_val };
            } else |_| {
                value = integration.InterpreterValue.Null;
            }
            
            const owned_name = try self.allocator.dupe(u8, name);
            try self.variables.put(owned_name, value);
            std.debug.print("[CURSED VAR] {s} = {s}\n", .{ name, value });
        }
        
        fn handleFunctionDefinition(self: *EnhancedInterpreter, command: []const u8) !void {
            _ = command;
            std.debug.print("[CURSED FUNC] Function definition (simplified)\n");
        }
        
        fn handleFunctionCall(self: *EnhancedInterpreter, command: []const u8) !void {
            const trimmed = std.mem.trim(u8, command, " \t\r\n");
            std.debug.print("[CURSED CALL] Function call: {s}\n", .{trimmed});
            
            // Handle main() specifically
            if (std.mem.eql(u8, trimmed, "main()")) {
                std.debug.print("[CURSED MAIN] Executing main function\n");
            }
        }
    };
    
    pub fn init(allocator: Allocator) !*Self {
        _ = allocator;
        const parser = try allocator.create(Self);
        parser.* = Self{
            .allocator = allocator,
            .interpreter = try EnhancedInterpreter.init(allocator),
        };
        return parser;
    }
    
    pub fn deinit(self: *Self) void {
        self.interpreter.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    pub fn parseFile(self: *Self, file_path: []const u8) !void {
        std.debug.print("=== Parsing CURSED file: {s} ===\n", .{file_path});
        
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
            std.debug.print("Error opening file {s}: {s}\n", .{ file_path, err });
            return;
        };
        defer file.close();
        
        const file_size = try file.getEndPos();
        const content = try self.allocator.alloc(u8, file_size);
        defer self.allocator.free(content);
        _ = try file.readAll(content);
        
        var lines = std.mem.split(u8, content, "\n");
        var line_number: usize = 1;
        
        while (lines.next()) |line| {
            defer line_number += 1;
            
            self.interpreter.executeCommand(line) catch |err| {
                std.debug.print("Error on line {s}: {s} - {s}\n", .{ line_number, err, line });
            };
        }
        
        std.debug.print("=== Finished parsing {s} ===\n", .{file_path});
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("CURSED Demo Runner - Error Handling and Concurrency\n");
    std.debug.print("===================================================\n");
    
    const parser = try DemoParser.init(allocator);
    defer parser.deinit();
    
    // Try to run the error handling demo
    std.debug.print("\n");
    parser.parseFile("error_handling_demo.💀") catch |err| {
        std.debug.print("Could not run error_handling_demo.💀.💀: {s}\n", .{err});
    };
    
    std.debug.print("\n");
    parser.parseFile("concurrency_demo.💀") catch |err| {
        std.debug.print("Could not run concurrency_demo.💀.💀: {s}\n", .{err});
    };
    
    // Run integration tests directly
    std.debug.print("\n=== Direct Integration Tests ===\n");
    try integration.testIntegration(allocator);
    
    std.debug.print("\n=== Demo Runner Complete ===\n");
}
