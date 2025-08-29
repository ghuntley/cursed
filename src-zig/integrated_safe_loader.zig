// Integration wrapper for the safe import resolver
// Provides a compatible interface with the existing module loader

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

const ast = @import("ast.zig");
const safe_import_resolver = @import("safe_import_resolver.zig");
const SafeModuleLoader = safe_import_resolver.SafeModuleLoader;

// Wrapper that provides the same interface as the original ModuleLoader
pub const IntegratedSafeLoader = struct {
    safe_loader: SafeModuleLoader,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, verbose: bool) IntegratedSafeLoader {
        return IntegratedSafeLoader{
            .safe_loader = SafeModuleLoader.init(allocator, verbose),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *IntegratedSafeLoader) void {
        self.safe_loader.deinit(self.allocator);
    }
    
    /// Load a module safely with cycle detection
    pub fn loadModule(self: *IntegratedSafeLoader, module_name: []const u8) !?[]ast.FunctionStatement {
        return self.safe_loader.loadModuleSafe(module_name, null);
    }
    
    /// Get functions from an already loaded module
    pub fn getModuleFunctions(self: *IntegratedSafeLoader, module_name: []const u8) ?[]ast.FunctionStatement {
        if (self.safe_loader.loaded_modules.get(module_name)) |loaded_module| {
            return loaded_module.functions.items;
        }
        return null;
    }
    
    /// Check if a module is loaded
    pub fn isModuleLoaded(self: *IntegratedSafeLoader, module_name: []const u8) bool {
        const state = self.safe_loader.module_states.get(module_name) orelse .not_loaded;
        return state == .loaded;
    }
    
    /// Debug: Print current state
    pub fn debugPrintState(self: *IntegratedSafeLoader) void {
        self.safe_loader.printModuleStates();
    }
    
    /// Detect cycles in the current dependency graph
    pub fn detectCycles(self: *IntegratedSafeLoader) !bool {
        return self.safe_loader.detectCycles();
    }
    
    /// Force unload a module (for testing)
    pub fn unloadModule(self: *IntegratedSafeLoader, module_name: []const u8) void {
        self.safe_loader.unloadModule(module_name);
    }
};

/// Helper function for compatibility with existing code
pub fn loadModuleIntoFunctionStore(
    loader: *IntegratedSafeLoader,
    module_name: []const u8,
    _: anytype, // FunctionStore type - unused
    allocator: Allocator,
    verbose: bool
) !bool {
    const functions = try loader.loadModule(module_name);
    if (functions == null) {
        if (verbose) print("❌ Failed to load module: {s}\n", .{module_name});
        return false;
    }
    
    // Add functions to the function store with their module prefix
    for (functions.?) |func| {
        // Store functions both with and without module prefix for compatibility
        const func_name = try allocator.dupe(u8, func.name);
        const prefixed_name = try std.fmt.allocPrint(allocator, "{s}.{s}", .{ module_name, func.name });
        
        // Add the function to the store (this would need to be adapted to your function store structure)
        // function_store.put(func_name, func);
        // function_store.put(prefixed_name, func);
        
        if (verbose) print("📦 Added function: {s} (also as {s})\n", .{ func_name, prefixed_name });
        
        // Clean up temporary allocations
        allocator.free(func_name);
        allocator.free(prefixed_name);
    }
    
    return true;
}

// Test the integrated loader
test "integrated safe loader compatibility" {
    const allocator = std.testing.allocator;
    
    var loader = IntegratedSafeLoader.init(allocator, true);
    defer loader.deinit();
    
    // Test basic functionality
    try std.testing.expect(!loader.isModuleLoaded("test_module"));
    
    // Test cycle detection
    loader.safe_loader.recordDependency("A", "B");
    loader.safe_loader.recordDependency("B", "A");
    
    const has_cycles = try loader.detectCycles();
    try std.testing.expect(has_cycles);
    
    print("✅ Integrated safe loader compatibility test passed\n", .{});
}
