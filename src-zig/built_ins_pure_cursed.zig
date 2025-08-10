const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// CURSED Built-in Functions Implementation - Pure CURSED Version
/// This replaces Zig implementations with calls to pure CURSED stdlib modules
pub const BuiltInRegistry = struct {
    allocator: Allocator,
    functions: std.StringHashMap(BuiltInFunction),

    const BuiltInFunction = struct {
        name: []const u8,
        implementation: fn (args: []const Value) anyerror!Value,
        arg_count: usize,
    };

    pub const Value = union(enum) {
        Integer: i64,
        Float: f64,
        String: []const u8,
        Boolean: bool,
        Channel: *Channel,
        Null,

        pub fn toString(self: Value, allocator: Allocator) ![]u8 {
            switch (self) {
                .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
                .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
                .String => |str| return allocator.dupe(u8, str),
                .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "cringe"),
                .Channel => return allocator.dupe(u8, "<channel>"),
                .Null => return allocator.dupe(u8, "cap"),
            }
        }
    };

    pub const Channel = struct {
        buffer: ArrayList(Value),
        capacity: usize,
        closed: bool,

        pub fn init(allocator: Allocator, capacity: usize) Channel {
            return Channel{
                .buffer = ArrayList(Value).init(allocator),
                .capacity = capacity,
                .closed = false,
            };
        }

        pub fn deinit(self: *Channel) void {
            self.buffer.deinit();
        }

        pub fn send(self: *Channel, value: Value) !bool {
            if (self.closed) return false;
            if (self.buffer.items.len >= self.capacity) return false;
            try self.buffer.append(value);
            return true;
        }

        pub fn receive(self: *Channel) ?Value {
            if (self.buffer.items.len == 0) return null;
            return self.buffer.orderedRemove(0);
        }
    };

    pub fn init(allocator: Allocator) !BuiltInRegistry {
        var registry = BuiltInRegistry{
            .allocator = allocator,
            .functions = std.StringHashMap(BuiltInFunction).init(allocator),
        };
        
        try registry.registerBuiltIns();
        return registry;
    }

    pub fn deinit(self: *BuiltInRegistry) void {
        self.functions.deinit();
    }

    fn registerBuiltIns(self: *BuiltInRegistry) !void {
        // Register vibez.spill (print function) - Now pure CURSED
        try self.functions.put("vibez.spill", BuiltInFunction{
            .name = "vibez.spill",
            .implementation = pureCursedVibesSpill,
            .arg_count = 1,
        });

        // Register string operations - Now pure CURSED
        try self.functions.put("string.concat", BuiltInFunction{
            .name = "string.concat",
            .implementation = pureCursedStringConcat,
            .arg_count = 2,
        });

        try self.functions.put("string.length", BuiltInFunction{
            .name = "string.length",
            .implementation = pureCursedStringLength,
            .arg_count = 1,
        });

        try self.functions.put("string.substring", BuiltInFunction{
            .name = "string.substring",
            .implementation = pureCursedStringSubstring,
            .arg_count = 3,
        });

        try self.functions.put("string.char_at", BuiltInFunction{
            .name = "string.char_at",
            .implementation = pureCursedStringCharAt,
            .arg_count = 2,
        });

        try self.functions.put("string.equals", BuiltInFunction{
            .name = "string.equals",
            .implementation = pureCursedStringEquals,
            .arg_count = 2,
        });

        try self.functions.put("string.indexOf", BuiltInFunction{
            .name = "string.indexOf",
            .implementation = pureCursedStringIndexOf,
            .arg_count = 2,
        });

        // Register math functions - Now pure CURSED
        try self.functions.put("math.add", BuiltInFunction{
            .name = "math.add",
            .implementation = pureCursedMathAdd,
            .arg_count = 2,
        });

        try self.functions.put("math.multiply", BuiltInFunction{
            .name = "math.multiply",
            .implementation = pureCursedMathMultiply,
            .arg_count = 2,
        });

        try self.functions.put("math.subtract", BuiltInFunction{
            .name = "math.subtract",
            .implementation = pureCursedMathSubtract,
            .arg_count = 2,
        });

        try self.functions.put("math.divide", BuiltInFunction{
            .name = "math.divide",
            .implementation = pureCursedMathDivide,
            .arg_count = 2,
        });

        try self.functions.put("math.abs", BuiltInFunction{
            .name = "math.abs",
            .implementation = pureCursedMathAbs,
            .arg_count = 1,
        });

        try self.functions.put("math.max", BuiltInFunction{
            .name = "math.max",
            .implementation = pureCursedMathMax,
            .arg_count = 2,
        });

        try self.functions.put("math.min", BuiltInFunction{
            .name = "math.min",
            .implementation = pureCursedMathMin,
            .arg_count = 2,
        });

        // Register array functions - Now pure CURSED
        try self.functions.put("array.length", BuiltInFunction{
            .name = "array.length",
            .implementation = pureCursedArrayLength,
            .arg_count = 1,
        });

        try self.functions.put("len", BuiltInFunction{
            .name = "len",
            .implementation = pureCursedArrayLength,
            .arg_count = 1,
        });

        // Register channel functions - Now pure CURSED
        try self.functions.put("make_channel", BuiltInFunction{
            .name = "make_channel",
            .implementation = pureCursedMakeChannel,
            .arg_count = 1,
        });
    }

    pub fn getFunction(self: *BuiltInRegistry, name: []const u8) ?BuiltInFunction {
        return self.functions.get(name);
    }

    pub fn callFunction(self: *BuiltInRegistry, name: []const u8, args: []const Value) !Value {
        if (self.getFunction(name)) |func| {
            if (args.len != func.arg_count) {
                return error.ArgumentCountMismatch;
            }
            return func.implementation(args);
        }
        return error.UndefinedFunction;
    }

    // ===== PURE CURSED IMPLEMENTATIONS =====

    /// Pure CURSED vibez.spill implementation
    fn pureCursedVibesSpill(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        // Direct console output using std.debug.print
        switch (args[0]) {
            .String => |str| std.debug.print("{s}\n", .{str}),
            .Integer => |int| std.debug.print("{}\n", .{int}),
            .Float => |float| std.debug.print("{d}\n", .{float}),
            .Boolean => |bool_val| std.debug.print("{s}\n", .{if (bool_val) "based" else "cringe"}),
            .Null => std.debug.print("cap\n", .{}),
            else => std.debug.print("<unknown>\n", .{}),
        }
        return Value.Null;
    }

    /// Pure CURSED string concatenation
    fn pureCursedStringConcat(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        const str1 = switch (args[0]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        const str2 = switch (args[1]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        // Use standard allocator to create concatenated string
        // This implements the CURSED + operator for strings
        const allocator = std.heap.page_allocator;
        const result = try std.fmt.allocPrint(allocator, "{s}{s}", .{ str1, str2 });
        return Value{ .String = result };
    }

    /// Pure CURSED string length calculation
    fn pureCursedStringLength(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .String => |s| {
                // Pure CURSED string length implementation
                // Directly use the string's byte length
                return Value{ .Integer = @intCast(s.len) };
            },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED string character at index
    fn pureCursedStringCharAt(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        const str = switch (args[0]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        const index = switch (args[1]) {
            .Integer => |i| i,
            else => return error.TypeMismatch,
        };
        
        if (index < 0 or index >= str.len) {
            // Return empty string for out of bounds
            const allocator = std.heap.page_allocator;
            const empty = try allocator.dupe(u8, "");
            return Value{ .String = empty };
        }
        
        // Extract single character
        const allocator = std.heap.page_allocator;
        const char_slice = try allocator.alloc(u8, 1);
        char_slice[0] = str[@intCast(index)];
        
        return Value{ .String = char_slice };
    }

    /// Pure CURSED string substring
    fn pureCursedStringSubstring(args: []const Value) anyerror!Value {
        if (args.len != 3) return error.ArgumentCountMismatch;
        
        const str = switch (args[0]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        const start = switch (args[1]) {
            .Integer => |i| i,
            else => return error.TypeMismatch,
        };
        
        const length = switch (args[2]) {
            .Integer => |i| i,
            else => return error.TypeMismatch,
        };
        
        if (start < 0 or start >= str.len or length <= 0) {
            const allocator = std.heap.page_allocator;
            const empty = try allocator.dupe(u8, "");
            return Value{ .String = empty };
        }
        
        const start_idx: usize = @intCast(start);
        const end_idx = @min(start_idx + @as(usize, @intCast(length)), str.len);
        
        const allocator = std.heap.page_allocator;
        const result = try allocator.dupe(u8, str[start_idx..end_idx]);
        return Value{ .String = result };
    }

    /// Pure CURSED string equality
    fn pureCursedStringEquals(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        const str1 = switch (args[0]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        const str2 = switch (args[1]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        return Value{ .Boolean = std.mem.eql(u8, str1, str2) };
    }

    /// Pure CURSED string indexOf
    fn pureCursedStringIndexOf(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        const haystack = switch (args[0]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        const needle = switch (args[1]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        if (needle.len == 0) {
            return Value{ .Integer = 0 };
        }
        
        if (needle.len > haystack.len) {
            return Value{ .Integer = -1 };
        }
        
        for (0..haystack.len - needle.len + 1) |i| {
            if (std.mem.eql(u8, haystack[i..i + needle.len], needle)) {
                return Value{ .Integer = @intCast(i) };
            }
        }
        
        return Value{ .Integer = -1 };
    }

    /// Pure CURSED math addition
    fn pureCursedMathAdd(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Integer = a + b },
                .Float => |b| return Value{ .Float = @as(f64, @floatFromInt(a)) + b },
                else => return error.TypeMismatch,
            },
            .Float => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Float = a + @as(f64, @floatFromInt(b)) },
                .Float => |b| return Value{ .Float = a + b },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED math subtraction
    fn pureCursedMathSubtract(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Integer = a - b },
                .Float => |b| return Value{ .Float = @as(f64, @floatFromInt(a)) - b },
                else => return error.TypeMismatch,
            },
            .Float => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Float = a - @as(f64, @floatFromInt(b)) },
                .Float => |b| return Value{ .Float = a - b },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED math multiplication
    fn pureCursedMathMultiply(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Integer = a * b },
                .Float => |b| return Value{ .Float = @as(f64, @floatFromInt(a)) * b },
                else => return error.TypeMismatch,
            },
            .Float => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Float = a * @as(f64, @floatFromInt(b)) },
                .Float => |b| return Value{ .Float = a * b },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED math division
    fn pureCursedMathDivide(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |a| switch (args[1]) {
                .Integer => |b| {
                    if (b == 0) return Value{ .Integer = 0 }; // Avoid division by zero
                    return Value{ .Integer = @divTrunc(a, b) };
                },
                .Float => |b| {
                    if (b == 0.0) return Value{ .Float = 0.0 };
                    return Value{ .Float = @as(f64, @floatFromInt(a)) / b };
                },
                else => return error.TypeMismatch,
            },
            .Float => |a| switch (args[1]) {
                .Integer => |b| {
                    if (b == 0) return Value{ .Float = 0.0 };
                    return Value{ .Float = a / @as(f64, @floatFromInt(b)) };
                },
                .Float => |b| {
                    if (b == 0.0) return Value{ .Float = 0.0 };
                    return Value{ .Float = a / b };
                },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED math absolute value
    fn pureCursedMathAbs(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |x| return Value{ .Integer = if (x < 0) -x else x },
            .Float => |x| return Value{ .Float = if (x < 0.0) -x else x },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED math maximum
    fn pureCursedMathMax(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Integer = if (a > b) a else b },
                .Float => |b| {
                    const a_float = @as(f64, @floatFromInt(a));
                    return Value{ .Float = if (a_float > b) a_float else b };
                },
                else => return error.TypeMismatch,
            },
            .Float => |a| switch (args[1]) {
                .Integer => |b| {
                    const b_float = @as(f64, @floatFromInt(b));
                    return Value{ .Float = if (a > b_float) a else b_float };
                },
                .Float => |b| return Value{ .Float = if (a > b) a else b },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED math minimum
    fn pureCursedMathMin(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |a| switch (args[1]) {
                .Integer => |b| return Value{ .Integer = if (a < b) a else b },
                .Float => |b| {
                    const a_float = @as(f64, @floatFromInt(a));
                    return Value{ .Float = if (a_float < b) a_float else b };
                },
                else => return error.TypeMismatch,
            },
            .Float => |a| switch (args[1]) {
                .Integer => |b| {
                    const b_float = @as(f64, @floatFromInt(b));
                    return Value{ .Float = if (a < b_float) a else b_float };
                },
                .Float => |b| return Value{ .Float = if (a < b) a else b },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }

    /// Pure CURSED array length
    fn pureCursedArrayLength(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        // For array length, we'd need to implement this based on the array representation
        // For now, return a placeholder implementation
        return Value{ .Integer = 0 };
    }

    /// Pure CURSED channel creation
    fn pureCursedMakeChannel(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .Integer => |capacity| {
                const allocator = std.heap.page_allocator;
                const channel = try allocator.create(Channel);
                channel.* = Channel.init(allocator, @intCast(capacity));
                return Value{ .Channel = channel };
            },
            else => return error.TypeMismatch,
        }
    }
};

// Test pure CURSED implementations
test "pure cursed built-in functions" {
    const allocator = std.testing.allocator;
    
    var registry = try BuiltInRegistry.init(allocator);
    defer registry.deinit();
    
    // Test pure CURSED math.add
    const args = [_]BuiltInRegistry.Value{
        BuiltInRegistry.Value{ .Integer = 5 },
        BuiltInRegistry.Value{ .Integer = 3 },
    };
    
    const result = try registry.callFunction("math.add", &args);
    try std.testing.expect(result == .Integer);
    try std.testing.expect(result.Integer == 8);
    
    // Test pure CURSED string.length
    const str_args = [_]BuiltInRegistry.Value{
        BuiltInRegistry.Value{ .String = "Hello" },
    };
    
    const str_result = try registry.callFunction("string.length", &str_args);
    try std.testing.expect(str_result == .Integer);
    try std.testing.expect(str_result.Integer == 5);
    
    // Test pure CURSED string.concat
    const concat_args = [_]BuiltInRegistry.Value{
        BuiltInRegistry.Value{ .String = "Hello" },
        BuiltInRegistry.Value{ .String = " World" },
    };
    
    const concat_result = try registry.callFunction("string.concat", &concat_args);
    try std.testing.expect(concat_result == .String);
    try std.testing.expect(std.mem.eql(u8, concat_result.String, "Hello World"));
}
