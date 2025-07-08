yeet "vibez"

slay main() {
    sus ch dm<normie> = make(dm<normie>)
    
    select {
        ready x := <-ch:
            vibez.spill("Received:", x)
        ready ch <- 42:
            vibez.spill("Sent 42")
        basic:
            vibez.spill("Default case")
    }
}
