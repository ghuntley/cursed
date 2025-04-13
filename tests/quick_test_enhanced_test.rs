use std::rc::Rc;
use cursed::object::Object;
use cursed::prelude::*;
use cursed::ast::expr;
use cursed::parser::Parser;
use cursed::core::interpreter::Interpreter;

#[test]
fn test_generators() {
    let code = r#"
    vibe main

    slay main() {
        fr Test enhanced generators
        test_combined_generators()
        test_one_of_generators()
        test_specific_type_generators()
    }

    slay test_combined_generators() {
        fr Test combined generators
        
        fr Combine int_range and boolean generators
        gen_func := func(size) {
            num := quick_test.int_range(-size, size)
            flag := quick_test.boolean()
            return [num, flag]
        }
        
        vibecheck i := 0; i < 5; i++ {
            value := quick_test.Generate(null, 10, gen_func)
            vibez.spill("Combined value:", value)
            
            fr Verify value is an array with 2 elements
            fr First element should be an integer, second a boolean
            if len(value) != 2 {
                vibez.spill("Error: Expected array of length 2, got", len(value))
                return
            }
            
            if typeof(value[0]) != "integer" {
                vibez.spill("Error: Expected integer, got", typeof(value[0]))
                return
            }
            
            if typeof(value[1]) != "boolean" {
                vibez.spill("Error: Expected boolean, got", typeof(value[1]))
                return
            }
        }
        
        vibez.spill("Combined generators test passed")
    }
    
    slay test_one_of_generators() {
        fr Test OneOf generator
        values := ["one", "two", "three", "four", "five"]
        one_of_gen := quick_test.OneOf(values)
        
        vibecheck i := 0; i < 10; i++ {
            value := quick_test.Generate(null, 0, one_of_gen)
            vibez.spill("OneOf value:", value)
            
            fr Verify value is one of the provided options
            found := false
            vibecheck j := 0; j < len(values); j++ {
                if value == values[j] {
                    found = true
                    break
                }
            }
            
            if !found {
                vibez.spill("Error: Generated value not in provided options")
                return
            }
        }
        
        vibez.spill("OneOf generator test passed")
    }
    
    slay test_specific_type_generators() {
        fr Test the specific type generators
        
        fr Test Int8 generator
        vibez.spill("\nTesting Int8 generator:")
        int8_gen := quick_test.Int8()
        vibecheck i := 0; i < 5; i++ {
            value := quick_test.Generate(null, 10, int8_gen)
            vibez.spill(value)
            
            fr Verify value is an integer in the Int8 range
            if typeof(value) != "integer" {
                vibez.spill("Error: Expected integer, got", typeof(value))
                return
            }
            
            if value < -128 || value > 127 {
                vibez.spill("Error: Value outside Int8 range:", value)
                return
            }
        }
        
        fr Test Int16 generator
        vibez.spill("\nTesting Int16 generator:")
        int16_gen := quick_test.Int16()
        vibecheck i := 0; i < 5; i++ {
            value := quick_test.Generate(null, 10, int16_gen)
            vibez.spill(value)
            
            fr Verify value is an integer in the Int16 range
            if typeof(value) != "integer" {
                vibez.spill("Error: Expected integer, got", typeof(value))
                return
            }
            
            if value < -32768 || value > 32767 {
                vibez.spill("Error: Value outside Int16 range:", value)
                return
            }
        }
        
        vibez.spill("Specific type generators test passed")
    }
    "#;

    let mut parser = Parser::new(code);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new(program);
    interpreter.run().unwrap();
}

#[test]
fn test_check_property() {
    let code = r#"
    vibe main

    slay main() {
        fr Test CheckProperty function
        test_simple_property()
        test_complex_property()
    }

    slay test_simple_property() {
        vibez.spill("\nTesting CheckProperty with simple property:")
        
        fr Define a property: all numbers divisible by 2 are even
        property := func(n) {
            return n % 2 == 0
        }
        
        fr Generate test values: all even numbers
        generator := func(rand, size) {
            return (rand.int_range(0, size) * 2)
        }
        
        fr Run the property test
        config := quick_test.Config{
            max_count: 50,
            quiet: false,
        }
        
        result := quick_test.CheckProperty(property, generator, config)
        vibez.spill("Simple property test result: passed =", result.passed)
        vibez.spill("Iterations:", result.count)
        
        if !result.passed {
            vibez.spill("Test failed after", result.failed_after, "iterations")
            vibez.spill("Failing input:", result.failed_value)
        }
    }
    
    slay test_complex_property() {
        vibez.spill("\nTesting CheckProperty with complex property:")
        
        fr Define a property: the length of a string is always >= 0
        property := func(s) {
            return len(s) >= 0
        }
        
        fr Generate test values: random strings
        generator := func(rand, size) {
            length := rand.int_range(0, size)
            chars := []
            vibecheck i := 0; i < length; i++ {
                chars = append(chars, char(rand.int_range(65, 90)))  fr ASCII A-Z
            }
            return join("", chars)
        }
        
        fr Run the property test
        config := quick_test.Config{
            max_count: 20,
            quiet: false,
        }
        
        result := quick_test.CheckProperty(property, generator, config)
        vibez.spill("Complex property test result: passed =", result.passed)
        vibez.spill("Iterations:", result.count)
    }
    "#;

    let mut parser = Parser::new(code);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new(program);
    interpreter.run().unwrap();
}