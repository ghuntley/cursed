vibe main

slay main() {
    fr fr Create a buffered channel with capacity 1
    sus ch = dm smol[1]
    
    fr fr Try to send without blocking (should succeed)
    sus sent = ch.try_send(42)
    
    fr fr Try to send again without blocking (would block, returns false)
    sus sent_again = ch.try_send(43)
    
    fr fr Try to receive without blocking (should succeed)
    sus result_opt = ch.try_receive()
    
    fr fr Check if receive returned a value
    lowkey result_opt.has_value() {
        sus result = result_opt.value()
    }
    
    yolo 0
}