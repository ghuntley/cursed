vibe main

slay main() {
    fr Test the property-based testing module
    vibez.spill("Testing the quick_test module")
    
    fr Basic random generators
    test_basic_generators()
    
    fr Property-based testing
    test_properties()
    
    vibez.spill("All quick_test tests passed!")
}

slay test_basic_generators() {
    vibez.spill("\nTesting basic generators...")
    
    fr Generate random integers in range
    vibez.spill("Random integers in range [-10, 10]:")
    vibecheck i := 0; i < 5; i++ {
        val := quick_test.int_range(-10, 10)
        vibez.spill(val)
    }
    
    fr Generate random booleans
    vibez.spill("\nRandom booleans:")
    vibecheck i := 0; i < 5; i++ {
        val := quick_test.boolean()
        vibez.spill(val)
    }
    
    fr Generate random strings
    vibez.spill("\nRandom strings:")
    vibecheck i := 0; i < 5; i++ {
        val := quick_test.string()
        vibez.spill(val)
    }
    
    fr Generate random arrays
    vibez.spill("\nRandom integer arrays (length 3-7, values 0-100):")
    vibecheck i := 0; i < 3; i++ {
        val := quick_test.int_array(3, 7, 0, 100)
        vibez.spill(val)
    }
    
    fr Generate random floats
    vibez.spill("\nRandom floats in range [-1.0, 1.0]:")
    vibecheck i := 0; i < 5; i++ {
        val := quick_test.float_range(-1.0, 1.0)
        vibez.spill(val)
    }
    
    fr Generate random hash maps
    vibez.spill("\nRandom hash maps (2-5 entries):")
    vibecheck i := 0; i < 2; i++ {
        val := quick_test.hash_map(2, 5)
        vibez.spill(val)
    }
}

slay test_properties() {
    vibez.spill("\nTesting property-based functions...")
    
    fr Test a simple property: integers divisible by 2 are even
    test_even_numbers()
    
    fr Test a property with string lengths
    test_string_lengths()
}

slay test_even_numbers() {
    vibez.spill("\nProperty test: Numbers divisible by 2 are even")
    
    fr Create test configuration
    config := quick_test.Config{
        max_count: 20,
        min_size: 1,
        max_size: 100,
        quiet: false,
    }
    
    fr Create a test property function
    is_even := func(n) {
        return n % 2 == 0
    }
    
    fr Run the test (generate random integers divisible by 2)
    generator := func() {
        n := quick_test.int_range(1, 50) * 2
        return n
    }
    
    result := quick_test.for_all(generator, is_even, config)
    vibez.spill("Test result: passed =", result.passed)
    vibez.spill("Iterations:", result.count)
}

slay test_string_lengths() {
    vibez.spill("\nProperty test: Strings generated with length constraints have correct length")
    
    fr Create test configuration
    config := quick_test.Config{
        max_count: 20,
        quiet: false,
    }
    
    fr Create a test property function
    has_correct_length := func(s) {
        return len(s) >= 5 && len(s) <= 10
    }
    
    fr Run the test (generate strings with specific length constraints)
    generator := func() {
        return quick_test.string_with_length(5, 10)
    }
    
    result := quick_test.for_all(generator, has_correct_length, config)
    vibez.spill("Test result: passed =", result.passed)
    vibez.spill("Iterations:", result.count)
}