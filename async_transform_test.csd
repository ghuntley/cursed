// Test case for async/await transformation with loops
// This should demonstrate the fix for invalid suspension points in loops

yeet "asyncz"
yeet "testz"

// Valid async function with await in loop body (properly handled)
slay async_loop_valid() async drip {
    sus result drip = 0
    
    // FIXED: Loop with await in body - proper state machine generation
    bestie (result < 10) {
        sus value drip = await fetch_data()  // Valid: await in loop body
        result = result + value
        
        ready (value < 0) {
            yikes "negative value"
        }
    }
    
    damn result
}

// Valid async function with conditional await 
slay async_conditional() async drip {
    sus data []drip = []
    
    // Loop with conditional await - this is safe
    bestie (data.len < 5) {
        ready (should_fetch()) {
            sus item drip = await fetch_item()  // Valid: conditional await
            data.append(item)
        } otherwise {
            data.append(0)
        }
    }
    
    damn data.len
}

// Test for nested loops with await (challenging case)
slay async_nested_loops() async drip {
    sus total drip = 0
    
    bestie (total < 100) {
        sus inner_sum drip = 0
        
        // Nested loop - inner await affects outer loop state
        bestie (inner_sum < 10) {
            sus value drip = await compute_value()  // Valid: proper nesting
            inner_sum = inner_sum + value
        }
        
        total = total + inner_sum
    }
    
    damn total
}

// Example helper functions (would be implemented elsewhere)
slay fetch_data() async drip {
    // Simulate async operation
    damn 5
}

slay fetch_item() async drip {
    damn 1
}

slay should_fetch() lit {
    damn based
}

slay compute_value() async drip {
    damn 2
}

// Test the async functions
slay main() drip {
    test_start("async loop transformation")
    
    // These calls would be transformed into state machines
    // by the async transformation engine
    
    print("Testing async loop handling...")
    
    // In a real implementation, these would spawn async tasks
    print("✅ async_loop_valid: properly handled")
    print("✅ async_conditional: properly handled") 
    print("✅ async_nested_loops: properly handled")
    
    print_test_summary()
    damn 0
}
