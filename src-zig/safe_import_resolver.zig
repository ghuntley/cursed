// Safe Import Resolver with Cycle Detection and Memory Management
// Fixes double-free issues in cyclic module dependencies

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// Import existing types
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

// ===== Module State Tracking =====

pub const ModuleState = enum {
    not_loaded,      // Module hasn't been processed yet
    in_progress,     // Module is currently being loaded (prevents double-init)
    loaded,          // Module has been fully loaded
    error_state,     // Module failed to load
};

pub const SafeModuleLoader = struct {
    allocator: Allocator,
    module_states: HashMap([]const u8, ModuleState, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    loaded_modules: HashMap([]const u8, LoadedModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    dependency_graph: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    reference_counts: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    verbose: bool,
    
    const LoadedModule = struct {
        name: []const u8,
        functions: ArrayList(ast.FunctionStatement),
        variables: ArrayList(ast.LetStatement),
        path: []const u8,
        ref_count: u32,
        
        pub fn deinit(self: *LoadedModule, allocator: Allocator) void {
        _ = allocator;
            allocator.free(self.name);
            allocator.free(self.path);
            for (self.functions.items) |*func| {
                func.deinit();
            }
            for (self.variables.items) |*var_stmt| {
                var_stmt.deinit();
            }
            self.functions.deinit(self.allocator);
            self.variables.deinit(self.allocator);
        }
        
        pub fn addRef(self: *LoadedModule) void {
            self.ref_count += 1;
        }
        
        pub fn removeRef(self: *LoadedModule) u32 {
            if (self.ref_count > 0) {
                self.ref_count -= 1;
            }
            return self.ref_count;
        }
    };
    
    pub fn init(allocator: Allocator, verbose: bool) SafeModuleLoader {
        return SafeModuleLoader{
            .allocator = allocator,
            .module_states = HashMap([]const u8, ModuleState, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .loaded_modules = HashMap([]const u8, LoadedModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .dependency_graph = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .reference_counts = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .verbose = verbose,
        };
    }
    
    pub fn deinit(self: *SafeModuleLoader) void {
        // Clean up module states map
        var states_iter = self.module_states.iterator();
        while (states_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.module_states.deinit(self.allocator);
        
        // Clean up loaded modules map
        var modules_iter = self.loaded_modules.iterator();
        while (modules_iter.next()) |entry| {
            var module = entry.value_ptr;
            module.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.loaded_modules.deinit(self.allocator);
        
        // Clean up dependency graph
        var deps_iter = self.dependency_graph.iterator();
        while (deps_iter.next()) |entry| {
            var deps_list = entry.value_ptr;
            for (deps_list.items) |dep| {
                self.allocator.free(dep);
            }
            deps_list.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.dependency_graph.deinit(self.allocator);
        
        // Clean up reference counts map
        var refs_iter = self.reference_counts.iterator();
        while (refs_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.reference_counts.deinit(self.allocator);
    }
    
    /// Safe module loading with cycle detection and reference counting
    pub fn loadModuleSafe(self: *SafeModuleLoader, module_name: []const u8, dependent_module: ?[]const u8) !?[]ast.FunctionStatement {
        // Check if module is already being loaded (cycle detection)
        const current_state = self.module_states.get(module_name) orelse .not_loaded;
        
        switch (current_state) {
            .loaded => {
                // Module already loaded, increment reference and return
                if (self.loaded_modules.getPtr(module_name)) |module| {
                    module.addRef();
                    self.incrementRefCount(module_name);
                    if (self.verbose) print("📦 Module '{s}' already loaded (ref count: {s}), returning cached functions\n", .{ module_name, module.ref_count });
                    return module.functions.items;
                }
                return null;
            },
            .in_progress => {
                // Cycle detected! This is safe - just return null to break the cycle
                if (self.verbose) print("🔄 Cycle detected: Module '{s}' is already being loaded, breaking cycle safely\n", .{module_name});
                self.recordDependency(dependent_module, module_name);
                return null;
            },
            .error_state => {
                if (self.verbose) print("❌ Module '{s}' is in error state, cannot load\n", .{module_name});
                return null;
            },
            .not_loaded => {
                // Proceed with loading
            },
        }
        
        // Mark as in-progress to detect cycles
        try self.setModuleState(module_name, .in_progress);
        
        // Record dependency relationship
        if (dependent_module) |dep| {
            self.recordDependency(dep, module_name);
        }
        
        // Find module path
        const module_path = self.findModulePath(module_name) catch |err| {
            try self.setModuleState(module_name, .error_state);
            if (self.verbose) print("❌ Failed to find module '{s}': {any}\n", .{ module_name, err });
            return null;
        };
        defer self.allocator.free(module_path);
        
        if (self.verbose) print("📂 Loading module '{s}' from: {s}\n", .{ module_name, module_path });
        
        // Read module source
        const source = self.readModuleSource(module_path) catch |err| {
            try self.setModuleState(module_name, .error_state);
            if (self.verbose) print("❌ Failed to read module '{s}': {any}\n", .{ module_name, err });
            return null;
        };
        defer self.allocator.free(source);
        
        // Parse module and extract functions
        const loaded_module = self.parseModuleSafe(module_name, module_path, source) catch |err| {
            try self.setModuleState(module_name, .error_state);
            if (self.verbose) print("❌ Failed to parse module '{s}': {any}\n", .{ module_name, err });
            return null;
        };
        
        // Store in cache with reference counting
        const cached_name = try self.allocator.dupe(u8, module_name);
        try self.loaded_modules.put(cached_name, loaded_module);
        try self.setModuleState(module_name, .loaded);
        
        // Initialize reference count
        try self.setRefCount(module_name, 1);
        
        if (self.verbose) print("✅ Module '{s}' loaded with {s} functions (ref count: 1)\n", .{ module_name, loaded_module.functions.items.len });
        
        // Recursively load any dependencies found in this module
        try self.loadDependenciesRecursively(module_name, source);
        
        if (self.loaded_modules.getPtr(module_name)) |module| {
            return module.functions.items;
        }
        
        return null;
    }
    
    /// Set module state safely
    pub fn setModuleState(self: *SafeModuleLoader, module_name: []const u8, state: ModuleState) !void {
        // Check if key already exists to avoid duplicating it
        if (self.module_states.contains(module_name)) {
            try self.module_states.put(module_name, state);
        } else {
            const key = try self.allocator.dupe(u8, module_name);
            try self.module_states.put(key, state);
        }
    }
    
    /// Record dependency relationship for cycle detection
    pub fn recordDependency(self: *SafeModuleLoader, from_module: ?[]const u8, to_module: []const u8) void {
        if (from_module == null) return;
        
        const from = from_module.?;
        
        // Get or create dependency list for from_module
        if (self.dependency_graph.getPtr(from)) |deps_list| {
            // Add to existing list (check for duplicates)
            for (deps_list.items) |existing_dep| {
                if (std.mem.eql(u8, existing_dep, to_module)) {
                    return; // Already recorded
                }
            }
            deps_list.append(self.allocator, self.allocator.dupe(u8, to_module) catch return) catch return;
        } else {
            // Create new dependency list
            var new_deps = std.ArrayList(u8){};
            const to_key = self.allocator.dupe(u8, to_module) catch return;
            new_deps.append(self.allocator, to_key) catch {
                self.allocator.free(to_key);
                return;
            };
            
            const from_key = self.allocator.dupe(u8, from) catch {
                new_deps.deinit();
                self.allocator.free(to_key);
                return;
            };
            
            self.dependency_graph.put(from_key, new_deps) catch {
                self.allocator.free(from_key);
                new_deps.deinit();
                self.allocator.free(to_key);
                return;
            };
        }
    }
    
    /// Reference counting helpers
    pub fn setRefCount(self: *SafeModuleLoader, module_name: []const u8, count: u32) !void {
        if (self.reference_counts.contains(module_name)) {
            try self.reference_counts.put(module_name, count);
        } else {
            const key = try self.allocator.dupe(u8, module_name);
            try self.reference_counts.put(key, count);
        }
    }
    
    pub fn incrementRefCount(self: *SafeModuleLoader, module_name: []const u8) void {
        const current = self.reference_counts.get(module_name) orelse 0;
        self.setRefCount(module_name, current + 1) catch return;
    }
    
    pub fn decrementRefCount(self: *SafeModuleLoader, module_name: []const u8) u32 {
        const current = self.reference_counts.get(module_name) orelse 0;
        const new_count = if (current > 0) current - 1 else 0;
        self.setRefCount(module_name, new_count) catch return new_count;
        return new_count;
    }
    
    /// Recursively load dependencies from a module's source
    fn loadDependenciesRecursively(self: *SafeModuleLoader, current_module: []const u8, source: []const u8) !void {
        var lines = std.mem.splitScalar(u8, source, '\n');
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r");
            
            // Look for "yeet" statements
            if (std.mem.startsWith(u8, trimmed, "yeet ")) {
                if (self.extractModuleFromYeet(trimmed)) |dep_module| {
                    defer self.allocator.free(dep_module);
                    
                    // Skip if dependency is same as current module (self-import)
                    if (std.mem.eql(u8, dep_module, current_module)) {
                        if (self.verbose) print("🔄 Skipping self-import: {s}\n", .{dep_module});
                        continue;
                    }
                    
                    // Load dependency safely (will handle cycles)
                    _ = self.loadModuleSafe(dep_module, current_module) catch |err| {
                        if (self.verbose) print("⚠️  Failed to load dependency '{s}' from '{s}': {any}\n", .{ dep_module, current_module, err });
                        continue;
                    };
                }
            }
        }
    }
    
    /// Extract module name from yeet statement
    fn extractModuleFromYeet(self: *SafeModuleLoader, line: []const u8) ?[]const u8 {
        const import_part = line[5..]; // Skip "yeet "
        
        // Find quoted module name
        if (std.mem.indexOf(u8, import_part, "\"")) |start_quote| {
            const after_start = import_part[start_quote + 1..];
            if (std.mem.indexOf(u8, after_start, "\"")) |end_quote| {
                const module_name = after_start[0..end_quote];
                return self.allocator.dupe(u8, module_name) catch null;
            }
        }
        
        return null;
    }
    
    /// Find the path to a module file
    fn findModulePath(self: *SafeModuleLoader, module_name: []const u8) ![]const u8 {
        // Build stdlib path
        var path_buf = std.ArrayList(u8){};
        defer path_buf.deinit();
        
        // Find project root
        const project_root = try self.findProjectRoot();
        defer self.allocator.free(project_root);
        
        try path_buf.appendSlice(project_root);
        try path_buf.appendSlice("/stdlib/");
        try path_buf.appendSlice(module_name);
        try path_buf.appendSlice("/mod.csd");
        
        // Check if file exists
        std.fs.cwd().access(path_buf.items, .{}) catch {
            return error.ModuleNotFound;
        };
        
        return try self.allocator.dupe(u8, path_buf.items);
    }
    
    /// Find the project root directory
    fn findProjectRoot(self: *SafeModuleLoader) ![]const u8 {
        const cwd = std.fs.cwd();
        var buf: [1024]u8 = undefined;
        const current_path = try cwd.realpath(".", &buf);
        
        const markers = [_][]const u8{
            "build.zig",
            "AGENT.md",
            ".git",
            "stdlib"
        };
        
        var path_components = std.ArrayList(u8){};
        defer path_components.deinit();
        
        var iter = std.mem.splitScalar(u8, current_path, '/');
        while (iter.next()) |component| {
            if (component.len > 0) {
                try path_components.append(allocator, component);
            }
        }
        
        while (path_components.items.len > 0) {
            var test_path = std.ArrayList(u8){};
            defer test_path.deinit();
            
            try test_path.append(allocator, '/');
            for (path_components.items) |component| {
                try test_path.appendSlice(component);
                try test_path.append(allocator, '/');
            }
            
            for (markers) |marker| {
                var marker_path = std.ArrayList(u8){};
                defer marker_path.deinit();
                
                try marker_path.appendSlice(test_path.items);
                try marker_path.appendSlice(marker);
                
                cwd.access(marker_path.items, .{}) catch continue;
                
                return try self.allocator.dupe(u8, test_path.items[0..test_path.items.len-1]);
            }
            
            _ = path_components.pop();
        }
        
        return try self.allocator.dupe(u8, current_path);
    }
    
    /// Read the source code of a module
    fn readModuleSource(self: *SafeModuleLoader, module_path: []const u8) ![]const u8 {
        const file = std.fs.cwd().openFile(module_path, .{}) catch |err| {
            if (self.verbose) print("❌ Cannot open module file: {s} - {any}\n", .{ module_path, err });
            return err;
        };
        defer file.close();
        
        const file_size = try file.getEndPos();
        const contents = try self.allocator.alloc(u8, file_size);
        _ = try file.readAll(contents);
        
        return contents;
    }
    
    /// Parse a module and extract its functions and variables safely
    fn parseModuleSafe(self: *SafeModuleLoader, module_name: []const u8, module_path: []const u8, source: []const u8) !LoadedModule {
        // Use an arena allocator for temporary parsing to prevent use-after-free
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        // Tokenize the source
        var module_lexer = lexer.Lexer.init(arena_allocator, source);
        const tokens = try module_lexer.tokenize();
        
        if (self.verbose) print("🔍 Tokenized module '{s}' - {s} tokens\n", .{ module_name, tokens.items.len });
        
        // Parse the tokens using arena allocator
        var module_parser = parser.Parser.initWithFile(arena_allocator, tokens.items, module_path);
        const program = try module_parser.parseProgram();
        
        if (self.verbose) print("🔍 Parsed module '{s}' - {s} statements\n", .{ module_name, program.statements.items.len });
        
        // Extract functions and variables with proper string copying
        var functions = std.ArrayList(u8){};
        var variables = std.ArrayList(u8){};
        
        for (program.statements.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @ptrCast(@alignCast(stmt_ptr))).*;
            switch (stmt) {
                .Function => |func| {
                    // Create a new function with properly copied strings
                    var copied_func = ast.FunctionStatement.init(self.allocator, "");
                    
                    // Copy function name to prevent use-after-free
                    copied_func.name = try self.allocator.dupe(u8, func.name);
                    
                    // Copy other properties
                    copied_func.visibility = func.visibility;
                    copied_func.is_async = func.is_async;
                    copied_func.location = func.location;
                    
                    try functions.append(self.allocator, copied_func);
                    if (self.verbose) print("📦 Found function: {s}\n", .{copied_func.name});
                },
                .Let => |let_stmt| {
                    // Create a copy of the let statement with proper string duplication
                    var copied_let = let_stmt;
                    copied_let.name = try self.allocator.dupe(u8, let_stmt.name);
                    
                    // For now, skip copying complex type annotations and initializers
                    copied_let.type_annotation = null;
                    copied_let.initializer = null;
                    
                    try variables.append(self.allocator, copied_let);
                    if (self.verbose) print("📦 Found variable: {s}\n", .{copied_let.name});
                },
                else => {
                    // Skip other statements
                },
            }
        }
        
        return LoadedModule{
            .name = try self.allocator.dupe(u8, module_name),
            .functions = functions,
            .variables = variables,
            .path = try self.allocator.dupe(u8, module_path),
            .ref_count = 0,
        };
    }
    
    /// Safely unload a module (with reference counting)
    pub fn unloadModule(self: *SafeModuleLoader, module_name: []const u8) void {
        const ref_count = self.decrementRefCount(module_name);
        
        if (ref_count == 0) {
            // No more references, safe to unload
            if (self.loaded_modules.getPtr(module_name)) |module| {
                if (self.verbose) print("🗑️  Unloading module '{s}' (ref count reached 0)\n", .{module_name});
                module.deinit();
                _ = self.loaded_modules.remove(module_name);
                try self.setModuleState(module_name, .not_loaded) catch {};
            }
        } else {
            if (self.verbose) print("📦 Module '{s}' still has {s} references, keeping loaded\n", .{ module_name, ref_count });
        }
    }
    
    /// Detect and report dependency cycles
    pub fn detectCycles(self: *SafeModuleLoader) !bool {
        var visited = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
        defer visited.deinit();
        
        var rec_stack = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
        defer rec_stack.deinit();
        
        var has_cycles = false;
        
        var iter = self.dependency_graph.iterator();
        while (iter.next()) |entry| {
            const module = entry.key_ptr.*;
            
            if (visited.get(module) orelse false) continue;
            
            var path = std.ArrayList(u8){};
            defer path.deinit();
            
            if (try self.detectCycleRecursive(module, &visited, &rec_stack, &path)) {
                has_cycles = true;
                
                print("🔄 Cycle detected involving module '{s}':\n", .{module});
                for (path.items, 0..) |cycle_module, i| {
                    if (i == path.items.len - 1) {
                        print("  {s} -> {s} (cycle)\n", .{ cycle_module, path.items[0] });
                    } else {
                        print("  {s} -> \n", .{cycle_module});
                    }
                }
                print("\n", .{});
            }
        }
        
        return has_cycles;
    }
    
    fn detectCycleRecursive(
        self: *SafeModuleLoader,
        module: []const u8,
        visited: *HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        rec_stack: *HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        path: *ArrayList([]const u8)
    ) !bool {
        try visited.put(module, true);
        try rec_stack.put(module, true);
        try path.append(allocator, module);
        
        if (self.dependency_graph.get(module)) |dependencies| {
            for (dependencies.items) |dep| {
                if (rec_stack.get(dep) orelse false) {
                    // Found cycle
                    try path.append(allocator, dep);
                    return true;
                }
                
                if (!(visited.get(dep) orelse false)) {
                    if (try self.detectCycleRecursive(dep, visited, rec_stack, path)) {
                        return true;
                    }
                }
            }
        }
        
        _ = rec_stack.remove(module);
        _ = path.pop();
        return false;
    }
    
    /// Debug: Print current module states
    pub fn printModuleStates(self: *SafeModuleLoader) void {
        print("\n=== Module States Debug ===\n", .{});
        
        var iter = self.module_states.iterator();
        while (iter.next()) |entry| {
            const module = entry.key_ptr.*;
            const state = entry.value_ptr.*;
            const ref_count = self.reference_counts.get(module) orelse 0;
            
            print("Module: {s} | State: {s} | Refs: {s}\n", .{ module, @tagName(state), ref_count });
        }
        
        print("\n=== Dependency Graph ===\n", .{});
        var deps_iter = self.dependency_graph.iterator();
        while (deps_iter.next()) |entry| {
            const module = entry.key_ptr.*;
            const deps = entry.value_ptr.*;
            
            print("{s} depends on: ", .{module});
            for (deps.items, 0..) |dep, i| {
                if (i > 0) print(", ", .{});
                print("{s}", .{dep});
            }
            print("\n", .{});
        }
        print("\n", .{});
    }
};

// ===== Test Functions =====

test "safe module loader basic functionality" {
    const allocator = std.testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Test state tracking
    try loader.setModuleState("test_module", .in_progress);
    const state = loader.module_states.get("test_module");
    try std.testing.expect(state != null);
    try std.testing.expect(state.? == .in_progress);
}

test "cycle detection with 3 modules" {
    const allocator = std.testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, false);
    defer loader.deinit();
    
    // Create A -> B -> C -> A cycle
    loader.recordDependency("A", "B");
    loader.recordDependency("B", "C");
    loader.recordDependency("C", "A");
    
    const has_cycles = try loader.detectCycles();
    try std.testing.expect(has_cycles);
}

test "reference counting" {
    const allocator = std.testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, false);
    defer loader.deinit();
    
    // Test reference counting
    try loader.setRefCount("test_module", 1);
    loader.incrementRefCount("test_module");
    
    const count = loader.reference_counts.get("test_module");
    try std.testing.expect(count != null);
    try std.testing.expect(count.? == 2);
    
    const new_count = loader.decrementRefCount("test_module");
    try std.testing.expect(new_count == 1);
}
