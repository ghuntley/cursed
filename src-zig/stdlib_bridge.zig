const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const print = std.debug.print;

const Variable = @import("main_unified.zig").Variable;
const stdlib_core = @import("stdlib_core.zig");
const missing_impl = @import("missing_impl_functions.zig");

/// Bridge between CURSED stdlib modules and Zig runtime
/// Converts between CURSED Variable types and native Zig types
/// Provides seamless integration for stdlib function calls

pub const StdlibBridge = struct {
    allocator: Allocator,
    core: *stdlib_core.StdlibCore,
    
    pub fn init(allocator: Allocator) StdlibBridge {
        return StdlibBridge{
            .allocator = allocator,
            .core = stdlib_core.get_stdlib_core(),
        };
    }
    
    // ===== VIBEZ MODULE BRIDGE FUNCTIONS =====
    
    /// Bridge for vibez.spill() - print function
    pub fn vibez_spill(self: *StdlibBridge, message: Variable) !Variable {
        const message_str = try self.variableToString(message);
        defer if (message.type != .String) self.allocator.free(message_str);
        
        self.core.print_string(message_str);
        return Variable{ .type = .Boolean, .boolean = true };
    }
    
    /// Bridge for vibez.spillf() - formatted print
    pub fn vibez_spillf(self: *StdlibBridge, format: Variable, args: []const Variable) !Variable {
        const format_str = try self.variableToString(format);
        defer if (format.type != .String) self.allocator.free(format_str);
        
        // Simple format string replacement
        var output = try self.allocator.dupe(u8, format_str);
        defer self.allocator.free(output);
        
        // Replace %s, %d, %f placeholders
        for (args, 0..) |arg, i| {
            const placeholder = switch (arg.type) {
                .String => "%s",
                .Integer => "%d", 
                .Float => "%f",
                .Boolean => "%s",
                else => continue,
            };
            
            const arg_str = try self.variableToString(arg);
            defer if (arg.type != .String) self.allocator.free(arg_str);
            
            if (std.mem.indexOf(u8, output, placeholder)) |index| {
                const new_output = try std.fmt.allocPrint(self.allocator, "{s}{s}{s}", .{
                    output[0..index],
                    arg_str,
                    output[index + placeholder.len..]
                });
                self.allocator.free(output);
                output = new_output;
                break;
            }
        }
        
        self.core.print_string(output);
        return Variable{ .type = .Boolean, .boolean = true };
    }
    
    /// Bridge for vibez.scanln() - read line
    pub fn vibez_scanln(self: *StdlibBridge) !Variable {
        const input = try self.core.read_line();
        return Variable{ .type = .String, .string = input };
    }
    
    // ===== STRINGZ MODULE BRIDGE FUNCTIONS =====
    
    /// Bridge for stringz.length() - string length
    pub fn stringz_length(self: *StdlibBridge, string: Variable) !Variable {
        const str = try self.variableToString(string);
        defer if (string.type != .String) self.allocator.free(str);
        
        return Variable{ .type = .Integer, .integer = @intCast(str.len) };
    }
    
    /// Bridge for stringz.char_at() - character at index
    pub fn stringz_char_at(self: *StdlibBridge, string: Variable, index: Variable) !Variable {
        const str = try self.variableToString(string);
        defer if (string.type != .String) self.allocator.free(str);
        
        const idx = try self.variableToInt(index);
        const char = self.core.string_char_at(str, @intCast(idx));
        
        const char_str = try self.core.char_to_string(char);
        return Variable{ .type = .String, .string = char_str };
    }
    
    /// Bridge for stringz.concat() - string concatenation
    pub fn stringz_concat(self: *StdlibBridge, str1: Variable, str2: Variable) !Variable {
        const s1 = try self.variableToString(str1);
        defer if (str1.type != .String) self.allocator.free(s1);
        
        const s2 = try self.variableToString(str2);
        defer if (str2.type != .String) self.allocator.free(s2);
        
        const result = try std.fmt.allocPrint(self.allocator, "{s}{s}", .{s1, s2});
        return Variable{ .type = .String, .string = result };
    }
    
    /// Bridge for stringz.substring() - substring extraction
    pub fn stringz_substring(self: *StdlibBridge, string: Variable, start: Variable, length: Variable) !Variable {
        const str = try self.variableToString(string);
        defer if (string.type != .String) self.allocator.free(str);
        
        const start_idx = try self.variableToInt(start);
        const len = try self.variableToInt(length);
        
        if (start_idx < 0 or start_idx >= str.len) {
            return Variable{ .type = .String, .string = try self.allocator.dupe(u8, "") };
        }
        
        const end_idx = @min(@intCast(start_idx + len), str.len);
        const result = try self.allocator.dupe(u8, str[@intCast(start_idx)..end_idx]);
        return Variable{ .type = .String, .string = result };
    }
    
    // ===== MATHZ MODULE BRIDGE FUNCTIONS =====
    
    /// Bridge for mathz.abs_normie() - absolute value for integers
    pub fn mathz_abs_normie(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToInt(value);
        const result = self.core.abs_int(val);
        return Variable{ .type = .Integer, .integer = result };
    }
    
    /// Bridge for mathz.abs_meal() - absolute value for floats
    pub fn mathz_abs_meal(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToFloat(value);
        const result = self.core.abs_float(val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.sqrt_meal() - square root
    pub fn mathz_sqrt_meal(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToFloat(value);
        const result = self.core.sqrt(val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.pow_meal() - power function
    pub fn mathz_pow_meal(self: *StdlibBridge, base: Variable, exponent: Variable) !Variable {
        const base_val = try self.variableToFloat(base);
        const exp_val = try self.variableToFloat(exponent);
        const result = self.core.pow(base_val, exp_val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.sin_meal() - sine function
    pub fn mathz_sin_meal(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToFloat(value);
        const result = self.core.sin(val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.cos_meal() - cosine function
    pub fn mathz_cos_meal(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToFloat(value);
        const result = self.core.cos(val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.ln_meal() - natural logarithm
    pub fn mathz_ln_meal(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToFloat(value);
        const result = self.core.ln(val);
        return Variable{ .type = .Float, .float = result };
    }
    
    // ===== NEW MISSING BRIDGE FUNCTIONS =====
    
    /// Bridge for mathz.pow_meal() - power function using missing impl
    pub fn mathz_pow_meal_impl(self: *StdlibBridge, base: Variable, exponent: Variable) !Variable {
        const base_val = try self.variableToFloat(base);
        const exp_val = try self.variableToFloat(exponent);
        const result = missing_impl.math_pow_impl(base_val, exp_val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.log_meal() - logarithm using missing impl
    pub fn mathz_log_meal_impl(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToFloat(value);
        const result = missing_impl.math_log_impl(val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.sqrt_meal() - square root using missing impl
    pub fn mathz_sqrt_meal_impl(self: *StdlibBridge, value: Variable) !Variable {
        const val = try self.variableToFloat(value);
        const result = missing_impl.math_sqrt_impl(val);
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.random_meal() - random float
    pub fn mathz_random_meal(self: *StdlibBridge) !Variable {
        _ = self;
        const result = missing_impl.random_float_impl();
        return Variable{ .type = .Float, .float = result };
    }
    
    /// Bridge for mathz.random_int_range() - random integer in range
    pub fn mathz_random_int_range(self: *StdlibBridge, min: Variable, max: Variable) !Variable {
        _ = self;
        const min_val = try self.variableToInt(min);
        const max_val = try self.variableToInt(max);
        const result = missing_impl.random_int_range_impl(min_val, max_val);
        return Variable{ .type = .Integer, .integer = result };
    }
    
    // ===== ARRAYZ MODULE BRIDGE FUNCTIONS =====
    
    /// Bridge for arrayz.len() - array length
    pub fn arrayz_len(self: *StdlibBridge, array: Variable) !Variable {
        const length = switch (array.type) {
            .Array => array.array.len,
            .String => array.string.len,
            else => 0,
        };
        return Variable{ .type = .Integer, .integer = @intCast(length) };
    }
    
    /// Bridge for arrayz.append(self.allocator, ) - append to array
    pub fn arrayz_append(self: *StdlibBridge, array: Variable, item: Variable) !Variable {
        _ = self;
        
        switch (array.type) {
            .Array => {
                // Create new array with appended item
                var new_array = try self.allocator.alloc(Variable, array.array.len + 1);
                @memcpy(new_array[0..array.array.len], array.array);
                new_array[array.array.len] = item;
                
                return Variable{ .type = .Array, .array = new_array };
            },
            else => return array, // Return original if not an array
        }
    }
    
    // ===== CONVERSION UTILITIES =====
    
    /// Convert Variable to string
    fn variableToString(self: *StdlibBridge, variable: Variable) ![]const u8 {
        return switch (variable.type) {
            .String => variable.string,
            .Integer => try self.core.int_to_string(variable.integer),
            .Float => try self.core.float_to_string(variable.float),
            .Boolean => if (variable.boolean) "true" else "false",
            .Array => "array",
            .Object => "object",
            .Null => "null",
            else => "unknown",
        };
    }
    
    /// Convert Variable to integer
    fn variableToInt(self: *StdlibBridge, variable: Variable) !i64 {
        return switch (variable.type) {
            .Integer => variable.integer,
            .Float => @intFromFloat(variable.float),
            .String => try self.core.string_to_int(variable.string),
            .Boolean => if (variable.boolean) 1 else 0,
            else => 0,
        };
    }
    
    /// Convert Variable to float
    fn variableToFloat(self: *StdlibBridge, variable: Variable) !f64 {
        return switch (variable.type) {
            .Float => variable.float,
            .Integer => @floatFromInt(variable.integer),
            .String => std.fmt.parseFloat(f64, variable.string),
            .Boolean => if (variable.boolean) 1.0 else 0.0,
            else => 0.0,
        };
    }
    
    // ===== FUNCTION REGISTRY =====
    
    /// Function signature for stdlib bridge functions
    pub const BridgeFunction = *const fn(*StdlibBridge, []const Variable) anyerror!Variable;
    
    /// Registry of all stdlib bridge functions
    pub const function_registry = struct {
        pub const vibez = struct {
            pub fn spill(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Boolean, .boolean = false };
                return try bridge.vibez_spill(args[0]);
            }
            
            pub fn spillf(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 1) return Variable{ .type = .Boolean, .boolean = false };
                return try bridge.vibez_spillf(args[0], args[1..]);
            }
            
            pub fn scanln(bridge: *StdlibBridge, args: []const Variable) !Variable {
                _ = args;
                return try bridge.vibez_scanln();
            }
        };
        
        pub const stringz = struct {
            pub fn length(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Integer, .integer = 0 };
                return try bridge.stringz_length(args[0]);
            }
            
            pub fn char_at(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "") };
                return try bridge.stringz_char_at(args[0], args[1]);
            }
            
            pub fn concat(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "") };
                return try bridge.stringz_concat(args[0], args[1]);
            }
            
            pub fn substring(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 3) return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "") };
                return try bridge.stringz_substring(args[0], args[1], args[2]);
            }
        };
        
        pub const mathz = struct {
            pub fn abs_normie(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Integer, .integer = 0 };
                return try bridge.mathz_abs_normie(args[0]);
            }
            
            pub fn abs_meal(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_abs_meal(args[0]);
            }
            
            pub fn sqrt_meal(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_sqrt_meal(args[0]);
            }
            
            pub fn pow_meal(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_pow_meal(args[0], args[1]);
            }
            
            pub fn sin_meal(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_sin_meal(args[0]);
            }
            
            pub fn cos_meal(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_cos_meal(args[0]);
            }
            
            pub fn ln_meal(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_ln_meal(args[0]);
            }
            
            pub fn pow_meal_impl(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_pow_meal_impl(args[0], args[1]);
            }
            
            pub fn log_meal_impl(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_log_meal_impl(args[0]);
            }
            
            pub fn sqrt_meal_impl(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Float, .float = 0.0 };
                return try bridge.mathz_sqrt_meal_impl(args[0]);
            }
            
            pub fn random_meal(bridge: *StdlibBridge, args: []const Variable) !Variable {
                _ = args;
                return try bridge.mathz_random_meal();
            }
            
            pub fn random_int_range(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return Variable{ .type = .Integer, .integer = 0 };
                return try bridge.mathz_random_int_range(args[0], args[1]);
            }
        };
        
        pub const arrayz = struct {
            pub fn len(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Integer, .integer = 0 };
                return try bridge.arrayz_len(args[0]);
            }
            
            pub fn append(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return args[0];
                return try bridge.arrayz_append(args[0], args[1]);
            }
        };
        
        // ===== NEW MISSING FUNCTION REGISTRY =====
        
        pub const filez = struct {
            pub fn exists(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Boolean, .boolean = false };
                const path_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(path_str);
                const result = bridge.core.file_exists(path_str);
                return Variable{ .type = .Boolean, .boolean = result };
            }
            
            pub fn is_directory(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .Boolean, .boolean = false };
                const path_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(path_str);
                const result = missing_impl.is_directory_impl(path_str);
                return Variable{ .type = .Boolean, .boolean = result };
            }
            
            pub fn copy_file(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return Variable{ .type = .Boolean, .boolean = false };
                const src_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(src_str);
                const dest_str = try bridge.variableToString(args[1]);
                defer if (args[1].type != .String) bridge.allocator.free(dest_str);
                const result = missing_impl.copy_file_impl(src_str, dest_str);
                return Variable{ .type = .Boolean, .boolean = result };
            }
        };
        
        pub const envz = struct {
            pub fn getenv(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "") };
                const name_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(name_str);
                const result = missing_impl.getenv_impl(bridge.allocator, name_str) catch null;
                if (result) |val| {
                    return Variable{ .type = .String, .string = val };
                } else {
                    return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "") };
                }
            }
            
            pub fn setenv(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len < 2) return Variable{ .type = .Boolean, .boolean = false };
                const name_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(name_str);
                const value_str = try bridge.variableToString(args[1]);
                defer if (args[1].type != .String) bridge.allocator.free(value_str);
                const result = missing_impl.setenv_impl(name_str, value_str);
                return Variable{ .type = .Boolean, .boolean = result };
            }
            
            pub fn getcwd(bridge: *StdlibBridge, args: []const Variable) !Variable {
                _ = args;
                const result = missing_impl.getcwd_impl(bridge.allocator) catch return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "") };
                return Variable{ .type = .String, .string = result };
            }
        };
        
        pub const jsonz = struct {
            pub fn parse(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "{}") };
                const json_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(json_str);
                const result = try missing_impl.json_parse_impl(bridge.allocator, json_str);
                return Variable{ .type = .String, .string = result };
            }
            
            pub fn stringify(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "\"\"") };
                const data_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(data_str);
                const result = try missing_impl.json_stringify_impl(bridge.allocator, data_str);
                return Variable{ .type = .String, .string = result };
            }
        };
        
        pub const cryptz = struct {
            pub fn hash_string(bridge: *StdlibBridge, args: []const Variable) !Variable {
                if (args.len == 0) return Variable{ .type = .String, .string = try bridge.allocator.dupe(u8, "") };
                const input_str = try bridge.variableToString(args[0]);
                defer if (args[0].type != .String) bridge.allocator.free(input_str);
                const result = try missing_impl.hash_string_impl(bridge.allocator, input_str);
                return Variable{ .type = .String, .string = result };
            }
        };
    };
    
    /// Resolve and call a stdlib function
    pub fn callStdlibFunction(self: *StdlibBridge, module_name: []const u8, function_name: []const u8, args: []const Variable) !Variable {
        // Vibez module functions
        if (std.mem.eql(u8, module_name, "vibez")) {
            if (std.mem.eql(u8, function_name, "spill")) {
                return try function_registry.vibez.spill(self, args);
            } else if (std.mem.eql(u8, function_name, "spillf")) {
                return try function_registry.vibez.spillf(self, args);
            } else if (std.mem.eql(u8, function_name, "scanln")) {
                return try function_registry.vibez.scanln(self, args);
            }
        }
        
        // Stringz module functions
        if (std.mem.eql(u8, module_name, "stringz")) {
            if (std.mem.eql(u8, function_name, "length")) {
                return try function_registry.stringz.length(self, args);
            } else if (std.mem.eql(u8, function_name, "char_at")) {
                return try function_registry.stringz.char_at(self, args);
            } else if (std.mem.eql(u8, function_name, "concat")) {
                return try function_registry.stringz.concat(self, args);
            } else if (std.mem.eql(u8, function_name, "substring")) {
                return try function_registry.stringz.substring(self, args);
            }
        }
        
        // Mathz module functions  
        if (std.mem.eql(u8, module_name, "mathz")) {
            if (std.mem.eql(u8, function_name, "abs_normie")) {
                return try function_registry.mathz.abs_normie(self, args);
            } else if (std.mem.eql(u8, function_name, "abs_meal")) {
                return try function_registry.mathz.abs_meal(self, args);
            } else if (std.mem.eql(u8, function_name, "sqrt_meal")) {
                return try function_registry.mathz.sqrt_meal(self, args);
            } else if (std.mem.eql(u8, function_name, "pow_meal")) {
                return try function_registry.mathz.pow_meal(self, args);
            } else if (std.mem.eql(u8, function_name, "sin_meal")) {
                return try function_registry.mathz.sin_meal(self, args);
            } else if (std.mem.eql(u8, function_name, "cos_meal")) {
                return try function_registry.mathz.cos_meal(self, args);
            } else if (std.mem.eql(u8, function_name, "ln_meal")) {
                return try function_registry.mathz.ln_meal(self, args);
            } else if (std.mem.eql(u8, function_name, "pow_meal_impl")) {
                return try function_registry.mathz.pow_meal_impl(self, args);
            } else if (std.mem.eql(u8, function_name, "log_meal_impl")) {
                return try function_registry.mathz.log_meal_impl(self, args);
            } else if (std.mem.eql(u8, function_name, "sqrt_meal_impl")) {
                return try function_registry.mathz.sqrt_meal_impl(self, args);
            } else if (std.mem.eql(u8, function_name, "random_meal")) {
                return try function_registry.mathz.random_meal(self, args);
            } else if (std.mem.eql(u8, function_name, "random_int_range")) {
                return try function_registry.mathz.random_int_range(self, args);
            }
        }
        
        // Arrayz module functions
        if (std.mem.eql(u8, module_name, "arrayz")) {
            if (std.mem.eql(u8, function_name, "len")) {
                return try function_registry.arrayz.len(self, args);
            } else if (std.mem.eql(u8, function_name, "append")) {
                return try function_registry.arrayz.append(allocator, self, args);
            }
        }
        
        // Filez module functions
        if (std.mem.eql(u8, module_name, "filez")) {
            if (std.mem.eql(u8, function_name, "exists")) {
                return try function_registry.filez.exists(self, args);
            } else if (std.mem.eql(u8, function_name, "is_directory")) {
                return try function_registry.filez.is_directory(self, args);
            } else if (std.mem.eql(u8, function_name, "copy_file")) {
                return try function_registry.filez.copy_file(self, args);
            }
        }
        
        // Envz module functions
        if (std.mem.eql(u8, module_name, "envz")) {
            if (std.mem.eql(u8, function_name, "getenv")) {
                return try function_registry.envz.getenv(self, args);
            } else if (std.mem.eql(u8, function_name, "setenv")) {
                return try function_registry.envz.setenv(self, args);
            } else if (std.mem.eql(u8, function_name, "getcwd")) {
                return try function_registry.envz.getcwd(self, args);
            }
        }
        
        // Jsonz module functions
        if (std.mem.eql(u8, module_name, "jsonz")) {
            if (std.mem.eql(u8, function_name, "parse")) {
                return try function_registry.jsonz.parse(self, args);
            } else if (std.mem.eql(u8, function_name, "stringify")) {
                return try function_registry.jsonz.stringify(self, args);
            }
        }
        
        // Cryptz module functions
        if (std.mem.eql(u8, module_name, "cryptz")) {
            if (std.mem.eql(u8, function_name, "hash_string")) {
                return try function_registry.cryptz.hash_string(self, args);
            }
        }
        
        // Default fallback
        print("⚠️ Stdlib function not found: {s}.{s}\n", .{module_name, function_name});
        return Variable{ .type = .Null };
    }
};

/// Test the stdlib bridge functionality
pub fn test_stdlib_bridge() !void {
    print("\n🧪 Testing CURSED Stdlib Bridge\n");
    print("===============================\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    // Initialize stdlib core and bridge
    stdlib_core.init_stdlib_core(allocator);
    var bridge = StdlibBridge.init(allocator);
    
    // Test vibez.spill()
    print("Testing vibez.spill()...\n");
    const hello_var = Variable{ .type = .String, .string = "Hello from CURSED!" };
    const spill_result = try bridge.callStdlibFunction("vibez", "spill", &[_]Variable{hello_var});
    print("spill() result: {any}\n", .{spill_result.boolean});
    
    // Test stringz.length()
    print("Testing stringz.length()...\n");
    const test_string = Variable{ .type = .String, .string = "Hello" };
    const length_result = try bridge.callStdlibFunction("stringz", "length", &[_]Variable{test_string});
    print("length() result: {}\n", .{length_result.integer});
    
    // Test mathz.abs_normie()
    print("Testing mathz.abs_normie()...\n");
    const negative_int = Variable{ .type = .Integer, .integer = -42 };
    const abs_result = try bridge.callStdlibFunction("mathz", "abs_normie", &[_]Variable{negative_int});
    print("abs_normie(-42) result: {}\n", .{abs_result.integer});
    
    // Test mathz.sqrt_meal()
    print("Testing mathz.sqrt_meal()...\n");
    const sixteen = Variable{ .type = .Float, .float = 16.0 };
    const sqrt_result = try bridge.callStdlibFunction("mathz", "sqrt_meal", &[_]Variable{sixteen});
    print("sqrt_meal(16.0) result: {d}\n", .{sqrt_result.float});
    
    print("\n✅ Stdlib Bridge tests completed successfully\n");
}
