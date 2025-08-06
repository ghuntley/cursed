sus ch dm<normie> = dm_create(1)
stan {
    dm_send(ch, 42)
}
sus value normie = dm_recv(ch)
vibez.spill("Received: ", value)
