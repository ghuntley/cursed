fr fr CURSED Concurrency Integration Test
fr fr Tests: goroutines (stan), channels (dm), select statements (ready)
fr fr This program validates the complete concurrency system integration

fr fr Basic goroutine test
stan {
    vibez.spill("Hello from goroutine!")
}

fr fr Channel creation and operations
sus channel dm<normie> = dm<normie>(3)
dm_send(channel, 42)
dm_send(channel, 43)
dm_send(channel, 44)

sus value1 normie = dm_recv(channel)
sus value2 normie = dm_recv(channel) 
sus value3 normie = dm_recv(channel)

vibez.spill("Received from channel:", value1)
vibez.spill("Received from channel:", value2)
vibez.spill("Received from channel:", value3)

fr fr Select statement test
sus ch1 dm<normie> = dm<normie>(1)
sus ch2 dm<tea> = dm<tea>(1)

dm_send(ch1, 100)
dm_send(ch2, "test message")

ready {
    mood value := dm_recv(ch1):
        vibez.spill("Received from ch1:", value)
    mood msg := dm_recv(ch2):
        vibez.spill("Received from ch2:", msg)
    basic:
        vibez.spill("Default case executed")
}

fr fr Multiple goroutines with shared channel
sus shared_channel dm<normie> = dm<normie>(5)

stan {
    bestie i := 0; i < 3; i = i + 1 {
        dm_send(shared_channel, i * 10)
    }
}

stan {
    bestie j := 0; j < 3; j = j + 1 {
        sus received normie = dm_recv(shared_channel)
        vibez.spill("Worker 2 received:", received)
    }
}

fr fr Complex select with multiple channels
sus num_channel dm<normie> = dm<normie>(2)
sus str_channel dm<tea> = dm<tea>(2)
sus done_channel dm<lit> = dm<lit>(1)

fr fr Producer goroutines
stan {
    dm_send(num_channel, 123)
    dm_send(num_channel, 456)
}

stan {
    dm_send(str_channel, "first")
    dm_send(str_channel, "second")
}

stan {
    dm_send(done_channel, based)
}

fr fr Consumer with select
bestie i := 0; i < 5; i = i + 1 {
    ready {
        mood num := dm_recv(num_channel):
            vibez.spill("Number received:", num)
        mood str := dm_recv(str_channel):
            vibez.spill("String received:", str)
        mood done := dm_recv(done_channel):
            if done {
                vibez.spill("Done signal received, exiting")
                vibes
            }
        basic:
            vibez.spill("No channels ready")
    }
}

vibez.spill("Concurrency integration test completed!")
