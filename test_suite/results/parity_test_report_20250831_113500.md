# CURSED Interpreter vs Compiler Parity Test Report

**Generated:** Sun Aug 31 11:35:25 AM EEST 2025
**Test Suite Version:** 1.0.0
**CURSED Compiler:** /home/ghuntley/cursed/test_suite/../zig-out/bin/cursed-compiler

## Executive Summary

- **Total Tests:** 29
- **Passed:** 0
- **Failed:** 24
- **Compile Errors:** 5
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
- **parser_fixes:** 3 tests
- **performance:** 2 tests
- **stdlib:** 4 tests
- **strings:** 1 tests

## Recommendations

- 🚨 **High Priority:** Fix compilation failures (5 tests)
- 📋 **Low Priority:** Investigate output differences (24 tests)

## Detailed Test Results

## Test: 01_mixed_types
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:6:53 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:31:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffffa856d48 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x764c96a60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x764c96aa0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0x12170f8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0x11d58c9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x764c96ae0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.Pppu7AanHy	2025-08-31 11:35:01.509280883 +0300
+++ /tmp/tmp.zXEVevBPBx	2025-08-31 11:35:01.509280883 +0300
@@ -1,78 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:6:53 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.💀:31:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffffa856d48 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc128cdbe8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7d1dc81e0000 with parent@*interpreter.Environment@7ffc128cdbe8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffc128cb4e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
error(gpa): memory address 0x7d1dc81e0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123972a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239c50 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f23b7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c73f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7d1dcfaa0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d1dcfaa0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfaa0700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7d1dcfae0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0x12517b1 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1324de6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7d1dcfb20800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x132465b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0x124e643 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x7d1dcfa60000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d1dcfa40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d1dcfa82000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.v92pnTVxhR	2025-08-31 11:35:02.297277582 +0300
+++ /tmp/tmp.OxURZuIjVu	2025-08-31 11:35:02.297277582 +0300
@@ -1,500 +0,0 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc128cdbe8 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@7d1dc81e0000 with parent@*interpreter.Environment@7ffc128cdbe8
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffc128cb4e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe81946df8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@71b2bdba0000 with parent@*interpreter.Environment@7ffe81946df8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffe819446f8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
error(gpa): memory address 0x71b2bdba0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123972a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239c50 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f23b7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c73f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x71b2c5460000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x71b2c5460100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c5460580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x71b2c54a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0x12517b1 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1324de6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x71b2c54e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x132465b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0x124e643 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x71b2bdbe0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x71b2bdbc0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x71b2c5442000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.sL5sEGCsiP	2025-08-31 11:35:03.084631240 +0300
+++ /tmp/tmp.ZwrravxQMI	2025-08-31 11:35:03.084631240 +0300
@@ -1,425 +0,0 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe81946df8 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@71b2bdba0000 with parent@*interpreter.Environment@7ffe81946df8
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffe819446f8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:6:51 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:23:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 7
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffecf72b328 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x71a580660000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x71a578de0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0x12170f8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0x11d58c9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x71a578dc0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.h0aeU7VOS0	2025-08-31 11:35:03.750271496 +0300
+++ /tmp/tmp.kVMISj2llb	2025-08-31 11:35:03.751271492 +0300
@@ -1,80 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:6:51 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.💀:23:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffecf72b328 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff8a97c568 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7e01609c0000 with parent@*interpreter.Environment@7fff8a97c568
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7fff8a979e68, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Hello, CURSED World!"
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7e01609c0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123972a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239c50 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f23b7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c73f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7e01682a0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7e01682a0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7e01682a0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7e0168260400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0x12517b1 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1324de6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7e0168240400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x132465b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0x124e643 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.FxZ7MZkeiN	2025-08-31 11:35:04.442608050 +0300
+++ /tmp/tmp.I435I6mnhM	2025-08-31 11:35:04.442608050 +0300
@@ -1,133 +0,0 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff8a97c568 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@7e01609c0000 with parent@*interpreter.Environment@7fff8a97c568
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7fff8a979e68, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:6:49 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:24:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd5b90e148 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7e191e560000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7e191e5a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0x12170f8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0x11d58c9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7e191e5e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.WQcgGXDqE7	2025-08-31 11:35:05.111265797 +0300
+++ /tmp/tmp.IsX89xjFgx	2025-08-31 11:35:05.112265792 +0300
@@ -1,78 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:6:49 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.💀:24:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd5b90e148 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:6:51 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:21:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdb3751bd8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x787a9f7c0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x787aa7060200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0x12170f8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0x11d58c9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x787aa7040000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.OaPreal7CC	2025-08-31 11:35:05.785540802 +0300
+++ /tmp/tmp.FjaQqngiBw	2025-08-31 11:35:05.785540802 +0300
@@ -1,78 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:6:51 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.💀:21:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdb3751bd8 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:5:25 - Error parsing function statement
INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:23:9 - Failed to parse statement
INFO: Recovered at delimiter 'RightParen' after skipping 5 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:32:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:32:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:32:39 - Failed to parse statement
INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Synchronizing parser after error (context: synchronize)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 4
Semicolon recoveries: 6
Statement recoveries: 4
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 11
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe5feb4308 (parent: *interpreter.Environment@0)
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
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5feb4308
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5feb4308
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5feb4308
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5feb4308
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5feb4308
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5feb4308
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 5 variables for 'final'
DEBUG: Found 'final' in environment@interpreter.Environment@7ffe5feb4308
12
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"=== Test Complete ==="
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x75f97b5a0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x75f97b5a0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x75f97b5a0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x75f982e40200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1268:61: 0x1205080 in parseLetStatement (cursed_compiler_main.zig)
            const init_ptr = try self.arena_allocator.create(Expression);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0x11d345d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x75f97b5c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x75f97b5e1000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x75f97b580000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0x12517b1 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:409:52: 0x1324896 in parsePrattCall (cursed_compiler_main.zig)
            .function = try self.allocateExpression(left),
                                                   ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.W3XBEm0bdV	2025-08-31 11:35:06.474691947 +0300
+++ /tmp/tmp.SZdhUo1Dsc	2025-08-31 11:35:06.474691947 +0300
@@ -1,208 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:5:25 - Error parsing function statement
-INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:23:9 - Failed to parse statement
-INFO: Recovered at delimiter 'RightParen' after skipping 5 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:32:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:32:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:32:39 - Failed to parse statement
-INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Synchronizing parser after error (context: synchronize)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe5feb4308 (parent: *interpreter.Environment@0)
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
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'a'
-DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5feb4308
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'b'
-DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5feb4308
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'b'
-DEBUG: Found 'b' in environment@interpreter.Environment@7ffe5feb4308
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'c'
-DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5feb4308
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'a'
-DEBUG: Found 'a' in environment@interpreter.Environment@7ffe5feb4308
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 4 variables for 'c'
-DEBUG: Found 'c' in environment@interpreter.Environment@7ffe5feb4308
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffe5feb4308 with 5 variables for 'final'
-DEBUG: Found 'final' in environment@interpreter.Environment@7ffe5feb4308
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
-/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1268:61: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            const init_ptr = try self.arena_allocator.create(Expression);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:409:52: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-            .function = try self.allocateExpression(left),
-                                                   ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:5:16 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 6
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd2451aea8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.zASbszR7fO	2025-08-31 11:35:06.828425024 +0300
+++ /tmp/tmp.TJKIQplAnx	2025-08-31 11:35:06.828425024 +0300
@@ -1,17 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:5:16 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:28:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd2451aea8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_if_statements
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:6:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 7 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:37:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 11
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdd4e209d8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x78acc8960000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x78acc89a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0x12170f8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0x11d58c9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x78acc89e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.qvXs563gDZ	2025-08-31 11:35:07.507255764 +0300
+++ /tmp/tmp.ZvTQmC2cHs	2025-08-31 11:35:07.507255764 +0300
@@ -1,80 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:6:44 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 7 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.💀:37:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdd4e209d8 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:6:37 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:24:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd81f07258 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x78c239d20000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x78c239da0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0x12170f8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0x11d58c9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x78c239d40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.DjAAD0uG8S	2025-08-31 11:35:08.179252950 +0300
+++ /tmp/tmp.EeJkQsy2LO	2025-08-31 11:35:08.179252950 +0300
@@ -1,78 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:6:37 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.💀:24:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd81f07258 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdef1de8e8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@776a73360000 with parent@*interpreter.Environment@7ffdef1de8e8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffdef1dc1e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
error(gpa): memory address 0x776a73360000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x123972a in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2639:60: 0x1239c50 in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f23b7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c73f3 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x11a402c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x776a7ac60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x776a7ac60100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a7ac60780 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x776a733e0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0x12517b1 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1324de6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776a733c0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x132465b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0x124e643 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x776a7ac40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x776a73380000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0x12517b1 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:362:51: 0x132419f in parsePrattUnary (cursed_compiler_main.zig)
            .operand = try self.allocateExpression(right),
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:1653:35: 0x128b901 in parseExpressionPrattPrec (cursed_compiler_main.zig)
        var left = try prefix_fn.?(self);
                                  ^

error(gpa): memory address 0x776a733a2000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x132465b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0x124e643 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.PHEM0oRRE7	2025-08-31 11:35:08.998249521 +0300
+++ /tmp/tmp.zbQUiS0VC9	2025-08-31 11:35:08.999249517 +0300
@@ -1,508 +0,0 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdef1de8e8 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@776a73360000 with parent@*interpreter.Environment@7ffdef1de8e8
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffdef1dc1e8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4144:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:362:51: 0xADDRESS in parsePrattUnary (cursed_compiler_main.zig)
-            .operand = try self.allocateExpression(right),
-                                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:1653:35: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:5:20 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 0
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcfefcb1b8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.mWofJIkDfB	2025-08-31 11:35:09.361248001 +0300
+++ /tmp/tmp.VGyq3GAEwX	2025-08-31 11:35:09.361248001 +0300
@@ -1,15 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:5:20 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:27:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcfefcb1b8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_division_by_zero
**Status:** FAIL
**Details:** Interpreter failed but compiled mode succeeded

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc2b483118 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@7b27fade0000 with parent@*interpreter.Environment@7ffc2b483118
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffc2b480a18, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
thread 1711147 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:1487:39: 0x12a5fdd in evaluateBinary (cursed_compiler_main.zig)
                if (right_num == 0.0) return InterpreterError.DivisionByZero;
                                      ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1417:37: 0x126ea61 in evaluateExpression (cursed_compiler_main.zig)
            .Binary => |bin| return try self.evaluateBinary(bin),
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1588:37: 0x12ada04 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x126ed00 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123b94f in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x1314f9c in sliceAsBytes__anon_46289 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12ce634 in free__anon_42358 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1270cc9 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123ad0d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f23b7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c73f3 in interpret (cursed_compiler_main.zig)
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7b2802a2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7b2802a2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1503f94 in ??? (???)
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
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.💀:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.💀:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.💀:6:48 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.💀:15:1 - Failed to parse statement
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
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe2be49d58 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@781c9fd80000 with parent@*interpreter.Environment@7ffe2be49d58
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffe2be47658, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@781c9fd80000 with 0 variables for 'undefined_var'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffe2be49d58 with 1 variables for 'undefined_var'
DEBUG: Variable 'undefined_var' not found in any environment after 2 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@781c9fd80000 with 0 variables for 'self'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffe2be49d58 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 2 hops
DEBUG: Variable 'undefined_var' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/undefined_var/mod.💀: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'undefined_var': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/undefined_var/mod.💀 for true self-hosting
DEBUG: Failed to load module 'undefined_var'
thread 1711173 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0x12a449e in get (cursed_compiler_main.zig)
        return InterpreterError.UndefinedVariable;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0x126e97a in evaluateExpression (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedVariable;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1588:37: 0x12ada04 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x126ed00 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123b94f in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x1314f9c in sliceAsBytes__anon_46289 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12ce634 in free__anon_42358 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1270cc9 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123ad0d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f23b7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c73f3 in interpret (cursed_compiler_main.zig)
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x781ca782a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x781ca782a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1503f94 in ??? (???)
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
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.💀:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.💀:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.💀:6:50 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.💀:12:1 - Failed to parse statement
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
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:5:19 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 0
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd71fd5e48 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.dnULO7HTRn	2025-08-31 11:35:15.752221250 +0300
+++ /tmp/tmp.t1ZKoOa2uF	2025-08-31 11:35:15.752221250 +0300
@@ -1,15 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:5:19 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:25:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd71fd5e48 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_recursive_function
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:5:17 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:31:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 1
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffff5def198 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.ObBdhwXkMV	2025-08-31 11:35:16.108601413 +0300
+++ /tmp/tmp.80uKMXxfiD	2025-08-31 11:35:16.108601413 +0300
@@ -1,17 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:5:17 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:31:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffff5def198 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 03_nested_function_calls
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.💀:5:20 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.💀:33:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff6fb14a28 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.4vZx2IlGVi	2025-08-31 11:35:16.463218274 +0300
+++ /tmp/tmp.N2ZVqPkZzN	2025-08-31 11:35:16.463218274 +0300
@@ -1,17 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.💀:5:20 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.💀:33:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff6fb14a28 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 04_function_parameters
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/04_function_parameters.💀:9:17 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/04_function_parameters.💀:33:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 6
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc746ec6b8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'no_params'
DEBUG: Executing statement type: Function
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x73afab780000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x73afab780080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x73afab7c0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2655:62: 0x120607a in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:989:49: 0x11d43fb in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x73afb3020800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../04_function_parameters
```

---

## Test: 01_corrected_hello_world
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd8eb035f8 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Let
DEBUG: Executing statement type: Block
DEBUG: Executing block with 4 statements
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Hello, CURSED World!"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
DEBUG: Executing statement type: Return
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7fcf8b9c0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7fcf8b9c0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7fcf8b9c0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1130:34: 0x1202400 in parseBlockStatement (cursed_compiler_main.zig)
            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0x11d2dde in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^

error(gpa): memory address 0x7fcf93240400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1288830 in create__anon_39381 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1502:66: 0x1249eda in parseType (cursed_compiler_main.zig)
                    return_type = try self.arena_allocator.create(ast.Type);
                                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1261:51: 0x1204d8e in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0x11d345d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^

error(gpa): memory address 0x7fcf8b9e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7fcf93280800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1126:61: 0x1202146 in parseBlockStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0x11d2dde in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.WPDqYeu4n7	2025-08-31 11:35:17.772212797 +0300
+++ /tmp/tmp.6JdBqweJY0	2025-08-31 11:35:17.772212797 +0300
@@ -1,138 +0,0 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd8eb035f8 (parent: *interpreter.Environment@0)
-DEBUG: Executing statement type: Let
-DEBUG: Executing statement type: Block
-DEBUG: Executing block with 4 statements
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Hello, CURSED World!"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Literal
-Unsupported expression type in interpreter: Literal
-DEBUG: Executing statement type: Return
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Literal
-Unsupported expression type in interpreter: Literal
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
-/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1130:34: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseBlockStatement();
-                                               ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_39381 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1502:66: 0xADDRESS in parseType (cursed_compiler_main.zig)
-                    return_type = try self.arena_allocator.create(ast.Type);
-                                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1261:51: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            let_stmt.var_type = try self.parseType();
-                                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
-                                                           ^
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
-/home/ghuntley/cursed/src-zig/parser.zig:1126:61: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseBlockStatement();
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 02_corrected_arithmetic
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc81b61548 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Let
DEBUG: Executing statement type: Block
DEBUG: Executing block with 10 statements
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Integer
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Integer
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc81b61548 with 4 variables for 'x'
DEBUG: Found 'x' in environment@interpreter.Environment@7ffc81b61548
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc81b61548 with 4 variables for 'y'
DEBUG: Found 'y' in environment@interpreter.Environment@7ffc81b61548
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Call
DEBUG: Evaluating expression type: String
"Arithmetic test complete"
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
DEBUG: Executing statement type: Return
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x750f1e4a0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x750f1e4a0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1130:34: 0x1202400 in parseBlockStatement (cursed_compiler_main.zig)
            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0x11d2dde in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^

error(gpa): memory address 0x750f1e4a0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x750f1e520200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1288830 in create__anon_39381 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1502:66: 0x1249eda in parseType (cursed_compiler_main.zig)
                    return_type = try self.arena_allocator.create(ast.Type);
                                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1261:51: 0x1204d8e in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0x11d345d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^

error(gpa): memory address 0x750f1e4c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x750f1e4e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1268:61: 0x1205080 in parseLetStatement (cursed_compiler_main.zig)
            const init_ptr = try self.arena_allocator.create(Expression);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0x11d345d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1125:49: 0x1201fe5 in parseBlockStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x750f1e1e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1126:61: 0x1202146 in parseBlockStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0x11d2dde in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.LnuYMN50CY	2025-08-31 11:35:18.459209922 +0300
+++ /tmp/tmp.SMy5f2NkVg	2025-08-31 11:35:18.460209918 +0300
@@ -1,179 +0,0 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc81b61548 (parent: *interpreter.Environment@0)
-DEBUG: Executing statement type: Let
-DEBUG: Executing statement type: Block
-DEBUG: Executing block with 10 statements
-DEBUG: Executing statement type: Let
-DEBUG: Evaluating expression type: Integer
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Literal
-Unsupported expression type in interpreter: Literal
-DEBUG: Executing statement type: Let
-DEBUG: Evaluating expression type: Integer
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Literal
-Unsupported expression type in interpreter: Literal
-DEBUG: Executing statement type: Let
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc81b61548 with 4 variables for 'x'
-DEBUG: Found 'x' in environment@interpreter.Environment@7ffc81b61548
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc81b61548 with 4 variables for 'y'
-DEBUG: Found 'y' in environment@interpreter.Environment@7ffc81b61548
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Literal
-Unsupported expression type in interpreter: Literal
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Call
-DEBUG: Evaluating expression type: String
-"Arithmetic test complete"
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Literal
-Unsupported expression type in interpreter: Literal
-DEBUG: Executing statement type: Return
-DEBUG: Executing statement type: Expression
-DEBUG: Evaluating expression type: Literal
-Unsupported expression type in interpreter: Literal
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
-/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1130:34: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseBlockStatement();
-                                               ^
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
-/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_39381 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1502:66: 0xADDRESS in parseType (cursed_compiler_main.zig)
-                    return_type = try self.arena_allocator.create(ast.Type);
-                                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1261:51: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            let_stmt.var_type = try self.parseType();
-                                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
-                                                           ^
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1268:61: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            const init_ptr = try self.arena_allocator.create(Expression);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
-                                                           ^
-/home/ghuntley/cursed/src-zig/parser.zig:1125:49: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1126:61: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseBlockStatement();
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 03_corrected_function_parameters
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:9:17 - Expected ')'. Expected .RightParen, got .Normie (context: consume)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:9:17 - Error parsing variable declaration
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:9:23 - Failed to parse statement
INFO: Recovered at semicolon after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:13:23 - Expected ')'. Expected .RightParen, got .Normie (context: consume)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:13:23 - Error parsing variable declaration
INFO: Recovered at delimiter 'RightParen' after skipping 7 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:13:49 - Failed to parse statement
INFO: Recovered at statement keyword 'Sus' after skipping 4 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:16:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:16:1 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:18:1 - Failed to parse statement
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:25:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:25:1 - Synchronizing parser after error (context: synchronize)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 4
Semicolon recoveries: 8
Statement recoveries: 4
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 23
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff91df7ab8 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Let
DEBUG: Executing statement type: Block
DEBUG: Executing block with 2 statements
DEBUG: Executing statement type: Return
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Literal
Unsupported expression type in interpreter: Literal
DEBUG: Executing statement type: Let
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fff91df7ab8 with 2 variables for 'a'
DEBUG: Variable 'a' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fff91df7ab8 with 2 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'a' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/a/mod.💀: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'a': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/a/mod.💀 for true self-hosting
DEBUG: Failed to load module 'a'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7f0349760000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7f0349760080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1130:34: 0x1202400 in parseBlockStatement (cursed_compiler_main.zig)
            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:937:48: 0x11d2dde in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^

error(gpa): memory address 0x7f0349760100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1321d2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e3c45 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128b362 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124d5c5 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1492:47: 0x1249d4a in parseType (cursed_compiler_main.zig)
                        try param_types.append(self.allocator, param_type);
                                              ^
/home/ghuntley/cursed/src-zig/parser.zig:1261:51: 0x1204d8e in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^

error(gpa): memory address 0x7f0349760180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1321d2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e3c45 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128b362 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124d5c5 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1492:47: 0x1249d4a in parseType (cursed_compiler_main.zig)
                        try param_types.append(self.allocator, param_type);
                                              ^
/home/ghuntley/cursed/src-zig/parser.zig:1261:51: 0x1204d8e in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^

error(gpa): memory address 0x7f0349760200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7f0349760280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7f0349760300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x132be2e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12e8855 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x128de92 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1252f9c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x132475e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7f03497a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1288830 in create__anon_39381 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1502:66: 0x1249eda in parseType (cursed_compiler_main.zig)
                    return_type = try self.arena_allocator.create(ast.Type);
                                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1261:51: 0x1204d8e in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:955:60: 0x11d345d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^

error(gpa): memory address 0x7f03497e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7f0349720000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7f0349700000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x132465b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1660:34: 0x128bb3b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1643:45: 0x124e643 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_corrected_function_parameters
```

---

## Test: 01_recursive_depth
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.💀:5:22 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.💀:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 0
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffab6518a8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.X4EOD9tfi2	2025-08-31 11:35:19.504205550 +0300
+++ /tmp/tmp.Jb7KehHUZF	2025-08-31 11:35:19.504205550 +0300
@@ -1,15 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.💀:5:22 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.💀:25:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffab6518a8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_computation_intensive
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.💀:5:17 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.💀:33:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 2
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 6
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd3fce6e88 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.H70PP4QLyy	2025-08-31 11:35:19.867478748 +0300
+++ /tmp/tmp.JDUNS5kWSq	2025-08-31 11:35:19.867478748 +0300
@@ -1,17 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.💀:5:17 - Error parsing function statement
-INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.💀:33:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd3fce6e88 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_mathz_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:8:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:8:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:8:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc1c45b508 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc1c45b508 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc1c45b508 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.💀: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.💀 for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x769a91720000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x769a91720080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x769a91760200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x769a917a0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x769a916e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:8:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:8:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:8:46 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffebb95bb28 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffebb95bb28 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffebb95bb28 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.💀: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.💀 for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7f680f900000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7f680f900080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7f680f940200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7f680f980800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7f680f8c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
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
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffca08592d8 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'main_character'
DEBUG: Created new environment@interpreter.Environment@74db86d80000 with parent@*interpreter.Environment@7ffca08592d8
DEBUG: Calling function with 0 parameters, got 0 args
DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffca0856bd8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@74db86d80000 with 0 variables for 'mathz'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffca08592d8 with 1 variables for 'mathz'
DEBUG: Variable 'mathz' not found in any environment after 2 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@74db86d80000 with 0 variables for 'self'
DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffca08592d8 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 2 hops
DEBUG: Variable 'mathz' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/mathz/mod.💀: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/mathz/mod.💀 for true self-hosting
DEBUG: Failed to load module 'mathz'
thread 1711439 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0x12a449e in get (cursed_compiler_main.zig)
        return InterpreterError.UndefinedVariable;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0x126e97a in evaluateExpression (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedVariable;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2072:24: 0x12ca3d9 in evaluateMethodCall (cursed_compiler_main.zig)
        const object = try self.evaluateExpression(member.object.*);
                       ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1602:28: 0x12adf82 in evaluateCall (cursed_compiler_main.zig)
                    return try self.evaluateMethodCall(member.*, call.arguments.items);
                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x126ed00 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1588:37: 0x12ada04 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1419:36: 0x126ed00 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:680:21: 0x123b94f in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x1314f9c in sliceAsBytes__anon_46289 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x12ce634 in free__anon_42358 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x1270cc9 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2671:35: 0x123ad0d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:666:38: 0x11f23b7 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:629:28: 0x11c73f3 in interpret (cursed_compiler_main.zig)
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x74db8e82a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x74db8e82a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1503f94 in ??? (???)
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
--- /tmp/tmp.ju3Air7ov7	2025-08-31 11:35:24.238185746 +0300
+++ /tmp/tmp.8BA3PC7paz	2025-08-31 11:35:24.239185742 +0300
@@ -1,90 +1 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffca08592d8 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@74db86d80000 with parent@*interpreter.Environment@7ffca08592d8
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7ffca0856bd8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@74db86d80000 with 0 variables for 'mathz'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffca08592d8 with 1 variables for 'mathz'
-DEBUG: Variable 'mathz' not found in any environment after 2 hops
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@74db86d80000 with 0 variables for 'self'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7ffca08592d8 with 1 variables for 'self'
-DEBUG: Variable 'self' not found in any environment after 2 hops
-DEBUG: Variable 'mathz' not found, attempting lazy module loading...
-DEBUG: Could not open CURSED stdlib file stdlib/mathz/mod.💀: error.FileNotFound
-ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/mathz/mod.💀 for true self-hosting
-DEBUG: Failed to load module 'mathz'
-thread 1711439 panic: integer overflow
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
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_46289 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_42358 (std.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:8:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:8:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:8:49 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 10
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff4d5eaa18 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fff4d5eaa18 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fff4d5eaa18 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.💀: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.💀 for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x768361d80000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1287c0e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247355 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201cc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2acc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:510:42: 0x11b22f9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x768361d80080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x768361dc0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1096:53: 0x11d5b82 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x768369640800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e62 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x768361d40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:6:49 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:29:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffeb4287058 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x77717af60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125c2ce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121dad5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbd32 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b69dc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0x1203d14 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x77717afa0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205740 in create__anon_30705 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0x12170f8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0x11d58c9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0x1203adb in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x77717afe0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d2950 in create__anon_26064 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0x1203ba9 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0x11d2ed5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d94 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.BjO9RsWog9	2025-08-31 11:35:25.545180280 +0300
+++ /tmp/tmp.S9KQdlYNva	2025-08-31 11:35:25.545180280 +0300
@@ -1,78 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:6:49 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.💀:29:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffeb4287058 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1228:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30705 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3790:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1084:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1218:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1219:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:942:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

