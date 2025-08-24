# CURSED Pattern Matching Internals

*Technical documentation for CURSED's advanced pattern matching implementation*

## Architecture Overview

CURSED's pattern matching system consists of several interconnected components:

```
Source Code (.csd)
    ↓
Pattern Parser (parser.zig)
    ↓
Pattern AST (ast_advanced.zig)
    ↓
Pattern Analysis & Optimization (pattern_optimization.zig)
    ↓
Decision Tree Generation (pattern_decision_tree.zig)
    ↓
Code Generation (pattern_matching.zig)
    ↓
Runtime Context (pattern_variable_context.zig)
    ↓
Exhaustiveness Checking (exhaustive_pattern_checking.zig)
```

## Core Components

### 1. Pattern AST Representation

Located in `src-zig/ast_advanced.zig`:

```zig
pub const Pattern = union(enum) {
    Literal: LiteralPattern,
    Variable: VariablePattern,
    Wildcard: void,
    Tuple: TuplePattern,
    Struct: StructPattern,
    Array: ArrayPattern,
    Slice: SlicePattern,
    Or: OrPattern,
    Range: RangePattern,
    Guard: GuardPattern,
    Enum: EnumPattern,
    Type: TypePattern,
};
```

Each pattern type contains specific information:

- **LiteralPattern**: Contains the literal value (integer, string, boolean, float)
- **VariablePattern**: Variable name and mutability flag
- **TuplePattern**: Array of sub-patterns
- **StructPattern**: Struct name and field patterns
- **GuardPattern**: Base pattern + guard expression
- **RangePattern**: Start/end expressions + inclusivity flag

### 2. Pattern Parsing

Located in `src-zig/parser.zig`:

The parser handles pattern syntax in `parsePattern()`:

1. **OR patterns** - `parsePatternOr()` handles `|` operators
2. **Range patterns** - `parsePatternRange()` handles `..` operators
3. **Primary patterns** - `parsePatternPrimary()` handles literals, variables, etc.

```zig
fn parsePattern(self: *Parser) ParserError!ast.Pattern {
    return self.parsePatternOr();
}

fn parsePatternOr(self: *Parser) ParserError!ast.Pattern {
    var patterns = ArrayList(ast.Pattern){};
    
    const first_pattern = try self.parsePatternRange();
    try patterns.append(self.allocator, first_pattern);
    
    while (self.match(.Pipe)) {
        const pattern = try self.parsePatternRange();
        try patterns.append(self.allocator, pattern);
    }
    
    if (patterns.items.len == 1) {
        const single_pattern = patterns.items[0];
        patterns.deinit();
        return single_pattern;
    }
    
    return ast.Pattern{ .Or = OrPattern{ .patterns = patterns.items } };
}
```

### 3. Pattern Optimization

Located in `src-zig/pattern_optimization.zig`:

The optimizer analyzes patterns and applies several strategies:

#### Jump Table Optimization

For patterns with many literals:

```zig
pub fn generateJumpTableCode(
    self: *PatternOptimizer, 
    patterns: []const ast.Pattern, 
    match_value: []const u8, 
    output: *ArrayList(u8),
    analysis: PatternAnalysis
) !void {
    try output.writer().print("switch ({s}) {{\n", .{match_value});
    
    for (patterns, 0..) |pattern, i| {
        if (pattern == .Literal) {
            const literal = pattern.Literal;
            switch (literal.value) {
                .Integer => |val| {
                    try output.writer().print("case {}:\n", .{val});
                    try output.writer().print("    goto pattern_action_{};\n", .{i});
                },
                // ... other literal types
            }
        }
    }
    
    try output.writer().print("default:\n");
    try output.writer().print("    goto pattern_match_failed;\n");
    try output.writer().print("}}\n");
}
```

#### Pattern Reordering

Sorts patterns by complexity:

```zig
fn applyPatternReordering(
    self: *PatternOptimizer, 
    patterns: []ast.Pattern, 
    opportunity: PatternAnalysis.OptimizationOpportunity
) !void {
    const Context = struct {
        patterns: []ast.Pattern,
        
        pub fn lessThan(context: @This(), lhs_index: usize, rhs_index: usize) bool {
            const lhs_complexity = calculateSimpleComplexity(context.patterns[lhs_index]);
            const rhs_complexity = calculateSimpleComplexity(context.patterns[rhs_index]);
            return lhs_complexity < rhs_complexity;
        }
        
        fn calculateSimpleComplexity(pattern: ast.Pattern) usize {
            return switch (pattern) {
                .Literal => 1,
                .Variable, .Wildcard => 0,
                .Guard => 10,
                else => 5,
            };
        }
    };
    
    // Sort patterns and reorder
    // ... sorting implementation
}
```

### 4. Decision Tree Compilation

Located in `src-zig/pattern_decision_tree.zig`:

Converts patterns into optimized decision trees:

```zig
pub const DecisionNode = union(enum) {
    Test: TestNode,      // Test a condition
    Action: ActionNode,  // Execute pattern action
    Failure: FailureNode, // Pattern match failed
    Switch: SwitchNode,  // Multi-way switch
    Guard: GuardNode,    // Guard evaluation
    Bind: BindNode,      // Variable binding
};
```

The compiler builds trees recursively:

```zig
fn buildDecisionTree(
    self: *PatternDecisionTreeCompiler, 
    match_value: []const u8, 
    start_index: usize, 
    end_index: usize
) !*DecisionNode {
    if (start_index >= end_index) {
        return createFailureNode("No patterns matched");
    }
    
    if (end_index - start_index == 1) {
        return createActionNode(self.pattern_info[start_index], match_value);
    }
    
    // Find optimal split strategy
    const strategy = try self.findOptimalSplit(match_value, start_index, end_index);
    
    return switch (strategy.strategy_type) {
        .LiteralSwitch => try self.buildLiteralSwitchNode(match_value, strategy),
        .TypeDispatch => try self.buildTypeDispatchNode(match_value, strategy),
        .GuardEvaluation => try self.buildGuardNode(match_value, strategy),
        .SequentialTest => try self.buildSequentialNode(match_value, strategy),
    };
}
```

### 5. Exhaustiveness Checking

Located in `src-zig/exhaustive_pattern_checking.zig`:

#### Enum Exhaustiveness

```zig
pub fn checkEnumExhaustiveness(
    self: *EnumExhaustivenessChecker, 
    enum_name: []const u8, 
    patterns: []const ast.Pattern
) !ExhaustivenessResult {
    const variants = self.enum_registry.getEnumVariants(enum_name) orelse {
        return error.EnumNotFound;
    };
    
    var coverage = try EnumCoverage.init(self.allocator, enum_name, variants.items.len);
    defer coverage.deinit();
    
    // Analyze each pattern
    for (patterns) |pattern| {
        try self.analyzePattern(pattern, enum_name, &coverage);
    }
    
    // Check if all variants are covered
    const is_exhaustive = coverage.has_wildcard or 
                         coverage.covered_variants.count() == variants.items.len;
    
    return ExhaustivenessResult{
        .is_exhaustive = is_exhaustive,
        .covered_count = coverage.covered_variants.count(),
        .total_count = variants.items.len,
        .missing_variants = missing_variants,
        .has_wildcard = coverage.has_wildcard,
    };
}
```

#### Boolean Exhaustiveness

```zig
pub fn checkBooleanExhaustiveness(
    self: *EnumExhaustivenessChecker, 
    patterns: []const ast.Pattern
) !BooleanExhaustivenessResult {
    var has_true = false;
    var has_false = false;
    var has_wildcard = false;
    
    for (patterns) |pattern| {
        switch (pattern) {
            .Literal => |literal| {
                switch (literal.value) {
                    .Boolean => |val| {
                        if (val) has_true = true else has_false = true;
                    },
                    else => {},
                }
            },
            .Wildcard, .Variable => has_wildcard = true,
            .Or => |or_pattern| {
                // Recursively check OR alternatives
                const sub_result = try self.checkBooleanExhaustiveness(or_pattern.patterns);
                defer sub_result.missing_patterns.deinit();
                
                if (sub_result.has_true) has_true = true;
                if (sub_result.has_false) has_false = true;
                if (sub_result.has_wildcard) has_wildcard = true;
            },
            else => {},
        }
    }
    
    const is_exhaustive = has_wildcard or (has_true and has_false);
    
    return BooleanExhaustivenessResult{
        .is_exhaustive = is_exhaustive,
        .has_true = has_true,
        .has_false = has_false,
        .has_wildcard = has_wildcard,
        .missing_patterns = missing_patterns,
    };
}
```

### 6. Variable Context Management

Located in `src-zig/pattern_variable_context.zig`:

Manages variable bindings during pattern matching:

```zig
pub const PatternContext = struct {
    allocator: Allocator,
    variable_stack: ArrayList(VariableBinding),
    scope_depth: usize,
    guard_evaluation_active: bool,
    
    pub fn bindVariable(
        self: *PatternContext, 
        name: []const u8, 
        value: PatternValue, 
        type_name: []const u8, 
        is_mutable: bool
    ) !void {
        // Check if variable already exists in current scope
        for (self.variable_stack.items) |binding| {
            if (binding.scope_depth == self.scope_depth and 
               std.mem.eql(u8, binding.name, name)) {
                return error.VariableAlreadyBound;
            }
        }
        
        const owned_name = try self.allocator.dupe(u8, name);
        const owned_type = try self.allocator.dupe(u8, type_name);
        
        const binding = VariableBinding.init(
            owned_name, value, owned_type, is_mutable, self.scope_depth
        );
        try self.variable_stack.append(binding);
    }
    
    pub fn lookupVariable(self: *PatternContext, name: []const u8) ?*VariableBinding {
        var i: usize = self.variable_stack.items.len;
        while (i > 0) {
            i -= 1;
            if (std.mem.eql(u8, self.variable_stack.items[i].name, name)) {
                return &self.variable_stack.items[i];
            }
        }
        return null;
    }
};
```

## Code Generation Strategies

### 1. Sequential Pattern Matching

Basic approach - test patterns in order:

```c
// Generated C code for sequential matching
if (match_value == 1) goto pattern_1;
if (match_value == 2) goto pattern_2;
if (match_value == 3) goto pattern_3;
goto pattern_default;

pattern_1:
    // Handle case 1
    result = handle_case_1();
    goto pattern_end;

pattern_2:
    // Handle case 2
    result = handle_case_2();
    goto pattern_end;

// ... etc
```

### 2. Jump Table Generation

For many literal patterns:

```c
// Generated C code for jump table
static void* jump_table[] = {
    &&case_0, &&case_1, &&case_2, &&case_3,
    &&case_4, &&case_5, &&case_6, &&case_7
};

if (match_value >= 0 && match_value < 8) {
    goto *jump_table[match_value];
}
goto pattern_default;

case_0:
    result = handle_case_0();
    goto pattern_end;

case_1:
    result = handle_case_1();
    goto pattern_end;

// ... etc
```

### 3. Decision Tree Code

For complex patterns:

```c
// Generated C code for decision tree
if (match_value < 50) {
    if (match_value < 25) {
        if (match_value == 10) goto pattern_10;
        if (match_value == 20) goto pattern_20;
        goto pattern_small;
    } else {
        if (match_value == 30) goto pattern_30;
        if (match_value == 40) goto pattern_40;
        goto pattern_medium;
    }
} else {
    if (match_value < 75) {
        // ... more conditions
    } else {
        // ... more conditions
    }
}
```

### 4. Guard Code Generation

For patterns with guards:

```c
// Generated C code for guards
// First match base pattern
if (test_base_pattern(match_value)) {
    // Set up guard evaluation context
    cursed_set_guard_variable("x", match_value);
    
    // Evaluate guard condition
    if (evaluate_guard_condition()) {
        // Guard passed - execute action
        result = pattern_action();
        goto pattern_end;
    }
    
    // Guard failed - clear context and try next pattern
    cursed_clear_guard_context();
}

// Try next pattern...
```

## Performance Characteristics

### Compilation Time Complexity

- **Pattern parsing**: O(n) where n = pattern AST nodes
- **Optimization analysis**: O(n log n) for pattern sorting
- **Decision tree building**: O(n log n) for balanced trees  
- **Jump table generation**: O(n) for literal patterns
- **Exhaustiveness checking**: O(v * p) where v = variants, p = patterns

### Runtime Performance

- **Jump table lookup**: O(1) - fastest for dense literal patterns
- **Decision tree traversal**: O(log n) - good for sparse patterns
- **Sequential matching**: O(n) - baseline performance
- **Guard evaluation**: Variable - depends on guard complexity
- **Variable binding**: O(1) per binding operation

### Memory Usage

Pattern matching data structures:

```
Pattern AST node: 64 bytes average
- Tag: 8 bytes
- Union data: 56 bytes max
- Metadata: varies

Jump table entry: 16 bytes
- Value: 8 bytes  
- Action pointer: 8 bytes

Decision tree node: 32 bytes
- Node type: 8 bytes
- Test info: 16 bytes
- Child pointers: 8 bytes

Variable binding: 96 bytes
- Name: 8 bytes (pointer to heap)
- Value: 64 bytes (PatternValue union)
- Type info: 16 bytes
- Metadata: 8 bytes
```

## Optimization Techniques

### 1. Pattern Complexity Analysis

```zig
fn calculatePatternComplexity(pattern: ast.Pattern) usize {
    return switch (pattern) {
        .Literal => 1,                    // O(1) comparison
        .Variable, .Wildcard => 0,        // Always matches
        .Range => 4,                      // Two comparisons + bounds check
        .Guard => |guard| blk: {
            const base = calculatePatternComplexity(guard.pattern.*);
            break :blk base + 8;          // Guards are expensive
        },
        .Tuple => |tuple| blk: {
            var complexity: usize = 2;    // Length check + allocation
            for (tuple.patterns) |sub_pattern| {
                complexity += calculatePatternComplexity(sub_pattern);
            }
            break :blk complexity;
        },
        .Or => |or_pattern| blk: {
            var complexity: usize = 1;
            for (or_pattern.patterns) |alt| {
                complexity += calculatePatternComplexity(alt);
            }
            break :blk complexity * 2;   // May try multiple alternatives
        },
        else => 5,                        // Default complexity
    };
}
```

### 2. Pattern Reachability Analysis

```zig
fn analyzeReachability(patterns: []const ast.Pattern, reachability: []bool) void {
    var has_wildcard = false;
    var wildcard_index: ?usize = null;
    
    for (patterns, 0..) |pattern, i| {
        switch (pattern) {
            .Wildcard, .Variable => {
                if (!has_wildcard) {
                    has_wildcard = true;
                    wildcard_index = i;
                }
            },
            else => {},
        }
    }
    
    // Mark patterns after wildcard as unreachable
    if (wildcard_index) |index| {
        for (reachability[index + 1..]) |*reachable| {
            reachable.* = false;
        }
    }
}
```

### 3. Guard Optimization

Guards can be optimized by:

1. **Short-circuiting**: Evaluate cheapest conditions first
2. **Common sub-expression elimination**: Cache repeated computations
3. **Variable context optimization**: Minimize variable lookups

```zig
// Optimized guard evaluation
fn evaluateGuardOptimized(guard_expr: GuardExpression, context: *PatternContext) bool {
    // Cache frequently accessed variables
    const cached_vars = context.getCachedVariables();
    
    // Evaluate conditions in order of complexity
    for (guard_expr.conditions) |condition| {
        if (!evaluateConditionWithCache(condition, cached_vars)) {
            return false; // Short-circuit on first failure
        }
    }
    
    return true;
}
```

## Integration with CURSED Compiler

### 1. Parser Integration

Pattern parsing is integrated into the main parser:

```zig
// In expression parsing
.Sick => {
    return try self.parseMatchExpression();
},

fn parseMatchExpression(self: *Parser) ParserError!Expression {
    _ = try self.consume(.Sick, "Expected 'sick'");
    
    const value = try self.allocateExpression(try self.parseExpression());
    _ = try self.consume(.LeftBrace, "Expected '{' after match value");
    
    var cases = ArrayList(ast.MatchCase){};
    
    while (!self.check(.RightBrace) and !self.isAtEnd()) {
        if (self.match(.Newline)) continue;
        
        const pattern = try self.parsePattern();
        
        var guard: ?Expression = null;
        if (self.matchIdentifier("when")) {
            guard = try self.parseExpression();
        }
        
        _ = try self.consume(.Arrow, "Expected '->' after pattern");
        const result = try self.parseExpression();
        
        // ... create MatchCase and add to cases
    }
    
    return Expression{ .Match = ast.MatchExpression{
        .expression = value,
        .cases = cases,
        .default_case = default_case,
    }};
}
```

### 2. Code Generation Integration

Pattern matching integrates with both interpreter and compiler modes:

```zig
// In interpreter
.Match => |match| return try self.evaluateMatch(match),

fn evaluateMatch(self: *Interpreter, match_expr: ast.MatchExpression) !Value {
    const match_value = try self.evaluateExpression(match_expr.expression.*);
    defer match_value.deinit(self.allocator);
    
    // Try each pattern in order
    for (match_expr.cases.items) |case| {
        if (try self.matchPattern(case.pattern, match_value)) {
            // Pattern matched - evaluate guard if present
            if (case.guard) |guard| {
                const guard_result = try self.evaluateExpression(@ptrFromInt(@intFromPtr(guard)));
                defer guard_result.deinit(self.allocator);
                
                if (!guard_result.isTruthy()) {
                    continue; // Guard failed, try next pattern
                }
            }
            
            // Execute pattern action
            return try self.evaluateExpression(@ptrFromInt(@intFromPtr(case.result)));
        }
    }
    
    // No pattern matched
    if (match_expr.default_case) |default| {
        return try self.evaluateExpression(default.*);
    } else {
        return error.NoPatternMatched;
    }
}
```

### 3. Type System Integration

Pattern matching integrates with CURSED's type system:

```zig
fn inferPatternType(pattern: ast.Pattern, matched_type: Type) !Type {
    return switch (pattern) {
        .Literal => |literal| inferLiteralType(literal),
        .Variable => matched_type, // Variable takes type of matched value
        .Wildcard => matched_type,
        .Tuple => |tuple| blk: {
            if (matched_type != .Tuple) return error.TypeMismatch;
            // Infer types for tuple elements
            break :blk try inferTuplePatternType(tuple, matched_type.Tuple);
        },
        .Guard => |guard| try inferPatternType(guard.pattern.*, matched_type),
        // ... other pattern types
    };
}
```

## Testing Strategy

### 1. Unit Tests

Each component has comprehensive unit tests:

```zig
// Test pattern compilation
test "pattern decision tree compilation" {
    var output = ArrayList(u8).init(std.testing.allocator);
    defer output.deinit();
    
    var registry = pattern_matching.EnumVariantRegistry.init(std.testing.allocator);
    defer registry.deinit();
    
    var compiler = PatternDecisionTreeCompiler.init(std.testing.allocator, &registry, &output);
    
    const patterns = [_]ast.Pattern{
        ast.Pattern{ .Literal = ast.Pattern.LiteralPattern{ .value = ast.LiteralValue{ .Integer = 42 } } },
        ast.Pattern{ .Literal = ast.Pattern.LiteralPattern{ .value = ast.LiteralValue{ .Integer = 100 } } },
        ast.Pattern{ .Wildcard = {} },
    };
    
    const decision_tree = try compiler.compilePatterns(&patterns, "match_value");
    try compiler.generateCode(decision_tree, "match_value");
    
    const generated_code = output.items;
    try std.testing.expect(generated_code.len > 0);
    try std.testing.expect(std.mem.indexOf(u8, generated_code, "switch") != null);
}
```

### 2. Integration Tests

Full pattern matching workflows are tested:

```cursed
// Test file: advanced_pattern_matching_test.csd
slay test_exhaustive_enum_patterns() {
    sus status Status = Status.Success
    
    sus result drip = sick status {
        when Status.Success -> 1
        when Status.Error -> 2
        when Status.Pending -> 3
        when Status.Cancelled -> 4
    }
    
    assert_eq_int(result, 1)
}
```

### 3. Performance Tests

Performance characteristics are validated:

```cursed
// Test file: pattern_performance_benchmark.csd
slay benchmark_literal_patterns() {
    sus iterations drip = 10000
    sus start_time drip = timez.now_micros()
    
    bestie (i drip = 0; i < iterations; i = i + 1) {
        sus result drip = match_literal_jump_table(42)
        if (result == -1) vibez.spill("Unexpected result")
    }
    
    sus total_time drip = timez.now_micros() - start_time
    sus avg_time spill = total_time / iterations
    
    assert(avg_time < 1.0, "Jump table should be under 1μs per operation")
}
```

## Future Enhancements

### 1. LLVM Integration

Direct LLVM IR generation for pattern matching:

```zig
pub fn generateLLVMPatternMatch(
    builder: c.LLVMBuilderRef,
    patterns: []ast.Pattern,
    match_value: c.LLVMValueRef
) !c.LLVMValueRef {
    // Generate optimal LLVM IR for pattern matching
    // - Use LLVM switch instructions for jump tables
    // - Generate efficient branch structures for decision trees
    // - Optimize guard evaluation with LLVM optimizations
}
```

### 2. Advanced Optimizations

- **Profile-guided optimization**: Optimize based on runtime pattern frequency
- **Whole-program optimization**: Cross-function pattern analysis
- **Machine code generation**: Direct assembly generation for hot patterns

### 3. Extended Pattern Types

- **Regular expression patterns**: Built-in regex matching
- **XML/JSON patterns**: Structured data matching
- **Custom pattern types**: User-defined pattern matching logic

This comprehensive technical documentation covers all aspects of CURSED's pattern matching implementation, from parsing and AST representation through optimization and code generation.
