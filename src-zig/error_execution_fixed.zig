const std = @import("std");
const main_unified = @import("main_unified.zig");
const Variable = main_unified.Variable;

/// Fixed error handling execution for CURSED language  
/// Properly implements yikes/shook/fam error semantics

pub const ErrorState = struct {
    current_error: ?ErrorValue,
    allocator: Allocator,
    
    const Allocator = std.mem.Allocator;
    
    pub fn init() ErrorState {
        return ErrorState{
            .current_error = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ErrorState) void {
        if (self.current_error) |*err| {
            err.deinit();
        }
    }
    
    pub fn pushError(self: *ErrorState, error_val: ErrorValue) !void {
        self.current_error = error_val;
    }
    
    pub fn popError(self: *ErrorState) ?ErrorValue {
        const current = self.current_error;
        self.current_error = null;
        return current;
    }
    
    pub fn hasError(self: *ErrorState) bool {
        return self.current_error != null;
    }
    
    pub fn clearError(self: *ErrorState) void {
        if (self.current_error) |*err| {
            err.deinit();
        }
        self.current_error = null;
    }
};

pub const ErrorValue = struct {
    message: []const u8,
    error_type: []const u8,
    line: u32,
    column: u32,
    file: []const u8,
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator, message: []const u8, error_type: []const u8, line: u32, column: u32, file: []const u8) !ErrorValue {
        return ErrorValue{
            .message = try allocator.dupe(u8, message),
            .error_type = try allocator.dupe(u8, error_type),
            .line = line,
            .column = column,
            .file = try allocator.dupe(u8, file),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ErrorValue) void {
        self.allocator.free(self.message);
        self.allocator.free(self.error_type);
        self.allocator.free(self.file);
    }
    
    pub fn toString(self: *const ErrorValue, allocator: std.mem.Allocator) ![]const u8 {
        return std.fmt.allocPrint(allocator, 
            "Error[{s}]: {s} at {s}:{d}:{d}", 
            .{ self.error_type, self.message, self.file, self.line, self.column });
    }
};

pub const ExecutionResult = union(enum) {
    Ok: void,
    Error: ErrorValue,
    Exception: ErrorValue,  // For yikes statements that should terminate
};

/// Execute a yikes statement - creates and throws an error
pub fn executeYikes(
    error_state: *ErrorState,
    allocator: std.mem.Allocator,
    variables: anytype,
    _: anytype,
    message_expr: []const u8,
    line: u32,
    column: u32,
    file: []const u8,
    verbose: bool
) !ExecutionResult {
    if (verbose) std.debug.print("🚨 Executing yikes: {s}\n", .{message_expr});
    
    // Evaluate the message expression
    const message = blk: {
        if (std.mem.startsWith(u8, message_expr, "\"") and std.mem.endsWith(u8, message_expr, "\"")) {
            // String literal
            const unquoted = message_expr[1..message_expr.len-1];
            break :blk try allocator.dupe(u8, unquoted);
        } else if (variables.get(message_expr)) |var_val| {
            // Variable lookup
            switch (var_val) {
                .String => |s| {
                    break :blk try allocator.dupe(u8, s.data);
                },
                else => break :blk try allocator.dupe(u8, "Invalid error message type"),
            }
        } else {
            // Fallback
            break :blk try allocator.dupe(u8, message_expr);
        }
    };
    defer allocator.free(message);
    
    // Create error value
    const error_val = try ErrorValue.init(
        allocator, 
        message, 
        "RuntimeError", 
        line, 
        column, 
        file
    );
    
    // Push error to state
    try error_state.pushError(error_val);
    
    if (verbose) std.debug.print("🚨 Error created: {s}\n", .{message});
    
    return ExecutionResult{ .Exception = error_val };
}

/// Execute a fam try-catch block  
pub fn executeFamBlock(
    error_state: *ErrorState,
    allocator: std.mem.Allocator,
    variables: anytype,
    functions: anytype,
    try_statements: []const []const u8,
    error_variable: ?[]const u8,
    catch_statements: []const []const u8,
    executeStatement: anytype,
    verbose: bool
) !ExecutionResult {
    if (verbose) std.debug.print("🔧 Executing fam try-catch block\n", .{});
    
    // Clear any existing error
    error_state.clearError();
    
    // Execute try block
    for (try_statements) |stmt| {
        if (std.mem.trim(u8, stmt, " \t\r\n").len == 0) continue;
        
        const result = executeStatement(variables, functions, allocator, stmt, verbose) 
            catch |err| {
                if (verbose) std.debug.print("❌ Error in try block: {}\n", .{err});
                const error_msg = @errorName(err);
                const error_val = ErrorValue.init(
                    allocator, 
                    error_msg, 
                    "RuntimeError", 
                    0, 0, 
                    "<unknown>"
                ) catch return ExecutionResult{ .Error = undefined };
                
                try error_state.pushError(error_val);
                break;
            };
        
        // Handle specific result types
        switch (result) {
            .Exception => |err_val| {
                if (verbose) std.debug.print("🚨 Exception caught in try block\n", .{});
                try error_state.pushError(err_val);
                break;
            },
            .Error => |err_val| {
                if (verbose) std.debug.print("⚠️ Error caught in try block\n", .{});
                try error_state.pushError(err_val);
                break;
            },
            .Ok => continue,
        }
    }
    
    // If error occurred, execute catch block
    if (error_state.hasError()) {
        if (verbose) std.debug.print("🔧 Error occurred, executing catch block\n", .{});
        
        // Bind error variable if specified
        if (error_variable) |err_var| {
            if (error_state.current_error) |current_err| {
                const error_msg_str = try current_err.toString(allocator);
                defer allocator.free(error_msg_str);
                
                // Create string variable for error message
                const StringVariable = Variable;
                const string_var = StringVariable{ 
                .String = .{ 
                .data = try allocator.dupe(u8, current_err.message),
                .owned = true
                }
                };
                
                try variables.put(err_var, string_var);
                
                if (verbose) std.debug.print("📌 Bound error variable '{s}' = '{s}'\n", .{err_var, current_err.message});
            }
        }
        
        // Execute catch statements
        for (catch_statements) |stmt| {
            if (std.mem.trim(u8, stmt, " \t\r\n").len == 0) continue;
            
            _ = executeStatement(variables, functions, allocator, stmt, verbose) 
                catch |err| {
                    if (verbose) std.debug.print("❌ Error in catch block: {}\n", .{err});
                    continue;
                };
        }
        
        // Clear the handled error
        error_state.clearError();
    }
    
    if (verbose) std.debug.print("✅ Fam block completed\n", .{});
    return ExecutionResult{ .Ok = {} };
}
