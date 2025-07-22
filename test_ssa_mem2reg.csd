fr fr Test SSA form generation with mem2reg pass
slay test_ssa_form() {
    sus x drip = 42
    sus y drip = x + 10
    
    x = y * 2
    
    damn x + y
}

slay main() {
    sus result drip = test_ssa_form()
    vibez.spill(result)
}
