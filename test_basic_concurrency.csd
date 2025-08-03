fr fr Basic concurrency test
vibez.spill("Testing basic concurrency features")

stan {
    vibez.spill("Hello from goroutine!")
}

sus ch dm<normie> = dm<normie>(1)
dm_send(ch, 42)
sus value normie = dm_recv(ch)

vibez.spill("Test completed!")
