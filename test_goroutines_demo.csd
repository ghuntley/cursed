// CURSED Goroutine Demo Program
// This demonstrates the use of goroutines with "stan" and "yolo" keywords

package main

import (
    "runtime"
    "channels"
)

func worker(id int, ch chan int) {
    for i := 0; i < 3; i++ {
        print("Worker ", id, " iteration ", i)
        ch <- i * id  // Send work result to channel
        yolo()  // Yield control to scheduler
    }
    ch <- -1  // Signal completion
}

func main() {
    // Initialize the goroutine scheduler
    runtime.InitScheduler(4)  // 4 worker threads
    
    // Create a channel for communication
    ch := make(chan int, 10)
    
    // Spawn 3 goroutines using "stan" keyword
    stan { worker(1, ch) }
    stan { worker(2, ch) }  
    stan { worker(3, ch) }
    
    // Collect results from all workers
    completed := 0
    for completed < 3 {
        result := <-ch
        if result == -1 {
            completed++
            print("Worker completed")
        } else {
            print("Received result:", result)
        }
        yolo()  // Cooperative yield
    }
    
    print("All goroutines completed")
    runtime.ShutdownScheduler()
}
