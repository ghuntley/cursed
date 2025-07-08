yeet "vibez"

slay main() {
    sus ch dm<normie> = make(dm<normie>)
    
    vibe_check {
        mood x := <-ch:
            vibez.spill("Received:", x)
        mood ch <- 42:
            vibez.spill("Sent 42")
        basic:
            vibez.spill("Default case")
    }
}
