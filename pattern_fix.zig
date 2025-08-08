const std = @import("std");

// Add these functions to main_unified.zig

fn executePatternMatchingStatement(variables: *VariableStore, functions: *FunctionStore, allocator: Allocator, line: []const u8, verbose: bool) !void {
    // Parse: ready (value) { 1 => action; _ => default }
    const trimmed = std.mem.trim(u8, line, " \t");
    
    if (verbose) std.debug.print("  🎯 Pattern matching: {s}\n", .{trimmed});
    
    // Find the value expression: ready (value) { ... }
    if (std.mem.indexOf(u8, trimmed, "(")) |start_paren| {
        if (std.mem.indexOf(u8, trimmed[start_paren..], ")")) |rel_end_paren| {
            const end_paren = start_paren + rel_end_paren;
            if (std.mem.indexOf(u8, trimmed[end_paren..], "{")) |rel_brace_start| {
                if (std.mem.lastIndexOf(u8, trimmed, "}")) |brace_end| {
                    const value_str = std.mem.trim(u8, trimmed[start_paren + 1..end_paren], " \t");
                    const body_str = std.mem.trim(u8, trimmed[end_paren + rel_brace_start + 1..brace_end], " \t");
                    
                    if (verbose) std.debug.print("    📊 Value: '{s}', Body: '{s}'\n", .{ value_str, body_str });
                    
                    // Evaluate the value to match against
                    const match_value = try evaluateExpression(variables, functions, allocator, value_str, verbose);
                    defer { var val = match_value; val.deinit(allocator); }
                    
                    // Split the body by newlines first, then by semicolons for each line
                    var line_iterator = std.mem.split(u8, body_str, "\n");
                    var matched = false;
                    
                    while (line_iterator.next()) |line_part| {
                        const line_trimmed = std.mem.trim(u8, line_part, " \t");
                        if (line_trimmed.len == 0) continue;
                        
                        // Handle multiple cases on the same line separated by semicolons
                        var case_iterator = std.mem.split(u8, line_trimmed, ";");
                        while (case_iterator.next()) |case_part| {
                            const case_trimmed = std.mem.trim(u8, case_part, " \t");
                            if (case_trimmed.len == 0) continue;
                            
                            if (std.mem.indexOf(u8, case_trimmed, "=>")) |arrow_pos| {
                                const pattern_str = std.mem.trim(u8, case_trimmed[0..arrow_pos], " \t");
                                const action_str = std.mem.trim(u8, case_trimmed[arrow_pos + 2..], " \t");
                                
                                if (verbose) std.debug.print("    🎪 Pattern: '{s}' => Action: '{s}'\n", .{ pattern_str, action_str });
                                
                                // Check if pattern matches
                                var pattern_matches = false;
                                
                                if (std.mem.eql(u8, pattern_str, "_")) {
                                    // Wildcard pattern - always matches if no previous pattern matched
                                    pattern_matches = !matched;
                                } else {
                                    // Try to match literal pattern
                                    if (try matchLiteralPattern(match_value, pattern_str)) {
                                        pattern_matches = true;
                                    }
                                }
                                
                                if (pattern_matches and !matched) {
                                    if (verbose) std.debug.print("    ✅ Pattern matched, executing action\n", .{});
                                    try executeFunctionBodyLine(variables, functions, allocator, action_str, verbose);
                                    matched = true;
                                    return; // Exit immediately after first match
                                }
                            }
                        }
                    }
                    
                    if (!matched) {
                        if (verbose) std.debug.print("    ❌ No pattern matched\n", .{});
                    }
                    
                    return;
                }
            }
        }
    }
    
    if (verbose) std.debug.print("❌ Failed to parse pattern matching statement: {s}\n", .{trimmed});
}

fn matchLiteralPattern(value: Variable, pattern_str: []const u8) !bool {
    // Try to match the value against the pattern string
    switch (value) {
        .Integer => |val| {
            if (std.fmt.parseInt(i64, pattern_str, 10)) |pattern_int| {
                return val == pattern_int;
            } else |_| {
                return false;
            }
        },
        .Float => |val| {
            if (std.fmt.parseFloat(f64, pattern_str)) |pattern_float| {
                return val == pattern_float;
            } else |_| {
                return false;
            }
        },
        .String => |val| {
            // Check if pattern is a quoted string
            if (pattern_str.len >= 2 and pattern_str[0] == '"' and pattern_str[pattern_str.len - 1] == '"') {
                const unquoted = pattern_str[1..pattern_str.len - 1];
                return std.mem.eql(u8, val, unquoted);
            } else {
                return std.mem.eql(u8, val, pattern_str);
            }
        },
        .Boolean => |val| {
            if (std.mem.eql(u8, pattern_str, "based")) {
                return val == true;
            } else if (std.mem.eql(u8, pattern_str, "cringe")) {
                return val == false;
            } else if (std.mem.eql(u8, pattern_str, "true")) {
                return val == true;
            } else if (std.mem.eql(u8, pattern_str, "false")) {
                return val == false;
            } else {
                return false;
            }
        },
        else => return false,
    }
}
