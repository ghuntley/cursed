fr fr Fan-In and Fan-Out Patterns Example
fr fr Demonstrates merging multiple inputs and distributing to multiple outputs

fr fr Fan-In: Multiple inputs -> Single output
func fanIn(input1 dm<string>, input2 dm<string>, input3 dm<string>, output dm<string>) {
    print("Fan-In: Starting to merge inputs")
    
    facts activeChannels = 3
    
    for activeChannels > 0 {
        vibe_check {
            mood msg := <-input1:
                if msg != "" {
                    output <- "Input1: " + msg
                } else {
                    activeChannels--
                    print("Fan-In: Input1 closed")
                }
            
            mood msg := <-input2:
                if msg != "" {
                    output <- "Input2: " + msg
                } else {
                    activeChannels--
                    print("Fan-In: Input2 closed")
                }
            
            mood msg := <-input3:
                if msg != "" {
                    output <- "Input3: " + msg
                } else {
                    activeChannels--
                    print("Fan-In: Input3 closed")
                }
            
            basic:
                sleep(10) // Brief pause when no channels ready
        }
    }
    
    close(output)
    print("Fan-In: Finished merging")
}

fr fr Fan-Out: Single input -> Multiple outputs
func fanOut(input dm<string>, output1 dm<string>, output2 dm<string>, output3 dm<string>) {
    print("Fan-Out: Starting to distribute messages")
    
    for msg := range input {
        print("Fan-Out: Distributing:", msg)
        
        // Send to all outputs (broadcast)
        output1 <- msg
        output2 <- msg
        output3 <- msg
    }
    
    close(output1)
    close(output2)
    close(output3)
    print("Fan-Out: Finished distributing")
}

func inputGenerator(id int, output dm<string>) {
    print("Generator", id, "starting")
    
    for i := 0; i < 5; i++ {
        msg := fmt.sprintf("Message %d from Generator %d", i, id)
        output <- msg
        print("Generator", id, "sent:", msg)
        sleep(100 + (id * 50)) // Different timing per generator
    }
    
    close(output)
    print("Generator", id, "finished")
}

func outputConsumer(id int, input dm<string>) {
    print("Consumer", id, "starting")
    
    for msg := range input {
        print("Consumer", id, "received:", msg)
        sleep(80) // Simulate processing
    }
    
    print("Consumer", id, "finished")
}

func main() {
    print("Starting Fan-In/Fan-Out example...")
    
    // === Fan-In Example ===
    print("\n=== Fan-In Pattern ===")
    
    // Create input channels for fan-in
    facts input1 = make(dm<string>, 2)
    facts input2 = make(dm<string>, 2)
    facts input3 = make(dm<string>, 2)
    facts merged = make(dm<string>, 5)
    
    // Start input generators
    stan inputGenerator(1, input1)
    stan inputGenerator(2, input2)
    stan inputGenerator(3, input3)
    
    // Start fan-in process
    stan fanIn(input1, input2, input3, merged)
    
    // Consume merged output
    stan func() {
        print("Merged consumer starting")
        for msg := range merged {
            print("MERGED:", msg)
        }
        print("Merged consumer finished")
    }()
    
    // Wait for fan-in to complete
    sleep(2000)
    
    // === Fan-Out Example ===
    print("\n=== Fan-Out Pattern ===")
    
    // Create channels for fan-out
    facts source = make(dm<string>, 3)
    facts out1 = make(dm<string>, 3)
    facts out2 = make(dm<string>, 3)
    facts out3 = make(dm<string>, 3)
    
    // Start fan-out process
    stan fanOut(source, out1, out2, out3)
    
    // Start output consumers
    stan outputConsumer(1, out1)
    stan outputConsumer(2, out2)
    stan outputConsumer(3, out3)
    
    // Send messages to source
    stan func() {
        print("Source generator starting")
        for i := 0; i < 4; i++ {
            msg := fmt.sprintf("Broadcast message %d", i)
            source <- msg
            print("SOURCE: Sent", msg)
            sleep(200)
        }
        close(source)
        print("Source generator finished")
    }()
    
    // Wait for fan-out to complete
    sleep(3000)
    
    print("\nFan-In/Fan-Out example completed!")
}
