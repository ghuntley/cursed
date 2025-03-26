use crate::compiler::{Bytecode, CompiledFunction, Instructions, Opcode};
use crate::object::Object;
use crate::error::{Error, ErrorReporter};
use crate::memory::{MemoryManager};
use crate::memory::gc::{Traceable, Visitor};
use crate::memory::tagged::Tag;
use crate::prelude::{VecExt, StringExt};
use std::fmt;
use std::rc::Rc;
use std::alloc::Layout;
use std::ptr::NonNull;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::ops::{Deref, RangeBounds};
use std::iter::Iterator;
use std::vec::Drain;
use std::slice::Iter;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::parser_impl::Parser;
use crate::compiler::Compiler;
use std::convert::TryFrom;

#[cfg(test)]
pub mod tests;

pub mod constants;

/// Stack size for the VM
const STACK_SIZE: usize = 2048;
/// Global store size
const GLOBALS_SIZE: usize = 65536;
/// Max number of frames for function calls
const MAX_FRAMES: usize = 1024;
/// Default memory size for the VM
const DEFAULT_MEMORY_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Default memory sizes
const DEFAULT_BUMP_ALLOCATOR_SIZE: usize = 1024 * 1024;    // 1MB
const DEFAULT_BLOCK_ALLOCATOR_SIZE: usize = 2 * 1024 * 1024; // 2MB
const DEFAULT_GC_SIZE: usize = 7 * 1024 * 1024;             // 7MB

/// Minimum alignment for allocations
const MIN_ALIGNMENT: usize = 8;

/// Frame for function calls
#[derive(Debug, Clone)]
pub struct Frame {
    /// The compiled function being executed
    pub function: Rc<CompiledFunction>,
    /// The instruction pointer within the function
    pub ip: usize,
    /// The base pointer for local variables
    pub base_pointer: usize,
    /// The instructions for this frame
    pub instructions: Vec<u8>,
    /// Optional closure for this frame
    pub closure: Option<Rc<Closure>>,
    /// Optional try handler IP for this frame
    pub try_handler_ip: Option<usize>,
    /// Optional switch value for case statements
    pub switch_value: Option<Object>,
}

impl Frame {
    /// Create a new function call frame
    ///
    /// # Arguments
    ///
    /// * `function` - The compiled function to execute
    /// * `base_pointer` - The base pointer for local variables
    ///
    /// # Returns
    ///
    /// A new Frame instance
    pub fn new(function: Rc<CompiledFunction>, base_pointer: usize) -> Self {
        Self {
            function: function.clone(),
            ip: 0,
            base_pointer,
            instructions: function.instructions.clone(),
            closure: None,
            try_handler_ip: None,
            switch_value: None,
        }
    }
    
    /// Create a new frame with a closure
    ///
    /// # Arguments
    ///
    /// * `function` - The compiled function to execute in this frame
    /// * `free_variables` - The free variables captured by the closure
    /// * `base_pointer` - The base pointer for local variables
    ///
    /// # Returns
    ///
    /// A new Frame instance with a closure
    pub fn new_with_closure(
        function: Rc<CompiledFunction>, 
        free_variables: Vec<Object>, 
        base_pointer: usize
    ) -> Self {
        // Create a closure
        let closure = Rc::new(Closure {
            function: function.clone(),
            free_variables,
        });
        
        Self {
            function: function.clone(),
            ip: 0,
            base_pointer,
            instructions: function.instructions.clone(),
            closure: Some(closure),
            try_handler_ip: None,
            switch_value: None,
        }
    }
    
    /// Get the current function
    pub fn function(&self) -> &Rc<CompiledFunction> {
        &self.function
    }
    
    /// Get the current closure if any
    pub fn closure(&self) -> Option<&Rc<Closure>> {
        self.closure.as_ref()
    }
    
    /// Get a reference to the traceable parts of this frame
    pub fn as_traceable(&self) -> Option<NonNull<dyn Traceable>> {
        // Convert the function reference to a traceable pointer
        if let Some(closure) = &self.closure {
            // Use the as_ref method to get a pointer to the Traceable trait
            let t: &dyn Traceable = closure.as_ref();
            // Create a non-null pointer from the trait object reference
            Some(unsafe { NonNull::new_unchecked(t as *const dyn Traceable as *mut dyn Traceable) })
        } else {
            // Use the function as a traceable
            let t: &dyn Traceable = self.function.as_ref();
            Some(unsafe { NonNull::new_unchecked(t as *const dyn Traceable as *mut dyn Traceable) })
        }
    }
}

/// Closure object for the VM
#[derive(Debug, Clone)]
pub struct Closure {
    /// The function being called
    pub function: Rc<CompiledFunction>,
    /// Free variables captured by the closure
    pub free_variables: Vec<Object>,
}

/// Class object for the VM
#[derive(Debug, Clone)]
pub struct Class {
    /// The name of the class
    pub name: String,
    /// Methods defined in the class
    pub methods: HashMap<String, Rc<CompiledFunction>>,
    /// Optional superclass
    pub superclass: Option<Rc<Class>>,
}

impl Class {
    /// Add a method to the class
    pub fn add_method(&mut self, method: Rc<CompiledFunction>) {
        if let Some(name) = &method.name {
            self.methods.insert(name.clone(), method);
        }
    }
    
    /// Get a method by name
    pub fn get_method(&self, name: &str) -> Option<Rc<CompiledFunction>> {
        if let Some(method) = self.methods.get(name) {
            Some(Rc::clone(method))
        } else if let Some(superclass) = &self.superclass {
            superclass.get_method(name)
        } else {
            None
        }
    }
    
    /// Set the superclass
    pub fn inherit_from(&mut self, superclass: Rc<Class>) {
        self.superclass = Some(superclass);
    }
}

/// Instance object for the VM
#[derive(Debug, Clone)]
pub struct Instance {
    /// The class this instance belongs to
    pub class: Rc<Class>,
    /// Instance fields
    pub fields: HashMap<String, Object>,
}

impl Instance {
    /// Get a field by name
    pub fn get_field(&self, name: &str) -> Option<Object> {
        self.fields.get(name).cloned()
    }
    
    /// Set a field by name
    pub fn set_field(&mut self, name: String, value: Object) {
        self.fields.insert(name, value);
    }
    
    /// Get a method by name
    pub fn get_method(&self, name: &str) -> Option<Rc<CompiledFunction>> {
        self.class.get_method(name)
    }
    
    /// Get a method from the superclass
    pub fn get_super_method(&self, name: &str) -> Option<Rc<CompiledFunction>> {
        if let Some(superclass) = &self.class.superclass {
            superclass.get_method(name)
        } else {
            None
        }
    }
}

/// Error location information
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorLocation {
    /// Line number where the error occurred
    pub line: usize,
    /// Column number where the error occurred
    pub column: usize,
    /// Source code line where the error occurred
    pub source_line: String,
}

impl ErrorLocation {
    /// Create a new error location
    ///
    /// # Arguments
    ///
    /// * `line` - The line number where the error occurred
    /// * `column` - The column number where the error occurred
    /// * `source_line` - The source code line where the error occurred
    ///
    /// # Returns
    ///
    /// A new ErrorLocation instance
    pub fn new(line: usize, column: usize, source_line: String) -> Self {
        Self {
            line,
            column,
            source_line,
        }
    }
    
    /// Calculate the size of the error location in bytes
    pub fn size(&self) -> usize {
        std::mem::size_of::<usize>() * 2 + // line and column
        std::mem::size_of::<String>() +    // String struct size
        self.source_line.capacity()        // Actual string capacity
    }
}

/// The virtual machine that executes the bytecode
pub struct VM {
    /// Constants from the bytecode
    pub constants: Vec<Object>,
    /// Instructions from the bytecode
    pub instructions: Instructions,
    
    /// The stack for storing values
    pub stack: Vec<Object>,
    /// The stack pointer (points to the next available slot)
    pub sp: usize,
    
    /// Call frames for function execution
    pub frames: Vec<Frame>,
    /// Current frame index
    pub frame_index: usize,
    
    /// Global variables store
    pub globals: HashMap<String, Object>,
    
    /// Memory manager for custom allocation
    pub memory: MemoryManager,
    
    /// The last popped value from the stack
    pub last_popped: Object,
    
    /// Whether the VM has halted
    pub halted: bool,
}

/// Constants for VM configuration
impl VM {
    /// Maximum number of frames for the call stack
    pub const MAX_FRAMES: usize = 1024;
    
    /// Default stack size
    pub const STACK_SIZE: usize = 2048;
    
    /// Default size for global variables
    pub const GLOBALS_SIZE: usize = 65536;
    
    /// Default memory size for the memory manager (8MB)
    pub const DEFAULT_MEMORY_SIZE: usize = 8 * 1024 * 1024;
    
    /// Maximum stack size for the VM
    pub const MAX_STACK_SIZE: usize = 2048;
    /// Default heap size for the VM (512KB)
    pub const DEFAULT_HEAP_SIZE: usize = 512 * 1024;
    /// Default GC size for the VM (256KB)
    pub const DEFAULT_GC_SIZE: usize = 256 * 1024;
}

impl VM {
    fn pop(&mut self) -> Result<Object, Error> {
        if self.sp > 0 {
            self.sp -= 1;
            let obj = self.stack[self.sp].clone();
            self.last_popped = obj.clone();
            Ok(obj)
        } else {
            Err(Error::Runtime("Stack underflow".to_string()))
        }
    }
    
    fn push_frame(&mut self, frame: Frame) -> Result<(), Error> {
        if self.frame_index >= MAX_FRAMES - 1 {
            return Err(Error::Runtime("Stack overflow: too many function calls".to_string()));
        }
        
        self.frames.push(frame);
        self.frame_index += 1;
        
        Ok(())
    }
    
    /// Create a new VM with the given bytecode
    pub fn new(bytecode: Bytecode) -> Self {
        let mut vm = Self {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: Vec::with_capacity(STACK_SIZE),
            sp: 0,
            globals: Vec::with_capacity(GLOBALS_SIZE),
            frames: Vec::with_capacity(MAX_FRAMES),
            frame_index: 0,
            last_popped: None,
            halted: false,
        };
        
        // Initialize the first frame
        let main_fn = CompiledFunction::new(vm.instructions.clone());
        let main_closure = Closure::new(main_fn, Vec::new());
        let main_frame = Frame::new(main_closure, 0);
        vm.frames.push(main_frame);
        
        vm
    }
    
    /// Get a reference to the current frame
    pub fn current_frame(&self) -> &Frame {
        &self.frames[self.frame_index - 1]
    }
    
    /// Get a mutable reference to the current frame
    pub fn current_frame_mut(&mut self) -> &mut Frame {
        &mut self.frames[self.frame_index - 1]
    }
    
    /// Push an object onto the stack
    pub fn push(&mut self, obj: Object) -> Result<(), Error> {
        if self.sp >= STACK_SIZE {
            return Err(Error::StackOverflow("Stack overflow during push".to_string()));
        }
        
        self.stack[self.sp] = obj;
        self.sp += 1;
        Ok(())
    }
    
    /// Pop an object from the stack
    pub fn pop(&mut self) -> Result<Object, Error> {
        if self.sp == 0 {
            return Err(Error::Runtime("Stack underflow".to_string()));
        }
        self.sp -= 1;
        let value = self.stack[self.sp].clone();
        self.last_popped = value.clone();
        Ok(value)
    }
    
    /// Read a uint8 from the current instruction
    pub fn read_uint8(&mut self) -> Result<u8, Error> {
        let frame = self.current_frame();
        let ip = frame.ip;
        
        if ip >= frame.instructions.len() {
            return Err(Error::Runtime("Instruction pointer out of range".to_string()));
        }
        
        let result = frame.instructions[ip];
        self.current_frame().ip += 1;
        
        Ok(result)
    }
    
    /// Read a 16-bit unsigned integer from the current instruction
    pub fn read_uint16(&mut self) -> Result<usize, Error> {
        let frame = self.current_frame();
        let ip = frame.ip;
        
        // Check that we have enough bytes to read a uint16
        if ip + 1 >= frame.instructions.len() {
            return Err(Error::Runtime("Unexpected end of bytecode".to_string()));
        }
        
        // Read the high and low bytes
        let high = frame.instructions[ip] as usize;
        let low = frame.instructions[ip + 1] as usize;
        
        // Increment the instruction pointer
        self.current_frame_mut().ip += 2;
        
        // Combine the high and low bytes
        Ok((high << 8) | low)
    }
    
    /// Execute a call instruction
    pub fn execute_call(&mut self, num_args: usize) -> Result<(), Error> {
        let callee = self.pop()?;
        
        match callee {
            Object::Closure(closure) => {
                // Check that the argument count matches
                if num_args != closure.function.num_parameters {
                    return Err(Error::Runtime(format!(
                        "Wrong number of arguments: expected {}, got {}",
                        closure.function.num_parameters, num_args
                    )));
                }
                
                // Create a new frame for the function
                let frame = Frame::new_with_closure(closure.function, closure.free_variables, self.sp - num_args);
                
                // Push the frame
                self.push_frame(frame)?;
                
                // Make room for local variables
                self.sp += closure.function.num_locals;
                
                Ok(())
            },
            Object::Builtin(builtin) => {
                // Get arguments from the stack
                let mut args = Vec::with_capacity(num_args);
                for _ in 0..num_args {
                    args.push(self.pop()?);
                }
                args.reverse();
                
                // Call the builtin function
                let result = builtin(args)?;
                self.push(result);
                
                Ok(())
            },
            _ => Err(Error::Runtime(format!("Calling non-function: {:?}", callee))),
        }
    }
    
    /// Execute a BuildArray instruction
    pub fn execute_build_array(&mut self, num_elements: usize) -> Result<(), Error> {
        if self.stack.len() < num_elements {
            return Err(Error::Runtime(format!(
                "Not enough elements on stack to build array: need {}, have {}",
                num_elements, self.stack.len()
            )));
        }
        
        // Get the index to start collecting elements from
        let start_index = self.stack.len() - num_elements;
        
        // Collect the elements
        let mut array_elements = Vec::with_capacity(num_elements);
        for _ in 0..num_elements {
            array_elements.push(self.pop()?);
        }
        
        // Reverse the array since we popped in reverse order
        array_elements.reverse();
        
        // Create and push the array
        self.push(Object::Array(array_elements))?;
        
        Ok(())
    }
    
    /// Execute a build hash operation
    pub fn execute_build_hash(&mut self, num_pairs: usize) -> Result<(), Error> {
        if self.stack.len() < num_pairs * 2 {
            return Err(Error::Runtime(format!(
                "Not enough elements on stack to build hash: need {}, have {}",
                num_pairs * 2, self.stack.len()
            )));
        }
        
        // Create a new hash table
        let mut hash = std::collections::HashMap::new();
        
        // Pop key-value pairs from the stack
        for _ in 0..num_pairs {
            // Pop in reverse order: value, then key
            let value = self.pop()?;
            let key = self.pop()?;
            
            // Ensure key is hashable
            if !key.is_hashable() {
                return Err(Error::Runtime(format!("Unhashable type: {}", key)));
            }
            
            // Convert key to string for the hash key
            let key_str = match key {
                Object::String(s) => s,
                Object::Integer(i) => i.to_string(),
                Object::Boolean(b) => b.to_string(),
                _ => return Err(Error::Runtime(format!("Unhashable type: {}", key))),
            };
            
            // Insert the key-value pair
            hash.insert(key_str, value);
        }
        
        // Create and push the hash table
        self.push(Object::HashTable(hash))?;
        
        Ok(())
    }
    
    /// Execute an index operation (array[index] or hash[key])
    pub fn execute_index_operation(&mut self) -> Result<(), Error> {
        // Pop the index and the object being indexed
        let index = self.pop()?;
        let obj = self.pop()?;

        match obj {
            Object::Array(array) => {
                // Check if the index is an integer
                if let Object::Integer(i) = index {
                    // Check if the index is in range
                    if i < 0 || i as usize >= array.len() {
                        return Err(Error::Runtime(format!("Index out of range: {}", i)));
                    }
                    
                    // Get the value at the index
                    let value = array[i as usize].clone();
                    self.push(value)?;
                } else {
                    return Err(Error::Runtime(format!("Array index must be an integer, got {}", index)));
                }
            },
            Object::HashTable(hash) => {
                // Check if the key is hashable
                if !index.is_hashable() {
                    return Err(Error::Runtime(format!("Unhashable type: {}", index)));
                }
                
                // Convert the index to a string for the hash key
                let key = match index {
                    Object::String(s) => s,
                    Object::Integer(i) => i.to_string(),
                    Object::Boolean(b) => b.to_string(),
                    _ => return Err(Error::Runtime(format!("Unhashable type: {}", index))),
                };
                
                // Get the value from the hash
                let value = hash.get(&key).cloned().unwrap_or(Object::Null);
                self.push(value)?;
            },
            _ => {
                return Err(Error::Runtime(format!("Cannot index into type: {}", obj)));
            }
        }
        
        Ok(())
    }
    
    /// Push a closure onto the stack
    pub fn push_closure(&mut self, const_index: usize, num_free: usize) -> Result<(), Error> {
        let constant = self.constants.get(const_index)
            .ok_or_else(|| Error::Runtime("Invalid constant index".to_string()))?;

        let function = match constant {
            Object::CompiledFunction(f) => Rc::clone(f),
            _ => return Err(Error::Runtime("Expected compiled function".to_string())),
        };

        let mut free_variables = Vec::with_capacity(num_free);
        for i in 0..num_free {
            let value = self.pop()?;
            free_variables.push(value);
        }
        free_variables.reverse();

        let closure = Rc::new(Closure {
            function,
            free_variables,
        });

        self.push(Object::Closure { function, free_vars: free_variables })
    }
    
    /// Collect garbage
    pub fn collect_garbage(&mut self) -> Result<(), Error> {
        // Implementation using our custom ObjectCollector
        let mut visitor = ObjectCollector::new();
        
        // Mark phase
        for global in &self.globals {
            if !global.is_null() {
                if let Some(ptr) = global.as_traceable() {
                    visitor.visit_object(ptr.as_ptr() as usize);
                }
            }
        }
        
        // Mark stack
        for i in 0..self.sp {
            if let Some(obj) = self.stack.get(i) {
                if !obj.is_null() {
                    if let Some(ptr) = obj.as_traceable() {
                        visitor.visit_object(ptr.as_ptr() as usize);
                    }
                }
            }
        }
        
        // Mark frames
        for frame in &self.frames {
            // Mark function
            if let Some(ptr) = frame.function.as_traceable() {
                visitor.visit_object(ptr.as_ptr() as usize);
            }
            
            // Mark closure if present
            if let Some(closure) = &frame.closure {
                if let Some(ptr) = closure.as_traceable() {
                    visitor.visit_object(ptr.as_ptr() as usize);
                }
            }
        }
        
        // Sweep phase
        self.memory.sweep(&visitor.marked)?;
        
        Ok(())
    }
    
    /// Get the last executed value (at the top of the stack)
    pub fn last_popped_value(&self) -> Option<Object> {
        if self.sp > 0 {
            Some(self.stack[self.sp - 1].clone())
        } else {
            None
        }
    }
    
    /// Get the current stack as a vector
    pub fn stack_values(&self) -> Vec<Object> {
        self.stack[0..self.sp].to_vec()
    }
    
    /// Execute a get field operation
    pub fn execute_get_field(&mut self) -> Result<(), Error> {
        // Read the field name index from the constants
        let const_index = self.read_uint16()? as usize;
        
        // Get the field name from the constants
        let field_name = match &self.constants[const_index] {
            Object::String(name) => name.clone(),
            _ => return Err(Error::Runtime("Invalid field name in constants".to_string())),
        };
        
        // Get the object from the stack
        let obj = self.pop()?;
        
        // Check if the object is an instance
        match &obj {
            Object::Instance { struct_type, fields } => {
                // Get the field from the instance
                if let Some(value) = fields.get(&field_name) {
                    // Push the field value onto the stack
                    self.push(value.clone())?;
                } else {
                    // Field not found - push null
                    self.push(Object::Null)?;
                }
            },
            _ => {
                return Err(Error::Runtime(format!(
                    "Cannot get field '{}' from non-instance object", field_name
                )));
            }
        }
        
        Ok(())
    }
    
    /// Execute a set field operation
    pub fn execute_set_field(&mut self) -> Result<(), Error> {
        // Read the field name index from the constants
        let const_index = self.read_uint16()? as usize;
        
        // Get the field name from the constants
        let field_name = match &self.constants[const_index] {
            Object::String(name) => name.clone(),
            _ => return Err(Error::Runtime("Invalid field name in constants".to_string())),
        };
        
        // Get the value to set from the stack
        let value = self.pop()?;
        
        // Get the object from the stack
        let mut obj = self.pop()?;
        
        // Check if the object is an instance
        match &mut obj {
            Object::Instance { fields, .. } => {
                // Set the field in the instance
                fields.insert(field_name, value.clone());
                
                // Push the value back onto the stack (for assignment expressions)
                self.push(value)?;
            },
            _ => {
                return Err(Error::Runtime(format!(
                    "Cannot set field '{}' on non-instance object", field_name
                )));
            }
        }
        
        Ok(())
    }
    
    /// Execute struct instantiation
    pub fn execute_instantiate_struct(&mut self) -> Result<(), Error> {
        // Read the struct name index from the constants
        let const_index = self.read_uint16()? as usize;
        
        // Get the struct definition from the constants
        let struct_def = match &self.constants[const_index] {
            Object::Struct { name, fields } => {
                Object::Struct {
                    name: name.clone(),
                    fields: fields.clone(),
                }
            },
            _ => return Err(Error::Runtime("Invalid struct definition in constants".to_string())),
        };
        
        // Create a new instance with empty fields
        let instance = Object::Instance {
            struct_type: Rc::new(struct_def),
            fields: HashMap::new(),
        };
        
        // Push the instance onto the stack
        self.push(instance)?;
        
        Ok(())
    }
    
    /// Run the VM until it halts or encounters an error
    pub fn run(&mut self) -> Result<Object, Error> {
        while !self.halted && self.current_frame().ip() < self.current_frame().instructions().len() {
            let op = self.fetch_opcode();
            match op {
                Opcode::Constant => self.execute_constant()?,
                Opcode::Add => self.execute_binary_operation(|a, b| a + b)?,
                Opcode::Sub => self.execute_binary_operation(|a, b| a - b)?,
                Opcode::Mul => self.execute_binary_operation(|a, b| a * b)?,
                Opcode::Div => self.execute_binary_operation(|a, b| a / b)?,
                Opcode::Pop => { self.pop()?; },
                Opcode::True => self.push(Object::Boolean(true))?,
                Opcode::False => self.push(Object::Boolean(false))?,
                Opcode::Equal => self.execute_comparison(|a, b| a == b)?,
                Opcode::NotEqual => self.execute_comparison(|a, b| a != b)?,
                Opcode::GreaterThan => self.execute_comparison(|a, b| a > b)?,
                Opcode::Minus => self.execute_minus()?,
                Opcode::Bang => self.execute_bang_operator()?,
                Opcode::Jump => self.execute_jump()?,
                Opcode::JumpNotTruthy => self.execute_jump_not_truthy()?,
                Opcode::Null => self.push(Object::Null)?,
                Opcode::GetGlobal => self.execute_get_global()?,
                Opcode::SetGlobal => self.execute_set_global()?,
                Opcode::Array => self.execute_array()?,
                Opcode::Hash => self.execute_hash()?,
                Opcode::Index => self.execute_index()?,
                Opcode::Call => self.execute_call()?,
                Opcode::ReturnValue => self.execute_return_value()?,
                Opcode::Return => self.execute_return()?,
                Opcode::GetLocal => self.execute_get_local()?,
                Opcode::SetLocal => self.execute_set_local()?,
                Opcode::GetBuiltin => self.execute_get_builtin()?,
                Opcode::Closure => self.execute_closure()?,
                Opcode::GetFree => self.execute_get_free()?,
                Opcode::CurrentClosure => self.execute_current_closure()?,
                _ => return Err(Error::UnknownOpcode(op)),
            }
        }
        
        Ok(self.last_popped.clone().unwrap_or(Object::Null))
    }
    
    /// Handle runtime errors in try blocks
    pub fn handle_runtime_error(&mut self, error: Error) -> Result<(), Error> {
        // Check if we're in a try block
        if let Some(catch_ip) = self.current_frame().try_handler_ip {
            // Create an error object
            let error_obj = self.create_error(
                match &error {
                    Error::Runtime(msg) => msg.clone(),
                    _ => format!("{}", error),
                },
                Some("RuntimeError".to_string())
            );
            
            // Push the error object onto the stack
            self.push(error_obj);
            
            // Jump to the catch block
            self.current_frame_mut().ip = catch_ip;
            
            Ok(())
        } else {
            // No try block, propagate the error
            Err(error)
        }
    }
    
    /// Create an error object with stack trace
    pub fn create_error(&mut self, message: String, error_type: Option<String>) -> Object {
        // Collect stack trace from frames
        let mut stack_trace = Vec::new();
        
        // Add all frames to stack trace
        for frame_idx in (0..=self.frame_index).rev() {
            let caller_frame = &self.frames[frame_idx];
            
            // We don't have actual source locations, so create a basic one
            // In a real implementation, we would get this from a source map
            let function_name = caller_frame.function.name.clone()
                .unwrap_or_else(|| "anonymous".to_string());
            
            stack_trace.push(ErrorLocation::new(
                0, // Line not available
                0, // Column not available
                format!("In function: {}", function_name),
            ));
        }
        
        // Create error object
        Object::Error {
            message,
            error_type,
            stack_trace,
        }
    }
    
    /// Execute a binary operation (add, subtract, multiply, divide)
    pub fn execute_binary_operation(&mut self, op: Opcode) -> Result<(), Error> {
        // Get operands
        let right = self.pop().ok_or_else(|| Error::Runtime("Stack underflow".to_string()))?;
        let left = self.pop().ok_or_else(|| Error::Runtime("Stack underflow".to_string()))?;
        
        match op {
            Opcode::Add => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => {
                        let result = Object::Integer(l + r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Float(r)) => {
                        let result = Object::Float(l + r);
                        self.push(result);
                    },
                    (Object::Integer(l), Object::Float(r)) => {
                        let result = Object::Float(*l as f64 + r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Integer(r)) => {
                        let result = Object::Float(l + *r as f64);
                        self.push(result);
                    },
                    (Object::String(l), Object::String(r)) => {
                        let result = Object::String(l.to_string() + r);
                        self.push(result);
                    },
                    (Object::String(l), _) => {
                        let result = Object::String(l.to_string() + &right.to_string());
                        self.push(result);
                    },
                    (_, Object::String(r)) => {
                        let result = Object::String(left.to_string() + r);
                        self.push(result);
                    },
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for +: {} and {}", left, right
                    ))),
                }
            },
            Opcode::Sub => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => {
                        let result = Object::Integer(l - r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Float(r)) => {
                        let result = Object::Float(l - r);
                        self.push(result);
                    },
                    (Object::Integer(l), Object::Float(r)) => {
                        let result = Object::Float(*l as f64 - r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Integer(r)) => {
                        let result = Object::Float(l - *r as f64);
                        self.push(result);
                    },
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for -: {} and {}", left, right
                    ))),
                }
            },
            Opcode::Mul => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => {
                        let result = Object::Integer(l * r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Float(r)) => {
                        let result = Object::Float(l * r);
                        self.push(result);
                    },
                    (Object::Integer(l), Object::Float(r)) => {
                        let result = Object::Float(*l as f64 * r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Integer(r)) => {
                        let result = Object::Float(l * *r as f64);
                        self.push(result);
                    },
                    (Object::String(s), Object::Integer(n)) if *n >= 0 => {
                        let result = Object::String(s.repeat(*n as usize));
                        self.push(result);
                    },
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for *: {} and {}", left, right
                    ))),
                }
            },
            Opcode::Div => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => {
                        if *r == 0 {
                            return Err(Error::Runtime("Division by zero".to_string()));
                        }
                        let result = Object::Integer(l / r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Float(r)) => {
                        if *r == 0.0 {
                            return Err(Error::Runtime("Division by zero".to_string()));
                        }
                        let result = Object::Float(l / r);
                        self.push(result);
                    },
                    (Object::Integer(l), Object::Float(r)) => {
                        if *r == 0.0 {
                            return Err(Error::Runtime("Division by zero".to_string()));
                        }
                        let result = Object::Float(*l as f64 / r);
                        self.push(result);
                    },
                    (Object::Float(l), Object::Integer(r)) => {
                        if *r == 0 {
                            return Err(Error::Runtime("Division by zero".to_string()));
                        }
                        let result = Object::Float(l / *r as f64);
                        self.push(result);
                    },
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for /: {} and {}", left, right
                    ))),
                }
            },
            Opcode::Modulo => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => {
                        if *r == 0 {
                            return Err(Error::Runtime("Modulo by zero".to_string()));
                        }
                        let result = Object::Integer(l % r);
                        self.push(result);
                    },
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for %: {} and {}", left, right
                    ))),
                }
            },
            _ => return Err(Error::Runtime(format!(
                "Unknown binary operator: {:?}", op
            ))),
        }
        
        Ok(())
    }
    
    /// Execute a comparison operation (equals, not equals, greater than, etc.)
    pub fn execute_comparison(&mut self, op: Opcode) -> Result<(), Error> {
        // Get operands
        let right = self.pop().ok_or_else(|| Error::Runtime("Stack underflow".to_string()))?;
        let left = self.pop().ok_or_else(|| Error::Runtime("Stack underflow".to_string()))?;
        
        let result = match op {
            Opcode::Equal => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => l == r,
                    (Object::Float(l), Object::Float(r)) => l == r,
                    (Object::Boolean(l), Object::Boolean(r)) => l == r,
                    (Object::String(l), Object::String(r)) => l == r,
                    (Object::Null, Object::Null) => true,
                    _ => false,
                }
            },
            Opcode::NotEqual => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => l != r,
                    (Object::Float(l), Object::Float(r)) => l != r,
                    (Object::Boolean(l), Object::Boolean(r)) => l != r,
                    (Object::String(l), Object::String(r)) => l != r,
                    (Object::Null, Object::Null) => false,
                    _ => true,
                }
            },
            Opcode::GreaterThan => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => l > r,
                    (Object::Float(l), Object::Float(r)) => l > r,
                    (Object::Integer(l), Object::Float(r)) => (*l as f64) > *r,
                    (Object::Float(l), Object::Integer(r)) => *l > (*r as f64),
                    (Object::String(l), Object::String(r)) => l > r,
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for >: {} and {}", left, right
                    ))),
                }
            },
            Opcode::GreaterThanEqual => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => l >= r,
                    (Object::Float(l), Object::Float(r)) => l >= r,
                    (Object::Integer(l), Object::Float(r)) => (*l as f64) >= *r,
                    (Object::Float(l), Object::Integer(r)) => *l >= (*r as f64),
                    (Object::String(l), Object::String(r)) => l >= r,
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for >=: {} and {}", left, right
                    ))),
                }
            },
            Opcode::LessThan => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => l < r,
                    (Object::Float(l), Object::Float(r)) => l < r,
                    (Object::Integer(l), Object::Float(r)) => (*l as f64) < *r,
                    (Object::Float(l), Object::Integer(r)) => *l < (*r as f64),
                    (Object::String(l), Object::String(r)) => l < r,
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for <: {} and {}", left, right
                    ))),
                }
            },
            Opcode::LessThanEqual => {
                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => l <= r,
                    (Object::Float(l), Object::Float(r)) => l <= r,
                    (Object::Integer(l), Object::Float(r)) => (*l as f64) <= *r,
                    (Object::Float(l), Object::Integer(r)) => *l <= (*r as f64),
                    (Object::String(l), Object::String(r)) => l <= r,
                    _ => return Err(Error::Runtime(format!(
                        "Unsupported types for <=: {} and {}", left, right
                    ))),
                }
            },
            _ => return Err(Error::Runtime(format!(
                "Unknown comparison operator: {:?}", op
            ))),
        };
        
        self.push(Object::Boolean(result));
        
        Ok(())
    }
    
    /// Execute a prefix operation
    pub fn execute_prefix_operation(&mut self, op: Opcode) -> Result<(), Error> {
        // Pop operand from the stack
        let operand = self.pop().ok_or_else(|| Error::Runtime("Stack underflow".to_string()))?;
        
        // Apply prefix operator
        match op {
            Opcode::Minus => {
                match operand {
                    Object::Integer(val) => self.push(Object::Integer(-val)),
                    Object::Float(val) => self.push(Object::Float(-val)),
                    _ => return Err(Error::Runtime(format!(
                        "Prefix operator - not supported for type {}", operand
                    ))),
                }
            },
            Opcode::Bang => {
                match operand {
                    Object::Boolean(val) => self.push(Object::Boolean(!val)),
                    _ => self.push(Object::Boolean(false)), // Anything else is falsey
                }
            },
            _ => return Err(Error::Runtime(format!(
                "Unknown prefix operator: {:?}", op
            ))),
        }
        
        Ok(())
    }

    /// Pop a frame from the call stack
    pub fn pop_frame(&mut self) -> Frame {
        if self.frame_index == 0 {
            panic!("Cannot pop the main frame");
        }
        
        let frame = self.frames.pop().unwrap();
        self.frame_index -= 1;
        
        frame
    }

    /// Compare two values for equality
    pub fn values_equal(&self, left: &Object, right: &Object) -> bool {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => l == r,
            (Object::Float(l), Object::Float(r)) => (l - r).abs() < std::f64::EPSILON,
            (Object::Boolean(l), Object::Boolean(r)) => l == r,
            (Object::String(l), Object::String(r)) => l == r,
            (Object::Null, Object::Null) => true,
            // Classes are equal if they have the same name
            (Object::Class(l), Object::Class(r)) => l.name == r.name,
            // Instances are equal if they are the same reference
            (Object::Instance(l), Object::Instance(r)) => Rc::ptr_eq(l, r),
            // Arrays are equal if they have the same elements
            (Object::Array(l), Object::Array(r)) => {
                if l.len() != r.len() {
                    return false;
                }
                for (l_item, r_item) in l.iter().zip(r.iter()) {
                    if !self.values_equal(l_item, r_item) {
                        return false;
                    }
                }
                true
            },
            // All other combinations are not equal
            _ => false,
        }
    }

    /// Get the current memory usage
    pub fn memory_usage(&self) -> usize {
        self.sp * std::mem::size_of::<Object>()
    }

    /// Get the current stack pointer
    pub fn stack_pointer(&self) -> usize {
        self.sp
    }

    /// Reset the VM
    pub fn reset(&mut self) {
        self.stack.clear();
        self.frames.clear();
        self.current_frame = None;
        self.last_popped = None;
        self.halted = false;
    }
}

impl Object {
    pub fn is_null(&self) -> bool {
        match self {
            Object::Null => true,
            _ => false,
        }
    }
    
    pub fn is_hashable(&self) -> bool {
        match self {
            Object::Integer(_) => true,
            Object::String(_) => true,
            Object::Boolean(_) => true,
            _ => false,
        }
    }
    
    pub fn as_traceable(&self) -> Option<NonNull<dyn Traceable>> {
        match self {
            // For each reference type, convert it to a trait object pointer
            Object::Array(arr) => {
                // Get the raw pointer to the Vec<Object>
                let ptr = arr as *const _ as *mut Vec<Object>;
                // Create a trait object pointer
                unsafe {
                    // We know this pointer is not null because we just got it from a reference
                    Some(NonNull::new_unchecked(ptr as *mut dyn Traceable))
                }
            },
            Object::HashTable(hash) => {
                // Get the raw pointer to the HashMap<String, Object>
                let ptr = hash as *const _ as *mut HashMap<String, Object>;
                // Create a trait object pointer
                unsafe {
                    Some(NonNull::new_unchecked(ptr as *mut dyn Traceable))
                }
            },
            Object::CompiledFunction(function) => {
                // Verify that CompiledFunction implements Traceable
                let ptr = function as *const _ as *mut CompiledFunction;
                unsafe {
                    Some(NonNull::new_unchecked(ptr as *mut dyn Traceable))
                }
            },
            Object::Struct { .. } => {
                // Structs themselves can't be traced
                None
            },
            Object::Instance { struct_type, .. } => {
                // Get the raw pointer to the Rc<Object>
                let ptr = struct_type as *const _ as *mut Rc<Object>;
                unsafe {
                    Some(NonNull::new_unchecked(ptr as *mut dyn Traceable))
                }
            },
            // For non-reference types, return None
            _ => None,
        }
    }
    
    pub fn is_struct(&self) -> bool {
        match self {
            Object::Struct { .. } => true,
            _ => false,
        }
    }
    
    pub fn is_instance(&self) -> bool {
        match self {
            Object::Instance { .. } => true,
            _ => false,
        }
    }
    
    // Get the field of an instance
    pub fn get_field(&self, field_name: &str) -> Option<Object> {
        match self {
            Object::Instance { fields, .. } => {
                fields.get(field_name).cloned()
            },
            _ => None,
        }
    }
    
    // Set a field of an instance
    pub fn set_field(&mut self, field_name: &str, value: Object) -> Result<(), Error> {
        match self {
            Object::Instance { fields, .. } => {
                fields.insert(field_name.to_string(), value);
                Ok(())
            },
            _ => Err(Error::Runtime("Cannot set field on non-instance object".to_string())),
        }
    }
}

// Custom visitor for garbage collection
struct ObjectCollector {
    // Objects pending processing 
    pending: Vec<usize>,
    // Objects that have been marked
    marked: std::collections::HashSet<usize>,
}

impl ObjectCollector {
    fn new() -> Self {
        ObjectCollector {
            pending: Vec::new(),
            marked: std::collections::HashSet::new(),
        }
    }
    
    fn visit_object(&mut self, ptr: usize) {
        // Don't process objects that have already been marked
        if self.marked.insert(ptr) {
            // Only add to pending if it's a new object
            self.pending.push(ptr);
        }
    }
    
    // Process a batch of objects
    fn process_batch<F>(&mut self, mut process_fn: F) -> usize
    where
        F: FnMut(usize)
    {
        let count = self.pending.len();
        
        // Process all pending objects
        for ptr in self.pending.drain(..) {
            process_fn(ptr);
        }
        
        count
    }
    
    // Process all pending objects until there are none left
    fn process_pending<F>(&mut self, mut process_fn: F) -> usize
    where
        F: FnMut(usize)
    {
        let mut total_processed = 0;
        
        // Process objects in batches until there are no more pending objects
        while self.has_pending() {
            total_processed += self.process_batch(&mut process_fn);
        }
        
        total_processed
    }
    
    // Check if there are pending objects
    fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }
    
    // Get the number of pending objects
    fn pending_count(&self) -> usize {
        self.pending.len()
    }
    
    // Get the number of marked objects
    fn marked_count(&self) -> usize {
        self.marked.len()
    }
}