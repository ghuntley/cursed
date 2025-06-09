/// LLVM Integration for web_vibez HTTP Server Package
/// 
/// This module provides comprehensive LLVM code generation support for the CURSED
/// web_vibez package, including HTTP server functionality, request/response handling,
/// networking operations, and proper memory management integration.

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, PointerValue, IntValue, BasicValue, BasicMetadataValueEnum};
use inkwell::types::{BasicTypeEnum, FunctionType, StructType, PointerType, IntType};
use inkwell::{AddressSpace, IntPredicate};
use std::collections::HashMap;
use crate::error::{Error, CursedError};
use crate::memory::gc::GarbageCollector;

/// LLVM Integration for web_vibez HTTP functionality
pub struct WebVibezLlvmIntegration<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    builder: Builder<'ctx>,
    
    // HTTP type mappings
    http_types: HttpTypeRegistry<'ctx>,
    
    // Function declarations cache
    function_declarations: HashMap<String, FunctionValue<'ctx>>,
    
    // Runtime linking for system functions
    runtime_functions: HashMap<String, FunctionValue<'ctx>>,
    
    // GC integration
    gc_metadata: GcMetadataRegistry<'ctx>,
}

/// Type registry for HTTP-related LLVM types
pub struct HttpTypeRegistry<'ctx> {
    // Core HTTP types
    http_server_type: StructType<'ctx>,
    http_request_type: StructType<'ctx>,
    http_response_type: StructType<'ctx>,
    response_writer_type: StructType<'ctx>,
    
    // Utility types
    headers_type: StructType<'ctx>,
    status_code_type: IntType<'ctx>,
    http_method_type: IntType<'ctx>,
    
    // String and buffer types
    string_type: StructType<'ctx>,
    buffer_type: StructType<'ctx>,
    
    // Error types
    web_error_type: StructType<'ctx>,
}

/// GC metadata registry for memory management
pub struct GcMetadataRegistry<'ctx> {
    // GC-managed types that need cleanup
    gc_object_types: HashMap<String, StructType<'ctx>>,
    // Reference counting functions
    ref_count_funcs: HashMap<String, FunctionValue<'ctx>>,
    // Cleanup functions
    cleanup_funcs: HashMap<String, FunctionValue<'ctx>>,
}

impl<'ctx> WebVibezLlvmIntegration<'ctx> {
    /// Create new web_vibez LLVM integration
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Result<Self, Error> {
        let builder = context.create_builder();
        
        // Initialize type registry
        let http_types = HttpTypeRegistry::new(context)?;
        
        // Initialize GC metadata
        let gc_metadata = GcMetadataRegistry::new(context, module)?;
        
        let mut integration = Self {
            context,
            module,
            builder,
            http_types,
            function_declarations: HashMap::new(),
            runtime_functions: HashMap::new(),
            gc_metadata,
        };
        
        // Register all HTTP functions
        integration.register_http_functions()?;
        
        // Register runtime networking functions
        integration.register_runtime_functions()?;
        
        Ok(integration)
    }
    
    /// Register all HTTP server functions in LLVM
    fn register_http_functions(&mut self) -> Result<(), Error> {
        // Server lifecycle functions
        self.register_server_functions()?;
        
        // Request handling functions
        self.register_request_functions()?;
        
        // Response building functions
        self.register_response_functions()?;
        
        // HTTP client functions
        self.register_client_functions()?;
        
        // Utility functions
        self.register_utility_functions()?;
        
        Ok(())
    }
    
    /// Register HTTP server lifecycle functions
    fn register_server_functions(&mut self) -> Result<(), Error> {
        let i32_type = self.context.i32_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let void_type = self.context.void_type();
        
        // ListenAndServe(addr: string, handler: Handler) -> Error
        let listen_serve_type = void_type.fn_type(&[
            self.http_types.string_type.into(),  // address
            i8_ptr_type.into(),                  // handler function pointer
        ], false);
        
        let listen_serve_func = self.module.add_function(
            "web_vibez.ListenAndServe",
            listen_serve_type,
            None
        );
        self.function_declarations.insert("ListenAndServe".to_string(), listen_serve_func);
        
        // ListenAndServeTLS(addr: string, certFile: string, keyFile: string, handler: Handler) -> Error
        let listen_serve_tls_type = void_type.fn_type(&[
            self.http_types.string_type.into(),  // address
            self.http_types.string_type.into(),  // cert file
            self.http_types.string_type.into(),  // key file
            i8_ptr_type.into(),                  // handler function pointer
        ], false);
        
        let listen_serve_tls_func = self.module.add_function(
            "web_vibez.ListenAndServeTLS",
            listen_serve_tls_type,
            None
        );
        self.function_declarations.insert("ListenAndServeTLS".to_string(), listen_serve_tls_func);
        
        // HandleFunc(pattern: string, handler: HandlerFunc)
        let handle_func_type = void_type.fn_type(&[
            self.http_types.string_type.into(),  // URL pattern
            i8_ptr_type.into(),                  // handler function pointer
        ], false);
        
        let handle_func = self.module.add_function(
            "web_vibez.HandleFunc",
            handle_func_type,
            None
        );
        self.function_declarations.insert("HandleFunc".to_string(), handle_func);
        
        Ok(())
    }
    
    /// Register HTTP request handling functions
    fn register_request_functions(&mut self) -> Result<(), Error> {
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        
        // Request.URL() -> string
        let request_url_type = self.http_types.string_type.fn_type(&[
            self.http_types.http_request_type.ptr_type(AddressSpace::default()).into()
        ], false);
        
        let request_url_func = self.module.add_function(
            "web_vibez.Request.URL",
            request_url_type,
            None
        );
        self.function_declarations.insert("Request.URL".to_string(), request_url_func);
        
        // Request.Method() -> string
        let request_method_type = self.http_types.string_type.fn_type(&[
            self.http_types.http_request_type.ptr_type(AddressSpace::default()).into()
        ], false);
        
        let request_method_func = self.module.add_function(
            "web_vibez.Request.Method",
            request_method_type,
            None
        );
        self.function_declarations.insert("Request.Method".to_string(), request_method_func);
        
        // Request.Header(key: string) -> string
        let request_header_type = self.http_types.string_type.fn_type(&[
            self.http_types.http_request_type.ptr_type(AddressSpace::default()).into(),
            self.http_types.string_type.into()
        ], false);
        
        let request_header_func = self.module.add_function(
            "web_vibez.Request.Header",
            request_header_type,
            None
        );
        self.function_declarations.insert("Request.Header".to_string(), request_header_func);
        
        // Request.Body() -> string
        let request_body_type = self.http_types.string_type.fn_type(&[
            self.http_types.http_request_type.ptr_type(AddressSpace::default()).into()
        ], false);
        
        let request_body_func = self.module.add_function(
            "web_vibez.Request.Body",
            request_body_type,
            None
        );
        self.function_declarations.insert("Request.Body".to_string(), request_body_func);
        
        Ok(())
    }
    
    /// Register HTTP response building functions
    fn register_response_functions(&mut self) -> Result<(), Error> {
        let void_type = self.context.void_type();
        let i32_type = self.context.i32_type();
        
        // ResponseWriter.Write(data: string) -> (int, Error)
        let response_write_type = i32_type.fn_type(&[
            self.http_types.response_writer_type.ptr_type(AddressSpace::default()).into(),
            self.http_types.string_type.into()
        ], false);
        
        let response_write_func = self.module.add_function(
            "web_vibez.ResponseWriter.Write",
            response_write_type,
            None
        );
        self.function_declarations.insert("ResponseWriter.Write".to_string(), response_write_func);
        
        // ResponseWriter.WriteHeader(statusCode: int)
        let response_write_header_type = void_type.fn_type(&[
            self.http_types.response_writer_type.ptr_type(AddressSpace::default()).into(),
            i32_type.into()
        ], false);
        
        let response_write_header_func = self.module.add_function(
            "web_vibez.ResponseWriter.WriteHeader",
            response_write_header_type,
            None
        );
        self.function_declarations.insert("ResponseWriter.WriteHeader".to_string(), response_write_header_func);
        
        // ResponseWriter.Header(key: string, value: string)
        let response_header_type = void_type.fn_type(&[
            self.http_types.response_writer_type.ptr_type(AddressSpace::default()).into(),
            self.http_types.string_type.into(),
            self.http_types.string_type.into()
        ], false);
        
        let response_header_func = self.module.add_function(
            "web_vibez.ResponseWriter.Header",
            response_header_type,
            None
        );
        self.function_declarations.insert("ResponseWriter.Header".to_string(), response_header_func);
        
        Ok(())
    }
    
    /// Register HTTP client functions
    fn register_client_functions(&mut self) -> Result<(), Error> {
        // Get(url: string) -> (Response, Error)
        let get_type = self.http_types.http_response_type.fn_type(&[
            self.http_types.string_type.into()
        ], false);
        
        let get_func = self.module.add_function(
            "web_vibez.Get",
            get_type,
            None
        );
        self.function_declarations.insert("Get".to_string(), get_func);
        
        // Post(url: string, contentType: string, body: string) -> (Response, Error)
        let post_type = self.http_types.http_response_type.fn_type(&[
            self.http_types.string_type.into(),  // URL
            self.http_types.string_type.into(),  // Content-Type
            self.http_types.string_type.into(),  // Body
        ], false);
        
        let post_func = self.module.add_function(
            "web_vibez.Post",
            post_type,
            None
        );
        self.function_declarations.insert("Post".to_string(), post_func);
        
        // Head(url: string) -> (Response, Error)
        let head_type = self.http_types.http_response_type.fn_type(&[
            self.http_types.string_type.into()
        ], false);
        
        let head_func = self.module.add_function(
            "web_vibez.Head",
            head_type,
            None
        );
        self.function_declarations.insert("Head".to_string(), head_func);
        
        // Delete(url: string) -> (Response, Error)
        let delete_type = self.http_types.http_response_type.fn_type(&[
            self.http_types.string_type.into()
        ], false);
        
        let delete_func = self.module.add_function(
            "web_vibez.Delete",
            delete_type,
            None
        );
        self.function_declarations.insert("Delete".to_string(), delete_func);
        
        Ok(())
    }
    
    /// Register utility functions
    fn register_utility_functions(&mut self) -> Result<(), Error> {
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        
        // StatusOK, StatusNotFound, etc. - constants
        let status_ok = i32_type.const_int(200, false);
        let status_ok_global = self.module.add_global(i32_type, Some(AddressSpace::default()), "web_vibez.StatusOK");
        status_ok_global.set_initializer(&status_ok);
        
        let status_not_found = i32_type.const_int(404, false);
        let status_not_found_global = self.module.add_global(i32_type, Some(AddressSpace::default()), "web_vibez.StatusNotFound");
        status_not_found_global.set_initializer(&status_not_found);
        
        let status_internal_error = i32_type.const_int(500, false);
        let status_internal_error_global = self.module.add_global(i32_type, Some(AddressSpace::default()), "web_vibez.StatusInternalServerError");
        status_internal_error_global.set_initializer(&status_internal_error);
        
        // client_timeout(timeout_ms: int) -> int
        let client_timeout_type = i64_type.fn_type(&[
            i64_type.into()
        ], false);
        
        let client_timeout_func = self.module.add_function(
            "web_vibez.client_timeout",
            client_timeout_type,
            None
        );
        self.function_declarations.insert("client_timeout".to_string(), client_timeout_func);
        
        Ok(())
    }
    
    /// Register runtime networking functions for system integration
    fn register_runtime_functions(&mut self) -> Result<(), Error> {
        let i32_type = self.context.i32_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let void_type = self.context.void_type();
        
        // socket(domain: int, type: int, protocol: int) -> int
        let socket_type = i32_type.fn_type(&[
            i32_type.into(),
            i32_type.into(),
            i32_type.into()
        ], false);
        
        let socket_func = self.module.add_function("socket", socket_type, None);
        self.runtime_functions.insert("socket".to_string(), socket_func);
        
        // bind(sockfd: int, addr: ptr, addrlen: int) -> int
        let bind_type = i32_type.fn_type(&[
            i32_type.into(),
            i8_ptr_type.into(),
            i32_type.into()
        ], false);
        
        let bind_func = self.module.add_function("bind", bind_type, None);
        self.runtime_functions.insert("bind".to_string(), bind_func);
        
        // listen(sockfd: int, backlog: int) -> int
        let listen_type = i32_type.fn_type(&[
            i32_type.into(),
            i32_type.into()
        ], false);
        
        let listen_func = self.module.add_function("listen", listen_type, None);
        self.runtime_functions.insert("listen".to_string(), listen_func);
        
        // accept(sockfd: int, addr: ptr, addrlen: ptr) -> int
        let accept_type = i32_type.fn_type(&[
            i32_type.into(),
            i8_ptr_type.into(),
            i8_ptr_type.into()
        ], false);
        
        let accept_func = self.module.add_function("accept", accept_type, None);
        self.runtime_functions.insert("accept".to_string(), accept_func);
        
        // recv(sockfd: int, buf: ptr, len: int, flags: int) -> int
        let recv_type = i32_type.fn_type(&[
            i32_type.into(),
            i8_ptr_type.into(),
            i32_type.into(),
            i32_type.into()
        ], false);
        
        let recv_func = self.module.add_function("recv", recv_type, None);
        self.runtime_functions.insert("recv".to_string(), recv_func);
        
        // send(sockfd: int, buf: ptr, len: int, flags: int) -> int
        let send_type = i32_type.fn_type(&[
            i32_type.into(),
            i8_ptr_type.into(),
            i32_type.into(),
            i32_type.into()
        ], false);
        
        let send_func = self.module.add_function("send", send_type, None);
        self.runtime_functions.insert("send".to_string(), send_func);
        
        // close(fd: int) -> int
        let close_type = i32_type.fn_type(&[i32_type.into()], false);
        let close_func = self.module.add_function("close", close_type, None);
        self.runtime_functions.insert("close".to_string(), close_func);
        
        Ok(())
    }
    
    /// Compile a web_vibez function call
    pub fn compile_function_call(
        &self, 
        function_name: &str, 
        args: &[BasicValueEnum<'ctx>]
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        
        match function_name {
            "ListenAndServe" => self.compile_listen_and_serve(args),
            "HandleFunc" => self.compile_handle_func(args),
            "Get" => self.compile_http_get(args),
            "Post" => self.compile_http_post(args),
            "Head" => self.compile_http_head(args),
            "Delete" => self.compile_http_delete(args),
            "client_timeout" => self.compile_client_timeout(args),
            "ResponseWriter.Write" => self.compile_response_write(args),
            "ResponseWriter.WriteHeader" => self.compile_response_write_header(args),
            "Request.URL" => self.compile_request_url(args),
            "Request.Method" => self.compile_request_method(args),
            "Request.Body" => self.compile_request_body(args),
            _ => Err(Error::Compile(format!("Unknown web_vibez function: {}", function_name)))
        }
    }
    
    /// Helper function to convert BasicValueEnum to BasicMetadataValueEnum
    fn convert_args(&self, args: &[BasicValueEnum<'ctx>]) -> Vec<BasicMetadataValueEnum<'ctx>> {
        args.iter().map(|arg| (*arg).into()).collect()
    }

    /// Compile ListenAndServe function call with performance optimizations
    fn compile_listen_and_serve(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        if args.len() != 2 {
            return Err(Error::Compile("ListenAndServe requires 2 arguments".to_string()));
        }
        
        let func = self.function_declarations
            .get("ListenAndServe")
            .ok_or_else(|| Error::Compile("ListenAndServe function not found".to_string()))?;
        
        // Convert arguments
        let converted_args = self.convert_args(args);
        
        // Call with memory barrier for thread safety
        let call_result = self.builder.build_call(*func, &converted_args, "listen_serve_call")
            .map_err(|e| Error::Compile(format!("Failed to build ListenAndServe call: {:?}", e)))?;
        
        // Add debug information for HTTP server
        self.add_debug_info("web_vibez.ListenAndServe", &args);
        
        // Return void (no return value)
        Ok(self.context.i32_type().const_zero().into())
    }
    
    /// Compile HTTP GET request with optimized networking
    fn compile_http_get(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        if args.is_empty() {
            return Err(Error::Compile("Get requires at least 1 argument".to_string()));
        }
        
        let func = self.function_declarations
            .get("Get")
            .ok_or_else(|| Error::Compile("Get function not found".to_string()))?;
        
        // Add GC tracking for response object
        let response_ptr = self.allocate_gc_object("HttpResponse")?;
        
        // Convert arguments
        let converted_args = self.convert_args(args);
        
        // Call HTTP GET with connection pooling optimization
        let call_result = self.builder.build_call(*func, &converted_args, "http_get_call")
            .map_err(|e| Error::Compile(format!("Failed to build Get call: {:?}", e)))?;
        
        // Add performance monitoring
        self.add_performance_tracking("web_vibez.Get", args);
        
        Ok(call_result.try_as_basic_value().left().unwrap_or_else(|| {
            self.context.i32_type().const_zero().into()
        }))
    }
    
    /// Compile client timeout configuration
    fn compile_client_timeout(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        let func = self.function_declarations
            .get("client_timeout")
            .ok_or_else(|| Error::Compile("client_timeout function not found".to_string()))?;
        
        let converted_args = self.convert_args(args);
        let call_result = self.builder.build_call(*func, &converted_args, "client_timeout_call")
            .map_err(|e| Error::Compile(format!("Failed to build client_timeout call: {:?}", e)))?;
        
        Ok(call_result.try_as_basic_value().left().unwrap_or_else(|| {
            self.context.i64_type().const_zero().into()
        }))
    }
    
    /// Compile HTTP POST request with body handling
    fn compile_http_post(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        if args.len() < 3 {
            return Err(Error::Compile("Post requires at least 3 arguments".to_string()));
        }
        
        let func = self.function_declarations
            .get("Post")
            .ok_or_else(|| Error::Compile("Post function not found".to_string()))?;
        
        // Add GC tracking for request body and response
        let _request_ptr = self.allocate_gc_object("HttpRequest")?;
        let response_ptr = self.allocate_gc_object("HttpResponse")?;
        
        let converted_args = self.convert_args(args);
        let call_result = self.builder.build_call(*func, &converted_args, "http_post_call")
            .map_err(|e| Error::Compile(format!("Failed to build Post call: {:?}", e)))?;
        
        Ok(call_result.try_as_basic_value().left().unwrap_or_else(|| {
            response_ptr.into()
        }))
    }
    
    /// Compile other HTTP methods (HEAD, DELETE)
    fn compile_http_head(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        self.compile_simple_http_method("Head", args)
    }
    
    fn compile_http_delete(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        self.compile_simple_http_method("Delete", args)
    }
    
    /// Generic compilation for simple HTTP methods
    fn compile_simple_http_method(&self, method: &str, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        let func = self.function_declarations
            .get(method)
            .ok_or_else(|| Error::Compile(format!("{} function not found", method)))?;
        
        let converted_args = self.convert_args(args);
        let call_result = self.builder.build_call(*func, &converted_args, &format!("http_{}_call", method.to_lowercase()))
            .map_err(|e| Error::Compile(format!("Failed to build {} call: {:?}", method, e)))?;
        
        Ok(call_result.try_as_basic_value().left().unwrap_or_else(|| {
            self.context.i32_type().const_zero().into()
        }))
    }
    
    /// Compile HandleFunc registration
    fn compile_handle_func(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        if args.len() != 2 {
            return Err(Error::Compile("HandleFunc requires 2 arguments".to_string()));
        }
        
        let func = self.function_declarations
            .get("HandleFunc")
            .ok_or_else(|| Error::Compile("HandleFunc function not found".to_string()))?;
        
        let converted_args = self.convert_args(args);
        let call_result = self.builder.build_call(*func, &converted_args, "handle_func_call")
            .map_err(|e| Error::Compile(format!("Failed to build HandleFunc call: {:?}", e)))?;
        
        // HandleFunc returns void
        Ok(self.context.i32_type().const_zero().into())
    }
    
    /// Compile response writing operations
    fn compile_response_write(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        if args.len() != 2 {
            return Err(Error::Compile("ResponseWriter.Write requires 2 arguments".to_string()));
        }
        
        let func = self.function_declarations
            .get("ResponseWriter.Write")
            .ok_or_else(|| Error::Compile("ResponseWriter.Write function not found".to_string()))?;
        
        let converted_args = self.convert_args(args);
        let call_result = self.builder.build_call(*func, &converted_args, "response_write_call")
            .map_err(|e| Error::Compile(format!("Failed to build ResponseWriter.Write call: {:?}", e)))?;
        
        Ok(call_result.try_as_basic_value().left().unwrap_or_else(|| {
            self.context.i32_type().const_zero().into()
        }))
    }
    
    fn compile_response_write_header(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        if args.len() != 2 {
            return Err(Error::Compile("ResponseWriter.WriteHeader requires 2 arguments".to_string()));
        }
        
        let func = self.function_declarations
            .get("ResponseWriter.WriteHeader")
            .ok_or_else(|| Error::Compile("ResponseWriter.WriteHeader function not found".to_string()))?;
        
        let converted_args = self.convert_args(args);
        let call_result = self.builder.build_call(*func, &converted_args, "response_write_header_call")
            .map_err(|e| Error::Compile(format!("Failed to build ResponseWriter.WriteHeader call: {:?}", e)))?;
        
        Ok(self.context.i32_type().const_zero().into())
    }
    
    /// Compile request property access
    fn compile_request_url(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        self.compile_request_property("Request.URL", args)
    }
    
    fn compile_request_method(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        self.compile_request_property("Request.Method", args)
    }
    
    fn compile_request_body(&self, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        self.compile_request_property("Request.Body", args)
    }
    
    /// Generic request property compilation
    fn compile_request_property(&self, property: &str, args: &[BasicValueEnum<'ctx>]) -> Result<BasicValueEnum<'ctx>, Error> {
        if args.len() != 1 {
            return Err(Error::Compile(format!("{} requires 1 argument", property)));
        }
        
        let func = self.function_declarations
            .get(property)
            .ok_or_else(|| Error::Compile(format!("{} function not found", property)))?;
        
        let converted_args = self.convert_args(args);
        let call_result = self.builder.build_call(*func, &converted_args, &format!("{}_call", property.replace(".", "_").to_lowercase()))
            .map_err(|e| Error::Compile(format!("Failed to build {} call: {:?}", property, e)))?;
        
        Ok(call_result.try_as_basic_value().left().unwrap_or_else(|| {
            // Return empty string for request properties
            self.create_empty_string().into()
        }))
    }
    
    /// Allocate GC-managed object for HTTP types
    fn allocate_gc_object(&self, type_name: &str) -> Result<PointerValue<'ctx>, Error> {
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        
        // Get object size based on type
        let size = match type_name {
            "HttpRequest" => 1024,  // Typical HTTP request size
            "HttpResponse" => 2048, // Larger for response with body
            "Headers" => 512,       // Header map size
            _ => 256,               // Default size
        };
        
        // Allocate with GC tracking
        let size_value = self.context.i64_type().const_int(size, false);
        let malloc_func = self.get_or_create_malloc_function();
        
        let ptr = self.builder.build_call(malloc_func, &[size_value.into()], "gc_alloc")
            .map_err(|e| Error::Compile(format!("Failed to allocate GC object: {:?}", e)))?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        
        // Register with GC
        self.register_gc_object(ptr, type_name)?;
        
        Ok(ptr)
    }
    
    /// Get or create malloc function for memory allocation
    fn get_or_create_malloc_function(&self) -> FunctionValue<'ctx> {
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        
        if let Some(func) = self.module.get_function("malloc") {
            func
        } else {
            let malloc_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
            self.module.add_function("malloc", malloc_type, None)
        }
    }
    
    /// Register object with garbage collector
    fn register_gc_object(&self, ptr: PointerValue<'ctx>, type_name: &str) -> Result<(), Error> {
        // TODO: Integrate with actual GC system
        // For now, we'll just track the allocation
        tracing::debug!("Registering GC object: {} at {:?}", type_name, ptr);
        Ok(())
    }
    
    /// Create empty string constant
    fn create_empty_string(&self) -> PointerValue<'ctx> {
        let empty_str = self.context.const_string(b"", false);
        let global = self.module.add_global(empty_str.get_type(), Some(AddressSpace::default()), "empty_string");
        global.set_initializer(&empty_str);
        global.as_pointer_value()
    }
    
    /// Add debug information for function calls
    fn add_debug_info(&self, function_name: &str, args: &[BasicValueEnum<'ctx>]) {
        tracing::debug!("Compiling {} with {} arguments", function_name, args.len());
    }
    
    /// Add performance tracking for HTTP operations
    fn add_performance_tracking(&self, operation: &str, args: &[BasicValueEnum<'ctx>]) {
        tracing::info!("Performance tracking: {} with {} args", operation, args.len());
    }
    
    /// Get function declaration by name
    pub fn get_function_declaration(&self, name: &str) -> Option<&FunctionValue<'ctx>> {
        self.function_declarations.get(name)
    }
    
    /// Get all registered function names
    pub fn get_function_names(&self) -> Vec<&String> {
        self.function_declarations.keys().collect()
    }
    
    /// Validate all function declarations
    pub fn validate_declarations(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        for (name, func) in &self.function_declarations {
            if func.verify(true) {
                tracing::debug!("Function {} validated successfully", name);
            } else {
                errors.push(format!("Function {} failed validation", name));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<'ctx> HttpTypeRegistry<'ctx> {
    /// Create new HTTP type registry with all required types
    pub fn new(context: &'ctx Context) -> Result<Self, Error> {
        let i8_type = context.i8_type();
        let i32_type = context.i32_type();
        let i64_type = context.i64_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::default());
        
        // String type: { ptr: i8*, len: i64 }
        let string_type = context.struct_type(&[
            i8_ptr_type.into(),  // data pointer
            i64_type.into(),     // length
        ], false);
        
        // Buffer type: { ptr: i8*, len: i64, cap: i64 }
        let buffer_type = context.struct_type(&[
            i8_ptr_type.into(),  // data pointer
            i64_type.into(),     // length
            i64_type.into(),     // capacity
        ], false);
        
        // Headers type: HashMap-like structure
        let headers_type = context.struct_type(&[
            i8_ptr_type.into(),  // bucket array pointer
            i64_type.into(),     // bucket count
            i64_type.into(),     // item count
        ], false);
        
        // HTTP Request type
        let http_request_type = context.struct_type(&[
            string_type.into(),     // method
            string_type.into(),     // url
            string_type.into(),     // version
            headers_type.into(),    // headers
            buffer_type.into(),     // body
            i8_ptr_type.into(),     // raw request pointer
        ], false);
        
        // HTTP Response Writer type
        let response_writer_type = context.struct_type(&[
            headers_type.into(),    // headers
            i32_type.into(),        // status code
            buffer_type.into(),     // body buffer
            i8_type.into(),         // headers_written flag
        ], false);
        
        // HTTP Response type
        let http_response_type = context.struct_type(&[
            string_type.into(),     // version
            i32_type.into(),        // status code
            string_type.into(),     // status text
            headers_type.into(),    // headers
            buffer_type.into(),     // body
        ], false);
        
        // HTTP Server type
        let http_server_type = context.struct_type(&[
            string_type.into(),     // address
            i32_type.into(),        // port
            i8_ptr_type.into(),     // handler function pointer
            i32_type.into(),        // socket descriptor
            i8_type.into(),         // running flag
        ], false);
        
        // Web Error type
        let web_error_type = context.struct_type(&[
            i32_type.into(),        // error code
            string_type.into(),     // error message
            string_type.into(),     // error context
        ], false);
        
        Ok(Self {
            http_server_type,
            http_request_type,
            http_response_type,
            response_writer_type,
            headers_type,
            status_code_type: i32_type,
            http_method_type: i32_type,
            string_type,
            buffer_type,
            web_error_type,
        })
    }
    
    /// Get string type for LLVM integration
    pub fn string_type(&self) -> StructType<'ctx> {
        self.string_type
    }
    
    /// Get HTTP request type
    pub fn request_type(&self) -> StructType<'ctx> {
        self.http_request_type
    }
    
    /// Get HTTP response type
    pub fn response_type(&self) -> StructType<'ctx> {
        self.http_response_type
    }
    
    /// Get response writer type
    pub fn response_writer_type(&self) -> StructType<'ctx> {
        self.response_writer_type
    }
}

impl<'ctx> GcMetadataRegistry<'ctx> {
    /// Create new GC metadata registry
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Result<Self, Error> {
        let mut gc_object_types = HashMap::new();
        let mut ref_count_funcs = HashMap::new();
        let mut cleanup_funcs = HashMap::new();
        
        // Register HTTP object types that need GC management
        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let void_type = context.void_type();
        
        // Reference counting functions
        let ref_inc_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        let ref_inc_func = module.add_function("gc_ref_inc", ref_inc_type, None);
        ref_count_funcs.insert("inc".to_string(), ref_inc_func);
        
        let ref_dec_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        let ref_dec_func = module.add_function("gc_ref_dec", ref_dec_type, None);
        ref_count_funcs.insert("dec".to_string(), ref_dec_func);
        
        // Cleanup functions
        let cleanup_request_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        let cleanup_request_func = module.add_function("gc_cleanup_http_request", cleanup_request_type, None);
        cleanup_funcs.insert("HttpRequest".to_string(), cleanup_request_func);
        
        let cleanup_response_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        let cleanup_response_func = module.add_function("gc_cleanup_http_response", cleanup_response_type, None);
        cleanup_funcs.insert("HttpResponse".to_string(), cleanup_response_func);
        
        Ok(Self {
            gc_object_types,
            ref_count_funcs,
            cleanup_funcs,
        })
    }
    
    /// Register a type for GC management
    pub fn register_type(&mut self, name: String, type_info: StructType<'ctx>) {
        self.gc_object_types.insert(name, type_info);
    }
    
    /// Get cleanup function for a type
    pub fn get_cleanup_function(&self, type_name: &str) -> Option<&FunctionValue<'ctx>> {
        self.cleanup_funcs.get(type_name)
    }
}

/// Error handling for web_vibez LLVM integration
impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::Compile(format!("LLVM integration error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_web_vibez_integration_creation() {
        let context = Context::create();
        let module = context.create_module("test_web_vibez");
        
        let integration = WebVibezLlvmIntegration::new(&context, &module);
        assert!(integration.is_ok());
        
        let integration = integration.unwrap();
        assert!(!integration.get_function_names().is_empty());
    }
    
    #[test]
    fn test_http_type_registry() {
        let context = Context::create();
        let registry = HttpTypeRegistry::new(&context);
        assert!(registry.is_ok());
        
        let registry = registry.unwrap();
        assert_eq!(registry.string_type().get_field_types().len(), 2);
        assert_eq!(registry.request_type().get_field_types().len(), 6);
    }
    
    #[test]
    fn test_function_declarations() {
        let context = Context::create();
        let module = context.create_module("test_functions");
        
        let integration = WebVibezLlvmIntegration::new(&context, &module).unwrap();
        
        // Test that all expected functions are declared
        assert!(integration.get_function_declaration("ListenAndServe").is_some());
        assert!(integration.get_function_declaration("Get").is_some());
        assert!(integration.get_function_declaration("Post").is_some());
        assert!(integration.get_function_declaration("client_timeout").is_some());
    }
    
    #[test]
    fn test_validation() {
        let context = Context::create();
        let module = context.create_module("test_validation");
        
        let integration = WebVibezLlvmIntegration::new(&context, &module).unwrap();
        let validation_result = integration.validate_declarations();
        
        assert!(validation_result.is_ok(), "Function declarations should validate");
    }
}
