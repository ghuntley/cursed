// Test LLVM compilation of shook/fam error handling
slay test_llvm_error_handling() drip {
    shook {
        sus risky drip = 42 / 2  // Safe division
        damn risky
    } fam err {
        vibez.spill("Caught LLVM error:", err)
        damn -1
    }
}

slay main() drip {
    sus result drip = test_llvm_error_handling()
    vibez.spill("LLVM test result:", result)
    damn 0
}
