slay concurrency_example() {
    // Channel creation
    sus ch dm normie = make(dm normie, 10)
    
    // Goroutine spawning
    stan producer(ch)
    stan consumer(ch)
    
    // Channel operations
    ch <- 42        // Send
    sus value = <-ch // Receive
    
    // Buffered channel
    sus buf_ch = make(dm tea, 5)
    buf_ch <- "message"
}

slay producer(ch dm normie) {
    bestie i := 0; i < 10; i++ {
        ch <- i
    }
    close(ch)
}

slay consumer(ch dm normie) {
    bestie value := flex ch {
        print("Received:", value)
    }
}