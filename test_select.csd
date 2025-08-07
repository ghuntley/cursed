sus ch1 dm<drip> = make_channel()
sus ch2 dm<drip> = make_channel()

stan {
    dm_send(ch1, 42)
}

ready {
    mood value1 := dm_recv(ch1):
        vibez.spill("Received from ch1:", value1)
    mood value2 := dm_recv(ch2):
        vibez.spill("Received from ch2:", value2)
    basic:
        vibez.spill("No channels ready")
}

wait_all()
