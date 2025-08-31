# CURSED Interpreter vs Compiler Parity Test Report

**Generated:** Sun Aug 31 10:58:57 AM EEST 2025
**Test Suite Version:** 1.0.0
**CURSED Compiler:** /home/ghuntley/cursed/test_suite/../zig-out/bin/cursed-compiler

## Executive Summary

- **Total Tests:** 26
- **Passed:** 0
- **Failed:** 21
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
- **performance:** 2 tests
- **stdlib:** 4 tests
- **strings:** 1 tests

## Recommendations

- 🚨 **High Priority:** Fix compilation failures (5 tests)
- 📋 **Low Priority:** Investigate output differences (21 tests)

## Detailed Test Results

## Test: 01_mixed_types
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc56934738 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x704d6bfc0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x704d73840200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0x1216df8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0x11d56fe in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x704d73880800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.zF9katJ8eZ	2025-08-31 10:58:45.331514814 +0300
+++ /tmp/tmp.Hm5udgh2sm	2025-08-31 10:58:45.332514810 +0300
@@ -1,78 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc56934738 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:6:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:6:53 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:23:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 3
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd45e64fa8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.9cbvrM2bi1	2025-08-31 10:58:45.679513432 +0300
+++ /tmp/tmp.nFbQHFtLYR	2025-08-31 10:58:45.679513432 +0300
@@ -1,18 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:6:53 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/02_edge_cases.csd:23:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd45e64fa8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 03_operator_precedence
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:6:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:6:51 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:21:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 3
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc927b2458 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.jHO8oBgtjk	2025-08-31 10:58:46.026512053 +0300
+++ /tmp/tmp.TFlCnWBecr	2025-08-31 10:58:46.026512053 +0300
@@ -1,18 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:6:51 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/arithmetic/03_operator_precedence.csd:21:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc927b2458 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 04_complex_expressions
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc9b0068d8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x785aa7a60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x785aa7a80200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0x1216df8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0x11d56fe in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x785aa7ac0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.co178KIDnc	2025-08-31 10:58:46.679509459 +0300
+++ /tmp/tmp.dOILds10ZL	2025-08-31 10:58:46.679509459 +0300
@@ -1,80 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc9b0068d8 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:6:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:6:39 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:7:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 3
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe031f1b78 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.rV6LhsdwGA	2025-08-31 10:58:47.028508073 +0300
+++ /tmp/tmp.sqKWqJ6Tlc	2025-08-31 10:58:47.029508068 +0300
@@ -1,18 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:6:39 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/basic/01_hello_world.csd:7:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe031f1b78 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_simple_arithmetic
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdd7b5c4c8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7c8e98b60000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7c8e98ba0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0x1216df8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0x11d56fe in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7c8e98be0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.ZSvzUO7Cc1	2025-08-31 10:58:47.671505518 +0300
+++ /tmp/tmp.XlgKEh5M7L	2025-08-31 10:58:47.671505518 +0300
@@ -1,78 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdd7b5c4c8 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff7521b618 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x790ad8de0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x790ae06a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0x1216df8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0x11d56fe in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x790ae0640000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.8A6RLTTHcJ	2025-08-31 10:58:48.319575922 +0300
+++ /tmp/tmp.vNVQvytGst	2025-08-31 10:58:48.319575922 +0300
@@ -1,78 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff7521b618 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:37:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:37:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:37:22 - Failed to parse statement
INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
INFO: Attempting additional statement recovery

=== Error Recovery Statistics ===
Total errors encountered: 4
Semicolon recoveries: 6
Statement recoveries: 4
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 19
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc4b078958 (parent: *interpreter.Environment@0)
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
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffc4b078958
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffc4b078958
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'b'
DEBUG: Found 'b' in environment@interpreter.Environment@7ffc4b078958
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffc4b078958
DEBUG: Evaluating expression type: Binary
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'a'
DEBUG: Found 'a' in environment@interpreter.Environment@7ffc4b078958
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'c'
DEBUG: Found 'c' in environment@interpreter.Environment@7ffc4b078958
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7e0b315e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128790e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247055 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a72 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2a1c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:505:42: 0x11b2269 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7e0b38ea0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1245:61: 0x1204d80 in parseLetStatement (cursed_compiler_main.zig)
            const init_ptr = try self.arena_allocator.create(Expression);
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:938:60: 0x11d3286 in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Let = self.parseLetStatement() catch |parse_err| {
                                                           ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

error(gpa): memory address 0x7e0b38e60000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e22 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7e0b38ee1000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e22 in parseProgram (cursed_compiler_main.zig)
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
```

### Output Diff:
```diff
--- /tmp/tmp.f53eifP68e	2025-08-31 10:58:48.955500417 +0300
+++ /tmp/tmp.Wudfy70Bba	2025-08-31 10:58:48.955500417 +0300
@@ -1,141 +0,0 @@
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
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:37:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:37:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/complex/01_nested_operations.csd:37:22 - Failed to parse statement
-INFO: Recovered at delimiter 'RightParen' after skipping 6 tokens
-INFO: Attempting additional statement recovery
-
-=== Error Recovery Statistics ===
-Total errors encountered: 4
-Semicolon recoveries: 6
-Statement recoveries: 4
-Expression recoveries: 0
-Delimiter recoveries: 0
-Total tokens skipped: 19
-================================
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc4b078958 (parent: *interpreter.Environment@0)
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
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'a'
-DEBUG: Found 'a' in environment@interpreter.Environment@7ffc4b078958
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'b'
-DEBUG: Found 'b' in environment@interpreter.Environment@7ffc4b078958
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'b'
-DEBUG: Found 'b' in environment@interpreter.Environment@7ffc4b078958
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'c'
-DEBUG: Found 'c' in environment@interpreter.Environment@7ffc4b078958
-DEBUG: Evaluating expression type: Binary
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'a'
-DEBUG: Found 'a' in environment@interpreter.Environment@7ffc4b078958
-DEBUG: Evaluating expression type: Identifier
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffc4b078958 with 4 variables for 'c'
-DEBUG: Found 'c' in environment@interpreter.Environment@7ffc4b078958
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
-/home/ghuntley/cursed/src-zig/parser.zig:505:42: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1245:61: 0xADDRESS in parseLetStatement (cursed_compiler_main.zig)
-            const init_ptr = try self.arena_allocator.create(Expression);
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:938:60: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
```

---

## Test: 02_fizzbuzz
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcb5c10908 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.CZcmIrzqZz	2025-08-31 10:58:49.301499043 +0300
+++ /tmp/tmp.dNeFNfhF5C	2025-08-31 10:58:49.302499039 +0300
@@ -1,17 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcb5c10908 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_if_statements
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff28003de8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x7d1561900000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7d1561940200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0x1216df8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0x11d56fe in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x7d1561980800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.n2cOaY6cHr	2025-08-31 10:58:49.964496409 +0300
+++ /tmp/tmp.HvdGRsRqxH	2025-08-31 10:58:49.964496409 +0300
@@ -1,80 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff28003de8 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff141f5c58 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x78538a660000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x78538a6e0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0x1216df8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0x11d56fe in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x78538a680000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.HMaszYtWQI	2025-08-31 10:58:50.595493903 +0300
+++ /tmp/tmp.ZookWGevKB	2025-08-31 10:58:50.595493903 +0300
@@ -1,78 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff141f5c58 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:6:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:6:47 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:24:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 3
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc23e78528 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.ePqdKZ0ZVs	2025-08-31 10:58:50.947492505 +0300
+++ /tmp/tmp.TJzpVfDGrB	2025-08-31 10:58:50.947492505 +0300
@@ -1,18 +0,0 @@
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:6:11 - Synchronizing parser after error (context: synchronize)
-INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:6:47 - Error parsing function statement
-Error at /home/ghuntley/cursed/test_suite/test_programs/edge_cases/01_boundary_values.csd:24:1 - Failed to parse statement
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc23e78528 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_empty_inputs
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffddf945438 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.C8JIP8fk0V	2025-08-31 10:58:51.300676359 +0300
+++ /tmp/tmp.l1Da31y1uO	2025-08-31 10:58:51.300676359 +0300
@@ -1,15 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffddf945438 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 01_division_by_zero
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:6:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:6:48 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/01_division_by_zero.csd:15:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 3
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc92b4c468 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
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
**Details:** Both modes succeeded but with different outputs

### Interpreter Output:
```
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:6:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:6:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:6:50 - Error parsing function statement
Error at /home/ghuntley/cursed/test_suite/test_programs/errors/02_undefined_variable.csd:12:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 3
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe70c8fcf8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc547ac108 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.bMhwBgsthK	2025-08-31 10:58:52.352486924 +0300
+++ /tmp/tmp.No9Wx8KUb7	2025-08-31 10:58:52.352486924 +0300
@@ -1,15 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffc547ac108 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_recursive_function
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff968a76c8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.6Y3VCiVSC9	2025-08-31 10:58:52.700485541 +0300
+++ /tmp/tmp.WMyWb4DrLR	2025-08-31 10:58:52.700485541 +0300
@@ -1,17 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff968a76c8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 03_nested_function_calls
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcde0ec218 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.GqOpUcb00e	2025-08-31 10:58:53.045484171 +0300
+++ /tmp/tmp.3xe1qdgW4Y	2025-08-31 10:58:53.045484171 +0300
@@ -1,17 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffcde0ec218 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe0ac1ff98 (parent: *interpreter.Environment@0)
DEBUG: Registering function 'no_params'
DEBUG: Executing statement type: Function
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x729bef180000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x729bef180080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128790e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247055 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a72 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2a1c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:505:42: 0x11b2269 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x729bef1c0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:2632:62: 0x1205d7a in parseReturnStatement (cursed_compiler_main.zig)
            const value_ptr = try self.arena_allocator.create(Expression);
                                                             ^
/home/ghuntley/cursed/src-zig/parser.zig:972:49: 0x11d4230 in parseStatement (cursed_compiler_main.zig)
            return try self.parseReturnStatement();
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x729bf6a20800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffa9fd8628 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.TV5i462T0T	2025-08-31 10:58:53.991480414 +0300
+++ /tmp/tmp.qlSNUrQxEN	2025-08-31 10:58:53.992480410 +0300
@@ -1,15 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffa9fd8628 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
```

---

## Test: 02_computation_intensive
**Status:** FAIL
**Details:** Both modes succeeded but with different outputs

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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe769c87f8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.Q8L85qbajt	2025-08-31 10:58:54.350478988 +0300
+++ /tmp/tmp.l71V6EwD42	2025-08-31 10:58:54.350478988 +0300
@@ -1,17 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffe769c87f8 (parent: *interpreter.Environment@0)
-✅ Program completed
-Executing 0 deferred statements
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffeebb68168 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffeebb68168 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffeebb68168 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/01_mathz_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7c52bb380000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128790e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247055 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a72 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2a1c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:505:42: 0x11b2269 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7c52bb380080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7c52bb3c0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1079:53: 0x11d59b7 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7c52c2c40800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e22 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7c52bb340000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffdc3b0b608 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdc3b0b608 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffdc3b0b608 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/02_stringz_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x7a6cbd960000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128790e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247055 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a72 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2a1c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:505:42: 0x11b2269 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7a6cbd960080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x7a6cbd9a0200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1079:53: 0x11d59b7 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x7a6cbd9e0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e22 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x7a6cbd920000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
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
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/03_mathz_advanced.csd:8:11 - Error parsing complex expression statement - check for misplaced braces or operator precedence issues (context: parseStatement)
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/03_mathz_advanced.csd:8:11 - Synchronizing parser after error (context: synchronize)
INFO: Recovered at delimiter 'RightParen' after skipping 3 tokens
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/03_mathz_advanced.csd:8:46 - Error parsing function statement
INFO: Recovered at delimiter 'RightParen' after skipping 13 tokens
INFO: Attempting additional statement recovery
Error at /home/ghuntley/cursed/test_suite/test_programs/stdlib/03_mathz_advanced.csd:26:1 - Failed to parse statement

=== Error Recovery Statistics ===
Total errors encountered: 2
Semicolon recoveries: 3
Statement recoveries: 2
Expression recoveries: 0
Delimiter recoveries: 0
Total tokens skipped: 16
================================
DEBUG: Registered global builtin functions (yap)
DEBUG: Initialized interpreter with globals@interpreter.Environment@7fffa8e1bc28 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fffa8e1bc28 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7fffa8e1bc28 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/03_mathz_advanced.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x78af0f100000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128790e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247055 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a72 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2a1c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:505:42: 0x11b2269 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x78af0f160200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1079:53: 0x11d59b7 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x78af0f1a0800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e22 in parseProgram (cursed_compiler_main.zig)
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
COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_mathz_advanced
```

### Output Diff:
```diff
--- /tmp/tmp.4LYUN4SGa0	2025-08-31 10:02:10.131411601 +0300
+++ /tmp/tmp.GfXMb99dib	2025-08-31 10:02:10.132411596 +0300
@@ -1,83 +1 @@
-DEBUG: Registered global builtin functions (yap)
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7fff3d5f81a8 (parent: *interpreter.Environment@0)
-DEBUG: Registering function 'main_character'
-DEBUG: Created new environment@interpreter.Environment@7f483fdc0000 with parent@*interpreter.Environment@7fff3d5f81a8
-DEBUG: Calling function with 0 parameters, got 0 args
-DEBUG: Function declaration ptr: @ast.FunctionStatement@7fff3d5f5aa8, parameters ptr: @ast.Parameter@aaaaaaaaaaaaaaaa
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
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f483fdc0000 with 0 variables for 'mathz'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7fff3d5f81a8 with 1 variables for 'mathz'
-DEBUG: Variable 'mathz' not found in any environment after 2 hops
-DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7f483fdc0000 with 0 variables for 'self'
-DEBUG: Environment.get() hop 1: checking env@interpreter.Environment@7fff3d5f81a8 with 1 variables for 'self'
-DEBUG: Variable 'self' not found in any environment after 2 hops
-DEBUG: Variable 'mathz' not found, attempting lazy module loading...
-DEBUG: Could not open CURSED stdlib file stdlib/mathz/mod.csd: error.FileNotFound
-ERROR: No CURSED stdlib implementation found for module 'mathz': error.ModuleNotFound
-SELF-HOSTING: Please implement stdlib/mathz/mod.csd for true self-hosting
-DEBUG: Failed to load module 'mathz'
-thread 1688963 panic: integer overflow
-/home/ghuntley/cursed/src-zig/interpreter.zig:466:9: 0xADDRESS in get (cursed_compiler_main.zig)
-        return InterpreterError.UndefinedVariable;
-        ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:1414:21: 0xADDRESS in evaluateExpression (cursed_compiler_main.zig)
-                    return InterpreterError.UndefinedVariable;
-                    ^
-/home/ghuntley/cursed/src-zig/interpreter.zig:2072:24: 0xADDRESS in evaluateMethodCall (cursed_compiler_main.zig)
-        const object = try self.evaluateExpression(member.object.*);
-                       ^
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
+COMPILE_ERROR: Binary not created at /home/ghuntley/cursed/test_suite/../03_mathz_advanced
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffec1083738 (parent: *interpreter.Environment@0)
DEBUG: Executing statement type: Expression
DEBUG: Evaluating expression type: Identifier
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffec1083738 with 1 variables for 'using'
DEBUG: Variable 'using' not found in any environment after 1 hops
DEBUG: Environment.get() hop 0: checking env@interpreter.Environment@7ffec1083738 with 1 variables for 'self'
DEBUG: Variable 'self' not found in any environment after 1 hops
DEBUG: Variable 'using' not found, attempting lazy module loading...
DEBUG: Could not open CURSED stdlib file stdlib/using/mod.csd: error.FileNotFound
ERROR: No CURSED stdlib implementation found for module 'using': error.ModuleNotFound
SELF-HOSTING: Please implement stdlib/using/mod.csd for true self-hosting
DEBUG: Failed to load module 'using'
❌ Runtime error in /home/ghuntley/cursed/test_suite/test_programs/stdlib/04_collections_basic.csd: error.UndefinedVariable
Executing 0 deferred statements
error(gpa): memory address 0x79de200e0000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x128790e in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x1247055 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x1201a72 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11d2a1c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:505:42: 0x11b2269 in parseProgram (cursed_compiler_main.zig)
                program.statements.append(self.allocator, anyopaque_ptr) catch {
                                         ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x79de200e0080 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x79de20120200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1079:53: 0x11d59b7 in parseStatement (cursed_compiler_main.zig)
        const expr_ptr = self.arena_allocator.create(Expression) catch {
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^

error(gpa): memory address 0x79de20160800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:493:61: 0x11b1e22 in parseProgram (cursed_compiler_main.zig)
                const stmt_ptr = self.arena_allocator.create(Statement) catch {
                                                            ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:235:47: 0x11a3cdd in interpretSource (cursed_compiler_main.zig)
    const program = cursed_parser.parseProgram() catch |err| {
                                              ^
/home/ghuntley/cursed/src-zig/cursed_compiler_main.zig:103:28: 0x11a5f97 in main (cursed_compiler_main.zig)
        try interpretSource(allocator, source, filename.?, verbose);
                           ^

error(gpa): memory address 0x79de200a0000 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
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
DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd7a977ed8 (parent: *interpreter.Environment@0)
✅ Program completed
Executing 0 deferred statements
error(gpa): memory address 0x74f7a6900000 leaked: 
/snap/zig/14937/lib/std/array_list.zig:1231:56: 0x125bfce in ensureTotalCapacityPrecise (std.zig)
                const new_memory = try gpa.alignedAlloc(T, alignment, new_capacity);
                                                       ^
/snap/zig/14937/lib/std/array_list.zig:1207:51: 0x121d7d5 in ensureTotalCapacity (std.zig)
            return self.ensureTotalCapacityPrecise(gpa, growCapacity(self.capacity, new_capacity));
                                                  ^
/snap/zig/14937/lib/std/array_list.zig:1261:41: 0x11dbb02 in addOne (std.zig)
            try self.ensureTotalCapacity(gpa, newlen);
                                        ^
/snap/zig/14937/lib/std/array_list.zig:894:49: 0x11b692c in append (std.zig)
            const new_item_ptr = try self.addOne(gpa);
                                                ^
/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0x1203a2e in parseFunctionStatement (cursed_compiler_main.zig)
            try func.body.append(self.allocator, stmt_ptr);
                                ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^

error(gpa): memory address 0x74f7a6940200 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:193:29: 0x10f4135 in alloc (std.zig)
            (self.createNode(0, n + ptr_align) orelse return null);
                            ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x1205440 in create__anon_30696 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0x1216df8 in parseAssignmentStatement (cursed_compiler_main.zig)
        const target_ptr = try self.arena_allocator.create(Expression);
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0x11d56fe in parseStatement (cursed_compiler_main.zig)
            return try self.parseAssignmentStatement();
                                                    ^
/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0x120384b in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt = try self.parseStatement();
                                                ^

error(gpa): memory address 0x74f7a6980800 leaked: 
/snap/zig/14937/lib/std/heap/arena_allocator.zig:213:43: 0x10f48a6 in alloc (std.zig)
                cur_node = self.createNode(cur_buf.len, n + ptr_align) orelse return null;
                                          ^
/snap/zig/14937/lib/std/mem/Allocator.zig:129:26: 0x10c6d2e in allocBytesWithAlignment__anon_8545 (std.zig)
    return a.vtable.alloc(a.ptr, len, alignment, ret_addr);
                         ^
/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0x11d28a0 in create__anon_26059 (std.zig)
    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
                                                          ^
/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0x1203919 in parseFunctionStatement (cursed_compiler_main.zig)
            const stmt_ptr = try self.arena_allocator.create(Statement); 
                                                            ^
/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0x11d2cfe in parseStatement (cursed_compiler_main.zig)
            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
                                                                     ^
/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0x11b1d54 in parseProgram (cursed_compiler_main.zig)
            if (self.parseStatement()) |stmt| {
                                   ^

```

### Compiled Output:
```
```

### Output Diff:
```diff
--- /tmp/tmp.01h9RHbtDV	2025-08-31 10:58:57.441466713 +0300
+++ /tmp/tmp.8Asq62uJvY	2025-08-31 10:58:57.441466713 +0300
@@ -1,78 +0,0 @@
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
-DEBUG: Initialized interpreter with globals@interpreter.Environment@7ffd7a977ed8 (parent: *interpreter.Environment@0)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1205:33: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            try func.body.append(self.allocator, stmt_ptr);
-                                ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
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
-/snap/zig/14937/lib/std/mem/Allocator.zig:157:59: 0xADDRESS in create__anon_30696 (std.zig)
-    const ptr: *T = @ptrCast(try a.allocBytesWithAlignment(.of(T), @sizeOf(T), @returnAddress()));
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:3767:59: 0xADDRESS in parseAssignmentStatement (cursed_compiler_main.zig)
-        const target_ptr = try self.arena_allocator.create(Expression);
-                                                          ^
-/home/ghuntley/cursed/src-zig/parser.zig:1067:53: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return try self.parseAssignmentStatement();
-                                                    ^
-/home/ghuntley/cursed/src-zig/parser.zig:1201:49: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
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
-/home/ghuntley/cursed/src-zig/parser.zig:1202:61: 0xADDRESS in parseFunctionStatement (cursed_compiler_main.zig)
-            const stmt_ptr = try self.arena_allocator.create(Statement); 
-                                                            ^
-/home/ghuntley/cursed/src-zig/parser.zig:925:70: 0xADDRESS in parseStatement (cursed_compiler_main.zig)
-            return Statement{ .Function = self.parseFunctionStatement() catch |parse_err| {
-                                                                     ^
-/home/ghuntley/cursed/src-zig/parser.zig:492:36: 0xADDRESS in parseProgram (cursed_compiler_main.zig)
-            if (self.parseStatement()) |stmt| {
-                                   ^
-
```

---

