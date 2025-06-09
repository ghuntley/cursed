use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::Object;
use cursed::Error;
use cursed::ast::traits::Node;

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

// Helper function to parse and validate CURSED code
fn parse_test(input: &str) -> Result<cursed::ast::base::Program, String> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).map_err(|e| format!("Parser creation error: {}", e))?;
    let program = parser.parse_program().map_err(|e| format!("Parse error: {}", e))?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msgs: Vec<String> = parser.errors().iter().map(|e| e.to_string()).collect();
        let error_msg = error_msgs.join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    Ok(program)
}

#[test]
fn test_interface_type_assertion_nested_path_tracking() {
    init_tracing!();
    
    // Define a program with a complex nested interface hierarchy
    let input = r#""
        // Define a complex interface hierarchy
        collab BasicObject {
            id() lit;
            getType() tea;
        }
        
        collab GameObject : BasicObject {
            position() normie;
            getSize() normie;
        }
        
        collab AnimatedObject : GameObject {
            frameCount() lit;
            currentFrame() lit;
        }
        
        collab Drawable : BasicObject {
            draw() tea;
        }
        
        collab AnimatedDrawable : Drawable, AnimatedObject {
            drawFrame(frame lit) tea;
        }
        
        // Implement a class that satisfies multiple interfaces
        squad Character {
            id lit,
            typeStr tea,
            x normie,
            y normie,
            size normie,
            frames lit,
            current_frame lit,
            name tea
        }
        
        // Implement BasicObject interface
        slay (c Character) id() lit {
            return c.id
        }
        
        slay (c Character) getType() tea {
            return c.typeStr
        }
        
        // Implement GameObject interface
        slay (c Character) position() normie {
            return c.x
        }
        
        slay (c Character) getSize() normie {
            return c.size
        }
        
        // Implement AnimatedObject interface
        slay (c Character) frameCount() lit {
            return c.frames
        }
        
        slay (c Character) currentFrame() lit {
            return c.current_frame
        }
        
        // Implement Drawable interface
        slay (c Character) draw() tea {
            return "Drawing " + c.name
        }
        
        // Implement AnimatedDrawable interface
        slay (c Character) drawFrame(frame lit) tea {
            return "Drawing " + c.name + " at frame " + vibe.toString(frame)
        }
        
        // Main function to test nested interface assertions
        slay main() tea {
            // Create a Character
            sus char = Character{
                id: 123,
                typeStr: "Player",
                x: [42.0],
                y: [24.0],
                size: 100.0,
                frames: 8,
                current_frame: 0,
                name: "Hero"
            }
            
            // Assign to the most specific interface
            sus animated AnimatedDrawable = char
            
            // Try assertions at different interface levels with multiple paths
            sus basic, isBasic = animated.(BasicObject)
            sus gameObj, isGameObj = animated.(GameObject)
            sus animObj, isAnimObj = animated.(AnimatedObject)
            sus drawable, isDrawable = animated.(Drawable)
            sus original, isOriginal = animated.(Character)
            
            // Collect test results
            sus result tea = ""
            
            if isBasic {
                result = result + "BasicObject: " + vibe.toString(basic.id()) + "\n"
            }
            
            if isGameObj {
                result = result + "GameObject: " + vibe.toString(gameObj.getSize()) + "\n"
            }
            
            if isAnimObj {
                result = result + "AnimatedObject: " + vibe.toString(animObj.frameCount()) + "\n"
            }
            
            if isDrawable {
                result = result + "Drawable: " + drawable.draw() + "\n"
            }
            
            if isOriginal {
                result = result + "Character: " + original.name
            }
            
            return result
        }
    "#";
    
    // Parse the test and verify it compiles correctly
    match parse_test(input) {
        Ok(program) => {
            // Check that the program parsed successfully
            println!("✅ Program parsed successfully");
            println!("Program structure:\n{}", program.string());
            
            // For now, we just verify parsing - full execution would require JIT setup
            assert!(program.statements.len() > 0, "Program should have statements");
        },
        Err(e) => panic!("Failed to parse test program: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_diamond_inheritance() {
    init_tracing!();
    
    // Define a program with diamond inheritance pattern
    let input = r#""
        // Define a diamond inheritance pattern
        collab Base {
            baseMethod() tea;
        }
        
        collab Left : Base {
            leftMethod() tea;
        }
        
        collab Right : Base {
            rightMethod() tea;
        }
        
        collab Bottom : Left, Right {
            bottomMethod() tea;
        }
        
        // Implement a class that satisfies the Bottom interface
        squad DiamondImpl {
            name tea
        }
        
        // Implement all required methods
        slay (d DiamondImpl) baseMethod() tea {
            return "Base: " + d.name
        }
        
        slay (d DiamondImpl) leftMethod() tea {
            return "Left: " + d.name
        }
        
        slay (d DiamondImpl) rightMethod() tea {
            return "Right: " + d.name
        }
        
        slay (d DiamondImpl) bottomMethod() tea {
            return "Bottom: " + d.name
        }
        
        // Main function to test diamond inheritance assertions
        slay main() tea {
            // Create an implementation
            sus impl = DiamondImpl{name: "Diamond"}
            
            // Assign to Bottom interface
            sus bottom Bottom = impl
            
            // Assert to each level in different paths
            sus b, isBase = bottom.(Base)
            sus l, isLeft = bottom.(Left)
            sus r, isRight = bottom.(Right)
            sus original, isOriginal = bottom.(DiamondImpl)
            
            // Collect test results
            sus result tea = ""
            
            if isBase {
                result = result + b.baseMethod() + "\n"
            }
            
            if isLeft {
                result = result + l.leftMethod() + "\n"
            }
            
            if isRight {
                result = result + r.rightMethod() + "\n"
            }
            
            if isOriginal {
                result = result + original.bottomMethod()
            }
            
            return result
        }
    "#";
    
    // Parse the test and verify it compiles correctly
    match parse_test(input) {
        Ok(program) => {
            // Check that the program parsed successfully
            println!("✅ Diamond inheritance test parsed successfully");
            println!("Program structure:\n{}", program.string());
            
            // For now, we just verify parsing - full execution would require JIT setup
            assert!(program.statements.len() > 0, "Program should have statements");
        },
        Err(e) => panic!("Failed to parse diamond inheritance test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_path_error_propagation() {
    init_tracing!();
    
    // Test with error propagation that includes path information
    let input = r#""
        // Define interfaces
        collab Animal {
            makeSound() tea;
        }
        
        collab Mammal : Animal {
            hasHair() lit;
        }
        
        collab Bird : Animal {
            canFly() lit;
        }
        
        // Define implementations
        squad Dog {
            name tea
        }
        
        squad Canary {
            name tea,
            wingspan normie
        }
        
        // Implement Animal and Mammal for Dog
        slay (d Dog) makeSound() tea {
            return d.name + " barks!"
        }
        
        slay (d Dog) hasHair() lit {
            return based
        }
        
        // Implement Animal and Bird for Canary
        slay (c Canary) makeSound() tea {
            return c.name + " chirps!"
        }
        
        slay (c Canary) canFly() lit {
            return c.wingspan > 5.0
        }
        
        // Main function with invalid assertion
        slay main() tea {
            // Create objects
            sus dog = Dog{name: "Rover"}
            sus bird = Canary{name: "Tweety", wingspan: 8.0}
            
            // Assign to interfaces
            sus dogAnimal Animal = dog
            sus birdAnimal Animal = bird
            
            // Valid assertion
            sus mammal, isMammal = dogAnimal.(Mammal)
            
            // Invalid assertion - try to convert dog to Bird
            sus birdTest, isBird = dogAnimal.(Bird)
            
            // Invalid assertion - try to convert bird to Mammal
            sus mammalTest, isMammalTest = birdAnimal.(Mammal)
            
            // Check results
            sus result tea = ""
            
            if isMammal {
                result = result + "Dog is a mammal: " + vibe.toString(mammal.hasHair()) + "\n"
            }
            
            if isBird {
                result = result + "Dog is a bird: " + vibe.toString(birdTest.canFly()) + "\n"
            } else {
                result = result + "Dog is not a bird\n"
            }
            
            if isMammalTest {
                result = result + "Bird is a mammal: " + vibe.toString(mammalTest.hasHair())
            } else {
                result = result + "Bird is not a mammal"
            }
            
            return result
        }
    "#";
    
    // Parse the test and verify it compiles correctly
    match parse_test(input) {
        Ok(program) => {
            // Check that the program parsed successfully
            println!("✅ Path error propagation test parsed successfully");
            println!("Program structure:\n{}", program.string());
            
            // For now, we just verify parsing - full execution would require JIT setup
            assert!(program.statements.len() > 0, "Program should have statements");
        },
        Err(e) => panic!("Failed to parse path error propagation test: {}", e),
    }
}