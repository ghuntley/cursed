yeet "concurrenz"
sus ch drip = make_channel(10)
stan {
    sus i drip = 0
    bestie (i < 5) {
        send_channel(ch, i)
        i = i + 1
    }
}
stan {
    sus j drip = 0
    bestie (j < 5) {
        sus val drip = recv_channel(ch)
        vibez.spill("Received:", val)
        j = j + 1
    }
}
vibez.spill("Main done")
