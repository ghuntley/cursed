// Concurrency constructs
stan {
    vibez.spill("Goroutine executing")
}

sus ch drip_chan = make(drip_chan)
dm_send(ch, 42)
sus value drip = dm_recv(ch)

ready {
    case value := dm_recv(ch):
        vibez.spill("Received:", value)
    case timeout(1000):
        vibez.spill("Timeout")
}
