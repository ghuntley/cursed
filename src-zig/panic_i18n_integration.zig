const std = @import("std");
const panic_i18n = @import("panic_i18n.zig");
const error_operators = @import("error_operators.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

/// Integration layer between CURSED error handling and i18n system
/// Provides localized panic message formatting for yikes/shook/fam operators

const I18nManager = panic_i18n.I18nManager;
const Locale = panic_i18n.Locale;
const UnicodeUtils = panic_i18n.UnicodeUtils;
const YikesError = error_operators.YikesError;
const ShookResult = error_operators.ShookResult;

/// Enhanced panic context with i18n support
pub const I18nPanicContext = struct {
    source_location: ?SourceLocation,
    error_code: i64,
    error_type: ErrorType,
    custom_data: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    stack_trace: ?[][]const u8,
    allocator: Allocator,

    pub const SourceLocation = struct {
        function: []const u8,
        file: []const u8,
        line: u32,
        column: u32,
    };

    pub const ErrorType = enum {
        Yikes,           // General error
        Shook,           // Panic
        Fam,             // Exception
        Memory,          // Memory error
        Type,            // Type error
        Runtime,         // Runtime error
        DivisionByZero,  // Division by zero
        IndexOutOfBounds, // Array bounds error
        NullPointer,     // Null pointer
        InvalidOperation, // Invalid operation
        Unknown,         // Unknown error

        pub fn getMessageKey(self: ErrorType) []const u8 {
            return switch (self) {
                .Yikes => "panic.yikes",
                .Shook => "panic.shook",
                .Fam => "panic.fam",
                .Memory => "panic.memory_error",
                .Type => "panic.type_error",
                .Runtime => "panic.runtime_error",
                .DivisionByZero => "panic.division_by_zero",
                .IndexOutOfBounds => "panic.index_out_of_bounds",
                .NullPointer => "panic.null_pointer",
                .InvalidOperation => "panic.invalid_operation",
                .Unknown => "panic.unknown_error",
            };
        }
    };

    pub fn init(allocator: Allocator, error_type: ErrorType, code: i64) I18nPanicContext {
        return I18nPanicContext{
            .source_location = null,
            .error_code = code,
            .error_type = error_type,
            .custom_data = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .stack_trace = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *I18nPanicContext) void {
        var iterator = self.custom_data.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.custom_data.deinit();

        if (self.stack_trace) |trace| {
            for (trace) |frame| {
                self.allocator.free(frame);
            }
            self.allocator.free(trace);
        }
    }

    pub fn setLocation(self: *I18nPanicContext, function: []const u8, file: []const u8, line: u32, column: u32) !void {
        self.source_location = SourceLocation{
            .function = try self.allocator.dupe(u8, function),
            .file = try self.allocator.dupe(u8, file),
            .line = line,
            .column = column,
        };
    }

    pub fn addData(self: *I18nPanicContext, key: []const u8, value: []const u8) !void {
        const owned_key = try self.allocator.dupe(u8, key);
        const owned_value = try self.allocator.dupe(u8, value);
        try self.custom_data.put(owned_key, owned_value);
    }

    pub fn addStackTrace(self: *I18nPanicContext, trace: [][]const u8) !void {
        var owned_trace = try self.allocator.alloc([]u8, trace.len);
        for (trace, 0..) |frame, i| {
            owned_trace[i] = try self.allocator.dupe(u8, frame);
        }
        self.stack_trace = owned_trace;
    }

    pub fn formatMessage(self: I18nPanicContext, i18n: *I18nManager, base_message: []const u8) ![]u8 {
        var values = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer values.deinit();

        // Add base message
        try values.put("message", base_message);

        // Add error code
        const code_str = try std.fmt.allocPrint(self.allocator, "{}", .{self.error_code});
        defer self.allocator.free(code_str);
        try values.put("code", code_str);

        // Add custom data
        var custom_iterator = self.custom_data.iterator();
        while (custom_iterator.next()) |entry| {
            try values.put(entry.key_ptr.*, entry.value_ptr.*);
        }

        // Format main error message
        const message_key = self.error_type.getMessageKey();
        var result = .empty;
        defer result.deinit();

        const main_message = try i18n.formatPanicMessage(message_key, values);
        defer self.allocator.free(main_message);
        
        try result.appendSlice(main_message);
        try result.append(self.allocator, '\n');

        // Add location information if available
        if (self.source_location) |loc| {
            var location_values = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
            defer location_values.deinit();

            try location_values.put("function", loc.function);
            try location_values.put("file", loc.file);
            
            const line_str = try std.fmt.allocPrint(self.allocator, "{}", .{loc.line});
            defer self.allocator.free(line_str);
            try location_values.put("line", line_str);
            
            const column_str = try std.fmt.allocPrint(self.allocator, "{}", .{loc.column});
            defer self.allocator.free(column_str);
            try location_values.put("column", column_str);

            const location_message = try i18n.formatPanicMessage("panic.location", location_values);
            defer self.allocator.free(location_message);
            
            try result.appendSlice(location_message);
            try result.append(self.allocator, '\n');
        }

        // Add stack trace if available
        if (self.stack_trace) |trace| {
            var empty_values = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
            defer empty_values.deinit();

            const trace_header = try i18n.formatPanicMessage("panic.stack_trace", empty_values);
            defer self.allocator.free(trace_header);
            
            try result.appendSlice(trace_header);
            try result.append(self.allocator, '\n');

            for (trace) |frame| {
                try result.appendSlice("  ");
                try result.appendSlice(frame);
                try result.append(self.allocator, '\n');
            }
        }

        return try self.allocator.dupe(u8, result.items);
    }
};

/// Enhanced YikesError with i18n support
pub const I18nYikesError = struct {
    base_error: YikesError,
    context: I18nPanicContext,
    formatted_message: ?[]u8,

    pub fn init(allocator: Allocator, message: []const u8, code: i64) !I18nYikesError {
        const base_error = try YikesError.init(allocator, message, code);
        const context = I18nPanicContext.init(allocator, .Yikes, code);
        
        return I18nYikesError{
            .base_error = base_error,
            .context = context,
            .formatted_message = null,
        };
    }

    pub fn initWithLocation(
        allocator: Allocator,
        message: []const u8,
        code: i64,
        function: []const u8,
        file: []const u8,
        line: u32,
        column: u32
    ) !I18nYikesError {
        var err = try init(allocator, message, code);
        try err.context.setLocation(function, file, line, column);
        return err;
    }

    pub fn deinit(self: *I18nYikesError) void {
        self.base_error.deinit();
        self.context.deinit();
        if (self.formatted_message) |msg| {
            self.context.allocator.free(msg);
        }
    }

    pub fn addContextData(self: *I18nYikesError, key: []const u8, value: []const u8) !void {
        try self.context.addData(key, value);
    }

    pub fn formatForLocale(self: *I18nYikesError, i18n: *I18nManager) ![]u8 {
        if (self.formatted_message) |msg| {
            self.context.allocator.free(msg);
        }
        
        self.formatted_message = try self.context.formatMessage(i18n, self.base_error.message);
        return try self.context.allocator.dupe(u8, self.formatted_message.?);
    }

    pub fn toString(self: I18nYikesError) ![]u8 {
        if (self.formatted_message) |msg| {
            return try self.context.allocator.dupe(u8, msg);
        }
        return try self.base_error.toString(self.context.allocator);
    }
};

/// Convenience functions for creating localized panic messages
pub const I18nPanicUtils = struct {
    pub fn createYikes(allocator: Allocator, message: []const u8, code: i64) !I18nYikesError {
        return I18nYikesError.init(allocator, message, code);
    }

    pub fn createShook(allocator: Allocator, message: []const u8, code: i64) !I18nYikesError {
        var err = try I18nYikesError.init(allocator, message, code);
        err.context.error_type = .Shook;
        return err;
    }

    pub fn createFam(allocator: Allocator, message: []const u8, code: i64) !I18nYikesError {
        var err = try I18nYikesError.init(allocator, message, code);
        err.context.error_type = .Fam;
        return err;
    }

    pub fn createMemoryError(allocator: Allocator, message: []const u8) !I18nYikesError {
        var err = try I18nYikesError.init(allocator, message, 100);
        err.context.error_type = .Memory;
        return err;
    }

    pub fn createTypeError(allocator: Allocator, message: []const u8) !I18nYikesError {
        var err = try I18nYikesError.init(allocator, message, 200);
        err.context.error_type = .Type;
        return err;
    }

    pub fn createDivisionByZeroError(allocator: Allocator) !I18nYikesError {
        var err = try I18nYikesError.init(allocator, "Division by zero", 300);
        err.context.error_type = .DivisionByZero;
        return err;
    }

    pub fn createIndexOutOfBoundsError(allocator: Allocator, index: i64, size: i64) !I18nYikesError {
        const message = try std.fmt.allocPrint(allocator, "Index {} out of bounds (size: {})", .{ index, size });
        defer allocator.free(message);
        
        var err = try I18nYikesError.init(allocator, message, 400);
        err.context.error_type = .IndexOutOfBounds;
        
        const index_str = try std.fmt.allocPrint(allocator, "{}", .{index});
        defer allocator.free(index_str);
        const size_str = try std.fmt.allocPrint(allocator, "{}", .{size});
        defer allocator.free(size_str);
        
        try err.addContextData("index", index_str);
        try err.addContextData("size", size_str);
        
        return err;
    }

    pub fn createNullPointerError(allocator: Allocator) !I18nYikesError {
        var err = try I18nYikesError.init(allocator, "Null pointer dereference", 500);
        err.context.error_type = .NullPointer;
        return err;
    }

    pub fn createInvalidOperationError(allocator: Allocator, operation: []const u8) !I18nYikesError {
        const message = try std.fmt.allocPrint(allocator, "Invalid operation: {s}", .{operation});
        defer allocator.free(message);
        
        var err = try I18nYikesError.init(allocator, message, 600);
        err.context.error_type = .InvalidOperation;
        try err.addContextData("operation", operation);
        
        return err;
    }
};

/// Runtime panic handler with i18n support
pub const I18nPanicHandler = struct {
    i18n_manager: ?*I18nManager,
    panic_hook: ?*const fn(context: *I18nPanicContext, formatted_message: []const u8) void,
    allocator: Allocator,

    pub fn init() I18nPanicHandler {
        return I18nPanicHandler{
            .i18n_manager = null,
            .panic_hook = null,
            .allocator = allocator,
        };
    }

    pub fn setI18nManager(self: *I18nPanicHandler, i18n: *I18nManager) void {
        self.i18n_manager = i18n;
    }

    pub fn setPanicHook(self: *I18nPanicHandler, hook: *const fn(context: *I18nPanicContext, formatted_message: []const u8) void) void {
        self.panic_hook = hook;
    }

    pub fn handlePanic(self: *I18nPanicHandler, error_info: I18nYikesError) noreturn {
        var context = error_info.context;
        
        if (self.i18n_manager) |i18n| {
            if (context.formatMessage(i18n, error_info.base_error.message)) |formatted| {
                defer self.allocator.free(formatted);
                
                if (self.panic_hook) |hook| {
                    hook(&context, formatted);
                }
                
                // Output to stderr with proper Unicode handling
                self.outputUnicodeMessage(formatted);
            } else |_| {
                // Fallback to basic message
                self.outputFallbackMessage(error_info.base_error.message);
            }
        } else {
            // No i18n manager available, use fallback
            self.outputFallbackMessage(error_info.base_error.message);
        }

        std.process.exit(1);
    }

    fn outputUnicodeMessage(self: *I18nPanicHandler, message: []const u8) void {
        _ = self;
        
        // Validate UTF-8 and output appropriately
        if (UnicodeUtils.validateUTF8(message)) {
            std.debug.print("{s}\n", .{message});
        } else {
            // Attempt to clean up invalid UTF-8
            const cleaned = self.cleanInvalidUTF8(message) catch {
                std.debug.print("CURSED PANIC: [Invalid UTF-8 message]\n", .{});
                return;
            };
            defer self.allocator.free(cleaned);
            std.debug.print("{s}\n", .{cleaned});
        }
    }

    fn outputFallbackMessage(self: *I18nPanicHandler, message: []const u8) void {
        _ = self;
        std.debug.print("🚨 CURSED PANIC: {s}\n", .{message});
    }

    fn cleanInvalidUTF8(self: *I18nPanicHandler, input: []const u8) ![]u8 {
        var result = .empty;
        defer result.deinit();

        var i: usize = 0;
        while (i < input.len) {
            if (input[i] < 128) {
                // ASCII character, safe to include
                try result.append(input[i]);
                i += 1;
            } else {
                // Non-ASCII, try to decode as UTF-8
                const remaining = input[i..];
                if (std.unicode.utf8ByteSequenceLength(input[i])) |seq_len| {
                    if (i + seq_len <= input.len) {
                        const sequence = input[i..i+seq_len];
                        if (std.unicode.utf8Decode(sequence)) |_| {
                            // Valid UTF-8 sequence
                            try result.appendSlice(sequence);
                            i += seq_len;
                        } else |_| {
                            // Invalid sequence, replace with replacement character
                            try result.appendSlice("�");
                            i += 1;
                        }
                    } else {
                        // Incomplete sequence at end of input
                        try result.appendSlice("�");
                        break;
                    }
                } else |_| {
                    // Invalid UTF-8 start byte
                    try result.appendSlice("�");
                    i += 1;
                }
            }
        }

        return try self.allocator.dupe(u8, result.items);
    }
};

/// Global panic handler instance
var global_panic_handler: ?I18nPanicHandler = null;
var global_panic_mutex: std.Thread.Mutex = .{};

/// Initialize the global panic handler with i18n support
pub fn initGlobalPanicHandler(allocator: Allocator, locale: ?Locale) !void {
    global_panic_mutex.lock();
    defer global_panic_mutex.unlock();

    // Initialize i18n system
    try panic_i18n.initGlobalI18n(allocator, locale);

    // Initialize panic handler
    global_panic_handler = I18nPanicHandler.init(allocator);
    
    // TODO: Get the global i18n manager and set it
    // global_panic_handler.?.setI18nManager(&global_i18n_manager);
}

/// Deinitialize the global panic handler
pub fn deinitGlobalPanicHandler() void {
    global_panic_mutex.lock();
    defer global_panic_mutex.unlock();

    panic_i18n.deinitGlobalI18n();
    global_panic_handler = null;
}

/// Panic with i18n support - replacement for standard panic
pub fn panicWithI18n(message: []const u8, error_type: I18nPanicContext.ErrorType, code: i64) noreturn {
    const allocator = std.heap.page_allocator;
    
    var panic_error = I18nYikesError.init(allocator, message, code) catch {
        std.debug.print("🚨 CURSED PANIC: {s}\n", .{message});
        std.process.exit(1);
    };
    panic_error.context.error_type = error_type;

    global_panic_mutex.lock();
    defer global_panic_mutex.unlock();

    if (global_panic_handler) |*handler| {
        handler.handlePanic(panic_error);
    } else {
        // Fallback handling
        std.debug.print("🚨 CURSED PANIC: {s}\n", .{message});
        panic_error.deinit();
        std.process.exit(1);
    }
}

/// CURSED-specific panic functions with i18n
pub fn yikesI18n(message: []const u8) noreturn {
    panicWithI18n(message, .Yikes, 1);
}

pub fn shookI18n(message: []const u8, code: i64) noreturn {
    panicWithI18n(message, .Shook, code);
}

pub fn famI18n(message: []const u8) noreturn {
    panicWithI18n(message, .Fam, 10);
}

/// Specialized panic functions
pub fn memoryPanicI18n(message: []const u8) noreturn {
    panicWithI18n(message, .Memory, 100);
}

pub fn typePanicI18n(message: []const u8) noreturn {
    panicWithI18n(message, .Type, 200);
}

pub fn divisionByZeroPanicI18n() noreturn {
    panicWithI18n("Division by zero", .DivisionByZero, 300);
}

pub fn indexOutOfBoundsPanicI18n(index: i64, size: i64) noreturn {
    const allocator = std.heap.page_allocator;
    const message = std.fmt.allocPrint(allocator, "Index {} out of bounds (size: {})", .{ index, size }) catch "Index out of bounds";
    defer if (!std.mem.eql(u8, message, "Index out of bounds")) allocator.free(message);
    panicWithI18n(message, .IndexOutOfBounds, 400);
}

pub fn nullPointerPanicI18n() noreturn {
    panicWithI18n("Null pointer dereference", .NullPointer, 500);
}

pub fn invalidOperationPanicI18n(operation: []const u8) noreturn {
    const allocator = std.heap.page_allocator;
    const message = std.fmt.allocPrint(allocator, "Invalid operation: {s}", .{operation}) catch "Invalid operation";
    defer if (!std.mem.eql(u8, message, "Invalid operation")) allocator.free(message);
    panicWithI18n(message, .InvalidOperation, 600);
}

// Test suite for i18n integration
test "i18n panic context" {
    const allocator = std.testing.allocator;
    
    var context = I18nPanicContext.init(allocator, .Yikes, 42);
    defer context.deinit();

    try context.setLocation("test_function", "test.csd", 10, 5);
    try context.addData("extra", "test_data");

    try std.testing.expect(context.error_code == 42);
    try std.testing.expect(context.error_type == .Yikes);
    try std.testing.expect(context.source_location != null);
}

test "i18n yikes error" {
    const allocator = std.testing.allocator;
    
    var err = try I18nYikesError.init(allocator, "Test error", 123);
    defer err.deinit();

    try err.addContextData("test_key", "test_value");
    try std.testing.expect(err.base_error.getCode() == 123);
    try std.testing.expectEqualStrings("Test error", err.base_error.getMessage());
}

test "panic utils functions" {
    const allocator = std.testing.allocator;
    
    var div_error = try I18nPanicUtils.createDivisionByZeroError(allocator);
    defer div_error.deinit();
    
    try std.testing.expect(div_error.context.error_type == .DivisionByZero);
    try std.testing.expect(div_error.context.error_code == 300);

    var index_error = try I18nPanicUtils.createIndexOutOfBoundsError(allocator, 5, 3);
    defer index_error.deinit();
    
    try std.testing.expect(index_error.context.error_type == .IndexOutOfBounds);
    try std.testing.expect(index_error.context.custom_data.contains("index"));
    try std.testing.expect(index_error.context.custom_data.contains("size"));
}

test "unicode message cleaning" {
    const allocator = std.testing.allocator;
    
    var handler = I18nPanicHandler.init(allocator);
    
    // Test with valid UTF-8
    const valid_utf8 = "Hello 世界";
    try std.testing.expect(UnicodeUtils.validateUTF8(valid_utf8));
    
    // Test with invalid UTF-8 (incomplete sequence)
    const invalid_utf8 = [_]u8{ 'H', 'e', 'l', 'l', 'o', 0xFF, 0xFE };
    const cleaned = try handler.cleanInvalidUTF8(&invalid_utf8);
    defer allocator.free(cleaned);
    
    try std.testing.expect(UnicodeUtils.validateUTF8(cleaned));
    try std.testing.expect(std.mem.startsWith(u8, cleaned, "Hello"));
}
