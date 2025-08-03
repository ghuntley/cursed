const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const ArrayList = std.ArrayList;

// Import modules
const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");
const ast = @import("src-zig/ast_simple.zig");
const concurrency = @import("src-zig/concurrency.zig");

/// Simple test suite for CURSED concurrency system
pub const SimpleConcurrencyTest = struct {
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) SimpleConcurrencyTest {
        return SimpleConcurrencyTest{
            .allocator = allocator,
        };
    }
    
    /// Test parsing of goroutine statements (stan keyword)
    pub fn testGoroutineStatementParsing(self: *SimpleConcurrencyTest) !void {
        print("🧪 Testing goroutine statement parsing...\n", .{});
        
        // Test 1: Block form - stan { vibez.spill("Hello") }
        const block_source = "stan { vibez.spill(\"Hello\") }";
        const block_result = try self.parseSource(block_source);
        defer block_result.deinit(self.allocator);
        
        try testing.expect(block_result.statements.items.len == 1);
        switch (block_result.statements.items[0]) {
            .Goroutine => |goroutine| {
                switch (goroutine.call) {
                    .Block => |block| {
                        try testing.expect(block.statements.items.len > 0);
                        print("✅ Block-form goroutine parsed successfully\n", .{});
                    },
                    else => return error.TestFailed,
                }
            },
            else => return error.TestFailed,
        }
    }
    
    /// Test parsing of channel types
    pub fn testChannelParsing(self: *SimpleConcurrencyTest) !void {
        print("🧪 Testing channel parsing...\n", .{});
        
        // Test channel type parsing: sus ch dm<normie>
        const channel_decl_source = "sus ch dm<normie>";
        const channel_result = try self.parseSource(channel_decl_source);
        defer channel_result.deinit(self.allocator);
        
        try testing.expect(channel_result.statements.items.len == 1);
        switch (channel_result.statements.items[0]) {
            .Let => |let_stmt| {
                if (let_stmt.var_type) |var_type| {
                    switch (var_type) {
                        .Channel => {
                            print("✅ Channel type parsed successfully\n", .{});
                        },
                        else => return error.TestFailed,
                    }
                } else {
                    return error.TestFailed;
                }
            },
            else => return error.TestFailed,
        }
    }
    
    /// Test channel operations in the runtime
    pub fn testChannelOperations(self: *SimpleConcurrencyTest) !void {
        print("🧪 Testing channel operations...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        // Create channel
        var channel = try concurrency.makeChannel(i32, self.allocator, 3);
        defer {
            channel.deinit();
            self.allocator.destroy(channel);
        }
        
        // Test send operation
        const send_result = try channel.send(42);
        try testing.expect(send_result == concurrency.SendResult.sent);
        print("✅ Channel send operation successful\n", .{});
        
        // Test receive operation
        const received = try channel.receive();
        try testing.expect(received != null);
        try testing.expect(received.? == 42);
        print("✅ Channel receive operation successful\n", .{});
        
        // Test channel closing
        channel.close();
        try testing.expect(channel.isClosed());
        print("✅ Channel close operation successful\n", .{});
    }
    
    /// Test goroutine spawning and execution
    pub fn testGoroutineExecution(self: *SimpleConcurrencyTest) !void {
        print("🧪 Testing goroutine execution...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        // Test context for goroutine
        var executed = false;
        const TestContext = struct {
            executed: *bool,
        };
        
        var context = TestContext{ .executed = &executed };
        
        const testFn = struct {
            fn run(ctx: ?*anyopaque) void {
                const test_ctx: *TestContext = @ptrCast(@alignCast(ctx.?));
                test_ctx.executed.* = true;
            }
        }.run;
        
        // Spawn goroutine
        const goroutine_id = try concurrency.stan(testFn, &context);
        try testing.expect(goroutine_id > 0);
        
        // Wait for execution
        std.time.sleep(100_000_000); // 100ms
        
        try testing.expect(executed);
        print("✅ Goroutine execution successful\n", .{});
    }
    
    /// Helper function to parse CURSED source code
    fn parseSource(self: *SimpleConcurrencyTest, source: []const u8) !ast.Program {
        var lex = lexer.Lexer.init(self.allocator, source);
        
        const token_list = try lex.tokenize();
        defer token_list.deinit();
        
        var pars = parser.Parser.init(self.allocator, token_list.items);
        return try pars.parseProgram();
    }
    
    /// Run all concurrency tests
    pub fn runAllTests(self: *SimpleConcurrencyTest) !void {
        print("\n🚀 Running Simple CURSED Concurrency Tests\n", .{});
        print("==========================================\n\n", .{});
        
        try self.testGoroutineStatementParsing();
        try self.testChannelParsing();
        try self.testChannelOperations();
        try self.testGoroutineExecution();
        
        print("\n🎉 All simple concurrency tests passed!\n", .{});
        print("✅ Parser: Goroutines and channels\n", .{});
        print("✅ Runtime: Channel operations and goroutine execution\n", .{});
    }
};

// Individual test functions for the Zig test runner
test "goroutine statement parsing" {
    const allocator = testing.allocator;
    var test_suite = SimpleConcurrencyTest.init(allocator);
    try test_suite.testGoroutineStatementParsing();
}

test "channel parsing" {
    const allocator = testing.allocator;
    var test_suite = SimpleConcurrencyTest.init(allocator);
    try test_suite.testChannelParsing();
}

test "channel operations runtime" {
    const allocator = testing.allocator;
    var test_suite = SimpleConcurrencyTest.init(allocator);
    try test_suite.testChannelOperations();
}

test "goroutine execution runtime" {
    const allocator = testing.allocator;
    var test_suite = SimpleConcurrencyTest.init(allocator);
    try test_suite.testGoroutineExecution();
}

// Example usage and demonstration
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = SimpleConcurrencyTest.init(allocator);
    try test_suite.runAllTests();
}
