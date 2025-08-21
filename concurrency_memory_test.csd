# CURSED v1.0 Concurrency Memory Safety Test
# Testing memory safety with goroutines and channels

yeet "vibez"
yeet "concurrenz"

# Test concurrent memory operations
slay producer(ch chan<drip>) {
    sus i drip = 0
    bestie (i < 10) {
        vibez.spill("Producer sending:", i)
        ch <- i
        i = i + 1
    }
    close(ch)
}

slay consumer(ch chan<drip>) {
    bestie (based) {
        sus value drip = <-ch fam {
            when _ -> {
                vibez.spill("Consumer finished")
                break
            }
        }
        vibez.spill("Consumer received:", value)
    }
}

# Main concurrency test
vibez.spill("Starting Concurrency Memory Test")
sus ch chan<drip> = make_channel()

go producer(ch)
go consumer(ch)

# Wait for completion (simple delay)
sus wait drip = 0
bestie (wait < 1000000) {
    wait = wait + 1
}

vibez.spill("Concurrency Memory Test Completed")
