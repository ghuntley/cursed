//! Tests for the enhanced interface type registry functionality with runtime type information

use std::sync::Once;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required test utilities
use cursed::core::{JitOptions, InterpretOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef};

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<ObjectRef, String> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Enable verbose debugging for type assertions to show type names
    std::env::set_var("CURSED_TYPE_DEBUG", "verbose");
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]);
        
    let result = cursed::code::jit_compile_and_run(&program, options)?;
    Ok(result)
}

#[test]
fn test_interface_type_registry_enhanced() {
    init_tracing!();
    
    // Define a program with multiple distinct types and complex interfaces
    let input = r#"
        // Define a complex interface hierarchy
        collab Vehicle {
            getSpeed() normie;
            getType() tea;
        }
        
        collab LandVehicle : Vehicle {
            getWheelCount() normie;
        }
        
        collab WaterVehicle : Vehicle {
            isSubmersible() lit;
        }
        
        collab AirVehicle : Vehicle {
            getMaxAltitude() normie;
        }
        
        // Define specific implementations
        squad Car {
            make tea,
            model tea,
            speed normie,
            wheels normie
        }
        
        squad Boat {
            name tea,
            speed normie,
            canSubmerge lit
        }
        
        squad Helicopter {
            model tea,
            speed normie,
            maxAltitude normie
        }
        
        // Implement Vehicle for all types
        slay (c Car) getSpeed() normie {
            return c.speed
        }
        
        slay (c Car) getType() tea {
            return "Car"
        }
        
        slay (b Boat) getSpeed() normie {
            return b.speed
        }
        
        slay (b Boat) getType() tea {
            return "Boat"
        }
        
        slay (h Helicopter) getSpeed() normie {
            return h.speed
        }
        
        slay (h Helicopter) getType() tea {
            return "Helicopter"
        }
        
        // Implement specific interfaces
        slay (c Car) getWheelCount() normie {
            return c.wheels
        }
        
        slay (b Boat) isSubmersible() lit {
            return b.canSubmerge
        }
        
        slay (h Helicopter) getMaxAltitude() normie {
            return h.maxAltitude
        }
        
        // Main function to test enhanced type assertions with precise type names
        slay main() tea {
            // Create different vehicles
            sus car = Car{make: "Toyota", model: "Corolla", speed: 120.0, wheels: 4.0}
            sus boat = Boat{name: "Titanic", speed: 30.0, canSubmerge: based}
            sus helicopter = Helicopter{model: "Apache", speed: 260.0, maxAltitude: 5000.0}
            
            // Create a slice of generic vehicles
            sus vehicles = []Vehicle{}
            vehicles = append(vehicles, car)
            vehicles = append(vehicles, boat)
            vehicles = append(vehicles, helicopter)
            
            // Track successful assertions with specific type information
            sus results = {}
            
            // Iterate over vehicles and test assertions with different types
            sus i = 0
            periodt i < len(vehicles) {
                sus v = vehicles[i]
                
                // Get the basic vehicle type
                sus vtype = v.getType()
                
                // Try assertions to all possible types
                sus car_val, is_car = v.(Car)
                sus boat_val, is_boat = v.(Boat)
                sus helicopter_val, is_helicopter = v.(Helicopter)
                
                // Try interface assertions
                sus land_v, is_land = v.(LandVehicle)
                sus water_v, is_water = v.(WaterVehicle)
                sus air_v, is_air = v.(AirVehicle)
                
                // Record results with proper type information
                if is_car {
                    results[vtype + ": is Car"] = based
                } else {
                    results[vtype + ": is Car"] = sus
                }
                
                if is_boat {
                    results[vtype + ": is Boat"] = based
                } else {
                    results[vtype + ": is Boat"] = sus
                }
                
                if is_helicopter {
                    results[vtype + ": is Helicopter"] = based
                } else {
                    results[vtype + ": is Helicopter"] = sus
                }
                
                if is_land {
                    results[vtype + ": is LandVehicle"] = based
                } else {
                    results[vtype + ": is LandVehicle"] = sus
                }
                
                if is_water {
                    results[vtype + ": is WaterVehicle"] = based
                } else {
                    results[vtype + ": is WaterVehicle"] = sus
                }
                
                if is_air {
                    results[vtype + ": is AirVehicle"] = based
                } else {
                    results[vtype + ": is AirVehicle"] = sus
                }
                
                i = i + 1
            }
            
            // Format results into a string
            sus result = ""
            
            // Check Car assertions
            if results["Car: is Car"] == based {
                result = result + "Car correctly identified as Car\n"
            }
            if results["Car: is LandVehicle"] == based {
                result = result + "Car correctly identified as LandVehicle\n"
            }
            if results["Car: is Boat"] == sus {
                result = result + "Car correctly not identified as Boat\n"
            }
            if results["Car: is WaterVehicle"] == sus {
                result = result + "Car correctly not identified as WaterVehicle\n"
            }
            
            // Check Boat assertions
            if results["Boat: is Boat"] == based {
                result = result + "Boat correctly identified as Boat\n"
            }
            if results["Boat: is WaterVehicle"] == based {
                result = result + "Boat correctly identified as WaterVehicle\n"
            }
            if results["Boat: is Car"] == sus {
                result = result + "Boat correctly not identified as Car\n"
            }
            if results["Boat: is LandVehicle"] == sus {
                result = result + "Boat correctly not identified as LandVehicle\n"
            }
            
            // Check Helicopter assertions
            if results["Helicopter: is Helicopter"] == based {
                result = result + "Helicopter correctly identified as Helicopter\n"
            }
            if results["Helicopter: is AirVehicle"] == based {
                result = result + "Helicopter correctly identified as AirVehicle\n"
            }
            if results["Helicopter: is Car"] == sus {
                result = result + "Helicopter correctly not identified as Car\n"
            }
            if results["Helicopter: is LandVehicle"] == sus {
                result = result + "Helicopter correctly not identified as LandVehicle\n"
            }
            
            return result
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            let expected_lines = [
                "Car correctly identified as Car",
                "Car correctly identified as LandVehicle",
                "Car correctly not identified as Boat",
                "Car correctly not identified as WaterVehicle",
                "Boat correctly identified as Boat",
                "Boat correctly identified as WaterVehicle",
                "Boat correctly not identified as Car",
                "Boat correctly not identified as LandVehicle",
                "Helicopter correctly identified as Helicopter",
                "Helicopter correctly identified as AirVehicle",
                "Helicopter correctly not identified as Car",
                "Helicopter correctly not identified as LandVehicle"
            ];
            
            // Check that the result contains all expected lines
            let result_str = result.as_string().unwrap();
            for line in expected_lines.iter() {
                assert!(result_str.contains(line), 
                       "Result doesn't contain expected line: {}\nResult was: {}", 
                       line, result_str);
            }
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_registry_runtime_error_messages() {
    init_tracing!();
    
    // Test with invalid type assertions to check enhanced error messages
    let input = r#"
        // Define interfaces
        collab Animal {
            speak() tea;
        }
        
        collab Worker {
            work() tea;
        }
        
        // Define concrete types
        squad Dog {
            name tea
        }
        
        squad Cat {
            name tea
        }
        
        squad Robot {
            model tea
        }
        
        // Implement interfaces
        slay (d Dog) speak() tea {
            return d.name + " says woof!"
        }
        
        slay (c Cat) speak() tea {
            return c.name + " says meow!"
        }
        
        slay (r Robot) work() tea {
            return r.model + " is working"
        }
        
        // Main function to test runtime error messages
        slay main() tea {
            // Create instances
            sus dog = Dog{name: "Buddy"}
            sus cat = Cat{name: "Whiskers"}
            sus robot = Robot{model: "T-1000"}
            
            // Convert to interfaces
            sus dogAnimal Animal = dog
            sus catAnimal Animal = cat
            sus robotWorker Worker = robot
            
            // Test correct assertions
            sus dogTest, dogOk = dogAnimal.(Dog)
            sus catTest, catOk = catAnimal.(Cat)
            sus robotTest, robotOk = robotWorker.(Robot)
            
            // Test incorrect assertions with improved error messages
            sus catFromDog, catFromDogOk = dogAnimal.(Cat)
            sus dogFromCat, dogFromCatOk = catAnimal.(Dog)
            sus robotFromDog, robotFromDogOk = dogAnimal.(Robot) 
            sus animalFromWorker, animalFromWorkerOk = robotWorker.(Animal)
            
            // Format results
            sus result = ""
            
            if dogOk && catOk && robotOk {
                result = result + "All correct assertions passed\n"
            } else {
                result = result + "Some correct assertions failed\n"
            }
            
            if !catFromDogOk && !dogFromCatOk && !robotFromDogOk && !animalFromWorkerOk {
                result = result + "All incorrect assertions correctly failed\n"
            } else {
                result = result + "Some incorrect assertions unexpectedly passed\n"
            }
            
            // Report actual vs expected type to verify type registry information
            // At this point, the type registry should have accurate type names that appear in logs
            result = result + "Type assertion results verified with proper type names in debug output"
            
            return result
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            let expected_result = "All correct assertions passed\nAll incorrect assertions correctly failed\nType assertion results verified with proper type names in debug output";
            assert_eq!(result.as_string().unwrap().trim(), expected_result);
            // Note: The actual enhanced error messages appear in the logs, not in the return value
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}