// Select Statement Demonstration
// Shows how to handle multiple channel operations with vibe_check

func slowSender(ch dm<string>, delay int, prefix string) {
    for i := 0; i < 5; i++ {
        sleep(delay)
        msg := fmt.sprintf("%s-%d", prefix, i)
        ch <- msg
        print("Slow sender sent:", msg)
    }
    close(ch)
}

func fastSender(ch dm<int>, delay int) {
    for i := 0; i < 10; i++ {
        sleep(delay)
        ch <- i * 10
        print("Fast sender sent:", i * 10)
    }
    close(ch)
}

func selectDemo() {
    print("=== Select Demo Starting ===")
    
    facts stringChan = make(dm<string>, 2)
    facts intChan = make(dm<int>, 2)
    facts done = make(dm<bool>, 1)
    
    // Start senders with different timing
    stan slowSender(stringChan, 300, "SLOW")  // 300ms delay
    stan fastSender(intChan, 100)              // 100ms delay
    
    // Start a timer
    stan func() {
        sleep(2000) // 2 seconds
        done <- true
        print("Timer expired!")
    }()
    
    facts stringClosed = false
    facts intClosed = false
    facts timerExpired = false
    
    // Select loop
    for !timerExpired && (!stringClosed || !intClosed) {
        vibe_check {
            mood msg := <-stringChan:
                if msg != "" {
                    print("SELECT: Received string:", msg)
                } else {
                    stringClosed = true
                    print("SELECT: String channel closed")
                }
            
            mood num := <-intChan:
                if num >= 0 {
                    print("SELECT: Received int:", num)
                } else {
                    intClosed = true
                    print("SELECT: Int channel closed")
                }
            
            mood <-done:
                timerExpired = true
                print("SELECT: Timer expired, exiting")
            
            basic:
                print("SELECT: No channels ready, waiting...")
                sleep(50)
        }
    }
    
    print("=== Select Demo Finished ===")
}

func timeoutDemo() {
    print("\n=== Timeout Demo Starting ===")
    
    facts dataChan = make(dm<string>, 1)
    facts timeout = make(dm<bool>, 1)
    
    // Start a delayed sender
    stan func() {
        sleep(1500) // 1.5 seconds delay
        dataChan <- "Important data"
        print("Data sent (might be too late)")
    }()
    
    // Start timeout timer
    stan func() {
        sleep(1000) // 1 second timeout
        timeout <- true
        print("Timeout triggered")
    }()
    
    vibe_check {
        mood data := <-dataChan:
            print("TIMEOUT DEMO: Received data in time:", data)
        
        mood <-timeout:
            print("TIMEOUT DEMO: Operation timed out!")
        
        basic:
            print("TIMEOUT DEMO: Still waiting...")
    }
    
    print("=== Timeout Demo Finished ===")
}

func nonBlockingDemo() {
    print("\n=== Non-blocking Demo Starting ===")
    
    facts ch = make(dm<int>, 2)
    
    // Fill the channel
    ch <- 1
    ch <- 2
    print("Channel filled with 2 items")
    
    // Try non-blocking operations
    for i := 3; i <= 5; i++ {
        vibe_check {
            // Try to send
            ch <- i:
                print("NON-BLOCKING: Successfully sent", i)
            
            basic:
                print("NON-BLOCKING: Channel full, couldn't send", i)
        }
    }
    
    // Drain the channel
    for len(ch) > 0 {
        vibe_check {
            mood value := <-ch:
                print("NON-BLOCKING: Received", value)
            
            basic:
                print("NON-BLOCKING: Channel empty")
                break
        }
    }
    
    print("=== Non-blocking Demo Finished ===")
}

func priorityDemo() {
    print("\n=== Priority Demo Starting ===")
    
    facts highPriority = make(dm<string>, 3)
    facts lowPriority = make(dm<string>, 3)
    
    // Send to both channels
    stan func() {
        highPriority <- "HIGH-1"
        sleep(100)
        lowPriority <- "LOW-1"
        sleep(100)
        highPriority <- "HIGH-2"
        sleep(100)
        lowPriority <- "LOW-2"
        
        close(highPriority)
        close(lowPriority)
    }()
    
    // Process with priority (check high priority first)
    facts processing = true
    for processing {
        vibe_check {
            mood msg := <-highPriority:
                if msg != "" {
                    print("PRIORITY: HIGH priority message:", msg)
                } else {
                    print("PRIORITY: High priority channel closed")
                }
            
            basic:
                // Only check low priority if high priority not available
                vibe_check {
                    mood msg := <-lowPriority:
                        if msg != "" {
                            print("PRIORITY: LOW priority message:", msg)
                        } else {
                            print("PRIORITY: Low priority channel closed")
                            processing = false
                        }
                    
                    basic:
                        print("PRIORITY: No messages available")
                        sleep(50)
                }
        }
    }
    
    print("=== Priority Demo Finished ===")
}

func main() {
    print("Channel Select Patterns Demonstration")
    print("=====================================")
    
    selectDemo()
    timeoutDemo()
    nonBlockingDemo()
    priorityDemo()
    
    print("\nAll select pattern demos completed!")
}
