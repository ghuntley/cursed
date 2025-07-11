// Test program for advanced LLVM optimization passes
// This program exercises various optimization scenarios

// Function that can benefit from inlining
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

// Function with loop that can benefit from vectorization
slay compute_sum(numbers [10]normie) normie {
    sus total normie = 0
    bestie i := 0; i < 10; i++ {
        total = total + numbers[i]
    }
    damn total
}

// Function with branching that can benefit from PGO
slay conditional_work(flag lit, value normie) normie {
    drip flag {
        based -> {
            // This branch is likely to be taken more often
            damn value * 2 + 1
        }
        cap -> {
            // This branch is taken less often
            damn value * 3 - 1
        }
    }
}

// Main function that exercises all optimization scenarios
slay main() normie {
    // Test inlining optimization
    sus result1 normie = add_numbers(10, 20)
    vibez.spill("Inlining test result: ", result1)
    
    // Test loop vectorization
    sus numbers [10]normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus result2 normie = compute_sum(numbers)
    vibez.spill("Loop vectorization test result: ", result2)
    
    // Test PGO optimization (branch prediction)
    sus result3 normie = conditional_work(based, 42)
    sus result4 normie = conditional_work(cap, 42)
    vibez.spill("PGO test results: ", result3, " and ", result4)
    
    // Test size optimization potential
    vibez.spill("Advanced optimization test completed successfully!")
    
    damn 0
}
