// Test defer functionality
slay main() {
    vibez.spill("Starting function")
    later vibez.spill("This should execute at function end")
    vibez.spill("About to return")
}
