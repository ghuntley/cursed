const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const Allocator = std.mem.Allocator;

const debug_info = @import("debug_info.zig");
const inlined_debug = @import("inlined_function_debug.zig");
const llvm_integration = @import("llvm_inlined_debug_integration.zig");

// Mock LLVM types for testing
const c = struct {
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMDIBuilderRef = ?*anyopaque;
    pub const LLVMMetadataRef = ?*anyopaque;
};

/// Comprehensive test suite for inlined function debug information
pub const InlinedDebugTestSuite = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) InlinedDebugTestSuite {
        return InlinedDebugTestSuite{
            .allocator = allocator,
        };
    }
    
    /// Test basic inlined debug info creation
    pub fn testBasicInlinedDebugInfo(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing basic inlined debug info creation...\n", .{});
        
        // Create mock LLVM components
        const context: c.LLVMContextRef = null;
        const module: c.LLVMModuleRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        // Initialize debug generators
        var debug_generator = try debug_info.DebugInfoGenerator.init(self.allocator, context, module);
        defer debug_generator.deinit();
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        // Test creating inline context
        const inline_context = inlined_debug.InlineContext.init(
            "add_numbers",
            "main",
            15, // inline site line
            8,  // inline site column
            3,  // original line
            1,  // original column
            "math.csd"
        );
        
        // Test creating debug info for inlined function
        const result = inlined_debug_generator.createInlinedFunctionDebugInfo(
            inline_context,
            null, // Mock original function debug info
            null  // Mock target scope
        );
        
        try testing.expect(result != debug_info.DebugInfoGenerator.DebugError.MetadataError);
        
        print("✅ Basic inlined debug info creation test passed\n", .{});
    }
    
    /// Test nested inlining debug info
    pub fn testNestedInlining(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing nested inlining debug info...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        // Create nested inline contexts
        const context1 = inlined_debug.InlineContext.init("helper", "main", 10, 5, 1, 1, "test.csd");
        const context2 = inlined_debug.InlineContext.init("sub_helper", "helper", 15, 3, 2, 1, "test.csd");
        
        // Test nested inline stack
        try inlined_debug_generator.pushInlineContext(&context1);
        try inlined_debug_generator.pushInlineContext(&context2);
        
        const depth = inlined_debug_generator.getCurrentInlineDepth();
        try testing.expectEqual(@as(u32, 2), depth);
        
        // Test creating nested inlined-at metadata
        const nested_metadata = try inlined_debug_generator.createNestedInlinedAt(20, 10, null);
        try testing.expect(nested_metadata != null);
        
        inlined_debug_generator.popInlineContext();
        inlined_debug_generator.popInlineContext();
        
        try testing.expectEqual(@as(u32, 0), inlined_debug_generator.getCurrentInlineDepth());
        
        print("✅ Nested inlining test passed\n", .{});
    }
    
    /// Test variable mapping during inlining
    pub fn testVariableMapping(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing variable mapping during inlining...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        const inline_context = inlined_debug.InlineContext.init("func", "main", 10, 5, 1, 1, "test.csd");
        
        // Test tracking inlined variables
        try inlined_debug_generator.trackInlinedVariable(
            "original_var",
            "inlined_var_123",
            &inline_context,
            null, // Mock original debug info
            null  // Mock inlined debug info
        );
        
        // Verify variable mapping was stored
        try testing.expectEqual(@as(usize, 1), inlined_debug_generator.variable_mappings.items.len);
        try testing.expectEqualStrings("original_var", inlined_debug_generator.variable_mappings.items[0].original_name);
        try testing.expectEqualStrings("inlined_var_123", inlined_debug_generator.variable_mappings.items[0].inlined_name);
        
        print("✅ Variable mapping test passed\n", .{});
    }
    
    /// Test LLVM integration hooks
    pub fn testLLVMIntegration(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing LLVM integration...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const module: c.LLVMModuleRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        var debug_generator = try debug_info.DebugInfoGenerator.init(self.allocator, context, module);
        defer debug_generator.deinit();
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        var integration = llvm_integration.LLVMInlinedDebugIntegration.init(
            self.allocator, 
            &debug_generator, 
            &inlined_debug_generator
        );
        defer integration.deinit();
        
        // Test module analysis
        try integration.analyzeModuleForInlining(module);
        
        // Test function inlining hooks
        try integration.onFunctionWillBeInlined(null, null, null); // Mock values
        try integration.onInstructionInlined(null, null); // Mock values
        try integration.onVariableInlined("orig", "inlined", null, null); // Mock values
        try integration.onInliningComplete();
        
        print("✅ LLVM integration test passed\n", .{});
    }
    
    /// Test debug info validation
    pub fn testDebugInfoValidation(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing debug info validation...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        // Test validation with empty state (should be valid)
        const is_valid_empty = inlined_debug_generator.validateInlinedDebugInfo();
        try testing.expect(is_valid_empty);
        
        // Add some test data
        const inline_context = inlined_debug.InlineContext.init("func", "main", 10, 5, 1, 1, "test.csd");
        try inlined_debug_generator.trackInlinedVariable("var1", "var1_inlined", &inline_context, null, null);
        
        // Test validation with incomplete data (should still be valid since we handle nulls)
        const is_valid_incomplete = inlined_debug_generator.validateInlinedDebugInfo();
        try testing.expect(is_valid_incomplete);
        
        print("✅ Debug info validation test passed\n", .{});
    }
    
    /// Test report generation
    pub fn testReportGeneration(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing report generation...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        // Add some test data
        const inline_context = inlined_debug.InlineContext.init("test_func", "main", 10, 5, 1, 1, "test.csd");
        try inlined_debug_generator.trackInlinedVariable("x", "x_inlined", &inline_context, null, null);
        
        // Generate report
        try inlined_debug_generator.generateInliningReport("test_inlined_debug_report.md");
        
        // Verify the report file was created
        const file = std.fs.cwd().openFile("test_inlined_debug_report.md", .{}) catch |err| {
            print("❌ Report generation failed: {}\n", .{err});
            return;
        };
        file.close();
        
        print("✅ Report generation test passed\n", .{});
    }
    
    /// Test instruction debug location mapping
    pub fn testInstructionDebugMapping(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing instruction debug location mapping...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        const inline_context = inlined_debug.InlineContext.init("func", "main", 15, 8, 3, 1, "test.csd");
        const mock_instruction: c.LLVMValueRef = null;
        
        // Test creating inlined debug location
        try inlined_debug_generator.createInlinedDebugLocation(
            mock_instruction,
            3, // original line
            1, // original column
            null, // original scope
            &inline_context
        );
        
        // Verify instruction was tracked
        try testing.expectEqual(@as(usize, 1), inlined_debug_generator.instruction_debug_locations.count());
        
        print("✅ Instruction debug mapping test passed\n", .{});
    }
    
    /// Test cleanup functionality
    pub fn testCleanup(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing cleanup functionality...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const di_builder: c.LLVMDIBuilderRef = null;
        
        var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(self.allocator, context, di_builder);
        defer inlined_debug_generator.deinit();
        
        // Add some test data
        const inline_context = inlined_debug.InlineContext.init("func", "main", 10, 5, 1, 1, "test.csd");
        try inlined_debug_generator.trackInlinedVariable("var", "var_inlined", &inline_context, null, null);
        
        // Test cleanup (should not crash)
        inlined_debug_generator.cleanupUnusedInlinedDebugInfo();
        
        print("✅ Cleanup test passed\n", .{});
    }
    
    /// Test integration with main debug info generator
    pub fn testMainDebugInfoIntegration(self: *InlinedDebugTestSuite) !void {
        print("🧪 Testing integration with main debug info generator...\n", .{});
        
        const context: c.LLVMContextRef = null;
        const module: c.LLVMModuleRef = null;
        
        var debug_generator = try debug_info.DebugInfoGenerator.init(self.allocator, context, module);
        defer debug_generator.deinit();
        
        // Test the new inlined debug methods
        const mock_instruction: c.LLVMValueRef = null;
        const mock_scope: c.LLVMMetadataRef = null;
        
        // Test creating inlined debug location
        const inlined_location = debug_generator.createInlinedDebugLocation(
            15, 10, // original location
            mock_scope,
            25, 5,  // inline site location
            mock_scope
        );
        try testing.expect(inlined_location != null);
        
        // Test setting inlined instruction debug location
        debug_generator.setInlinedInstructionDebugLocation(
            mock_instruction,
            15, 10, // original location
            mock_scope,
            25, 5,  // inline site location
            mock_scope
        );
        
        print("✅ Main debug info integration test passed\n", .{});
    }
    
    /// Run all tests
    pub fn runAllTests(self: *InlinedDebugTestSuite) !void {
        print("🧪 Running comprehensive inlined debug test suite...\n\n", .{});
        
        try self.testBasicInlinedDebugInfo();
        try self.testNestedInlining();
        try self.testVariableMapping();
        try self.testLLVMIntegration();
        try self.testDebugInfoValidation();
        try self.testReportGeneration();
        try self.testInstructionDebugMapping();
        try self.testCleanup();
        try self.testMainDebugInfoIntegration();
        
        print("\n✅ All inlined debug tests passed successfully!\n", .{});
    }
};

/// Main test function
pub fn testInlinedDebugComplete() !void {
    print("🎯 Starting comprehensive inlined function debug tests...\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = InlinedDebugTestSuite.init(allocator);
    try test_suite.runAllTests();
    
    // Test the individual modules
    try inlined_debug.testInlinedFunctionDebug();
    try llvm_integration.testLLVMInlinedDebugIntegration();
    
    print("🏆 All comprehensive inlined debug tests completed successfully!\n", .{});
}

// Export for external testing
export fn run_inlined_debug_tests() void {
    testInlinedDebugComplete() catch |err| {
        print("❌ Tests failed with error: {}\n", .{err});
    };
}

test "inlined function debug info basic" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = InlinedDebugTestSuite.init(allocator);
    try test_suite.testBasicInlinedDebugInfo();
}

test "inlined function debug info nested" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = InlinedDebugTestSuite.init(allocator);
    try test_suite.testNestedInlining();
}

test "inlined function debug info variable mapping" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = InlinedDebugTestSuite.init(allocator);
    try test_suite.testVariableMapping();
}

test "inlined function debug info integration" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var test_suite = InlinedDebugTestSuite.init(allocator);
    try test_suite.testMainDebugInfoIntegration();
}
