// Simple panic/recover test

slay main() {
    vibez.spill("Testing panic/recover system...")
    
    // Test basic panic
    fam {
        yikes "test_error" := "This is a test panic"
    } {
        vibez.spill("Panic was recovered!")
    }
    
    vibez.spill("Panic/recover test completed!")
}
