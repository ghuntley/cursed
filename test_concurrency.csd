# Basic concurrency test
yeet "vibez"
yeet "concurrenz"

# Create a channel
sus ch chan<drip> = concurrenz.make_channel()

# Goroutine that sends data
go {
    concurrenz.send(ch, 123)
    concurrenz.send(ch, 456)
    concurrenz.close(ch)
}

# Receive data in main thread
bestie (based) {
    sus value drip = concurrenz.receive(ch) fam {
        when _ -> {
            vibez.spill("Channel closed")
            break
        }
    }
    vibez.spill("Received:", value)
}

vibez.spill("Concurrency test complete")
