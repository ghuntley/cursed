// Test CURSED error handling compilation to native binary

slay main() {
    vibez.spill("Testing compiled error handling...")
    
    fam {
        yikes "Compiled error test"
    } sus err {
        vibez.spill("Caught compiled error:", err)
    }
    
    sus error_result = yikes "Another error"
    sus propagated = shook error_result
    vibez.spill("Propagated in compilation:", propagated)
}
