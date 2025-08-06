slay main_character() {
    vibez.spill("Testing comprehensive statement compilation")
    
    // Test ForIn loop
    sus numbers = [10, 20, 30]
    bestie num yeet numbers {
        vibez.spill("Number:")
        vibez.spill(num)
    }
    
    // Test Switch statement
    sus choice drip = 2
    vibe_check choice {
        case 1:
            vibez.spill("Choice one")
        case 2:
            vibez.spill("Choice two selected")
        default:
            vibez.spill("Other choice")
    }
    
    // Test Channel operations
    sus ch = dm(drip, 3)
    ch <- 100
    sus result = <-ch
    vibez.spill("Channel result:")
    vibez.spill(result)
    
    // Test Select statement
    sus ch1 = dm(drip, 2)
    sus ch2 = dm(drip, 2)
    
    vibe_check_multi {
        case ch1 <- 42:
            vibez.spill("Sent 42 to ch1")
        case value := <-ch2:
            vibez.spill("Received from ch2")
        default:
            vibez.spill("No channel ready")
    }
    
    vibez.spill("All statement types work correctly!")
}
