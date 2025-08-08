# Test advanced concurrency patterns
vibez.spill("Testing advanced concurrency patterns")

# Test multiple goroutines with coordination
sus counter drip = 0

stan {
    vibez.spill("Worker 1 starting")
    sus i drip = 0
    bestie (i < 3) {
        vibez.spill("Worker 1 iteration:", i)
        i = i + 1
    }
    vibez.spill("Worker 1 finished")
}

stan {
    vibez.spill("Worker 2 starting") 
    sus j drip = 0
    bestie (j < 3) {
        vibez.spill("Worker 2 iteration:", j)
        j = j + 1  
    }
    vibez.spill("Worker 2 finished")
}

vibez.spill("Main thread continues execution")
vibez.spill("All workers started")
