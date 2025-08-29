// Enhanced import safety and memory management for stdlib modules
// Fixes memory corruption issues and improves module loading robustness

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const module_loader = @import("module_loader.zig");

/// Enhanced import safety wrapper
pub const SafeImportManager = struct {
    allocator: Allocator,
    module_loader_instance: module_loader.ModuleLoader,
    import_cache: HashMap([]const u8, ImportResult, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    verbose: bool,
    memory_guard: MemoryGuard,

    const ImportResult = struct {
        success: bool,
        error_message: ?[]const u8,
        function_count: u32,
        load_time_ms: u64,

        pub fn deinit(self: *ImportResult, allocator: Allocator) void {
        _ = allocator;
            if (self.error_message) |msg| {
                allocator.free(msg);
            }
        }
    };

    const MemoryGuard = struct {
        max_allocations: u32,
        current_allocations: u32,
        allocation_size_limit: usize,

        pub fn init() MemoryGuard {
            return MemoryGuard{
                .max_allocations = 10000,
                .current_allocations = 0,
                .allocation_size_limit = 100 * 1024 * 1024, // 100MB
            };
        }

        pub fn canAllocate(self: *MemoryGuard, size: usize) bool {
            return self.current_allocations < self.max_allocations and 
                   size <= self.allocation_size_limit;
        }

        pub fn trackAllocation(self: *MemoryGuard) void {
            self.current_allocations += 1;
        }

        pub fn trackDeallocation(self: *MemoryGuard) void {
            if (self.current_allocations > 0) {
                self.current_allocations -= 1;
            }
        }
    };

    pub fn init(allocator: Allocator, verbose: bool) SafeImportManager {
        return SafeImportManager{
            .allocator = allocator,
            .module_loader_instance = module_loader.ModuleLoader.init(allocator, verbose),
            .import_cache = HashMap([]const u8, ImportResult, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .verbose = verbose,
            .memory_guard = MemoryGuard.init(),
        };
    }

    pub fn deinit(self: *SafeImportManager) void {
        // Clean up import cache
        var iter = self.import_cache.iterator();
        while (iter.next()) |entry| {
            var result = entry.value_ptr;
            result.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.import_cache.deinit(self.allocator);

        // Clean up module loader
        self.module_loader_instance.deinit(self.allocator);

        if (self.verbose) {
            print("📊 Memory Guard Final Report:\n", .{});
            print("  Remaining allocations: {s}\n", .{self.memory_guard.current_allocations});
        }
    }

    /// Safely import a module with comprehensive error handling
    pub fn safeImportModule(self: *SafeImportManager, module_name: []const u8) !bool {
        // Input validation
        if (module_name.len == 0 or module_name.len > 255) {
            if (self.verbose) print("❌ Invalid module name length: {s}\n", .{module_name.len});
            return false;
        }

        // Check cache first
        if (self.import_cache.get(module_name)) |cached_result| {
            if (self.verbose) print("📦 Using cached import result for '{s}'\n", .{module_name});
            return cached_result.success;
        }

        const start_time = std.time.milliTimestamp();

        // Memory guard check
        if (!self.memory_guard.canAllocate(1024 * 1024)) { // 1MB allocation check
            if (self.verbose) print("❌ Memory limit reached, cannot import '{s}'\n", .{module_name});
            return false;
        }

        // Attempt import with error handling
        var import_result = ImportResult{
            .success = false,
            .error_message = null,
            .function_count = 0,
            .load_time_ms = 0,
        };

        // Try to load the module
        const functions = self.module_loader_instance.loadModule(module_name) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Module load failed: {any}", .{err});
            import_result.error_message = error_msg;
            self.memory_guard.trackAllocation();

            if (self.verbose) print("❌ Failed to load module '{s}': {any}\n", .{ module_name, err });
            
            // Cache the failure
            const cache_key = try self.allocator.dupe(u8, module_name);
            try self.import_cache.put(cache_key, import_result);
            return false;
        };

        if (functions) |func_list| {
            import_result.success = true;
            import_result.function_count = @intCast(func_list.len);
            import_result.load_time_ms = @intCast(std.time.milliTimestamp() - start_time);

            if (self.verbose) {
                print("✅ Successfully imported module '{s}' with {} functions in {}ms\n", 
                      .{ module_name, func_list.len, import_result.load_time_ms });
            }
        } else {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Module returned null functions");
            import_result.error_message = error_msg;
            self.memory_guard.trackAllocation();

            if (self.verbose) print("❌ Module '{s}' returned no functions\n", .{module_name});
        }

        // Cache the result
        const cache_key = try self.allocator.dupe(u8, module_name);
        try self.import_cache.put(cache_key, import_result);

        return import_result.success;
    }

    /// Check if a module is safely imported
    pub fn isModuleSafelyImported(self: *SafeImportManager, module_name: []const u8) bool {
        if (self.import_cache.get(module_name)) |result| {
            return result.success;
        }
        return self.module_loader_instance.isModuleLoaded(module_name);
    }

    /// Get import statistics
    pub fn getImportStats(self: *SafeImportManager) ImportStats {
        var successful_imports: u32 = 0;
        var failed_imports: u32 = 0;
        var total_functions: u32 = 0;
        var total_load_time: u64 = 0;

        var iter = self.import_cache.iterator();
        while (iter.next()) |entry| {
            const result = entry.value_ptr;
            if (result.success) {
                successful_imports += 1;
                total_functions += result.function_count;
                total_load_time += result.load_time_ms;
            } else {
                failed_imports += 1;
            }
        }

        return ImportStats{
            .successful_imports = successful_imports,
            .failed_imports = failed_imports,
            .total_functions = total_functions,
            .total_load_time_ms = total_load_time,
            .memory_allocations = self.memory_guard.current_allocations,
        };
    }

    /// Validate all imported modules
    pub fn validateAllImports(self: *SafeImportManager) !bool {
        var all_valid = true;
        var iter = self.import_cache.iterator();
        
        while (iter.next()) |entry| {
            const module_name = entry.key_ptr.*;
            const result = entry.value_ptr;
            
            if (!result.success) {
                if (self.verbose) {
                    if (result.error_message) |msg| {
                        print("❌ Module '{s}' failed: {s}\n", .{ module_name, msg });
                    } else {
                        print("❌ Module '{s}' failed with unknown error\n", .{module_name});
                    }
                }
                all_valid = false;
            }
        }

        return all_valid;
    }

    /// Force cleanup of all cached imports
    pub fn clearImportCache(self: *SafeImportManager) void {
        var iter = self.import_cache.iterator();
        while (iter.next()) |entry| {
            var result = entry.value_ptr;
            result.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.import_cache.clearAndFree(self.allocator);

        if (self.verbose) {
            print("🧹 Import cache cleared\n", .{});
        }
    }

    /// Get module functions safely
    pub fn getModuleFunctionsSafely(self: *SafeImportManager, module_name: []const u8) ?[]ast.FunctionStatement {
        // Check if module is properly imported
        if (!self.isModuleSafelyImported(module_name)) {
            if (self.verbose) print("⚠️  Module '{s}' not safely imported\n", .{module_name});
            return null;
        }

        return self.module_loader_instance.getModuleFunctions(module_name);
    }
};

pub const ImportStats = struct {
    successful_imports: u32,
    failed_imports: u32,
    total_functions: u32,
    total_load_time_ms: u64,
    memory_allocations: u32,

    pub fn print(self: ImportStats) void {
        std.debug.print("📊 Import Statistics:\n", .{});
        std.debug.print("  Successful imports: {s}\n", .{self.successful_imports});
        std.debug.print("  Failed imports: {s}\n", .{self.failed_imports});
        std.debug.print("  Total functions loaded: {s}\n", .{self.total_functions});
        std.debug.print("  Total load time: {s}ms\n", .{self.total_load_time_ms});
        std.debug.print("  Memory allocations: {s}\n", .{self.memory_allocations});
        
        if (self.successful_imports > 0) {
            const avg_time = self.total_load_time_ms / self.successful_imports;
            std.debug.print("  Average load time: {s}ms\n", .{avg_time});
        }
    }
};

/// Enhanced import extraction with safety checks
pub fn extractImportsSafely(allocator: Allocator, source: []const u8) !ArrayList([]const u8) {
    var imports = std.ArrayList(u8){};
    errdefer {
        for (imports.items) |import_name| {
            allocator.free(import_name);
        }
        imports.deinit();
    }

    // Input validation
    if (source.len == 0) {
        return imports;
    }

    // Protect against extremely large source files
    if (source.len > 10 * 1024 * 1024) { // 10MB limit
        return error.SourceFileTooLarge;
    }

    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_count: u32 = 0;
    const max_lines = 100000; // Protect against infinite loops

    while (lines.next()) |line| {
        line_count += 1;
        if (line_count > max_lines) {
            return error.TooManyLines;
        }

        const trimmed = std.mem.trim(u8, line, " \t\r");

        // Look for "yeet" statements with safety checks
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            const import_part = trimmed[5..]; // Skip "yeet "

            // Extract module name from quotes with bounds checking
            if (std.mem.indexOf(u8, import_part, "\"")) |start_quote| {
                if (start_quote < import_part.len - 1) {
                    const after_start = import_part[start_quote + 1 ..];
                    if (std.mem.indexOf(u8, after_start, "\"")) |end_quote| {
                        if (end_quote > 0 and end_quote < after_start.len) {
                            const module_name = after_start[0..end_quote];
                            
                            // Validate module name
                            if (module_name.len > 0 and module_name.len <= 255) {
                                // Check for invalid characters
                                var valid = true;
                                for (module_name) |c| {
                                    if (!std.ascii.isAlphanumeric(c) and c != '_' and c != '-') {
                                        valid = false;
                                        break;
                                    }
                                }
                                
                                if (valid) {
                                    const module_copy = try allocator.dupe(u8, module_name);
                                    try imports.append(allocator, module_copy);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return imports;
}

/// Validate imports with comprehensive checks
pub fn validateImportsSafely(allocator: Allocator, imports: ArrayList([]const u8)) !bool {
    if (imports.items.len == 0) {
        return true;
    }

    // Use safe import manager for validation
    var safe_manager = SafeImportManager.init(allocator, false); // Non-verbose for validation
    defer safe_manager.deinit();

    var all_valid = true;

    for (imports.items) |module_name| {
        // Try to import safely
        const result = safe_manager.safeImportModule(module_name) catch false;
        if (!result) {
            all_valid = false;
        }
    }

    return all_valid;
}

/// Test the safe import system
pub fn testSafeImportSystem(allocator: Allocator) !void {
        _ = allocator;
    print("🧪 Testing Safe Import System...\n", .{});

    var safe_manager = SafeImportManager.init(allocator, true);
    defer safe_manager.deinit();

    // Test importing standard modules
    const test_modules = [_][]const u8{ "mathz", "stringz", "arrayz" };

    for (test_modules) |module_name| {
        const result = try safe_manager.safeImportModule(module_name);
        if (result) {
            print("✅ Successfully imported {s}\n", .{module_name});
        } else {
            print("❌ Failed to import {s}\n", .{module_name});
        }
    }

    // Print statistics
    const stats = safe_manager.getImportStats();
    stats.writer().print();

    // Validate all imports
    const all_valid = try safe_manager.validateAllImports();
    if (all_valid) {
        print("✅ All imports validated successfully\n", .{});
    } else {
        print("❌ Some imports failed validation\n", .{});
    }

    print("🧪 Safe import system test completed\n", .{});
}
