/// Standard Library Function Registry for LLVM Code Generation
/// 
/// This module provides a comprehensive registry of all CURSED stdlib functions
/// with their LLVM type information, GC requirements, and metadata for proper
/// code generation and runtime linking.

use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, FunctionType, BasicType, BasicMetadataTypeEnum};
use inkwell::values::FunctionValue;
use inkwell::AddressSpace;

/// Function metadata for stdlib functions
#[derive(Debug, Clone)]
pub struct StdlibFunction {
    pub name: String,
    pub package: String,
    pub return_type: String,
    pub param_types: Vec<String>,
    pub requires_gc: bool,
    pub is_variadic: bool,
    pub description: String,
    pub llvm_name: String,
}

/// Comprehensive stdlib function registry
pub struct StdlibRegistry {
    functions: HashMap<String, StdlibFunction>,
    qualified_functions: HashMap<String, StdlibFunction>,
    packages: HashMap<String, Vec<String>>,
}

/// LLVM integration for stdlib functions
pub struct StdlibLlvmIntegration<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    registry: StdlibRegistry,
    function_declarations: HashMap<String, FunctionValue<'ctx>>,
}

impl StdlibRegistry {
    /// Create a new stdlib registry with all packages
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
            qualified_functions: HashMap::new(),
            packages: HashMap::new(),
        };
        
        // Register all stdlib packages
        registry.register_core_functions();
        registry.register_vibez_functions();
        registry.register_mathz_functions();
        registry.register_stringz_functions();
        registry.register_dropz_functions();
        registry.register_concurrenz_functions();
        registry.register_web_vibez_functions(); // New HTTP package
        registry.register_json_tea_functions();
        registry.register_regex_vibez_functions();
        registry.register_cryptz_functions();
        registry.register_reflectz_functions();
        registry.register_rizztemplate_functions();
        registry.register_htmlrizzler_functions();
        registry.register_chadlogging_functions();
        registry.register_char_functions();
        registry.register_vibe_life_functions();
        registry.register_timez_functions();
        
        registry
    }
    
    /// Register core built-in functions
    fn register_core_functions(&mut self) {
        let core_functions = vec![
            StdlibFunction {
                name: "len".to_string(),
                package: "core".to_string(),
                return_type: "i64".to_string(),
                param_types: vec!["any".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Get length of array, slice, string, or map".to_string(),
                llvm_name: "cursed.len".to_string(),
            },
            StdlibFunction {
                name: "cap".to_string(),
                package: "core".to_string(),
                return_type: "i64".to_string(),
                param_types: vec!["any".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Get capacity of slice or channel".to_string(),
                llvm_name: "cursed.cap".to_string(),
            },
            StdlibFunction {
                name: "append".to_string(),
                package: "core".to_string(),
                return_type: "slice".to_string(),
                param_types: vec!["slice".to_string(), "any...".to_string()],
                requires_gc: true,
                is_variadic: true,
                description: "Append elements to slice".to_string(),
                llvm_name: "cursed.append".to_string(),
            },
            StdlibFunction {
                name: "make".to_string(),
                package: "core".to_string(),
                return_type: "any".to_string(),
                param_types: vec!["type".to_string(), "i64...".to_string()],
                requires_gc: true,
                is_variadic: true,
                description: "Create slice, map, or channel".to_string(),
                llvm_name: "cursed.make".to_string(),
            },
            StdlibFunction {
                name: "panic".to_string(),
                package: "core".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["any".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Panic with message".to_string(),
                llvm_name: "cursed.panic".to_string(),
            },
            StdlibFunction {
                name: "recover".to_string(),
                package: "core".to_string(),
                return_type: "any".to_string(),
                param_types: vec![],
                requires_gc: true,
                is_variadic: false,
                description: "Recover from panic".to_string(),
                llvm_name: "cursed.recover".to_string(),
            },
        ];
        
        self.register_package("core", core_functions);
    }
    
    /// Register web_vibez HTTP functions - COMPREHENSIVE SET
    fn register_web_vibez_functions(&mut self) {
        let web_vibez_functions = vec![
            // HTTP Server Functions
            StdlibFunction {
                name: "ListenAndServe".to_string(),
                package: "web_vibez".to_string(),
                return_type: "error".to_string(),
                param_types: vec!["string".to_string(), "handler".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Start HTTP server on given address".to_string(),
                llvm_name: "web_vibez.ListenAndServe".to_string(),
            },
            StdlibFunction {
                name: "ListenAndServeTLS".to_string(),
                package: "web_vibez".to_string(),
                return_type: "error".to_string(),
                param_types: vec!["string".to_string(), "string".to_string(), "string".to_string(), "handler".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Start HTTPS server with TLS certificates".to_string(),
                llvm_name: "web_vibez.ListenAndServeTLS".to_string(),
            },
            StdlibFunction {
                name: "HandleFunc".to_string(),
                package: "web_vibez".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["string".to_string(), "handler_func".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Register handler function for URL pattern".to_string(),
                llvm_name: "web_vibez.HandleFunc".to_string(),
            },
            
            // HTTP Client Functions
            StdlibFunction {
                name: "Get".to_string(),
                package: "web_vibez".to_string(),
                return_type: "response".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Send HTTP GET request".to_string(),
                llvm_name: "web_vibez.Get".to_string(),
            },
            StdlibFunction {
                name: "Post".to_string(),
                package: "web_vibez".to_string(),
                return_type: "response".to_string(),
                param_types: vec!["string".to_string(), "string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Send HTTP POST request with body".to_string(),
                llvm_name: "web_vibez.Post".to_string(),
            },
            StdlibFunction {
                name: "Head".to_string(),
                package: "web_vibez".to_string(),
                return_type: "response".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Send HTTP HEAD request".to_string(),
                llvm_name: "web_vibez.Head".to_string(),
            },
            StdlibFunction {
                name: "Delete".to_string(),
                package: "web_vibez".to_string(),
                return_type: "response".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Send HTTP DELETE request".to_string(),
                llvm_name: "web_vibez.Delete".to_string(),
            },
            StdlibFunction {
                name: "Put".to_string(),
                package: "web_vibez".to_string(),
                return_type: "response".to_string(),
                param_types: vec!["string".to_string(), "string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Send HTTP PUT request with body".to_string(),
                llvm_name: "web_vibez.Put".to_string(),
            },
            StdlibFunction {
                name: "Patch".to_string(),
                package: "web_vibez".to_string(),
                return_type: "response".to_string(),
                param_types: vec!["string".to_string(), "string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Send HTTP PATCH request with body".to_string(),
                llvm_name: "web_vibez.Patch".to_string(),
            },
            
            // Request Handling Functions
            StdlibFunction {
                name: "Request.URL".to_string(),
                package: "web_vibez".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["request".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get request URL".to_string(),
                llvm_name: "web_vibez.Request.URL".to_string(),
            },
            StdlibFunction {
                name: "Request.Method".to_string(),
                package: "web_vibez".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["request".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get request HTTP method".to_string(),
                llvm_name: "web_vibez.Request.Method".to_string(),
            },
            StdlibFunction {
                name: "Request.Header".to_string(),
                package: "web_vibez".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["request".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get request header value".to_string(),
                llvm_name: "web_vibez.Request.Header".to_string(),
            },
            StdlibFunction {
                name: "Request.Body".to_string(),
                package: "web_vibez".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["request".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get request body".to_string(),
                llvm_name: "web_vibez.Request.Body".to_string(),
            },
            StdlibFunction {
                name: "Request.FormValue".to_string(),
                package: "web_vibez".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["request".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get form field value".to_string(),
                llvm_name: "web_vibez.Request.FormValue".to_string(),
            },
            
            // Response Writing Functions
            StdlibFunction {
                name: "ResponseWriter.Write".to_string(),
                package: "web_vibez".to_string(),
                return_type: "i32".to_string(),
                param_types: vec!["response_writer".to_string(), "string".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Write data to response".to_string(),
                llvm_name: "web_vibez.ResponseWriter.Write".to_string(),
            },
            StdlibFunction {
                name: "ResponseWriter.WriteHeader".to_string(),
                package: "web_vibez".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["response_writer".to_string(), "i32".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Write HTTP status code".to_string(),
                llvm_name: "web_vibez.ResponseWriter.WriteHeader".to_string(),
            },
            StdlibFunction {
                name: "ResponseWriter.Header".to_string(),
                package: "web_vibez".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["response_writer".to_string(), "string".to_string(), "string".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Set response header".to_string(),
                llvm_name: "web_vibez.ResponseWriter.Header".to_string(),
            },
            
            // Utility Functions
            StdlibFunction {
                name: "client_timeout".to_string(),
                package: "web_vibez".to_string(),
                return_type: "i64".to_string(),
                param_types: vec!["i64...".to_string()],
                requires_gc: false,
                is_variadic: true,
                description: "Set or get HTTP client timeout".to_string(),
                llvm_name: "web_vibez.client_timeout".to_string(),
            },
            StdlibFunction {
                name: "NewServeMux".to_string(),
                package: "web_vibez".to_string(),
                return_type: "serve_mux".to_string(),
                param_types: vec![],
                requires_gc: true,
                is_variadic: false,
                description: "Create new HTTP request multiplexer".to_string(),
                llvm_name: "web_vibez.NewServeMux".to_string(),
            },
            StdlibFunction {
                name: "FileServer".to_string(),
                package: "web_vibez".to_string(),
                return_type: "handler".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Create file server handler".to_string(),
                llvm_name: "web_vibez.FileServer".to_string(),
            },
            StdlibFunction {
                name: "StripPrefix".to_string(),
                package: "web_vibez".to_string(),
                return_type: "handler".to_string(),
                param_types: vec!["string".to_string(), "handler".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Strip URL prefix from requests".to_string(),
                llvm_name: "web_vibez.StripPrefix".to_string(),
            },
            
            // Cookie Functions
            StdlibFunction {
                name: "SetCookie".to_string(),
                package: "web_vibez".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["response_writer".to_string(), "cookie".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Set HTTP cookie".to_string(),
                llvm_name: "web_vibez.SetCookie".to_string(),
            },
            StdlibFunction {
                name: "Request.Cookie".to_string(),
                package: "web_vibez".to_string(),
                return_type: "cookie".to_string(),
                param_types: vec!["request".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get cookie from request".to_string(),
                llvm_name: "web_vibez.Request.Cookie".to_string(),
            },
        ];
        
        self.register_package("web_vibez", web_vibez_functions);
    }
    
    /// Register vibez I/O functions
    fn register_vibez_functions(&mut self) {
        let vibez_functions = vec![
            StdlibFunction {
                name: "spill".to_string(),
                package: "vibez".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["any...".to_string()],
                requires_gc: false,
                is_variadic: true,
                description: "Print values to stdout".to_string(),
                llvm_name: "vibez.spill".to_string(),
            },
            StdlibFunction {
                name: "spillf".to_string(),
                package: "vibez".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["string".to_string(), "any...".to_string()],
                requires_gc: false,
                is_variadic: true,
                description: "Print formatted string".to_string(),
                llvm_name: "vibez.spillf".to_string(),
            },
            StdlibFunction {
                name: "spillstr".to_string(),
                package: "vibez".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string(), "any...".to_string()],
                requires_gc: true,
                is_variadic: true,
                description: "Format string without printing".to_string(),
                llvm_name: "vibez.spillstr".to_string(),
            },
        ];
        
        self.register_package("vibez", vibez_functions);
    }
    
    /// Register mathematical functions
    fn register_mathz_functions(&mut self) {
        let mathz_functions = vec![
            StdlibFunction {
                name: "abs".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Absolute value".to_string(),
                llvm_name: "mathz.abs".to_string(),
            },
            StdlibFunction {
                name: "sqrt".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Square root".to_string(),
                llvm_name: "mathz.sqrt".to_string(),
            },
            StdlibFunction {
                name: "sin".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Sine function".to_string(),
                llvm_name: "mathz.sin".to_string(),
            },
            StdlibFunction {
                name: "cos".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Cosine function".to_string(),
                llvm_name: "mathz.cos".to_string(),
            },
            StdlibFunction {
                name: "tan".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Tangent function".to_string(),
                llvm_name: "mathz.tan".to_string(),
            },
            StdlibFunction {
                name: "log".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Natural logarithm".to_string(),
                llvm_name: "mathz.log".to_string(),
            },
            StdlibFunction {
                name: "pow".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string(), "f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Power function".to_string(),
                llvm_name: "mathz.pow".to_string(),
            },
            StdlibFunction {
                name: "max".to_string(),
                package: "mathz".to_string(),
                return_type: "f64".to_string(),
                param_types: vec!["f64".to_string(), "f64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Maximum of two values".to_string(),
                llvm_name: "mathz.max".to_string(),
            },
        ];
        
        self.register_package("mathz", mathz_functions);
    }
    
    /// Register string manipulation functions
    fn register_stringz_functions(&mut self) {
        let stringz_functions = vec![
            StdlibFunction {
                name: "contains".to_string(),
                package: "stringz".to_string(),
                return_type: "bool".to_string(),
                param_types: vec!["string".to_string(), "string".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Check if string contains substring".to_string(),
                llvm_name: "stringz.contains".to_string(),
            },
            StdlibFunction {
                name: "join".to_string(),
                package: "stringz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["slice".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Join strings with separator".to_string(),
                llvm_name: "stringz.join".to_string(),
            },
            StdlibFunction {
                name: "split".to_string(),
                package: "stringz".to_string(),
                return_type: "slice".to_string(),
                param_types: vec!["string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Split string by separator".to_string(),
                llvm_name: "stringz.split".to_string(),
            },
            StdlibFunction {
                name: "trim".to_string(),
                package: "stringz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Trim whitespace from string".to_string(),
                llvm_name: "stringz.trim".to_string(),
            },
            StdlibFunction {
                name: "lower".to_string(),
                package: "stringz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Convert string to lowercase".to_string(),
                llvm_name: "stringz.lower".to_string(),
            },
            StdlibFunction {
                name: "upper".to_string(),
                package: "stringz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Convert string to uppercase".to_string(),
                llvm_name: "stringz.upper".to_string(),
            },
        ];
        
        self.register_package("stringz", stringz_functions);
    }
    
    /// Register remaining packages with placeholder functions
    fn register_dropz_functions(&mut self) {
        let dropz_functions = vec![
            StdlibFunction {
                name: "read_file".to_string(),
                package: "dropz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Read file contents".to_string(),
                llvm_name: "dropz.read_file".to_string(),
            },
            StdlibFunction {
                name: "write_file".to_string(),
                package: "dropz".to_string(),
                return_type: "error".to_string(),
                param_types: vec!["string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Write file contents".to_string(),
                llvm_name: "dropz.write_file".to_string(),
            },
            StdlibFunction {
                name: "exists".to_string(),
                package: "dropz".to_string(),
                return_type: "bool".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Check if file exists".to_string(),
                llvm_name: "dropz.exists".to_string(),
            },
        ];
        
        self.register_package("dropz", dropz_functions);
    }
    
    fn register_concurrenz_functions(&mut self) {
        let concurrenz_functions = vec![
            StdlibFunction {
                name: "new_mutex".to_string(),
                package: "concurrenz".to_string(),
                return_type: "mutex".to_string(),
                param_types: vec![],
                requires_gc: true,
                is_variadic: false,
                description: "Create new mutex".to_string(),
                llvm_name: "concurrenz.new_mutex".to_string(),
            },
            StdlibFunction {
                name: "new_channel".to_string(),
                package: "concurrenz".to_string(),
                return_type: "channel".to_string(),
                param_types: vec!["i64".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Create new channel".to_string(),
                llvm_name: "concurrenz.new_channel".to_string(),
            },
        ];
        
        self.register_package("concurrenz", concurrenz_functions);
    }
    
    fn register_json_tea_functions(&mut self) {
        let json_tea_functions = vec![
            StdlibFunction {
                name: "marshal".to_string(),
                package: "json_tea".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["any".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Marshal to JSON".to_string(),
                llvm_name: "json_tea.marshal".to_string(),
            },
            StdlibFunction {
                name: "unmarshal".to_string(),
                package: "json_tea".to_string(),
                return_type: "any".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Unmarshal from JSON".to_string(),
                llvm_name: "json_tea.unmarshal".to_string(),
            },
        ];
        
        self.register_package("json_tea", json_tea_functions);
    }
    
    fn register_regex_vibez_functions(&mut self) {
        let regex_vibez_functions = vec![
            StdlibFunction {
                name: "compile".to_string(),
                package: "regex_vibez".to_string(),
                return_type: "regex".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Compile regular expression".to_string(),
                llvm_name: "regex_vibez.compile".to_string(),
            },
            StdlibFunction {
                name: "match_str".to_string(),
                package: "regex_vibez".to_string(),
                return_type: "bool".to_string(),
                param_types: vec!["regex".to_string(), "string".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Test if string matches regex".to_string(),
                llvm_name: "regex_vibez.match_str".to_string(),
            },
        ];
        
        self.register_package("regex_vibez", regex_vibez_functions);
    }
    
    fn register_cryptz_functions(&mut self) {
        let cryptz_functions = vec![
            StdlibFunction {
                name: "hash".to_string(),
                package: "cryptz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Hash data with algorithm".to_string(),
                llvm_name: "cryptz.hash".to_string(),
            },
            StdlibFunction {
                name: "encrypt".to_string(),
                package: "cryptz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Encrypt data".to_string(),
                llvm_name: "cryptz.encrypt".to_string(),
            },
            StdlibFunction {
                name: "decrypt".to_string(),
                package: "cryptz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Decrypt data".to_string(),
                llvm_name: "cryptz.decrypt".to_string(),
            },
        ];
        
        self.register_package("cryptz", cryptz_functions);
    }
    
    fn register_reflectz_functions(&mut self) {
        let reflectz_functions = vec![
            StdlibFunction {
                name: "type_name".to_string(),
                package: "reflectz".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["any".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get type name".to_string(),
                llvm_name: "reflectz.type_name".to_string(),
            },
            StdlibFunction {
                name: "deep_equal".to_string(),
                package: "reflectz".to_string(),
                return_type: "bool".to_string(),
                param_types: vec!["any".to_string(), "any".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Deep equality comparison".to_string(),
                llvm_name: "reflectz.deep_equal".to_string(),
            },
        ];
        
        self.register_package("reflectz", reflectz_functions);
    }
    
    fn register_rizztemplate_functions(&mut self) {
        let rizztemplate_functions = vec![
            StdlibFunction {
                name: "parse_template".to_string(),
                package: "rizztemplate".to_string(),
                return_type: "template".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Parse template string".to_string(),
                llvm_name: "rizztemplate.parse_template".to_string(),
            },
            StdlibFunction {
                name: "execute_template".to_string(),
                package: "rizztemplate".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["template".to_string(), "any".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Execute template with data".to_string(),
                llvm_name: "rizztemplate.execute_template".to_string(),
            },
        ];
        
        self.register_package("rizztemplate", rizztemplate_functions);
    }
    
    fn register_htmlrizzler_functions(&mut self) {
        let htmlrizzler_functions = vec![
            StdlibFunction {
                name: "escape_html".to_string(),
                package: "htmlrizzler".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Escape HTML entities".to_string(),
                llvm_name: "htmlrizzler.escape_html".to_string(),
            },
            StdlibFunction {
                name: "escape_js".to_string(),
                package: "htmlrizzler".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Escape JavaScript string".to_string(),
                llvm_name: "htmlrizzler.escape_js".to_string(),
            },
        ];
        
        self.register_package("htmlrizzler", htmlrizzler_functions);
    }
    
    fn register_chadlogging_functions(&mut self) {
        let chadlogging_functions = vec![
            StdlibFunction {
                name: "debug".to_string(),
                package: "chadlogging".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["string".to_string(), "any...".to_string()],
                requires_gc: false,
                is_variadic: true,
                description: "Log debug message".to_string(),
                llvm_name: "chadlogging.debug".to_string(),
            },
            StdlibFunction {
                name: "info".to_string(),
                package: "chadlogging".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["string".to_string(), "any...".to_string()],
                requires_gc: false,
                is_variadic: true,
                description: "Log info message".to_string(),
                llvm_name: "chadlogging.info".to_string(),
            },
            StdlibFunction {
                name: "error".to_string(),
                package: "chadlogging".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["string".to_string(), "any...".to_string()],
                requires_gc: false,
                is_variadic: true,
                description: "Log error message".to_string(),
                llvm_name: "chadlogging.error".to_string(),
            },
        ];
        
        self.register_package("chadlogging", chadlogging_functions);
    }
    
    fn register_char_functions(&mut self) {
        let char_functions = vec![
            StdlibFunction {
                name: "is_uppercase".to_string(),
                package: "char".to_string(),
                return_type: "bool".to_string(),
                param_types: vec!["char".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Check if character is uppercase".to_string(),
                llvm_name: "char.is_uppercase".to_string(),
            },
            StdlibFunction {
                name: "to_lowercase".to_string(),
                package: "char".to_string(),
                return_type: "char".to_string(),
                param_types: vec!["char".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Convert character to lowercase".to_string(),
                llvm_name: "char.to_lowercase".to_string(),
            },
            StdlibFunction {
                name: "to_uppercase".to_string(),
                package: "char".to_string(),
                return_type: "char".to_string(),
                param_types: vec!["char".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Convert character to uppercase".to_string(),
                llvm_name: "char.to_uppercase".to_string(),
            },
        ];
        
        self.register_package("char", char_functions);
    }
    
    fn register_vibe_life_functions(&mut self) {
        let vibe_life_functions = vec![
            StdlibFunction {
                name: "getenv".to_string(),
                package: "vibe_life".to_string(),
                return_type: "string".to_string(),
                param_types: vec!["string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Get environment variable".to_string(),
                llvm_name: "vibe_life.getenv".to_string(),
            },
            StdlibFunction {
                name: "setenv".to_string(),
                package: "vibe_life".to_string(),
                return_type: "error".to_string(),
                param_types: vec!["string".to_string(), "string".to_string()],
                requires_gc: true,
                is_variadic: false,
                description: "Set environment variable".to_string(),
                llvm_name: "vibe_life.setenv".to_string(),
            },
            StdlibFunction {
                name: "exit".to_string(),
                package: "vibe_life".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["i32".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Exit program with code".to_string(),
                llvm_name: "vibe_life.exit".to_string(),
            },
        ];
        
        self.register_package("vibe_life", vibe_life_functions);
    }
    
    fn register_timez_functions(&mut self) {
        let timez_functions = vec![
            StdlibFunction {
                name: "now".to_string(),
                package: "timez".to_string(),
                return_type: "i64".to_string(),
                param_types: vec![],
                requires_gc: false,
                is_variadic: false,
                description: "Get current timestamp".to_string(),
                llvm_name: "timez.now".to_string(),
            },
            StdlibFunction {
                name: "sleep".to_string(),
                package: "timez".to_string(),
                return_type: "void".to_string(),
                param_types: vec!["i64".to_string()],
                requires_gc: false,
                is_variadic: false,
                description: "Sleep for duration".to_string(),
                llvm_name: "timez.sleep".to_string(),
            },
        ];
        
        self.register_package("timez", timez_functions);
    }
    
    /// Register a package with its functions
    fn register_package(&mut self, package_name: &str, functions: Vec<StdlibFunction>) {
        let mut function_names = Vec::new();
        
        for func in functions {
            let qualified_name = format!("{}.{}", package_name, func.name);
            
            // Store function
            self.functions.insert(func.name.clone(), func.clone());
            self.qualified_functions.insert(qualified_name, func.clone());
            function_names.push(func.name.clone());
        }
        
        self.packages.insert(package_name.to_string(), function_names);
    }
    
    /// Get function by name (unqualified)
    pub fn get_function(&self, name: &str) -> Option<&StdlibFunction> {
        self.functions.get(name)
    }
    
    /// Get function by qualified name (package.function)
    pub fn get_qualified_function(&self, qualified_name: &str) -> Option<&StdlibFunction> {
        self.qualified_functions.get(qualified_name)
    }
    
    /// Get all packages
    pub fn get_packages(&self) -> impl Iterator<Item = &String> {
        self.packages.keys()
    }
    
    /// Get functions in a package
    pub fn get_package_functions(&self, package: &str) -> Option<&Vec<String>> {
        self.packages.get(package)
    }
    
    /// Get all function names
    pub fn get_all_functions(&self) -> impl Iterator<Item = &String> {
        self.functions.keys()
    }
    
    /// Get total number of functions
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }
    
    /// Get total number of packages
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }
}

impl<'ctx> StdlibLlvmIntegration<'ctx> {
    /// Create new LLVM integration for stdlib
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Self {
        let registry = StdlibRegistry::new();
        let function_declarations = HashMap::new();
        
        Self {
            context,
            module,
            registry,
            function_declarations,
        }
    }
    
    /// Initialize function declarations in LLVM module
    pub fn initialize_function_declarations(&mut self) -> Result<(), String> {
        // Clone the function maps to avoid borrowing issues
        let functions: Vec<_> = self.registry.functions.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let qualified_functions: Vec<_> = self.registry.qualified_functions.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        
        for (name, func_info) in functions {
            self.declare_function(&name, &func_info)?;
        }
        
        for (qualified_name, func_info) in qualified_functions {
            self.declare_function(&qualified_name, &func_info)?;
        }
        
        Ok(())
    }
    
    /// Declare a function in the LLVM module
    fn declare_function(&mut self, name: &str, func_info: &StdlibFunction) -> Result<(), String> {
        let return_type = self.get_llvm_type(&func_info.return_type)?;
        let param_types: Result<Vec<BasicMetadataTypeEnum>, String> = func_info.param_types
            .iter()
            .map(|t| self.get_llvm_type(t).map(|bt| bt.into()))
            .collect();
        
        let param_types = param_types?;
        
        let function_type = if func_info.return_type == "void" {
            self.context.void_type().fn_type(&param_types, func_info.is_variadic)
        } else {
            return_type.fn_type(&param_types, func_info.is_variadic)
        };
        
        let function = self.module.add_function(&func_info.llvm_name, function_type, None);
        self.function_declarations.insert(name.to_string(), function);
        
        Ok(())
    }
    
    /// Convert CURSED type to LLVM type
    fn get_llvm_type(&self, cursed_type: &str) -> Result<BasicTypeEnum<'ctx>, String> {
        match cursed_type {
            "void" => Err("void type not allowed as BasicType".to_string()),
            "i32" => Ok(self.context.i32_type().into()),
            "i64" => Ok(self.context.i64_type().into()),
            "f64" => Ok(self.context.f64_type().into()),
            "bool" => Ok(self.context.bool_type().into()),
            "char" => Ok(self.context.i8_type().into()),
            "string" | "slice" | "any" | "error" | "mutex" | "channel" | "regex" | "template" | 
            "response" | "request" | "response_writer" | "handler" | "serve_mux" | "cookie" => {
                // These are pointer types to structures
                let i8_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(i8_ptr.into())
            },
            "any..." => {
                // Variadic parameters - use i8*
                let i8_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(i8_ptr.into())
            },
            "i64..." => Ok(self.context.i64_type().into()),
            _ => {
                // Unknown type, default to i8*
                let i8_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(i8_ptr.into())
            }
        }
    }
    
    /// Get function info by name
    pub fn get_function_info(&self, name: &str) -> Option<&StdlibFunction> {
        self.registry.get_function(name)
            .or_else(|| self.registry.get_qualified_function(name))
    }
    
    /// Get function declaration by name
    pub fn get_function_declaration(&self, name: &str) -> Option<&FunctionValue<'ctx>> {
        self.function_declarations.get(name)
    }
    
    /// Get all packages
    pub fn get_packages(&self) -> impl Iterator<Item = &String> {
        self.registry.get_packages()
    }
    
    /// Check if package exists
    pub fn has_package(&self, package: &str) -> bool {
        self.registry.packages.contains_key(package)
    }
    
    /// Get function count
    pub fn function_count(&self) -> usize {
        self.registry.function_count()
    }
    
    /// Get package count
    pub fn package_count(&self) -> usize {
        self.registry.package_count()
    }
}

impl Default for StdlibRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_stdlib_registry_creation() {
        let registry = StdlibRegistry::new();
        
        // Test basic functionality
        assert!(registry.function_count() > 0);
        assert!(registry.package_count() > 0);
        
        // Test core functions
        assert!(registry.get_function("len").is_some());
        assert!(registry.get_function("cap").is_some());
        
        // Test web_vibez functions
        assert!(registry.get_qualified_function("web_vibez.Get").is_some());
        assert!(registry.get_qualified_function("web_vibez.Post").is_some());
        assert!(registry.get_qualified_function("web_vibez.ListenAndServe").is_some());
        
        // Test package listing
        let packages: Vec<_> = registry.get_packages().collect();
        assert!(packages.contains(&&"web_vibez".to_string()));
        assert!(packages.contains(&&"core".to_string()));
        assert!(packages.contains(&&"vibez".to_string()));
    }
    
    #[test]
    fn test_web_vibez_comprehensive_functions() {
        let registry = StdlibRegistry::new();
        
        // Test all web_vibez functions are registered
        let web_vibez_functions = vec![
            "web_vibez.ListenAndServe",
            "web_vibez.ListenAndServeTLS", 
            "web_vibez.HandleFunc",
            "web_vibez.Get",
            "web_vibez.Post",
            "web_vibez.Head",
            "web_vibez.Delete",
            "web_vibez.Put",
            "web_vibez.Patch",
            "web_vibez.client_timeout",
            "web_vibez.Request.URL",
            "web_vibez.Request.Method",
            "web_vibez.Request.Header",
            "web_vibez.Request.Body",
            "web_vibez.ResponseWriter.Write",
            "web_vibez.ResponseWriter.WriteHeader",
            "web_vibez.ResponseWriter.Header",
        ];
        
        for func_name in web_vibez_functions {
            assert!(registry.get_qualified_function(func_name).is_some(), 
                   "Function {} should be registered", func_name);
        }
    }
    
    #[test]
    fn test_llvm_integration_initialization() {
        let context = Context::create();
        let module = context.create_module("test_stdlib");
        
        let mut integration = StdlibLlvmIntegration::new(&context, &module);
        let result = integration.initialize_function_declarations();
        
        assert!(result.is_ok(), "Function declarations should initialize successfully");
        
        // Test that functions can be retrieved
        assert!(integration.get_function_info("len").is_some());
        assert!(integration.get_function_info("web_vibez.Get").is_some());
    }
}
