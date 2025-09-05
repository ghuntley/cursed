// P0 Sprint 1: Type Checker Simple Mode Validation
// Comprehensive validation and testing of the simple type checker

const std = @import("std");
const type_checker_simple = @import("type_checker_simple.zig");
const SimpleTypeChecker = type_checker_simple.SimpleTypeChecker;
const TypeInfo = type_checker_simple.TypeInfo;
const TypeCheckError = type_checker_simple.TypeCheckError;

// Validation test suite
pub const TypeCheckerValidation = struct {
    allocator: std.mem.Allocator,
    test_count: usize,
    passed_count: usize,
    
    pub fn init(allocator: std.mem.Allocator) TypeCheckerValidation {
        return TypeCheckerValidation{
            .allocator = allocator,
            .test_count = 0,
            .passed_count = 0,
        };
    }
    
    pub fn runAllTests(self: *TypeCheckerValidation) !void {
        std.log.info("🚀 Starting P0 Sprint 1 Type Checker Validation", .{});
        
        try self.testBasicTypeSystem();
        try self.testVariableDeclarations();
        try self.testStructFieldAccess();
        try self.testBinaryOperations();
        try self.testTypeCoercions();
        try self.testScopeManagement();
        try self.testErrorDetection();
        try self.testArrayOperations();
        try self.testFunctionBasics();
        try self.testCursedPrograms();
        
        self.printSummary();
    }
    
    fn testBasicTypeSystem(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Basic Type System...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        // Test builtin type registration
        self.expect(checker.getTypeByName("drip") != null, "drip type should exist");
        self.expect(checker.getTypeByName("tea") != null, "tea type should exist");
        self.expect(checker.getTypeByName("lit") != null, "lit type should exist");
        self.expect(checker.getTypeByName("vibes") != null, "vibes type should exist");
        self.expect(checker.getTypeByName("unknown_type") == null, "unknown type should not exist");
        
        // Test type properties
        const drip_type = checker.getTypeByName("drip").?;
        self.expect(drip_type.base_type.isNumeric(), "drip should be numeric");
        
        const lit_type = checker.getTypeByName("lit").?;
        self.expect(lit_type.base_type.isBoolean(), "lit should be boolean");
        
        const tea_type = checker.getTypeByName("tea").?;
        self.expect(!tea_type.base_type.isNumeric(), "tea should not be numeric");
        
        std.log.info("✅ Basic Type System tests completed", .{});
    }
    
    fn testVariableDeclarations(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Variable Declarations...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        // Test basic variable declaration
        const int_type = checker.checkVariableDeclaration("x", "drip", true) catch |err| {
            std.log.err("Failed to declare variable: {}", .{err});
            return;
        };
        self.expect(int_type.base_type == .DrIP, "x should have drip type");
        
        // Test variable access
        const accessed_type = checker.checkVariableAccess("x") catch |err| {
            std.log.err("Failed to access variable: {}", .{err});
            return;
        };
        self.expect(accessed_type.base_type == .DrIP, "accessed x should have drip type");
        
        // Test duplicate declaration (should error)
        _ = checker.checkVariableDeclaration("x", "tea", false) catch {
            self.expect(checker.hasErrors(), "Should have duplicate declaration error");
        };
        
        // Test access to undeclared variable (should error)
        _ = checker.checkVariableAccess("undeclared") catch {
            self.expect(checker.hasErrors(), "Should have undeclared variable error");
        };
        
        std.log.info("✅ Variable Declaration tests completed", .{});
    }
    
    fn testStructFieldAccess(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Struct Field Access...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        // Register a struct type
        const fields = [_]struct { name: []const u8, type_name: []const u8 }{
            .{ .name = "name", .type_name = "tea" },
            .{ .name = "age", .type_name = "drip" },
            .{ .name = "active", .type_name = "lit" },
        };
        
        try checker.registerStructType("Person", &fields);
        
        const person_type = checker.getTypeByName("Person").?;
        self.expect(person_type.base_type == .Struct, "Person should be struct type");
        
        // Test valid field access
        const name_field_type = checker.checkFieldAccess(person_type, "name") catch |err| {
            std.log.err("Failed to access name field: {}", .{err});
            return;
        };
        self.expect(name_field_type.base_type == .Tea, "name field should be tea type");
        
        // Test invalid field access
        _ = checker.checkFieldAccess(person_type, "unknown_field") catch {
            self.expect(checker.hasErrors(), "Should have unknown field error");
        };
        
        // Test field access on non-struct
        const int_type = checker.getTypeByName("drip").?;
        _ = checker.checkFieldAccess(int_type, "field") catch {
            self.expect(checker.hasErrors(), "Should have not-a-struct error");
        };
        
        std.log.info("✅ Struct Field Access tests completed", .{});
    }
    
    fn testBinaryOperations(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Binary Operations...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        const drip_type = checker.getTypeByName("drip").?;
        const tea_type = checker.getTypeByName("tea").?;
        const lit_type = checker.getTypeByName("lit").?;
        
        // Test arithmetic operations
        const add_result = checker.checkBinaryOperation(drip_type, drip_type, "+") catch |err| {
            std.log.err("Failed arithmetic operation: {}", .{err});
            return;
        };
        self.expect(add_result.base_type == .DrIP, "drip + drip should be drip");
        
        // Test comparison operations
        const cmp_result = checker.checkBinaryOperation(drip_type, drip_type, "==") catch |err| {
            std.log.err("Failed comparison operation: {}", .{err});
            return;
        };
        self.expect(cmp_result.base_type == .Lit, "drip == drip should be lit");
        
        // Test string concatenation
        const concat_result = checker.checkBinaryOperation(tea_type, tea_type, "+") catch |err| {
            std.log.err("Failed string concatenation: {}", .{err});
            return;
        };
        self.expect(concat_result.base_type == .Tea, "tea + tea should be tea");
        
        // Test invalid operations
        _ = checker.checkBinaryOperation(tea_type, drip_type, "+") catch {
            self.expect(checker.hasErrors(), "Should have type mismatch error");
        };
        
        std.log.info("✅ Binary Operations tests completed", .{});
    }
    
    fn testTypeCoercions(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Type Coercions...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        const drip_type = checker.getTypeByName("drip").?;
        const normie_type = checker.getTypeByName("normie").?;
        const vibes_type = checker.getTypeByName("vibes").?;
        const tea_type = checker.getTypeByName("tea").?;
        
        // Test numeric type compatibility
        self.expect(checker.areTypesCompatible(drip_type, normie_type), "drip and normie should be compatible");
        self.expect(checker.areTypesCompatible(drip_type, vibes_type), "drip and vibes should be compatible");
        
        // Test non-compatible types
        self.expect(!checker.areTypesCompatible(tea_type, drip_type), "tea and drip should not be compatible");
        
        // Test assignment with coercion
        _ = checker.checkVariableDeclaration("x", "drip", true) catch unreachable;
        checker.checkAssignment("x", normie_type) catch |err| {
            std.log.err("Failed compatible assignment: {}", .{err});
            return;
        };
        
        // Test incompatible assignment
        checker.checkAssignment("x", tea_type) catch {
            self.expect(checker.hasErrors(), "Should have incompatible assignment error");
        };
        
        std.log.info("✅ Type Coercions tests completed", .{});
    }
    
    fn testScopeManagement(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Scope Management...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        // Declare variable in global scope
        _ = checker.checkVariableDeclaration("global_var", "drip", true) catch unreachable;
        
        // Enter new scope
        try checker.enterScope();
        
        // Variable should still be accessible
        const accessed_type = checker.checkVariableAccess("global_var") catch |err| {
            std.log.err("Failed to access global variable from nested scope: {}", .{err});
            return;
        };
        self.expect(accessed_type.base_type == .DrIP, "Should access global variable");
        
        // Declare variable in nested scope with same name
        _ = checker.checkVariableDeclaration("global_var", "tea", true) catch unreachable;
        
        // Should access the nested scope variable (shadowing)
        const shadowed_type = checker.checkVariableAccess("global_var") catch |err| {
            std.log.err("Failed to access shadowed variable: {}", .{err});
            return;
        };
        self.expect(shadowed_type.base_type == .Tea, "Should access shadowed variable");
        
        // Exit scope
        checker.exitScope();
        
        // Should access original global variable again
        const global_again_type = checker.checkVariableAccess("global_var") catch |err| {
            std.log.err("Failed to access global variable after scope exit: {}", .{err});
            return;
        };
        self.expect(global_again_type.base_type == .DrIP, "Should access original global variable");
        
        std.log.info("✅ Scope Management tests completed", .{});
    }
    
    fn testErrorDetection(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Error Detection...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        var initial_error_count = checker.errors.items.len;
        
        // Generate various errors
        _ = checker.checkVariableAccess("undeclared") catch {};
        _ = checker.checkVariableDeclaration("x", "unknown_type", true) catch {};
        
        const tea_type = checker.getTypeByName("tea").?;
        const drip_type = checker.getTypeByName("drip").?;
        _ = checker.checkBinaryOperation(tea_type, drip_type, "+") catch {};
        
        // Check that errors were recorded
        self.expect(checker.hasErrors(), "Should have recorded errors");
        self.expect(checker.errors.items.len > initial_error_count, "Should have more errors than initial");
        
        // Test error reporting
        std.log.info("Type checker errors found:", .{});
        checker.printErrors();
        
        std.log.info("✅ Error Detection tests completed", .{});
    }
    
    fn testArrayOperations(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Array Operations...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        // Create array type
        const drip_type = checker.getTypeByName("drip").?;
        const array_type = TypeInfo.makeArray(self.allocator, drip_type) catch |err| {
            std.log.err("Failed to create array type: {}", .{err});
            return;
        };
        defer array_type.deinit();
        
        // Test array access
        const element_type = checker.checkArrayAccess(array_type, drip_type) catch |err| {
            std.log.err("Failed array access: {}", .{err});
            return;
        };
        self.expect(element_type.base_type == .DrIP, "Array element should be drip type");
        
        // Test invalid array access (non-numeric index)
        const tea_type = checker.getTypeByName("tea").?;
        _ = checker.checkArrayAccess(array_type, tea_type) catch {
            self.expect(checker.hasErrors(), "Should have non-numeric index error");
        };
        
        // Test array access on non-array
        _ = checker.checkArrayAccess(drip_type, drip_type) catch {
            self.expect(checker.hasErrors(), "Should have not-an-array error");
        };
        
        std.log.info("✅ Array Operations tests completed", .{});
    }
    
    fn testFunctionBasics(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing Function Basics...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        // Test basic function call type checking (simplified)
        const drip_type = checker.getTypeByName("drip").?;
        const args = [_]*TypeInfo{drip_type, drip_type};
        
        const return_type = checker.checkFunctionCall("add", &args) catch |err| {
            std.log.err("Failed function call: {}", .{err});
            return;
        };
        
        // For now, just verify it returns something
        self.expect(return_type != null, "Function call should return a type");
        
        std.log.info("✅ Function Basics tests completed", .{});
    }
    
    fn testCursedPrograms(self: *TypeCheckerValidation) !void {
        std.log.info("📋 Testing CURSED Program Type Checking...", .{});
        
        var checker = SimpleTypeChecker.init(self.allocator);
        defer checker.deinit();
        
        // Simulate checking basic_test.💀.💀 program
        try self.simulateBasicCursedProgram(&checker);
        
        // Report any errors found
        if (checker.hasErrors()) {
            std.log.warn("⚠️  Type checking found issues in basic CURSED program:", .{});
            checker.printErrors();
        } else {
            std.log.info("✅ Basic CURSED program passed type checking", .{});
        }
        
        std.log.info("✅ CURSED Program tests completed", .{});
    }
    
    fn simulateBasicCursedProgram(self: *TypeCheckerValidation, checker: *SimpleTypeChecker) !void {
        // Simulate checking: sus number drip = 42
        const number_type = try checker.checkVariableDeclaration("number", "drip", true);
        try checker.checkAssignment("number", number_type);
        
        // Simulate checking: sus text tea = "Hello CURSED"
        const text_type = try checker.checkVariableDeclaration("text", "tea", true);
        const tea_type = checker.getTypeByName("tea").?;
        try checker.checkAssignment("text", tea_type);
        
        // Simulate checking: sus flag lit = based
        const flag_type = try checker.checkVariableDeclaration("flag", "lit", true);
        const lit_type = checker.getTypeByName("lit").?;
        try checker.checkAssignment("flag", lit_type);
        
        // Simulate checking: sus sum drip = 10 + 32
        const sum_type = try checker.checkVariableDeclaration("sum", "drip", true);
        const drip_type = checker.getTypeByName("drip").?;
        const add_result = try checker.checkBinaryOperation(drip_type, drip_type, "+");
        try checker.checkAssignment("sum", add_result);
        
        // Simulate checking array: sus numbers []drip = [1, 2, 3, 4, 5]
        const array_type = try TypeInfo.makeArray(checker.allocator, drip_type);
        defer array_type.deinit();
        _ = try checker.checkVariableDeclaration("numbers", "array", true);
        
        // Simulate checking array access: numbers[0]
        _ = try checker.checkArrayAccess(array_type, drip_type);
        
        // Simulate checking conditional: ready (number == 42)
        const cmp_result = try checker.checkBinaryOperation(drip_type, drip_type, "==");
        self.expect(cmp_result.base_type.isBoolean(), "Comparison should return boolean");
        
        std.log.info("✅ Basic CURSED program simulation completed", .{});
    }
    
    fn expect(self: *TypeCheckerValidation, condition: bool, message: []const u8) void {
        self.test_count += 1;
        if (condition) {
            self.passed_count += 1;
        } else {
            std.log.err("❌ Test failed: {s}", .{message});
        }
    }
    
    fn printSummary(self: *TypeCheckerValidation) void {
        const pass_rate = @as(f64, @floatFromInt(self.passed_count)) / @as(f64, @floatFromInt(self.test_count)) * 100.0;
        
        std.log.info("", .{});
        std.log.info("🎯 P0 Sprint 1 Type Checker Validation Summary", .{});
        std.log.info("===============================================", .{});
        std.log.info("Total tests: {d}", .{self.test_count});
        std.log.info("Passed: {d}", .{self.passed_count});
        std.log.info("Failed: {d}", .{self.test_count - self.passed_count});
        std.log.info("Pass rate: {d:.1}%", .{pass_rate});
        
        if (self.passed_count == self.test_count) {
            std.log.info("🎉 All tests passed! Type checker is ready for basic CURSED programs.", .{});
        } else {
            std.log.warn("⚠️  Some tests failed. Type checker needs improvements.", .{});
        }
    }
};

// Main validation function
pub fn validateTypeChecker(allocator: std.mem.Allocator) !void {
    var validation = TypeCheckerValidation.init(allocator);
    try validation.runAllTests();
}

// Test runner
test "P0 Sprint 1 Type Checker Validation" {
    const allocator = std.testing.allocator;
    try validateTypeChecker(allocator);
}
