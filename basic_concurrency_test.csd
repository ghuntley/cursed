fr fr Basic CURSED Concurrency Test
fr fr Simple test to verify concurrency features work

yeet "testz"

fr fr Test 1: Basic goroutine
slay test_basic_goroutine() {
    test_start("Basic Goroutine")
    
    vibez.spill("Main: About to spawn goroutine")
    
    stan {
        vibez.spill("Goroutine: Hello from inside goroutine!")
        vibez.spill("Goroutine: This is executing concurrently")
    }
    
    vibez.spill("Main: Goroutine spawned")
    
    fr fr Give goroutine time to execute
    yolo()
    
    print_test_summary()
}

fr fr Test 2: Channel send and receive
slay test_basic_channel() {
    test_start("Basic Channel")
    
    fr fr Create unbuffered channel
    sus ch dm<normie>
    
    vibez.spill("Main: Created channel")
    
    fr fr Sender
    stan {
        vibez.spill("Sender: Sending value 42")
        dm_send(ch, 42)
        vibez.spill("Sender: Value sent!")
    }
    
    fr fr Receiver
    vibez.spill("Main: Waiting to receive...")
    sus received normie = dm_recv(ch)
    vibez.spillf("Main: Received value: {}", received)
    
    assert_eq_int(received, 42)
    
    print_test_summary()
}

fr fr Test 3: Simple select statement
slay test_basic_select() {
    test_start("Basic Select")
    
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<normie> = dm<normie>(1)
    
    vibez.spill("Main: Sending on ch1")
    dm_send(ch1, 100)
    
    vibez.spill("Main: Executing select")
    ready {
        mood value := dm_recv(ch1):
            vibez.spillf("Select: Received from ch1: {}", value)
            assert_eq_int(value, 100)
            
        mood value := dm_recv(ch2):
            vibez.spillf("Select: Received from ch2: {}", value)
            
        basic:
            vibez.spill("Select: Default case executed")
    }
    
    print_test_summary()
}

fr fr Main function
slay main() {
    vibez.spill("🧪 Basic CURSED Concurrency Test")
    vibez.spill("===============================")
    
    test_basic_goroutine()
    vibez.spill("")
    
    test_basic_channel()
    vibez.spill("")
    
    test_basic_select()
    vibez.spill("")
    
    vibez.spill("✅ Basic concurrency tests completed!")
}
