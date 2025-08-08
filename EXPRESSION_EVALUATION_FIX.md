# CURSED Zig Compiler: Expression Evaluation Fix

## Problem
Variable expressions like `sus result drip = x + y` fail to evaluate, causing:
- Variable `result` to not be stored  
- `vibez.spill(result)` to print literal "result" instead of computed value

## Root Cause
The `evaluateExpression` function in `src-zig/main_unified.zig` has a logic flow issue where binary operators (`+`, `-`) are not being properly handled before the function falls back to literal evaluation.

## Solution

### 1. Fix Addition Operator Logic
In `evaluateExpression` function (around line 1610), ensure the addition logic is actually reached:

```zig
// + and - (lowest precedence, evaluated last) 
// The current logic is correct but may not be reached due to early returns
const low_ops = [_][]const u8{ "+", "-" };
for (low_ops) |op| {
    // Find the operator position
    if (std.mem.indexOf(u8, trimmed, op)) |pos| {
        // Skip unary operators at beginning
        if (pos == 0 and std.mem.eql(u8, op, "-")) continue;
        
        const left_str = std.mem.trim(u8, trimmed[0..pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[pos + op.len..], " \t");
        
        if (left_str.len == 0 or right_str.len == 0) continue;
        
        // Debug output
        if (verbose) print("🔍 Found operator '{s}': left='{s}', right='{s}'\n", .{ op, left_str, right_str });
        
        // Recursively evaluate operands
        const left = try evaluateExpression(variables, functions, allocator, left_str, verbose);
        const right = try evaluateExpression(variables, functions, allocator, right_str, verbose);
        
        const result = try performBinaryOperation(left, right, op, allocator, verbose);
        return result;
    }
}
```

### 2. Fix Error Handling in Variable Declaration
In `handleVariableDeclaration` function (around line 2070), ensure proper error propagation:

```zig
if (evaluateExpression(variables, functions, allocator, value_str, verbose)) |result| {
    switch (result) {
        .Integer => |int_val| break :blk Variable{ .Integer = int_val },
        // ... other cases
    }
} else |err| {
    // Better error handling - don't silently fail
    if (verbose) print("❌ Failed to evaluate expression '{s}': {}\n", .{value_str, err});
    return; // Don't store the variable if evaluation fails
}
```

### 3. Test Commands

After applying the fix:

```bash
zig build
echo 'sus x drip = 10
sus y drip = 20  
sus result drip = x + y
vibez.spill("result:", result)' > test_fix.csd
./zig-out/bin/cursed test_fix.csd
```

Expected output:
```
result: 30
```

### 4. Memory Safety Check
```bash
valgrind ./zig-out/bin/cursed test_fix.csd
```

Should show zero memory leaks.

## Files to Modify
- `src-zig/main_unified.zig` (lines ~1610 and ~2070)

## Root Issue Analysis
The problem was that the operator precedence logic had correct implementation but was not being reached due to early returns or error conditions in the expression evaluation pipeline.
