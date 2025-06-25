// Type Switch Demo - Showcasing CURSED type switch (vibe_check) compilation
// This example demonstrates the type switch feature with interface types and variable binding

package type_switch_demo;

// Define some types to work with
squad Person {
    name: tea,
    age: normie,
}

squad Company {
    name: tea,
    employees: normie,
    revenue: vibes,
}

squad Product {
    name: tea,
    price: vibes,
    category: tea,
}

// Interface for processing entities
collab EntityProcessor {
    fn process(entity: sus) -> tea;
}

// Main function demonstrating type switches
fn main() {
    // Create some test entities
    sus person = Person { name: "Alice", age: 25 };
    sus company = Company { name: "Tech Corp", employees: 100, revenue: 1000000.0 };
    sus product = Product { name: "Laptop", price: 999.99, category: "Electronics" };
    
    // Array of mixed entities
    sus entities = [person, company, product];
    
    // Process each entity using type switches
    lowkey (sus entity in entities) {
        process_entity(entity);
    }
}

// Function that uses type switch to handle different entity types
fn process_entity(entity: sus) -> tea {
    // Type switch with vibe_check syntax
    vibe_check entity.(Type) {
        mood Person as person: {
            sus greeting = "Hello, " + person.name + "!";
            lowkey (person.age >= 18) {
                greeting = greeting + " (Adult)";
            } bestie {
                greeting = greeting + " (Minor)";
            }
            facts "Processed person: " + greeting;
        }
        
        mood Company as company: {
            sus description = company.name + " has " + company.employees + " employees";
            lowkey (company.revenue > 500000.0) {
                description = description + " (Large company)";
            } bestie {
                description = description + " (Small company)";
            }
            facts "Processed company: " + description;
        }
        
        mood Product as product: {
            sus info = product.name + " costs $" + product.price;
            lowkey (product.price > 500.0) {
                info = info + " (Premium)";
            } bestie {
                info = info + " (Budget)";
            }
            facts "Processed product: " + info;
        }
        
        basic: {
            facts "Unknown entity type - no cap!";
        }
    }
}

// Advanced type switch with nested patterns
fn advanced_type_switch_demo(value: sus) {
    vibe_check value.(Type) {
        mood tea as text: {
            lowkey (text.length() > 10) {
                println("Long string: " + text);
            } bestie {
                println("Short string: " + text);
            }
        }
        
        mood normie as number: {
            lowkey (number > 0) {
                println("Positive number: " + number);
            } bestie lowkey (number < 0) {
                println("Negative number: " + number);
            } bestie {
                println("Zero");
            }
        }
        
        mood facts as flag: {
            lowkey (flag) {
                println("True value");
            } bestie {
                println("False value");
            }
        }
        
        basic: {
            yeet_error("Unsupported type in advanced demo");
        }
    }
}

// Type switch with multiple interface types
collab Drawable {
    fn draw() -> tea;
}

collab Clickable {
    fn click() -> tea;
}

fn handle_ui_element(element: sus) {
    vibe_check element.(Type) {
        mood Drawable as drawable: {
            stan drawable.draw();
        }
        
        mood Clickable as clickable: {
            stan clickable.click();
        }
        
        basic: {
            println("Element doesn't support drawing or clicking");
        }
    }
}

// Type switch in error handling context
fn safe_type_switch_demo(maybe_value: sus?) -> tea? {
    lowkey (maybe_value != nil) {
        vibe_check maybe_value.(Type) {
            mood tea as text: {
                facts "String value: " + text;
            }
            
            mood normie as num: {
                facts "Number value: " + num;
            }
            
            basic: {
                facts "Other type";
            }
        }
    } bestie {
        facts nil;
    }
}

// Type switch with goroutines
fn concurrent_type_processing(entities: [sus]) {
    lowkey (sus entity in entities) {
        stan {
            vibe_check entity.(Type) {
                mood Person as person: {
                    // Process person in background
                    println("Processing person: " + person.name + " concurrently");
                }
                
                mood Company as company: {
                    // Process company in background
                    println("Processing company: " + company.name + " concurrently");
                }
                
                basic: {
                    println("Processing unknown entity concurrently");
                }
            }
        };
        
        yolo; // Yield point for cooperative scheduling
    }
}

// Type switch with result types
fn type_switch_with_results(input: sus) -> Result<tea, tea> {
    vibe_check input.(Type) {
        mood tea as text: {
            lowkey (text.length() > 0) {
                Ok("Valid string: " + text)
            } bestie {
                Err("Empty string")
            }
        }
        
        mood normie as number: {
            lowkey (number >= 0) {
                Ok("Valid number: " + number)
            } bestie {
                Err("Negative number")
            }
        }
        
        basic: {
            Err("Unsupported type")
        }
    }
}
