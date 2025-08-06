#!/usr/bin/env cursed
# Enhanced Self-Hosting Test Program
# Tests advanced compilation features

yeet "testz"
yeet "vibez"

vibe "enhanced_self_hosting_test"

# Test struct compilation
squad TestStruct {
    spill name tea
    spill value normie
    spill active lit
}

# Test interface compilation  
collab TestInterface {
    slay process() tea
    slay validate(input tea) lit
}

# Test generic function
slay generic_function<T>(input T) T {
    damn input
}

# Test complex function with multiple features
slay complex_computation(data TestStruct) TestStruct {
    sus result TestStruct = TestStruct{
        name: "computed_" + data.name,
        value: data.value * 2,
        active: !data.active
    }
    damn result
}

# Test pattern matching
slay test_pattern_matching(value normie) tea {
    ready value {
        1 => damn "one"
        2 => damn "two"
        3 => damn "three"
        _ => damn "other"
    }
}

# Test error handling
slay risky_operation(input normie) (normie, tea) {
    lowkey (input < 0) {
        damn (0, "negative input not allowed")
    }
    damn (input * 2, "")
}

# Test concurrency
slay test_goroutines() {
    sus ch chan<normie> = make_chan<normie>(5)
    
    # Start goroutine
    stan {
        bestie i := 0; i < 5; i = i + 1 {
            ch <- i
        }
        close(ch)
    }
    
    # Receive from channel
    bestie {
        sus (value, ok) = <-ch
        lowkey (!ok) {
            break
        }
        vibez.spill("Received: " + value)
    }
}

# Main function testing compilation capabilities
slay main() {
    vibez.spill("=== Enhanced Self-Hosting Test ===")
    
    # Test basic struct operations
    sus test_data TestStruct = TestStruct{
        name: "test",
        value: 42,
        active: based
    }
    
    vibez.spill("Original struct: " + test_data.name + " = " + test_data.value)
    
    # Test complex computation
    sus computed TestStruct = complex_computation(test_data)
    vibez.spill("Computed struct: " + computed.name + " = " + computed.value)
    
    # Test pattern matching
    sus pattern_result tea = test_pattern_matching(2)
    vibez.spill("Pattern matching result: " + pattern_result)
    
    # Test generic function
    sus generic_int normie = generic_function<normie>(100)
    sus generic_str tea = generic_function<tea>("hello")
    vibez.spill("Generic int: " + generic_int)
    vibez.spill("Generic string: " + generic_str)
    
    # Test error handling
    sus (result, error) = risky_operation(10)
    lowkey (error.length() > 0) {
        vibez.spill("Error: " + error)
    } highkey {
        vibez.spill("Safe operation result: " + result)
    }
    
    # Test concurrency
    vibez.spill("Testing goroutines...")
    test_goroutines()
    
    # Test arrays
    sus numbers []normie = [1, 2, 3, 4, 5]
    sus sum normie = 0
    bestie i := 0; i < numbers.length(); i = i + 1 {
        sum = sum + numbers[i]
    }
    vibez.spill("Array sum: " + sum)
    
    # Test maps
    sus data_map map[tea]normie = {
        "first": 1,
        "second": 2,
        "third": 3
    }
    vibez.spill("Map size: " + data_map.size())
    
    # Test tuples
    sus tuple_data := (42, "test", based)
    vibez.spill("Tuple: (" + tuple_data.0 + ", " + tuple_data.1 + ", " + tuple_data.2 + ")")
    
    vibez.spill("=== All enhanced tests completed successfully ===")
}
