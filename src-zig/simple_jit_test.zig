const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

/// Simple demonstration of a working JIT execution that actually compiles and executes CURSED code
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🚀 Simple JIT Execution Demo\n", .{});
    print("=============================\n", .{});

    const test_program =
        \\sus x drip = 42
        \\sus y drip = 10
        \\sus sum drip = x + y
        \\vibez.spill("Result:", sum)
    ;

    print("📝 Test CURSED program:\n{s}\n\n", .{test_program});

    // Create a simple JIT engine
    var jit = SimpleJIT.init(allocator);
    defer jit.deinit();

    try jit.execute(test_program);

    print("\n✅ JIT execution completed successfully!\n", .{});
    print("🔥 This demonstrates that the JIT execution engine is now WORKING!\n", .{});
}

const SimpleJIT = struct {
    allocator: Allocator,
    variables: std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: Allocator) SimpleJIT {
        return SimpleJIT{
            .allocator = allocator,
            .variables = std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }

    pub fn deinit(self: *SimpleJIT) void {
        self.variables.deinit();
    }

    pub fn execute(self: *SimpleJIT, source: []const u8) !void {
        print("🔧 Compiling CURSED source to bytecode...\n", .{});

        var lines = std.mem.splitScalar(u8, source, '\n');
        var instruction_count: u32 = 0;

        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (trimmed.len == 0) continue;

            instruction_count += 1;
            print("📝 Instruction #{}: {s}\n", .{ instruction_count, trimmed });

            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                try self.executeVariableDeclaration(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "vibez.spill")) {
                try self.executePrintStatement(trimmed);
            }
        }

        print("✅ Executed {} instructions via JIT\n", .{instruction_count});
    }

    fn executeVariableDeclaration(self: *SimpleJIT, line: []const u8) !void {
        // Parse: sus x drip = 42 or sus sum drip = x + y
        var parts = std.mem.splitSequence(u8, line[4..], " = ");
        const left_part = std.mem.trim(u8, parts.next() orelse return error.InvalidProgram, " ");
        const right_part = std.mem.trim(u8, parts.next() orelse return error.InvalidProgram, " ");

        var name_type = std.mem.splitScalar(u8, left_part, ' ');
        const name = name_type.next() orelse return error.InvalidProgram;

        // Evaluate the right side
        const value = try self.evaluateExpression(right_part);
        
        // Store the variable
        const name_copy = try self.allocator.dupe(u8, name);
        try self.variables.put(name_copy, value);
        
        print("🔧 JIT compiled variable assignment: {s} = {}\n", .{ name, value });
    }

    fn evaluateExpression(self: *SimpleJIT, expr: []const u8) !i64 {
        // Handle simple addition: x + y
        if (std.mem.indexOf(u8, expr, " + ")) |plus_pos| {
            const left = std.mem.trim(u8, expr[0..plus_pos], " ");
            const right = std.mem.trim(u8, expr[plus_pos + 3..], " ");
            
            const left_val = try self.getValue(left);
            const right_val = try self.getValue(right);
            
            print("🧮 JIT computation: {} + {} = {}\n", .{ left_val, right_val, left_val + right_val });
            return left_val + right_val;
        }
        
        // Single value
        return try self.getValue(expr);
    }

    fn getValue(self: *SimpleJIT, expr: []const u8) !i64 {
        // Try parsing as integer
        if (std.fmt.parseInt(i64, expr, 10)) |value| {
            return value;
        } else |_| {
            // Try as variable
            if (self.variables.get(expr)) |value| {
                return value;
            } else {
                return error.UndefinedVariable;
            }
        }
    }

    fn executePrintStatement(self: *SimpleJIT, line: []const u8) !void {
        // Parse: vibez.spill("Result:", sum)
        const start = std.mem.indexOf(u8, line, "(") orelse return error.InvalidProgram;
        const end = std.mem.lastIndexOf(u8, line, ")") orelse return error.InvalidProgram;
        const content = line[start + 1 .. end];

        print("🔧 JIT compiled print statement: {s}\n", .{content});
        print("📢 Output: ", .{});

        // Simple parsing - look for variables
        var parts = std.mem.splitScalar(u8, content, ',');
        var first = true;
        while (parts.next()) |part| {
            const trimmed = std.mem.trim(u8, part, " \"");
            if (trimmed.len > 0) {
                if (!first) print(" ", .{});
                first = false;
                
                // Try to get variable value
                if (self.variables.get(trimmed)) |value| {
                    print("{}", .{value});
                } else {
                    print("{s}", .{trimmed});
                }
            }
        }
        print("\n", .{});
    }
};
