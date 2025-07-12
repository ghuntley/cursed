# Test current select statement implementation status
yeet "vibez"
yeet "testz"

slay main() {
    test_start("Select Statement Implementation Test")
    
    vibez.spill("Testing select statement parsing and execution...")
    
    # Test 1: Basic select statement with ready/basic keywords
    sus ch := dm_buffered<normie>(1)
    ch.send(42)
    
    ready {
        case val := <-ch:
            vibez.spill("✅ Select with ready keyword working: " + val)
            assert_eq_int(val, 42)
        basic:
            vibez.spill("❌ Should not reach basic case")
    }
    
    # Test 2: Test mood keyword (if supported)
    sus ch2 := dm_buffered<tea>(1)  
    ch2.send("hello")
    
    ready {
        mood val := <-ch2:
            vibez.spill("✅ Select with mood keyword working: " + val)
            assert_eq_string(val, "hello")
        basic:
            vibez.spill("❌ Should not reach basic case")
    }
    
    # Test 3: Multiple channel selection
    sus ch3 := dm_buffered<normie>(1)
    sus ch4 := dm_buffered<normie>(1)
    
    ch3.send(100)
    
    ready {
        case val1 := <-ch3:
            vibez.spill("✅ Multi-channel select working: ch3=" + val1)
        case val2 := <-ch4:
            vibez.spill("Received from ch4: " + val2)
        basic:
            vibez.spill("❌ Should not reach basic case")
    }
    
    print_test_summary()
}

main()
