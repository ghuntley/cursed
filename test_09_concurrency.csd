# Test 9: Concurrency (stan, dm channels)
sus ch drip_chan = dm_create()

stan {
    dm_send(ch, 42)
    vibez.spill("Sent 42")
}

sus value drip = dm_recv(ch)
vibez.spill("Received: " + value)
