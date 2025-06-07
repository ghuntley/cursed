//! Dot Expression Registry
//! 
//! This module provides a registry for dot expressions (package.function).
//! It allows registering handlers for different package functions and
//! resolving dot expressions at runtime.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::error::Error;
use serde_json::{Value, json};

/// Function type for dot expression handlers (package functions) with string arguments
pub type DotHandlerFn = fn(Vec<String>) -> Result<String, Error>;

/// Function type for method handlers on user-defined types with string arguments
/// Takes the receiver object (as JSON) and arguments (as JSON strings)
pub type MethodHandlerFn = fn(String, Vec<String>) -> Result<String, Error>;

/// Function type for generic dot expression handlers with JSON arguments
pub type GenericDotHandlerFn = fn(Vec<Value>) -> Result<Value, Error>;

/// Function type for generic method handlers with JSON arguments
pub type GenericMethodHandlerFn = fn(Value, Vec<Value>) -> Result<Value, Error>;

/// Registry for dot expression handlers
pub struct DotRegistry {
    // Package name -> (Function name -> Handler)
    handlers: HashMap<String, HashMap<String, DotHandlerFn>>,
    // Type name -> (Method name -> Handler)
    methods: HashMap<String, HashMap<String, MethodHandlerFn>>,
    // Package name -> (Function name -> Generic Handler)
    generic_handlers: HashMap<String, HashMap<String, GenericDotHandlerFn>>,
    // Type name -> (Method name -> Generic Handler)
    generic_methods: HashMap<String, HashMap<String, GenericMethodHandlerFn>>,
}

/// Global dot expression registry
pub static DOT_REGISTRY: Lazy<Mutex<DotRegistry>> = Lazy::new(|| {
    let mut registry = DotRegistry::new();
    registry.init_stdlib();
    Mutex::new(registry)
});

impl DotRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        DotRegistry {
            handlers: HashMap::new(),
            methods: HashMap::new(),
            generic_handlers: HashMap::new(),
            generic_methods: HashMap::new(),
        }
    }
    
    /// Initialize standard library dot functions
    fn init_stdlib(&mut self) {
        // Register reflectz package directly to avoid circular dependency
        self.register_handler("reflectz", "TypeOf", |args| {
            // Simplified implementation
            Ok("<type>".to_string())
        });
        
        self.register_handler("reflectz", "ValueOf", |args| {
            // Simplified implementation
            Ok("<value>".to_string())
        });
        
        self.register_handler("reflectz", "IsType", |args| {
            if args.len() < 2 {
                return Err(crate::error::Error::from_str("reflectz.IsType requires 2 arguments"));
            }
            // Simplified implementation
            Ok("true".to_string())
        });
        
        self.register_handler("reflectz", "Implements", |args| {
            if args.len() < 2 {
                return Err(crate::error::Error::from_str("reflectz.Implements requires 2 arguments"));
            }
            // Simplified implementation
            Ok("true".to_string())
        });
        
        // Register vibez package
        self.register_handler("vibez", "spill", vibez_spill_handler);
        
        // Register htmlrizzler package
        self.register_handler("htmlrizzler", "escape_html", htmlrizzler_escape_html_handler);
        
        // Register timez package
        self.register_handler("timez", "Now", timez_now_handler);

        // Register cryptz package
        self.register_handler("cryptz", "md5sum", cryptz_md5sum_handler);
        self.register_handler("cryptz", "sha1sum", cryptz_sha1sum_handler);
        self.register_handler("cryptz", "sha256sum", cryptz_sha256sum_handler);
        self.register_handler("cryptz", "hmac", cryptz_hmac_handler);
        self.register_handler("cryptz", "random_bytes", cryptz_random_bytes_handler);
        
        // Register generic handlers for mathz package
        self.register_generic_handler("mathz", "CalculateArea", mathz_calculate_area_handler);
        self.register_generic_handler("mathz", "ConditionalCalculation", mathz_conditional_calculation_handler);
        
        // Register core package functions
        self.register_handler("core", "len", core_len_handler);
        self.register_handler("core", "cap", core_cap_handler);
        self.register_handler("core", "lit", core_lit_handler);
        self.register_handler("core", "normie", core_normie_handler);
        self.register_handler("core", "thicc", core_thicc_handler);
        self.register_handler("core", "snack", core_snack_handler);
        self.register_handler("core", "meal", core_meal_handler);
        self.register_handler("core", "tea", core_tea_handler);
        self.register_handler("core", "append", core_append_handler);
        self.register_handler("core", "make", core_make_handler);
        self.register_handler("core", "new", core_new_handler);
    }
    
    /// Register a handler for a dot expression
    #[tracing::instrument(skip(self, handler), fields(package = ?package, function = ?function), level = "debug")]
    pub fn register_handler(&mut self, package: &str, function: &str, handler: DotHandlerFn) {
        let package_handlers = self.handlers.entry(package.to_string())
            .or_insert_with(HashMap::new);
            
        package_handlers.insert(function.to_string(), handler);
    }
    
    /// Register a method for a user-defined type
    pub fn register_method(&mut self, type_name: &str, method_name: &str, handler: MethodHandlerFn) {
        let type_methods = self.methods.entry(type_name.to_string())
            .or_insert_with(HashMap::new);
            
        type_methods.insert(method_name.to_string(), handler);
    }
    
    /// Register a generic handler for a dot expression (with JSON arguments)
    pub fn register_generic_handler(&mut self, package: &str, function: &str, handler: GenericDotHandlerFn) {
        let package_handlers = self.generic_handlers.entry(package.to_string())
            .or_insert_with(HashMap::new);
            
        package_handlers.insert(function.to_string(), handler);
    }
    
    /// Register a generic method for a user-defined type (with JSON arguments)
    pub fn register_generic_method(&mut self, type_name: &str, method_name: &str, handler: GenericMethodHandlerFn) {
        let type_methods = self.generic_methods.entry(type_name.to_string())
            .or_insert_with(HashMap::new);
            
        type_methods.insert(method_name.to_string(), handler);
    }

    /// Check if a handler exists for a given package.function
    pub fn has_handler(&self, package: &str, function: &str) -> bool {
        if let Some(package_handlers) = self.handlers.get(package) {
            package_handlers.contains_key(function)
        } else {
            // Check generic handlers as fallback
            if let Some(generic_handlers) = self.generic_handlers.get(package) {
                generic_handlers.contains_key(function)
            } else {
                false
            }
        }
    }
    
    /// Check if a method exists for a given type.method
    pub fn has_method(&self, type_name: &str, method_name: &str) -> bool {
        if let Some(type_methods) = self.methods.get(type_name) {
            type_methods.contains_key(method_name)
        } else {
            // Check generic methods as fallback
            if let Some(generic_methods) = self.generic_methods.get(type_name) {
                generic_methods.contains_key(method_name)
            } else {
                false
            }
        }
    }
    
    /// Check if a generic handler exists for a given package.function
    pub fn has_generic_handler(&self, package: &str, function: &str) -> bool {
        if let Some(package_handlers) = self.generic_handlers.get(package) {
            package_handlers.contains_key(function)
        } else {
            false
        }
    }
    
    /// Check if a generic method exists for a given type.method
    pub fn has_generic_method(&self, type_name: &str, method_name: &str) -> bool {
        if let Some(type_methods) = self.generic_methods.get(type_name) {
            type_methods.contains_key(method_name)
        } else {
            false
        }
    }
    
    /// Get a handler for a dot expression
    pub fn get_handler(&self, package: &str, function: &str) -> Option<&DotHandlerFn> {
        self.handlers.get(package)
            .and_then(|package_handlers| package_handlers.get(function))
    }
    
    /// Get a method handler for a user-defined type
    pub fn get_method(&self, type_name: &str, method_name: &str) -> Option<&MethodHandlerFn> {
        self.methods.get(type_name)
            .and_then(|type_methods| type_methods.get(method_name))
    }
    
    /// Get a generic handler for a dot expression
    pub fn get_generic_handler(&self, package: &str, function: &str) -> Option<&GenericDotHandlerFn> {
        self.generic_handlers.get(package)
            .and_then(|package_handlers| package_handlers.get(function))
    }
    
    /// Get a generic method handler for a user-defined type
    pub fn get_generic_method(&self, type_name: &str, method_name: &str) -> Option<&GenericMethodHandlerFn> {
        self.generic_methods.get(type_name)
            .and_then(|type_methods| type_methods.get(method_name))
    }
    
    /// Execute a dot expression with the given arguments
    #[tracing::instrument(skip(self), fields(package = ?package, function = ?function, args_count = args.len()), level = "debug")]
    pub fn execute(&self, package: &str, function: &str, args: Vec<String>) -> Result<String, Error> {
        if let Some(handler) = self.get_handler(package, function) {
            handler(args)
        } else if let Some(generic_handler) = self.get_generic_handler(package, function) {
            // Convert string arguments to JSON Values
            let json_args = args.iter()
                .map(|s| parse_json_value(s))
                .collect();
            
            // Execute the generic handler and convert the result back to a string
            match generic_handler(json_args) {
                Ok(value) => Ok(value.to_string()),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::from_str(&format!("No handler found for {}.{}", package, function)))
        }
    }
    
    /// Execute a method on a user-defined type
    pub fn execute_method(&self, type_name: &str, method_name: &str, object_json: String, args: Vec<String>) -> Result<String, Error> {
        if let Some(handler) = self.get_method(type_name, method_name) {
            handler(object_json, args)
        } else if let Some(generic_handler) = self.get_generic_method(type_name, method_name) {
            // Convert object JSON string to a Value
            match serde_json::from_str::<Value>(&object_json) {
                Ok(obj) => {
                    // Convert string arguments to JSON Values
                    let json_args = args.iter()
                        .map(|s| parse_json_value(s))
                        .collect();
                    
                    // Execute the generic handler and convert the result back to a string
                    match generic_handler(obj, json_args) {
                        Ok(value) => Ok(value.to_string()),
                        Err(e) => Err(e),
                    }
                },
                Err(_) => Err(Error::from_str("Invalid JSON for object")),
            }
        } else {
            Err(Error::from_str(&format!("No method found for {}.{}", type_name, method_name)))
        }
    }
    
    /// Execute a dot expression with generic JSON arguments
    pub fn execute_generic(&self, package: &str, function: &str, args: Vec<Value>) -> Result<Value, Error> {
        if let Some(generic_handler) = self.get_generic_handler(package, function) {
            generic_handler(args)
        } else if let Some(handler) = self.get_handler(package, function) {
            // Convert JSON Values to strings for the string-based handler
            let string_args = args.iter()
                .map(|v| v.to_string())
                .collect();
            
            // Execute the string handler and parse the result back to JSON
            match handler(string_args) {
                Ok(result) => serde_json::from_str(&result).map_err(|_| Error::from_str("Failed to parse result as JSON")),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::from_str(&format!("No handler found for {}.{}", package, function)))
        }
    }
    
    /// Execute a method with generic JSON arguments
    pub fn execute_generic_method(&self, type_name: &str, method_name: &str, object: Value, args: Vec<Value>) -> Result<Value, Error> {
        if let Some(generic_handler) = self.get_generic_method(type_name, method_name) {
            generic_handler(object, args)
        } else if let Some(handler) = self.get_method(type_name, method_name) {
            // Convert object to JSON string
            let object_json = object.to_string();
            
            // Convert JSON Values to strings for the string-based handler
            let string_args = args.iter()
                .map(|v| v.to_string())
                .collect();
            
            // Execute the string handler and parse the result back to JSON
            match handler(object_json, string_args) {
                Ok(result) => serde_json::from_str(&result).map_err(|_| Error::from_str("Failed to parse result as JSON")),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::from_str(&format!("No method found for {}.{}", type_name, method_name)))
        }
    }
    
    /// Get all registered packages
    pub fn packages(&self) -> Vec<String> {
        let mut packages: Vec<String> = self.handlers.keys().cloned().collect();
        // Add packages from generic handlers that aren't already in the list
        for package in self.generic_handlers.keys() {
            if !packages.contains(package) {
                packages.push(package.clone());
            }
        }
        packages
    }
    
    /// Get all registered user-defined types
    pub fn types(&self) -> Vec<String> {
        let mut types: Vec<String> = self.methods.keys().cloned().collect();
        // Add types from generic methods that aren't already in the list
        for typ in self.generic_methods.keys() {
            if !types.contains(typ) {
                types.push(typ.clone());
            }
        }
        types
    }
    
    /// Get all registered functions for a package
    pub fn functions(&self, package: &str) -> Vec<String> {
        let mut functions = Vec::new();
        // Get functions from string handlers
        if let Some(package_handlers) = self.handlers.get(package) {
            functions.extend(package_handlers.keys().cloned());
        }
        // Add functions from generic handlers that aren't already in the list
        if let Some(package_handlers) = self.generic_handlers.get(package) {
            for function in package_handlers.keys() {
                if !functions.contains(function) {
                    functions.push(function.clone());
                }
            }
        }
        functions
    }
    
    /// Get all registered methods for a user-defined type
    pub fn methods(&self, type_name: &str) -> Vec<String> {
        let mut methods = Vec::new();
        // Get methods from string handlers
        if let Some(type_methods) = self.methods.get(type_name) {
            methods.extend(type_methods.keys().cloned());
        }
        // Add methods from generic handlers that aren't already in the list
        if let Some(type_methods) = self.generic_methods.get(type_name) {
            for method in type_methods.keys() {
                if !methods.contains(method) {
                    methods.push(method.clone());
                }
            }
        }
        methods
    }
}

// Helper function to parse a string into a JSON Value
fn parse_json_value(s: &str) -> Value {
    // Try to parse as a JSON value
    if let Ok(value) = serde_json::from_str::<Value>(s) {
        return value;
    }
    
    // Try to parse as a number
    if let Ok(num) = s.parse::<f64>() {
        return json!(num);
    }
    
    // Try to parse as a boolean
    if s == "true" || s == "True" {
        return Value::Bool(true);
    } else if s == "false" || s == "False" {
        return Value::Bool(false);
    }
    
    // Default to a string
    Value::String(s.to_string())
}

// Standard handler implementations

/// Handler for vibez.spill function
fn vibez_spill_handler(args: Vec<String>) -> Result<String, Error> {
    if let Some(arg) = args.get(0) {
        println!("{}", arg);
        Ok(arg.clone())
    } else {
        Err(Error::from_str("vibez.spill requires one argument"))
    }
}

/// Handler for htmlrizzler.escape_html function
fn htmlrizzler_escape_html_handler(args: Vec<String>) -> Result<String, Error> {
    if let Some(arg) = args.get(0) {
        let escaped = arg
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;");
        Ok(escaped)
    } else {
        Err(Error::from_str("htmlrizzler.escape_html requires one argument"))
    }
}

/// Handler for timez.Now function
fn timez_now_handler(args: Vec<String>) -> Result<String, Error> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
        let time_str = format!("{}s", duration.as_secs());
        Ok(time_str)
    } else {
        Err(Error::from_str("Failed to get system time"))
    }
}

/// Generic handler for mathz.CalculateArea function
fn mathz_calculate_area_handler(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(Error::from_str("mathz.CalculateArea requires at least 2 arguments"));
    }
    
    // Extract the first argument (pi)
    let pi = args[0].as_f64().unwrap_or(3.14159);
    
    // Extract the second argument (radius)
    let radius = args[1].as_f64().unwrap_or(0.0);
    
    // Calculate the area of a circle: pi * r^2
    let area = pi * radius * radius;
    
    // Return the result as a JSON Value
    Ok(json!(area))
}

/// Generic handler for mathz.ConditionalCalculation function
fn mathz_conditional_calculation_handler(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(Error::from_str("mathz.ConditionalCalculation requires at least 2 arguments"));
    }
    
    // Extract the first argument (boolean flag)
    let is_enabled = args[0].as_bool().unwrap_or(false);
    
    // Extract the second argument (value)
    let value = args[1].as_f64().unwrap_or(0.0);
    
    // Perform a conditional calculation
    let result = if is_enabled {
        value * 2.0  // Double the value if enabled
    } else {
        value / 2.0  // Halve the value if disabled
    };
    
    // Return the result as a JSON Value
    Ok(json!(result))
}

// Core function handlers

/// Handler for core.len function
fn core_len_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.len requires one argument"));
    }

    // For strings, return the string length
    Ok(args[0].len().to_string())
}

/// Handler for core.cap function
fn core_cap_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.cap requires one argument"));
    }

    // This is a placeholder - in a real implementation, this would
    // determine the capacity based on the type of the argument
    Ok(args[0].capacity().to_string())
}

/// Handler for core.lit function (boolean conversion)
fn core_lit_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.lit requires one argument"));
    }

    let value = &args[0];
    let result = match value.as_str() {
        "0" | "false" | "False" | "" => "false",
        _ => "true",
    };

    Ok(result.to_string())
}

/// Handler for core.normie function (int32 conversion)
fn core_normie_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.normie requires one argument"));
    }

    let value = args[0].parse::<i32>().unwrap_or(0);
    Ok(value.to_string())
}

/// Handler for core.thicc function (int64 conversion)
fn core_thicc_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.thicc requires one argument"));
    }

    let value = args[0].parse::<i64>().unwrap_or(0);
    Ok(value.to_string())
}

/// Handler for core.snack function (float32 conversion)
fn core_snack_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.snack requires one argument"));
    }

    let value = args[0].parse::<f32>().unwrap_or(0.0);
    Ok(value.to_string())
}

/// Handler for core.meal function (float64 conversion)
fn core_meal_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.meal requires one argument"));
    }

    let value = args[0].parse::<f64>().unwrap_or(0.0);
    Ok(value.to_string())
}

/// Handler for core.tea function (string conversion)
fn core_tea_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.tea requires one argument"));
    }

    // Already a string, just return it
    Ok(args[0].clone())
}

/// Handler for core.append function
fn core_append_handler(args: Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(Error::from_str("core.append requires at least two arguments"));
    }

    // In a real implementation, this would parse the first argument as an array
    // and append the remaining arguments to it
    // For this simple handler, we'll just join all the arguments
    let slice = &args[0];
    let mut elements = slice.clone();
    for elem in &args[1..] {
        elements.push(',');
        elements.push_str(elem);
    }

    Ok(format!("[{}]", elements))
}

/// Handler for core.make function
fn core_make_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.make requires at least one argument"));
    }

    let type_name = &args[0];
    let size = args.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);

    match type_name.as_str() {
        "slice" | "array" => {
            // Create an array of nulls with the given size
            let mut nulls = Vec::new();
            for _ in 0..size {
                nulls.push("null");
            }
            Ok(format!("[{}]", nulls.join(", ")))
        },
        "map" => {
            // Create an empty map
            Ok("{}".to_string())
        },
        "channel" => {
            // Create a channel representation
            Ok(format!("channel({})", size))
        },
        _ => Err(Error::from_str(&format!("Unsupported type for make: {}", type_name))),
    }
}

/// Handler for core.new function
fn core_new_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("core.new requires one argument"));
    }

    let type_name = &args[0];
    match type_name.as_str() {
        "int" | "normie" => Ok("0".to_string()),
        "int64" | "thicc" => Ok("0".to_string()),
        "float32" | "snack" => Ok("0.0".to_string()),
        "float64" | "meal" => Ok("0.0".to_string()),
        "string" | "tea" => Ok("\"\"".to_string()),
        "bool" | "lit" => Ok("false".to_string()),
        _ => Ok("null".to_string()),
    }
}

// Global API functions

/// Global function to check if a dot expression is supported
pub fn is_supported(package: &str, function: &str) -> bool {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.has_handler(package, function)
    } else {
        false
    }
}

/// Global function to check if a method is supported for a type
pub fn is_method_supported(type_name: &str, method_name: &str) -> bool {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.has_method(type_name, method_name)
    } else {
        false
    }
}

/// Global function to execute a dot expression
pub fn execute_dot(package: &str, function: &str, args: Vec<String>) -> Result<String, Error> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.execute(package, function, args)
    } else {
        Err(Error::from_str("Failed to lock dot registry"))
    }
}

/// Global function to execute a method on a user-defined type
pub fn execute_method(type_name: &str, method_name: &str, object_json: String, args: Vec<String>) -> Result<String, Error> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.execute_method(type_name, method_name, object_json, args)
    } else {
        Err(Error::from_str("Failed to lock dot registry"))
    }
}

/// Global function to execute a dot expression with JSON arguments
pub fn execute_generic_dot(package: &str, function: &str, args: Vec<Value>) -> Result<Value, Error> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.execute_generic(package, function, args)
    } else {
        Err(Error::from_str("Failed to lock dot registry"))
    }
}

/// Global function to execute a method with JSON arguments
pub fn execute_generic_method(type_name: &str, method_name: &str, object: Value, args: Vec<Value>) -> Result<Value, Error> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.execute_generic_method(type_name, method_name, object, args)
    } else {
        Err(Error::from_str("Failed to lock dot registry"))
    }
}

/// Global function to get all registered packages
pub fn get_packages() -> Vec<String> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.packages()
    } else {
        Vec::new()
    }
}

/// Global function to get all registered types
pub fn get_types() -> Vec<String> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.types()
    } else {
        Vec::new()
    }
}

/// Global function to get all registered functions for a package
pub fn get_functions(package: &str) -> Vec<String> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.functions(package)
    } else {
        Vec::new()
    }
}

/// Global function to get all registered methods for a type
pub fn get_methods(type_name: &str) -> Vec<String> {
    if let Ok(registry) = DOT_REGISTRY.lock() {
        registry.methods(type_name)
    } else {
        Vec::new()
    }
}

// Cryptz package handlers

/// Handler for cryptz.md5sum function
fn cryptz_md5sum_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("cryptz.md5sum requires one argument"));
    }
    let input = &args[0];
    
    // Call the md5sum function from the cryptz module
    match crate::stdlib::cryptz::md5sum(&[Arc::new(crate::object::Object::String(input.clone()))]) {
        Ok(result) => {
            if let crate::object::Object::String(hash) = result.as_ref() {
                Ok(hash.clone())
            } else {
                Err(Error::from_str("Unexpected result type from md5sum"))
            }
        },
        Err(e) => Err(e),
    }
}

/// Handler for cryptz.sha1sum function
fn cryptz_sha1sum_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("cryptz.sha1sum requires one argument"));
    }
    let input = &args[0];
    
    // Call the sha1sum function from the cryptz module
    match crate::stdlib::cryptz::sha1sum(&[Arc::new(crate::object::Object::String(input.clone()))]) {
        Ok(result) => {
            if let crate::object::Object::String(hash) = result.as_ref() {
                Ok(hash.clone())
            } else {
                Err(Error::from_str("Unexpected result type from sha1sum"))
            }
        },
        Err(e) => Err(e),
    }
}

/// Handler for cryptz.sha256sum function
fn cryptz_sha256sum_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("cryptz.sha256sum requires one argument"));
    }
    let input = &args[0];
    
    // Call the sha256sum function from the cryptz module
    match crate::stdlib::cryptz::sha256sum(&[Arc::new(crate::object::Object::String(input.clone()))]) {
        Ok(result) => {
            if let crate::object::Object::String(hash) = result.as_ref() {
                Ok(hash.clone())
            } else {
                Err(Error::from_str("Unexpected result type from sha256sum"))
            }
        },
        Err(e) => Err(e),
    }
}

/// Handler for cryptz.hmac function
fn cryptz_hmac_handler(args: Vec<String>) -> Result<String, Error> {
    if args.len() < 3 {
        return Err(Error::from_str("cryptz.hmac requires 3 arguments: data, key, and algorithm"));
    }
    let data = &args[0];
    let key = &args[1];
    let algorithm = &args[2];
    
    // Call the hmac function from the cryptz module
    let hmac_args = vec![
        Arc::new(crate::object::Object::String(data.clone())),
        Arc::new(crate::object::Object::String(key.clone())),
        Arc::new(crate::object::Object::String(algorithm.clone())),
    ];
    
    match crate::stdlib::cryptz::hmac(&hmac_args) {
        Ok(result) => {
            if let crate::object::Object::String(hash) = result.as_ref() {
                Ok(hash.clone())
            } else {
                Err(Error::from_str("Unexpected result type from hmac"))
            }
        },
        Err(e) => Err(e),
    }
}

/// Handler for cryptz.random_bytes function
fn cryptz_random_bytes_handler(args: Vec<String>) -> Result<String, Error> {
    if args.is_empty() {
        return Err(Error::from_str("cryptz.random_bytes requires one argument"));
    }
    
    // Parse the length argument as an integer
    let length = match args[0].parse::<i64>() {
        Ok(n) => n,
        Err(_) => return Err(Error::from_str("cryptz.random_bytes expects an integer length")),
    };
    
    // Call the random_bytes function from the cryptz module
    match crate::stdlib::cryptz::random_bytes(&[Arc::new(crate::object::Object::Integer(length))]) {
        Ok(result) => {
            if let crate::object::Object::String(bytes_hex) = result.as_ref() {
                Ok(bytes_hex.clone())
            } else {
                Err(Error::from_str("Unexpected result type from random_bytes"))
            }
        },
        Err(e) => Err(e),
    }
}