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
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::Object;

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<i32, String> {
    // Create a lexer and parser
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).map_err(|e| e.to_string())?;
    let program = parser.parse_program().map_err(|e| e.to_string())?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Create LLVM context and code generator
    let context = inkwell::context::Context::create();
    let file_path = std::path::PathBuf::from("test_program.csd");
    let mut code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(&context, "main", file_path.clone());
    
    // Compile the program
    code_gen.compile(&program).map_err(|e| e.to_string())?;
    
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
        .map_err(|e| e.to_string())?;
    
    // Initialize the goroutine manager
    cursed::codegen::jit::init_goroutine_manager();
    
    // Create JIT compiler
    let mut jit_compiler = cursed::codegen::jit::JitCompiler::new(
        &context, 
        execution_engine, 
        "_main_main", 
        file_path
    );
    
    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen);
    
    // Execute the program
    let result = jit_compiler.execute().map_err(|e| e.to_string())?;
    
    // Wait for any goroutines to complete (10ms timeout)
    let _remaining = cursed::codegen::jit::wait_for_goroutines(10);
    
    Ok(result)
}

#[test]
fn test_interface_type_assertion_with_runtime_errors() {
    init_tracing!();
    
    // This test verifies that runtime errors in type assertions are properly handled
    let input = r#"
        // Define an interface
        collab Resource {
            getId() tea;
            getName() tea;
        }
        
        // Define two struct types implementing the interface
        squad FileResource {
            path tea,
            id tea
        }
        
        squad NetworkResource {
            url tea,
            id tea
        }
        
        // Implement Resource for FileResource
        slay (fr FileResource) getId() tea {
            return fr.id
        }
        
        slay (fr FileResource) getName() tea {
            return "File: " + fr.path
        }
        
        // Implement Resource for NetworkResource
        slay (nr NetworkResource) getId() tea {
            return nr.id
        }
        
        slay (nr NetworkResource) getName() tea {
            return "Network: " + nr.url
        }
        
        // Function that tries to cast the resource and could cause runtime errors
        slay processFileResource(r Resource) tea {
            // Type assertion with no direct error checking
            sus file = r.(FileResource)
            
            // Accessing a field that should be verified for safety
            return "Processing file at: " + file.path
        }
        
        // Safer function with error checking
        slay safeProcessFileResource(r Resource) tea {
            // Type assertion with error checking
            sus file, ok = r.(FileResource)
            
            if !ok {
                return "Error: Not a file resource"
            }
            
            // Safely accessing the field after verification
            return "Processing file at: " + file.path
        }
        
        // Main function
        slay main() tea {
            // Create resources
            sus fileRes = FileResource{path: "/data/file.txt", id: "file-1"}
            sus netRes = NetworkResource{url: "https://example.com/api", id: "net-1"}
            
            // Process resources safely
            sus result1 = safeProcessFileResource(fileRes)
            sus result2 = safeProcessFileResource(netRes)
            
            // Try to process with unsafe function (should handle error gracefully)
            sus result3 = "";
            try {
                result3 = processFileResource(netRes)
            } catch {
                result3 = "Caught error in processFileResource"
            }
            
            return result1 + " | " + result2 + " | " + result3
        }
    "#;
    
    // Run the test
    match run_jit_test(input) {
        Ok(result) => {
            // Check that exit code is 0 indicating success
            assert_eq!(result, 0, "Expected program to exit with code 0, got {}", result);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_nested_interface_values_with_assertions() {
    init_tracing!();
    
    // Test handling of nested interface values with assertions
    let input = r#"
        // Define interfaces
        collab Container {
            getValue() any;
        }
        
        collab Stringable {
            toString() tea;
        }
        
        // Define concrete types
        squad Box {
            value any
        }
        
        squad StringValue {
            text tea
        }
        
        squad NumberValue {
            num lit
        }
        
        // Implementations
        slay (b Box) getValue() any {
            return b.value
        }
        
        slay (sv StringValue) toString() tea {
            return "String: " + sv.text
        }
        
        slay (nv NumberValue) toString() tea {
            return "Number: " + vibe.toString(nv.num)
        }
        
        // Function to safely extract nested values
        slay extractNestedValue(c Container) tea {
            // Get the value from the container
            sus value = c.getValue()
            
            // Try to assert the container as a Box
            sus box, isBox = c.(Box)
            if !isBox {
                return "Not a box container"
            }
            
            // Try to assert the container's value as Stringable
            sus stringable, isStringable = value.(Stringable)
            if !isStringable {
                return "Box does not contain a stringable value"
            }
            
            // Now try to determine the specific type
            sus strValue, isStrValue = stringable.(StringValue)
            if isStrValue {
                return "Box contains string: " + strValue.text
            }
            
            sus numValue, isNumValue = stringable.(NumberValue)
            if isNumValue {
                return "Box contains number: " + vibe.toString(numValue.num)
            }
            
            // Unknown but stringable type
            return "Box contains unknown stringable: " + stringable.toString()
        }
        
        // Main function
        slay main() tea {
            // Create string box
            sus strValue = StringValue{text: "Hello, world!"}
            sus strBox = Box{value: strValue}
            
            // Create number box
            sus numValue = NumberValue{num: 42}
            sus numBox = Box{value: numValue}
            
            // Create invalid box (non-stringable)
            sus nonStringable = Box{value: "just a plain string"}
            
            // Extract values
            sus result1 = extractNestedValue(strBox)
            sus result2 = extractNestedValue(numBox)
            sus result3 = extractNestedValue(nonStringable)
            
            return result1 + " | " + result2 + " | " + result3
        }
    "#;
    
    // Run the test
    match run_jit_test(input) {
        Ok(result) => {
            // Check that exit code is 0 indicating success
            assert_eq!(result, 0, "Expected program to exit with code 0, got {}", result);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_with_inheritance_pattern() {
    init_tracing!();
    
    // Test with an inheritance-like pattern using interfaces
    let input = r#"
        // Define interface hierarchy
        collab Entity {
            getId() tea;
        }
        
        collab LivingEntity {
            getId() tea;
            getHealth() lit;
        }
        
        collab Character {
            getId() tea;
            getHealth() lit;
            getName() tea;
        }
        
        // Concrete implementation
        squad Player {
            id tea,
            health lit,
            name tea,
            level lit
        }
        
        // Implement all interfaces for Player
        slay (p Player) getId() tea {
            return p.id
        }
        
        slay (p Player) getHealth() lit {
            return p.health
        }
        
        slay (p Player) getName() tea {
            return p.name
        }
        
        // Function to check type in hierarchy
        slay processEntity(entity any) tea {
            sus result = ""
            
            // Try to check if it's an Entity
            sus basicEntity, isEntity = entity.(Entity)
            if isEntity {
                result = result + "Entity with ID: " + basicEntity.getId() + "\n"
            } else {
                return "Not an entity"
            }
            
            // Try to check if it's a LivingEntity
            sus livingEntity, isLiving = entity.(LivingEntity)
            if isLiving {
                result = result + "Living entity with health: " + vibe.toString(livingEntity.getHealth()) + "\n"
            }
            
            // Try to check if it's a Character
            sus character, isCharacter = entity.(Character)
            if isCharacter {
                result = result + "Character named: " + character.getName() + "\n"
            }
            
            // Try to check if it's specifically a Player
            sus player, isPlayer = entity.(Player)
            if isPlayer {
                result = result + "Player at level: " + vibe.toString(player.level)
            }
            
            return result
        }
        
        // Main function
        slay main() tea {
            // Create a player
            sus player = Player{id: "player-1", health: 100, name: "Adventurer", level: 5}
            
            // Process as various entity types
            sus result = processEntity(player)
            
            return result
        }
    "#;
    
    // Run the test
    match run_jit_test(input) {
        Ok(result) => {
            // Check that exit code is 0 indicating success
            assert_eq!(result, 0, "Expected program to exit with code 0, got {}", result);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_type_assertion_error_recovery() {
    init_tracing!();
    
    // Test error recovery with type assertions
    let input = r#"
        // Define an interface
        collab Processor {
            process(data tea) tea;
        }
        
        // Define concrete processors
        squad TextProcessor {
            prefix tea
        }
        
        squad JsonProcessor {
            indent lit
        }
        
        // Implement the interface
        slay (tp TextProcessor) process(data tea) tea {
            return tp.prefix + ": " + data
        }
        
        slay (jp JsonProcessor) process(data tea) tea {
            // Simple JSON formatting simulation
            sus indentStr = ""
            periodt i := 0; i < jp.indent; i++ {
                indentStr = indentStr + " "
            }
            return "{ \n" + indentStr + "\"data\": \"" + data + "\"\n}"
        }
        
        // Function that attempts conversions with recovery
        slay tryProcess(processor any, data tea) tea {
            try {
                // First try as TextProcessor
                sus textProc = processor.(TextProcessor)
                return textProc.process(data)
            } catch {
                // Failed, record the error
                sus errorMsg = "Failed text processor assertion"
                
                try {
                    // Try as JsonProcessor
                    sus jsonProc = processor.(JsonProcessor)
                    return jsonProc.process(data)
                } catch {
                    // Both failed, return combined error
                    return errorMsg + " and json processor assertion"
                }
            }
        }
        
        // Another approach with explicit checks
        slay safeProcess(processor any, data tea) tea {
            // Try as TextProcessor
            sus textProc, isText = processor.(TextProcessor)
            if isText {
                return textProc.process(data)
            }
            
            // Try as JsonProcessor
            sus jsonProc, isJson = processor.(JsonProcessor)
            if isJson {
                return jsonProc.process(data)
            }
            
            // No compatible processor found
            return "No compatible processor found"
        }
        
        // Main function
        slay main() tea {
            // Create processors
            sus textProc = TextProcessor{prefix: "TEXT"}
            sus jsonProc = JsonProcessor{indent: 2}
            sus invalidProc = "not a processor"
            
            // Test different approaches
            sus result1 = tryProcess(textProc, "Hello, world!")
            sus result2 = tryProcess(jsonProc, "Hello, world!")
            sus result3 = ""
            
            try {
                result3 = tryProcess(invalidProc, "Hello, world!")
            } catch {
                result3 = "Caught completely invalid processor"
            }
            
            sus result4 = safeProcess(textProc, "Safe processing")
            sus result5 = safeProcess(invalidProc, "Safe processing")
            
            return "Try approach: " + result1 + " | " + result2 + " | " + result3 + 
                   "\nSafe approach: " + result4 + " | " + result5
        }
    "#;
    
    // Run the test
    match run_jit_test(input) {
        Ok(result) => {
            // Check that exit code is 0 indicating success
            assert_eq!(result, 0, "Expected program to exit with code 0, got {}", result);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}