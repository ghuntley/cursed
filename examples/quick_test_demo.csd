fr fr Comprehensive demonstration of CURSED Quick Test (property-based testing) module
fr fr This example showcases why property-based testing is critical for quality assurance

import "stdlib::packages::quick_test";
import "stdlib::vibez";

fr fr Example 1: Basic property testing - reverse twice property
fr fr This tests that reversing a slice twice returns the original slice
slay reverse(arr: []normie) -> []normie {
    sus result = []normie{};
    lowkey (sus i = len(arr) - 1; i >= 0; i--) {
        result = append(result, arr[i]);
    }
    yolo result;
}

slay test_reverse_twice_property() {
    vibez.spill("=== Testing Reverse Twice Property ===");
    
    fr fr Define the property
    reverseTwiceProperty := func(val: Value) -> lit {
        lowkey let arr = val.as_array() {
            sus original = arr.clone();
            sus reversed = reverse(arr);
            sus reversedTwice = reverse(reversed);
            yolo arrays_equal(original, reversedTwice);
        }
        yolo false;
    };
    
    fr fr Configure the test
    config := quick_test.Config{
        max_count: 100,
        max_size: 20,
        quiet: false,
    };
    
    fr fr Run the test with array generator
    result := quick_test.check_with_generator(
        reverseTwiceProperty,
        quick_test.slice_of(quick_test.int_range(-100, 100)),
        Some(config)
    );
    
    match result {
        Ok(res) => {
            vibez.spill("Reverse twice property test passed: %t", res.passed);
            vibez.spill("Number of test cases run: %d", res.count);
            vibez.spill("Runtime: %v", res.runtime);
        }
        Err(e) => {
            vibez.spill("Test error: %v", e);
        }
    }
}

fr fr Example 2: Mathematical property testing - absolute value
slay test_abs_positive_property() {
    vibez.spill("\n=== Testing Absolute Value Property ===");
    
    fr fr Property: abs(x) >= 0 for all integers x
    absPositiveProperty := func(val: Value) -> lit {
        lowkey let Value.Integer(x) = val {
            yolo abs(x) >= 0;
        }
        yolo false;
    };
    
    fr fr Test with integer generator
    config := quick_test.Config{
        max_count: 1000,
        max_size: 1000,
    };
    
    result := quick_test.check_with_generator(
        absPositiveProperty,
        quick_test.int64_range(i64.MIN, i64.MAX),
        Some(config)
    );
    
    match result {
        Ok(res) => {
            vibez.spill("Abs positive property test passed: %t", res.passed);
            vibez.spill("Number of test cases run: %d", res.count);
        }
        Err(e) => {
            vibez.spill("Test error: %v", e);
        }
    }
}

fr fr Example 3: String property testing
slay test_string_length_property() {
    vibez.spill("\n=== Testing String Length Property ===");
    
    fr fr Property: concatenating two strings should have length equal to sum of parts
    stringConcatProperty := func(val1: Value, val2: Value) -> lit {
        lowkey let (Value.String(s1), Value.String(s2)) = (val1, val2) {
            sus concatenated = s1 + s2;
            yolo len(concatenated) == len(s1) + len(s2);
        }
        yolo false;
    };
    
    fr fr Test with dual string generator
    stringGen := quick_test.string_of_n(0, 50, quick_test.alphanumeric());
    
    config := quick_test.Config{
        max_count: 500,
    };
    
    fr fr Create a generator that produces two strings
    dualStringGen := quick_test.GeneratorFunc(func(rng, size) {
        s1 := stringGen.generate(rng, size);
        s2 := stringGen.generate(rng, size);
        yolo Value.Array([s1, s2]);
    });
    
    dualStringProperty := func(val: Value) -> lit {
        lowkey let Value.Array(arr) = val {
            lowkey len(arr) == 2 {
                yolo stringConcatProperty(arr[0], arr[1]);
            }
        }
        yolo false;
    };
    
    result := quick_test.check_with_generator(
        dualStringProperty,
        Box.new(dualStringGen),
        Some(config)
    );
    
    match result {
        Ok(res) => {
            vibez.spill("String concat property test passed: %t", res.passed);
            vibez.spill("Number of test cases run: %d", res.count);
        }
        Err(e) => {
            vibez.spill("Test error: %v", e);
        }
    }
}

fr fr Example 4: Testing with shrinking - property that fails
slay test_failing_property_with_shrink() {
    vibez.spill("\n=== Testing Failing Property with Shrinking ===");
    
    fr fr Property that fails for negative numbers (intentionally failing)
    failingProperty := func(val: Value) -> lit {
        lowkey let Value.Integer(x) = val {
            yolo x >= 0; fr fr This will fail for negative numbers
        }
        yolo false;
    };
    
    fr fr Configure test with shrinking enabled
    config := quick_test.Config{
        max_count: 200,
        max_failures: 1,
        shrink_strategy: quick_test.ShrinkStrategy.SmartShrink,
        max_shrink_count: 50,
    };
    
    result := quick_test.check_with_generator(
        failingProperty,
        quick_test.int_range(-100, 100),
        Some(config)
    );
    
    match result {
        Ok(res) => {
            vibez.spill("Expected failing property test passed: %t", res.passed);
            
            lowkey !res.passed {
                vibez.spill("Failed after %d iterations", res.failed_after);
                vibez.spill("Original failing input: %v", res.input);
                vibez.spill("Shrunk failing input: %v", res.shrunk_input);
                vibez.spill("Shrink iterations: %d", res.shrink_count);
                vibez.spill("This demonstrates how shrinking finds minimal failing cases!");
            }
        }
        Err(e) => {
            vibez.spill("Test error: %v", e);
        }
    }
}

fr fr Example 5: Custom generators for complex types
be_like Person squad {
    name: tea,
    age: normie,
    hobbies: []tea,
}

slay test_custom_person_generator() {
    vibez.spill("\n=== Testing Custom Person Generator ===");
    
    fr fr Define a generator for Person structs
    personGen := quick_test.GeneratorFunc(func(rng, size) {
        fr fr Generate name
        nameLen := rng.gen_range(1..=size.min(20));
        sus nameChars = [];
        lowkey (sus i = 0; i < nameLen; i++) {
            nameChars.push(('A' as u8 + rng.gen_range(0..26)) as char);
        }
        name := nameChars.into_iter().collect::<String>();
        
        fr fr Generate age
        age := rng.gen_range(0..100);
        
        fr fr Generate hobbies
        hobbiesCount := rng.gen_range(0..=size.min(5));
        hobbyOptions := ["Reading", "Swimming", "Coding", "Gaming", "Cooking"];
        sus hobbies = [];
        lowkey (sus i = 0; i < hobbiesCount; i++) {
            hobbies.push(hobbyOptions[rng.gen_range(0..hobbyOptions.len())]);
        }
        
        fr fr Create person object
        sus personObj = HashMap.new();
        personObj.insert("name".to_string(), Value.String(name));
        personObj.insert("age".to_string(), Value.Integer(age as i64));
        personObj.insert("hobbies".to_string(), Value.Array(
            hobbies.into_iter().map(|h| Value.String(h.to_string())).collect()
        ));
        
        yolo Value.Object(personObj);
    });
    
    fr fr Property: Person should have valid age and non-empty name
    personProperty := func(val: Value) -> lit {
        lowkey let Value.Object(obj) = val {
            lowkey let (Some(Value.Integer(age)), Some(Value.String(name))) = 
                (obj.get("age"), obj.get("name")) {
                yolo age >= &0 && age < &100 && !name.is_empty();
            }
        }
        yolo false;
    };
    
    config := quick_test.Config{
        max_count: 50,
        max_size: 10,
    };
    
    result := quick_test.check_with_generator(
        personProperty,
        Box.new(personGen),
        Some(config)
    );
    
    match result {
        Ok(res) => {
            vibez.spill("Person property test passed: %t", res.passed);
            vibez.spill("Number of test cases run: %d", res.count);
        }
        Err(e) => {
            vibez.spill("Test error: %v", e);
        }
    }
}

fr fr Example 6: Stateful testing with counter
be_like Counter squad {
    value: normie,
}

impl Counter {
    slay new() -> Self {
        Self { value: 0 }
    }
    
    slay increment(&sus self) {
        self.value += 1;
    }
    
    slay reset(&sus self) {
        self.value = 0;
    }
    
    slay get_value(&self) -> normie {
        self.value
    }
}

slay test_stateful_counter() {
    vibez.spill("\n=== Testing Stateful Counter ===");
    
    fr fr Create a state machine model
    sus model = quick_test.new_state_machine(|| Counter.new());
    
    fr fr Add increment action
    model.add_action(
        "increment",
        |counter: &sus Counter| counter.increment(),
        |_counter: &Counter| based, fr fr Can always increment
        |counter: &Counter, prev_value: normie| counter.get_value() == prev_value + 1,
    );
    
    fr fr Add reset action
    model.add_action(
        "reset",
        |counter: &sus Counter| counter.reset(),
        |_counter: &Counter| based, fr fr Can always reset
        |counter: &Counter, _prev_value: normie| counter.get_value() == 0,
    );
    
    fr fr Run the state machine
    config := quick_test.Config{
        max_count: 100,
    };
    
    result := model.run(Some(config));
    
    match result {
        Ok(res) => {
            vibez.spill("State machine test passed: %t", res.passed);
            vibez.spill("Number of action sequences run: %d", res.count);
            
            lowkey !res.passed {
                lowkey let Some(Value.Array(sequence)) = res.shrunk_input {
                    vibez.spill("Failing action sequence:");
                    lowkey action in sequence {
                        lowkey let Value.String(action_name) = action {
                            vibez.spill("  - %s", action_name);
                        }
                    }
                }
            }
        }
        Err(e) => {
            vibez.spill("Test error: %v", e);
        }
    }
}

fr fr Example 7: Weighted generators
slay test_weighted_generators() {
    vibez.spill("\n=== Testing Weighted Generators ===");
    
    fr fr Create a weighted generator that favors small numbers
    weightedGen := quick_test.weighted(vec![
        (10, quick_test.int_range(0, 10)),    fr fr 10x more likely
        (1, quick_test.int_range(11, 100)),   fr fr 1x likely  
    ]);
    
    fr fr Property to test distribution (most numbers should be small)
    distributionProperty := func(val: Value) -> lit {
        fr fr This is just a demonstration - in practice you'd collect
        fr fr statistics and verify the distribution
        yolo based; fr fr Always pass for demo
    };
    
    config := quick_test.Config{
        max_count: 100,
    };
    
    result := quick_test.check_with_generator(
        distributionProperty,
        weightedGen,
        Some(config)
    );
    
    match result {
        Ok(res) => {
            vibez.spill("Weighted generator test passed: %t", res.passed);
            vibez.spill("Number of test cases run: %d", res.count);
        }
        Err(e) => {
            vibez.spill("Test error: %v", e);
        }
    }
}

fr fr Example 8: Test reproducibility with replay
slay test_reproducibility() {
    vibez.spill("\n=== Testing Reproducibility and Replay ===");
    
    fr fr Property that sometimes fails randomly
    randomFailProperty := func(val: Value) -> lit {
        lowkey let Value.Integer(x) = val {
            fr fr Fail if number is divisible by 13 (somewhat arbitrary)
            yolo x % 13 != 0;
        }
        yolo false;
    };
    
    fr fr First run with fixed seed
    config := quick_test.Config{
        max_count: 200,
        seed: Some(12345),
        max_failures: 1,
    };
    
    result1 := quick_test.check_with_generator(
        randomFailProperty,
        quick_test.int_range(1, 100),
        Some(config.clone())
    );
    
    fr fr Second run with same seed should give same result
    result2 := quick_test.check_with_generator(
        randomFailProperty,
        quick_test.int_range(1, 100),
        Some(config)
    );
    
    match (result1, result2) {
        (Ok(res1), Ok(res2)) => {
            vibez.spill("First run passed: %t, failed after: %d", res1.passed, res1.failed_after);
            vibez.spill("Second run passed: %t, failed after: %d", res2.passed, res2.failed_after);
            vibez.spill("Reproducible: %t", res1.passed == res2.passed && res1.failed_after == res2.failed_after);
            
            lowkey !res1.passed {
                vibez.spill("Both runs failed with same input: %v", res1.input);
                
                fr fr Create replay config for this failure
                lowkey let Some(input) = res1.input {
                    replay := quick_test.replay_config(res1.seed, input);
                    vibez.spill("Replay config created with seed: %d", replay.seed);
                }
            }
        }
        (Err(e1), _) => vibez.spill("First test error: %v", e1),
        (_, Err(e2)) => vibez.spill("Second test error: %v", e2),
    }
}

fr fr Helper functions
slay arrays_equal(a: []normie, b: []normie) -> lit {
    lowkey len(a) != len(b) {
        yolo false;
    }
    
    lowkey (sus i = 0; i < len(a); i++) {
        lowkey a[i] != b[i] {
            yolo false;
        }
    }
    
    yolo based;
}

slay abs(x: normie) -> normie {
    lowkey x < 0 {
        yolo -x;
    }
    yolo x;
}

fr fr Main function to run all examples
slay main() {
    vibez.spill("CURSED Quick Test (Property-Based Testing) Demonstration");
    vibez.spill("========================================================");
    vibez.spill("");
    vibez.spill("Property-based testing is critical for quality assurance because:");
    vibez.spill("1. Discovers edge cases that manual testing misses");
    vibez.spill("2. Tests mathematical properties that should always hold");
    vibez.spill("3. Provides extensive coverage with minimal test code");
    vibez.spill("4. Shrinks failing cases to minimal examples");
    vibez.spill("5. Enables reproducible testing for debugging");
    vibez.spill("");
    
    test_reverse_twice_property();
    test_abs_positive_property();
    test_string_length_property();
    test_failing_property_with_shrink();
    test_custom_person_generator();
    test_stateful_counter();
    test_weighted_generators();
    test_reproducibility();
    
    vibez.spill("\n========================================================");
    vibez.spill("All property-based testing examples completed!");
    vibez.spill("This demonstrates the power of Quick Test for finding bugs");
    vibez.spill("and ensuring code correctness across wide input spaces.");
}
