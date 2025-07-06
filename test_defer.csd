fr fr Test defer statements in CURSED

slay testDefer() {
    vibez.spill("Starting function")
    
    later vibez.spill("This will execute at the end")
    
    vibez.spill("Middle of function")
    
    later vibez.spill("This will execute second to last")
    
    vibez.spill("End of function")
}

slay main() {
    testDefer()
}
