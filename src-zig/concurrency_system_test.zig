const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const ArrayList = std.ArrayList;

// Import modules
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast_simple.zig");
const concurrency = @import("concurrency.zig");
const concurrency_codegen = @import("codegen_concurrency_implementation.zig");

// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};

/// Test suite for CURSED concurrency system
pub const ConcurrencySystemTest = struct {
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) ConcurrencySystemTest {
        return ConcurrencySystemTest{
            .allocator = allocator,
        };
    }
    
    /// Test parsing of goroutine statements (stan keyword)
    pub fn testGoroutineStatementParsing(self: *ConcurrencySystemTest) !void {
        print("🧪 Testing goroutine statement parsing...\n", .{});
        
        // Test 1: Block form - stan { vibez.spill("Hello from goroutine!") }
        const block_source = "stan { vibez.spill(\"Hello from goroutine!\") }";
        const block_result = try self.parseSource(block_source);
        defer block_result.deinit();
        
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
        
        // Test 2: Expression form - stan doWork()
        const expr_source = "stan doWork()";
        const expr_result = try self.parseSource(expr_source);
        defer expr_result.deinit();
        
        try testing.expect(expr_result.statements.items.len == 1);
        switch (expr_result.statements.items[0]) {
            .Goroutine => |goroutine| {
                switch (goroutine.call) {
                    .Call => {
                        print("✅ Expression-form goroutine parsed successfully\n", .{});
                    },
                    else => return error.TestFailed,
                }
            },
            else => return error.TestFailed,
        }
    }
    
    /// Test parsing of channel types and operations
    pub fn testChannelParsing(self: *ConcurrencySystemTest) !void {
        print("🧪 Testing channel parsing...\n", .{});
        
        // Test channel type parsing: sus ch dm<normie>
        const channel_decl_source = "sus ch dm<normie>";
        const channel_result = try self.parseSource(channel_decl_source);
        defer channel_result.deinit();
        
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
    
    /// Test parsing of select statements (ready keyword)
    pub fn testSelectStatementParsing(self: *ConcurrencySystemTest) !void {
        print("🧪 Testing select statement parsing...\n", .{});
        
        const select_source = 
            \\ready {
            \\    mood ch <- value:
            \\        vibez.spill("Sent")
            \\    mood result := <-ch2:
            \\        vibez.spill("Received")
            \\    basic:
            \\        vibez.spill("Default case")
            \\}
        ;
        
        const select_result = try self.parseSource(select_source);
        defer select_result.deinit();
        
        try testing.expect(select_result.statements.items.len == 1);
        switch (select_result.statements.items[0]) {
            .Select => |select_stmt| {
                try testing.expect(select_stmt.cases.items.len == 2);
                try testing.expect(select_stmt.default_case != null);
                print("✅ Select statement parsed successfully\n", .{});
            },
            else => return error.TestFailed,
        }
    }
    
    /// Test channel operations in the runtime
    pub fn testChannelOperations(self: *ConcurrencySystemTest) !void {
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
    pub fn testGoroutineExecution(self: *ConcurrencySystemTest) !void {
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
        std.Thread.sleep(100_000_000); // 100ms
        
        try testing.expect(executed);
        print("✅ Goroutine execution successful\n", .{});
    }
    
    /// Test select statement functionality
    pub fn testSelectOperations(self: *ConcurrencySystemTest) !void {
        print("🧪 Testing select operations...\n", .{});
        
        // Initialize scheduler
        const config = concurrency.SchedulerConfig.default();
        try concurrency.initializeScheduler(self.allocator, config);
        defer concurrency.shutdownScheduler(self.allocator);
        
        // Create channels
        var ch1 = try concurrency.makeChannel(i32, self.allocator, 1);
        defer {
            ch1.deinit();
            self.allocator.destroy(ch1);
        }
        
        var ch2 = try concurrency.makeChannel(i32, self.allocator, 1);
        defer {
            ch2.deinit();
            self.allocator.destroy(ch2);
        }
        
        // Send on one channel
        _ = try ch1.send(100);
        
        // Create select statement
        var select_stmt = concurrency.Select.init(self.allocator);
        defer select_stmt.deinit();
        
        try select_stmt.addReceive(ch1.id, 0);
        try select_stmt.addReceive(ch2.id, 1);
        try select_stmt.addDefault(2);
        
        // Execute select
        const result = try select_stmt.execute();
        try testing.expect(result == concurrency.SelectResult.receive_completed);
        print("✅ Select operation successful\n", .{});
    }
    
    /// Test concurrency code generation
    pub fn testConcurrencyCodeGen(self: *ConcurrencySystemTest) !void {
        print("🧪 Testing concurrency code generation...\n", .{});
        
        // Set up LLVM context
        const context = c.LLVMContextCreate();
        defer c.LLVMContextDispose(context);
        
        const module = c.LLVMModuleCreateWithNameInContext("test_concurrency", context);
        defer c.LLVMDisposeModule(module);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        // Set up concurrency code generator
        var concurrency_gen = try concurrency_codegen.setupConcurrencyIntegration(context, module, builder, self.allocator);
        
        // Verify runtime functions are available
        try testing.expect(c.LLVMGetNamedFunction(module, "cursed_stan_goroutine") != null);
        try testing.expect(c.LLVMGetNamedFunction(module, "cursed_dm_create") != null);
        try testing.expect(c.LLVMGetNamedFunction(module, "cursed_dm_send") != null);
        try testing.expect(c.LLVMGetNamedFunction(module, "cursed_dm_receive") != null);
        try testing.expect(c.LLVMGetNamedFunction(module, "cursed_ready_select") != null);
        
        print("✅ Concurrency runtime functions declared successfully\n", .{});
        
        // Test channel creation codegen
        const channel_value = try concurrency_gen.generateChannelCreation(ast.Type.Basic, null);
        try testing.expect(channel_value != null);
        print("✅ Channel creation codegen successful\n", .{});
        
        // Ensure no memory leaks
        _ = &concurrency_gen;
    }
    
    /// Test comprehensive concurrency workflow
    pub fn testConcurrencyWorkflow(self: *ConcurrencySystemTest) !void {
        print("🧪 Testing comprehensive concurrency workflow...\n", .{});
        
        // Test parsing → execution workflow
        const concurrency_source = 
            \\sus ch dm<normie> = dm<normie>(5)
            \\
            \\stan {
            \\    ch <- 42
            \\    vibez.spill("Sent value")
            \\}
            \\
            \\stan {
            \\    value := <-ch
            \\    vibez.spill("Received:", value)
            \\}
            \\
            \\ready {
            \\    mood result := <-ch:
            \\        vibez.spill("Got result:", result)
            \\    basic:
            \\        vibez.spill("No data available")
            \\}
        ;
        
        // Parse the program
        const program = try self.parseSource(concurrency_source);
        defer program.deinit();
        
        // Verify parsing results
        var goroutine_count: u32 = 0;
        var select_count: u32 = 0;
        var channel_decl_count: u32 = 0;
        
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Goroutine => goroutine_count += 1,
                .Select => select_count += 1,
                .Let => |let_stmt| {
                    if (let_stmt.var_type) |var_type| {
                        switch (var_type) {
                            .Channel => channel_decl_count += 1,
                            else => {},
                        }
                    }
                },
                else => {},
            }
        }
        
        try testing.expect(goroutine_count == 2);
        try testing.expect(select_count == 1);
        try testing.expect(channel_decl_count == 1);
        
        print("✅ Comprehensive concurrency workflow parsed successfully\n", .{});
        print("   - Goroutines: {}\n", .{goroutine_count});
        print("   - Select statements: {}\n", .{select_count});
        print("   - Channel declarations: {}\n", .{channel_decl_count});
    }
    
    /// Helper function to parse CURSED source code
    fn parseSource(self: *ConcurrencySystemTest, source: []const u8) !ast.Program {
        var lex = lexer.Lexer.init(self.allocator, source);
        defer lex.deinit();
        
        const tokens = try lex.tokenize();
        defer self.allocator.free(tokens);
        
        var pars = parser.Parser.init(self.allocator, tokens);
        return try pars.parseProgram();
    }
    
    /// Run all concurrency tests
    pub fn runAllTests(self: *ConcurrencySystemTest) !void {
        print("\n🚀 Running CURSED Concurrency System Tests\n", .{});
        print("==========================================\n\n", .{});
        
        try self.testGoroutineStatementParsing();
        try self.testChannelParsing();
        try self.testSelectStatementParsing();
        try self.testChannelOperations();
        try self.testGoroutineExecution();
        try self.testSelectOperations();
        try self.testConcurrencyCodeGen();
        try self.testConcurrencyWorkflow();
        
        print("\n🎉 All concurrency tests passed!\n", .{});
        print("✅ Parser: Goroutines, channels, and select statements\n", .{});
        print("✅ Runtime: Channel operations and goroutine execution\n", .{});
        print("✅ Codegen: LLVM IR generation for concurrency constructs\n", .{});
        print("✅ Integration: Complete concurrency workflow\n\n", .{});
    }
};

// Individual test functions for the Zig test runner
test "goroutine statement parsing" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testGoroutineStatementParsing();
}

test "channel parsing" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testChannelParsing();
}

test "select statement parsing" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testSelectStatementParsing();
}

test "channel operations runtime" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testChannelOperations();
}

test "goroutine execution runtime" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testGoroutineExecution();
}

test "select operations runtime" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testSelectOperations();
}

test "concurrency code generation" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testConcurrencyCodeGen();
}

test "comprehensive concurrency workflow" {
    const allocator = testing.allocator;
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.testConcurrencyWorkflow();
}

// Example usage and demonstration
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = ConcurrencySystemTest.init(allocator);
    try test_suite.runAllTests();
}
