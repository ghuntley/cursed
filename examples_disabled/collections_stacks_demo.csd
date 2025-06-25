/// Comprehensive demonstration of CURSED stack collections
/// 
/// This example showcases all stack variants:
/// - Stack<T>: Dynamic LIFO stack
/// - FixedStack<T>: Fixed-capacity stack
/// - ThreadSafeStack<T>: Concurrent stack
/// - StackWithMin<T>: Stack that tracks minimum in O(1)

import "stdlib::collections::stacks";
import "stdlib::io::console";

slay demo_basic_stack() -> Result<(), String> {
    println("=== Basic Stack Demo ===")?;
    
    sus mut stack = Stack::new();
    
    // Push elements
    stack.push(10);
    stack.push(20);
    stack.push(30);
    
    printf("Stack contents: {}\n", &[stack.to_string()])?;
    printf("Top element: {:?}\n", &[stack.peek()])?;
    printf("Stack size: {}\n", &[stack.len()])?;
    
    // Pop elements (LIFO order)
    lowkey (!stack.is_empty()) {
        facts popped = stack.pop().unwrap();
        printf("Popped: {}, remaining: {}\n", &[popped, stack.len()])?;
    }
    
    println("")?;
    Ok(())
}

slay demo_fixed_stack() -> Result<(), String> {
    println("=== Fixed Stack Demo ===")?;
    
    sus mut stack = FixedStack::new(3)?;
    
    // Fill to capacity
    stack.push(1)?;
    stack.push(2)?;
    stack.push(3)?;
    
    printf("Fixed stack: {}\n", &[stack.to_string()])?;
    printf("Is full: {}\n", &[stack.is_full()])?;
    printf("Remaining capacity: {}\n", &[stack.remaining_capacity()])?;
    
    // Try to exceed capacity
    lowkey (stack.push(4).is_err()) {
        println("Cannot push - stack is full!")?;
    }
    
    // Pop one and try again
    facts popped = stack.pop().unwrap();
    printf("Popped: {}, now can push again\n", &[popped])?;
    stack.push(4)?;
    printf("After push: {}\n", &[stack.to_string()])?;
    
    println("")?;
    Ok(())
}

slay demo_stack_with_min() -> Result<(), String> {
    println("=== Stack with Minimum Tracking Demo ===")?;
    
    sus mut stack = StackWithMin::new();
    
    // Push elements and track minimum
    facts values = [15, 3, 8, 1, 12, 7];
    bestie value in values {
        stack.push(value);
        printf("Pushed: {}, current min: {:?}\n", &[value, stack.min()])?;
    }
    
    printf("Final stack: {}\n", &[stack.to_string()])?;
    
    // Pop elements and observe minimum changes
    lowkey (!stack.is_empty()) {
        facts popped = stack.pop().unwrap();
        printf("Popped: {}, new min: {:?}\n", &[popped, stack.min()])?;
    }
    
    println("")?;
    Ok(())
}

slay demo_thread_safe_stack() -> Result<(), String> {
    println("=== Thread-Safe Stack Demo ===")?;
    
    facts stack = ThreadSafeStack::new();
    
    // Add elements
    stack.push(100)?;
    stack.push(200)?;
    stack.push(300)?;
    
    printf("Thread-safe stack: {}\n", &[stack.to_string()])?;
    printf("Stack length: {}\n", &[stack.len()?])?;
    
    // Peek with transformation
    facts doubled = stack.peek(|x| x * 2)?.unwrap_or(0);
    printf("Top element doubled: {}\n", &[doubled])?;
    
    // Get snapshot
    facts snapshot = stack.snapshot()?;
    printf("Snapshot: {:?}\n", &[snapshot])?;
    
    // Pop elements
    lowkey (!stack.is_empty()?) {
        facts popped = stack.pop()?.unwrap();
        printf("Popped: {}, remaining: {}\n", &[popped, stack.len()?])?;
    }
    
    println("")?;
    Ok(())
}

slay demo_bulk_operations() -> Result<(), String> {
    println("=== Bulk Operations Demo ===")?;
    
    sus mut stack = Stack::new();
    
    // Push many elements at once
    facts data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    stack.push_many(data);
    printf("After push_many: {}\n", &[stack.to_string()])?;
    
    // Pop many elements
    facts popped = stack.pop_many(4);
    printf("Popped 4 elements: {:?}\n", &[popped])?;
    printf("Remaining stack: {}\n", &[stack.to_string()])?;
    
    // Peek at multiple elements
    facts peeked = stack.peek_many(3);
    printf("Top 3 elements: {:?}\n", &[peeked])?;
    
    println("")?;
    Ok(())
}

slay demo_stack_operations() -> Result<(), String> {
    println("=== Special Stack Operations Demo ===")?;
    
    sus mut stack = Stack::new();
    stack.push(42);
    
    // Duplicate top element
    stack.dup()?;
    printf("After dup: {}\n", &[stack.to_string()])?;
    
    stack.push(13);
    printf("After push 13: {}\n", &[stack.to_string()])?;
    
    // Swap top two elements
    stack.swap()?;
    printf("After swap: {}\n", &[stack.to_string()])?;
    
    println("")?;
    Ok(())
}

slay demo_iterators() -> Result<(), String> {
    println("=== Iterator Demo ===")?;
    
    sus mut stack = Stack::new();
    stack.push_many([5, 10, 15, 20, 25]);
    
    printf("Stack: {}\n", &[stack.to_string()])?;
    
    print("Elements (top to bottom): ")?;
    bestie (i, value) in stack.iter().enumerate() {
        lowkey (i > 0) { print(", ")?; }
        printf("{}", &[value])?;
    }
    println("")?;
    
    // Convert to vector
    facts vec = stack.into_vec();
    printf("As vector: {:?}\n", &[vec])?;
    
    println("")?;
    Ok(())
}

slay demo_practical_example() -> Result<(), String> {
    println("=== Practical Example: Expression Evaluation ===")?;
    
    // Simulate evaluating a postfix expression: 3 4 + 2 *
    // Should result in: (3 + 4) * 2 = 14
    sus mut eval_stack = Stack::new();
    facts expression = ["3", "4", "+", "2", "*"];
    
    printf("Evaluating postfix expression: {}\n", &[expression.join(" ")])?;
    
    bestie token in expression {
        vibe_check token {
            mood "+" => {
                facts b = eval_stack.pop().unwrap();
                facts a = eval_stack.pop().unwrap();
                eval_stack.push(a + b);
                printf("  {} + {} = {}\n", &[a, b, a + b])?;
            }
            mood "*" => {
                facts b = eval_stack.pop().unwrap();
                facts a = eval_stack.pop().unwrap();
                eval_stack.push(a * b);
                printf("  {} * {} = {}\n", &[a, b, a * b])?;
            }
            basic => {
                facts num = token.parse::<i32>().unwrap();
                eval_stack.push(num);
                printf("  Push {}\n", &[num])?;
            }
        }
    }
    
    facts result = eval_stack.pop().unwrap();
    printf("Final result: {}\n", &[result])?;
    
    println("")?;
    Ok(())
}

slay main() -> Result<(), String> {
    println("🔥 CURSED Stack Collections Demo 🔥")?;
    println("=====================================")?;
    println("")?;
    
    demo_basic_stack()?;
    demo_fixed_stack()?;
    demo_stack_with_min()?;
    demo_thread_safe_stack()?;
    demo_bulk_operations()?;
    demo_stack_operations()?;
    demo_iterators()?;
    demo_practical_example()?;
    
    println("✅ All stack demos completed successfully!")?;
    Ok(())
}
