# CURSED Interpreter vs Compiler Parity Test Report

**Generated:** Sun Aug 31 11:51:49 AM EEST 2025
**Test Suite Version:** 1.0.0
**CURSED Compiler:** /home/ghuntley/cursed/test_suite/../zig-out/bin/cursed-compiler

## Executive Summary

- **Total Tests:** 29
- **Passed:** 0
- **Failed:** 20
- **Compile Errors:** 9
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

- 🚨 **High Priority:** Fix compilation failures (9 tests)
- 📋 **Low Priority:** Investigate output differences (20 tests)

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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x713844cc0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x713844d00200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0x1211d68 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0x11cfba9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x713844d40800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.NNsrWTr17D	2025-08-31 11:51:19.254125551 +0300
+++ /tmp/tmp.vSj2R8qGNs	2025-08-31 11:51:19.255125543 +0300
@@ -1,76 +0,0 @@
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
"=== Arithmetic Edge Cases Test ==="
"Zero operations:"
5
0
0
"Negative numbers:"
-2
-6
-6
"Large numbers:"
3000
999998
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7a1064de0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7a1065100000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a1065100100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065100700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7a1065140200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1313ba6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7a1065180800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x131341b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0x1249723 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x7a10650c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a10650a0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a10650e2000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.KAke2d7cgJ	2025-08-31 11:51:19.926120305 +0300
+++ /tmp/tmp.hPRqfleyjY	2025-08-31 11:51:19.926120305 +0300
@@ -1,436 +0,0 @@
-"=== Arithmetic Edge Cases Test ==="
-"Zero operations:"
-5
-0
-0
-"Negative numbers:"
--2
--6
--6
-"Large numbers:"
-3000
-999998
-"=== Test Complete ==="
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
"=== Operator Precedence Test ==="
"2 + 3 * 4 should be 14:"
14
"(2 + 3) * 4 should be 20:"
20
"10 - 3 + 2 should be 9:"
9
"8 / 2 * 3 should be 12:"
12
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x708e4ff40000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x708e4ffc0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x708e4ffc0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e4ffc0580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x708e57840200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1313ba6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x708e57880800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x131341b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0x1249723 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x708e4ff80000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x708e4ff60000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x708e4ffa2000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.64RozgZoMf	2025-08-31 11:51:20.578115219 +0300
+++ /tmp/tmp.4PuvIL7DjQ	2025-08-31 11:51:20.578115219 +0300
@@ -1,373 +0,0 @@
-"=== Operator Precedence Test ==="
-"2 + 3 * 4 should be 14:"
-14
-"(2 + 3) * 4 should be 20:"
-20
-"10 - 3 + 2 should be 9:"
-9
-"8 / 2 * 3 should be 12:"
-12
-"=== Test Complete ==="
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7943aa7c0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7943b2040200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0x1211d68 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0x11cfba9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7943b2080800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.Uz81SWvJKq	2025-08-31 11:51:21.128110930 +0300
+++ /tmp/tmp.DiTg4PfGmD	2025-08-31 11:51:21.128110930 +0300
@@ -1,78 +0,0 @@
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
"Hello, CURSED World!"
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x753e5cfc0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x753e648a0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x753e648a0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x753e648a0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x753e64860400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1313ba6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x753e64840400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x131341b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0x1249723 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.i4nJLrQJaT	2025-08-31 11:51:21.690106549 +0300
+++ /tmp/tmp.0nPkKLuqKv	2025-08-31 11:51:21.690106549 +0300
@@ -1,124 +0,0 @@
-"Hello, CURSED World!"
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7d62c5d60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d62c5da0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0x1211d68 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0x11cfba9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7d62c5de0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.bKcP0ahgp2	2025-08-31 11:51:22.236102296 +0300
+++ /tmp/tmp.PJgQBN5ejB	2025-08-31 11:51:22.237102288 +0300
@@ -1,76 +0,0 @@
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7a116da60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a116dae0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0x1211d68 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0x11cfba9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7a116da80000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.Qbmwr0mr2n	2025-08-31 11:51:22.785098020 +0300
+++ /tmp/tmp.Oghj43zMLa	2025-08-31 11:51:22.786098013 +0300
@@ -1,76 +0,0 @@
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

## Test: 01_nested_operations
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:10:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:10:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:10:49 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:33:5 - Failed to parse statement
INFO: Recovered at statement keyword 'Sus' after skipping 5 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Synchronizing parser after error (context: synchronize)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀:40:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 3
Semicolon recoveries: 5
Statement recoveries: 3
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 16
================================
ERROR: No CURSED stdlib implementation found for module 'a': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/a/mod.💀 for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.💀: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x719847100000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x719847100080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x719847100100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x719847100180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x719847100200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7198471a0300 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:374:48: 0x13144d2 in parsePrattBinary (cursed_compiler_main.zig)
            .left = try self.allocateExpression(left),
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x719847180000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x719847120000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:374:48: 0x13144d2 in parsePrattBinary (cursed_compiler_main.zig)
            .left = try self.allocateExpression(left),
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7198471e1000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x7198470e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:376:49: 0x1314590 in parsePrattBinary (cursed_compiler_main.zig)
            .right = try self.allocateExpression(right),
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_nested_operations
```

### Output Diff:
```diff
--- /tmp/tmp.pQzFQswjjk	2025-08-31 11:45:03.424794524 +0300
+++ /tmp/tmp.dK8an9626r	2025-08-31 11:45:03.424794524 +0300
@@ -1,168 +0,0 @@
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
-12
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30678 (std.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30678 (std.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:10:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:10:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:10:16 - Error parsing function statement
INFO: Recovered at statement keyword 'Slay' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:21:1 - Failed to parse statement
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
INFO: Attempting additional statement recovery

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x766ce94e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x766ce94e0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x766ce94e0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x766ce94e0180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:2699:35: 0x1201581 in parseIfStatement (cursed_compiler_main.zig)
            try then_branch.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:996:62: 0x11ce810 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .If = try self.parseIfStatement() };
                                                             ^

error(gpa): memory address 0x766ce94e0200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:2774:108: 0x1202da1 in parseWhileStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); stmt_ptr.* = stmt; try body.append(self.allocator, stmt_ptr);
                                                                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1001:68: 0x11ce9d1 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .While = try self.parseWhileStatement() };
                                                                   ^

error(gpa): memory address 0x766ce9520200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1270:61: 0x11ffcf0 in parseLetStatement (cursed_compiler_main.zig)
            const init_ptr = try self.arena_allocator.create(Expression);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0x11cd73d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x766ce9560800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x766ce94a0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1313ba6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.u1v9ZsEIvy	2025-08-31 11:51:23.935089071 +0300
+++ /tmp/tmp.r9e0TE0RL6	2025-08-31 11:51:23.935089071 +0300
@@ -1,180 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:10:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:10:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:10:16 - Error parsing function statement
-INFO: Recovered at statement keyword 'Slay' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.💀:21:1 - Failed to parse statement
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-INFO: Attempting additional statement recovery
-
-=== Error Recovery Statistics ===
-Total errors encountered: 2
-Semicolon recoveries: 3
-Statement recoveries: 2
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 5
-================================
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
-/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-                try func.parameters.append(self.allocator, param);
-                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:2699:35: 0xADDRESS in parseIfStatement (cursed_compiler_main.zig)
-            try then_branch.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
-                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:996:62: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .If = try self.parseIfStatement() };
-                                                             ^
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
-/home/ghuntley/cursed/src-zig/parser.zig:2774:108: 0xADDRESS in parseWhileStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); stmt_ptr.* = stmt; try body.append(self.allocator, stmt_ptr);
-                                                                                                           ^
-/home/ghuntley/cursed/src-zig/parser.zig:1001:68: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .While = try self.parseWhileStatement() };
-                                                                   ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0xADDRESS in alloc (std.zig)
-            (self.createNode(0, n + ptr_align) orelse return null);
-                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0xADDRESS in allocBytesWithAlignment__anon_8545 (std.zig)
-    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
-                         ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1270:61: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            const init_ptr = try self.arena_allocator.create(Expression);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
-                                                           ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-
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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7aa3e39c0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7aa3eb240200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0x1211d68 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0x11cfba9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7aa3eb280800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.uw5enWuk4J	2025-08-31 11:51:24.485084795 +0300
+++ /tmp/tmp.OVr2NDkZ5T	2025-08-31 11:51:24.485084795 +0300
@@ -1,78 +0,0 @@
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7c29f1540000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7c29f15c0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0x1211d68 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0x11cfba9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7c29f1560000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.70rOh5lMca	2025-08-31 11:51:25.033080535 +0300
+++ /tmp/tmp.aL5suCW7r2	2025-08-31 11:51:25.034080527 +0300
@@ -1,76 +0,0 @@
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
"=== Boundary Values Test ==="
"Small numbers:"
1
0
-1
"Large numbers:"
999999
1000000
-999999
"Decimal boundaries:"
0.1
0
-0.1
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7cd3a4380000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7cd3a43e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7cd3a43e0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3a43e0780 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7cd3abc60200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0x1313ba6 in parsePrattMemberAccess (cursed_compiler_main.zig)
            .object = try self.allocateExpression(left),
                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7cd3abca0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x131341b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0x1249723 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

error(gpa): memory address 0x7cd3a43c0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7cd3a43a0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:362:51: 0x1312f5f in parsePrattUnary (cursed_compiler_main.zig)
            .operand = try self.allocateExpression(right),
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:1658:35: 0x1285631 in parseExpressionPrattPrec (cursed_compiler_main.zig)
        var left = try prefix_fn.?(self);
                                  ^

error(gpa): memory address 0x7cd3abc42000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x131341b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0x1249723 in parseExpressionPratt (cursed_compiler_main.zig)
        return self.parseExpressionPrattPrec(.None);
                                            ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.BUJeYmVZDJ	2025-08-31 11:51:25.715075237 +0300
+++ /tmp/tmp.q5ql2T44Jq	2025-08-31 11:51:25.715075237 +0300
@@ -1,457 +0,0 @@
-"=== Boundary Values Test ==="
-"Small numbers:"
-1
-0
--1
-"Large numbers:"
-999999
-1000000
--999999
-"Decimal boundaries:"
-0.1
-0
--0.1
-"=== Test Complete ==="
-Executing defers from size 0 to 0
-✅ Program completed
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0xADDRESS in newEnvironment (cursed_compiler_main.zig)
-        const env = try allocator.create(Environment);
-                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
-                                                           ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:422:50: 0xADDRESS in parsePrattMemberAccess (cursed_compiler_main.zig)
-            .object = try self.allocateExpression(left),
-                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-        const expr_ptr = self.arena_allocator.create(Expression) catch {
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt = try self.parseStatement();
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0xADDRESS in allocateExpression (cursed_compiler_main.zig)
-        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:362:51: 0xADDRESS in parsePrattUnary (cursed_compiler_main.zig)
-            .operand = try self.allocateExpression(right),
-                                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:1658:35: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0xADDRESS in parsePrattCall (cursed_compiler_main.zig)
-                const arg_ptr = try self.arena_allocator.create(Expression);
-                                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
-            left = try infix_fn.?(self, left);
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0xADDRESS in parseExpressionPratt (cursed_compiler_main.zig)
-        return self.parseExpressionPrattPrec(.None);
-                                            ^
-
```

---

## Test: 02_empty_inputs
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:13:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:13:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:13:44 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.💀:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x703a7ff80000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x703a7ff80080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:2699:35: 0x1201581 in parseIfStatement (cursed_compiler_main.zig)
            try then_branch.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:996:62: 0x11ce810 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .If = try self.parseIfStatement() };
                                                             ^

error(gpa): memory address 0x703a7ff80100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x703a7ff80180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x703a7ff80200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x703a7ffc0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:374:48: 0x13144d2 in parsePrattBinary (cursed_compiler_main.zig)
            .left = try self.allocateExpression(left),
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x703a87840800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2660:62: 0x1200cea in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:991:49: 0x11ce6db in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:2695:49: 0x120135e in parseIfStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x703a7ff40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_empty_inputs
```

### Output Diff:
```diff
--- /tmp/tmp.E4A63qK3h6	2025-08-31 11:45:05.761784962 +0300
+++ /tmp/tmp.i18KzDBEWW	2025-08-31 11:45:05.761784962 +0300
@@ -1,13 +0,0 @@
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_division_by_zero
**Status:** FAIL
**Details:** Interpreter failed but compiled mode succeeded

### Interpreter Output:
```
"=== Division by Zero Test ==="
"Normal division:"
5
"Attempting division by zero:"
thread 1720056 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:1486:39: 0x129ce2d in evaluateBinary (cursed_compiler_main.zig)
                if (right_num == 0.0) return InterpreterError.DivisionByZero;
                                      ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1416:37: 0x1267948 in evaluateExpression (cursed_compiler_main.zig)
            .Binary => |bin| return try self.evaluateBinary(bin),
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:37: 0x12a4854 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:36: 0x1267be9 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:679:21: 0x1234e04 in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x12755cc in sliceAsBytes__anon_38537 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x1236ee4 in free__anon_33739 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x11ee069 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0x123430d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a0d9d in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7b2d2fc2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7b2d2fc2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
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
"=== Undefined Variable Test ==="
"Using undefined variable:"
ERROR: No CURSED stdlib implementation found for module 'undefined_var': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/undefined_var/mod.💀 for true self-hosting
thread 1720081 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:465:9: 0x129b914 in get (cursed_compiler_main.zig)
        return InterpreterError.UndefinedVariable;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1413:21: 0x1267860 in evaluateExpression (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedVariable;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:37: 0x12a4854 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:36: 0x1267be9 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:679:21: 0x1234e04 in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x12755cc in sliceAsBytes__anon_38537 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x1236ee4 in free__anon_33739 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x11ee069 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0x123430d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a0d9d in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7253acc2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7253acc2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:15:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:15:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:15:47 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.💀:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x776b89180000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x776b89180080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x776b89180100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x776b89180180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776b89180200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x776b89180280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776b89180300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x776b891a0200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x776b891a0300 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:374:48: 0x13144d2 in parsePrattBinary (cursed_compiler_main.zig)
            .left = try self.allocateExpression(left),
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x776b891e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2660:62: 0x1200cea in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:991:49: 0x11ce6db in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x776b89120000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:409:52: 0x1313656 in parsePrattCall (cursed_compiler_main.zig)
            .function = try self.allocateExpression(left),
                                                   ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_simple_function
```

### Output Diff:
```diff
--- /tmp/tmp.080ckvFqzx	2025-08-31 11:45:11.854760036 +0300
+++ /tmp/tmp.x0wgrfrAWy	2025-08-31 11:45:11.855760032 +0300
@@ -1,13 +0,0 @@
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_recursive_function
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:20:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:20:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:20:50 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.💀:31:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x72976a160000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x72976a160080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:2699:35: 0x1201581 in parseIfStatement (cursed_compiler_main.zig)
            try then_branch.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:996:62: 0x11ce810 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .If = try self.parseIfStatement() };
                                                             ^

error(gpa): memory address 0x72976a160100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x72976a160180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x72976a160200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x72976a160280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x72976a160300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x72976a160380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:2699:35: 0x1201581 in parseIfStatement (cursed_compiler_main.zig)
            try then_branch.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:996:62: 0x11ce810 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .If = try self.parseIfStatement() };
                                                             ^

error(gpa): memory address 0x72976a160400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x72976a160480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x72976a160500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x72976a1a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:374:48: 0x13144d2 in parsePrattBinary (cursed_compiler_main.zig)
            .left = try self.allocateExpression(left),
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x72976a1e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2660:62: 0x1200cea in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:991:49: 0x11ce6db in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:2695:49: 0x120135e in parseIfStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x72976a120000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x131341b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:371:56: 0x1314447 in parsePrattBinary (cursed_compiler_main.zig)
        const right = try self.parseExpressionPrattPrec(precedence);
                                                       ^

error(gpa): memory address 0x72976a100000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2696:61: 0x1201417 in parseIfStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:996:62: 0x11ce810 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .If = try self.parseIfStatement() };
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_recursive_function
```

### Output Diff:
```diff
--- /tmp/tmp.kK7qrFHBuD	2025-08-31 11:45:12.148758834 +0300
+++ /tmp/tmp.MzIVEwqQ6j	2025-08-31 11:45:12.148758834 +0300
@@ -1,15 +0,0 @@
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 03_nested_function_calls
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
"=== Nested Function Calls Test ==="
"double_value(5):"
Executing defers from size 0 to 0
10
"add_one(7):"
Executing defers from size 0 to 0
8
"complex_calc(3) = double_value(add_one(3)):"
Executing defers from size 0 to 0
Executing defers from size 0 to 0
Executing defers from size 0 to 0
8
"Nested in expression:"
Executing defers from size 0 to 0
Executing defers from size 0 to 0
7
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7989d1160000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7989d1160080 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x7989d11600c0 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x7989d1160140 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x7989d1160180 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1997:68: 0x12b0e80 in evaluateCall (cursed_compiler_main.zig)
                            const arg = try self.evaluateExpression(arg_expr.*);
                                                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x7989d11601c0 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2670:67: 0x1233e88 in callFunction (cursed_compiler_main.zig)
                        const result = try self.evaluateExpression(expr.*);
                                                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^

error(gpa): memory address 0x7989d1160200 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1997:68: 0x12b0e80 in evaluateCall (cursed_compiler_main.zig)
                            const arg = try self.evaluateExpression(arg_expr.*);
                                                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x7989d1160240 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1455:49: 0x129bc3e in evaluateBinary (cursed_compiler_main.zig)
        const left = try self.evaluateExpression(bin.left.*);
                                                ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1416:60: 0x1267924 in evaluateExpression (cursed_compiler_main.zig)
            .Binary => |bin| return try self.evaluateBinary(bin),
                                                           ^

error(gpa): memory address 0x7989d8a60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d8a60080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d8a60100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7989d8a60180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d8a60200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d8a60280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d8a60300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d8a60480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d8a60580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60780 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60800 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60880 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60900 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60980 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60a00 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60a80 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60b00 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60b80 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d8a60c00 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d11e0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:374:48: 0x13144d2 in parsePrattBinary (cursed_compiler_main.zig)
            .left = try self.allocateExpression(left),
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7989d11c0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2660:62: 0x1200cea in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:991:49: 0x11ce6db in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7989d8a40000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x7989d1180000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7989d1183000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x7989d1184000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x7989d1185000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x7989d1186000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x7989d1187000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x7989d1188000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x7989d1189000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x7989d11a2000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:4149:48: 0x124c891 in allocateExpression (cursed_compiler_main.zig)
        const ptr = self.arena_allocator.create(Expression) catch return ParserError.OutOfMemory;
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:409:52: 0x1313656 in parsePrattCall (cursed_compiler_main.zig)
            .function = try self.allocateExpression(left),
                                                   ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_nested_function_calls
```

### Output Diff:
```diff
--- /tmp/tmp.yEWs1ASOt9	2025-08-31 11:45:12.444757623 +0300
+++ /tmp/tmp.RgZAn8UglI	2025-08-31 11:45:12.445757619 +0300
@@ -1,15 +0,0 @@
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 04_function_parameters
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
"=== Function Parameters Test ==="
"No parameters:"
Executing defers from size 0 to 0
42
"One parameter:"
Executing defers from size 0 to 0
20
"Multiple parameters:"
Executing defers from size 0 to 0
6
"Parameters with expressions:"
Executing defers from size 0 to 0
20
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x73416b5c0000 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x73416b5c0080 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x73416b5c00c0 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x73416b5c0100 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x73416b5c0140 leaked: 
/home/ghuntley/cursed/src-zig/interpreter.zig:425:41: 0x12330ba in newEnvironment (cursed_compiler_main.zig)
        const env = try allocator.create(Environment);
                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2631:60: 0x12334be in callFunction (cursed_compiler_main.zig)
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
                                                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^

error(gpa): memory address 0x734172ea0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x734172ea0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x734172ea0100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x734172ea0180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x734172ea0280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x734172ea0300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0380 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x734172ea0400 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0480 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0500 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0580 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0600 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0680 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0700 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0780 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0800 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0880 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0900 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ea0980 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x734172ee0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2660:62: 0x1200cea in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:991:49: 0x11ce6db in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x734172e40000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1311d4e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9cf5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285412 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248ff1 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1197:43: 0x11fe3d9 in parseFunctionStatement (cursed_compiler_main.zig)
                try func.parameters.append(self.allocator, param);
                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x734172f20800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x734172e60000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x73416b5e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x73416b5e3000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x73416b5e4000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x73416b5e5000 leaked: 
/snap/zig/14937/lib/std/hash_map.zig:1478:53: 0x1360def in allocate (std.zig)
            const slice = try allocator.alignedAlloc(u8, max_align, total_size);
                                                    ^
/snap/zig/14937/lib/std/hash_map.zig:1435:29: 0x132d75a in grow (std.zig)
            try map.allocate(allocator, new_cap);
                            ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x12eb977 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x12966bb in getOrPutContextAdapted__anon_41539 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1264bd5 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x1230c3b in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^

error(gpa): memory address 0x734172e82000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1098:53: 0x11cfe62 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
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
"Hello, CURSED World!"
Unsupported expression type in interpreter: Literal
Unsupported expression type in interpreter: Literal
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7f6426b80000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7f6426b80080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7f6426b80100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1132:34: 0x11fd070 in parseBlockStatement (cursed_compiler_main.zig)
            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0x11cd0be in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^

error(gpa): memory address 0x7f6426bc0400 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1282560 in create__anon_39180 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1507:66: 0x1244e8a in parseType (cursed_compiler_main.zig)
                    return_type = try self.arena_allocator.create(ast.Type);
                                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1263:51: 0x11ff9fe in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0x11cd73d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^

error(gpa): memory address 0x7f6426ba0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:495:61: 0x11abe52 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7f642e440800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1128:61: 0x11fcdb6 in parseBlockStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0x11cd0be in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.G8blNjfm50	2025-08-31 11:51:35.239001556 +0300
+++ /tmp/tmp.su5niVFXPF	2025-08-31 11:51:35.239001556 +0300
@@ -1,125 +0,0 @@
-"Hello, CURSED World!"
-Unsupported expression type in interpreter: Literal
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
-/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1132:34: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_39180 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1507:66: 0xADDRESS in parseType (cursed_compiler_main.zig)
-                    return_type = try self.arena_allocator.create(ast.Type);
-                                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1263:51: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            let_stmt.var_type = try self.parseType();
-                                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:495:61: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1128:61: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseBlockStatement();
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
Unsupported expression type in interpreter: Literal
Unsupported expression type in interpreter: Literal
Unsupported expression type in interpreter: Literal
"Arithmetic test complete"
Unsupported expression type in interpreter: Literal
Unsupported expression type in interpreter: Literal
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x77270d240000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x77270d240080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1132:34: 0x11fd070 in parseBlockStatement (cursed_compiler_main.zig)
            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0x11cd0be in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^

error(gpa): memory address 0x77270d240100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x77270d2c0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1282560 in create__anon_39180 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1507:66: 0x1244e8a in parseType (cursed_compiler_main.zig)
                    return_type = try self.arena_allocator.create(ast.Type);
                                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1263:51: 0x11ff9fe in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0x11cd73d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^

error(gpa): memory address 0x77270d260000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:495:61: 0x11abe52 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x77270d280800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1270:61: 0x11ffcf0 in parseLetStatement (cursed_compiler_main.zig)
            const init_ptr = try self.arena_allocator.create(Expression);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0x11cd73d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:1127:49: 0x11fcc55 in parseBlockStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7727059e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1128:61: 0x11fcdb6 in parseBlockStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0x11cd0be in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.OOOzU3KKvg	2025-08-31 11:51:35.806667612 +0300
+++ /tmp/tmp.U2Svz8WxT7	2025-08-31 11:51:35.806667612 +0300
@@ -1,148 +0,0 @@
-Unsupported expression type in interpreter: Literal
-Unsupported expression type in interpreter: Literal
-Unsupported expression type in interpreter: Literal
-"Arithmetic test complete"
-Unsupported expression type in interpreter: Literal
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
-/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1132:34: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
-                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0xADDRESS in parseExpressionPrattPrec (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_39180 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1507:66: 0xADDRESS in parseType (cursed_compiler_main.zig)
-                    return_type = try self.arena_allocator.create(ast.Type);
-                                                                 ^
-/home/ghuntley/cursed/src-zig/parser.zig:1263:51: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            let_stmt.var_type = try self.parseType();
-                                                  ^
-/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:495:61: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1270:61: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            const init_ptr = try self.arena_allocator.create(Expression);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
-                                                           ^
-/home/ghuntley/cursed/src-zig/parser.zig:1127:49: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1128:61: 0xADDRESS in parseBlockStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseBlockStatement();
-                                               ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
Unsupported expression type in interpreter: Literal
ERROR: No CURSED stdlib implementation found for module 'a': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/a/mod.💀 for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.💀: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7185faf60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:512:42: 0x11ac2e9 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7185faf60080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1132:34: 0x11fd070 in parseBlockStatement (cursed_compiler_main.zig)
            try statements.append(self.allocator, try self.statementToAnyopaque(stmt_ptr));
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:939:48: 0x11cd0be in parseStatement (cursed_compiler_main.zig)
            return try self.parseBlockStatement();
                                               ^

error(gpa): memory address 0x7185faf60100 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1310aee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9655 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285092 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248575 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1497:47: 0x1244cfa in parseType (cursed_compiler_main.zig)
                        try param_types.append(self.allocator, param_type);
                                              ^
/home/ghuntley/cursed/src-zig/parser.zig:1263:51: 0x11ff9fe in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^

error(gpa): memory address 0x7185faf60180 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x1310aee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12d9655 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1285092 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x1248575 in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1497:47: 0x1244cfa in parseType (cursed_compiler_main.zig)
                        try param_types.append(self.allocator, param_type);
                                              ^
/home/ghuntley/cursed/src-zig/parser.zig:1263:51: 0x11ff9fe in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^

error(gpa): memory address 0x7185faf60200 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7185faf60280 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7185faf60300 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x131abee in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12de265 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1287bc2 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x124e07c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:400:37: 0x131351e in parsePrattCall (cursed_compiler_main.zig)
                try arguments.append(self.allocator, arg_ptr);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^

error(gpa): memory address 0x7185fafa0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1282560 in create__anon_39180 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1507:66: 0x1244e8a in parseType (cursed_compiler_main.zig)
                    return_type = try self.arena_allocator.create(ast.Type);
                                                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1263:51: 0x11ff9fe in parseLetStatement (cursed_compiler_main.zig)
            let_stmt.var_type = try self.parseType();
                                                  ^
/home/ghuntley/cursed/src-zig/parser.zig:957:60: 0x11cd73d in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^

error(gpa): memory address 0x7185fafe0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:495:61: 0x11abe52 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7185faf20000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:495:61: 0x11abe52 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7185faf00000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:398:64: 0x131341b in parsePrattCall (cursed_compiler_main.zig)
                const arg_ptr = try self.arena_allocator.create(Expression);
                                                               ^
/home/ghuntley/cursed/src-zig/parser.zig:1665:34: 0x128586b in parseExpressionPrattPrec (cursed_compiler_main.zig)
            left = try infix_fn.?(self, left);
                                 ^
/home/ghuntley/cursed/src-zig/parser.zig:1648:45: 0x1249723 in parseExpressionPratt (cursed_compiler_main.zig)
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
**Details:** Both modes failed, but differently

### Interpreter Output:
```
"=== Recursive Depth Test ==="
"Shallow recursion:"
thread 1720228 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:1523:9: 0x129e26f in evaluateBinary (cursed_compiler_main.zig)
        return InterpreterError.TypeMismatch;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1416:37: 0x1267948 in evaluateExpression (cursed_compiler_main.zig)
            .Binary => |bin| return try self.evaluateBinary(bin),
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1347:27: 0x126bb56 in executeIfStatement (cursed_compiler_main.zig)
        const condition = try self.evaluateExpression(condition_expr.*);
                          ^
/home/ghuntley/cursed/src-zig/interpreter.zig:689:30: 0x12351c5 in executeStatement (cursed_compiler_main.zig)
            .If => |if_stmt| try self.executeIfStatement(if_stmt),
                             ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x12755cc in sliceAsBytes__anon_38537 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x1236ee4 in free__anon_33739 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x11ee069 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0x123430d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:679:48: 0x1234dd8 in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2684:50: 0x1234212 in callFunction (cursed_compiler_main.zig)
                else => try self.executeStatement(stmt.*),
                                                 ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a0d9d in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7e7d4502a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7e7d4502a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_recursive_depth
```

### Output Diff:
```diff
--- /tmp/tmp.mYzvuFaD7c	2025-08-31 11:51:39.300970306 +0300
+++ /tmp/tmp.DhmBWKl45D	2025-08-31 11:51:39.302970290 +0300
@@ -1,66 +1 @@
-"=== Recursive Depth Test ==="
-"Shallow recursion:"
-thread 1720228 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:1523:9: 0xADDRESS in evaluateBinary (cursed_compiler_main.zig)
-        return InterpreterError.TypeMismatch;
-        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1416:37: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Binary => |bin| return try self.evaluateBinary(bin),
-                                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1347:27: 0xADDRESS in executeIfStatement (cursed_compiler_main.zig)
-        const condition = try self.evaluateExpression(condition_expr.*);
-                          ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:689:30: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-            .If => |if_stmt| try self.executeIfStatement(if_stmt),
-                             ^
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_38537 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_33739 (std.zig)
-    const bytes = mem.sliceAsBytes(memory);
-                                  ^
-/snap/zig/14937/lib/std/array_list.zig:655:21: 0xADDRESS in deinit (std.zig)
-            gpa.free(self.allocatedSlice());
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        defer return_values.deinit(self.allocator);
-                                  ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                        return try self.callFunction(func, args.items);
-                                                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                        const arg = try self.evaluateExpression(arg_expr.*);
-                                                               ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:679:48: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-                _ = try self.evaluateExpression(expr);
-                                               ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2684:50: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-                else => try self.executeStatement(stmt.*),
-                                                 ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
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
+COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_recursive_depth
```

---

## Test: 02_computation_intensive
**Status:** FAIL
**Details:** Both modes failed, but differently

### Interpreter Output:
```
"=== Computation Intensive Test ==="
"Fibonacci sequence:"
thread 1720253 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:1523:9: 0x129e26f in evaluateBinary (cursed_compiler_main.zig)
        return InterpreterError.TypeMismatch;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1416:37: 0x1267948 in evaluateExpression (cursed_compiler_main.zig)
            .Binary => |bin| return try self.evaluateBinary(bin),
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1347:27: 0x126bb56 in executeIfStatement (cursed_compiler_main.zig)
        const condition = try self.evaluateExpression(condition_expr.*);
                          ^
/home/ghuntley/cursed/src-zig/interpreter.zig:689:30: 0x12351c5 in executeStatement (cursed_compiler_main.zig)
            .If => |if_stmt| try self.executeIfStatement(if_stmt),
                             ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x12755cc in sliceAsBytes__anon_38537 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x1236ee4 in free__anon_33739 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x11ee069 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0x123430d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0x12b130c in evaluateCall (cursed_compiler_main.zig)
                        return try self.callFunction(func, args.items);
                                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0x12a4830 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0x1267bc5 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                                        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:679:48: 0x1234dd8 in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                                               ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2684:50: 0x1234212 in callFunction (cursed_compiler_main.zig)
                else => try self.executeStatement(stmt.*),
                                                 ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a0d9d in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7f8069e2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7f8069e2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
INTERPRETER_ERROR: Exit code 134
```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_computation_intensive
```

### Output Diff:
```diff
--- /tmp/tmp.NjcUSq05Qn	2025-08-31 11:51:42.254947644 +0300
+++ /tmp/tmp.TEFKypRWb9	2025-08-31 11:51:42.255947636 +0300
@@ -1,66 +1 @@
-"=== Computation Intensive Test ==="
-"Fibonacci sequence:"
-thread 1720253 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:1523:9: 0xADDRESS in evaluateBinary (cursed_compiler_main.zig)
-        return InterpreterError.TypeMismatch;
-        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1416:37: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Binary => |bin| return try self.evaluateBinary(bin),
-                                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1347:27: 0xADDRESS in executeIfStatement (cursed_compiler_main.zig)
-        const condition = try self.evaluateExpression(condition_expr.*);
-                          ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:689:30: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-            .If => |if_stmt| try self.executeIfStatement(if_stmt),
-                             ^
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_38537 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_33739 (std.zig)
-    const bytes = mem.sliceAsBytes(memory);
-                                  ^
-/snap/zig/14937/lib/std/array_list.zig:655:21: 0xADDRESS in deinit (std.zig)
-            gpa.free(self.allocatedSlice());
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        defer return_values.deinit(self.allocator);
-                                  ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2001:53: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                        return try self.callFunction(func, args.items);
-                                                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1587:64: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                        const arg = try self.evaluateExpression(arg_expr.*);
-                                                               ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1418:57: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                                        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:679:48: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-                _ = try self.evaluateExpression(expr);
-                                               ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2684:50: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-                else => try self.executeStatement(stmt.*),
-                                                 ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
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
+COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_computation_intensive
```

---

## Test: 01_mathz_basic
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.💀 for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7ca08bbc0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:481:46: 0x11ab99e in parseProgram (cursed_compiler_main.zig)
                    program.statements.append(self.allocator, @ptrCast(stmt_ptr)) catch {
                                             ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7ca08bbc0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7ca08bbe0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0x11ab767 in parseProgram (cursed_compiler_main.zig)
                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7ca08bba0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0x11ab767 in parseProgram (cursed_compiler_main.zig)
                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

```

### Compiled Output:
```
COMPILE_ERROR: Exit code 0
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
🔧 Initializing LLVM components...
✅ C library functions declared
✅ Builtin functions registered (yap)
✅ LLVM IR Pipeline initialized successfully
🚀 Starting complete LLVM compilation pipeline...
Error at unknown:9:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at unknown:9:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at unknown:9:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at unknown:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
📦 Processing import: vibez
DEBUG: Loading and compiling CURSED module: vibez
DEBUG: Successfully read CURSED module stdlib/vibez/mod.💀 (655 bytes)
DEBUG: Successfully parsed CURSED module vibez (6 statements)
DEBUG: Compiling CURSED stdlib function: vibez.spill
DEBUG: Compiling CURSED stdlib function: vibez.spillln_message
DEBUG: Compiling CURSED stdlib function: vibez.spillln_drip
DEBUG: Compiling CURSED stdlib function: vibez.spillln_tea
DEBUG: Compiling CURSED stdlib function: vibez.spillln_lit
DEBUG: Compiling CURSED stdlib function: vibez.print_separator
DEBUG: Successfully compiled CURSED module: vibez
📦 Processing import: mathz
DEBUG: Loading and compiling CURSED module: mathz
DEBUG: Successfully read CURSED module stdlib/mathz/mod.💀 (1357 bytes)
DEBUG: Successfully parsed CURSED module mathz (11 statements)
DEBUG: Compiling CURSED stdlib function: mathz.abs
DEBUG: Compiling CURSED stdlib function: mathz.max
thread 1720283 panic: reached unreachable code
/snap/zig/14937/lib/std/debug.zig:559:14: 0x1093a69 in assert (std.zig)
    if (!ok) unreachable; // assertion failure
             ^
/snap/zig/14937/lib/std/hash_map.zig:873:19: 0x1322cdb in putAssumeCapacityNoClobberContext (std.zig)
            assert(!self.containsContext(key, ctx));
                  ^
/snap/zig/14937/lib/std/hash_map.zig:1449:58: 0x12e465c in grow (std.zig)
                    map.putAssumeCapacityNoClobberContext(k, v, ctx);
                                                         ^
/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0x128d237 in growIfNeeded (std.zig)
                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
                             ^
/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0x125512b in getOrPutContextAdapted__anon_36567 (std.zig)
                self.growIfNeeded(allocator, 1, ctx) catch |err| {
                                 ^
/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0x1217d95 in getOrPutContext (std.zig)
            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                       ^
/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0x11d4a1a in putContext (std.zig)
            const result = try self.getOrPutContext(allocator, key, ctx);
                                                   ^
/snap/zig/14937/lib/std/hash_map.zig:323:45: 0x11af26d in put (std.zig)
            return self.unmanaged.putContext(self.allocator, key, value, self.ctx);
                                            ^
/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:491:35: 0x11b33de in generateFunction (cursed_compiler_main.zig)
            try self.variables.put(param.name, param_alloca);
                                  ^
/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:1942:46: 0x11dfd53 in loadAndCompileModule (cursed_compiler_main.zig)
                    try self.generateFunction(func_decl);
                                             ^
/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:415:46: 0x11b9e90 in generateStatement (cursed_compiler_main.zig)
                try self.loadAndCompileModule(import_stmt.path);
                                             ^
/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:2045:39: 0x1197635 in ensureMainFunctionWithGlobalStatements (cursed_compiler_main.zig)
            try self.generateStatement(stmt.*);
                                      ^
/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:380:56: 0x11986bf in generateIR (cursed_compiler_main.zig)
        try self.ensureMainFunctionWithGlobalStatements(global_statements.items);
                                                       ^
/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:260:28: 0x119bd26 in compileSource (cursed_compiler_main.zig)
        try self.generateIR(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:194:27: 0x119cd9e in compileToExecutable (cursed_compiler_main.zig)
    pipeline.compileSource(source, final_output, verbose) catch |err| {
                          ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:101:32: 0x119fd61 in main (cursed_compiler_main.zig)
        try compileToExecutable(allocator, source, filename.?, output_name.?, verbose, debug_mode, optimize, emit_ir);
                               ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a0d9d in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7a0ed8c2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7a0ed8c2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
```

### Output Diff:
```diff
--- /tmp/tmp.2RG8C3eFhT	2025-08-31 11:51:45.088925953 +0300
+++ /tmp/tmp.JmWBY2jIfp	2025-08-31 11:51:45.090682709 +0300
@@ -1,3 +1,4 @@
+COMPILE_ERROR: Exit code 0
 Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
 Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀:9:10 - Synchronizing parser after error (context: synchronize)
 INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
@@ -14,87 +15,98 @@
 Delimiter recoveries: 0
 Total tokens skipped: 5
 ================================
-ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/vibez/mod.💀 for true self-hosting
-❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.💀: error.ModuleNotFound
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
+🔧 Initializing LLVM components...
+✅ C library functions declared
+✅ Builtin functions registered (yap)
+✅ LLVM IR Pipeline initialized successfully
+🚀 Starting complete LLVM compilation pipeline...
+Error at unknown:9:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
+Error at unknown:9:10 - Synchronizing parser after error (context: synchronize)
+INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
+Error at unknown:9:44 - Error parsing function statement
+INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
+INFO: Attempting additional statement recovery
+Error at unknown:28:1 - Failed to parse statement
+
+=== Error Recovery Statistics ===
+Total errors encountered: 2
+Semicolon recoveries: 3
+Statement recoveries: 2
+Expression recoveries: 0
+Delimiter recoveries: 0
+Total tokens skipped: 5
+================================
+📦 Processing import: vibez
+DEBUG: Loading and compiling CURSED module: vibez
+DEBUG: Successfully read CURSED module stdlib/vibez/mod.💀 (655 bytes)
+DEBUG: Successfully parsed CURSED module vibez (6 statements)
+DEBUG: Compiling CURSED stdlib function: vibez.spill
+DEBUG: Compiling CURSED stdlib function: vibez.spillln_message
+DEBUG: Compiling CURSED stdlib function: vibez.spillln_drip
+DEBUG: Compiling CURSED stdlib function: vibez.spillln_tea
+DEBUG: Compiling CURSED stdlib function: vibez.spillln_lit
+DEBUG: Compiling CURSED stdlib function: vibez.print_separator
+DEBUG: Successfully compiled CURSED module: vibez
+📦 Processing import: mathz
+DEBUG: Loading and compiling CURSED module: mathz
+DEBUG: Successfully read CURSED module stdlib/mathz/mod.💀 (1357 bytes)
+DEBUG: Successfully parsed CURSED module mathz (11 statements)
+DEBUG: Compiling CURSED stdlib function: mathz.abs
+DEBUG: Compiling CURSED stdlib function: mathz.max
+thread 1720283 panic: reached unreachable code
+/snap/zig/14937/lib/std/debug.zig:559:14: 0xADDRESS in assert (std.zig)
+    if (!ok) unreachable; // assertion failure
+             ^
+/snap/zig/14937/lib/std/hash_map.zig:873:19: 0xADDRESS in putAssumeCapacityNoClobberContext (std.zig)
+            assert(!self.containsContext(key, ctx));
+                  ^
+/snap/zig/14937/lib/std/hash_map.zig:1449:58: 0xADDRESS in grow (std.zig)
+                    map.putAssumeCapacityNoClobberContext(k, v, ctx);
+                                                         ^
+/snap/zig/14937/lib/std/hash_map.zig:1296:30: 0xADDRESS in growIfNeeded (std.zig)
+                try self.grow(allocator, capacityForSize(self.load() + new_count), ctx);
+                             ^
+/snap/zig/14937/lib/std/hash_map.zig:1115:34: 0xADDRESS in getOrPutContextAdapted__anon_36567 (std.zig)
+                self.growIfNeeded(allocator, 1, ctx) catch |err| {
+                                 ^
+/snap/zig/14937/lib/std/hash_map.zig:1100:56: 0xADDRESS in getOrPutContext (std.zig)
+            const gop = try self.getOrPutContextAdapted(allocator, key, ctx, ctx);
                                                        ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:481:46: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                    program.statements.append(self.allocator, @ptrCast(stmt_ptr)) catch {
+/snap/zig/14937/lib/std/hash_map.zig:1026:52: 0xADDRESS in putContext (std.zig)
+            const result = try self.getOrPutContext(allocator, key, ctx);
+                                                   ^
+/snap/zig/14937/lib/std/hash_map.zig:323:45: 0xADDRESS in put (std.zig)
+            return self.unmanaged.putContext(self.allocator, key, value, self.ctx);
+                                            ^
+/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:491:35: 0xADDRESS in generateFunction (cursed_compiler_main.zig)
+            try self.variables.put(param.name, param_alloca);
+                                  ^
+/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:1942:46: 0xADDRESS in loadAndCompileModule (cursed_compiler_main.zig)
+                    try self.generateFunction(func_decl);
                                              ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
+/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:415:46: 0xADDRESS in generateStatement (cursed_compiler_main.zig)
+                try self.loadAndCompileModule(import_stmt.path);
+                                             ^
+/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:2045:39: 0xADDRESS in ensureMainFunctionWithGlobalStatements (cursed_compiler_main.zig)
+            try self.generateStatement(stmt.*);
+                                      ^
+/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:380:56: 0xADDRESS in generateIR (cursed_compiler_main.zig)
+        try self.ensureMainFunctionWithGlobalStatements(global_statements.items);
                                                        ^
-/snap/zig/14937/lib/std/array_list.zig:1207:51: 0xADDRESS in ensureTotalCapacity (std.zig)
-            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
-                                                  ^
-/snap/zig/14937/lib/std/array_list.zig:1261:41: 0xADDRESS in addOne (std.zig)
-            try self.ensureTotalCapacity(gpa, newlen);
-                                        ^
-/snap/zig/14937/lib/std/array_list.zig:894:49: 0xADDRESS in append (std.zig)
-            const new_item_ptr = try self.addOne(gpa);
-                                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
-                                                                ^
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
-                                                                ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0xADDRESS in interpretSource (cursed_compiler_main.zig)
-    const program = cursed_parser.parseProgram() catch |err| {
-                                              ^
-/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0xADDRESS in main (cursed_compiler_main.zig)
-        try interpretSource(allocator, source, filename.?, verbose);
+/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:260:28: 0xADDRESS in compileSource (cursed_compiler_main.zig)
+        try self.generateIR(program);
                            ^
-
+/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:194:27: 0xADDRESS in compileToExecutable (cursed_compiler_main.zig)
+    pipeline.compileSource(source, final_output, verbose) catch |err| {
+                          ^
+/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:101:32: 0xADDRESS in main (cursed_compiler_main.zig)
+        try compileToExecutable(allocator, source, filename.?, output_name.?, verbose, debug_mode, optimize, emit_ir);
+                               ^
+/snap/zig/14937/lib/std/start.zig:627:37: 0xADDRESS in main (std.zig)
+            const result = root.main() catch |err| {
+                                    ^
+../sysdeps/nptl/libc_start_call_main.h:58:16: 0xADDRESS in __libc_start_call_main (../sysdeps/x86/libc-start.c)
+../csu/libc-start.c:360:3: 0xADDRESS in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
+???:?:?: 0xADDRESS in ??? (???)
+???:?:?: 0xADDRESS in ??? (???)
+timeout: the monitored command dumped core
```

---

## Test: 02_stringz_basic
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:9:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:9:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:9:46 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.💀 for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.💀: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x70bdc3540000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:481:46: 0x11ab99e in parseProgram (cursed_compiler_main.zig)
                    program.statements.append(self.allocator, @ptrCast(stmt_ptr)) catch {
                                             ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x70bdc3540080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x70bdc3580000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0x11ab767 in parseProgram (cursed_compiler_main.zig)
                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x70bdc3520000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0x11ab767 in parseProgram (cursed_compiler_main.zig)
                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
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
"=== Advanced Mathz Test ==="
"Power function:"
ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/mathz/mod.💀 for true self-hosting
thread 1720317 panic: integer overflow
/home/ghuntley/cursed/src-zig/interpreter.zig:465:9: 0x129b914 in get (cursed_compiler_main.zig)
        return InterpreterError.UndefinedVariable;
        ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1413:21: 0x1267860 in evaluateExpression (cursed_compiler_main.zig)
                    return InterpreterError.UndefinedVariable;
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2071:24: 0x12c08c9 in evaluateMethodCall (cursed_compiler_main.zig)
        const object = try self.evaluateExpression(member.object.*);
                       ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1601:28: 0x12a4c98 in evaluateCall (cursed_compiler_main.zig)
                    return try self.evaluateMethodCall(member.*, call.arguments.items);
                           ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:36: 0x1267be9 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1587:37: 0x12a4854 in evaluateCall (cursed_compiler_main.zig)
                        const arg = try self.evaluateExpression(arg_expr.*);
                                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:1418:36: 0x1267be9 in evaluateExpression (cursed_compiler_main.zig)
            .Call => |call| return try self.evaluateCall(call),
                                   ^
/home/ghuntley/cursed/src-zig/interpreter.zig:679:21: 0x1234e04 in executeStatement (cursed_compiler_main.zig)
                _ = try self.evaluateExpression(expr);
                    ^
/snap/zig/14937/lib/std/mem.zig:4356:61: 0x12755cc in sliceAsBytes__anon_38537 (std.zig)
    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
                                                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0x1236ee4 in free__anon_33739 (std.zig)
    const bytes = mem.sliceAsBytes(memory);
                                  ^
/snap/zig/14937/lib/std/array_list.zig:655:21: 0x11ee069 in deinit (std.zig)
            gpa.free(self.allocatedSlice());
                    ^
/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0x123430d in callFunction (cursed_compiler_main.zig)
        defer return_values.deinit(self.allocator);
                                  ^
/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0x11ebdf2 in execute (cursed_compiler_main.zig)
            _ = try self.callFunction(main_func, &[_]Value{});
                                     ^
/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0x11c1243 in interpret (cursed_compiler_main.zig)
        return self.execute(program);
                           ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:251:33: 0x119e01c in interpretSource (cursed_compiler_main.zig)
    cursed_interpreter.interpret(program) catch |err| {
                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^
/snap/zig/14937/lib/std/start.zig:627:37: 0x11a0d9d in main (std.zig)
            const result = root.main() catch |err| {
                                    ^
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x776591c2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x776591c2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
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
--- /tmp/tmp.8rQXJRaBWA	2025-08-31 11:51:48.567899396 +0300
+++ /tmp/tmp.1qKb5v1LXt	2025-08-31 11:51:48.569899381 +0300
@@ -1,62 +1 @@
-"=== Advanced Mathz Test ==="
-"Power function:"
-ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/mathz/mod.💀 for true self-hosting
-thread 1720317 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:465:9: 0xADDRESS in get (cursed_compiler_main.zig)
-        return InterpreterError.UndefinedVariable;
-        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1413:21: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                    return InterpreterError.UndefinedVariable;
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2071:24: 0xADDRESS in evaluateMethodCall (cursed_compiler_main.zig)
-        const object = try self.evaluateExpression(member.object.*);
-                       ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1601:28: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                    return try self.evaluateMethodCall(member.*, call.arguments.items);
-                           ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1418:36: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                   ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1587:37: 0xADDRESS in evaluateCall (cursed_compiler_main.zig)
-                        const arg = try self.evaluateExpression(arg_expr.*);
-                                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1418:36: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-            .Call => |call| return try self.evaluateCall(call),
-                                   ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:679:21: 0xADDRESS in executeStatement (cursed_compiler_main.zig)
-                _ = try self.evaluateExpression(expr);
-                    ^
-/snap/zig/14937/lib/std/mem.zig:4356:61: 0xADDRESS in sliceAsBytes__anon_38537 (std.zig)
-    return @as(cast_target, @ptrCast(slice))[0 .. slice.len * @sizeOf(std.meta.Elem(Slice))];
-                                                            ^
-/snap/zig/14937/lib/std/mem/Allocator.zig:424:35: 0xADDRESS in free__anon_33739 (std.zig)
-    const bytes = mem.sliceAsBytes(memory);
-                                  ^
-/snap/zig/14937/lib/std/array_list.zig:655:21: 0xADDRESS in deinit (std.zig)
-            gpa.free(self.allocatedSlice());
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2662:35: 0xADDRESS in callFunction (cursed_compiler_main.zig)
-        defer return_values.deinit(self.allocator);
-                                  ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:665:38: 0xADDRESS in execute (cursed_compiler_main.zig)
-            _ = try self.callFunction(main_func, &[_]Value{});
-                                     ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:628:28: 0xADDRESS in interpret (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:9:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:9:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:9:49 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 10
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.💀 for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.💀: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7d3e0df20000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128193e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x12422e5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11fc932 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11ccdac in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:481:46: 0x11ab99e in parseProgram (cursed_compiler_main.zig)
                    program.statements.append(self.allocator, @ptrCast(stmt_ptr)) catch {
                                             ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7d3e0df20080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d3e0df60000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0x11ab767 in parseProgram (cursed_compiler_main.zig)
                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7d3e0df00000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:475:65: 0x11ab767 in parseProgram (cursed_compiler_main.zig)
                    const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                                ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x119dccd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x119ff87 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x773409be0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x12573ae in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1218745 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11d6012 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b09cc in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0x11fe984 in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x773411460200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10ee135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x12003b0 in create__anon_30719 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0x1211d68 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0x11cfba9 in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7734114a0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0x11fe819 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0x11cd1b5 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0x11abd84 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.0lqEHoMJFL	2025-08-31 11:51:49.625891332 +0300
+++ /tmp/tmp.iXKI9YdkrW	2025-08-31 11:51:49.626891324 +0300
@@ -1,76 +0,0 @@
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
-/home/ghuntley/cursed/src-zig/parser.zig:1230:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30719 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3795:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1086:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_26059 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1221:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:944:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:494:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

