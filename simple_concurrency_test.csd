fr fr Test basic goroutine functionality
sus ch = make_channel()

fr fr Spawn goroutine that sends a message
stan {
    send_channel(ch, 42)
    vibez.spill("Goroutine sent 42")
}

fr fr Receive message in main thread
sus result = recv_channel(ch)
vibez.spill("Received:", result)

fr fr Test multiple goroutines
sus ch2 = make_channel() 

stan {
    send_channel(ch2, 100)
    vibez.spill("Goroutine 1 sent 100")
}

stan {
    send_channel(ch2, 200)
    vibez.spill("Goroutine 2 sent 200")
}

sus val1 = recv_channel(ch2)
sus val2 = recv_channel(ch2)
vibez.spill("Received from goroutines:", val1, val2)
