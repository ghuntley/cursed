slay test_yolo_statements() {
    # Test return statement without value
    yolo
    
    # Test return statement with value
    yolo 42
    
    # Test return statement with expression
    yolo 2 + 3
}

slay test_break_continue() {
    bestie i := 0; i < 10; i++ {
        lowkey i == 5 {
            ghosted  # break without label
        }
        
        lowkey i == 3 {
            simp    # continue without label
        }
    }
}

slay test_labeled_break_continue() {
    outer_loop: bestie i := 0; i < 10; i++ {
        inner_loop: bestie j := 0; j < 10; j++ {
            lowkey i == 5 && j == 5 {
                ghosted outer_loop  # break with label
            }
            
            lowkey i == 3 && j == 3 {
                simp inner_loop     # continue with label
            }
        }
    }
}

vibez.spill("Parser statement tests compiled successfully!")
