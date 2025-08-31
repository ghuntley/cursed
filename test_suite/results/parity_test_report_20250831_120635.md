# CURSED Interpreter vs Compiler Parity Test Report

**Generated:** Sun Aug 31 12:06:59 PM EEST 2025
**Test Suite Version:** 1.0.0
**CURSED Compiler:** /home/ghuntley/cursed/test_suite/../zig-out/bin/cursed-compiler

## Executive Summary

- **Total Tests:** 35
- **Passed:** 0
- **Failed:** 4
- **Compile Errors:** 31
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

- 🚨 **High Priority:** Fix compilation failures (31 tests)
- 📋 **Low Priority:** Investigate output differences (4 tests)

## Detailed Test Results

## Test: 01_mixed_types
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:10:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:10:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:10:53 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd:35:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/arithmetic/01_mixed_types.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x769886520000 leaked: 
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

error(gpa): memory address 0x769886520080 leaked: 
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

error(gpa): memory address 0x769886560000 leaked: 
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

error(gpa): memory address 0x769886500000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_mixed_types
```

### Output Diff:
```diff
--- /tmp/tmp.qg8PlYQ9WP	2025-08-31 12:01:46.277517415 +0300
+++ /tmp/tmp.ZMBo1wJcjI	2025-08-31 12:01:46.277517415 +0300
@@ -1,76 +0,0 @@
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7ea1e7520000 leaked: 
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

error(gpa): memory address 0x7ea1e7520080 leaked: 
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

error(gpa): memory address 0x7ea1e7520100 leaked: 
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

error(gpa): memory address 0x7ea1e7520180 leaked: 
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

error(gpa): memory address 0x7ea1e7520200 leaked: 
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

error(gpa): memory address 0x7ea1e7520280 leaked: 
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

error(gpa): memory address 0x7ea1e7520300 leaked: 
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

error(gpa): memory address 0x7ea1e7520380 leaked: 
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

error(gpa): memory address 0x7ea1e7520400 leaked: 
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

error(gpa): memory address 0x7ea1e7520480 leaked: 
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

error(gpa): memory address 0x7ea1e7520500 leaked: 
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

error(gpa): memory address 0x7ea1e7520580 leaked: 
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

error(gpa): memory address 0x7ea1e7520600 leaked: 
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

error(gpa): memory address 0x7ea1e7520680 leaked: 
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

error(gpa): memory address 0x7ea1e7520700 leaked: 
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

error(gpa): memory address 0x7ea1e7560000 leaked: 
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

error(gpa): memory address 0x7ea1e75e0800 leaked: 
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

error(gpa): memory address 0x7ea1e7500000 leaked: 
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

error(gpa): memory address 0x7ea1e74e0000 leaked: 
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

error(gpa): memory address 0x7ea1e7542000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_edge_cases
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x72ceb1940000 leaked: 
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

error(gpa): memory address 0x72ceb1940080 leaked: 
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

error(gpa): memory address 0x72ceb1940100 leaked: 
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

error(gpa): memory address 0x72ceb1940180 leaked: 
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

error(gpa): memory address 0x72ceb1940200 leaked: 
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

error(gpa): memory address 0x72ceb1940280 leaked: 
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

error(gpa): memory address 0x72ceb1940300 leaked: 
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

error(gpa): memory address 0x72ceb1940380 leaked: 
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

error(gpa): memory address 0x72ceb1940400 leaked: 
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

error(gpa): memory address 0x72ceb1940480 leaked: 
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

error(gpa): memory address 0x72ceb1940500 leaked: 
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

error(gpa): memory address 0x72ceb1940580 leaked: 
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

error(gpa): memory address 0x72ceb1980000 leaked: 
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

error(gpa): memory address 0x72ceb9240800 leaked: 
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

error(gpa): memory address 0x72ceb1920000 leaked: 
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

error(gpa): memory address 0x72ceb1900000 leaked: 
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

error(gpa): memory address 0x72ceb1962000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_operator_precedence
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
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/arithmetic/04_complex_expressions.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7f92de2e0000 leaked: 
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

error(gpa): memory address 0x7f92de2e0080 leaked: 
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

error(gpa): memory address 0x7f92de320000 leaked: 
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

error(gpa): memory address 0x7f92de2c0000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../04_complex_expressions
```

### Output Diff:
```diff
--- /tmp/tmp.obu32Wi4Jp	2025-08-31 12:01:48.117100568 +0300
+++ /tmp/tmp.xptGZ4YlC4	2025-08-31 12:01:48.117100568 +0300
@@ -1,78 +0,0 @@
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x76ed32b60000 leaked: 
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

error(gpa): memory address 0x76ed32b60080 leaked: 
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

error(gpa): memory address 0x76ed32b60100 leaked: 
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

error(gpa): memory address 0x76ed32b80000 leaked: 
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

error(gpa): memory address 0x76ed32b40000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_hello_world
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:10:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:10:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:10:49 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/basic/02_simple_arithmetic.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7c4995920000 leaked: 
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

error(gpa): memory address 0x7c4995920080 leaked: 
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

error(gpa): memory address 0x7c4995960000 leaked: 
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

error(gpa): memory address 0x7c4995900000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_simple_arithmetic
```

### Output Diff:
```diff
--- /tmp/tmp.H3IUILilY6	2025-08-31 12:01:49.215094572 +0300
+++ /tmp/tmp.3kjvfzttbG	2025-08-31 12:01:49.216094567 +0300
@@ -1,76 +0,0 @@
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:10:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:10:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:10:51 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/basic/03_variable_assignment.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x778d46740000 leaked: 
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

error(gpa): memory address 0x778d46740080 leaked: 
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

error(gpa): memory address 0x778d467e0400 leaked: 
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

error(gpa): memory address 0x778d46720000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_variable_assignment
```

### Output Diff:
```diff
--- /tmp/tmp.xvOEP8nfNW	2025-08-31 12:01:49.753091635 +0300
+++ /tmp/tmp.45AtdyInV8	2025-08-31 12:01:49.753091635 +0300
@@ -1,76 +0,0 @@
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
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:10:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:10:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:10:49 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:33:5 - Failed to parse statement
INFO: Recovered at statement keyword 'Sus' after skipping 5 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Synchronizing parser after error (context: synchronize)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:40:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 3
Semicolon recoveries: 5
Statement recoveries: 3
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 16
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7cb028d80000 leaked: 
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

error(gpa): memory address 0x7cb028d80100 leaked: 
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

error(gpa): memory address 0x7cb028d80180 leaked: 
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

error(gpa): memory address 0x7cb028d80200 leaked: 
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

error(gpa): memory address 0x7cb028d80280 leaked: 
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

error(gpa): memory address 0x7cb028da0000 leaked: 
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

error(gpa): memory address 0x7cb028da0200 leaked: 
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

error(gpa): memory address 0x7cb030680800 leaked: 
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

error(gpa): memory address 0x7cb028d60000 leaked: 
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

error(gpa): memory address 0x7cb028d40000 leaked: 
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
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:5:25 - Error parsing function statement
-INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:23:9 - Failed to parse statement
-INFO: Recovered at delimiter 'RightParen' after skipping 5 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:32:39 - Failed to parse statement
-INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
-INFO: Attempting additional statement recovery
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:10:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:10:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:10:16 - Error parsing function statement
INFO: Recovered at statement keyword 'Slay' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:21:1 - Failed to parse statement
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
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x70b7c44c0000 leaked: 
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

error(gpa): memory address 0x70b7c44c0080 leaked: 
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

error(gpa): memory address 0x70b7c44c0100 leaked: 
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

error(gpa): memory address 0x70b7c44c0180 leaked: 
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

error(gpa): memory address 0x70b7c44c0200 leaked: 
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

error(gpa): memory address 0x70b7c44c0280 leaked: 
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

error(gpa): memory address 0x70b7c4500000 leaked: 
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

error(gpa): memory address 0x70b7c44a0000 leaked: 
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

error(gpa): memory address 0x70b7c41e0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10ee8a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c0d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11ccc30 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2774:61: 0x1202c95 in parseWhileStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); stmt_ptr.* = stmt; try body.append(self.allocator, stmt_ptr);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:1001:68: 0x11ce9d1 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .While = try self.parseWhileStatement() };
                                                                   ^
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_fizzbuzz
```

### Output Diff:
```diff
--- /tmp/tmp.WjE6PLi9a6	2025-08-31 12:01:50.878085495 +0300
+++ /tmp/tmp.0KV5YyoYV3	2025-08-31 12:01:50.878085495 +0300
@@ -1,180 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:10:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:10:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at statement keyword 'Lowkey' after skipping 1 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:10:16 - Error parsing function statement
-INFO: Recovered at statement keyword 'Slay' after skipping 1 tokens
-INFO: Attempting additional statement recovery
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/02_fizzbuzz.csd:21:1 - Failed to parse statement
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
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/control_flow/01_if_statements.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7187a84a0000 leaked: 
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

error(gpa): memory address 0x7187a84a0080 leaked: 
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

error(gpa): memory address 0x7187a84c0000 leaked: 
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

error(gpa): memory address 0x7187a81e0000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_if_statements
```

### Output Diff:
```diff
--- /tmp/tmp.ov6RMgIEDa	2025-08-31 12:01:51.416082560 +0300
+++ /tmp/tmp.a793CS1IJp	2025-08-31 12:01:51.417082555 +0300
@@ -1,78 +0,0 @@
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
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/control_flow/02_loops.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7d0d2db40000 leaked: 
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

error(gpa): memory address 0x7d0d2db40080 leaked: 
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

error(gpa): memory address 0x7d0d2dbe0400 leaked: 
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

error(gpa): memory address 0x7d0d2db20000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_loops
```

### Output Diff:
```diff
--- /tmp/tmp.qXH6Oa5QOs	2025-08-31 12:01:51.952079636 +0300
+++ /tmp/tmp.o1Z7HHCt4K	2025-08-31 12:01:51.952079636 +0300
@@ -1,76 +0,0 @@
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7b9136da0000 leaked: 
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

error(gpa): memory address 0x7b9136da0080 leaked: 
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

error(gpa): memory address 0x7b9136da0100 leaked: 
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

error(gpa): memory address 0x7b9136da0180 leaked: 
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

error(gpa): memory address 0x7b9136da0200 leaked: 
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

error(gpa): memory address 0x7b9136da0280 leaked: 
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

error(gpa): memory address 0x7b9136da0300 leaked: 
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

error(gpa): memory address 0x7b9136da0380 leaked: 
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

error(gpa): memory address 0x7b9136da0400 leaked: 
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

error(gpa): memory address 0x7b9136da0480 leaked: 
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

error(gpa): memory address 0x7b9136da0500 leaked: 
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

error(gpa): memory address 0x7b9136da0580 leaked: 
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

error(gpa): memory address 0x7b9136da0600 leaked: 
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

error(gpa): memory address 0x7b9136da0680 leaked: 
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

error(gpa): memory address 0x7b9136da0700 leaked: 
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

error(gpa): memory address 0x7b9136da0780 leaked: 
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

error(gpa): memory address 0x7b913e680400 leaked: 
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

error(gpa): memory address 0x7b9136d80000 leaked: 
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

error(gpa): memory address 0x7b9136d60000 leaked: 
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

error(gpa): memory address 0x7b9136d40000 leaked: 
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

error(gpa): memory address 0x7b9136dc2000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_boundary_values
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
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:13:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:13:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:13:44 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd:27:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/edge_cases/02_empty_inputs.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x795db6280000 leaked: 
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

error(gpa): memory address 0x795db6280080 leaked: 
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

error(gpa): memory address 0x795db6280100 leaked: 
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

error(gpa): memory address 0x795db6280180 leaked: 
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

error(gpa): memory address 0x795db6280200 leaked: 
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

error(gpa): memory address 0x795db6320400 leaked: 
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

error(gpa): memory address 0x795db6260000 leaked: 
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

error(gpa): memory address 0x795db6240000 leaked: 
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_division_by_zero
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x74bb42660000 leaked: 
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

error(gpa): memory address 0x74bb42660080 leaked: 
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

error(gpa): memory address 0x74bb42660100 leaked: 
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

error(gpa): memory address 0x74bb42660180 leaked: 
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

error(gpa): memory address 0x74bb42660200 leaked: 
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

error(gpa): memory address 0x74bb42660280 leaked: 
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

error(gpa): memory address 0x74bb42660300 leaked: 
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

error(gpa): memory address 0x74bb42660380 leaked: 
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

error(gpa): memory address 0x74bb3adc0400 leaked: 
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

error(gpa): memory address 0x74bb42640000 leaked: 
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

error(gpa): memory address 0x74bb3ad80000 leaked: 
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

error(gpa): memory address 0x74bb3ada1000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_division_by_zero
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x754c25440000 leaked: 
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

error(gpa): memory address 0x754c25440080 leaked: 
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

error(gpa): memory address 0x754c25440100 leaked: 
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

error(gpa): memory address 0x754c25440180 leaked: 
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

error(gpa): memory address 0x754c25440200 leaked: 
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

error(gpa): memory address 0x754c25440280 leaked: 
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

error(gpa): memory address 0x754c25460000 leaked: 
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

error(gpa): memory address 0x754c1dba0000 leaked: 
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

error(gpa): memory address 0x754c1dbc0800 leaked: 
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

error(gpa): memory address 0x754c1db80000 leaked: 
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

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../02_undefined_variable
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

## Test: feature_control_flow_simple
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:6:44 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:21:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x758230660000 leaked: 
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

error(gpa): memory address 0x758230660080 leaked: 
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

error(gpa): memory address 0x758230700400 leaked: 
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

error(gpa): memory address 0x758230640000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../feature_control_flow_simple
```

### Output Diff:
```diff
--- /tmp/tmp.UHXrrmtvxX	2025-08-31 12:01:59.420038956 +0300
+++ /tmp/tmp.4hfRxXBBDL	2025-08-31 12:01:59.420038956 +0300
@@ -1,76 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:6:44 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_control_flow_simple.csd:21:1 - Failed to parse statement
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
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:38 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:19:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7a6d02da0000 leaked: 
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

error(gpa): memory address 0x7a6d02da0080 leaked: 
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

error(gpa): memory address 0x7a6d0a680400 leaked: 
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

error(gpa): memory address 0x7a6d02d80000 leaked: 
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
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:38 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:19:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
🔧 Initializing LLVM components...
✅ C library functions declared
✅ Builtin functions registered (yap)
✅ LLVM IR Pipeline initialized successfully
🚀 Starting complete LLVM compilation pipeline...
Error at unknown:7:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at unknown:7:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at unknown:7:38 - Error parsing function statement
Error at unknown:19:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
📦 Processing import: mathz
DEBUG: Loading and compiling CURSED module: mathz
DEBUG: Successfully read CURSED module stdlib/mathz/mod.csd (1357 bytes)
DEBUG: Successfully parsed CURSED module mathz (11 statements)
DEBUG: Compiling CURSED stdlib function: mathz.abs
DEBUG: Compiling CURSED stdlib function: mathz.max
DEBUG: Compiling CURSED stdlib function: mathz.min
DEBUG: Compiling CURSED stdlib function: mathz.add_two
DEBUG: Compiling CURSED stdlib function: mathz.subtract_two
DEBUG: Compiling CURSED stdlib function: mathz.multiply_two
DEBUG: Compiling CURSED stdlib function: mathz.power_int
DEBUG: Compiling CURSED stdlib function: mathz.factorial
DEBUG: Compiling CURSED stdlib function: mathz.is_even
DEBUG: Compiling CURSED stdlib function: mathz.is_odd
DEBUG: Compiling CURSED stdlib function: mathz.clamp
DEBUG: Successfully compiled CURSED module: mathz
📦 Processing import: vibez
DEBUG: Loading and compiling CURSED module: vibez
DEBUG: Successfully read CURSED module stdlib/vibez/mod.csd (655 bytes)
DEBUG: Successfully parsed CURSED module vibez (6 statements)
DEBUG: Compiling CURSED stdlib function: vibez.spill
thread 1725585 panic: reached unreachable code
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x71be6ac2a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x71be6ac2a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
```

### Output Diff:
```diff
--- /tmp/tmp.JErsQGWeX9	2025-08-31 12:06:46.944538811 +0300
+++ /tmp/tmp.TcZVRKWZhs	2025-08-31 12:06:46.946538801 +0300
@@ -1,3 +1,4 @@
+COMPILE_ERROR: Exit code 0
 Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
 Error at /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd:7:10 - Synchronizing parser after error (context: synchronize)
 INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
@@ -12,87 +13,100 @@
 Delimiter recoveries: 0
 Total tokens skipped: 4
 ================================
-ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
-❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/features/feature_stdlib_integration.csd: error.ModuleNotFound
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
+🔧 Initializing LLVM components...
+✅ C library functions declared
+✅ Builtin functions registered (yap)
+✅ LLVM IR Pipeline initialized successfully
+🚀 Starting complete LLVM compilation pipeline...
+Error at unknown:7:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
+Error at unknown:7:10 - Synchronizing parser after error (context: synchronize)
+INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
+Error at unknown:7:38 - Error parsing function statement
+Error at unknown:19:1 - Failed to parse statement
+
+=== Error Recovery Statistics ===
+Total errors encountered: 2
+Semicolon recoveries: 3
+Statement recoveries: 2
+Expression recoveries: 0
+Delimiter recoveries: 0
+Total tokens skipped: 4
+================================
+📦 Processing import: mathz
+DEBUG: Loading and compiling CURSED module: mathz
+DEBUG: Successfully read CURSED module stdlib/mathz/mod.csd (1357 bytes)
+DEBUG: Successfully parsed CURSED module mathz (11 statements)
+DEBUG: Compiling CURSED stdlib function: mathz.abs
+DEBUG: Compiling CURSED stdlib function: mathz.max
+DEBUG: Compiling CURSED stdlib function: mathz.min
+DEBUG: Compiling CURSED stdlib function: mathz.add_two
+DEBUG: Compiling CURSED stdlib function: mathz.subtract_two
+DEBUG: Compiling CURSED stdlib function: mathz.multiply_two
+DEBUG: Compiling CURSED stdlib function: mathz.power_int
+DEBUG: Compiling CURSED stdlib function: mathz.factorial
+DEBUG: Compiling CURSED stdlib function: mathz.is_even
+DEBUG: Compiling CURSED stdlib function: mathz.is_odd
+DEBUG: Compiling CURSED stdlib function: mathz.clamp
+DEBUG: Successfully compiled CURSED module: mathz
+📦 Processing import: vibez
+DEBUG: Loading and compiling CURSED module: vibez
+DEBUG: Successfully read CURSED module stdlib/vibez/mod.csd (655 bytes)
+DEBUG: Successfully parsed CURSED module vibez (6 statements)
+DEBUG: Compiling CURSED stdlib function: vibez.spill
+thread 1725585 panic: reached unreachable code
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

## Test: 01_simple_function
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:15:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:15:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:15:47 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd:25:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/functions/01_simple_function.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7cb695b40000 leaked: 
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

error(gpa): memory address 0x7cb695b40100 leaked: 
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

error(gpa): memory address 0x7cb695b40180 leaked: 
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

error(gpa): memory address 0x7cb695b40200 leaked: 
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

error(gpa): memory address 0x7cb695b40280 leaked: 
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

error(gpa): memory address 0x7cb695b40300 leaked: 
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

error(gpa): memory address 0x7cb695b40380 leaked: 
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

error(gpa): memory address 0x7cb695ba0200 leaked: 
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

error(gpa): memory address 0x7cb695be0400 leaked: 
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

error(gpa): memory address 0x7cb695b20000 leaked: 
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

error(gpa): memory address 0x7cb695b00000 leaked: 
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_recursive_function
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:20:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:20:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:20:50 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd:31:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/functions/02_recursive_function.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x79ff63f40000 leaked: 
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

error(gpa): memory address 0x79ff63f40080 leaked: 
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

error(gpa): memory address 0x79ff63f40100 leaked: 
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

error(gpa): memory address 0x79ff63f40180 leaked: 
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

error(gpa): memory address 0x79ff63f40200 leaked: 
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

error(gpa): memory address 0x79ff63f40280 leaked: 
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

error(gpa): memory address 0x79ff63f40300 leaked: 
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

error(gpa): memory address 0x79ff63f40380 leaked: 
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

error(gpa): memory address 0x79ff63f40400 leaked: 
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

error(gpa): memory address 0x79ff63f40480 leaked: 
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

error(gpa): memory address 0x79ff63f40500 leaked: 
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

error(gpa): memory address 0x79ff63f80000 leaked: 
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

error(gpa): memory address 0x79ff6b840800 leaked: 
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

error(gpa): memory address 0x79ff63f20000 leaked: 
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

error(gpa): memory address 0x79ff63f00000 leaked: 
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 03_nested_function_calls
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/functions/03_nested_function_calls.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x75425a120000 leaked: 
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

error(gpa): memory address 0x75425a120080 leaked: 
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

error(gpa): memory address 0x75425a120100 leaked: 
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

error(gpa): memory address 0x75425a120180 leaked: 
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

error(gpa): memory address 0x75425a120200 leaked: 
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

error(gpa): memory address 0x75425a120280 leaked: 
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

error(gpa): memory address 0x75425a120300 leaked: 
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

error(gpa): memory address 0x75425a120380 leaked: 
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

error(gpa): memory address 0x75425a120400 leaked: 
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

error(gpa): memory address 0x75425a120480 leaked: 
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

error(gpa): memory address 0x75425a120500 leaked: 
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

error(gpa): memory address 0x75425a120580 leaked: 
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

error(gpa): memory address 0x75425a120600 leaked: 
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

error(gpa): memory address 0x75425a120680 leaked: 
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

error(gpa): memory address 0x75425a120700 leaked: 
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

error(gpa): memory address 0x75425a120780 leaked: 
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

error(gpa): memory address 0x75425a120800 leaked: 
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

error(gpa): memory address 0x75425a120880 leaked: 
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

error(gpa): memory address 0x75425a120900 leaked: 
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

error(gpa): memory address 0x75425a120980 leaked: 
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

error(gpa): memory address 0x75425a120a00 leaked: 
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

error(gpa): memory address 0x75425a120a80 leaked: 
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

error(gpa): memory address 0x75425a120b00 leaked: 
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

error(gpa): memory address 0x75425a120b80 leaked: 
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

error(gpa): memory address 0x75425a120c00 leaked: 
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

error(gpa): memory address 0x75425a160000 leaked: 
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

error(gpa): memory address 0x75425a1e0800 leaked: 
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

error(gpa): memory address 0x75425a100000 leaked: 
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

error(gpa): memory address 0x75425a0e0000 leaked: 
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

error(gpa): memory address 0x75425a142000 leaked: 
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
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 04_function_parameters
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/functions/04_function_parameters.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7a73938c0000 leaked: 
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

error(gpa): memory address 0x7a73938c0080 leaked: 
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

error(gpa): memory address 0x7a73938c0100 leaked: 
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

error(gpa): memory address 0x7a73938c0180 leaked: 
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

error(gpa): memory address 0x7a73938c0280 leaked: 
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

error(gpa): memory address 0x7a73938c0300 leaked: 
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

error(gpa): memory address 0x7a73938c0380 leaked: 
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

error(gpa): memory address 0x7a73938c0400 leaked: 
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

error(gpa): memory address 0x7a73938c0480 leaked: 
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

error(gpa): memory address 0x7a73938c0500 leaked: 
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

error(gpa): memory address 0x7a73938c0580 leaked: 
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

error(gpa): memory address 0x7a73938c0600 leaked: 
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

error(gpa): memory address 0x7a73938c0680 leaked: 
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

error(gpa): memory address 0x7a73938c0700 leaked: 
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

error(gpa): memory address 0x7a73938c0780 leaked: 
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

error(gpa): memory address 0x7a73938c0800 leaked: 
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

error(gpa): memory address 0x7a73938c0880 leaked: 
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

error(gpa): memory address 0x7a73938c0900 leaked: 
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

error(gpa): memory address 0x7a73938c0980 leaked: 
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

error(gpa): memory address 0x7a7393900000 leaked: 
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

error(gpa): memory address 0x7a7393900200 leaked: 
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

error(gpa): memory address 0x7a73938a0000 leaked: 
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

error(gpa): memory address 0x7a73935e0000 leaked: 
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

error(gpa): memory address 0x7a73938e2000 leaked: 
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
error(gpa): memory address 0x76c7c02e0100 leaked: 
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

error(gpa): memory address 0x76c7c02e0180 leaked: 
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

error(gpa): memory address 0x76c7c02e0200 leaked: 
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

error(gpa): memory address 0x76c7c02a0200 leaked: 
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

error(gpa): memory address 0x76c7c0260000 leaked: 
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

error(gpa): memory address 0x76c7c0240800 leaked: 
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
--- /tmp/tmp.WU9jyze5tY	2025-08-31 12:06:49.995523462 +0300
+++ /tmp/tmp.q0NnejOiiV	2025-08-31 12:06:49.995523462 +0300
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
error(gpa): memory address 0x72baad960000 leaked: 
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

error(gpa): memory address 0x72baad960080 leaked: 
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

error(gpa): memory address 0x72baad960100 leaked: 
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

error(gpa): memory address 0x72baad9a0400 leaked: 
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

error(gpa): memory address 0x72baad980000 leaked: 
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

error(gpa): memory address 0x72baad9e0800 leaked: 
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

error(gpa): memory address 0x72baad940000 leaked: 
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
--- /tmp/tmp.cjwvA8rHsS	2025-08-31 12:06:50.554520651 +0300
+++ /tmp/tmp.2UDfNexyPH	2025-08-31 12:06:50.554520651 +0300
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
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:7:17 - Expected ')'. Expected .RightParen, got .Normie (context: consume)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:7:17 - Error parsing variable declaration
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:7:23 - Failed to parse statement
INFO: Recovered at semicolon after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:11:23 - Expected ')'. Expected .RightParen, got .Normie (context: consume)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:11:23 - Error parsing variable declaration
INFO: Recovered at delimiter 'RightParen' after skipping 7 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:11:49 - Failed to parse statement
INFO: Recovered at statement keyword 'Sus' after skipping 4 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:14:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:14:1 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at statement keyword 'Sus' after skipping 1 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:16:1 - Failed to parse statement
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:23:1 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:23:1 - Synchronizing parser after error (context: synchronize)
Error at /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd:23:1 - Failed to parse statement

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
SELF-HOSTING: Please implement stdlib/a/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/parser_fixes/03_corrected_function_parameters.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x737755240000 leaked: 
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

error(gpa): memory address 0x737755240080 leaked: 
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

error(gpa): memory address 0x737755240100 leaked: 
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

error(gpa): memory address 0x737755240180 leaked: 
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

error(gpa): memory address 0x737755240200 leaked: 
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

error(gpa): memory address 0x737755240280 leaked: 
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

error(gpa): memory address 0x737755240300 leaked: 
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

error(gpa): memory address 0x7377552c0200 leaked: 
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

error(gpa): memory address 0x737755260000 leaked: 
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

error(gpa): memory address 0x73774d9e0000 leaked: 
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

error(gpa): memory address 0x73774d9c0000 leaked: 
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/performance/01_recursive_depth.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x712cdeb60000 leaked: 
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

error(gpa): memory address 0x712cdeb60080 leaked: 
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

error(gpa): memory address 0x712cdeb60100 leaked: 
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

error(gpa): memory address 0x712cdeb60180 leaked: 
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

error(gpa): memory address 0x712cdeb60200 leaked: 
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

error(gpa): memory address 0x712cdeb60280 leaked: 
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

error(gpa): memory address 0x712cdeb60300 leaked: 
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

error(gpa): memory address 0x712cdeb60380 leaked: 
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

error(gpa): memory address 0x712cdeb60400 leaked: 
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

error(gpa): memory address 0x712cdeb60480 leaked: 
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

error(gpa): memory address 0x712cdeb60500 leaked: 
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

error(gpa): memory address 0x712cdeb60580 leaked: 
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

error(gpa): memory address 0x712cdeb60600 leaked: 
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

error(gpa): memory address 0x712cdeb60680 leaked: 
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

error(gpa): memory address 0x712cdeb60700 leaked: 
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

error(gpa): memory address 0x712cdeb60780 leaked: 
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

error(gpa): memory address 0x712cdeb60800 leaked: 
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

error(gpa): memory address 0x712ce6440400 leaked: 
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

error(gpa): memory address 0x712cdeb40000 leaked: 
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

error(gpa): memory address 0x712cdeb20000 leaked: 
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

error(gpa): memory address 0x712cdeb00000 leaked: 
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

error(gpa): memory address 0x712cdeb82000 leaked: 
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/performance/02_computation_intensive.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7e539f4e0000 leaked: 
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

error(gpa): memory address 0x7e539f4e0080 leaked: 
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

error(gpa): memory address 0x7e539f4e0100 leaked: 
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

error(gpa): memory address 0x7e539f4e0180 leaked: 
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

error(gpa): memory address 0x7e539f4e0200 leaked: 
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

error(gpa): memory address 0x7e539f4e0280 leaked: 
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

error(gpa): memory address 0x7e539f4e0380 leaked: 
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

error(gpa): memory address 0x7e539f4e0400 leaked: 
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

error(gpa): memory address 0x7e539f4e0480 leaked: 
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
/home/ghuntley/cursed/src-zig/parser.zig:2786:37: 0x1203173 in parseWhileStatement (cursed_compiler_main.zig)
                try stmt_body.append(self.allocator, stmt);
                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1001:68: 0x11ce9d1 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .While = try self.parseWhileStatement() };
                                                                   ^

error(gpa): memory address 0x7e539f4e0500 leaked: 
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

error(gpa): memory address 0x7e539f4e0580 leaked: 
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

error(gpa): memory address 0x7e539f4e0600 leaked: 
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

error(gpa): memory address 0x7e539f4e0680 leaked: 
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

error(gpa): memory address 0x7e539f4e0700 leaked: 
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

error(gpa): memory address 0x7e539f4e0780 leaked: 
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

error(gpa): memory address 0x7e539f4e0800 leaked: 
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

error(gpa): memory address 0x7e539f4e0880 leaked: 
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

error(gpa): memory address 0x7e539f4e0900 leaked: 
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

error(gpa): memory address 0x7e539f4e0980 leaked: 
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

error(gpa): memory address 0x7e539f4e0a00 leaked: 
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

error(gpa): memory address 0x7e539f560200 leaked: 
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

error(gpa): memory address 0x7e539f520000 leaked: 
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

error(gpa): memory address 0x7e539f5a0800 leaked: 
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

error(gpa): memory address 0x7e539f4c0000 leaked: 
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

error(gpa): memory address 0x7e539f4a0000 leaked: 
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
/home/ghuntley/cursed/src-zig/parser.zig:1220:49: 0x11fe74b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7e539f502000 leaked: 
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
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.csd:13:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.csd:13:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.csd:13:38 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.csd:28:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/regression/regression_memory_management.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7a0a83040000 leaked: 
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

error(gpa): memory address 0x7a0a83040080 leaked: 
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

error(gpa): memory address 0x7a0a83040100 leaked: 
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

error(gpa): memory address 0x7a0a83040180 leaked: 
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

error(gpa): memory address 0x7a0a83040200 leaked: 
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

error(gpa): memory address 0x7a0a83060000 leaked: 
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

error(gpa): memory address 0x7a0a7b7c0000 leaked: 
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

error(gpa): memory address 0x7a0a7b7a0000 leaked: 
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

```

### Compiled Output:
```
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../regression_memory_management
```

---

## Test: regression_parser_precedence
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/regression/regression_parser_precedence.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7cefb6560000 leaked: 
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

error(gpa): memory address 0x7cefb6560080 leaked: 
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

error(gpa): memory address 0x7cefb6560100 leaked: 
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

error(gpa): memory address 0x7cefb6560180 leaked: 
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

error(gpa): memory address 0x7cefb6560200 leaked: 
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

error(gpa): memory address 0x7cefb6560280 leaked: 
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

error(gpa): memory address 0x7cefb6560300 leaked: 
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

error(gpa): memory address 0x7cefb6560380 leaked: 
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

error(gpa): memory address 0x7cefb6560400 leaked: 
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

error(gpa): memory address 0x7cefb6560480 leaked: 
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

error(gpa): memory address 0x7cefbde40400 leaked: 
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

error(gpa): memory address 0x7cefb6540000 leaked: 
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

error(gpa): memory address 0x7cefb6520000 leaked: 
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

error(gpa): memory address 0x7cefb6500000 leaked: 
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

error(gpa): memory address 0x7cefb6582000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../regression_parser_precedence
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:30:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x77b7e39e0000 leaked: 
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

error(gpa): memory address 0x77b7e39e0080 leaked: 
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

error(gpa): memory address 0x77b7eb260000 leaked: 
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

error(gpa): memory address 0x77b7e39c0000 leaked: 
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:30:1 - Failed to parse statement

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
Error at unknown:11:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at unknown:11:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at unknown:11:44 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
INFO: Attempting additional statement recovery
Error at unknown:30:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 5
================================
📦 Processing import: mathz
DEBUG: Loading and compiling CURSED module: mathz
DEBUG: Successfully read CURSED module stdlib/mathz/mod.csd (1357 bytes)
DEBUG: Successfully parsed CURSED module mathz (11 statements)
DEBUG: Compiling CURSED stdlib function: mathz.abs
DEBUG: Compiling CURSED stdlib function: mathz.max
DEBUG: Compiling CURSED stdlib function: mathz.min
DEBUG: Compiling CURSED stdlib function: mathz.add_two
DEBUG: Compiling CURSED stdlib function: mathz.subtract_two
DEBUG: Compiling CURSED stdlib function: mathz.multiply_two
DEBUG: Compiling CURSED stdlib function: mathz.power_int
DEBUG: Compiling CURSED stdlib function: mathz.factorial
DEBUG: Compiling CURSED stdlib function: mathz.is_even
DEBUG: Compiling CURSED stdlib function: mathz.is_odd
DEBUG: Compiling CURSED stdlib function: mathz.clamp
DEBUG: Successfully compiled CURSED module: mathz
📦 Processing import: vibez
DEBUG: Loading and compiling CURSED module: vibez
DEBUG: Successfully read CURSED module stdlib/vibez/mod.csd (655 bytes)
DEBUG: Successfully parsed CURSED module vibez (6 statements)
DEBUG: Compiling CURSED stdlib function: vibez.spill
Segmentation fault at address 0x7988d9de0094
/snap/zig/14937/lib/std/hash/wyhash.zig:115:37: 0x114ac0e in hash (std.zig)
            self.a = (@as(u64, input[0]) << 16) | (@as(u64, input[input.len >> 1]) << 8) | input[input.len - 1];
                                    ^
/snap/zig/14937/lib/std/hash_map.zig:90:32: 0x12e771b in hashString (std.zig)
    return std.hash.Wyhash.hash(0, s);
                               ^
/snap/zig/14937/lib/std/hash_map.zig:77:26: 0x1292f29 in hash (std.zig)
        return hashString(s);
                         ^
/snap/zig/14937/lib/std/hash_map.zig:875:40: 0x1322cf0 in putAssumeCapacityNoClobberContext (std.zig)
            const hash: Hash = ctx.hash(key);
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
../sysdeps/nptl/libc_start_call_main.h:58:16: 0x7988e182a1c9 in __libc_start_call_main (../sysdeps/x86/libc-start.c)
../csu/libc-start.c:360:3: 0x7988e182a28a in __libc_start_main_impl (../sysdeps/x86/libc-start.c)
???:?:?: 0x1445684 in ??? (???)
???:?:?: 0x0 in ??? (???)
timeout: the monitored command dumped core
```

### Output Diff:
```diff
--- /tmp/tmp.3XHbpFXqwm	2025-08-31 12:06:56.344491550 +0300
+++ /tmp/tmp.aZxkBjKjxd	2025-08-31 12:06:56.346491540 +0300
@@ -1,3 +1,4 @@
+COMPILE_ERROR: Exit code 0
 Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
 Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd:11:10 - Synchronizing parser after error (context: synchronize)
 INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
@@ -14,87 +15,108 @@
 Delimiter recoveries: 0
 Total tokens skipped: 5
 ================================
-ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
-❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd: error.ModuleNotFound
-Executing 0 deferred statements
-error(gpa): memory address 0xADDRESS leaked: 
-/snap/zig/14937/lib/std/array_list.zig:1231:56: 0xADDRESS in ensureTotalCapacityPrecise (std.zig)
-                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
+🔧 Initializing LLVM components...
+✅ C library functions declared
+✅ Builtin functions registered (yap)
+✅ LLVM IR Pipeline initialized successfully
+🚀 Starting complete LLVM compilation pipeline...
+Error at unknown:11:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
+Error at unknown:11:10 - Synchronizing parser after error (context: synchronize)
+INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
+Error at unknown:11:44 - Error parsing function statement
+INFO: Recovered at delimiter 'RightParen' after skipping 1 tokens
+INFO: Attempting additional statement recovery
+Error at unknown:30:1 - Failed to parse statement
+
+=== Error Recovery Statistics ===
+Total errors encountered: 2
+Semicolon recoveries: 3
+Statement recoveries: 2
+Expression recoveries: 0
+Delimiter recoveries: 0
+Total tokens skipped: 5
+================================
+📦 Processing import: mathz
+DEBUG: Loading and compiling CURSED module: mathz
+DEBUG: Successfully read CURSED module stdlib/mathz/mod.csd (1357 bytes)
+DEBUG: Successfully parsed CURSED module mathz (11 statements)
+DEBUG: Compiling CURSED stdlib function: mathz.abs
+DEBUG: Compiling CURSED stdlib function: mathz.max
+DEBUG: Compiling CURSED stdlib function: mathz.min
+DEBUG: Compiling CURSED stdlib function: mathz.add_two
+DEBUG: Compiling CURSED stdlib function: mathz.subtract_two
+DEBUG: Compiling CURSED stdlib function: mathz.multiply_two
+DEBUG: Compiling CURSED stdlib function: mathz.power_int
+DEBUG: Compiling CURSED stdlib function: mathz.factorial
+DEBUG: Compiling CURSED stdlib function: mathz.is_even
+DEBUG: Compiling CURSED stdlib function: mathz.is_odd
+DEBUG: Compiling CURSED stdlib function: mathz.clamp
+DEBUG: Successfully compiled CURSED module: mathz
+📦 Processing import: vibez
+DEBUG: Loading and compiling CURSED module: vibez
+DEBUG: Successfully read CURSED module stdlib/vibez/mod.csd (655 bytes)
+DEBUG: Successfully parsed CURSED module vibez (6 statements)
+DEBUG: Compiling CURSED stdlib function: vibez.spill
+Segmentation fault at address 0xADDRESS
+/snap/zig/14937/lib/std/hash/wyhash.zig:115:37: 0xADDRESS in hash (std.zig)
+            self.a = (@as(u64, input[0]) << 16) | (@as(u64, input[input.len >> 1]) << 8) | input[input.len - 1];
+                                    ^
+/snap/zig/14937/lib/std/hash_map.zig:90:32: 0xADDRESS in hashString (std.zig)
+    return std.hash.Wyhash.hash(0, s);
+                               ^
+/snap/zig/14937/lib/std/hash_map.zig:77:26: 0xADDRESS in hash (std.zig)
+        return hashString(s);
+                         ^
+/snap/zig/14937/lib/std/hash_map.zig:875:40: 0xADDRESS in putAssumeCapacityNoClobberContext (std.zig)
+            const hash: Hash = ctx.hash(key);
+                                       ^
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
+/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig:260:28: 0xADDRESS in compileSource (cursed_compiler_main.zig)
+        try self.generateIR(program);
                            ^
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
-                           ^
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:7:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:7:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:7:46 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd:26:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'stringz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/stringz/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7bcc0cc40000 leaked: 
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

error(gpa): memory address 0x7bcc0cc40080 leaked: 
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

error(gpa): memory address 0x7bcc0cc60000 leaked: 
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

error(gpa): memory address 0x7bcc05380000 leaked: 
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/03_mathz_advanced.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7a0f91fe0000 leaked: 
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

error(gpa): memory address 0x7a0f91fe0080 leaked: 
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

error(gpa): memory address 0x7a0f91fe0100 leaked: 
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

error(gpa): memory address 0x7a0f91fe0180 leaked: 
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

error(gpa): memory address 0x7a0f91fe0200 leaked: 
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

error(gpa): memory address 0x7a0f91fe0280 leaked: 
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

error(gpa): memory address 0x7a0f91fe0300 leaked: 
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

error(gpa): memory address 0x7a0f91fe0380 leaked: 
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

error(gpa): memory address 0x7a0f91fe0400 leaked: 
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

error(gpa): memory address 0x7a0f91fe0480 leaked: 
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

error(gpa): memory address 0x7a0f91fe0500 leaked: 
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

error(gpa): memory address 0x7a0f91fe0580 leaked: 
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

error(gpa): memory address 0x7a0f91fe0600 leaked: 
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

error(gpa): memory address 0x7a0f91fe0680 leaked: 
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

error(gpa): memory address 0x7a0f91fe0700 leaked: 
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

error(gpa): memory address 0x7a0f91fe0780 leaked: 
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

error(gpa): memory address 0x7a0f91fe0800 leaked: 
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

error(gpa): memory address 0x7a0f91fe0880 leaked: 
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

error(gpa): memory address 0x7a0f91fe0900 leaked: 
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

error(gpa): memory address 0x7a0f91fe0980 leaked: 
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

error(gpa): memory address 0x7a0f91fe0a00 leaked: 
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

error(gpa): memory address 0x7a0f91fe0a80 leaked: 
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

error(gpa): memory address 0x7a0f91fe0b00 leaked: 
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

error(gpa): memory address 0x7a0f91fe0b80 leaked: 
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

error(gpa): memory address 0x7a0f99840000 leaked: 
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

error(gpa): memory address 0x7a0f91fc0000 leaked: 
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

error(gpa): memory address 0x7a0f91fa0000 leaked: 
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

error(gpa): memory address 0x7a0f91f80000 leaked: 
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

error(gpa): memory address 0x7a0f99864000 leaked: 
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
-SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:7:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:7:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:7:49 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd:26:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 10
================================
ERROR: No CURSED stdlib implementation found for module 'collections': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/collections/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7f73169e0000 leaked: 
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

error(gpa): memory address 0x7f73169e0080 leaked: 
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

error(gpa): memory address 0x7f731e260000 leaked: 
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

error(gpa): memory address 0x7f73169c0000 leaked: 
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
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/strings/01_string_operations.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x77857a440000 leaked: 
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

error(gpa): memory address 0x77857a440080 leaked: 
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

error(gpa): memory address 0x77857a460000 leaked: 
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

error(gpa): memory address 0x778572b80000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../01_string_operations
```

### Output Diff:
```diff
--- /tmp/tmp.xwDM6aZBmY	2025-08-31 12:02:18.520570796 +0300
+++ /tmp/tmp.Xl7WkOMxMe	2025-08-31 12:02:18.520570796 +0300
@@ -1,76 +0,0 @@
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:6:44 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:18:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7755a7a60000 leaked: 
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

error(gpa): memory address 0x7755a7a60080 leaked: 
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

error(gpa): memory address 0x7755a01c0400 leaked: 
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

error(gpa): memory address 0x7755a7a40000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../validation_basic_syntax
```

### Output Diff:
```diff
--- /tmp/tmp.gnIdnLiiO4	2025-08-31 12:02:19.058932464 +0300
+++ /tmp/tmp.lAkv4KdUmU	2025-08-31 12:02:19.058932464 +0300
@@ -1,76 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:6:44 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_basic_syntax.csd:18:1 - Failed to parse statement
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
**Status:** COMPILE_ERROR
**Details:** Compilation failed

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:6:10 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:6:43 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:21:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 4
================================
ERROR: No CURSED stdlib implementation found for module 'vibez': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/vibez/mod.csd for true self-hosting
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd: error.ModuleNotFound
Executing 0 deferred statements
error(gpa): memory address 0x7b0a00d60000 leaked: 
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

error(gpa): memory address 0x7b0a00d60080 leaked: 
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

error(gpa): memory address 0x7b0a08620400 leaked: 
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

error(gpa): memory address 0x7b0a00d40000 leaked: 
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../validation_type_system
```

### Output Diff:
```diff
--- /tmp/tmp.E3iBSxOeCg	2025-08-31 12:02:19.595929562 +0300
+++ /tmp/tmp.41BfUxumfF	2025-08-31 12:02:19.595929562 +0300
@@ -1,76 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:6:10 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:6:10 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 4 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:6:43 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/validation/validation_type_system.csd:21:1 - Failed to parse statement
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

