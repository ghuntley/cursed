// Test goroutine parsing
vibe main

slay worker(id normie) {
    facts message = "Worker done"
    yolo
}

slay main() {
    // Spawn goroutine with function call
    stan worker(42)
    
    // Spawn goroutine with different expression
    stan println("Hello from goroutine!")
    
    yolo
}
