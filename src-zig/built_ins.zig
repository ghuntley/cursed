const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// CURSED Built-in Functions Implementation
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

    pub fn init(allocator: Allocator) BuiltInRegistry {
        var registry = BuiltInRegistry{
            .allocator = allocator,
            .functions = std.StringHashMap(BuiltInFunction).init(allocator),
        };
        
        registry.registerBuiltIns() catch unreachable;
        return registry;
    }

    pub fn deinit(self: *BuiltInRegistry) void {
        self.functions.deinit();
    }

    fn registerBuiltIns(self: *BuiltInRegistry) !void {
        // Register vibez.spill (print function)
        try self.functions.put("vibez.spill", BuiltInFunction{
            .name = "vibez.spill",
            .implementation = vibesSpill,
            .arg_count = 1,
        });

        // Register make function for creating channels and data structures
        try self.functions.put("make", BuiltInFunction{
            .name = "make",
            .implementation = makeFunction,
            .arg_count = 1, // make(chan, capacity) or make(type)
        });

        // Register string operations
        try self.functions.put("string.concat", BuiltInFunction{
            .name = "string.concat",
            .implementation = stringConcat,
            .arg_count = 2,
        });

        try self.functions.put("string.length", BuiltInFunction{
            .name = "string.length",
            .implementation = stringLength,
            .arg_count = 1,
        });

        // Register math functions
        try self.functions.put("math.add", BuiltInFunction{
            .name = "math.add",
            .implementation = mathAdd,
            .arg_count = 2,
        });

        try self.functions.put("math.multiply", BuiltInFunction{
            .name = "math.multiply",
            .implementation = mathMultiply,
            .arg_count = 2,
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

    // Built-in function implementations

    fn vibesSpill(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        const allocator = self.allocator;
        const str = try args[0].toString(allocator);
        defer allocator.free(str);
        
        std.debug.print("{s}\n", .{str});
        return Value.Null;
    }

    fn makeFunction(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        // For now, assume we're making a channel with capacity
        switch (args[0]) {
            .Integer => |capacity| {
                const allocator = self.allocator;
                const channel = try allocator.create(Channel);
                channel.* = Channel.init(allocator, @intCast(capacity));
                return Value{ .Channel = channel };
            },
            else => return error.TypeMismatch,
        }
    }

    fn stringConcat(args: []const Value) anyerror!Value {
        if (args.len != 2) return error.ArgumentCountMismatch;
        
        const allocator = self.allocator;
        
        const str1 = switch (args[0]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        const str2 = switch (args[1]) {
            .String => |s| s,
            else => return error.TypeMismatch,
        };
        
        const result = try std.fmt.allocPrint(allocator, "{s}{s}", .{ str1, str2 });
        return Value{ .String = result };
    }

    fn stringLength(args: []const Value) anyerror!Value {
        if (args.len != 1) return error.ArgumentCountMismatch;
        
        switch (args[0]) {
            .String => |s| return Value{ .Integer = @intCast(s.len) },
            else => return error.TypeMismatch,
        }
    }

    fn mathAdd(args: []const Value) anyerror!Value {
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

    fn mathMultiply(args: []const Value) anyerror!Value {
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
};

// Test built-in functions
test "built-in functions" {
    const allocator = std.testing.allocator;
    
    var registry = BuiltInRegistry.init(allocator);
    defer registry.deinit();
    
    // Test math.add
    const args = [_]BuiltInRegistry.Value{
        BuiltInRegistry.Value{ .Integer = 5 },
        BuiltInRegistry.Value{ .Integer = 3 },
    };
    
    const result = try registry.callFunction("math.add", &args);
    try std.testing.expect(result == .Integer);
    try std.testing.expect(result.Integer == 8);
    
    // Test string.length
    const str_args = [_]BuiltInRegistry.Value{
        BuiltInRegistry.Value{ .String = "Hello" },
    };
    
    const str_result = try registry.callFunction("string.length", &str_args);
    try std.testing.expect(str_result == .Integer);
    try std.testing.expect(str_result.Integer == 5);
}
