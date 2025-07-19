fr fr Producer-Consumer Pattern Example
fr fr Demonstrates basic channel communication between goroutines

func producer(out dm<int>, count int) {
    print("Producer starting to generate", count, "numbers")
    
    for i := 0; i < count; i++ {
        print("Producing:", i)
        out <- i
        
        // Simulate some work
        sleep(100) // 100ms
    }
    
    close(out)
    print("Producer finished")
}

func consumer(in dm<int>, id int) {
    print("Consumer", id, "starting")
    
    for value := range in {
        print("Consumer", id, "received:", value)
        
        // Simulate processing work
        sleep(50) // 50ms
    }
    
    print("Consumer", id, "finished")
}

func main() {
    facts itemCount = 10
    facts numConsumers = 3
    
    // Create a buffered channel
    facts items = make(dm<int>, 5)
    
    // Start producer goroutine
    stan producer(items, itemCount)
    
    // Start multiple consumer goroutines
    for i := 0; i < numConsumers; i++ {
        stan consumer(items, i)
    }
    
    // Wait for all goroutines to complete
    // In a real implementation, we'd use sync.WaitGroup
    sleep(2000) // 2 seconds
    
    print("All work completed!")
}
