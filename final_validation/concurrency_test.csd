slay worker_function(id drip) {
    sus i drip = 0
    bestie (i < 100) {
        sus computation drip = i * id
        i = i + 1
    }
    vibez.spill("Worker", id, "completed")
}

slay concurrency_main() {
    vibez.spill("Starting concurrency test")
    
    stan {
        worker_function(1)
    }
    
    stan {
        worker_function(2)
    }
    
    vibez.spill("Main thread completed")
}

concurrency_main()
