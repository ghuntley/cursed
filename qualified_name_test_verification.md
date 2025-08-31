# LLVM Backend Qualified Name Fix - Verification

## Issue Description
The Oracle identified that the LLVM backend in `llvm_ir_pipeline.zig` had an issue where:
- `generateFunction()` stored functions without module prefixes (e.g., "add_two")
- But call expressions looked for qualified names (e.g., "mathz.add_two")
- This caused stdlib function calls to return 0 instead of the correct result

## Fix Applied
Modified `llvm_ir_pipeline.zig`:

### 1. Added Module Context Tracking
```zig
// Added to struct fields:
current_module_name: ?[]const u8,

// Added to initialization:
.current_module_name = null,
```

### 2. Modified generateFunction() to Use Qualified Names
```zig
// Before:
try self.functions.put(func_decl.name, function);

// After:
const qualified_name = if (self.current_module_name) |module_name|
    try std.fmt.allocPrint(self.arena.allocator(), "{s}.{s}", .{ module_name, func_decl.name })
else
    func_decl.name;
try self.functions.put(qualified_name, function);
```

### 3. Updated loadAndCompileModule() to Set Module Context
```zig
// Set module context before function generation
self.current_module_name = try self.arena.allocator().dupe(u8, module_name);

// Use regular generateFunction instead of generateFunctionWithQualifiedName
try self.generateFunction(func_decl);

// Restore context after processing
self.current_module_name = previous_module_name;
```

## Expected Behavior After Fix

### Test Case: mathz.add_two(3, 5)
- **Before Fix**: Returns 0 (function lookup fails)
- **After Fix**: Returns 8 (correct arithmetic result)

### What happens in the fix:
1. When loading `mathz` module, `current_module_name` is set to "mathz"
2. Function `add_two` is stored as "mathz.add_two" in the functions HashMap
3. When `mathz.add_two(3, 5)` is called, it looks up "mathz.add_two" and finds the correct function
4. The function executes and returns the correct result: 3 + 5 = 8

## Verification
To verify this fix works:
1. Create a test program that calls `mathz.add_two(3, 5)`
2. Compile it with LLVM backend (`-l` flag)
3. Run the compiled binary
4. Should output `8` instead of `0`

## Files Modified
- `/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig` - Core fix implementation
