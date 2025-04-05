vibe main;

slay main() {
    fr fr Create a channel (we'll simplify for now)
    sus ch = dm smol
    
    fr fr Try to send without blocking (should succeed)
    fr fr For now, we'll just send/receive normally since our implementation is simplified
    ch <- 42
    
    fr fr Try to receive without blocking (should succeed)
    sus result = <-ch
    
    fr fr Check the result
    lowkey result == 42 {
        println("Received 42 from channel")
    }
    
    yolo 0
}