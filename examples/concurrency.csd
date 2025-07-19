vibe main

fr fr Define a buffered channel
sus ch = make(dm<tea>, 5)

fr fr Define a simple worker that processes messages
slay worker(id normie, ch dm<tea>) {
    periodt based {
        sus msg = <-ch
        puts(sprintf("Worker %d received: %s", id, msg))
    }
}

fr fr Define a function to send messages to workers
slay send_messages(ch dm<tea>, count normie) {
    sus i = 0
    periodt i < count {
        ch <- sprintf("Message %d", i)
        i = i + 1
    }
}

fr fr Main function demonstrates goroutines and channels
slay main() {
    fr fr Start some worker goroutines
    stan worker(1, ch)
    stan worker(2, ch)
    
    fr fr Send messages
    send_messages(ch, 10)
    
    fr fr Wait a bit for workers to process messages
    sleep(1000)  fr fr Sleep 1 second
    
    fr fr Close the channel when done
    close(ch)
    
    puts("Done!")
}