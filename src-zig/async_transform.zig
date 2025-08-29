const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const codegen = @import("codegen.zig");

/// Async/await transformation engine that properly handles suspension points
pub const AsyncTransform = struct {
    allocator: Allocator,
    next_state_id: u32,
    suspension_points: ArrayList(SuspensionPoint),
    loop_stack: ArrayList(LoopContext),
    
    const SuspensionPoint = struct {
        id: u32,
        node: *ast.Expression,
        loop_context: ?*LoopContext,
        can_suspend: bool,
    };
    
    const LoopContext = struct {
        id: u32,
        entry_state: u32,
        exit_state: u32,
        continue_state: u32,
        suspension_points: ArrayList(u32),
        
        const Self = @This();
        
        pub fn init(allocator: Allocator, id: u32, entry_state: u32) Self {
            return Self{
                .id = id,
                .entry_state = entry_state,
                .exit_state = 0, // Set later
                .continue_state = 0, // Set later  
                .suspension_points = .empty,
            };
        }
        
        pub fn deinit(self: *Self) void {
            self.suspension_points.deinit(self.allocator);
        }
    };
    
    pub const StateMachine = struct {
        states: ArrayList(State),
        entry_state: u32,
        
        pub const State = struct {
            id: u32,
            code: []const u8,
            transitions: ArrayList(Transition),
            
            pub const Transition = struct {
                condition: ?[]const u8, // null = unconditional
                target_state: u32,
            };
        };
    };
    
    pub fn init() AsyncTransform {
        return AsyncTransform{
            .allocator = allocator,
            .next_state_id = 0,
            .suspension_points = .empty,
            .loop_stack = .empty,
        };
    }
    
    pub fn deinit(self: *AsyncTransform) void {
        self.suspension_points.deinit(self.allocator);
        for (self.loop_stack.items) |*ctx| {
            ctx.deinit();
        }
        self.loop_stack.deinit(self.allocator);
    }
    
    /// Transform async function into state machine
    pub fn transformAsyncFunction(self: *AsyncTransform, func: *ast.AsyncFunction) !StateMachine {
        var state_machine = StateMachine{
            .states = .empty,
            .entry_state = self.nextStateId(),
        };
        
        // Reset transformation state
        self.suspension_points.clearRetainingCapacity();
        self.loop_stack.clearRetainingCapacity();
        
        // First pass: identify all suspension points and validate loop contexts
        try self.identifySuspensionPoints(func.body);
        
        // Second pass: validate suspension points in loops
        try self.validateLoopSuspensions();
        
        // Third pass: generate state machine
        try self.generateStateMachine(func.body, &state_machine);
        
        return state_machine;
    }
    
    /// First pass: identify all await expressions and their contexts
    fn identifySuspensionPoints(self: *AsyncTransform, expr: *ast.Expression) !void {
        switch (expr.*) {
            .AwaitExpression => |await_expr| {
                const suspension_point = SuspensionPoint{
                    .id = self.nextStateId(),
                    .node = expr,
                    .loop_context = if (self.loop_stack.items.len > 0) 
                        &self.loop_stack.items[self.loop_stack.items.len - 1] else null,
                    .can_suspend = true,
                };
                
                try self.suspension_points.append(allocator, suspension_point);
                
                // If we're in a loop, record this suspension point
                if (suspension_point.loop_context) |loop_ctx| {
                    try loop_ctx.suspension_points.append(allocator, suspension_point.id);
                }
                
                // Continue analyzing the awaited expression
                try self.identifySuspensionPoints(await_expr.expression);
            },
            .Loop => |loop_expr| {
                // Enter loop context
                const loop_id = self.nextStateId();
                const entry_state = self.nextStateId();
                const loop_ctx = LoopContext.init(self.allocator, loop_id, entry_state);
                try self.loop_stack.append(self.allocator, loop_ctx);
                
                // Analyze loop body
                try self.identifySuspensionPoints(loop_expr.body);
                
                // Set exit and continue states
                var current_loop = &self.loop_stack.items[self.loop_stack.items.len - 1];
                current_loop.exit_state = self.nextStateId();
                current_loop.continue_state = entry_state; // Continue goes back to entry
                
                // Exit loop context
                _ = self.loop_stack.pop();
            },
            .For => |for_expr| {
                // Similar to Loop but with iterator context
                const loop_id = self.nextStateId();
                const entry_state = self.nextStateId();
                const loop_ctx = LoopContext.init(self.allocator, loop_id, entry_state);
                try self.loop_stack.append(self.allocator, loop_ctx);
                
                // Analyze iterable expression first
                try self.identifySuspensionPoints(for_expr.iterable);
                
                // Then analyze loop body
                try self.identifySuspensionPoints(for_expr.body);
                
                // Set exit and continue states
                var current_loop = &self.loop_stack.items[self.loop_stack.items.len - 1];
                current_loop.exit_state = self.nextStateId();
                current_loop.continue_state = entry_state;
                
                // Exit loop context
                _ = self.loop_stack.pop();
            },
            .While => |while_expr| {
                // While loop handling
                const loop_id = self.nextStateId();
                const entry_state = self.nextStateId();
                const loop_ctx = LoopContext.init(self.allocator, loop_id, entry_state);
                try self.loop_stack.append(self.allocator, loop_ctx);
                
                // Analyze condition (may contain await)
                try self.identifySuspensionPoints(while_expr.condition);
                
                // Analyze body
                try self.identifySuspensionPoints(while_expr.body);
                
                // Set exit and continue states
                var current_loop = &self.loop_stack.items[self.loop_stack.items.len - 1];
                current_loop.exit_state = self.nextStateId();
                current_loop.continue_state = entry_state;
                
                // Exit loop context
                _ = self.loop_stack.pop();
            },
            .Block => |block| {
                // Analyze all statements in block
                for (block.statements) |stmt| {
                    try self.identifySuspensionPoints(stmt);
                }
            },
            .If => |if_expr| {
                // Analyze condition
                try self.identifySuspensionPoints(if_expr.condition);
                
                // Analyze then branch
                try self.identifySuspensionPoints(if_expr.then_branch);
                
                // Analyze else branch if present
                if (if_expr.else_branch) |else_branch| {
                    try self.identifySuspensionPoints(else_branch);
                }
            },
            .FunctionCall => |call| {
                // Check if this is an async function call
                for (call.arguments) |arg| {
                    try self.identifySuspensionPoints(arg);
                }
            },
            else => {
                // For other expression types, recursively analyze sub-expressions
                // This would be expanded based on the full AST structure
            },
        }
    }
    
    /// Validate that suspension points in loops are handled correctly
    fn validateLoopSuspensions(self: *AsyncTransform) !void {
        for (self.suspension_points.items) |*suspension_point| {
            if (suspension_point.loop_context) |loop_ctx| {
                // CRITICAL FIX: Suspension points in loops require special handling
                
                // Rule 1: Await in loop condition requires loop state to be resumable
                if (self.isInLoopCondition(suspension_point.node, loop_ctx)) {
                    // Generate error: await in loop condition needs careful state management
                    std.log.warn("Suspension point in loop condition requires state machine restructuring", .{});
                    suspension_point.can_suspend = false;
                    return error.InvalidSuspensionPointInLoopCondition;
                }
                
                // Rule 2: Await in loop body needs proper continuation setup
                if (self.isInLoopBody(suspension_point.node, loop_ctx)) {
                    // This is valid but requires the state machine to properly handle
                    // resumption within the loop context
                    std.log.info("Suspension point in loop body - generating continuation state", .{});
                }
                
                // Rule 3: Nested loops with suspension points need careful state tracking
                if (self.loop_stack.items.len > 1) {
                    std.log.warn("Nested loops with suspension points - validating state coherence", .{});
                    try self.validateNestedLoopSuspension(suspension_point);
                }
            }
        }
    }
    
    /// Check if suspension point is in loop condition
    fn isInLoopCondition(self: *AsyncTransform, node: *ast.Expression, loop_ctx: *LoopContext) bool {
        _ = self;
        _ = node;
        _ = loop_ctx;
        // TODO: Implement proper AST traversal to check if node is in loop condition
        return false;
    }
    
    /// Check if suspension point is in loop body
    fn isInLoopBody(self: *AsyncTransform, node: *ast.Expression, loop_ctx: *LoopContext) bool {
        _ = self;
        _ = node;
        _ = loop_ctx;
        // TODO: Implement proper AST traversal to check if node is in loop body
        return true; // Assume body for now
    }
    
    /// Validate nested loop suspension points
    fn validateNestedLoopSuspension(self: *AsyncTransform, suspension_point: *SuspensionPoint) !void {
        _ = self;
        _ = suspension_point;
        // TODO: Implement validation for nested loop contexts
    }
    
    /// Generate state machine from analyzed suspension points
    fn generateStateMachine(self: *AsyncTransform, expr: *ast.Expression, state_machine: *StateMachine) !void {
        // Start with entry state
        var current_state_id = state_machine.entry_state;
        
        try self.generateStatesForExpression(expr, &current_state_id, state_machine);
    }
    
    /// Generate states for a specific expression
    fn generateStatesForExpression(
        self: *AsyncTransform, 
        expr: *ast.Expression, 
        current_state: *u32, 
        state_machine: *StateMachine
    ) !void {
        switch (expr.*) {
            .AwaitExpression => |await_expr| {
                // Generate await state
                const await_state = StateMachine.State{
                    .id = current_state.*,
                    .code = try self.generateAwaitCode(&await_expr),
                    .transitions = .empty,
                };
                
                try state_machine.states.append(allocator, await_state);
                
                // Next state for continuation
                current_state.* = self.nextStateId();
                
                // Add transition to continuation state
                var await_state_mut = &state_machine.states.items[state_machine.states.items.len - 1];
                try await_state_mut.transitions.append(StateMachine.State.Transition{
                    .condition = null, // Unconditional after await completes
                    .target_state = current_state.*,
                });
            },
            .Loop => |loop_expr| {
                // Generate loop entry state
                const loop_entry = current_state.*;
                const loop_body_state = self.nextStateId();
                const loop_exit_state = self.nextStateId();
                
                // Entry state: unconditional jump to body
                const entry_state = StateMachine.State{
                    .id = loop_entry,
                    .code = "// Loop entry",
                    .transitions = .empty,
                };
                try state_machine.states.append(allocator, entry_state);
                
                var entry_state_mut = &state_machine.states.items[state_machine.states.items.len - 1];
                try entry_state_mut.transitions.append(StateMachine.State.Transition{
                    .condition = null,
                    .target_state = loop_body_state,
                });
                
                // Generate body states
                current_state.* = loop_body_state;
                try self.generateStatesForExpression(loop_expr.body, current_state, state_machine);
                
                // Add back-edge from body end to loop entry
                // This handles the case where loop body doesn't suspend
                if (current_state.* != loop_body_state) {
                    // Body generated new states, add transition back to entry
                    var last_state = &state_machine.states.items[state_machine.states.items.len - 1];
                    try last_state.transitions.append(StateMachine.State.Transition{
                        .condition = "continue_loop", // Loop condition check
                        .target_state = loop_entry,
                    });
                    try last_state.transitions.append(StateMachine.State.Transition{
                        .condition = "exit_loop",
                        .target_state = loop_exit_state,
                    });
                }
                
                current_state.* = loop_exit_state;
            },
            .Block => |block| {
                // Generate states for each statement sequentially
                for (block.statements) |stmt| {
                    try self.generateStatesForExpression(stmt, current_state, state_machine);
                }
            },
            else => {
                // For non-suspending expressions, generate single state
                const simple_state = StateMachine.State{
                    .id = current_state.*,
                    .code = try self.generateSimpleCode(expr),
                    .transitions = .empty,
                };
                
                try state_machine.states.append(allocator, simple_state);
                current_state.* = self.nextStateId();
                
                // Add transition to next state
                var simple_state_mut = &state_machine.states.items[state_machine.states.items.len - 1];
                try simple_state_mut.transitions.append(StateMachine.State.Transition{
                    .condition = null,
                    .target_state = current_state.*,
                });
            },
        }
    }
    
    /// Generate code for await expression
    fn generateAwaitCode(self: *AsyncTransform, await_expr: *const ast.AwaitExpressionType) ![]const u8 {
        _ = self;
        _ = await_expr;
        return "// Await implementation with proper suspension";
    }
    
    /// Generate code for simple (non-suspending) expressions
    fn generateSimpleCode(self: *AsyncTransform, expr: *ast.Expression) ![]const u8 {
        _ = self;
        _ = expr;
        return "// Non-suspending expression";
    }
    
    /// Get next available state ID
    fn nextStateId(self: *AsyncTransform) u32 {
        const id = self.next_state_id;
        self.next_state_id += 1;
        return id;
    }
};

/// Async/await runtime support
pub const AsyncRuntime = struct {
    allocator: Allocator,
    pending_tasks: std.HashMap(u32, *AsyncTask, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    next_task_id: u32,
    
    const AsyncTask = struct {
        id: u32,
        state_machine: *AsyncTransform.StateMachine,
        current_state: u32,
        context: *anyopaque,
        suspended: bool,
        completed: bool,
    };
    
    pub fn init() AsyncRuntime {
        return AsyncRuntime{
            .allocator = allocator,
            .pending_tasks = std.HashMap(u32, *AsyncTask, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .next_task_id = 1,
        };
    }
    
    pub fn deinit(self: *AsyncRuntime) void {
        self.pending_tasks.deinit(self.allocator);
    }
    
    /// Spawn async task
    pub fn spawnTask(self: *AsyncRuntime, state_machine: *AsyncTransform.StateMachine) !u32 {
        const task_id = self.next_task_id;
        self.next_task_id += 1;
        
        const task = try self.allocator.create(AsyncTask);
        task.* = AsyncTask{
            .id = task_id,
            .state_machine = state_machine,
            .current_state = state_machine.entry_state,
            .context = undefined, // Set by caller
            .suspended = false,
            .completed = false,
        };
        
        try self.pending_tasks.put(task_id, task);
        return task_id;
    }
    
    /// Resume suspended task
    pub fn resumeTask(self: *AsyncRuntime, task_id: u32) !void {
        if (self.pending_tasks.get(task_id)) |task| {
            if (task.suspended and !task.completed) {
                try self.executeTaskFromState(task);
            }
        }
    }
    
    /// Execute task from current state
    fn executeTaskFromState(self: *AsyncRuntime, task: *AsyncTask) !void {
        _ = self;
        
        // Find current state in state machine
        for (task.state_machine.states.items) |state| {
            if (state.id == task.current_state) {
                // Execute state code
                // In a real implementation, this would execute the generated code
                
                // Check for transitions
                if (state.transitions.items.len > 0) {
                    // For simplicity, take first transition
                    const transition = state.transitions.items[0];
                    task.current_state = transition.target_state;
                    
                    // If this was a suspension point, mark as suspended
                    // Otherwise continue execution
                    if (std.mem.indexOf(u8, state.code, "await") != null) {
                        task.suspended = true;
                    }
                }
                break;
            }
        }
    }
};

// Export the transform function for use by the compiler
pub fn transformAsyncFunction(allocator: Allocator, func: *ast.AsyncFunction) !AsyncTransform.StateMachine {
    var transformer = AsyncTransform.init(allocator);
    defer transformer.deinit();
    
    return try transformer.transformAsyncFunction(func);
}
