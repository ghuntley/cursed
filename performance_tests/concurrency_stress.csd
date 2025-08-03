slay worker_task(id drip) {
    sus i drip = 0
    bestie (i < 1000) {
        sus computation drip = i * id
        i = i + 1
    }
    vibez.spill("Worker", id, "completed")
}

slay concurrency_test() {
    sus worker_count drip = 0
    bestie (worker_count < 10) {
        stan {
            worker_task(worker_count)
        }
        worker_count = worker_count + 1
    }
    vibez.spill("All workers started")
}

concurrency_test()
