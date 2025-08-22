//! Enhanced CURSED Interpreter with Error Handling and Concurrency
//! Integrates yikes/fam/shook and stan/dm systems

const std = @import("std");
const Allocator = std.mem.Allocator;
const integration = @import("error_concurrency_integration.zig");

/// Enhanced interpreter that supports error handling and concurrency
pub const EnhancedInterpreter = struct {
    const Self = @This();
    
    allocator: Allocator,
    runtime: *integration.UnifiedRuntime,
    variables: std.StringHashMap(integration.InterpreterValue),
    
    pub fn init(allocator: Allocator) !*Self {
        const interpreter = try allocator.create(Self);
        interpreter.* = Self{
            .allocator = allocator,
            .runtime = try integration.UnifiedRuntime.init(allocator),
            .variables = std.StringHashMap(integration.InterpreterValue).init(allocator),
        };
        
        return interpreter;
    }
    
    pub fn deinit(self: *Self) void {
        self.variables.deinit();
        self.runtime.deinit();
        self.allocator.destroy(self);
    }
    
    /// Execute yikes statement: `yikes "message"`
    pub fn executeYikes(self: *Self, message: []const u8) !integration.InterpreterValue {
        return integration.executeYikesStatement(
            self.runtime,
            message,
            .Runtime,
            1000
        );
    }
    
    /// Execute shook expression: `expression shook`
    pub fn executeShook(self: *Self, value: integration.InterpreterValue) integration.InterpreterValue {
        return integration.executeShookExpression(self.runtime, value);
    }
    
    /// Execute fam statement: `fam { try_code } sus error { catch_code }`
    pub fn executeFam(self: *Self, try_fn: anytype, catch_fn: anytype) integration.InterpreterValue {
        const TryFn = @TypeOf(try_fn);
        const CatchFn = @TypeOf(catch_fn);
        
        const try_wrapper = struct {
            fn call(try_closure: TryFn) integration.InterpreterValue {
                return try_closure();
            }
        }.call;
        
        const catch_wrapper = struct {
            fn call(catch_closure: CatchFn, error_obj: *integration.advanced_error_handling.CursedError) integration.InterpreterValue {
                return catch_closure(error_obj);
            }
        }.call;
        
        return integration.executeFamStatement(
            self.runtime,
            struct {
                try_fn_local: TryFn,
                
                pub fn tryBlock(ctx: @This()) integration.InterpreterValue {
                    return try_wrapper(ctx.try_fn_local);
                }
            }{ .try_fn_local = try_fn }.tryBlock,
            struct {
                catch_fn_local: CatchFn,
                
                pub fn catchBlock(ctx: @This(), error_obj: *integration.advanced_error_handling.CursedError) integration.InterpreterValue {
                    return catch_wrapper(ctx.catch_fn_local, error_obj);
                }
            }{ .catch_fn_local = catch_fn }.catchBlock
        );
    }
    
    /// Execute stan statement: `stan function(args)`
    pub fn executeStan(self: *Self, entry_fn: integration.advanced_concurrency.GoroutineEntry, context: ?*anyopaque) !integration.InterpreterValue {
        return integration.executeStanStatement(self.runtime, entry_fn, context);
    }
    
    /// Create dm channel: `sus ch dm<type>[capacity]`
    pub fn createChannel(self: *Self, capacity: usize) !integration.InterpreterValue {
        return integration.createDmChannel(self.runtime, i64, capacity);
    }
    
    /// Send to channel: `dm_send(channel, value)`
    pub fn channelSend(self: *Self, channel: integration.InterpreterValue, value: integration.InterpreterValue) !integration.InterpreterValue {
        return integration.dmSendOperation(self.runtime, channel, value);
    }
    
    /// Receive from channel: `dm_recv(channel)`
    pub fn channelRecv(self: *Self, channel: integration.InterpreterValue) !integration.InterpreterValue {
        return integration.dmRecvOperation(self.runtime, channel);
    }
    
    /// Store variable
    pub fn setVariable(self: *Self, name: []const u8, value: integration.InterpreterValue) !void {
        const owned_name = try self.allocator.dupe(u8, name);
        try self.variables.put(owned_name, value);
    }
    
    /// Get variable
    pub fn getVariable(self: *Self, name: []const u8) ?integration.InterpreterValue {
        return self.variables.get(name);
    }
    
    /// Evaluate simple expressions for demo
    pub fn evaluateExpression(self: *Self, expr: []const u8) !integration.InterpreterValue {
        // Simple expression parser for demo purposes
        if (std.mem.eql(u8, expr, "yikes")) {
            return self.executeYikes("Demo error");
        } else if (std.mem.startsWith(u8, expr, "yikes \"")) {
            const start = 7; // Skip 'yikes "'
            const end = std.mem.lastIndexOf(u8, expr, "\"") orelse expr.len;
            const message = expr[start..end];
            return self.executeYikes(message);
        } else if (std.mem.eql(u8, expr, "42")) {
            return integration.InterpreterValue{ .Integer = 42 };
        } else if (self.getVariable(expr)) |value| {
            return value;
        } else {
            return integration.InterpreterValue.Null;
        }
    }
};

/// Demo function that shows error handling in action
fn demoErrorHandling(interpreter: *EnhancedInterpreter) !void {
    std.debug.print("=== Error Handling Demo ===\n");
    
    // Test yikes
    const error_result = try interpreter.executeYikes("This is a test error");
    std.debug.print("Created error: {}\n", .{error_result});
    
    // Test shook
    const shook_result = interpreter.executeShook(error_result);
    std.debug.print("Shook result: {}\n", .{shook_result});
    
    // Test fam with success
    const success_result = interpreter.executeFam(
        struct {
            fn tryIt() integration.InterpreterValue {
                return integration.InterpreterValue{ .Integer = 100 };
            }
        }.tryIt,
        struct {
            fn catchIt(error_obj: *integration.advanced_error_handling.CursedError) integration.InterpreterValue {
                std.debug.print("Caught error: {}\n", .{error_obj.*});
                return integration.InterpreterValue{ .String = "Error handled" };
            }
        }.catchIt
    );
    std.debug.print("Fam success result: {}\n", .{success_result});
    
    // Test fam with error
    const error_fam_result = interpreter.executeFam(
        struct {
            fn tryIt() integration.InterpreterValue {
                const runtime = integration.getUnifiedRuntime();
                const error_obj = runtime.error_runtime.executeYikes(
                    "Intentional error in fam block",
                    .Runtime,
                    4001
                ) catch unreachable;
                return integration.InterpreterValue{ .Error = error_obj };
            }
        }.tryIt,
        struct {
            fn catchIt(error_obj: *integration.advanced_error_handling.CursedError) integration.InterpreterValue {
                std.debug.print("Fam caught error: {}\n", .{error_obj.*});
                return integration.InterpreterValue{ .String = "Recovered from fam error" };
            }
        }.catchIt
    );
    std.debug.print("Fam error result: {}\n", .{error_fam_result});
}

/// Demo function that shows concurrency in action
fn demoConcurrency(interpreter: *EnhancedInterpreter) !void {
    std.debug.print("\n=== Concurrency Demo ===\n");
    
    // Create a channel
    const channel = try interpreter.createChannel(5);
    std.debug.print("Created channel: {}\n", .{channel});
    
    // Test channel operations
    const send_result = try interpreter.channelSend(channel, integration.InterpreterValue{ .Integer = 123 });
    std.debug.print("Send result: {}\n", .{send_result});
    
    const recv_result = try interpreter.channelRecv(channel);
    std.debug.print("Received: {}\n", .{recv_result});
    
    // Test goroutine spawning
    var shared_counter: i32 = 0;
    const goroutine_result = try interpreter.executeStan(demoGoroutineEntry, &shared_counter);
    std.debug.print("Spawned goroutine: {}\n", .{goroutine_result});
    
    // Give goroutine time to execute
    std.time.sleep(100_000_000); // 100ms
    std.debug.print("Counter after goroutine: {}\n", .{shared_counter});
}

fn demoGoroutineEntry(context: ?*anyopaque) void {
    if (context) |ctx| {
        const counter_ptr = @as(*i32, @ptrCast(@alignCast(ctx)));
        for (0..10) |i| {
            counter_ptr.* += 1;
            std.debug.print("Goroutine iteration {}, counter: {}\n", .{ i, counter_ptr.* });
            std.time.sleep(10_000_000); // 10ms
        }
        std.debug.print("Goroutine completed\n");
    }
}

/// Interactive demo runner
fn runInteractiveDemo(interpreter: *EnhancedInterpreter) !void {
    std.debug.print("\n=== Interactive Demo ===\n");
    std.debug.print("Try these commands:\n");
    std.debug.print("  yikes \"custom error message\"\n");
    std.debug.print("  42\n");
    std.debug.print("  exit\n");
    
    const stdin = std.io.getStdIn().reader();
    var buf: [256]u8 = undefined;
    
    while (true) {
        std.debug.print("cursed> ");
        
        if (try stdin.readUntilDelimiterOrEof(buf[0..], '\n')) |input| {
            const trimmed = std.mem.trim(u8, input, " \t\r\n");
            
            if (std.mem.eql(u8, trimmed, "exit")) {
                break;
            }
            
            const result = interpreter.evaluateExpression(trimmed) catch |err| {
                std.debug.print("Error evaluating expression: {}\n", .{err});
                continue;
            };
            
            std.debug.print("=> {}\n", .{result});
        } else {
            break;
        }
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("Enhanced CURSED Interpreter with Error Handling and Concurrency\n");
    std.debug.print("================================================================\n");
    
    const interpreter = try EnhancedInterpreter.init(allocator);
    defer interpreter.deinit();
    
    // Run demos
    try demoErrorHandling(interpreter);
    try demoConcurrency(interpreter);
    try runInteractiveDemo(interpreter);
    
    std.debug.print("\nEnhanced interpreter demo complete!\n");
}
