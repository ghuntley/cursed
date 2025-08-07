sus ch dm<drip> = make_channel()
stan {
    dm_send(ch, 42)
}
sus value drip = dm_recv(ch)
vibez.spill("Received:", value)
wait_all()
