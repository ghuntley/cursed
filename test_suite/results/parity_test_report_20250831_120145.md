# CURSED Interpreter vs Compiler Parity Test Report

**Generated:** Sun Aug 31 12:02:19 PM EEST 2025
**Test Suite Version:** 1.0.0
**CURSED Compiler:** /home/ghuntley/cursed/test_suite/../zig-out/bin/cursed-compiler

## Executive Summary

- **Total Tests:** 35
- **Passed:** 0
- **Failed:** 24
- **Compile Errors:** 11
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
- **features:** 2 tests
- **functions:** 4 tests
- **parser_fixes:** 3 tests
- **performance:** 2 tests
- **regression:** 2 tests
- **stdlib:** 4 tests
- **strings:** 1 tests
- **validation:** 2 tests

## Recommendations

- 🚨 **High Priority:** Fix compilation failures (11 tests)
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
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x76989eec0000 leaked: 
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

error(gpa): memory address 0x76989ef00200 leaked: 
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

error(gpa): memory address 0x76989ef40800 leaked: 
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
--- /tmp/tmp.qg8PlYQ9WP	2025-08-31 12:01:46.277517415 +0300
+++ /tmp/tmp.ZMBo1wJcjI	2025-08-31 12:01:46.277517415 +0300
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
error(gpa): memory address 0x7f45bb2e0000 leaked: 
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

error(gpa): memory address 0x7f45bb360000 leaked: 
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

error(gpa): memory address 0x7f45bb360080 leaked: 
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

error(gpa): memory address 0x7f45bb360100 leaked: 
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

error(gpa): memory address 0x7f45bb360180 leaked: 
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

error(gpa): memory address 0x7f45bb360200 leaked: 
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

error(gpa): memory address 0x7f45bb360280 leaked: 
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

error(gpa): memory address 0x7f45bb360300 leaked: 
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

error(gpa): memory address 0x7f45bb360380 leaked: 
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

error(gpa): memory address 0x7f45bb360400 leaked: 
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

error(gpa): memory address 0x7f45bb360480 leaked: 
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

error(gpa): memory address 0x7f45bb360500 leaked: 
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

error(gpa): memory address 0x7f45bb360580 leaked: 
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

error(gpa): memory address 0x7f45bb360600 leaked: 
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

error(gpa): memory address 0x7f45bb360680 leaked: 
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

error(gpa): memory address 0x7f45bb360700 leaked: 
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

error(gpa): memory address 0x7f45bb3a0200 leaked: 
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

error(gpa): memory address 0x7f45bb3e0800 leaked: 
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

error(gpa): memory address 0x7f45bb320000 leaked: 
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

error(gpa): memory address 0x7f45bb300000 leaked: 
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

error(gpa): memory address 0x7f45bb342000 leaked: 
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
--- /tmp/tmp.A8S3pLf9kl	2025-08-31 12:01:46.944106975 +0300
+++ /tmp/tmp.nrHTyumNLu	2025-08-31 12:01:46.944106975 +0300
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
error(gpa): memory address 0x747029ce0000 leaked: 
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

error(gpa): memory address 0x747029d60000 leaked: 
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

error(gpa): memory address 0x747029d60080 leaked: 
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

error(gpa): memory address 0x747029d60100 leaked: 
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

error(gpa): memory address 0x747029d60180 leaked: 
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

error(gpa): memory address 0x747029d60200 leaked: 
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

error(gpa): memory address 0x747029d60280 leaked: 
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

error(gpa): memory address 0x747029d60300 leaked: 
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

error(gpa): memory address 0x747029d60380 leaked: 
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

error(gpa): memory address 0x747029d60400 leaked: 
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

error(gpa): memory address 0x747029d60480 leaked: 
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

error(gpa): memory address 0x747029d60500 leaked: 
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

error(gpa): memory address 0x747029d60580 leaked: 
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

error(gpa): memory address 0x747029da0200 leaked: 
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

error(gpa): memory address 0x747029de0800 leaked: 
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

error(gpa): memory address 0x747029d20000 leaked: 
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

error(gpa): memory address 0x747029d00000 leaked: 
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

error(gpa): memory address 0x747029d42000 leaked: 
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
--- /tmp/tmp.xFLJ6T2vm9	2025-08-31 12:01:47.580469244 +0300
+++ /tmp/tmp.XRpjg2U7ap	2025-08-31 12:01:47.580469244 +0300
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
error(gpa): memory address 0x70028a9c0000 leaked: 
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

error(gpa): memory address 0x700292240200 leaked: 
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

error(gpa): memory address 0x700292280800 leaked: 
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
--- /tmp/tmp.obu32Wi4Jp	2025-08-31 12:01:48.117100568 +0300
+++ /tmp/tmp.xptGZ4YlC4	2025-08-31 12:01:48.117100568 +0300
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
error(gpa): memory address 0x7431ced40000 leaked: 
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

error(gpa): memory address 0x7431cede0000 leaked: 
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

error(gpa): memory address 0x7431cede0080 leaked: 
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

error(gpa): memory address 0x7431cede0100 leaked: 
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

error(gpa): memory address 0x7431ceda0400 leaked: 
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

error(gpa): memory address 0x7431ced80400 leaked: 
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
--- /tmp/tmp.G7i6Pj5XTg	2025-08-31 12:01:48.675097521 +0300
+++ /tmp/tmp.ag3Po7r3wU	2025-08-31 12:01:48.675097521 +0300
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
error(gpa): memory address 0x734a64660000 leaked: 
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

error(gpa): memory address 0x734a646a0200 leaked: 
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

error(gpa): memory address 0x734a646e0800 leaked: 
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
--- /tmp/tmp.H3IUILilY6	2025-08-31 12:01:49.215094572 +0300
+++ /tmp/tmp.3kjvfzttbG	2025-08-31 12:01:49.216094567 +0300
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
error(gpa): memory address 0x76ce8bbc0000 leaked: 
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

error(gpa): memory address 0x76ce93460200 leaked: 
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

error(gpa): memory address 0x76ce93440000 leaked: 
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
--- /tmp/tmp.xvOEP8nfNW	2025-08-31 12:01:49.753091635 +0300
+++ /tmp/tmp.45AtdyInV8	2025-08-31 12:01:49.753091635 +0300
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
error(gpa): memory address 0x75bc104c0000 leaked: 
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

error(gpa): memory address 0x75bc104c0080 leaked: 
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

error(gpa): memory address 0x75bc104c0100 leaked: 
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

error(gpa): memory address 0x75bc104c0180 leaked: 
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

error(gpa): memory address 0x75bc104c0200 leaked: 
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

error(gpa): memory address 0x75bc10560300 leaked: 
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

error(gpa): memory address 0x75bc10540000 leaked: 
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

error(gpa): memory address 0x75bc104e0000 leaked: 
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

error(gpa): memory address 0x75bc105a1000 leaked: 
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

error(gpa): memory address 0x75bc104a0000 leaked: 
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
error(gpa): memory address 0x70dbbd120000 leaked: 
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

error(gpa): memory address 0x70dbbd120080 leaked: 
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

error(gpa): memory address 0x70dbbd120100 leaked: 
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

error(gpa): memory address 0x70dbbd120180 leaked: 
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

error(gpa): memory address 0x70dbbd120200 leaked: 
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

error(gpa): memory address 0x70dbbd160200 leaked: 
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

error(gpa): memory address 0x70dbbd1a0800 leaked: 
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

error(gpa): memory address 0x70dbbd0e0000 leaked: 
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
--- /tmp/tmp.WjE6PLi9a6	2025-08-31 12:01:50.878085495 +0300
+++ /tmp/tmp.0KV5YyoYV3	2025-08-31 12:01:50.878085495 +0300
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
error(gpa): memory address 0x7d636d060000 leaked: 
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

error(gpa): memory address 0x7d636d0a0200 leaked: 
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

error(gpa): memory address 0x7d636d0e0800 leaked: 
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
--- /tmp/tmp.ov6RMgIEDa	2025-08-31 12:01:51.416082560 +0300
+++ /tmp/tmp.a793CS1IJp	2025-08-31 12:01:51.417082555 +0300
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
error(gpa): memory address 0x735db3740000 leaked: 
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

error(gpa): memory address 0x735db37c0200 leaked: 
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

error(gpa): memory address 0x735db3760000 leaked: 
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
--- /tmp/tmp.qXH6Oa5QOs	2025-08-31 12:01:51.952079636 +0300
+++ /tmp/tmp.o1Z7HHCt4K	2025-08-31 12:01:51.952079636 +0300
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
error(gpa): memory address 0x78d2b97a0000 leaked: 
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

error(gpa): memory address 0x78d2c1060000 leaked: 
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

error(gpa): memory address 0x78d2c1060080 leaked: 
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

error(gpa): memory address 0x78d2c1060100 leaked: 
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

error(gpa): memory address 0x78d2c1060180 leaked: 
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

error(gpa): memory address 0x78d2c1060200 leaked: 
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

error(gpa): memory address 0x78d2c1060280 leaked: 
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

error(gpa): memory address 0x78d2c1060300 leaked: 
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

error(gpa): memory address 0x78d2c1060380 leaked: 
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

error(gpa): memory address 0x78d2c1060400 leaked: 
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

error(gpa): memory address 0x78d2c1060480 leaked: 
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

error(gpa): memory address 0x78d2c1060500 leaked: 
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

error(gpa): memory address 0x78d2c1060580 leaked: 
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

error(gpa): memory address 0x78d2c1060600 leaked: 
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

error(gpa): memory address 0x78d2c1060680 leaked: 
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

error(gpa): memory address 0x78d2c1060700 leaked: 
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

error(gpa): memory address 0x78d2c1060780 leaked: 
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

error(gpa): memory address 0x78d2c1080200 leaked: 
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

error(gpa): memory address 0x78d2c10c0800 leaked: 
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

error(gpa): memory address 0x78d2c1040000 leaked: 
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

error(gpa): memory address 0x78d2b97c0000 leaked: 
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

error(gpa): memory address 0x78d2b97e2000 leaked: 
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
--- /tmp/tmp.ucYOjydIcB	2025-08-31 12:01:52.628075950 +0300
+++ /tmp/tmp.ylQZPVeDCY	2025-08-31 12:01:52.629075944 +0300
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
error(gpa): memory address 0x7fcb63360000 leaked: 
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

error(gpa): memory address 0x7fcb63360080 leaked: 
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

error(gpa): memory address 0x7fcb63360100 leaked: 
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

error(gpa): memory address 0x7fcb63360180 leaked: 
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

error(gpa): memory address 0x7fcb63360200 leaked: 
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

error(gpa): memory address 0x7fcb633a0200 leaked: 
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

error(gpa): memory address 0x7fcb633e0800 leaked: 
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

error(gpa): memory address 0x7fcb63320000 leaked: 
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
thread 1724618 panic: integer overflow
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7d5e6a42a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7d5e6a42a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
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
thread 1724644 panic: integer overflow
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7eab6be2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7eab6be2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
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

## Test: feature_control_flow_simple
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:6:44 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:21:1 - Failed to parse statement

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
error(gpa): memory address 0x7d837a7a0000 leaked: 
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

error(gpa): memory address 0x7d837a7e0200 leaked: 
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

error(gpa): memory address 0x7d8382040000 leaked: 
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
--- /tmp/tmp.UHXrrmtvxX	2025-08-31 12:01:59.420038956 +0300
+++ /tmp/tmp.4hfRxXBBDL	2025-08-31 12:01:59.420038956 +0300
@@ -1,76 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:6:44 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.💀:21:1 - Failed to parse statement
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

## Test: feature_stdlib_integration
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.💀:8:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.💀:8:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.💀:8:38 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.💀:20:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/mathz/mod.💀 for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.💀: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7a0839c40000 leaked: 
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

error(gpa): memory address 0x7a0839c40080 leaked: 
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

error(gpa): memory address 0x7a08323c0400 leaked: 
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

error(gpa): memory address 0x7a08323a0000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../feature_stdlib_integration
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
error(gpa): memory address 0x750c8e060000 leaked: 
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

error(gpa): memory address 0x750c8e060080 leaked: 
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

error(gpa): memory address 0x750c8e060100 leaked: 
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

error(gpa): memory address 0x750c8e060180 leaked: 
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

error(gpa): memory address 0x750c8e060200 leaked: 
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

error(gpa): memory address 0x750c8e060280 leaked: 
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

error(gpa): memory address 0x750c8e060300 leaked: 
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

error(gpa): memory address 0x750c867e0200 leaked: 
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

error(gpa): memory address 0x750c867e0300 leaked: 
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

error(gpa): memory address 0x750c867c0800 leaked: 
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

error(gpa): memory address 0x750c8e040000 leaked: 
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
error(gpa): memory address 0x7f4beb460000 leaked: 
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

error(gpa): memory address 0x7f4beb460080 leaked: 
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

error(gpa): memory address 0x7f4beb460100 leaked: 
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

error(gpa): memory address 0x7f4beb460180 leaked: 
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

error(gpa): memory address 0x7f4beb460200 leaked: 
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

error(gpa): memory address 0x7f4beb460280 leaked: 
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

error(gpa): memory address 0x7f4beb460300 leaked: 
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

error(gpa): memory address 0x7f4beb460380 leaked: 
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

error(gpa): memory address 0x7f4beb460400 leaked: 
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

error(gpa): memory address 0x7f4beb460480 leaked: 
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

error(gpa): memory address 0x7f4beb460500 leaked: 
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

error(gpa): memory address 0x7f4beb480200 leaked: 
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

error(gpa): memory address 0x7f4beb4c0800 leaked: 
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

error(gpa): memory address 0x7f4beb440000 leaked: 
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

error(gpa): memory address 0x7f4be3bc0000 leaked: 
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
error(gpa): memory address 0x759073f80000 leaked: 
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

error(gpa): memory address 0x759073f80080 leaked: 
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

error(gpa): memory address 0x759073f800c0 leaked: 
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

error(gpa): memory address 0x759073f80140 leaked: 
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

error(gpa): memory address 0x759073f80180 leaked: 
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

error(gpa): memory address 0x759073f801c0 leaked: 
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

error(gpa): memory address 0x759073f80200 leaked: 
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

error(gpa): memory address 0x759073f80240 leaked: 
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

error(gpa): memory address 0x759073fe0000 leaked: 
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

error(gpa): memory address 0x759073fe0080 leaked: 
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

error(gpa): memory address 0x759073fe0100 leaked: 
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

error(gpa): memory address 0x759073fe0180 leaked: 
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

error(gpa): memory address 0x759073fe0200 leaked: 
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

error(gpa): memory address 0x759073fe0280 leaked: 
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

error(gpa): memory address 0x759073fe0300 leaked: 
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

error(gpa): memory address 0x759073fe0380 leaked: 
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

error(gpa): memory address 0x759073fe0400 leaked: 
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

error(gpa): memory address 0x759073fe0480 leaked: 
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

error(gpa): memory address 0x759073fe0500 leaked: 
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

error(gpa): memory address 0x759073fe0580 leaked: 
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

error(gpa): memory address 0x759073fe0600 leaked: 
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

error(gpa): memory address 0x759073fe0680 leaked: 
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

error(gpa): memory address 0x759073fe0700 leaked: 
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

error(gpa): memory address 0x759073fe0780 leaked: 
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

error(gpa): memory address 0x759073fe0800 leaked: 
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

error(gpa): memory address 0x759073fe0880 leaked: 
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

error(gpa): memory address 0x759073fe0900 leaked: 
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

error(gpa): memory address 0x759073fe0980 leaked: 
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

error(gpa): memory address 0x759073fe0a00 leaked: 
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

error(gpa): memory address 0x759073fe0a80 leaked: 
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

error(gpa): memory address 0x759073fe0b00 leaked: 
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

error(gpa): memory address 0x759073fe0b80 leaked: 
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

error(gpa): memory address 0x759073fe0c00 leaked: 
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

error(gpa): memory address 0x75907b860200 leaked: 
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

error(gpa): memory address 0x75907b8a0800 leaked: 
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

error(gpa): memory address 0x759073fc0000 leaked: 
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

error(gpa): memory address 0x759073fa0000 leaked: 
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

error(gpa): memory address 0x759073fa3000 leaked: 
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

error(gpa): memory address 0x759073fa4000 leaked: 
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

error(gpa): memory address 0x759073fa5000 leaked: 
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

error(gpa): memory address 0x759073fa6000 leaked: 
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

error(gpa): memory address 0x759073fa7000 leaked: 
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

error(gpa): memory address 0x759073fa8000 leaked: 
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

error(gpa): memory address 0x759073fa9000 leaked: 
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

error(gpa): memory address 0x75907b842000 leaked: 
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
error(gpa): memory address 0x7bda7cac0000 leaked: 
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

error(gpa): memory address 0x7bda7cac0080 leaked: 
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

error(gpa): memory address 0x7bda7cac00c0 leaked: 
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

error(gpa): memory address 0x7bda7cac0100 leaked: 
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

error(gpa): memory address 0x7bda7cac0140 leaked: 
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

error(gpa): memory address 0x7bda7cb60000 leaked: 
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

error(gpa): memory address 0x7bda7cb60080 leaked: 
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

error(gpa): memory address 0x7bda7cb60100 leaked: 
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

error(gpa): memory address 0x7bda7cb60180 leaked: 
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

error(gpa): memory address 0x7bda7cb60280 leaked: 
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

error(gpa): memory address 0x7bda7cb60300 leaked: 
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

error(gpa): memory address 0x7bda7cb60380 leaked: 
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

error(gpa): memory address 0x7bda7cb60400 leaked: 
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

error(gpa): memory address 0x7bda7cb60480 leaked: 
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

error(gpa): memory address 0x7bda7cb60500 leaked: 
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

error(gpa): memory address 0x7bda7cb60580 leaked: 
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

error(gpa): memory address 0x7bda7cb60600 leaked: 
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

error(gpa): memory address 0x7bda7cb60680 leaked: 
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

error(gpa): memory address 0x7bda7cb60700 leaked: 
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

error(gpa): memory address 0x7bda7cb60780 leaked: 
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

error(gpa): memory address 0x7bda7cb60800 leaked: 
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

error(gpa): memory address 0x7bda7cb60880 leaked: 
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

error(gpa): memory address 0x7bda7cb60900 leaked: 
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

error(gpa): memory address 0x7bda7cb60980 leaked: 
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

error(gpa): memory address 0x7bda7cba0200 leaked: 
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

error(gpa): memory address 0x7bda7cb00000 leaked: 
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

error(gpa): memory address 0x7bda7cbe0800 leaked: 
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

error(gpa): memory address 0x7bda7cb20000 leaked: 
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

error(gpa): memory address 0x7bda7cae0000 leaked: 
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

error(gpa): memory address 0x7bda7cae3000 leaked: 
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

error(gpa): memory address 0x7bda7cae4000 leaked: 
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

error(gpa): memory address 0x7bda7cae5000 leaked: 
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

error(gpa): memory address 0x7bda7cb42000 leaked: 
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
error(gpa): memory address 0x75bf3ab20000 leaked: 
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

error(gpa): memory address 0x75bf3ab20080 leaked: 
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

error(gpa): memory address 0x75bf3ab20100 leaked: 
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

error(gpa): memory address 0x75bf3ab60400 leaked: 
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

error(gpa): memory address 0x75bf3ab40000 leaked: 
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

error(gpa): memory address 0x75bf3aba0800 leaked: 
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
--- /tmp/tmp.h7EvVTuCOT	2025-08-31 12:02:03.067019127 +0300
+++ /tmp/tmp.26tHSwIOg2	2025-08-31 12:02:03.067019127 +0300
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
error(gpa): memory address 0x74e9c0580000 leaked: 
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

error(gpa): memory address 0x74e9c0580080 leaked: 
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

error(gpa): memory address 0x74e9c0580100 leaked: 
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

error(gpa): memory address 0x74e9c05e0200 leaked: 
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

error(gpa): memory address 0x74e9c05a0000 leaked: 
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

error(gpa): memory address 0x74e9c7e40800 leaked: 
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

error(gpa): memory address 0x74e9c0560000 leaked: 
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
--- /tmp/tmp.iKwCMyfGZi	2025-08-31 12:02:03.624016101 +0300
+++ /tmp/tmp.MoHUuX1ETz	2025-08-31 12:02:03.625016095 +0300
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
error(gpa): memory address 0x775b26ce0000 leaked: 
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

error(gpa): memory address 0x775b26ce0080 leaked: 
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

error(gpa): memory address 0x775b26ce0100 leaked: 
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

error(gpa): memory address 0x775b26ce0180 leaked: 
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

error(gpa): memory address 0x775b26ce0200 leaked: 
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

error(gpa): memory address 0x775b26ce0280 leaked: 
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

error(gpa): memory address 0x775b26ce0300 leaked: 
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

error(gpa): memory address 0x775b26d20200 leaked: 
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

error(gpa): memory address 0x775b26d60800 leaked: 
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

error(gpa): memory address 0x775b26ca0000 leaked: 
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

error(gpa): memory address 0x775b269e0000 leaked: 
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
thread 1724832 panic: integer overflow
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7f223022a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7f223022a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
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
--- /tmp/tmp.MO3y3YVVp3	2025-08-31 12:02:07.099997228 +0300
+++ /tmp/tmp.LoWStSjW54	2025-08-31 12:02:07.102405753 +0300
@@ -1,66 +1 @@
-"=== Recursive Depth Test ==="
-"Shallow recursion:"
-thread 1724832 panic: integer overflow
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
thread 1724857 panic: integer overflow
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x71808942a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x71808942a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
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
--- /tmp/tmp.SiNvVbs94E	2025-08-31 12:02:10.056981190 +0300
+++ /tmp/tmp.KmqUwpsmaU	2025-08-31 12:02:10.057981185 +0300
@@ -1,66 +1 @@
-"=== Computation Intensive Test ==="
-"Fibonacci sequence:"
-thread 1724857 panic: integer overflow
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

## Test: regression_memory_management
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.💀:13:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.💀:13:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.💀:13:38 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.💀:28:1 - Failed to parse statement

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
error(gpa): memory address 0x77fe6a960000 leaked: 
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

error(gpa): memory address 0x77fe6a960080 leaked: 
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

error(gpa): memory address 0x77fe6a960100 leaked: 
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

error(gpa): memory address 0x77fe6a960180 leaked: 
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

error(gpa): memory address 0x77fe6a960200 leaked: 
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

error(gpa): memory address 0x77fe6a9a0200 leaked: 
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

error(gpa): memory address 0x77fe6a9e0800 leaked: 
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

error(gpa): memory address 0x77fe6a920000 leaked: 
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

error(gpa): memory address 0x77fe6a900000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../regression_memory_management
```

---

## Test: regression_parser_precedence
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
"=== Precedence Test ==="
"2 + 3 * 4 = 14"
14
"10 - 5 - 2 = 3"
3
"(2 + 3) * 4 = 20"
20
"=== Test Complete ==="
Executing defers from size 0 to 0
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x78294fb40000 leaked: 
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

error(gpa): memory address 0x78294fba0000 leaked: 
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

error(gpa): memory address 0x78294fba0080 leaked: 
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

error(gpa): memory address 0x78294fba0100 leaked: 
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

error(gpa): memory address 0x78294fba0180 leaked: 
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

error(gpa): memory address 0x78294fba0200 leaked: 
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

error(gpa): memory address 0x78294fba0280 leaked: 
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

error(gpa): memory address 0x78294fba0300 leaked: 
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

error(gpa): memory address 0x78294fba0380 leaked: 
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

error(gpa): memory address 0x78294fba0400 leaked: 
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

error(gpa): memory address 0x78294fba0480 leaked: 
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

error(gpa): memory address 0x78294fbe0200 leaked: 
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

error(gpa): memory address 0x782957440000 leaked: 
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

error(gpa): memory address 0x78294fb80000 leaked: 
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

error(gpa): memory address 0x78294fb60000 leaked: 
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

error(gpa): memory address 0x782957462000 leaked: 
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
--- /tmp/tmp.QjkwxR9vWh	2025-08-31 12:02:11.218974892 +0300
+++ /tmp/tmp.L847q1SKsM	2025-08-31 12:02:11.218974892 +0300
@@ -1,331 +0,0 @@
-"=== Precedence Test ==="
-"2 + 3 * 4 = 14"
-14
-"10 - 5 - 2 = 3"
-3
-"(2 + 3) * 4 = 20"
-20
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
error(gpa): memory address 0x799dc4740000 leaked: 
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

error(gpa): memory address 0x799dc4740080 leaked: 
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

error(gpa): memory address 0x799dc4780000 leaked: 
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

error(gpa): memory address 0x799dc4720000 leaked: 
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
thread 1724927 panic: reached unreachable code
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7dbd9ea2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7dbd9ea2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
```

### Output Diff:
```diff
--- /tmp/tmp.h1BeYM5b9H	2025-08-31 12:02:14.038959618 +0300
+++ /tmp/tmp.cXkDRE7Rnu	2025-08-31 12:02:14.041594808 +0300
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
+thread 1724927 panic: reached unreachable code
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
error(gpa): memory address 0x78ca20de0000 leaked: 
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

error(gpa): memory address 0x78ca20de0080 leaked: 
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

error(gpa): memory address 0x78ca28660000 leaked: 
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

error(gpa): memory address 0x78ca20dc0000 leaked: 
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
thread 1724961 panic: integer overflow
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7e96afe2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7e96afe2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
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
--- /tmp/tmp.H7WQRocPow	2025-08-31 12:02:17.464941082 +0300
+++ /tmp/tmp.buPZHbD7wR	2025-08-31 12:02:17.467473603 +0300
@@ -1,62 +1 @@
-"=== Advanced Mathz Test ==="
-"Power function:"
-ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/mathz/mod.💀 for true self-hosting
-thread 1724961 panic: integer overflow
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
error(gpa): memory address 0x7d90a13c0000 leaked: 
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

error(gpa): memory address 0x7d90a13c0080 leaked: 
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

error(gpa): memory address 0x7d90a13e0000 leaked: 
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

error(gpa): memory address 0x7d90a13a0000 leaked: 
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
error(gpa): memory address 0x7d186b860000 leaked: 
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

error(gpa): memory address 0x7d186b880200 leaked: 
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

error(gpa): memory address 0x7d186b8c0800 leaked: 
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
--- /tmp/tmp.xwDM6aZBmY	2025-08-31 12:02:18.520570796 +0300
+++ /tmp/tmp.Xl7WkOMxMe	2025-08-31 12:02:18.520570796 +0300
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

## Test: validation_basic_syntax
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:6:44 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:18:1 - Failed to parse statement

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
error(gpa): memory address 0x73098c720000 leaked: 
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

error(gpa): memory address 0x73098c7a0200 leaked: 
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

error(gpa): memory address 0x73098c760000 leaked: 
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
--- /tmp/tmp.gnIdnLiiO4	2025-08-31 12:02:19.058932464 +0300
+++ /tmp/tmp.lAkv4KdUmU	2025-08-31 12:02:19.058932464 +0300
@@ -1,76 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:6:44 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.💀:18:1 - Failed to parse statement
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

## Test: validation_type_system
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:6:43 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:21:1 - Failed to parse statement

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
error(gpa): memory address 0x737068460000 leaked: 
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

error(gpa): memory address 0x7370684a0200 leaked: 
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

error(gpa): memory address 0x7370684e0800 leaked: 
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
--- /tmp/tmp.E3iBSxOeCg	2025-08-31 12:02:19.595929562 +0300
+++ /tmp/tmp.41BfUxumfF	2025-08-31 12:02:19.595929562 +0300
@@ -1,76 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:6:43 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.💀:21:1 - Failed to parse statement
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

