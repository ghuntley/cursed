# CURSED Interpreter vs Compiler Parity Test Report

**Generated:** Sun Aug 31 09:59:15 AM EEST 2025
**Test Suite Version:** 1.0.0
**CURSED Compiler:** /home/ghuntley/cursed/test_suite/../zig-out/bin/cursed-compiler

## Executive Summary

- **Total Tests:** 26
- **Passed:** 0
- **Failed:** 3
- **Compile Errors:** 23
- **Runtime Errors:** 0
- **Success Rate:** 0%

## System Health Score

**Overall Health Score:** 0/100 🔴 CRITICAL

### Interpretation:
- **90-100:** CURSED self-hosting is production-ready
- **80-89:** Minor issues, mostly functional
- **60-79:** Moderate issues, needs work
- **40-59:** Significant problems, major work needed
- **0-39:** Critical issues, substantial development required

## Test Categories Analysis

### Test Category Results:

- **arithmetic:** 4 tests
- **basic:** 3 tests
- **complex:** 2 tests
- **control_flow:** 2 tests
- **edge_cases:** 2 tests
- **errors:** 2 tests
- **functions:** 4 tests
- **performance:** 2 tests
- **stdlib:** 4 tests
- **strings:** 1 tests

## Recommendations

- 🚨 **High Priority:** Fix compilation failures (23 tests)
- 📋 **Low Priority:** Investigate output differences (3 tests)

## Detailed Test Results

## Test: 01_mixed_types
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:6:53 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:31:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc985c6cf8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7a33ec860000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a33ec880200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3533:59: 0x1216d38 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:862:53: 0x11d56de in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7a33ec8c0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_edge_cases
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffce41d33a8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7a60e95e0000 with parent@*interpreter.Environment@7ffce41d33a8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffce41d0ca8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Arithmetic Edge Cases Test === 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Zero operations: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
5 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
0 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
0 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Negative numbers: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
-2 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
-6 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
-6 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Large numbers: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
3000 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
999998 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Test Complete === 
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7a60e95e0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123936a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239890 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f2167 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c7323 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7a60f0ec0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a60f0ec0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0ec0700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7a60f0f00200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

error(gpa): memory address 0x7a60f0e80000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x142b3c0 in create__anon_71883 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3960:48: 0x140b683 in allocateMethodCall (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1729:81: 0x13fca2b in parseCall (cursed_compiler_main.zig)
                    expr = Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                                                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0f40800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

error(gpa): memory address 0x7a60f0e60000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3887:48: 0x1251341 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1730:62: 0x13fc8ab in parseCall (cursed_compiler_main.zig)
                        .object = try self.allocateExpression(expr),
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7a60f0e40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x7a60f0ea2000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 03_operator_precedence
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff8981f248 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7b2dacbe0000 with parent@*interpreter.Environment@7fff8981f248
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7fff8981cb48, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Operator Precedence Test === 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
2 + 3 * 4 should be 14: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
14 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
(2 + 3) * 4 should be 20: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
20 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
10 - 3 + 2 should be 9: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
9 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
8 / 2 * 3 should be 12: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
12 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Test Complete === 
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7b2dacbe0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123936a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239890 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f2167 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c7323 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7b2dacf20000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7b2dacf20100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf20600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7b2dacf60200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

error(gpa): memory address 0x7b2dacee0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x142b3c0 in create__anon_71883 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3960:48: 0x140b683 in allocateMethodCall (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1729:81: 0x13fca2b in parseCall (cursed_compiler_main.zig)
                    expr = Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                                                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacfa0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

error(gpa): memory address 0x7b2dacec0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3887:48: 0x1251341 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1512:53: 0x1396efe in parseTerm (cursed_compiler_main.zig)
                .right = try self.allocateExpression(right),
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1485:38: 0x135ee51 in parseComparison (cursed_compiler_main.zig)
        var expr = try self.parseTerm();
                                     ^

error(gpa): memory address 0x7b2dacea0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3887:48: 0x1251341 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1730:62: 0x13fc8ab in parseCall (cursed_compiler_main.zig)
                        .object = try self.allocateExpression(expr),
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7b2dacf02000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 04_complex_expressions
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:6:51 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:23:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 7
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcefb96f78 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7f15fd980000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7f15fd9c0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3533:59: 0x1216d38 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:862:53: 0x11d56de in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7f1605240800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_hello_world
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd4990fb08 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7d6b2f9c0000 with parent@*interpreter.Environment@7ffd4990fb08
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffd4990d408, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Hello, CURSED World! 
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7d6b2f9c0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123936a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239890 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f2167 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c7323 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7d6b372e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7d6b372e0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d6b372e0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7d6b372a0400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

error(gpa): memory address 0x7d6b37260000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x142b3c0 in create__anon_71883 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3960:48: 0x140b683 in allocateMethodCall (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1729:81: 0x13fca2b in parseCall (cursed_compiler_main.zig)
                    expr = Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                                                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x7d6b37240000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:288:61: 0x11b1e12 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_simple_arithmetic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:6:49 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:24:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdf3575158 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x77a414060000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x77a4140a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3533:59: 0x1216d38 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:862:53: 0x11d56de in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x77a4140e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 03_variable_assignment
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:6:51 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:21:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffbacb8fe8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x72af05ba0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x72af05be0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3533:59: 0x1216d38 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:862:53: 0x11d56de in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x72af0d440000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_nested_operations
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:5:25 - Error parsing function statement
INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:23:9 - Failed to parse statement
INFO: Recovered at delimiter 'RightParen' after skipping 5 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:39 - Failed to parse statement
INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Synchronizing parser after error (context: synchronize)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 4
Semicolon recoveries: 6
Statement recoveries: 4
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 11
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdd7a26d68 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Integer
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Integer
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Integer
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd7a26d68 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffdd7a26d68
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd7a26d68 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffdd7a26d68
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd7a26d68 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffdd7a26d68
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd7a26d68 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffdd7a26d68
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd7a26d68 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffdd7a26d68
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd7a26d68 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffdd7a26d68
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd7a26d68 with 5 variables for 'final'
DEBUG: Found 'final' in environment@interpreter.Environment@7ffdd7a26d68
12 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Test Complete === 
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x74ba48920000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x74ba48920200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x74ba48920280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x74ba489a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1040:61: 0x1204d60 in parseLetStatement (cursed_compiler_main.zig)
            const init_ptr = try self.arena_allocator.create(Expression);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:733:60: 0x11d3266 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x74ba48960000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:288:61: 0x11b1e12 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x74ba489e1000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:288:61: 0x11b1e12 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x74ba48900000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x142b3c0 in create__anon_71883 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3960:48: 0x140b683 in allocateMethodCall (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1729:81: 0x13fca2b in parseCall (cursed_compiler_main.zig)
                    expr = Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                                                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_fizzbuzz
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:5:16 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 6
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd477a4288 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_if_statements
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:6:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 7 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:37:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 11
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd4fc75998 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x777492b20000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x777492b60200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3533:59: 0x1216d38 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:862:53: 0x11d56de in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x777492ba0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_loops
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:6:37 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:24:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffeab0bf868 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7aa1b47a0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7aa1b47e0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3533:59: 0x1216d38 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:862:53: 0x11d56de in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7aa1bc040000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_boundary_values
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe3f9aea48 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@78f8cbcc0000 with parent@*interpreter.Environment@7ffe3f9aea48
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffe3f9ac348, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Boundary Values Test === 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Small numbers: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Integer
1 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Integer
0 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
-1 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Large numbers: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Integer
999999 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Integer
1000000 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
-999999 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Decimal boundaries: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Float
0.1 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Float
0 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Float
-0.1 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Test Complete === 
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x78f8cbcc0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123936a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239890 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f2167 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c7323 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x78f8cbd60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x78f8cbd60100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132825e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e7745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128d3f2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252b2c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1721:49: 0x13fc719 in parseCall (cursed_compiler_main.zig)
                            try arguments.append(self.allocator, arg_ptr);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd60780 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x78f8cbda0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

error(gpa): memory address 0x78f8cbd20000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x142b3c0 in create__anon_71883 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3960:48: 0x140b683 in allocateMethodCall (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(ast.MethodCallExpression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1729:81: 0x13fca2b in parseCall (cursed_compiler_main.zig)
                    expr = Expression{ .MethodCall = try self.allocateMethodCall(ast.MethodCallExpression{
                                                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbde0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1719:76: 0x13fc616 in parseCall (cursed_compiler_main.zig)
                            const arg_ptr = try self.arena_allocator.create(Expression);
                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1548:39: 0x13d5611 in parseFactor (cursed_compiler_main.zig)
        var expr = try self.parseUnary();
                                      ^

error(gpa): memory address 0x78f8cbd00000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:874:53: 0x11d5997 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x78f8cbce0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3887:48: 0x1251341 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1730:62: 0x13fc8ab in parseCall (cursed_compiler_main.zig)
                        .object = try self.allocateExpression(expr),
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

error(gpa): memory address 0x78f8cbd42000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3887:48: 0x1251341 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1730:62: 0x13fc8ab in parseCall (cursed_compiler_main.zig)
                        .object = try self.allocateExpression(expr),
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1613:30: 0x13e7b26 in parseUnary (cursed_compiler_main.zig)
        return self.parseCall();
                             ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_empty_inputs
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:5:20 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 0
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdac57b118 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_division_by_zero
**Status:** FAIL
**Details:** Both modes failed, but differently

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd3af25598 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@72326ace0000 with parent@*interpreter.Environment@7ffd3af25598
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffd3af22e98, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Division by Zero Test === 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Normal division: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
5 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Attempting division by zero: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
thread 1688067 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:1487:39: 0x12a553d in evaluateBinary (cursed_compiler_main.zig)
                if (right_num == 0.0) return InterpreterError.DivisionByZero;
                                      ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1417:37: 0x126e5f1 in evaluateExpression (cursed_compiler_main.zig)
            .Binary => |bin| return try self.evaluateBinary(bin),
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1432:43: 0x126f57b in evaluateExpression (cursed_compiler_main.zig)
                        const arg_value = try self.evaluateExpression(arg_ptr.*);
                                          ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123b58f in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x1313e8c in sliceAsBytes__anon_46251 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12cdb94 in free__anon_42344 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1270859 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123a94d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f2167 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c7323 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a6dad in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x72327262a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x72327262a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x15003c4 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

### Output Diff:
```diff
--- /tmp/tmp.QeJ5L6Px0b	2025-08-31 09:59:08.822155560 +0300
+++ /tmp/tmp.bjDhw6NXtF	2025-08-31 09:59:08.823155555 +0300
@@ -1,75 +1 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd3af25598 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@72326ace0000 with parent@*interpreter.Environment@7ffd3af25598
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffd3af22e98, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: String
-=== Division by Zero Test === 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: String
-Normal division: 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-5 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: String
-Attempting division by zero: 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-thread 1688067 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:1487:39: 0xADDRESS in evaluateBinary (cursed_compiler_main.zig)
-                if (right_num == 0.0) return InterpreterError.DivisionByZero;
-                                      ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1417:37: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Binary => |bin| return try self.evaluateBinary(bin),
-                                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1432:43: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                        const arg_value = try self.evaluateExpression(arg_ptr.*);
-                                          ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-                _ = try self.evaluateExpression(expr);
-                    ^
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_46251 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_42344 (std.zig)
-    const bytes = mem.sliceAsBytes(memory);
-                                  ^
-/snap/zig/14937/lib/std/array_list.zig:655:21: 0xADDRESS in deinit (std.zig)
-            gpa.free(self.allocatedSlice());
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        defer return_values.deinit(self.allocator);
-                                  ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
-        return self.execute(program);
-                           ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    cursed_interpreter.interpret(program) catch |err| {
-                                ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0xADDRESS in main (cursed_compiler_main.zig)
-        try interpretSource(allocator, source, filename.?, verbose);
-                           ^
-/snap/zig/14937/lib/std/start.zig:627:37: 0xADDRESS in main (std.zig)
-            const result = root.main() catch |err| {
-                                    ^
-../sysdeps/nptl/libc_start_call_main.h:58:16: 0xADDRESS in __libc_start_call_main (../sysdeps/x86/libc-start.c)
-../csu/libc-start.c:360:3: 0xADDRESS in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
-???:?:?: 0xADDRESS in ??? (???)
-???:?:?: 0xADDRESS in ??? (???)
-timeout: the monitored command dumped core
-INTERPRETER_ERROR: Exit code 134
+COMPILE_ERROR: Binary not created
```

---

## Test: 02_undefined_variable
**Status:** FAIL
**Details:** Both modes failed, but differently

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff15b4f778 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@756181f40000 with parent@*interpreter.Environment@7fff15b4f778
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7fff15b4d078, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Undefined Variable Test === 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Using undefined variable: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@756181f40000 with 0 variables for 'undefined_var'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7fff15b4f778 with 1 variables for 'undefined_var'
DEBUG: Variable 'undefined_var' not found in any environment after 2 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@756181f40000 with 0 variables for 'self'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7fff15b4f778 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 2 hops
DEBUG: Variable 'undefined_var' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/undefined_var/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'undefined_var': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/undefined_var/mod.csd for true self-hosting
DEBUG: Failed to load module 'undefined_var'
thread 1688091 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0x12a39fe in get (cursed_compiler_main.zig)
        return InterpreterError.UndefinedVariable;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0x126e50a in evaluateExpression (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedVariable;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1432:43: 0x126f57b in evaluateExpression (cursed_compiler_main.zig)
                        const arg_value = try self.evaluateExpression(arg_ptr.*);
                                          ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123b58f in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x1313e8c in sliceAsBytes__anon_46251 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12cdb94 in free__anon_42344 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1270859 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123a94d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f2167 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c7323 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a6dad in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x756189a2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x756189a2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x15003c4 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

### Output Diff:
```diff
--- /tmp/tmp.GV4KxkdEOR	2025-08-31 09:59:11.420144924 +0300
+++ /tmp/tmp.RxZpad2mA3	2025-08-31 09:59:11.422144916 +0300
@@ -1,74 +1 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff15b4f778 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@756181f40000 with parent@*interpreter.Environment@7fff15b4f778
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7fff15b4d078, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: String
-=== Undefined Variable Test === 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: String
-Using undefined variable: 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@756181f40000 with 0 variables for 'undefined_var'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7fff15b4f778 with 1 variables for 'undefined_var'
-DEBUG: Variable 'undefined_var' not found in any environment after 2 hops
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@756181f40000 with 0 variables for 'self'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7fff15b4f778 with 1 variables for 'self'
-DEBUG: Variable 'self' not found in any environment after 2 hops
-DEBUG: Variable 'undefined_var' not found, attempting lazy module loading...
-DEBUG: Could not open CURSED stdlib file stdlib/undefined_var/mod.csd: error.FileNotFound
-ERROR: No CURSED stdlib implementation found for module 'undefined_var': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/undefined_var/mod.csd for true self-hosting
-DEBUG: Failed to load module 'undefined_var'
-thread 1688091 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0xADDRESS in get (cursed_compiler_main.zig)
-        return InterpreterError.UndefinedVariable;
-        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                    return InterpreterError.UndefinedVariable;
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1432:43: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                        const arg_value = try self.evaluateExpression(arg_ptr.*);
-                                          ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-                _ = try self.evaluateExpression(expr);
-                    ^
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_46251 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_42344 (std.zig)
-    const bytes = mem.sliceAsBytes(memory);
-                                  ^
-/snap/zig/14937/lib/std/array_list.zig:655:21: 0xADDRESS in deinit (std.zig)
-            gpa.free(self.allocatedSlice());
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        defer return_values.deinit(self.allocator);
-                                  ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
-        return self.execute(program);
-                           ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    cursed_interpreter.interpret(program) catch |err| {
-                                ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0xADDRESS in main (cursed_compiler_main.zig)
-        try interpretSource(allocator, source, filename.?, verbose);
-                           ^
-/snap/zig/14937/lib/std/start.zig:627:37: 0xADDRESS in main (std.zig)
-            const result = root.main() catch |err| {
-                                    ^
-../sysdeps/nptl/libc_start_call_main.h:58:16: 0xADDRESS in __libc_start_call_main (../sysdeps/x86/libc-start.c)
-../csu/libc-start.c:360:3: 0xADDRESS in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
-???:?:?: 0xADDRESS in ??? (???)
-???:?:?: 0xADDRESS in ??? (???)
-timeout: the monitored command dumped core
-INTERPRETER_ERROR: Exit code 134
+COMPILE_ERROR: Binary not created
```

---

## Test: 01_simple_function
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:5:19 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 0
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffdbc04288 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_recursive_function
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:5:17 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:31:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 1
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffeeb3b70e8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 03_nested_function_calls
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.csd:5:20 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.csd:33:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe9fc694f8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 04_function_parameters
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/04_function_parameters.csd:9:17 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/04_function_parameters.csd:33:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 6
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff0e57f9c8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'no_params'
DEBUG: Executing statement type: Function
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7a6770560000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a6770560080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7a67705a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2398:62: 0x1205cba in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:767:49: 0x11d4210 in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7a67705e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_recursive_depth
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.csd:5:22 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.csd:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 0
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff3974d548 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_computation_intensive
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.csd:5:17 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.csd:33:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 6
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc33cb95b8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_mathz_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:8:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:8:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:8:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffef42bab8 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fffef42bab8 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fffef42bab8 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7c346ba60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7c346ba60080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7c34641e0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:874:53: 0x11d5997 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7c34641c0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:288:61: 0x11b1e12 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7c346ba40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 02_stringz_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:8:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:8:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:8:46 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe6b443dd8 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe6b443dd8 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe6b443dd8 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7d0d5ed00000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7d0d5ed00080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d0d5ed40200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:874:53: 0x11d5997 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7d0d5ed80800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:288:61: 0x11b1e12 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7d0d5ecc0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 03_mathz_advanced
**Status:** FAIL
**Details:** Both modes failed, but differently

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc1aa56048 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7f1140d80000 with parent@*interpreter.Environment@7ffc1aa56048
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffc1aa53948, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
=== Advanced Mathz Test === 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: String
Power function: 
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: MethodCall
DEBUG: Evaluating expression type: MethodCall
DEBUG: Method call - evaluating object for 'pow' method
DEBUG: About to evaluate expression for object...
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f1140d80000 with 0 variables for 'mathz'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffc1aa56048 with 1 variables for 'mathz'
DEBUG: Variable 'mathz' not found in any environment after 2 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f1140d80000 with 0 variables for 'self'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffc1aa56048 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 2 hops
DEBUG: Variable 'mathz' not found, attempting lazy module loading...
DEBUG: Successfully read CURSED stdlib file stdlib/mathz/mod.csd (1378 bytes)
DEBUG: Successfully parsed CURSED stdlib stdlib/mathz/mod.csd (11 statements)
DEBUG: Registering CURSED stdlib function 'mathz.abs_normie'
DEBUG: Registering CURSED stdlib function 'mathz.max_normie'
DEBUG: Registering CURSED stdlib function 'mathz.min_normie'
DEBUG: Registering CURSED stdlib function 'mathz.add_two'
DEBUG: Registering CURSED stdlib function 'mathz.subtract_two'
DEBUG: Registering CURSED stdlib function 'mathz.multiply_two'
DEBUG: Registering CURSED stdlib function 'mathz.power_int'
DEBUG: Registering CURSED stdlib function 'mathz.factorial'
DEBUG: Registering CURSED stdlib function 'mathz.is_even'
DEBUG: Registering CURSED stdlib function 'mathz.is_odd'
DEBUG: Registering CURSED stdlib function 'mathz.clamp'
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Executing statement type: Function
DEBUG: Exporting function name='subtract_two' length=12 ptr=@u8@7f11486641df
DEBUG: Copied function name to stable memory: 'subtract_two' ptr=@u8@7f1140d40000
DEBUG: Exported CURSED function mathz.subtract_two
DEBUG: Exporting function name='max_normie' length=10 ptr=@u8@7f11486640eb
DEBUG: Copied function name to stable memory: 'max_normie' ptr=@u8@7f1140d40010
DEBUG: Exported CURSED function mathz.max_normie
DEBUG: Exporting function name='multiply_two' length=12 ptr=@u8@7f114866421a
DEBUG: Copied function name to stable memory: 'multiply_two' ptr=@u8@7f1140d40020
DEBUG: Exported CURSED function mathz.multiply_two
DEBUG: Exporting function name='factorial' length=9 ptr=@u8@7f1148664380
DEBUG: Copied function name to stable memory: 'factorial' ptr=@u8@7f1140d40030
DEBUG: Exported CURSED function mathz.factorial
DEBUG: Exporting function name='min_normie' length=10 ptr=@u8@7f114866414a
DEBUG: Copied function name to stable memory: 'min_normie' ptr=@u8@7f1140d40040
DEBUG: Exported CURSED function mathz.min_normie
DEBUG: Exporting function name='is_even' length=7 ptr=@u8@7f1148664451
DEBUG: Copied function name to stable memory: 'is_even' ptr=@u8@7f1140d20000
DEBUG: Exported CURSED function mathz.is_even
DEBUG: Exporting function name='add_two' length=7 ptr=@u8@7f11486641a9
DEBUG: Copied function name to stable memory: 'add_two' ptr=@u8@7f1140d20008
DEBUG: Exported CURSED function mathz.add_two
DEBUG: Exporting function name='power_int' length=9 ptr=@u8@7f1148664255
DEBUG: Copied function name to stable memory: 'power_int' ptr=@u8@7f1140d40050
DEBUG: Exported CURSED function mathz.power_int
DEBUG: Exporting function name='is_odd' length=6 ptr=@u8@7f1148664485
DEBUG: Copied function name to stable memory: 'is_odd' ptr=@u8@7f1140d20010
DEBUG: Exported CURSED function mathz.is_odd
DEBUG: Exporting function name='clamp' length=5 ptr=@u8@7f11486644b8
DEBUG: Copied function name to stable memory: 'clamp' ptr=@u8@7f1140d20018
DEBUG: Exported CURSED function mathz.clamp
DEBUG: Exporting function name='abs_normie' length=10 ptr=@u8@7f1148664090
DEBUG: Copied function name to stable memory: 'abs_normie' ptr=@u8@7f1140d40060
DEBUG: Exported CURSED function mathz.abs_normie
DEBUG: Extracted 11 functions from CURSED stdlib mathz
DEBUG: Successfully loaded CURSED stdlib module mathz with 11 functions
DEBUG: Successfully loaded mathz from CURSED stdlib
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f1140d80000 with 1 variables for 'mathz'
DEBUG: Found 'mathz' in environment@interpreter.Environment@7f1140d80000
DEBUG: Successfully loaded module 'mathz' on demand
DEBUG: Object evaluated to type: Module
DEBUG: Looking for function 'pow' in module (length: 3)
DEBUG: Function 'pow' not found in module
thread 1688202 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:2168:21: 0x12cc834 in evaluateMethodCall (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedFunction;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1445:24: 0x126fa1f in evaluateExpression (cursed_compiler_main.zig)
                return try self.evaluateMethodCall(object_as_member, method_call.arguments.items);
                       ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1432:43: 0x126f57b in evaluateExpression (cursed_compiler_main.zig)
                        const arg_value = try self.evaluateExpression(arg_ptr.*);
                                          ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123b58f in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x1313e8c in sliceAsBytes__anon_46251 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12cdb94 in free__anon_42344 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1270859 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123a94d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f2167 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c7323 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a6dad in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7f114882a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7f114882a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x15003c4 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

### Output Diff:
```diff
--- /tmp/tmp.X7sWgsFtXs	2025-08-31 09:59:15.095129879 +0300
+++ /tmp/tmp.h3lMLfsfvv	2025-08-31 09:59:15.096129875 +0300
@@ -1,139 +1 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc1aa56048 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@7f1140d80000 with parent@*interpreter.Environment@7ffc1aa56048
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffc1aa53948, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: String
-=== Advanced Mathz Test === 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: String
-Power function: 
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Evaluating expression type: MethodCall
-DEBUG: Method call - evaluating object for 'pow' method
-DEBUG: About to evaluate expression for object...
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f1140d80000 with 0 variables for 'mathz'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffc1aa56048 with 1 variables for 'mathz'
-DEBUG: Variable 'mathz' not found in any environment after 2 hops
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f1140d80000 with 0 variables for 'self'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffc1aa56048 with 1 variables for 'self'
-DEBUG: Variable 'self' not found in any environment after 2 hops
-DEBUG: Variable 'mathz' not found, attempting lazy module loading...
-DEBUG: Successfully read CURSED stdlib file stdlib/mathz/mod.csd (1378 bytes)
-DEBUG: Successfully parsed CURSED stdlib stdlib/mathz/mod.csd (11 statements)
-DEBUG: Registering CURSED stdlib function 'mathz.abs_normie'
-DEBUG: Registering CURSED stdlib function 'mathz.max_normie'
-DEBUG: Registering CURSED stdlib function 'mathz.min_normie'
-DEBUG: Registering CURSED stdlib function 'mathz.add_two'
-DEBUG: Registering CURSED stdlib function 'mathz.subtract_two'
-DEBUG: Registering CURSED stdlib function 'mathz.multiply_two'
-DEBUG: Registering CURSED stdlib function 'mathz.power_int'
-DEBUG: Registering CURSED stdlib function 'mathz.factorial'
-DEBUG: Registering CURSED stdlib function 'mathz.is_even'
-DEBUG: Registering CURSED stdlib function 'mathz.is_odd'
-DEBUG: Registering CURSED stdlib function 'mathz.clamp'
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Executing statement type: Function
-DEBUG: Exporting function name='subtract_two' length=12 ptr=@u8@7f11486641df
-DEBUG: Copied function name to stable memory: 'subtract_two' ptr=@u8@7f1140d40000
-DEBUG: Exported CURSED function mathz.subtract_two
-DEBUG: Exporting function name='max_normie' length=10 ptr=@u8@7f11486640eb
-DEBUG: Copied function name to stable memory: 'max_normie' ptr=@u8@7f1140d40010
-DEBUG: Exported CURSED function mathz.max_normie
-DEBUG: Exporting function name='multiply_two' length=12 ptr=@u8@7f114866421a
-DEBUG: Copied function name to stable memory: 'multiply_two' ptr=@u8@7f1140d40020
-DEBUG: Exported CURSED function mathz.multiply_two
-DEBUG: Exporting function name='factorial' length=9 ptr=@u8@7f1148664380
-DEBUG: Copied function name to stable memory: 'factorial' ptr=@u8@7f1140d40030
-DEBUG: Exported CURSED function mathz.factorial
-DEBUG: Exporting function name='min_normie' length=10 ptr=@u8@7f114866414a
-DEBUG: Copied function name to stable memory: 'min_normie' ptr=@u8@7f1140d40040
-DEBUG: Exported CURSED function mathz.min_normie
-DEBUG: Exporting function name='is_even' length=7 ptr=@u8@7f1148664451
-DEBUG: Copied function name to stable memory: 'is_even' ptr=@u8@7f1140d20000
-DEBUG: Exported CURSED function mathz.is_even
-DEBUG: Exporting function name='add_two' length=7 ptr=@u8@7f11486641a9
-DEBUG: Copied function name to stable memory: 'add_two' ptr=@u8@7f1140d20008
-DEBUG: Exported CURSED function mathz.add_two
-DEBUG: Exporting function name='power_int' length=9 ptr=@u8@7f1148664255
-DEBUG: Copied function name to stable memory: 'power_int' ptr=@u8@7f1140d40050
-DEBUG: Exported CURSED function mathz.power_int
-DEBUG: Exporting function name='is_odd' length=6 ptr=@u8@7f1148664485
-DEBUG: Copied function name to stable memory: 'is_odd' ptr=@u8@7f1140d20010
-DEBUG: Exported CURSED function mathz.is_odd
-DEBUG: Exporting function name='clamp' length=5 ptr=@u8@7f11486644b8
-DEBUG: Copied function name to stable memory: 'clamp' ptr=@u8@7f1140d20018
-DEBUG: Exported CURSED function mathz.clamp
-DEBUG: Exporting function name='abs_normie' length=10 ptr=@u8@7f1148664090
-DEBUG: Copied function name to stable memory: 'abs_normie' ptr=@u8@7f1140d40060
-DEBUG: Exported CURSED function mathz.abs_normie
-DEBUG: Extracted 11 functions from CURSED stdlib mathz
-DEBUG: Successfully loaded CURSED stdlib module mathz with 11 functions
-DEBUG: Successfully loaded mathz from CURSED stdlib
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f1140d80000 with 1 variables for 'mathz'
-DEBUG: Found 'mathz' in environment@interpreter.Environment@7f1140d80000
-DEBUG: Successfully loaded module 'mathz' on demand
-DEBUG: Object evaluated to type: Module
-DEBUG: Looking for function 'pow' in module (length: 3)
-DEBUG: Function 'pow' not found in module
-thread 1688202 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:2168:21: 0xADDRESS in evaluateMethodCall (cursed_compiler_main.zig)
-                    return InterpreterError.UndefinedFunction;
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1445:24: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                return try self.evaluateMethodCall(object_as_member, method_call.arguments.items);
-                       ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1432:43: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                        const arg_value = try self.evaluateExpression(arg_ptr.*);
-                                          ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-                _ = try self.evaluateExpression(expr);
-                    ^
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_46251 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_42344 (std.zig)
-    const bytes = mem.sliceAsBytes(memory);
-                                  ^
-/snap/zig/14937/lib/std/array_list.zig:655:21: 0xADDRESS in deinit (std.zig)
-            gpa.free(self.allocatedSlice());
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        defer return_values.deinit(self.allocator);
-                                  ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
-        return self.execute(program);
-                           ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    cursed_interpreter.interpret(program) catch |err| {
-                                ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0xADDRESS in main (cursed_compiler_main.zig)
-        try interpretSource(allocator, source, filename.?, verbose);
-                           ^
-/snap/zig/14937/lib/std/start.zig:627:37: 0xADDRESS in main (std.zig)
-            const result = root.main() catch |err| {
-                                    ^
-../sysdeps/nptl/libc_start_call_main.h:58:16: 0xADDRESS in __libc_start_call_main (../sysdeps/x86/libc-start.c)
-../csu/libc-start.c:360:3: 0xADDRESS in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
-???:?:?: 0xADDRESS in ??? (???)
-???:?:?: 0xADDRESS in ??? (???)
-timeout: the monitored command dumped core
-INTERPRETER_ERROR: Exit code 134
+COMPILE_ERROR: Binary not created
```

---

## Test: 04_collections_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:8:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:8:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:8:49 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 10
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff3e7bd6e8 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fff3e7bd6e8 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fff3e7bd6e8 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x739a07a60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128779e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1246f95 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a52 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d29fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:300:42: 0x11b2259 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x739a07a60080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x739a07aa0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:874:53: 0x11d5997 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x739a07ae0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:288:61: 0x11b1e12 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x739a001e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

## Test: 01_string_operations
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:6:49 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:29:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff92329448 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x71bfd08e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125be5e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d715 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbae2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b690c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:33: 0x1203a0e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x71bfd0920200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205380 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3533:59: 0x1216d38 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:862:53: 0x11d56de in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:996:49: 0x120382b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x71bfd0960800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2880 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:997:61: 0x12038f9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:720:70: 0x11d2cde in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:287:36: 0x11b1d44 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created
```

---

