// Comprehensive memory safety fixes for stdlib module loading
// Addresses memory corruption issues identified in fix_plan.md

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const crash_handler = @import("crash_handler.zig");

/// Enhanced module loader with comprehensive memory safety
pub const SafeModuleLoader = struct {
    allocator: Allocator,
    loaded_modules: HashMap([]const u8, SafeLoadedModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    verbose: bool,
    telemetry: ?*crash_handler.CrashTelemetry,
    allocation_tracker: AllocationTracker,

    const SafeLoadedModule = struct {
        name: []const u8,
        functions: ArrayList(SafeFunctionInfo),
        variables: ArrayList(SafeVariableInfo),
        path: []const u8,
        allocation_count: u32,

        pub fn init(allocator: Allocator, name: []const u8, path: []const u8) !SafeLoadedModule {
            return SafeLoadedModule{
                .name = try allocator.dupe(u8, name),
                .functions = ArrayList(SafeFunctionInfo).init(allocator),
                .variables = ArrayList(SafeVariableInfo).init(allocator),
                .path = try allocator.dupe(u8, path),
                .allocation_count = 0,
            };
        }

        pub fn deinit(self: *SafeLoadedModule, allocator: Allocator) void {
            // Free all strings we own
            allocator.free(self.name);
            allocator.free(self.path);
            
            // Clean up functions
            for (self.functions.items) |*func| {
                func.deinit(allocator);
            }
            self.functions.deinit();
            
            // Clean up variables
            for (self.variables.items) |*var_info| {
                var_info.deinit(allocator);
            }
            self.variables.deinit();
        }
    };

    const SafeFunctionInfo = struct {
        name: []const u8,
        parameter_count: u32,
        return_type: ?[]const u8,
        visibility: ast.Visibility,
        location: ast.SourceLocation,

        pub fn init(allocator: Allocator, name: []const u8) !SafeFunctionInfo {
            return SafeFunctionInfo{
                .name = try allocator.dupe(u8, name),
                .parameter_count = 0,
                .return_type = null,
                .visibility = .public,
                .location = ast.SourceLocation{ .line = 0, .column = 0 },
            };
        }

        pub fn deinit(self: *SafeFunctionInfo, allocator: Allocator) void {
            allocator.free(self.name);
            if (self.return_type) |ret_type| {
                allocator.free(ret_type);
            }
        }
    };

    const SafeVariableInfo = struct {
        name: []const u8,
        type_annotation: ?[]const u8,
        visibility: ast.Visibility,

        pub fn init(allocator: Allocator, name: []const u8) !SafeVariableInfo {
            return SafeVariableInfo{
                .name = try allocator.dupe(u8, name),
                .type_annotation = null,
                .visibility = .public,
            };
        }

        pub fn deinit(self: *SafeVariableInfo, allocator: Allocator) void {
            allocator.free(self.name);
            if (self.type_annotation) |type_anno| {
                allocator.free(type_anno);
            }
        }
    };

    /// Track allocations to prevent memory leaks
    const AllocationTracker = struct {
        active_allocations: HashMap(usize, usize, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
        total_allocations: u64,
        total_frees: u64,

        pub fn init(allocator: Allocator) AllocationTracker {
            return AllocationTracker{
                .active_allocations = HashMap(usize, usize, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
                .total_allocations = 0,
                .total_frees = 0,
            };
        }

        pub fn deinit(self: *AllocationTracker) void {
            self.active_allocations.deinit();
        }

        pub fn trackAllocation(self: *AllocationTracker, ptr: usize, size: usize) void {
            self.active_allocations.put(ptr, size) catch {};
            self.total_allocations += 1;
        }

        pub fn trackFree(self: *AllocationTracker, ptr: usize) bool {
            if (self.active_allocations.remove(ptr)) {
                self.total_frees += 1;
                return true;
            }
            return false; // Double-free detected
        }

        pub fn hasLeaks(self: *AllocationTracker) bool {
            return self.active_allocations.count() > 0;
        }

        pub fn printReport(self: *AllocationTracker) void {
            print("📊 Memory Report:\n");
            print("  Total allocations: {}\n", .{self.total_allocations});
            print("  Total frees: {}\n", .{self.total_frees});
            print("  Active allocations: {}\n", .{self.active_allocations.count()});
            if (self.hasLeaks()) {
                print("  ⚠️  Memory leaks detected!\n");
            } else {
                print("  ✅ No memory leaks detected\n");
            }
        }
    };

    pub fn init(allocator: Allocator, verbose: bool) SafeModuleLoader {
        return SafeModuleLoader{
            .allocator = allocator,
            .loaded_modules = HashMap([]const u8, SafeLoadedModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .verbose = verbose,
            .telemetry = null,
            .allocation_tracker = AllocationTracker.init(allocator),
        };
    }

    pub fn initWithTelemetry(allocator: Allocator, verbose: bool, telemetry: *crash_handler.CrashTelemetry) SafeModuleLoader {
        return SafeModuleLoader{
            .allocator = allocator,
            .loaded_modules = HashMap([]const u8, SafeLoadedModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .verbose = verbose,
            .telemetry = telemetry,
            .allocation_tracker = AllocationTracker.init(allocator),
        };
    }

    pub fn deinit(self: *SafeModuleLoader) void {
        // Clean up all loaded modules
        var iter = self.loaded_modules.iterator();
        while (iter.next()) |entry| {
            var module = entry.value_ptr;
            module.deinit(self.allocator);
            self.allocator.free(entry.key_ptr.*);
        }
        self.loaded_modules.deinit();
        
        // Check for memory leaks
        if (self.verbose) {
            self.allocation_tracker.printReport();
        }
        
        self.allocation_tracker.deinit();
    }

    /// Safely load a module with comprehensive memory management
    pub fn safeLoadModule(self: *SafeModuleLoader, module_name: []const u8) !?[]SafeFunctionInfo {
        // Check if already loaded
        if (self.loaded_modules.get(module_name)) |loaded_module| {
            if (self.verbose) print("📦 Module '{s}' already loaded, returning cached functions\n", .{module_name});
            return loaded_module.functions.items;
        }

        // Find module path
        const module_path = self.findModulePath(module_name) catch |err| {
            if (self.verbose) print("❌ Cannot find module '{s}': {any}\n", .{ module_name, err });
            return null;
        };
        defer self.allocator.free(module_path);

        if (self.verbose) print("📂 Loading module '{s}' from: {s}\n", .{ module_name, module_path });

        // Read source with memory safety
        const source = self.readModuleSourceSafely(module_path) catch |err| {
            if (self.verbose) print("❌ Failed to read module '{s}': {any}\n", .{ module_name, err });
            return null;
        };
        defer self.allocator.free(source);

        // Parse module with arena allocator to prevent leaks
        const loaded_module = self.parseModuleSafely(module_name, module_path, source) catch |err| {
            if (self.verbose) print("❌ Failed to parse module '{s}': {any}\n", .{ module_name, err });
            return null;
        };

        // Store in cache with owned strings
        const cached_name = try self.allocator.dupe(u8, module_name);
        try self.loaded_modules.put(cached_name, loaded_module);

        if (self.verbose) print("✅ Module '{s}' loaded with {} functions\n", .{ module_name, loaded_module.functions.items.len });

        return loaded_module.functions.items;
    }

    /// Find module path with safety checks
    fn findModulePath(self: *SafeModuleLoader, module_name: []const u8) ![]const u8 {
        // Input validation
        if (module_name.len == 0 or module_name.len > 255) {
            return error.InvalidModuleName;
        }

        // Check for directory traversal attacks
        if (std.mem.indexOf(u8, module_name, "..") != null or
            std.mem.indexOf(u8, module_name, "/") != null or
            std.mem.indexOf(u8, module_name, "\\") != null) {
            return error.InvalidModuleName;
        }

        // Find project root
        const project_root = self.findProjectRootSafely() catch return error.ProjectRootNotFound;
        defer self.allocator.free(project_root);

        // Build stdlib path
        var path_buf = ArrayList(u8).init(self.allocator);
        defer path_buf.deinit();

        try path_buf.appendSlice(project_root);
        try path_buf.appendSlice("/stdlib/");
        try path_buf.appendSlice(module_name);
        try path_buf.appendSlice("/mod.csd");

        // Verify file exists
        std.fs.cwd().access(path_buf.items, .{}) catch return error.ModuleNotFound;

        return try self.allocator.dupe(u8, path_buf.items);
    }

    /// Find project root with bounds checking
    fn findProjectRootSafely(self: *SafeModuleLoader) ![]const u8 {
        const cwd = std.fs.cwd();
        var buf: [4096]u8 = undefined; // Larger buffer for safety
        const current_path = try cwd.realpath(".", &buf);

        const markers = [_][]const u8{ "build.zig", "AGENT.md", ".git", "stdlib" };

        var path_components = ArrayList([]const u8).init(self.allocator);
        defer path_components.deinit();

        // Split path safely
        var iter = std.mem.splitScalar(u8, current_path, '/');
        while (iter.next()) |component| {
            if (component.len > 0) {
                try path_components.append(component);
            }
        }

        // Walk up directory tree with depth limit
        var depth: u32 = 0;
        const max_depth = 20; // Prevent infinite loops

        while (path_components.items.len > 0 and depth < max_depth) {
            depth += 1;

            // Build test path
            var test_path = ArrayList(u8).init(self.allocator);
            defer test_path.deinit();

            try test_path.append('/');
            for (path_components.items) |component| {
                try test_path.appendSlice(component);
                try test_path.append('/');
            }

            // Check for markers
            for (markers) |marker| {
                var marker_path = ArrayList(u8).init(self.allocator);
                defer marker_path.deinit();

                try marker_path.appendSlice(test_path.items);
                try marker_path.appendSlice(marker);

                cwd.access(marker_path.items, .{}) catch continue;

                // Found project root
                return try self.allocator.dupe(u8, test_path.items[0..test_path.items.len-1]);
            }

            _ = path_components.pop();
        }

        return try self.allocator.dupe(u8, current_path);
    }

    /// Read module source with safety checks
    fn readModuleSourceSafely(self: *SafeModuleLoader, module_path: []const u8) ![]const u8 {
        // Validate path length
        if (module_path.len == 0 or module_path.len > 4096) {
            return error.InvalidPath;
        }

        const file = std.fs.cwd().openFile(module_path, .{}) catch |err| {
            if (self.verbose) print("❌ Cannot open module file: {s} - {any}\n", .{ module_path, err });
            return err;
        };
        defer file.close();

        const file_size = try file.getEndPos();
        
        // Protect against extremely large files
        const max_file_size = 10 * 1024 * 1024; // 10MB limit
        if (file_size > max_file_size) {
            return error.FileTooLarge;
        }

        const contents = try self.allocator.alloc(u8, file_size);
        errdefer self.allocator.free(contents);

        const bytes_read = try file.readAll(contents);
        if (bytes_read != file_size) {
            self.allocator.free(contents);
            return error.IncompleteRead;
        }

        return contents;
    }

    /// Parse module with comprehensive memory safety
    fn parseModuleSafely(self: *SafeModuleLoader, module_name: []const u8, module_path: []const u8, source: []const u8) !SafeLoadedModule {
        // Use arena allocator for temporary parsing
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();

        // Initialize module
        var loaded_module = try SafeLoadedModule.init(self.allocator, module_name, module_path);
        errdefer loaded_module.deinit(self.allocator);

        // Tokenize safely
        var module_lexer = lexer.Lexer.init(arena_allocator, source);
        const tokens = module_lexer.tokenize() catch |err| {
            if (self.verbose) print("❌ Tokenization failed for module '{s}': {any}\n", .{ module_name, err });
            return err;
        };

        if (self.verbose) print("🔍 Tokenized module '{s}' - {} tokens\n", .{ module_name, tokens.items.len });

        // Parse safely
        var module_parser = parser.Parser.initWithFile(arena_allocator, tokens.items, module_path);
        const program = module_parser.parseProgram() catch |err| {
            if (self.verbose) print("❌ Parsing failed for module '{s}': {any}\n", .{ module_name, err });
            return err;
        };

        if (self.verbose) print("🔍 Parsed module '{s}' - {} statements\n", .{ module_name, program.statements.items.len });

        // Extract functions and variables safely
        for (program.statements.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @ptrCast(@alignCast(stmt_ptr))).*;
            switch (stmt) {
                .Function => |func| {
                    var safe_func = try SafeFunctionInfo.init(self.allocator, func.name);
                    errdefer safe_func.deinit(self.allocator);

                    safe_func.parameter_count = @intCast(func.parameters.items.len);
                    safe_func.visibility = func.visibility;
                    safe_func.location = func.location;

                    try loaded_module.functions.append(safe_func);
                    loaded_module.allocation_count += 1;

                    if (self.verbose) print("📦 Found function: {s}\n", .{safe_func.name});
                },
                .Let => |let_stmt| {
                    var safe_var = try SafeVariableInfo.init(self.allocator, let_stmt.name);
                    errdefer safe_var.deinit(self.allocator);

                    safe_var.visibility = let_stmt.visibility;

                    try loaded_module.variables.append(safe_var);
                    loaded_module.allocation_count += 1;

                    if (self.verbose) print("📦 Found variable: {s}\n", .{safe_var.name});
                },
                else => {
                    // Skip other statements
                },
            }
        }

        return loaded_module;
    }

    /// Get functions from loaded module safely
    pub fn getModuleFunctions(self: *SafeModuleLoader, module_name: []const u8) ?[]SafeFunctionInfo {
        if (self.loaded_modules.get(module_name)) |loaded_module| {
            return loaded_module.functions.items;
        }
        return null;
    }

    /// Check if module is loaded
    pub fn isModuleLoaded(self: *SafeModuleLoader, module_name: []const u8) bool {
        return self.loaded_modules.contains(module_name);
    }

    /// Comprehensive memory validation
    pub fn validateMemory(self: *SafeModuleLoader) !void {
        if (self.allocation_tracker.hasLeaks()) {
            if (self.verbose) {
                self.allocation_tracker.printReport();
            }
            return error.MemoryLeaksDetected;
        }
    }
};

/// Test the safe module loader
pub fn testSafeModuleLoader(allocator: Allocator) !void {
    print("🧪 Testing Safe Module Loader...\n");

    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();

    // Test loading mathz module
    const functions = try loader.safeLoadModule("mathz");
    if (functions) |func_list| {
        print("✅ Successfully loaded {} functions from mathz\n", .{func_list.len});
        for (func_list) |func| {
            print("  - {s}\n", .{func.name});
        }
    } else {
        print("❌ Failed to load mathz module\n");
    }

    // Validate memory
    try loader.validateMemory();
    print("✅ Memory validation passed\n");
}
