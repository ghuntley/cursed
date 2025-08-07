// Fixed function execution implementation for main_unified.zig
// This file contains the corrected implementations to fix function declaration parsing,
// function call evaluation, and return value handling

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

// Import the existing types from main_unified.zig
// These would need to be compatible with the existing Variable and FunctionDefinition types

// FIXED: Function declaration handler that properly parses single-line functions
fn handleFunctionDeclaration_FIXED(functions: *anytype, allocator: Allocator, source_lines: ArrayList([]const u8), start_line: usize, verbose: bool) !usize {
    if (start_line >= source_lines.items.len) return 1;
    
    // Get the current line (function signature)
    const line = source_lines.items[start_line];
    const trimmed = std.mem.trim(u8, line, " \t\r\n");
    
    // Parse function signature: slay funcname(param1 type1, param2 type2) { body } 
    if (!std.mem.startsWith(u8, trimmed, "slay ")) return 1;
    
    // Check if this line has additional code after the function (single-line case)
    var function_part = trimmed;
    var remaining_code: ?[]const u8 = null;
    
    // Look for the end of the function body in single-line format
    if (std.mem.indexOf(u8, trimmed, "{")) |open_brace| {
        var brace_count: i32 = 0;
        var end_brace: ?usize = null;
        
        for (trimmed[open_brace..], open_brace..) |char, i| {
            if (char == '{') {
                brace_count += 1;
            } else if (char == '}') {
                brace_count -= 1;
                if (brace_count == 0) {
                    end_brace = i;
                    break;
                }
            }
        }
        
        if (end_brace) |end_pos| {
            function_part = trimmed[0..end_pos + 1];
            // Check if there's more code after the function
            const after_func = std.mem.trim(u8, trimmed[end_pos + 1..], " \t");
            if (after_func.len > 0) {
                remaining_code = after_func;
            }
        }
    }
    
    // Extract function name
    const after_slay = std.mem.trim(u8, function_part[5..], " \t");
    const paren_pos = std.mem.indexOf(u8, after_slay, "(") orelse return 1;
    const func_name = std.mem.trim(u8, after_slay[0..paren_pos], " \t");
    
    if (verbose) print("🔍 Parsing function: {s}\n", .{func_name});
    
    // Create function definition - using the existing FunctionDefinition structure
    const FunctionDefinition = @TypeOf(functions.*).ValueType;
    const FunctionParameter = @TypeOf(functions.*).ValueType.parameters.Child;
    
    const func_name_copy = try allocator.dupe(u8, func_name);
    var func_def = FunctionDefinition.init(allocator, func_name_copy);
    
    // Parse parameters
    const params_start = paren_pos + 1;
    if (std.mem.indexOf(u8, after_slay[params_start..], ")")) |params_end_offset| {
        const params_end = params_start + params_end_offset;
        const params_str = std.mem.trim(u8, after_slay[params_start..params_end], " \t");
        
        if (params_str.len > 0) {
            var param_iter = std.mem.split(u8, params_str, ",");
            while (param_iter.next()) |param_str| {
                const trimmed_param = std.mem.trim(u8, param_str, " \t");
                if (trimmed_param.len == 0) continue;
                
                // Parse param: "name type"
                var param_parts = std.mem.split(u8, trimmed_param, " ");
                const param_name = param_parts.next() orelse continue;
                const param_type = param_parts.next() orelse "tea"; // default type
                
                const parameter = FunctionParameter{
                    .name = try allocator.dupe(u8, param_name),
                    .param_type = try allocator.dupe(u8, param_type),
                };
                
                try func_def.parameters.append(parameter);
                if (verbose) print("  📝 Parameter: {s} {s}\n", .{ param_name, param_type });
            }
        }
        
        // FIXED: Parse function body - check for single-line format first
        const remaining_part = after_slay[params_end + 1..];
        if (std.mem.indexOf(u8, remaining_part, "{")) |open_brace| {
            if (std.mem.lastIndexOf(u8, remaining_part, "}")) |close_brace| {
                if (close_brace > open_brace) {
                    // Single-line function body
                    const body_content = std.mem.trim(u8, remaining_part[open_brace + 1..close_brace], " \t");
                    if (body_content.len > 0) {
                        try func_def.body.append(try allocator.dupe(u8, body_content));
                        if (verbose) print("  📝 Body line: {s}\n", .{body_content});
                    }
                    
                    // Store function in function store
                    const func_store_key = try allocator.dupe(u8, func_name);
                    try functions.put(func_store_key, func_def);
                    if (verbose) print("✅ Function {s} stored with {} parameters and {} body lines\n", .{ func_name, func_def.parameters.items.len, func_def.body.items.len });
                    
                    // FIXED: If there's remaining code after the function, we need to process it
                    if (remaining_code) |code| {
                        // Insert remaining code as a new line for processing
                        try source_lines.insert(start_line + 1, code);
                        return 1; // Only consumed 1 line (function definition)
                    }
                    
                    return 1;
                }
            }
        }
    }
    
    // Multi-line function body parsing (fallback for complex functions)
    var current_line = start_line + 1;
    var lines_consumed: usize = 1;
    
    while (current_line < source_lines.items.len) {
        const body_line = source_lines.items[current_line];
        const body_trimmed = std.mem.trim(u8, body_line, " \t\r\n");
        
        lines_consumed += 1;
        
        if (std.mem.eql(u8, body_trimmed, "}")) {
            break;
        }
        
        if (body_trimmed.len > 0 and !std.mem.eql(u8, body_trimmed, "{")) {
            try func_def.body.append(try allocator.dupe(u8, body_trimmed));
            if (verbose) print("  📝 Body line: {s}\n", .{body_trimmed});
        }
        
        current_line += 1;
    }
    
    // Store function in function store
    const func_store_key = try allocator.dupe(u8, func_name);
    try functions.put(func_store_key, func_def);
    if (verbose) print("✅ Function {s} stored with {} parameters and {} body lines\n", .{ func_name, func_def.parameters.items.len, func_def.body.items.len });
    
    return lines_consumed;
}

// FIXED: Function execution that properly handles return values and arithmetic
fn executeFunctionBodyLine_FIXED(variables: *anytype, functions: *anytype, allocator: Allocator, line: []const u8, verbose: bool) !void {
    const trimmed = std.mem.trim(u8, line, " \t");
    
    // FIXED: Handle return statements: damn <expression>
    if (std.mem.startsWith(u8, trimmed, "damn ")) {
        const return_expr = std.mem.trim(u8, trimmed[5..], " \t");
        if (verbose) print("  ↩️ Processing return statement: {s}\n", .{return_expr});
        
        // FIXED: Evaluate return expression with proper arithmetic
        const return_value = try evaluateExpressionFixed(variables, functions, allocator, return_expr, verbose);
        
        // Store return value in special variable
        try variables.put("__return_value__", return_value);
        
        if (verbose) print("  ↩️ Return value: {any}\n", .{return_value});
        return error.FunctionReturn;
    }
    
    // Handle variable declarations in function body: sus varname type = value
    if (std.mem.startsWith(u8, trimmed, "sus ")) {
        if (verbose) print("  🔍 Processing local variable declaration: {s}\n", .{trimmed});
        try handleVariableDeclarationFixed(variables, functions, allocator, trimmed, verbose);
        return;
    }
    
    // Handle vibez.spill() calls in function body
    if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
        try handleVibesSpillFixed(variables, functions, allocator, trimmed, start, verbose);
    } else if (verbose) {
        print("  📝 Function body line: {s}\n", .{trimmed});
    }
}

// FIXED: Expression evaluation with proper arithmetic and function call handling
fn evaluateExpressionFixed(variables: *anytype, functions: *anytype, allocator: Allocator, expr_str: []const u8, verbose: bool) !@TypeOf(variables.*).ValueType {
    const Variable = @TypeOf(variables.*).ValueType;
    const trimmed = std.mem.trim(u8, expr_str, " \t");
    
    if (verbose) print("🧮 EXPR_EVAL: Evaluating expression: '{s}'\n", .{trimmed});
    
    // FIXED: Check for function calls first (before handling general parentheses)
    if (std.mem.indexOf(u8, trimmed, "(")) |paren_pos| {
        // Check if this looks like a function call (identifier followed by parentheses)
        const potential_func_name = std.mem.trim(u8, trimmed[0..paren_pos], " \t");
        
        // Simple check: if the part before parentheses is a single identifier (no spaces/operators)
        // and no arithmetic operators before the parentheses, it's likely a function call
        var is_likely_function_call = true;
        for (potential_func_name) |char| {
            if (!std.ascii.isAlphanumeric(char) and char != '_') {
                is_likely_function_call = false;
                break;
            }
        }
        
        if (is_likely_function_call and potential_func_name.len > 0) {
            // Try to evaluate as function call
            if (handleFunctionCallFixed(functions, variables, allocator, trimmed, verbose)) |return_value| {
                if (return_value) |ret_val| {
                    if (verbose) print("📊 Function call returned: {any}\n", .{ret_val});
                    return ret_val;
                } else {
                    if (verbose) print("📊 Function call returned void\n", .{});
                    return error.FunctionReturnedVoid;
                }
            } else |_| {
                // Not a valid function call, continue with normal evaluation
            }
        }
    }
    
    // FIXED: Handle arithmetic operations with proper precedence
    // Check for addition/subtraction first (lowest precedence)
    if (std.mem.lastIndexOf(u8, trimmed, "+")) |plus_pos| {
        // Make sure the + is not inside parentheses
        if (!isInsideParentheses(trimmed, plus_pos)) {
            const left_str = std.mem.trim(u8, trimmed[0..plus_pos], " \t");
            const right_str = std.mem.trim(u8, trimmed[plus_pos + 1..], " \t");
            
            const left_val = try evaluateExpressionFixed(variables, functions, allocator, left_str, verbose);
            const right_val = try evaluateExpressionFixed(variables, functions, allocator, right_str, verbose);
            
            // FIXED: Handle integer and string addition
            switch (left_val) {
                .Integer => |left_int| switch (right_val) {
                    .Integer => |right_int| return Variable{ .Integer = left_int + right_int },
                    else => {},
                },
                .String => |left_str| switch (right_val) {
                    .String => |right_str| {
                        const result = try std.fmt.allocPrint(allocator, "{s}{s}", .{ left_str, right_str });
                        return Variable{ .String = result };
                    },
                    else => {},
                },
                else => {},
            }
        }
    }
    
    if (std.mem.lastIndexOf(u8, trimmed, "-")) |minus_pos| {
        if (!isInsideParentheses(trimmed, minus_pos) and minus_pos > 0) { // Don't treat leading minus as subtraction
            const left_str = std.mem.trim(u8, trimmed[0..minus_pos], " \t");
            const right_str = std.mem.trim(u8, trimmed[minus_pos + 1..], " \t");
            
            const left_val = try evaluateExpressionFixed(variables, functions, allocator, left_str, verbose);
            const right_val = try evaluateExpressionFixed(variables, functions, allocator, right_str, verbose);
            
            switch (left_val) {
                .Integer => |left_int| switch (right_val) {
                    .Integer => |right_int| return Variable{ .Integer = left_int - right_int },
                    else => {},
                },
                else => {},
            }
        }
    }
    
    // Handle multiplication/division (higher precedence)
    if (std.mem.lastIndexOf(u8, trimmed, "*")) |mult_pos| {
        if (!isInsideParentheses(trimmed, mult_pos)) {
            const left_str = std.mem.trim(u8, trimmed[0..mult_pos], " \t");
            const right_str = std.mem.trim(u8, trimmed[mult_pos + 1..], " \t");
            
            const left_val = try evaluateExpressionFixed(variables, functions, allocator, left_str, verbose);
            const right_val = try evaluateExpressionFixed(variables, functions, allocator, right_str, verbose);
            
            switch (left_val) {
                .Integer => |left_int| switch (right_val) {
                    .Integer => |right_int| return Variable{ .Integer = left_int * right_int },
                    else => {},
                },
                else => {},
            }
        }
    }
    
    if (std.mem.lastIndexOf(u8, trimmed, "/")) |div_pos| {
        if (!isInsideParentheses(trimmed, div_pos)) {
            const left_str = std.mem.trim(u8, trimmed[0..div_pos], " \t");
            const right_str = std.mem.trim(u8, trimmed[div_pos + 1..], " \t");
            
            const left_val = try evaluateExpressionFixed(variables, functions, allocator, left_str, verbose);
            const right_val = try evaluateExpressionFixed(variables, functions, allocator, right_str, verbose);
            
            switch (left_val) {
                .Integer => |left_int| switch (right_val) {
                    .Integer => |right_int| {
                        if (right_int == 0) return error.DivisionByZero;
                        return Variable{ .Integer = @divTrunc(left_int, right_int) };
                    },
                    else => {},
                },
                else => {},
            }
        }
    }
    
    // Handle parentheses
    if (std.mem.indexOf(u8, trimmed, "(")) |start_paren| {
        var paren_count: i32 = 0;
        var end_paren: ?usize = null;
        
        for (trimmed[start_paren..], start_paren..) |char, i| {
            if (char == '(') {
                paren_count += 1;
            } else if (char == ')') {
                paren_count -= 1;
                if (paren_count == 0) {
                    end_paren = i;
                    break;
                }
            }
        }
        
        if (end_paren) |end_pos| {
            if (start_paren < end_pos) {
                const inner_expr = trimmed[start_paren + 1..end_pos];
                const inner_result = try evaluateExpressionFixed(variables, functions, allocator, inner_expr, verbose);
                
                // Replace the parentheses expression with its result
                const before = trimmed[0..start_paren];
                const after = trimmed[end_pos + 1..];
                
                if (before.len == 0 and after.len == 0) {
                    // Just parentheses around the whole expression
                    return inner_result;
                } else {
                    // Replace and re-evaluate
                    const result_str = try inner_result.toString(allocator);
                    defer allocator.free(result_str);
                    const new_expr = try std.fmt.allocPrint(allocator, "{s}{s}{s}", .{ before, result_str, after });
                    defer allocator.free(new_expr);
                    return evaluateExpressionFixed(variables, functions, allocator, new_expr, verbose);
                }
            }
        }
    }
    
    // Handle variable references
    if (variables.get(trimmed)) |var_value| {
        if (verbose) print("📊 Variable reference: {s} = {any}\n", .{ trimmed, var_value });
        return var_value;
    }
    
    // Handle literal values
    if (std.fmt.parseInt(i64, trimmed, 10)) |int_val| {
        return Variable{ .Integer = int_val };
    } else |_| {}
    
    if (std.fmt.parseFloat(f64, trimmed)) |float_val| {
        return Variable{ .Float = float_val };
    } else |_| {}
    
    // Handle string literals
    if (trimmed.len >= 2 and trimmed[0] == '"' and trimmed[trimmed.len - 1] == '"') {
        const string_content = trimmed[1..trimmed.len - 1];
        return Variable{ .String = try allocator.dupe(u8, string_content) };
    }
    
    // Handle boolean literals
    if (std.mem.eql(u8, trimmed, "based")) {
        return Variable{ .Boolean = true };
    } else if (std.mem.eql(u8, trimmed, "cringe")) {
        return Variable{ .Boolean = false };
    }
    
    if (verbose) print("❌ Unable to evaluate expression: '{s}'\n", .{trimmed});
    return error.UnknownExpression;
}

// Helper function to check if position is inside parentheses
fn isInsideParentheses(text: []const u8, pos: usize) bool {
    var paren_count: i32 = 0;
    for (text[0..pos]) |char| {
        if (char == '(') {
            paren_count += 1;
        } else if (char == ')') {
            paren_count -= 1;
        }
    }
    return paren_count > 0;
}

// FIXED: Function call handler with proper return value processing
fn handleFunctionCallFixed(functions: *anytype, variables: *anytype, allocator: Allocator, call_line: []const u8, verbose: bool) !?@TypeOf(variables.*).ValueType {
    const Variable = @TypeOf(variables.*).ValueType;
    
    // Parse function call: funcname(arg1, arg2, ...)
    const paren_pos = std.mem.indexOf(u8, call_line, "(") orelse return null;
    const func_name = std.mem.trim(u8, call_line[0..paren_pos], " \t");
    
    const func_def = functions.get(func_name) orelse return null;
    
    if (verbose) print("🚀 Executing function: {s}\n", .{func_name});
    
    // Parse arguments
    if (std.mem.lastIndexOf(u8, call_line, ")")) |end_paren| {
        const args_str = std.mem.trim(u8, call_line[paren_pos + 1..end_paren], " \t");
        
        // Create local variable scope for function execution
        var arena = std.heap.ArenaAllocator.init(allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        const VariableStore = @TypeOf(variables.*);
        var local_variables = VariableStore.init(arena_allocator);
        
        // Copy global variables to local scope
        var global_iter = variables.iterator();
        while (global_iter.next()) |entry| {
            const value = switch (entry.value_ptr.*) {
                .String => |str| Variable{ .String = str }, // Reference, don't duplicate
                else => entry.value_ptr.*,
            };
            try local_variables.put(try arena_allocator.dupe(u8, entry.key_ptr.*), value);
        }
        
        // FIXED: Bind arguments to parameters with proper evaluation
        if (args_str.len > 0) {
            var arg_iter = std.mem.split(u8, args_str, ",");
            var param_index: usize = 0;
            
            while (arg_iter.next()) |arg_str| {
                if (param_index >= func_def.parameters.items.len) break;
                
                const trimmed_arg = std.mem.trim(u8, arg_str, " \t");
                const param = func_def.parameters.items[param_index];
                
                // FIXED: Evaluate argument properly using the fixed expression evaluator
                const arg_value = try evaluateArgumentFixed(&local_variables, functions, arena_allocator, trimmed_arg, verbose);
                const param_name = try arena_allocator.dupe(u8, param.name);
                try local_variables.put(param_name, arg_value);
                
                if (verbose) print("  📝 Bound {s} = {any}\n", .{ param.name, arg_value });
                param_index += 1;
            }
        }
        
        // FIXED: Execute function body with proper return value handling
        var return_value: ?Variable = null;
        for (func_def.body.items) |body_line| {
            if (verbose) print("  🔍 Executing: {s}\n", .{body_line});
            if (executeFunctionBodyLine_FIXED(&local_variables, functions, arena_allocator, body_line, verbose)) |_| {
                // Continue execution
            } else |err| switch (err) {
                error.FunctionReturn => {
                    // Extract return value if available
                    if (local_variables.get("__return_value__")) |ret_val| {
                        return_value = ret_val;
                        if (verbose) print("  ↩️ Function returned: {any}\n", .{ret_val});
                    }
                    break;
                },
                else => return err,
            }
        }
        
        return return_value;
    }
    
    return null;
}

// FIXED: Argument evaluation with proper expression handling
fn evaluateArgumentFixed(variables: *anytype, functions: *anytype, allocator: Allocator, arg_str: []const u8, verbose: bool) !@TypeOf(variables.*).ValueType {
    // Use the fixed expression evaluator
    return evaluateExpressionFixed(variables, functions, allocator, arg_str, verbose) catch |err| switch (err) {
        error.UnknownExpression => {
            // Fallback to literal parsing
            return evaluateSingleValueFixed(variables, functions, allocator, arg_str, verbose) catch blk: {
                const Variable = @TypeOf(variables.*).ValueType;
                break :blk Variable{ .String = arg_str };
            };
        },
        else => return err,
    };
}

// Placeholder functions that would need to be implemented or imported
fn handleVariableDeclarationFixed(variables: *anytype, functions: *anytype, allocator: Allocator, line: []const u8, verbose: bool) !void {
    _ = variables;
    _ = functions;
    _ = allocator;
    _ = line;
    _ = verbose;
    // This would be the existing handleVariableDeclaration function
}

fn handleVibesSpillFixed(variables: *anytype, functions: *anytype, allocator: Allocator, line: []const u8, start: usize, verbose: bool) !void {
    _ = variables;
    _ = functions;
    _ = allocator;
    _ = line;
    _ = start;
    _ = verbose;
    // This would be the existing handleVibesSpill function
}

fn evaluateSingleValueFixed(variables: *anytype, functions: *anytype, allocator: Allocator, arg_str: []const u8, verbose: bool) !@TypeOf(variables.*).ValueType {
    _ = variables;
    _ = functions;
    _ = allocator;
    _ = arg_str;
    _ = verbose;
    // This would be the existing evaluateSingleValue function
    return error.NotImplemented;
}
