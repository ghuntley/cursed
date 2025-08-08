sus ch dm[drip] = dm_make[drip](1)
stan {
    vibez.spill("Goroutine sending value")
    dm_send(ch, 42)
}
sus value drip = dm_recv(ch)
vibez.spill("Received value:", value)
