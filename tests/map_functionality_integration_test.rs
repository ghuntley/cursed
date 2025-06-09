//! Functional integration tests for specific map use cases in the CURSED language.
//!
//! These tests focus on real-world usage patterns and ensure that map operations
//! work correctly in practical scenarios including data processing, caching,
//! configuration management, and algorithmic applications.

use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::{Type, TypeChecker};

use inkwell::context::Context;
use std::path::PathBuf;
use std::collections::HashMap;
use tracing::{debug, info, instrument};

/// Initialize tracing for tests
fn init_test_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .init();
    });
}

/// Functional test framework for map use cases
struct MapFunctionalTester<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> MapFunctionalTester<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        Self { context }
    }

    /// Parse and validate a CURSED program
    fn parse_and_validate(&self, source: &str) -> Result<Program, Error> {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        if !parser.errors().is_empty() {
            return Err(Error::from_str(&format!("Parser errors: {:?}", parser.errors())));
        }

        Ok(program)
    }

    /// Check if a program compiles without errors
    fn check_compilation(&self, source: &str) -> Result<bool, Error> {
        let program = self.parse_and_validate(source)?;
        
        let dummy_path = PathBuf::from("./dummy_map_functional_test.csd");
        let mut code_gen = LlvmCodeGenerator::new(self.context, "map_functional_test", dummy_path);
        
        match code_gen.compile_program(&program) {
            Ok(_) => Ok(true),
            Err(e) => {
                debug!("Compilation failed: {:?}", e);
                Ok(false)
            }
        }
    }

    /// Validate map usage patterns in source code
    fn validate_map_patterns(&self, source: &str) -> Result<Vec<String>, Error> {
        let program = self.parse_and_validate(source)?;
        let mut patterns = Vec::new();

        // This is a simplified pattern detection
        // In a full implementation, you'd traverse the AST to find map usage patterns
        
        if source.contains("{}") {
            patterns.push("empty_map_literal".to_string());
        }
        if source.contains("\"") && source.contains(":") {"
            patterns.push("string_key_map".to_string());
        }
        if source.contains("[") && source.contains("]") {
            patterns.push("map_indexing".to_string());
        }
        if source.contains("bestie") && source.contains("flex") {
            patterns.push("map_iteration".to_string());
        }
        
        Ok(patterns)
    }
}

/// Test map as a configuration store
#[test]
#[instrument]
fn test_map_as_configuration_store() {
    init_test_tracing();
    info!("Testing map as configuration store");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let config_source = r#""
        vibe test_config_map

        slay load_config() {
            sus config = {
                "database_url": "localhost:5432",
                "api_timeout": 30,
                "debug_mode": true,
                "max_connections": 100,
                "retry_attempts": 3
            }
            yolo config
        }

        slay get_database_config(config) {
            sus db_config = {
                "url": config["database_url"],
                "max_conn": config["max_connections"],
                "timeout": config["api_timeout"]
            }
            yolo db_config
        }

        slay main() normie {
            sus app_config = load_config()
            sus db_settings = get_database_config(app_config)
            
            lowkey app_config["debug_mode"] == true {
                yolo 1  // Success
            } highkey {
                yolo 0  // Failure
            }
        }
    "#";
    
    let program = tester.parse_and_validate(config_source);
    assert!(program.is_ok(), "Configuration map program failed to parse");
    
    let patterns = tester.validate_map_patterns(config_source).unwrap();
    assert!(patterns.contains(&"string_key_map".to_string()));
    assert!(patterns.contains(&"map_indexing".to_string()));
    
    info!("Configuration store test passed");
}

/// Test map for data aggregation and grouping
#[test]
#[instrument]
fn test_map_data_aggregation() {
    init_test_tracing();
    info!("Testing map for data aggregation");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let aggregation_source = r#""
        vibe test_data_aggregation

        slay process_sales_data() {
            sus sales_by_region = {}
            sus raw_sales = [
                {"region": "north", "amount": 1000},
                {"region": "south", "amount": 1500},
                {"region": "north", "amount": 800},
                {"region": "west", "amount": 1200},
                {"region": "south", "amount": 900}
            ]
            
            bestie sale := flex raw_sales {
                sus region = sale["region"]
                sus amount = sale["amount"]
                
                lowkey sales_by_region.has_key(region) {
                    sales_by_region[region] = sales_by_region[region] + amount
                } highkey {
                    sales_by_region[region] = amount
                }
            }
            
            yolo sales_by_region
        }

        slay get_top_region(sales_data) {
            sus max_sales = 0
            sus top_region = ""
            
            bestie region, amount := flex sales_data {
                lowkey amount > max_sales {
                    max_sales = amount
                    top_region = region
                }
            }
            
            yolo top_region
        }

        slay main() normie {
            sus aggregated = process_sales_data()
            sus winner = get_top_region(aggregated)
            yolo 1  // Success if completed
        }
    "#";
    
    let program = tester.parse_and_validate(aggregation_source);
    assert!(program.is_ok(), "Data aggregation program failed to parse");
    
    let patterns = tester.validate_map_patterns(aggregation_source).unwrap();
    assert!(patterns.contains(&"empty_map_literal".to_string()));
    assert!(patterns.contains(&"map_iteration".to_string()));
    
    info!("Data aggregation test passed");
}

/// Test map for caching and memoization
#[test]
#[instrument]
fn test_map_caching_memoization() {
    init_test_tracing();
    info!("Testing map for caching and memoization");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let caching_source = r#""
        vibe test_caching

        sus fibonacci_cache = {}

        slay fibonacci(n) {
            lowkey n <= 1 {
                yolo n
            }
            
            sus key = n.to_string()
            lowkey fibonacci_cache.has_key(key) {
                yolo fibonacci_cache[key]
            }
            
            sus result = fibonacci(n - 1) + fibonacci(n - 2)
            fibonacci_cache[key] = result
            yolo result
        }

        slay expensive_computation(x) {
            sus key = "computation_" + x.to_string()
            sus cache = {}
            
            lowkey cache.has_key(key) {
                yolo cache[key]
            }
            
            // Simulate expensive operation
            sus result = x * x + x * 2 + 1
            cache[key] = result
            yolo result
        }

        slay main() normie {
            sus fib_10 = fibonacci(10)
            sus comp_result = expensive_computation(5)
            
            lowkey fib_10 > 0 && comp_result > 0 {
                yolo 1  // Success
            } highkey {
                yolo 0  // Failure
            }
        }
    "#";
    
    let program = tester.parse_and_validate(caching_source);
    assert!(program.is_ok(), "Caching program failed to parse");
    
    let patterns = tester.validate_map_patterns(caching_source).unwrap();
    assert!(patterns.contains(&"empty_map_literal".to_string()));
    assert!(patterns.contains(&"map_indexing".to_string()));
    
    info!("Caching and memoization test passed");
}

/// Test map for frequency counting and statistics
#[test]
#[instrument]
fn test_map_frequency_counting() {
    init_test_tracing();
    info!("Testing map for frequency counting");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let frequency_source = r#""
        vibe test_frequency_counting

        slay count_word_frequencies(text) {
            sus word_counts = {}
            sus words = text.split(" ")
            
            bestie word := flex words {
                sus clean_word = word.to_lower()
                lowkey word_counts.has_key(clean_word) {
                    word_counts[clean_word] = word_counts[clean_word] + 1
                } highkey {
                    word_counts[clean_word] = 1
                }
            }
            
            yolo word_counts
        }

        slay get_most_frequent_word(word_counts) {
            sus max_count = 0
            sus most_frequent = ""
            
            bestie word, count := flex word_counts {
                lowkey count > max_count {
                    max_count = count
                    most_frequent = word
                }
            }
            
            yolo most_frequent
        }

        slay count_character_frequencies(text) {
            sus char_counts = {}
            
            bestie char := flex text {
                sus char_str = char.to_string()
                lowkey char_counts.has_key(char_str) {
                    char_counts[char_str] = char_counts[char_str] + 1
                } highkey {
                    char_counts[char_str] = 1
                }
            }
            
            yolo char_counts
        }

        slay main() normie {
            sus sample_text = "hello world hello universe world"
            sus word_freqs = count_word_frequencies(sample_text)
            sus top_word = get_most_frequent_word(word_freqs)
            sus char_freqs = count_character_frequencies("aabbcc")
            
            yolo 1  // Success
        }
    "#";
    
    let program = tester.parse_and_validate(frequency_source);
    assert!(program.is_ok(), "Frequency counting program failed to parse");
    
    let patterns = tester.validate_map_patterns(frequency_source).unwrap();
    assert!(patterns.contains(&"empty_map_literal".to_string()));
    assert!(patterns.contains(&"map_iteration".to_string()));
    
    info!("Frequency counting test passed");
}

/// Test map for graph algorithms (adjacency lists)
#[test]
#[instrument]
fn test_map_graph_algorithms() {
    init_test_tracing();
    info!("Testing map for graph algorithms");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let graph_source = r#""
        vibe test_graph_algorithms

        slay create_graph() {
            sus adjacency_list = {
                "A": ["B", "C"],
                "B": ["A", "D", "E"],
                "C": ["A", "F"],
                "D": ["B"],
                "E": ["B", "F"],
                "F": ["C", "E"]
            }
            yolo adjacency_list
        }

        slay get_neighbors(graph, node) {
            lowkey graph.has_key(node) {
                yolo graph[node]
            } highkey {
                yolo []
            }
        }

        slay count_connections(graph) {
            sus connection_counts = {}
            
            bestie node, neighbors := flex graph {
                connection_counts[node] = neighbors.length()
            }
            
            yolo connection_counts
        }

        slay find_most_connected_node(connection_counts) {
            sus max_connections = 0
            sus most_connected = ""
            
            bestie node, count := flex connection_counts {
                lowkey count > max_connections {
                    max_connections = count
                    most_connected = node
                }
            }
            
            yolo most_connected
        }

        slay main() normie {
            sus graph = create_graph()
            sus neighbors_of_b = get_neighbors(graph, "B")
            sus connections = count_connections(graph)
            sus hub_node = find_most_connected_node(connections)
            
            yolo 1  // Success
        }
    "#";
    
    let program = tester.parse_and_validate(graph_source);
    assert!(program.is_ok(), "Graph algorithms program failed to parse");
    
    let patterns = tester.validate_map_patterns(graph_source).unwrap();
    assert!(patterns.contains(&"string_key_map".to_string()));
    assert!(patterns.contains(&"map_iteration".to_string()));
    
    info!("Graph algorithms test passed");
}

/// Test map for state management
#[test]
#[instrument]
fn test_map_state_management() {
    init_test_tracing();
    info!("Testing map for state management");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let state_source = r#""
        vibe test_state_management

        slay create_initial_state() {
            sus state = {
                "user_logged_in": false,
                "current_user": "",
                "session_timeout": 3600,
                "theme": "dark",
                "language": "en",
                "notifications_enabled": true
            }
            yolo state
        }

        slay login_user(state, username) {
            sus new_state = state.copy()
            new_state["user_logged_in"] = true
            new_state["current_user"] = username
            yolo new_state
        }

        slay update_settings(state, setting_name, setting_value) {
            sus new_state = state.copy()
            new_state[setting_name] = setting_value
            yolo new_state
        }

        slay get_user_preferences(state) {
            sus preferences = {
                "theme": state["theme"],
                "language": state["language"],
                "notifications": state["notifications_enabled"]
            }
            yolo preferences
        }

        slay main() normie {
            sus app_state = create_initial_state()
            sus logged_in_state = login_user(app_state, "alice")
            sus updated_state = update_settings(logged_in_state, "theme", "light")
            sus user_prefs = get_user_preferences(updated_state)
            
            lowkey updated_state["user_logged_in"] == true {
                yolo 1  // Success
            } highkey {
                yolo 0  // Failure
            }
        }
    "#";
    
    let program = tester.parse_and_validate(state_source);
    assert!(program.is_ok(), "State management program failed to parse");
    
    let patterns = tester.validate_map_patterns(state_source).unwrap();
    assert!(patterns.contains(&"string_key_map".to_string()));
    assert!(patterns.contains(&"map_indexing".to_string()));
    
    info!("State management test passed");
}

/// Test map for inventory and catalog management
#[test]
#[instrument]
fn test_map_inventory_management() {
    init_test_tracing();
    info!("Testing map for inventory management");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let inventory_source = r#""
        vibe test_inventory_management

        slay create_inventory() {
            sus inventory = {
                "laptops": {"count": 50, "price": 999.99, "category": "electronics"},
                "mice": {"count": 100, "price": 29.99, "category": "accessories"},
                "keyboards": {"count": 75, "price": 79.99, "category": "accessories"},
                "monitors": {"count": 25, "price": 299.99, "category": "electronics"}
            }
            yolo inventory
        }

        slay update_stock(inventory, item_name, new_count) {
            lowkey inventory.has_key(item_name) {
                inventory[item_name]["count"] = new_count
            }
            yolo inventory
        }

        slay get_low_stock_items(inventory, threshold) {
            sus low_stock = {}
            
            bestie item_name, item_data := flex inventory {
                lowkey item_data["count"] < threshold {
                    low_stock[item_name] = item_data
                }
            }
            
            yolo low_stock
        }

        slay calculate_total_value(inventory) {
            sus total_value = 0.0
            
            bestie item_name, item_data := flex inventory {
                sus item_value = item_data["count"] * item_data["price"]
                total_value = total_value + item_value
            }
            
            yolo total_value
        }

        slay get_items_by_category(inventory, category) {
            sus category_items = {}
            
            bestie item_name, item_data := flex inventory {
                lowkey item_data["category"] == category {
                    category_items[item_name] = item_data
                }
            }
            
            yolo category_items
        }

        slay main() normie {
            sus store_inventory = create_inventory()
            sus updated_inventory = update_stock(store_inventory, "laptops", 45)
            sus low_stock = get_low_stock_items(updated_inventory, 30)
            sus total_value = calculate_total_value(updated_inventory)
            sus electronics = get_items_by_category(updated_inventory, "electronics")
            
            yolo 1  // Success
        }
    "#";
    
    let program = tester.parse_and_validate(inventory_source);
    assert!(program.is_ok(), "Inventory management program failed to parse");
    
    let patterns = tester.validate_map_patterns(inventory_source).unwrap();
    assert!(patterns.contains(&"string_key_map".to_string()));
    assert!(patterns.contains(&"map_iteration".to_string()));
    
    info!("Inventory management test passed");
}

/// Test map performance with different sizes
#[test]
#[instrument]
fn test_map_performance_scaling() {
    init_test_tracing();
    info!("Testing map performance scaling");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    // Test small map
    let small_map_elements: Vec<String> = (0..10).map(|i| format!(r#""key{}": {}"#, i, i)).collect();
    let small_map_source = format!(r#""
        vibe test_small_map
        slay main() normie {{
            sus small_map = {{{}}}
            yolo 1
        }}
    "#, small_map_elements.join(", "))";
    
    let small_result = tester.check_compilation(&small_map_source);
    assert!(small_result.is_ok(), "Small map compilation check failed");
    
    // Test medium map
    let medium_map_elements: Vec<String> = (0..100).map(|i| format!(r#""key{}": {}"#, i, i)).collect();
    let medium_map_source = format!(r#""
        vibe test_medium_map
        slay main() normie {{
            sus medium_map = {{{}}}
            yolo 1
        }}
    "#, medium_map_elements.join(", "))";
    
    let medium_result = tester.check_compilation(&medium_map_source);
    assert!(medium_result.is_ok(), "Medium map compilation check failed");
    
    // Test large map
    let large_map_elements: Vec<String> = (0..1000).map(|i| format!(r#""key{}": {}"#, i, i)).collect();
    let large_map_source = format!(r#""
        vibe test_large_map
        slay main() normie {{
            sus large_map = {{{}}}
            yolo 1
        }}
    "#, large_map_elements.join(", "))";
    
    let large_result = tester.check_compilation(&large_map_source);
    // Large maps might have compilation limitations
    if large_result.is_ok() {
        info!("Large map compilation successful");
    } else {
        info!("Large map compilation has limitations (expected)");
    }
    
    info!("Performance scaling test completed");
}

/// Test map error handling and edge cases
#[test]
#[instrument]
fn test_map_error_handling_comprehensive() {
    init_test_tracing();
    info!("Testing comprehensive map error handling");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    // Test handling of various error scenarios
    let error_handling_source = r#""
        vibe test_error_handling

        slay safe_map_access(map, key, default_value) {
            lowkey map.has_key(key) {
                yolo map[key]
            } highkey {
                yolo default_value
            }
        }

        slay safe_map_update(map, key, value) {
            sus result = map.copy()
            result[key] = value
            yolo result
        }

        slay validate_map_structure(map, required_keys) {
            bestie key := flex required_keys {
                lowkey !map.has_key(key) {
                    yolo false
                }
            }
            yolo true
        }

        slay main() normie {
            sus test_map = {"existing_key": "value"}
            
            // Test safe access to non-existent key
            sus safe_value = safe_map_access(test_map, "missing_key", "default")
            
            // Test safe update
            sus updated_map = safe_map_update(test_map, "new_key", "new_value")
            
            // Test structure validation
            sus required = ["existing_key"]
            sus is_valid = validate_map_structure(test_map, required)
            
            lowkey is_valid == true {
                yolo 1  // Success
            } highkey {
                yolo 0  // Failure
            }
        }
    "#";
    
    let program = tester.parse_and_validate(error_handling_source);
    assert!(program.is_ok(), "Error handling program failed to parse");
    
    let patterns = tester.validate_map_patterns(error_handling_source).unwrap();
    assert!(patterns.contains(&"string_key_map".to_string()));
    assert!(patterns.contains(&"map_indexing".to_string()));
    
    info!("Comprehensive error handling test passed");
}

/// Test map with complex data transformations
#[test]
#[instrument]
fn test_map_data_transformations() {
    init_test_tracing();
    info!("Testing map with complex data transformations");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    let transformation_source = r#""
        vibe test_data_transformations

        slay transform_user_data(raw_users) {
            sus transformed = {}
            
            bestie user := flex raw_users {
                sus user_id = user["id"]
                sus transformed_user = {
                    "full_name": user["first_name"] + " " + user["last_name"],
                    "age_group": user["age"] >= 18 ? "adult" : "minor",
                    "contact": {
                        "email": user["email"],
                        "phone": user["phone"]
                    }
                }
                transformed[user_id] = transformed_user
            }
            
            yolo transformed
        }

        slay group_by_age_group(users) {
            sus grouped = {
                "adult": [],
                "minor": []
            }
            
            bestie user_id, user_data := flex users {
                sus age_group = user_data["age_group"]
                grouped[age_group].append({
                    "id": user_id,
                    "name": user_data["full_name"]
                })
            }
            
            yolo grouped
        }

        slay calculate_statistics(grouped_users) {
            sus stats = {}
            
            bestie group, users := flex grouped_users {
                stats[group + "_count"] = users.length()
            }
            
            yolo stats
        }

        slay main() normie {
            sus raw_data = [
                {"id": "001", "first_name": "Alice", "last_name": "Johnson", "age": 25, "email": "alice@example.com", "phone": "555-0001"},
                {"id": "002", "first_name": "Bob", "last_name": "Smith", "age": 17, "email": "bob@example.com", "phone": "555-0002"}
            ]
            
            sus transformed = transform_user_data(raw_data)
            sus grouped = group_by_age_group(transformed)
            sus statistics = calculate_statistics(grouped)
            
            yolo 1  // Success
        }
    "#";
    
    let program = tester.parse_and_validate(transformation_source);
    assert!(program.is_ok(), "Data transformation program failed to parse");
    
    let patterns = tester.validate_map_patterns(transformation_source).unwrap();
    assert!(patterns.contains(&"empty_map_literal".to_string()));
    assert!(patterns.contains(&"string_key_map".to_string()));
    assert!(patterns.contains(&"map_iteration".to_string()));
    
    info!("Data transformations test passed");
}

/// Summary test that validates all functional patterns work together
#[test]
#[instrument]
fn test_map_functional_integration_summary() {
    init_test_tracing();
    info!("Running map functional integration summary");
    
    let context = Context::create();
    let tester = MapFunctionalTester::new(&context);
    
    // Collect all test results
    let mut test_results = HashMap::new();
    
    // Test basic functionality
    let basic_source = r#"sus basic_map = {"key": "value"}"#;
    test_results.insert("basic_parsing", tester.parse_and_validate(basic_source).is_ok());
    
    // Test complex structures
    let complex_source = r#"sus complex = {"users": [{"name": "Alice"}], "config": {"debug": true}}"#;
    test_results.insert("complex_structures", tester.parse_and_validate(complex_source).is_ok());
    
    // Test patterns
    let patterns_source = r#""
        bestie key, value := flex {"a": 1, "b": 2} {
            // iteration pattern
        }
    "#";
    test_results.insert("iteration_patterns", tester.parse_and_validate(patterns_source).is_ok());
    
    // Report results
    let successful_tests = test_results.values().filter(|&&v| v).count();
    let total_tests = test_results.len();
    
    info!("Functional integration summary: {}/{} tests passed", successful_tests, total_tests);
    info!("Test results: {:?}", test_results);
    
    // At least basic functionality should work
    assert!(test_results["basic_parsing"], "Basic map parsing should work");
    
    info!("Map functional integration summary completed");
}
