fr fr Advanced concurrency memory test
yeet "testz"

test_start("Concurrency Memory Test")

fr fr Create multiple channels and goroutines
sus channels []channel<normie> = []
sus results []normie = []

bestie i := 0; i < 20; i = i + 1 {
    sus ch = make_channel<normie>()
    channels.push(ch)
    
    fr fr Spawn goroutine for each channel
    stan {
        bestie j := 0; j < 100; j = j + 1 {
            dm_send(ch, i * j)
        }
        dm_close(ch)
    }
}

fr fr Collect from all channels
bestie ch in channels {
    bestie true {
        select {
            case value, ok := dm_recv(ch):
                if !ok { vibes }
                results.push(value)
            default:
                vibes
        }
    }
}

assert_true(results.len() > 0)
print_test_summary()
