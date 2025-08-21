const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const type_system_runtime = @import("type_system_runtime.zig");
const module_loader = @import("module_loader.zig");

// History persistence constants
const HISTORY_FILE_NAME = ".cursed_history";
const HISTORY_BACKUP_SUFFIX = ".backup";
const HISTORY_TEMP_SUFFIX = ".tmp";
const MAX_HISTORY_ENTRIES = 1000;

// Import Variable and VariableStore from main_unified.zig
const main = @import("main_unified.zig");
const Variable = main.Variable;
const VariableStore = main.VariableStore;
const FunctionStore = main.FunctionStore;
const StructStore = main.StructStore;

/// CURSED REPL session manager
pub const ReplSession = struct {
    variables: VariableStore,
    functions: FunctionStore,
    structs: StructStore,
    history: ArrayList([]const u8),
    allocator: Allocator,
    verbose: bool,
    line_number: u32,
    history_file_path: ?[]const u8,
    
    pub fn init(allocator: Allocator, verbose: bool) ReplSession {
        return ReplSession{
            .variables = VariableStore.init(allocator),
            .functions = FunctionStore.init(allocator),
            .structs = StructStore.init(allocator),
            .history = .empty,
            .allocator = allocator,
            .verbose = verbose,
            .line_number = 1,
            .history_file_path = null,
        };
    }
    
    pub fn deinit(self: *ReplSession) void {
        // Clean up variables
        var var_iter = self.variables.iterator();
        while (var_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.variables.deinit();
        
        // Clean up functions
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.functions.deinit();
        
        // Clean up structs
        var struct_iter = self.structs.iterator();
        while (struct_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.structs.deinit();
        
        // Clean up history
        for (self.history.items) |line| {
            self.allocator.free(line);
        }
        self.history.deinit();
        
        // Clean up history file path
        if (self.history_file_path) |path| {
            self.allocator.free(path);
        }
    }
    
    /// Initialize history persistence with robust file handling
    pub fn initHistoryPersistence(self: *ReplSession, custom_path: ?[]const u8) !void {
        // Determine history file path
        const path = if (custom_path) |p| 
            try self.allocator.dupe(u8, p)
        else 
            try self.getDefaultHistoryPath();
        
        self.history_file_path = path;
        
        // Perform crash recovery check
        try self.recoverFromCrash();
        
        // Load existing history
        try self.loadHistory();
    }
    
    /// Get default history file path (in user's home directory)
    fn getDefaultHistoryPath(self: *ReplSession) ![]const u8 {
        const home_dir = std.process.getEnvVarOwned(self.allocator, "HOME") catch {
            // Fallback to current directory if HOME is not available
            return try self.allocator.dupe(u8, HISTORY_FILE_NAME);
        };
        defer self.allocator.free(home_dir);
        
        return try std.fs.path.join(self.allocator, &[_][]const u8{ home_dir, HISTORY_FILE_NAME });
    }
    
    /// Recover from potential crash by checking for incomplete writes
    fn recoverFromCrash(self: *ReplSession) !void {
        if (self.history_file_path == null) return;
        
        const history_path = self.history_file_path.?;
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        // Check for temporary file (indicates interrupted write)
        const temp_path = try std.fmt.allocPrint(arena_allocator, "{s}{s}", .{ history_path, HISTORY_TEMP_SUFFIX });
        const backup_path = try std.fmt.allocPrint(arena_allocator, "{s}{s}", .{ history_path, HISTORY_BACKUP_SUFFIX });
        
        // If temp file exists, we had an interrupted write
        std.fs.cwd().access(temp_path, .{}) catch |err| switch (err) {
            error.FileNotFound => {
                // No temp file, check if backup is newer than main file
                const main_stat = std.fs.cwd().statFile(history_path) catch null;
                const backup_stat = std.fs.cwd().statFile(backup_path) catch null;
                
                if (main_stat != null and backup_stat != null) {
                    if (backup_stat.?.mtime > main_stat.?.mtime) {
                        // Backup is newer, restore it
                        std.fs.cwd().rename(backup_path, history_path) catch {};
                        if (self.verbose) {
                            print("✅ Restored newer backup history\n", .{});
                        }
                    }
                }
                return;
            },
            else => return,
        };
        
        // Temp file exists, we had an interrupted write
        if (self.verbose) {
            print("🔧 Recovering from interrupted history write...\n", .{});
        }
        
        // Remove the incomplete temp file
        std.fs.cwd().deleteFile(temp_path) catch {};
        
        // If backup exists, restore it
        std.fs.cwd().access(backup_path, .{}) catch {
            return; // No backup available
        };
        
        std.fs.cwd().rename(backup_path, history_path) catch {};
        if (self.verbose) {
            print("✅ History recovered from backup\n", .{});
        }
    }
    
    /// Load history from file with corruption handling
    fn loadHistory(self: *ReplSession) !void {
        if (self.history_file_path == null) return;
        
        const file = std.fs.cwd().openFile(self.history_file_path.?, .{}) catch |err| {
            if (err == error.FileNotFound) {
                // File doesn't exist yet, that's fine
                return;
            }
            return err;
        };
        defer file.close();
        
        const file_size = try file.getEndPos();
        if (file_size == 0) {
            if (self.verbose) {
                print("⚠️  History file is empty (possible corruption)\n", .{});
            }
            return;
        }
        
        const content = try file.readToEndAlloc(self.allocator, 10 * 1024 * 1024); // Max 10MB
        defer self.allocator.free(content);
        
        var lines = std.mem.splitScalar(u8, content, '\n');
        var loaded_count: usize = 0;
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (trimmed.len == 0) continue;
            
            // Validate line doesn't contain null bytes or other corruption
            if (std.mem.indexOfScalar(u8, trimmed, 0) != null) {
                if (self.verbose) {
                    print("⚠️  Skipping corrupted history line\n", .{});
                }
                continue;
            }
            
            const history_line = try self.allocator.dupe(u8, trimmed);
            try self.history.append(self.allocator, history_line);
            loaded_count += 1;
            
            // Prevent excessive memory usage
            if (loaded_count >= MAX_HISTORY_ENTRIES) {
                break;
            }
        }
        
        if (self.verbose and loaded_count > 0) {
            print("📜 Loaded {} history entries\n", .{loaded_count});
        }
    }
    
    /// Save history with atomic write and backup
    fn saveHistory(self: *ReplSession) !void {
        if (self.history_file_path == null or self.history.items.len == 0) return;
        
        const history_path = self.history_file_path.?;
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        const temp_path = try std.fmt.allocPrint(arena_allocator, "{s}{s}", .{ history_path, HISTORY_TEMP_SUFFIX });
        const backup_path = try std.fmt.allocPrint(arena_allocator, "{s}{s}", .{ history_path, HISTORY_BACKUP_SUFFIX });
        
        // Create backup of existing history file
        std.fs.cwd().access(history_path, .{}) catch {
            // File doesn't exist yet, no backup needed
        };
        std.fs.cwd().copyFile(history_path, std.fs.cwd(), backup_path, .{}) catch {};
        
        // Write to temporary file first (atomic operation)
        const temp_file = try std.fs.cwd().createFile(temp_path, .{});
        defer temp_file.close();
        
        // Only save the most recent entries to prevent unlimited growth
        const start_idx = if (self.history.items.len > MAX_HISTORY_ENTRIES) 
            self.history.items.len - MAX_HISTORY_ENTRIES 
        else 
            0;
        
        for (self.history.items[start_idx..]) |line| {
            try temp_file.writeAll(line);
            try temp_file.writeAll("\n");
        }
        
        try temp_file.sync(); // Ensure data is written to disk
        
        // Atomically replace the history file
        try std.fs.cwd().rename(temp_path, history_path);
        
        if (self.verbose) {
            print("💾 History saved ({} entries)\n", .{self.history.items.len - start_idx});
        }
    }
    
    /// Add entry to history and immediately persist it
    fn addToHistory(self: *ReplSession, line: []const u8) !void {
        // Don't save empty lines or duplicates
        if (line.len == 0) return;
        if (self.history.items.len > 0 and std.mem.eql(u8, self.history.items[self.history.items.len - 1], line)) {
            return;
        }
        
        const history_line = try self.allocator.dupe(u8, line);
        try self.history.append(self.allocator, history_line);
        
        // Immediately persist for crash safety
        try self.saveHistory();
    }
    
    /// Evaluate a CURSED expression in the REPL context
    pub fn evaluate(self: *ReplSession, input: []const u8) !?Variable {
        const trimmed = std.mem.trim(u8, input, " \t\r\n");
        if (trimmed.len == 0) return null;
        
        // Add to history with robust persistence
        try self.addToHistory(trimmed);
        
        // Try to parse and evaluate the input
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        // First try: parse as a complete statement
        if (try self.tryParseStatement(arena_allocator, trimmed)) |result| {
            return result;
        }
        
        // Second try: parse as an expression
        if (try self.tryParseExpression(arena_allocator, trimmed)) |result| {
            return result;
        }
        
        // If both fail, return an error
        return error.ParseError;
    }
    
    /// Try to parse input as a complete statement
    fn tryParseStatement(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !?Variable {
        // Handle variable declarations: sus x drip = 42
        if (std.mem.startsWith(u8, input, "sus ")) {
            return try self.handleVariableDeclaration(arena_allocator, input);
        }
        
        // Handle function definitions: slay func_name(params) type { body }
        if (std.mem.startsWith(u8, input, "slay ")) {
            return try self.handleFunctionDefinition(arena_allocator, input);
        }
        
        // Handle print statements: vibez.spill(...)
        if (std.mem.indexOf(u8, input, "vibez.spill(")) |_| {
            return try self.handlePrintStatement(arena_allocator, input);
        }
        
        // Handle module imports: yeet "module_name"
        if (std.mem.startsWith(u8, input, "yeet ")) {
            return try self.handleModuleImport(arena_allocator, input);
        }
        
        // Handle assignments: var = value
        if (std.mem.indexOf(u8, input, "=")) |equals_pos| {
            return try self.handleAssignment(arena_allocator, input, equals_pos);
        }
        
        return null;
    }
    
    /// Try to parse input as an expression
    fn tryParseExpression(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !?Variable {
        // Use the existing expression evaluation from main_unified.zig
        const main_module = @import("main_unified.zig");
        
        // Create a temporary variable to hold the result
        var result = main_module.evaluateExpression(&self.variables, &self.functions, arena_allocator, input, self.verbose) catch {
            return null;
        };
        
        // Clone the result so it persists beyond the arena
        return try result.clone(self.allocator);
    }
    
    /// Handle variable declarations
    fn handleVariableDeclaration(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        _ = arena_allocator;
        
        // Parse: sus variable_name type = value
        const declaration = std.mem.trim(u8, input[4..], " \t"); // Remove "sus "
        
        if (std.mem.indexOf(u8, declaration, "=")) |equals_pos| {
            const left_side = std.mem.trim(u8, declaration[0..equals_pos], " \t");
            const value_expr = std.mem.trim(u8, declaration[equals_pos + 1..], " \t");
            
            // Parse variable name and type
            var parts = std.mem.splitScalar(u8, left_side, ' ');
            const var_name = parts.next() orelse return error.InvalidSyntax;
            const var_type = parts.next(); // Optional type annotation
            
            if (var_type != null and self.verbose) {
                print("  📝 Variable type: {s}\n", .{var_type.?});
            }
            
            // Evaluate the value expression
            const main_module = @import("main_unified.zig");
            const value = try main_module.evaluateExpression(&self.variables, &self.functions, self.allocator, value_expr, self.verbose);
            
            // Store the variable
            const var_name_copy = try self.allocator.dupe(u8, var_name);
            try self.variables.put(var_name_copy, value);
            
            if (self.verbose) {
                print("  ✅ Variable declared: {s} = {any}\n", .{ var_name, value });
            }
            
            return value;
        }
        
        return error.InvalidSyntax;
    }
    
    /// Handle function definitions
    fn handleFunctionDefinition(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        _ = arena_allocator;
        
        // For now, just acknowledge the function definition
        // Full function parsing would require more complex AST handling
        const func_text = std.mem.trim(u8, input[5..], " \t"); // Remove "slay "
        
        if (std.mem.indexOf(u8, func_text, "(")) |paren_pos| {
            const func_name = std.mem.trim(u8, func_text[0..paren_pos], " \t");
            print("  📝 Function definition: {s}\n", .{func_name});
            
            // Store a placeholder function for now
            const name_copy = try self.allocator.dupe(u8, func_name);
            const func_def = main.FunctionDefinition.init(self.allocator, name_copy);
            try self.functions.put(name_copy, func_def);
            
            const result_str = try std.fmt.allocPrint(self.allocator, "Function '{s}' defined", .{func_name});
            return Variable{ .String = main.ManagedString.fromOwned(result_str) };
        }
        
        return error.InvalidSyntax;
    }
    
    /// Handle print statements
    fn handlePrintStatement(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        const main_module = @import("main_unified.zig");
        
        // Execute the print statement using the existing handler
        try main_module.handleVibesSpill(&self.variables, &self.functions, arena_allocator, input, 0, self.verbose);
        
        return Variable{ .String = main.ManagedString.fromLiteral("") };
    }
    
    /// Handle module imports
    fn handleModuleImport(self: *ReplSession, arena_allocator: Allocator, input: []const u8) !Variable {
        _ = arena_allocator;
        
        // Parse: yeet "module_name"
        const import_text = std.mem.trim(u8, input[5..], " \t"); // Remove "yeet "
        
        if (import_text.len >= 2 and import_text[0] == '"' and import_text[import_text.len - 1] == '"') {
            const module_name = import_text[1..import_text.len - 1];
            
            // Try to load the module using existing module loader
            const simple_resolver = @import("simple_import_resolver.zig");
            _ = simple_resolver.resolveImport(self.allocator, module_name, "stdlib", self.verbose) catch |err| {
                if (self.verbose) {
                    print("  ⚠️  Module import warning: {any}\n", .{err});
                }
                const result_str = try std.fmt.allocPrint(self.allocator, "Module '{s}' not found", .{module_name});
                return Variable{ .String = main.ManagedString.fromOwned(result_str) };
            };
            
            const result_str = try std.fmt.allocPrint(self.allocator, "Module '{s}' imported", .{module_name});
            return Variable{ .String = main.ManagedString.fromOwned(result_str) };
        }
        
        return error.InvalidSyntax;
    }
    
    /// Handle assignments
    fn handleAssignment(self: *ReplSession, arena_allocator: Allocator, input: []const u8, equals_pos: usize) !Variable {
        const var_name = std.mem.trim(u8, input[0..equals_pos], " \t");
        const value_expr = std.mem.trim(u8, input[equals_pos + 1..], " \t");
        
        // Evaluate the value expression
        const main_module = @import("main_unified.zig");
        const value = try main_module.evaluateExpression(&self.variables, &self.functions, arena_allocator, value_expr, self.verbose);
        
        // Clone the value to persist beyond arena
        const persistent_value = try value.clone(self.allocator);
        
        // Update or create the variable
        if (self.variables.getPtr(var_name)) |existing| {
            existing.deinit();
            existing.* = persistent_value;
        } else {
            const var_name_copy = try self.allocator.dupe(u8, var_name);
            try self.variables.put(var_name_copy, persistent_value);
        }
        
        if (self.verbose) {
            print("  ✅ Assignment: {s} = {any}\n", .{ var_name, persistent_value });
        }
        
        return try persistent_value.clone(self.allocator);
    }
    
    /// Show variables in the current session
    pub fn showVariables(self: *ReplSession) void {
        if (self.variables.count() == 0) {
            print("  No variables defined\n", .{});
            return;
        }
        
        print("  Current variables:\n", .{});
        var iter = self.variables.iterator();
        while (iter.next()) |entry| {
            const var_str = entry.value_ptr.toString(self.allocator) catch "???";
            defer if (!std.mem.eql(u8, var_str, "???")) self.allocator.free(var_str);
            print("    {s} = {s}\n", .{ entry.key_ptr.*, var_str });
        }
    }
    
    /// Show command history
    pub fn showHistory(self: *ReplSession) void {
        if (self.history.items.len == 0) {
            print("  No command history\n", .{});
            return;
        }
        
        print("  Command history:\n", .{});
        for (self.history.items, 0..) |line, i| {
            print("  {:3}: {s}\n", .{ i + 1, line });
        }
    }
    
    /// Clear the screen
    pub fn clearScreen(self: *ReplSession) void {
        _ = self;
        // ANSI escape sequence to clear screen and move cursor to top-left
        print("\x1B[2J\x1B[1;1H", .{});
    }
};

/// CURSED REPL implementation
pub fn runRepl(allocator: Allocator, verbose: bool) !void {
    return runReplWithHistory(allocator, verbose, null);
}

/// CURSED REPL implementation with custom history file
pub fn runReplWithHistory(allocator: Allocator, verbose: bool, history_file: ?[]const u8) !void {
    var session = ReplSession.init(allocator, verbose);
    defer session.deinit();
    
    // Initialize robust history persistence
    session.initHistoryPersistence(history_file) catch |err| {
        if (verbose) {
            print("⚠️  History persistence disabled: {any}\n", .{err});
        }
    };
    
    // Set up signal handler for graceful shutdown
    setupSignalHandler(&session) catch |err| {
        if (verbose) {
            print("⚠️  Signal handler setup failed: {any}\n", .{err});
        }
    };
    
    // Print welcome message
    printWelcome();
    
    // Main REPL loop
    var stdin = std.fs.File.stdin().reader(&[_]u8{});
    
    while (true) {
        // Print prompt
        print("{s}", .{"cursed> "});
        
        // Read input
        var input_buffer: [1024]u8 = undefined;
        const input = (try stdin.readUntilDelimiterOrEof(input_buffer[0..], '\n')) orelse break;
        
        // Handle special commands
        if (handleSpecialCommand(&session, input)) |should_exit| {
            if (should_exit) break;
            continue;
        }
        
        // Evaluate the input
        if (session.evaluate(input)) |result| {
            if (result) |value| {
                const value_str = value.toString(allocator) catch "???";
                defer if (!std.mem.eql(u8, value_str, "???")) allocator.free(value_str);
                
                // Only print non-empty results
                if (!std.mem.eql(u8, value_str, "")) {
                    print("{s}\n", .{value_str});
                }
                
                // Clean up the result
                var temp_value = value;
                temp_value.deinit();
            }
        } else |err| {
            switch (err) {
                error.ParseError => print("Error: Invalid syntax\n", .{}),
                else => print("Error: {any}\n", .{err}),
            }
        }
        
        session.line_number += 1;
    }
    
    // Final history save on normal exit
    session.saveHistory() catch |err| {
        if (verbose) {
            print("⚠️  Failed to save history on exit: {any}\n", .{err});
        }
    };
    
    print("Goodbye!\n", .{});
}

/// Handle special REPL commands
fn handleSpecialCommand(session: *ReplSession, input: []const u8) ?bool {
    const trimmed = std.mem.trim(u8, input, " \t\r\n");
    
    if (std.mem.eql(u8, trimmed, ":quit") or std.mem.eql(u8, trimmed, ":exit") or std.mem.eql(u8, trimmed, ":q")) {
        return true; // Signal to exit
    }
    
    if (std.mem.eql(u8, trimmed, ":help") or std.mem.eql(u8, trimmed, ":h")) {
        printHelp();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":vars") or std.mem.eql(u8, trimmed, ":variables")) {
        session.showVariables();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":history") or std.mem.eql(u8, trimmed, ":hist")) {
        session.showHistory();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":clear") or std.mem.eql(u8, trimmed, ":cls")) {
        session.clearScreen();
        return false;
    }
    
    if (std.mem.eql(u8, trimmed, ":version")) {
        print("CURSED REPL v1.0.0 (Zig implementation)\n", .{});
        return false;
    }
    
    if (std.mem.startsWith(u8, trimmed, ":")) {
        print("Unknown command: {s}. Type :help for available commands.\n", .{trimmed});
        return false;
    }
    
    return null; // Not a special command
}

/// Print welcome message
fn printWelcome() void {
    print("🔥 CURSED REPL v1.0.0\n", .{});
    print("Interactive CURSED language shell\n", .{});
    print("Type :help for help, :quit to exit\n", .{});
    print("\n", .{});
}

/// Print help message
fn printHelp() void {
    print("\n", .{});
    print("CURSED REPL Commands:\n", .{});
    print("  :help, :h         - Show this help message\n", .{});
    print("  :quit, :exit, :q  - Exit the REPL\n", .{});
    print("  :vars, :variables - Show current variables\n", .{});
    print("  :history, :hist   - Show command history\n", .{});
    print("  :clear, :cls      - Clear the screen\n", .{});
    print("  :version          - Show version information\n", .{});
    print("\n", .{});
    print("CURSED Language Features:\n", .{});
    print("  Variables:  sus x drip = 42\n", .{});
    print("  Functions:  slay add(a drip, b drip) drip {{ damn a + b }}\n", .{});
    print("  Arrays:     sus arr []drip = [1, 2, 3]\n", .{});
    print("  Print:      vibez.spill(\"Hello, world!\")\n", .{});
    print("  Import:     yeet \"stdlib_module\"\n", .{});
    print("  Control:    ready (condition) {{ ... }}\n", .{});
    print("              bestie (condition) {{ ... }}\n", .{});
    print("\n", .{});
}

// Global session pointer for signal handler
var global_session: ?*ReplSession = null;

/// Signal handler for graceful shutdown and history preservation
fn signalHandler(sig: c_int) callconv(.C) void {
    if (global_session) |session| {
        session.saveHistory() catch {};
    }
    
    switch (sig) {
        std.posix.SIG.INT => {
            print("\n💾 History saved. Goodbye!\n", .{});
            std.process.exit(0);
        },
        std.posix.SIG.TERM => {
            print("\n💾 History saved. Terminated.\n", .{});
            std.process.exit(0);
        },
        else => {},
    }
}

/// Setup signal handlers for crash recovery
fn setupSignalHandler(session: *ReplSession) !void {
    global_session = session;
    
    // Set up signal handlers for graceful shutdown
    const act = std.posix.Sigaction{
        .handler = .{ .handler = signalHandler },
        .mask = std.posix.empty_sigset,
        .flags = 0,
    };
    
    _ = std.posix.sigaction(std.posix.SIG.INT, &act, null);
    _ = std.posix.sigaction(std.posix.SIG.TERM, &act, null);
}
