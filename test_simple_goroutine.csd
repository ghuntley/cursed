// Simple goroutine test

slay worker() {
    vibez.spill("Worker called")
}

vibez.spill("Starting goroutine")
stan worker()
vibez.spill("Program complete")
