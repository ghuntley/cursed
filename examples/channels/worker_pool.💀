fr fr Worker Pool Pattern Example
fr fr Demonstrates distributing work across multiple worker goroutines

type Job struct {
    id    int
    value int
}

type Result struct {
    jobId  int
    result int
}

func worker(id int, jobs dm<Job>, results dm<Result>) {
    print("Worker", id, "started")
    
    for job := range jobs {
        print("Worker", id, "processing job", job.id)
        
        // Simulate work: square the value
        processedValue := job.value * job.value
        
        // Simulate processing time
        sleep(100 + (id * 20)) // Variable sleep per worker
        
        results <- Result{
            jobId:  job.id,
            result: processedValue,
        }
        
        print("Worker", id, "completed job", job.id)
    }
    
    print("Worker", id, "finished")
}

func main() {
    facts numWorkers = 4
    facts numJobs = 12
    
    // Create channels
    facts jobs = make(dm<Job>, numJobs)
    facts results = make(dm<Result>, numJobs)
    
    // Start workers
    for w := 0; w < numWorkers; w++ {
        stan worker(w, jobs, results)
    }
    
    // Send jobs
    for j := 0; j < numJobs; j++ {
        job := Job{
            id:    j,
            value: j + 1,
        }
        jobs <- job
        print("Sent job", j)
    }
    close(jobs)
    
    // Collect results
    for r := 0; r < numJobs; r++ {
        result := <-results
        print("Job", result.jobId, "result:", result.result)
    }
    
    print("All jobs completed!")
}
