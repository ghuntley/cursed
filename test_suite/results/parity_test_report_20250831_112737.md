# CURSED Interpreter vs Compiler Parity Test Report

**Generated:** Sun Aug 31 11:28:00 AM EEST 2025
**Test Suite Version:** 1.0.0
**CURSED Compiler:** /home/ghuntley/cursed/test_suite/../zig-out/bin/cursed-compiler

## Executive Summary

- **Total Tests:** 26
- **Passed:** 0
- **Failed:** 22
- **Compile Errors:** 4
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

- 🚨 **High Priority:** Fix compilation failures (4 tests)
- 📋 **Low Priority:** Investigate output differences (22 tests)

## Detailed Test Results

## Test: 01_mixed_types
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffca3828b88 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7176ed4e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7176ed520200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0x12191a8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d6d25 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7176ed560800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.ezkSuW8Eby	2025-08-31 11:27:38.534662539 +0300
+++ /tmp/tmp.gMXon5GbN7	2025-08-31 11:27:38.534662539 +0300
@@ -1,92 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG parseStatement: current token = '.' (.Dot)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:6:53 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:31:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 4
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffca3828b88 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 02_edge_cases
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffa2fc8de8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7aecc54a0000 with parent@*interpreter.Environment@7fffa2fc8de8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7fffa2fc66e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Arithmetic Edge Cases Test ==="
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Zero operations:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
5
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
0
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
0
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Negative numbers:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
-2
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
-6
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
-6
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Large numbers:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
3000
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
999998
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7aecc54a0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123b9ca in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x123bef0 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f3aa7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c83f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a502c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7aecc5520000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7aecc5520100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc5520700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7aecc5560200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0x1254a11 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x13283e6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7aecc55a0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x1327c5b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0x1250f43 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x7aecc54e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7aecc54c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7aecc5502000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.UZ2cN5NPA1	2025-08-31 11:27:39.313617682 +0300
+++ /tmp/tmp.FV1c8c0eW1	2025-08-31 11:27:39.313617682 +0300
@@ -1,592 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffa2fc8de8 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@7aecc54a0000 with parent@*interpreter.Environment@7fffa2fc8de8
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7fffa2fc66e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Arithmetic Edge Cases Test ==="
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Zero operations:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-5
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-0
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-0
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Negative numbers:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Unary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
--2
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Unary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
--6
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Unary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
--6
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Large numbers:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-3000
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-999998
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Test Complete ==="
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
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
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                program.statements.append(self.allocator, anyopaque_ptr) catch {
-                                         ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
-        return self.parseExpressionPrattPrec(.None);
-                                            ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 03_operator_precedence
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffff6515fe8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@70b90ab00000 with parent@*interpreter.Environment@7ffff6515fe8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffff65138e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Operator Precedence Test ==="
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"2 + 3 * 4 should be 14:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
14
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"(2 + 3) * 4 should be 20:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
20
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"10 - 3 + 2 should be 9:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
9
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"8 / 2 * 3 should be 12:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
12
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x70b90ab00000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123b9ca in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x123bef0 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f3aa7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c83f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a502c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x70b90ab80000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x70b90ab80100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b90ab80580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x70b90abc0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0x1254a11 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x13283e6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70b912440800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x1327c5b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0x1250f43 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x70b90ab40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x70b90ab20000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x70b90ab62000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.BjvKBDcquC	2025-08-31 11:27:40.071166511 +0300
+++ /tmp/tmp.bOnPMdTNtm	2025-08-31 11:27:40.071166511 +0300
@@ -1,496 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffff6515fe8 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@70b90ab00000 with parent@*interpreter.Environment@7ffff6515fe8
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffff65138e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Operator Precedence Test ==="
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"2 + 3 * 4 should be 14:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-14
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"(2 + 3) * 4 should be 20:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-20
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"10 - 3 + 2 should be 9:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-9
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"8 / 2 * 3 should be 12:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-DEBUG: Evaluating expression type: Integer
-12
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Test Complete ==="
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
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
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                program.statements.append(self.allocator, anyopaque_ptr) catch {
-                                         ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
-        return self.parseExpressionPrattPrec(.None);
-                                            ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 04_complex_expressions
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc8fead398 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7055a7160000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7055a71a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0x12191a8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d6d25 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7055a71e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.DBD1u94Kfg	2025-08-31 11:27:40.723163654 +0300
+++ /tmp/tmp.G5uOJSGPZ2	2025-08-31 11:27:40.723163654 +0300
@@ -1,94 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG parseStatement: current token = '.' (.Dot)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:6:51 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd:23:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 7
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc8fead398 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 01_hello_world
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdb5bc0e08 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7b75f8ee0000 with parent@*interpreter.Environment@7ffdb5bc0e08
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffdb5bbe708, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Hello, CURSED World!"
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7b75f8ee0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123b9ca in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x123bef0 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f3aa7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c83f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a502c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7b75f8f80000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7b75f8f80080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7b75f8f80100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7b75f8f40400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0x1254a11 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x13283e6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7b75f8f20400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x1327c5b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0x1250f43 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.p20jTYYrbz	2025-08-31 11:27:41.397160701 +0300
+++ /tmp/tmp.9Wr6UZnHGA	2025-08-31 11:27:41.397160701 +0300
@@ -1,141 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdb5bc0e08 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@7b75f8ee0000 with parent@*interpreter.Environment@7ffdb5bc0e08
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffdb5bbe708, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Hello, CURSED World!"
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
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
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                program.statements.append(self.allocator, anyopaque_ptr) catch {
-                                         ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
-        return self.parseExpressionPrattPrec(.None);
-                                            ^
-
```

---

## Test: 02_simple_arithmetic
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffe700c9f8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7fedfa700000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7fedfa740200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0x12191a8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d6d25 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7fedfa780800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.xxHmBq3rAJ	2025-08-31 11:27:42.051157836 +0300
+++ /tmp/tmp.SjCgVZKFM2	2025-08-31 11:27:42.052157832 +0300
@@ -1,92 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG parseStatement: current token = '.' (.Dot)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:6:49 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:24:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 4
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffe700c9f8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 03_variable_assignment
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcdc00b698 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x795c237c0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x795c2b060200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0x12191a8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d6d25 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x795c2b040000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.I1sVPSu7EY	2025-08-31 11:27:42.716154924 +0300
+++ /tmp/tmp.iavSAvd6Je	2025-08-31 11:27:42.716154924 +0300
@@ -1,92 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG parseStatement: current token = '.' (.Dot)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:6:51 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:21:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 4
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcdc00b698 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 01_nested_operations
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:5:25 - Error parsing function statement
INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:23:9 - Failed to parse statement
INFO: Recovered at delimiter 'RightParen' after skipping 5 tokens
INFO: Attempting additional statement recovery
DEBUG parseStatement: current token = 'spill' (.Spill)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:39 - Failed to parse statement
INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
INFO: Attempting additional statement recovery
DEBUG parseStatement: current token = 'sus' (.Sus)
DEBUG parseStatement: current token = 'sus' (.Sus)
DEBUG parseStatement: current token = 'sus' (.Sus)
DEBUG parseStatement: current token = 'sus' (.Sus)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = '}' (.RightBrace)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe5c456818 (parent: *interpreter.Environment@0)
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
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5c456818
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5c456818
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5c456818
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5c456818
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5c456818
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5c456818
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 5 variables for 'final'
DEBUG: Found 'final' in environment@interpreter.Environment@7ffe5c456818
12
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Test Complete ==="
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7c6f32080000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7c6f32080080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7c6f32080100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7c6f32100200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1282:61: 0x1206a10 in parseLetStatement (cursed_compiler_main.zig)
            const init_ptr = try self.arena_allocator.create(Expression);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:961:60: 0x11d47ed in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x7c6f320c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b2e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7c6f32141000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b2e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7c6f32060000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0x1254a11 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:409:52: 0x1327e96 in parsePrattCall (cursed_compiler_main.zig)
            .function = try self.allocateExpression(left),
                                                   ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.3Jnq5fuqaY	2025-08-31 11:27:43.393151958 +0300
+++ /tmp/tmp.b2JV9e7Xdn	2025-08-31 11:27:43.393151958 +0300
@@ -1,241 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:5:25 - Error parsing function statement
-INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:23:9 - Failed to parse statement
-INFO: Recovered at delimiter 'RightParen' after skipping 5 tokens
-INFO: Attempting additional statement recovery
-DEBUG parseStatement: current token = 'spill' (.Spill)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:39 - Failed to parse statement
-INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-DEBUG parseStatement: current token = 'sus' (.Sus)
-DEBUG parseStatement: current token = 'sus' (.Sus)
-DEBUG parseStatement: current token = 'sus' (.Sus)
-DEBUG parseStatement: current token = 'sus' (.Sus)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = '}' (.RightBrace)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Synchronizing parser after error (context: synchronize)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 4
-Semicolon recoveries: 6
-Statement recoveries: 4
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 11
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe5c456818 (parent: *interpreter.Environment@0)
-DEBUG: Executing statement type: Let
-DEBUG: Evaluating expression type: Integer
-DEBUG: Executing statement type: Let
-DEBUG: Evaluating expression type: Integer
-DEBUG: Executing statement type: Let
-DEBUG: Evaluating expression type: Integer
-DEBUG: Executing statement type: Let
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'a'
-DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5c456818
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'b'
-DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5c456818
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'b'
-DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5c456818
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'c'
-DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5c456818
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'a'
-DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5c456818
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 4 variables for 'c'
-DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5c456818
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5c456818 with 5 variables for 'final'
-DEBUG: Found 'final' in environment@interpreter.Environment@7ffe5c456818
-12
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Test Complete ==="
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                program.statements.append(self.allocator, anyopaque_ptr) catch {
-                                         ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1282:61: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            const init_ptr = try self.arena_allocator.create(Expression);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:961:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
-                                                           ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                const stmt_ptr = self.arena_allocator.create(Statement) catch {
-                                                            ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0xADDRESS in main (cursed_compiler_main.zig)
-        try interpretSource(allocator, source, filename.?, verbose);
-                           ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                const stmt_ptr = self.arena_allocator.create(Statement) catch {
-                                                            ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0xADDRESS in main (cursed_compiler_main.zig)
-        try interpretSource(allocator, source, filename.?, verbose);
-                           ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:409:52: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-            .function = try self.allocateExpression(left),
-                                                   ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
```

---

## Test: 02_fizzbuzz
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffca89c77a8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.1LGNEjFbOx	2025-08-31 11:27:43.748597338 +0300
+++ /tmp/tmp.qb3eUnvNfP	2025-08-31 11:27:43.748597338 +0300
@@ -1,18 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:5:16 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:28:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 2
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 6
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffca89c77a8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_if_statements
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff588fcbf8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x75524c160000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x75524c1a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0x12191a8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d6d25 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x75524c1e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.FIBWBWM9aH	2025-08-31 11:27:44.402147540 +0300
+++ /tmp/tmp.fExGExUg3L	2025-08-31 11:27:44.402147540 +0300
@@ -1,94 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG parseStatement: current token = '.' (.Dot)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:6:44 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 7 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd:37:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 11
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff588fcbf8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 02_loops
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffeb1f48a48 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x70a08f920000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x70a08f9a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0x12191a8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d6d25 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x70a08f940000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.G8JBGVDeEb	2025-08-31 11:27:45.057144672 +0300
+++ /tmp/tmp.dzPj9jKZ29	2025-08-31 11:27:45.057144672 +0300
@@ -1,92 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG parseStatement: current token = '.' (.Dot)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:6:37 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd:24:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 4
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffeb1f48a48 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 01_boundary_values
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdb57e3388 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@70e1c90e0000 with parent@*interpreter.Environment@7ffdb57e3388
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffdb57e0c88, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Boundary Values Test ==="
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Small numbers:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Integer
1
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Integer
0
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
-1
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Large numbers:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Integer
999999
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Integer
1000000
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Integer
-999999
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Decimal boundaries:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Float
0.1
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Float
0
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Unary
DEBUG: Evaluating expression type: Float
-0.1
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x70e1c90e0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123b9ca in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x123bef0 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f3aa7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c83f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a502c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x70e1c9160000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x70e1c9160100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132f42e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12ebfc5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12915e2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x12561fc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x1327d5e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c9160780 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x70e1c91a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0x1254a11 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x13283e6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x70e1c91e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x1327c5b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0x1250f43 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x70e1c9120000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x70e1c9100000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0x1254a11 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:362:51: 0x132779f in parsePrattUnary (cursed_compiler_main.zig)
            .operand = try self.allocateExpression(right),
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:1667:35: 0x128ee61 in parseExpressionPrattPrec (cursed_compiler_main.zig)
        var left = try prefix_fn.?(self);
                                  ^

error(gpa): memory address 0x70e1c9142000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x1327c5b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0x128f09b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0x1250f43 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.EjKuJxas6c	2025-08-31 11:27:45.852677095 +0300
+++ /tmp/tmp.7t6f49dhN2	2025-08-31 11:27:45.852677095 +0300
@@ -1,607 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdb57e3388 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@70e1c90e0000 with parent@*interpreter.Environment@7ffdb57e3388
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffdb57e0c88, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Boundary Values Test ==="
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Small numbers:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Integer
-1
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Integer
-0
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Unary
-DEBUG: Evaluating expression type: Integer
--1
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Large numbers:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Integer
-999999
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Integer
-1000000
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Unary
-DEBUG: Evaluating expression type: Integer
--999999
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Decimal boundaries:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Float
-0.1
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Float
-0
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Unary
-DEBUG: Evaluating expression type: Float
--0.1
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Test Complete ==="
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
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
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                try arguments.append(self.allocator, arg_ptr);
-                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                program.statements.append(self.allocator, anyopaque_ptr) catch {
-                                         ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
-        return self.parseExpressionPrattPrec(.None);
-                                            ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4158:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:362:51: 0xADDRESS in parsePrattUnary (cursed_compiler_main.zig)
-            .operand = try self.allocateExpression(right),
-                                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:1667:35: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-        var left = try prefix_fn.?(self);
-                                  ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1674:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1657:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
-        return self.parseExpressionPrattPrec(.None);
-                                            ^
-
```

---

## Test: 02_empty_inputs
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe1d529658 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.Lai9ph6OIf	2025-08-31 11:27:46.211139619 +0300
+++ /tmp/tmp.q59jxZpzyB	2025-08-31 11:27:46.211139619 +0300
@@ -1,16 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:5:20 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:27:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 2
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 0
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe1d529658 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_division_by_zero
**Status:** FAIL
**Details:** Interpreter failed but compiled mode succeeded

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff8c9af5f8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@76d8e8ee0000 with parent@*interpreter.Environment@7fff8c9af5f8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7fff8c9acef8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Division by Zero Test ==="
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Normal division:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
5
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Attempting division by zero:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Integer
DEBUG: Evaluating expression type: Integer
thread 1708316 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:1487:39: 0x12a972d in evaluateBinary (cursed_compiler_main.zig)
                if (right_num == 0.0) return InterpreterError.DivisionByZero;
                                      ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1417:37: 0x1271da1 in evaluateExpression (cursed_compiler_main.zig)
            .Binary => |bin| return try self.evaluateBinary(bin),
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1588:37: 0x12b1154 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x1272040 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123dbef in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x131870c in sliceAsBytes__anon_47519 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12d1d84 in free__anon_42931 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1274009 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123cfad in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f3aa7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c83f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a502c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a7dad in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x76d8f0a2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x76d8f0a2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1507594 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.Q7HrgfMszS	2025-08-31 10:58:51.648489720 +0300
+++ /tmp/tmp.3LBbZFjyZT	2025-08-31 10:58:51.649489716 +0300
@@ -1,18 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:6:48 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:15:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 3
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc92b4c468 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_undefined_variable
**Status:** FAIL
**Details:** Interpreter failed but compiled mode succeeded

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffee48ad5a8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7a94ce980000 with parent@*interpreter.Environment@7ffee48ad5a8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffee48aaea8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Undefined Variable Test ==="
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Using undefined variable:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7a94ce980000 with 0 variables for 'undefined_var'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffee48ad5a8 with 1 variables for 'undefined_var'
DEBUG: Variable 'undefined_var' not found in any environment after 2 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7a94ce980000 with 0 variables for 'self'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffee48ad5a8 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 2 hops
DEBUG: Variable 'undefined_var' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/undefined_var/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'undefined_var': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/undefined_var/mod.csd for true self-hosting
DEBUG: Failed to load module 'undefined_var'
thread 1708342 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0x12a7bee in get (cursed_compiler_main.zig)
        return InterpreterError.UndefinedVariable;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0x1271cba in evaluateExpression (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedVariable;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1588:37: 0x12b1154 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x1272040 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123dbef in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x131870c in sliceAsBytes__anon_47519 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12d1d84 in free__anon_42931 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1274009 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123cfad in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f3aa7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c83f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a502c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a7dad in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7a94d642a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7a94d642a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1507594 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.YJxunFFTN0	2025-08-31 10:58:52.001488318 +0300
+++ /tmp/tmp.wQHFcVaPCQ	2025-08-31 10:58:52.001488318 +0300
@@ -1,18 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:6:50 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:12:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 3
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe70c8fcf8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_simple_function
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffde93d2818 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.vu7pWZBG2B	2025-08-31 11:27:52.619575953 +0300
+++ /tmp/tmp.3U9JDhf1uT	2025-08-31 11:27:52.619575953 +0300
@@ -1,16 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:5:19 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:25:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 2
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 0
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffde93d2818 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_recursive_function
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe081ad5e8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.jN45BNvTKt	2025-08-31 11:27:52.975110023 +0300
+++ /tmp/tmp.xetSIL7sDR	2025-08-31 11:27:52.975110023 +0300
@@ -1,18 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:5:17 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:31:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 2
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 1
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe081ad5e8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 03_nested_function_calls
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe98aae1e8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.FYIVoTyZ9c	2025-08-31 11:27:53.327108484 +0300
+++ /tmp/tmp.VXfTwmMEkR	2025-08-31 11:27:53.327108484 +0300
@@ -1,18 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.csd:5:20 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.csd:33:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 2
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 4
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe98aae1e8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 04_function_parameters
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'damn' (.Damn)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: true
DEBUG: Return statement matched!
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff36705938 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'no_params'
DEBUG: Executing statement type: Function
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7a305b660000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a305b660080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7a305b6a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2669:62: 0x120800a in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1000:49: 0x11d5852 in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7a305b6e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../04_function_parameters
```

---

## Test: 01_recursive_depth
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcc4677428 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.yQf8cE0Px1	2025-08-31 11:27:54.308104194 +0300
+++ /tmp/tmp.y9grR3aFAq	2025-08-31 11:27:54.308104194 +0300
@@ -1,16 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.csd:5:22 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.csd:25:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 2
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 0
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcc4677428 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_computation_intensive
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdca794e48 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.yRD7171Ubx	2025-08-31 11:27:54.662102646 +0300
+++ /tmp/tmp.IUWbnVB2dp	2025-08-31 11:27:54.662102646 +0300
@@ -1,18 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.csd:5:17 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.csd:33:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 2
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 6
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdca794e48 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_mathz_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG parseStatement: current token = 'using' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'mathz' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcafac1f18 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffcafac1f18 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffcafac1f18 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7d481a0e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7d481a0e0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d481a120200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7d481a160800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b2e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7d481a0a0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_mathz_basic
```

---

## Test: 02_stringz_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG parseStatement: current token = 'using' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'stringz' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffd03e4698 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fffd03e4698 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fffd03e4698 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x782f8bf20000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x782f8bf20080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x782f8bf60200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x782f8bfa0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b2e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x782f8bee0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_stringz_basic
```

---

## Test: 03_mathz_advanced
**Status:** FAIL
**Details:** Both modes failed, but differently

### Interpreter Output:
```
DEBUG parseStatement: current token = 'using' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'mathz' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd1e087218 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7d460af80000 with parent@*interpreter.Environment@7ffd1e087218
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffd1e084b18, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Advanced Mathz Test ==="
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Power function:"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Call
DEBUG: Detected method call: Identifier.pow
DEBUG: Method call - evaluating object for 'pow' method
DEBUG: About to evaluate expression for object...
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7d460af80000 with 0 variables for 'mathz'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffd1e087218 with 1 variables for 'mathz'
DEBUG: Variable 'mathz' not found in any environment after 2 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7d460af80000 with 0 variables for 'self'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffd1e087218 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 2 hops
DEBUG: Variable 'mathz' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/mathz/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
DEBUG: Failed to load module 'mathz'
thread 1708544 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0x12a7bee in get (cursed_compiler_main.zig)
        return InterpreterError.UndefinedVariable;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0x1271cba in evaluateExpression (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedVariable;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2072:24: 0x12cdb29 in evaluateMethodCall (cursed_compiler_main.zig)
        const object = try self.evaluateExpression(member.object.*);
                       ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1602:28: 0x12b16d2 in evaluateCall (cursed_compiler_main.zig)
                    return try self.evaluateMethodCall(member.*, call.arguments.items);
                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x1272040 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1588:37: 0x12b1154 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x1272040 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123dbef in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x131870c in sliceAsBytes__anon_47519 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12d1d84 in free__anon_42931 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1274009 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123cfad in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f3aa7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c83f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a502c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a7dad in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7d4612a2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7d4612a2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1507594 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_mathz_advanced
```

### Output Diff:
```diff
--- /tmp/tmp.Uwsv28XJCD	2025-08-31 11:27:59.029083559 +0300
+++ /tmp/tmp.iqnrUBF6gg	2025-08-31 11:27:59.031491966 +0300
@@ -1,196 +1 @@
-DEBUG parseStatement: current token = 'using' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'mathz' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd1e087218 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@7d460af80000 with parent@*interpreter.Environment@7ffd1e087218
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffd1e084b18, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"=== Advanced Mathz Test ==="
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Power function:"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Call
-DEBUG: Detected method call: Identifier.pow
-DEBUG: Method call - evaluating object for 'pow' method
-DEBUG: About to evaluate expression for object...
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7d460af80000 with 0 variables for 'mathz'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffd1e087218 with 1 variables for 'mathz'
-DEBUG: Variable 'mathz' not found in any environment after 2 hops
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7d460af80000 with 0 variables for 'self'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffd1e087218 with 1 variables for 'self'
-DEBUG: Variable 'self' not found in any environment after 2 hops
-DEBUG: Variable 'mathz' not found, attempting lazy module loading...
-DEBUG: Could not open CURSED stdlib file stdlib/mathz/mod.csd: error.FileNotFound
-ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
-DEBUG: Failed to load module 'mathz'
-thread 1708544 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0xADDRESS in get (cursed_compiler_main.zig)
-        return InterpreterError.UndefinedVariable;
-        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                    return InterpreterError.UndefinedVariable;
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2072:24: 0xADDRESS in evaluateMethodCall (cursed_compiler_main.zig)
-        const object = try self.evaluateExpression(member.object.*);
-                       ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1602:28: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                    return try self.evaluateMethodCall(member.*, call.arguments.items);
-                           ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                   ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1588:37: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                        const arg = try self.evaluateExpression(arg_expr.*);
-                                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                   ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-                _ = try self.evaluateExpression(expr);
-                    ^
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_47519 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_42931 (std.zig)
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
+COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_mathz_advanced
```

---

## Test: 04_collections_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
DEBUG parseStatement: current token = 'using' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'collections' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdd086c6b8 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd086c6b8 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdd086c6b8 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x72ae3c900000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128af4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12495f5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x12033b2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d3acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:511:42: 0x11b32fe in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x72ae3c900080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x72ae3c940200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1109:53: 0x11d6fe3 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x72ae3c980800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b2e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a4cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a6f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x72ae3c8c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../04_collections_basic
```

---

## Test: 01_string_operations
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG parseStatement: current token = 'slay' (.Slay)
DEBUG parseStatement: current token = 'vibez' (.Identifier)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG parseStatement: current token = '.' (.Dot)
DEBUG: Checking return statement tokens...
DEBUG: Check .Return: false
DEBUG: Check .Yolo: false
DEBUG: Check .Damn: false
DEBUG: Return statement NOT matched
DEBUG: Falling back to expression statement parsing
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe775e9048 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x72125fbc0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125f60e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121fd75 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dd422 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b79dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0x1205699 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x721267440200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f5135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12070d0 in create__anon_30722 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0x12191a8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d6d25 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0x120545b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x721267480800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f58a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c7d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d3950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0x1205529 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0x11d4253 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b2d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.luFiwPkkWH	2025-08-31 11:28:00.321077914 +0300
+++ /tmp/tmp.LGCYuwHmPa	2025-08-31 11:28:00.322077909 +0300
@@ -1,92 +0,0 @@
-DEBUG parseStatement: current token = 'slay' (.Slay)
-DEBUG parseStatement: current token = 'vibez' (.Identifier)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG parseStatement: current token = '.' (.Dot)
-DEBUG: Checking return statement tokens...
-DEBUG: Check .Return: false
-DEBUG: Check .Yolo: false
-DEBUG: Check .Damn: false
-DEBUG: Return statement NOT matched
-DEBUG: Falling back to expression statement parsing
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:6:49 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd:29:1 - Failed to parse statement
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 4
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe775e9048 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
-                                                       ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1242:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30722 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3804:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1231:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0xADDRESS in alloc (std.zig)
-                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
-                                          ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26064 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1232:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:948:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

