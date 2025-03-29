// Virtual Machine implementation for CURSED
use crate::compiler::{Bytecode, Instructions, Opcode, CompiledFunction};
use crate::error::{Error, SourceLocation};
use crate::object::Object;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;

pub mod constants {
    // VM constants
    pub const STACK_SIZE: usize = 2048;
    pub const MAX_FRAMES: usize = 1024;
    pub const GLOBALS_SIZE: usize = 65536;
    pub const DEFAULT_MEMORY_SIZE: usize = 4 * 1024 * 1024; // 4MB
    pub const HEAP_SIZE: usize = 2 * 1024 * 1024; // 2MB
    pub const GC_SIZE: usize = 1024 * 1024; // 1MB
}

/// Represents a call frame on the VM's call stack
pub struct Frame {
    pub instructions: Instructions,
    pub ip: usize, // Instruction pointer
    pub base_pointer: usize,
    pub free_vars: Vec<Rc<Object>>,
}

impl Frame {
    /// Create a new call frame
    pub fn new(instructions: Instructions, base_pointer: usize) -> Self {
        Frame {
            instructions,
            ip: 0,
            base_pointer,
            free_vars: Vec::new(),
        }
    }
}

/// A closure in the CURSED language
pub struct Closure {
    pub function: Rc<Object>,
    pub free_vars: Vec<Rc<Object>>,
}

/// Represents a location where an error occurred during VM execution
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorLocation {
    pub ip: usize,
    pub frame_index: usize,
}

/// The CURSED Virtual Machine
pub struct VM {
    constants: Vec<Rc<Object>>,
    stack: Vec<Rc<Object>>,
    globals: Vec<Rc<Object>>,
    frames: Vec<Frame>,
    frame_index: usize,
    sp: usize, // Stack pointer
    builtins: Vec<Rc<Object>>, // Registry of builtin functions
}

impl VM {
    /// Create a new VM with the given constants and instructions
    pub fn new() -> Self {
        // Initialize empty globals array with capacity
        let mut globals = Vec::with_capacity(constants::GLOBALS_SIZE);
        for _ in 0..constants::GLOBALS_SIZE {
            globals.push(Rc::new(Object::Null));
        }
        
        // Create a new VM
        let mut vm = VM {
            constants: Vec::new(),
            stack: Vec::with_capacity(constants::STACK_SIZE),
            globals,
            frames: Vec::with_capacity(constants::MAX_FRAMES),
            frame_index: 0,
            sp: 0,
            builtins: Vec::new(),
        };
        
        // Initialize with an empty frame
        vm.frames.push(Frame::new(Vec::new(), 0));
        
        // Register built-in functions
        vm.register_builtins();
        
        vm
    }
    
    /// Create a new VM with the given bytecode
    pub fn with_bytecode(bytecode: Bytecode) -> Self {
        let mut vm = Self::new();
        vm.constants = bytecode.constants.into_iter().map(|obj| Rc::new(obj)).collect();
        
        // Create the main frame with the bytecode instructions
        let main_frame = Frame::new(bytecode.instructions, 0);
        vm.frames[0] = main_frame;
        vm.frame_index = 0;
        
        vm
    }
    
    /// Create a new VM with the given bytecode and global state (for testing)
    pub fn with_bytecode_and_state(&mut self, bytecode: Bytecode, globals: Vec<Rc<Object>>) {
        // Create the main frame with the bytecode instructions
        let main_frame = Frame::new(bytecode.instructions, 0);
        self.frames[0] = main_frame;
        
        // Add the constants
        self.constants = bytecode.constants.into_iter().map(Rc::new).collect();
        
        // Set the globals
        if !globals.is_empty() {
            self.globals = globals;
        }
    }
    
    /// Run a VM with the given bytecode
    pub fn run_with_bytecode(&mut self, bytecode: Bytecode) -> Result<Rc<Object>, Error> {
        self.with_bytecode_and_state(bytecode, Vec::new());
        self.run()
    }
    
    /// Register all built-in functions
    fn register_builtins(&mut self) {
        // Register each builtin function
        self.register_builtin("len", builtin_len);
        self.register_builtin("first", builtin_first);
        self.register_builtin("last", builtin_last);
        self.register_builtin("rest", builtin_rest);
        self.register_builtin("push", builtin_push);
        self.register_builtin("puts", builtin_puts);
        self.register_builtin("type", builtin_type);
        self.register_builtin("is_integer", builtin_is_integer);
        self.register_builtin("is_string", builtin_is_string);
        self.register_builtin("is_array", builtin_is_array);
        self.register_builtin("is_hash", builtin_is_hash);
        self.register_builtin("is_null", builtin_is_null);
        
        // Register vibez module functions
        self.register_builtin("vibez_spill", builtin_vibez_spill);
        self.register_builtin("vibez.spill", builtin_vibez_spill);
    }
    
    /// Register a built-in function
    fn register_builtin(&mut self, name: &str, function: crate::object::BuiltinFunction) {
        self.builtins.push(Rc::new(Object::Builtin {
            name: name.to_string(),
            function,
        }));
    }
    
    /// Push an object onto the stack
    pub fn push(&mut self, obj: Rc<Object>) -> Result<(), Error> {
        if self.sp >= constants::STACK_SIZE {
            return Err(Error::stack_overflow("Stack overflow".to_string()));
        }
        
        // Add object to stack and increment stack pointer
        if self.sp >= self.stack.len() {
            self.stack.push(obj);
        } else {
            self.stack[self.sp] = obj;
        }
        self.sp += 1;
        
        Ok(())
    }
    
    /// Pop an object from the stack
    pub fn pop(&mut self) -> Result<Rc<Object>, Error> {
        if self.sp == 0 {
            return Err(Error::vm("Stack underflow".to_string()));
        }
        
        self.sp -= 1;
        Ok(self.stack[self.sp].clone())
    }
    
    /// Peek at the top object on the stack without popping it
    pub fn peek(&self) -> Result<Rc<Object>, Error> {
        if self.sp == 0 {
            return Err(Error::vm("Stack empty".to_string()));
        }
        
        Ok(self.stack[self.sp - 1].clone())
    }
    
    /// Peek at an object at a specific distance from the top of the stack
    pub fn peek_at(&self, distance: usize) -> Result<Rc<Object>, Error> {
        let position = self.sp.checked_sub(1 + distance);
        
        match position {
            Some(pos) if pos < self.stack.len() => Ok(self.stack[pos].clone()),
            _ => Err(Error::vm(format!("Cannot peek at position {} from stack top", distance))),
        }
    }
    
    /// Get the current frame
    pub fn current_frame(&self) -> Result<&Frame, Error> {
        self.frames.get(self.frame_index).ok_or_else(|| {
            Error::vm("No frame available".to_string())
        })
    }
    
    /// Get the current frame as mutable
    pub fn current_frame_mut(&mut self) -> Result<&mut Frame, Error> {
        self.frames.get_mut(self.frame_index).ok_or_else(|| {
            Error::vm("No frame available".to_string())
        })
    }
    
    /// Read a u16 operand from the current frame at the given offset
    fn read_u16(&self, offset: usize) -> Result<u16, Error> {
        let frame = self.current_frame()?;
        
        if offset >= frame.instructions.len() - 1 {
            return Err(Error::vm(format!("Invalid operand offset: {}", offset)));
        }
        
        let hi = frame.instructions[offset] as u16;
        let lo = frame.instructions[offset + 1] as u16;
        
        Ok((hi << 8) | lo)
    }
    
    /// Run the VM
    pub fn run(&mut self) -> Result<Rc<Object>, Error> {
        while self.current_frame_mut()?.ip < self.current_frame()?.instructions.len() {
            // Get the current instruction
            let opcode = self.current_opcode()?;
            
            // Increment the instruction pointer
            self.current_frame_mut()?.ip += 1;
            
            match opcode {
                Opcode::Constant => {
                    let const_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip the operand bytes
                    
                    let constant = self.constants.get(const_idx).ok_or_else(|| {
                        Error::vm(format!("Invalid constant index: {}", const_idx))
                    })?.clone();
                    
                    self.push(constant)?;
                },
                Opcode::Add => {
                    self.execute_binary_operation(opcode)?;
                },
                Opcode::Sub => {
                    self.execute_binary_operation(opcode)?;
                },
                Opcode::Mul => {
                    self.execute_binary_operation(opcode)?;
                },
                Opcode::Div => {
                    self.execute_binary_operation(opcode)?;
                },
                Opcode::Modulo => {
                    self.execute_binary_operation(opcode)?;
                },
                Opcode::True => {
                    self.push(Rc::new(Object::Boolean(true)))?;
                },
                Opcode::False => {
                    self.push(Rc::new(Object::Boolean(false)))?;
                },
                Opcode::Equal => {
                    self.execute_comparison(opcode)?;
                },
                Opcode::NotEqual => {
                    self.execute_comparison(opcode)?;
                },
                Opcode::GreaterThan => {
                    self.execute_comparison(opcode)?;
                },
                Opcode::GreaterThanEqual => {
                    self.execute_comparison(opcode)?;
                },
                Opcode::LessThan => {
                    self.execute_comparison(opcode)?;
                },
                Opcode::LessThanEqual => {
                    self.execute_comparison(opcode)?;
                },
                Opcode::Minus => {
                    let right = self.pop()?;
                    if let Object::Integer(value) = *right {
                        self.push(Rc::new(Object::Integer(-value)))?;
                    } else if let Object::Float(value) = *right {
                        self.push(Rc::new(Object::Float(-value)))?;
                    } else {
                        return Err(Error::vm(format!("Cannot negate non-numeric value: {}", right)));
                    }
                },
                Opcode::Bang => {
                    let right = self.pop()?;
                    let bool_value = right.is_truthy();
                    self.push(Rc::new(Object::Boolean(!bool_value)))?;
                },
                Opcode::JumpNotTruthy => {
                    let pos = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    let condition = self.pop()?;
                    if !condition.is_truthy() {
                        self.current_frame_mut()?.ip = pos;
                    }
                },
                Opcode::Jump => {
                    let pos = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip = pos;
                },
                Opcode::Null => {
                    self.push(Rc::new(Object::Null))?;
                },
                Opcode::SetGlobal => {
                    let global_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    let value = self.pop()?;
                    if global_idx >= self.globals.len() {
                        // Resize globals if needed
                        while global_idx >= self.globals.len() {
                            self.globals.push(Rc::new(Object::Null));
                        }
                    }
                    self.globals[global_idx] = value;
                },
                Opcode::GetGlobal => {
                    let global_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    if global_idx >= self.globals.len() {
                        return Err(Error::vm(format!("Invalid global index: {}", global_idx)));
                    }
                    
                    let value = self.globals[global_idx].clone();
                    self.push(value)?;
                },
                Opcode::SetLocal => {
                    let local_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    let frame = self.current_frame()?;
                    let base_pointer = frame.base_pointer;
                    
                    // Store the value at the position: base_pointer + local_idx
                    let stack_position = base_pointer + local_idx;
                    if stack_position >= self.stack.len() {
                        return Err(Error::vm(format!("Invalid local index: {}", local_idx)));
                    }
                    
                    let value = self.pop()?;
                    self.stack[stack_position] = value;
                },
                Opcode::GetLocal => {
                    let local_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    let frame = self.current_frame()?;
                    let base_pointer = frame.base_pointer;
                    
                    // Get the value at position: base_pointer + local_idx
                    let stack_position = base_pointer + local_idx;
                    if stack_position >= self.stack.len() {
                        return Err(Error::vm(format!("Invalid local index: {}", local_idx)));
                    }
                    
                    let value = self.stack[stack_position].clone();
                    self.push(value)?;
                },
                Opcode::Call => {
                    let num_args = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    // Call function with arguments
                    self.execute_call(num_args)?;
                },
                Opcode::VariadicCall => {
                    let num_args = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    // Call variadic function with arguments
                    self.execute_call(num_args)?;
                },
                Opcode::ReturnValue => {
                    // Get the return value from the top of the stack
                    let return_value = self.pop()?;
                    
                    // Get the frame we're returning from
                    let frame = self.frames.get(self.frame_index).ok_or_else(|| {
                        Error::vm("No frame to return from".to_string())
                    })?;
                    
                    // Reset stack to previous position (before the function call)
                    self.sp = frame.base_pointer;
                    
                    // Remove the current frame
                    self.frame_index -= 1;
                    
                    // Push the return value onto the stack
                    self.push(return_value)?;
                },
                Opcode::Return => {
                    // Get the frame we're returning from
                    let frame = self.frames.get(self.frame_index).ok_or_else(|| {
                        Error::vm("No frame to return from".to_string())
                    })?;
                    
                    // Reset stack to previous position (before the function call)
                    self.sp = frame.base_pointer;
                    
                    // Remove the current frame
                    self.frame_index -= 1;
                    
                    // Push null as the return value (void return)
                    self.push(Rc::new(Object::Null))?;
                },
                Opcode::Pop => {
                    self.pop()?;
                },
                Opcode::Array => {
                    let num_elements = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    self.build_array(num_elements)?;
                },
                Opcode::Hash => {
                    let num_pairs = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    self.build_hash(num_pairs)?;
                },
                Opcode::Index => {
                    let index = self.pop()?;
                    let left = self.pop()?;
                    
                    self.execute_index_operation(left, index)?;
                },
                Opcode::Dup => {
                    // Duplicate the top stack value
                    if self.sp == 0 {
                        return Err(Error::vm("Cannot duplicate from empty stack".to_string()));
                    }
                    
                    let value = self.stack[self.sp - 1].clone();
                    self.push(value)?;
                },
                Opcode::DefineType => {
                    let num_fields = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    // Get the type name from the stack
                    let type_name = self.pop()?;
                    let name = match &*type_name {
                        Object::String(s) => s.clone(),
                        _ => return Err(Error::vm(format!("Type name must be a string, got {}", type_name))),
                    };
                    
                    // Create an empty struct object
                    let struct_obj = Rc::new(Object::Struct {
                        name,
                        fields: Vec::with_capacity(num_fields),
                    });
                    
                    // Push the struct object back onto the stack
                    self.push(struct_obj)?;
                },
                Opcode::DefineField => {
                    // Get field type and name from the stack
                    let field_type = self.pop()?;
                    let field_name = self.pop()?;
                    
                    // Get the struct object from the stack (don't pop it yet as we need to put it back)
                    let struct_obj = self.pop()?;
                    
                    // Get the field name and type as strings
                    let name = match &*field_name {
                        Object::String(s) => s.clone(),
                        _ => return Err(Error::vm(format!("Field name must be a string, got {}", field_name))),
                    };
                    
                    let typ = match &*field_type {
                        Object::String(s) => s.clone(),
                        _ => return Err(Error::vm(format!("Field type must be a string, got {}", field_type))),
                    };
                    
                    // Add the field to the struct definition
                    match &*struct_obj {
                        Object::Struct { name: struct_name, fields } => {
                            // We need to create a new struct object with the updated fields
                            let mut new_fields = fields.clone();
                            new_fields.push((name, typ));
                            
                            let updated_struct = Rc::new(Object::Struct {
                                name: struct_name.clone(),
                                fields: new_fields,
                            });
                            
                            // Push the updated struct back onto the stack
                            self.push(updated_struct)?;
                        },
                        _ => return Err(Error::vm(format!("Cannot define field on non-struct object: {}", struct_obj))),
                    }
                },
                Opcode::Closure => {
                    let const_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip const index bytes
                    
                    let num_free_vars = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip num free vars bytes
                    
                    let closure = self.build_closure(const_idx, num_free_vars)?;
                    self.push(closure)?;
                },
                Opcode::GetFree => {
                    let free_var_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    let frame = self.current_frame()?;
                    let free_var = frame.free_vars.get(free_var_idx).ok_or_else(|| {
                        Error::vm(format!("Invalid free variable index: {}", free_var_idx))
                    })?.clone();
                    
                    self.push(free_var)?;
                },
                Opcode::GetBuiltin => {
                    let builtin_idx = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    if builtin_idx >= self.builtins.len() {
                        return Err(Error::vm(format!("Invalid builtin index: {}", builtin_idx)));
                    }
                    
                    let builtin = self.builtins[builtin_idx].clone();
                    self.push(builtin)?;
                },
                Opcode::DefineInterface => {
                    let num_methods = self.read_u16(self.current_frame()?.ip)? as usize;
                    self.current_frame_mut()?.ip += 2; // Skip operand bytes
                    
                    // Get the interface name from the stack
                    let interface_name = self.pop()?;
                    let name = match &*interface_name {
                        Object::String(s) => s.clone(),
                        _ => return Err(Error::vm(format!("Interface name must be a string, got {}", interface_name))),
                    };
                    
                    // Create an empty interface object
                    let interface_obj = Rc::new(Object::Interface {
                        name,
                        methods: Vec::with_capacity(num_methods),
                    });
                    
                    // Push the interface object back onto the stack
                    self.push(interface_obj)?;
                },
                
                Opcode::DefineMethod => {
                    // Add debug print for the stack
                    println!("DEBUG VM: Stack before DefineMethod:");
                    for i in (0..self.sp).rev() {
                        println!("  {} ({}): {:?}", self.sp - i - 1, i, self.stack[i]);
                    }
                    
                    // Reverse order of popping operations to match compiler output
                    
                    // Get return type from the stack
                    let return_type = self.pop()?;
                    println!("DEBUG VM: Popped return type: {:?}", return_type);
                    
                    // Extract return type (if any)
                    let return_type_str = match &*return_type {
                        Object::String(s) => Some(s.clone()),
                        Object::Null => None,
                        _ => return Err(Error::vm(format!("Return type must be a string or null, got {}", return_type))),
                    };
                    
                    // Get the parameters from the stack
                    // We need to know the parameter count before we can start popping parameters
                    
                    // Method parameters are loaded onto the stack in this order by the compiler:
                    // [interface, method_name, param_count, param1_name, param1_type, ..., return_type]
                    
                    // Let's pop all the way to the parameter count first
                    let params_on_stack = (self.sp - 3) / 2; // Calculate how many params are on stack
                    println!("DEBUG VM: Estimated parameters on stack: {}", params_on_stack);
                    
                    // Initialize parameters vec
                    let mut parameters = Vec::new();
                    
                    // Get each parameter (name and type)
                    for _ in 0..params_on_stack {
                        // Get parameter type and name from the stack
                        let param_type = self.pop()?;
                        let param_name = self.pop()?;
                        
                        // Get the parameter name and type as strings
                        let name = match &*param_name {
                            Object::String(s) => s.clone(),
                            _ => return Err(Error::vm(format!("Parameter name must be a string, got {}", param_name))),
                        };
                        
                        let typ = match &*param_type {
                            Object::String(s) => s.clone(),
                            _ => return Err(Error::vm(format!("Parameter type must be a string, got {}", param_type))),
                        };
                        
                        // Add parameter to the list (at beginning since we're popping in reverse)
                        parameters.insert(0, (name, typ));
                    }
                    
                    // Get the parameter count as verification
                    let param_count_obj = self.pop()?;
                    println!("DEBUG VM: Popped param count: {:?}", param_count_obj);
                    
                    let expected_param_count = match &*param_count_obj {
                        Object::Integer(n) => *n as usize,
                        _ => return Err(Error::vm(format!("Parameter count must be an integer, got {}", param_count_obj))),
                    };
                    
                    // Verify param count matches what we found
                    if parameters.len() != expected_param_count {
                        return Err(Error::vm(format!("Parameter count mismatch: expected {}, got {}", expected_param_count, parameters.len())));
                    }
                    
                    // Get method name from the stack
                    let method_name_obj = self.pop()?;
                    println!("DEBUG VM: Popped method name: {:?}", method_name_obj);
                    
                    let method_name = match &*method_name_obj {
                        Object::String(s) => s.clone(),
                        _ => return Err(Error::vm(format!("Method name must be a string, got {}", method_name_obj))),
                    };
                    
                    // Get the interface object from the stack
                    let interface_obj = self.pop()?;
                    
                    // Add the method to the interface definition
                    match &*interface_obj {
                        Object::Interface { name: interface_name, methods } => {
                            // We need to create a new interface object with the updated methods
                            let mut new_methods = methods.clone();
                            new_methods.push((method_name, parameters, return_type_str));
                            
                            let updated_interface = Rc::new(Object::Interface {
                                name: interface_name.clone(),
                                methods: new_methods,
                            });
                            
                            // Push the updated interface back onto the stack
                            self.push(updated_interface)?;
                        },
                        _ => return Err(Error::vm(format!("Cannot define method on non-interface object: {}", interface_obj))),
                    }
                },
                Opcode::Method => {
                    // Add debug print for the stack
                    println!("DEBUG VM: Stack before Method opcode:");
                    for i in (0..self.sp).rev() {
                        println!("  {} ({}): {:?}", self.sp - i - 1, i, self.stack[i]);
                    }
                    
                    // Get return type from the stack
                    let return_type = self.pop()?;
                    println!("DEBUG VM: Popped return type: {:?}", return_type);
                    
                    // Extract return type (if any)
                    let return_type_str = match &*return_type {
                        Object::String(s) => Some(s.clone()),
                        Object::Null => None,
                        _ => return Err(Error::vm(format!("Return type must be a string or null, got {}", return_type))),
                    };
                    
                    // Similar to DefineMethod, get parameters from the stack
                    let mut parameters = Vec::new();
                    let params_on_stack = (self.sp - 3) / 2; // Calculate how many params are on stack
                    
                    // Get each parameter (name and type)
                    for _ in 0..params_on_stack {
                        // Get parameter type and name from the stack
                        let param_type = self.pop()?;
                        let param_name = self.pop()?;
                        
                        // Get the parameter name and type as strings
                        let name = match &*param_name {
                            Object::String(s) => s.clone(),
                            _ => return Err(Error::vm(format!("Parameter name must be a string, got {}", param_name))),
                        };
                        
                        let typ = match &*param_type {
                            Object::String(s) => s.clone(),
                            _ => return Err(Error::vm(format!("Parameter type must be a string, got {}", param_type))),
                        };
                        
                        // Add parameter to the list (at beginning since we're popping in reverse)
                        parameters.insert(0, (name, typ));
                    }
                    
                    // Get the parameter count as verification
                    let param_count_obj = self.pop()?;
                    println!("DEBUG VM: Popped param count: {:?}", param_count_obj);
                    
                    let expected_param_count = match &*param_count_obj {
                        Object::Integer(n) => *n as usize,
                        _ => return Err(Error::vm(format!("Parameter count must be an integer, got {}", param_count_obj))),
                    };
                    
                    // Verify param count matches what we found
                    if parameters.len() != expected_param_count {
                        return Err(Error::vm(format!("Parameter count mismatch: expected {}, got {}", expected_param_count, parameters.len())));
                    }
                    
                    // Get compiled function from the stack
                    let function_obj = self.pop()?;
                    println!("DEBUG VM: Popped function: {:?}", function_obj);
                    
                    let function = match &*function_obj {
                        Object::CompiledFunction { instructions, num_locals, num_parameters, free_variables: _, name, is_variadic } => {
                            // Create a new CompiledFunction to store in the closure
                            let func = CompiledFunction {
                                instructions: instructions.clone(),
                                num_locals: *num_locals as u8,
                                num_parameters: *num_parameters as u8,
                                free_variables: Vec::new(),
                                name: name.clone(),
                                is_variadic: *is_variadic,
                            };
                            Rc::new(func)
                        },
                        _ => return Err(Error::vm(format!("Expected compiled function, got {}", function_obj))),
                    };
                    
                    // Get method name from the stack
                    let method_name_obj = self.pop()?;
                    println!("DEBUG VM: Popped method name: {:?}", method_name_obj);
                    
                    let method_name = match &*method_name_obj {
                        Object::String(s) => s.clone(),
                        _ => return Err(Error::vm(format!("Method name must be a string, got {}", method_name_obj))),
                    };
                    
                    // Get the receiver type from the stack
                    let receiver_type_obj = self.pop()?;
                    println!("DEBUG VM: Popped receiver type: {:?}", receiver_type_obj);
                    
                    let receiver_type = match &*receiver_type_obj {
                        Object::String(s) => s.clone(),
                        _ => return Err(Error::vm(format!("Receiver type must be a string, got {}", receiver_type_obj))),
                    };
                    
                    // Create a Method object
                    let method_obj = Rc::new(Object::Method {
                        receiver_type,
                        name: method_name,
                        parameters,
                        return_type: return_type_str,
                        function,
                    });
                    
                    // Push the Method object onto the stack
                    self.push(method_obj)?;
                },
                _ => {
                    return Err(Error::vm(format!("Opcode not implemented: {:?}", opcode)));
                }
            }
        }
        
        // Return the top item on the stack or null if stack is empty
        if self.sp > 0 {
            Ok(self.pop()?)
        } else {
            Ok(Rc::new(Object::Null))
        }
    }
    
    /// Execute a binary arithmetic operation
    fn execute_binary_operation(&mut self, opcode: Opcode) -> Result<(), Error> {
        let right = self.pop()?;
        let left = self.pop()?;
        
        match (&*left, &*right) {
            (Object::Integer(left_val), Object::Integer(right_val)) => {
                let result = match opcode {
                    Opcode::Add => Object::Integer(left_val + right_val),
                    Opcode::Sub => Object::Integer(left_val - right_val),
                    Opcode::Mul => Object::Integer(left_val * right_val),
                    Opcode::Div => {
                        if *right_val == 0 {
                            return Err(Error::division_by_zero("Division by zero".to_string()));
                        }
                        Object::Integer(left_val / right_val)
                    },
                    Opcode::Modulo => {
                        if *right_val == 0 {
                            return Err(Error::division_by_zero("Modulo by zero".to_string()));
                        }
                        Object::Integer(left_val % right_val)
                    },
                    _ => return Err(Error::vm(format!("Unknown integer operation: {:?}", opcode))),
                };
                self.push(Rc::new(result))?;
            },
            (Object::Float(left_val), Object::Float(right_val)) => {
                let result = match opcode {
                    Opcode::Add => Object::Float(left_val + right_val),
                    Opcode::Sub => Object::Float(left_val - right_val),
                    Opcode::Mul => Object::Float(left_val * right_val),
                    Opcode::Div => {
                        if *right_val == 0.0 {
                            return Err(Error::division_by_zero("Division by zero".to_string()));
                        }
                        Object::Float(left_val / right_val)
                    },
                    Opcode::Modulo => {
                        if *right_val == 0.0 {
                            return Err(Error::division_by_zero("Modulo by zero".to_string()));
                        }
                        Object::Float(left_val % right_val)
                    },
                    _ => return Err(Error::vm(format!("Unknown float operation: {:?}", opcode))),
                };
                self.push(Rc::new(result))?;
            },
            (Object::Integer(left_val), Object::Float(right_val)) => {
                let left_val = *left_val as f64;
                let result = match opcode {
                    Opcode::Add => Object::Float(left_val + right_val),
                    Opcode::Sub => Object::Float(left_val - right_val),
                    Opcode::Mul => Object::Float(left_val * right_val),
                    Opcode::Div => {
                        if *right_val == 0.0 {
                            return Err(Error::division_by_zero("Division by zero".to_string()));
                        }
                        Object::Float(left_val / right_val)
                    },
                    Opcode::Modulo => {
                        if *right_val == 0.0 {
                            return Err(Error::division_by_zero("Modulo by zero".to_string()));
                        }
                        Object::Float(left_val % right_val)
                    },
                    _ => return Err(Error::vm(format!("Unknown mixed operation: {:?}", opcode))),
                };
                self.push(Rc::new(result))?;
            },
            (Object::Float(left_val), Object::Integer(right_val)) => {
                let right_val = *right_val as f64;
                let result = match opcode {
                    Opcode::Add => Object::Float(left_val + right_val),
                    Opcode::Sub => Object::Float(left_val - right_val),
                    Opcode::Mul => Object::Float(left_val * right_val),
                    Opcode::Div => {
                        if right_val == 0.0 {
                            return Err(Error::division_by_zero("Division by zero".to_string()));
                        }
                        Object::Float(left_val / right_val)
                    },
                    Opcode::Modulo => {
                        if right_val == 0.0 {
                            return Err(Error::division_by_zero("Modulo by zero".to_string()));
                        }
                        Object::Float(left_val % right_val)
                    },
                    _ => return Err(Error::vm(format!("Unknown mixed operation: {:?}", opcode))),
                };
                self.push(Rc::new(result))?;
            },
            (Object::String(left_val), Object::String(right_val)) => {
                // Only string concatenation is supported
                if opcode == Opcode::Add {
                    let concatenated = format!("{}{}", left_val, right_val);
                    self.push(Rc::new(Object::String(concatenated)))?;
                } else {
                    return Err(Error::vm(format!("Unsupported string operation: {:?}", opcode)));
                }
            },
            _ => {
                return Err(Error::type_error(
                    format!("Cannot apply operand {:?} to types {:?} and {:?}", opcode, left, right),
                    SourceLocation::default(),
                ));
            }
        }
        
        Ok(())
    }
    
    /// Execute a comparison operation
    fn execute_comparison(&mut self, opcode: Opcode) -> Result<(), Error> {
        let right = self.pop()?;
        let left = self.pop()?;
        
        match (&*left, &*right) {
            (Object::Integer(left_val), Object::Integer(right_val)) => {
                let result = match opcode {
                    Opcode::Equal => *left_val == *right_val,
                    Opcode::NotEqual => *left_val != *right_val,
                    Opcode::GreaterThan => *left_val > *right_val,
                    Opcode::GreaterThanEqual => *left_val >= *right_val,
                    Opcode::LessThan => *left_val < *right_val,
                    Opcode::LessThanEqual => *left_val <= *right_val,
                    _ => return Err(Error::vm(format!("Unknown comparison operation: {:?}", opcode))),
                };
                self.push(Rc::new(Object::Boolean(result)))?;
            },
            (Object::Float(left_val), Object::Float(right_val)) => {
                let result = match opcode {
                    Opcode::Equal => (*left_val - *right_val).abs() < f64::EPSILON,
                    Opcode::NotEqual => (*left_val - *right_val).abs() >= f64::EPSILON,
                    Opcode::GreaterThan => *left_val > *right_val,
                    Opcode::GreaterThanEqual => *left_val >= *right_val,
                    Opcode::LessThan => *left_val < *right_val,
                    Opcode::LessThanEqual => *left_val <= *right_val,
                    _ => return Err(Error::vm(format!("Unknown comparison operation: {:?}", opcode))),
                };
                self.push(Rc::new(Object::Boolean(result)))?;
            },
            // Allow mixed type comparisons (int/float)
            (Object::Integer(left_val), Object::Float(right_val)) => {
                let left_val = *left_val as f64;
                let result = match opcode {
                    Opcode::Equal => (left_val - *right_val).abs() < f64::EPSILON,
                    Opcode::NotEqual => (left_val - *right_val).abs() >= f64::EPSILON,
                    Opcode::GreaterThan => left_val > *right_val,
                    Opcode::GreaterThanEqual => left_val >= *right_val,
                    Opcode::LessThan => left_val < *right_val,
                    Opcode::LessThanEqual => left_val <= *right_val,
                    _ => return Err(Error::vm(format!("Unknown comparison operation: {:?}", opcode))),
                };
                self.push(Rc::new(Object::Boolean(result)))?;
            },
            (Object::Float(left_val), Object::Integer(right_val)) => {
                let right_val = *right_val as f64;
                let result = match opcode {
                    Opcode::Equal => (*left_val - right_val).abs() < f64::EPSILON,
                    Opcode::NotEqual => (*left_val - right_val).abs() >= f64::EPSILON,
                    Opcode::GreaterThan => *left_val > right_val,
                    Opcode::GreaterThanEqual => *left_val >= right_val,
                    Opcode::LessThan => *left_val < right_val,
                    Opcode::LessThanEqual => *left_val <= right_val,
                    _ => return Err(Error::vm(format!("Unknown comparison operation: {:?}", opcode))),
                };
                self.push(Rc::new(Object::Boolean(result)))?;
            },
            (Object::String(left_val), Object::String(right_val)) => {
                let result = match opcode {
                    Opcode::Equal => left_val == right_val,
                    Opcode::NotEqual => left_val != right_val,
                    Opcode::GreaterThan => left_val > right_val,
                    Opcode::GreaterThanEqual => left_val >= right_val,
                    Opcode::LessThan => left_val < right_val,
                    Opcode::LessThanEqual => left_val <= right_val,
                    _ => return Err(Error::vm(format!("Unknown string comparison: {:?}", opcode))),
                };
                self.push(Rc::new(Object::Boolean(result)))?;
            },
            (Object::Boolean(left_val), Object::Boolean(right_val)) => {
                let result = match opcode {
                    Opcode::Equal => left_val == right_val,
                    Opcode::NotEqual => left_val != right_val,
                    _ => return Err(Error::vm(format!("Invalid boolean comparison: {:?}", opcode))),
                };
                self.push(Rc::new(Object::Boolean(result)))?;
            },
            _ => {
                // For other types, only equality comparisons make sense
                let result = match opcode {
                    Opcode::Equal => left == right,
                    Opcode::NotEqual => left != right,
                    _ => return Err(Error::vm(format!("Cannot compare types {:?} and {:?} with {:?}", left, right, opcode))),
                };
                self.push(Rc::new(Object::Boolean(result)))?;
            }
        }
        
        Ok(())
    }
    
    /// Get the current opcode at the current instruction pointer
    fn current_opcode(&self) -> Result<Opcode, Error> {
        let frame = self.current_frame()?;
        if frame.ip >= frame.instructions.len() {
            return Err(Error::vm("Instruction pointer out of bounds".to_string()));
        }
        
        let op_byte = frame.instructions[frame.ip];
        match op_byte {
            0 => Ok(Opcode::Invalid),
            1 => Ok(Opcode::Nop),
            2 => Ok(Opcode::Constant),
            3 => Ok(Opcode::Add),
            4 => Ok(Opcode::Sub),
            5 => Ok(Opcode::Mul),
            6 => Ok(Opcode::Div),
            7 => Ok(Opcode::True),
            8 => Ok(Opcode::False),
            9 => Ok(Opcode::Equal),
            10 => Ok(Opcode::NotEqual),
            11 => Ok(Opcode::GreaterThan),
            12 => Ok(Opcode::Minus),
            13 => Ok(Opcode::Bang),
            14 => Ok(Opcode::JumpNotTruthy),
            15 => Ok(Opcode::Jump),
            16 => Ok(Opcode::Null),
            17 => Ok(Opcode::SetGlobal),
            18 => Ok(Opcode::GetGlobal),
            19 => Ok(Opcode::Array),
            20 => Ok(Opcode::Hash),
            21 => Ok(Opcode::Index),
            22 => Ok(Opcode::Call),
            23 => Ok(Opcode::ReturnValue),
            24 => Ok(Opcode::Return),
            25 => Ok(Opcode::SetLocal),
            26 => Ok(Opcode::GetLocal),
            27 => Ok(Opcode::GetBuiltin),
            28 => Ok(Opcode::Closure),
            29 => Ok(Opcode::GetFree),
            30 => Ok(Opcode::Pop),
            31 => Ok(Opcode::GreaterThanEqual),
            32 => Ok(Opcode::LessThan),
            33 => Ok(Opcode::LessThanEqual),
            34 => Ok(Opcode::Modulo),
            35 => Ok(Opcode::Dup),
            36 => Ok(Opcode::DefineType),
            37 => Ok(Opcode::DefineField),
            38 => Ok(Opcode::DefineInterface),
            39 => Ok(Opcode::DefineMethod),
            40 => Ok(Opcode::Method),
            41 => Ok(Opcode::VariadicCall),
            _ => Err(Error::vm(format!("Unknown opcode: {}", op_byte))),
        }
    }
    
    /// Get the latest stack item
    pub fn last_popped_stack_elem(&self) -> Option<Rc<Object>> {
        if self.sp > 0 && !self.stack.is_empty() {
            Some(self.stack[self.sp - 1].clone())
        } else {
            None
        }
    }
    
    /// Build a closure from a compiled function and free variables
    fn build_closure(&mut self, constant_idx: usize, num_free: usize) -> Result<Rc<Object>, Error> {
        // Get the compiled function from constants
        let obj = match self.constants.get(constant_idx) {
            Some(val) => val.clone(),
            None => {
                return Err(Error::vm(format!("Invalid constant index: {}", constant_idx)));
            }
        };
        
        // Verify that the constant is a function
        match &*obj {
            Object::CompiledFunction { instructions, num_locals, num_parameters, free_variables: _, name, is_variadic } => {
                // Create a CompiledFunction instance
                let func = CompiledFunction {
                    instructions: instructions.clone(),
                    num_locals: *num_locals as u8,
                    num_parameters: *num_parameters as u8,
                    free_variables: Vec::new(),
                    name: name.clone(),
                    is_variadic: *is_variadic,
                };
                
                // Collect free variables from the stack
                let mut free_vars = Vec::with_capacity(num_free);
                for i in 0..num_free {
                    let free_var_idx = self.sp - num_free + i;
                    if free_var_idx < self.stack.len() {
                        let obj_clone = (*self.stack[free_var_idx]).clone();
                        free_vars.push(obj_clone);
                    } else {
                        return Err(Error::vm(format!("Invalid free variable index: {}", free_var_idx)));
                    }
                }
                
                // Create a closure with the function and free variables
                Ok(Rc::new(Object::Closure {
                    function: Rc::new(func),
                    free_vars,
                }))
            },
            _ => Err(Error::vm(format!("Not a function: {:?}", obj))),
        }
    }

    /// Execute a function call
    fn execute_call(&mut self, num_args: usize) -> Result<(), Error> {
        let fn_obj = self.stack[self.sp - 1 - num_args].clone();
        
        match &*fn_obj {
            Object::CompiledFunction { instructions, num_locals, num_parameters, free_variables: _, name: _, is_variadic } => {
                // Check if number of arguments is correct
                if !(*is_variadic) && *num_parameters != num_args {
                    return Err(Error::vm(format!(
                        "Wrong number of arguments: expected {}, got {}",
                        num_parameters, num_args
                    )));
                } else if *is_variadic && *num_parameters > num_args {
                    return Err(Error::vm(format!(
                        "Not enough arguments for variadic function: expected at least {}, got {}",
                        num_parameters - 1, num_args
                    )));
                }
                
                // Create a new frame for the function call
                let base_pointer = self.sp - num_args;
                let frame = Frame::new(instructions.clone(), base_pointer);
                
                // Push the frame onto the call stack
                if self.frame_index + 1 >= constants::MAX_FRAMES {
                    return Err(Error::vm("Stack overflow".to_string()));
                }
                
                // Move to next frame
                self.frame_index += 1;
                if self.frame_index >= self.frames.len() {
                    self.frames.push(frame);
                } else {
                    self.frames[self.frame_index] = frame;
                }
                
                // Allocate space for local variables
                let num_locals = *num_locals;
                let sp = self.sp;
                
                // Initialize locals to null
                self.sp = self.frames[self.frame_index].base_pointer + num_locals;
                
                // Fill in with nulls
                for i in 0..num_locals {
                    let idx = sp + i;
                    if idx >= self.stack.len() {
                        self.stack.push(Rc::new(Object::Null));
                    } else if idx >= self.sp {
                        self.stack[idx] = Rc::new(Object::Null);
                    }
                }
                
                Ok(())
            },
            Object::Closure { function, free_vars } => {
                // For a closure, we need to get the compiled function from the function field
                // and then create a new frame with that function's instructions
                let instructions = function.instructions.clone();
                let num_locals = function.num_locals as usize;
                let num_parameters = function.num_parameters as usize;
                let is_variadic = function.is_variadic;
                
                // Check if number of arguments is correct
                if !is_variadic && num_parameters != num_args {
                    return Err(Error::vm(format!(
                        "Wrong number of arguments: expected {}, got {}",
                        num_parameters, num_args
                    )));
                } else if is_variadic && num_parameters > num_args {
                    return Err(Error::vm(format!(
                        "Not enough arguments for variadic function: expected at least {}, got {}",
                        num_parameters - 1, num_args
                    )));
                }
                
                // Create a new frame for the function call
                let base_pointer = self.sp - num_args;
                let mut frame = Frame::new(instructions.clone(), base_pointer);
                
                // Add free variables to the frame
                // Convert Vec<Object> to Vec<Rc<Object>>
                let free_vars_rc: Vec<Rc<Object>> = free_vars.iter()
                    .map(|obj| Rc::new(obj.clone()))
                    .collect();
                frame.free_vars = free_vars_rc;
                
                // Push the frame onto the call stack
                if self.frame_index + 1 >= constants::MAX_FRAMES {
                    return Err(Error::vm("Stack overflow".to_string()));
                }
                
                // Move to next frame
                self.frame_index += 1;
                if self.frame_index >= self.frames.len() {
                    self.frames.push(frame);
                } else {
                    self.frames[self.frame_index] = frame;
                }
                
                // Allocate space for local variables
                let sp = self.sp;
                
                // Initialize locals to null
                self.sp = self.frames[self.frame_index].base_pointer + num_locals;
                
                // Fill in with nulls
                for i in 0..num_locals {
                    let idx = sp + i;
                    if idx >= self.stack.len() {
                        self.stack.push(Rc::new(Object::Null));
                    } else if idx >= self.sp {
                        self.stack[idx] = Rc::new(Object::Null);
                    }
                }
                
                Ok(())
            },
            Object::Builtin { name: _, function } => {
                // Execute builtin function directly
                let args = self.stack
                    .iter()
                    .skip(self.sp - num_args)
                    .take(num_args)
                    .map(|obj| (**obj).clone())
                    .collect::<Vec<_>>();
                
                // Remove the function and arguments from the stack
                self.sp -= num_args + 1;
                
                // Call the builtin function
                match function(args) {
                    Ok(result) => {
                        // Push the result onto the stack
                        self.push(Rc::new(result))?;
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            },
            _ => Err(Error::vm(format!("Not a function: {:?}", fn_obj))),
        }
    }

    /// Build an array from elements on the stack
    fn build_array(&mut self, num_elements: usize) -> Result<(), Error> {
        if self.sp < num_elements {
            return Err(Error::vm("Not enough elements on stack".to_string()));
        }
        
        // Collect elements from the stack
        let elements = self.stack
            .iter()
            .skip(self.sp - num_elements)
            .take(num_elements)
            .map(|obj| (**obj).clone())
            .collect::<Vec<_>>();
        
        // Remove the elements from the stack
        self.sp -= num_elements;
        
        // Push the array onto the stack
        self.push(Rc::new(Object::Array(elements)))?;
        
        Ok(())
    }

    /// Build a hash table from key-value pairs on the stack
    fn build_hash(&mut self, num_pairs: usize) -> Result<(), Error> {
        if self.sp < num_pairs * 2 {
            return Err(Error::vm("Not enough elements on stack".to_string()));
        }
        
        // Collect key-value pairs from the stack
        let mut hash = HashMap::new();
        
        for i in 0..num_pairs {
            let key_idx = self.sp - 2 * (num_pairs - i);
            let val_idx = key_idx + 1;
            
            let key = match &*self.stack[key_idx] {
                Object::String(s) => s.clone(),
                k => return Err(Error::vm(format!("Hash key must be a string, got: {:?}", k))),
            };
            
            let value = (*self.stack[val_idx]).clone();
            hash.insert(key, value);
        }
        
        // Remove the key-value pairs from the stack
        self.sp -= num_pairs * 2;
        
        // Push the hash table onto the stack
        self.push(Rc::new(Object::HashTable(hash)))?;
        
        Ok(())
    }

    /// Execute an index operation (array[index] or hash[key])
    fn execute_index_operation(&mut self, left: Rc<Object>, index: Rc<Object>) -> Result<(), Error> {
        // Pop the left and index from the stack
        self.sp -= 2;
        
        match (&*left, &*index) {
            (Object::Array(elements), Object::Integer(idx)) => {
                let idx = *idx as usize;
                if idx >= elements.len() {
                    self.push(Rc::new(Object::Null))?;
                } else {
                    self.push(Rc::new(elements[idx].clone()))?;
                }
            },
            (Object::HashTable(hash), Object::String(key)) => {
                match hash.get(key) {
                    Some(value) => self.push(Rc::new(value.clone()))?,
                    None => self.push(Rc::new(Object::Null))?,
                }
            },
            _ => return Err(Error::vm(format!("Index operation not supported: {:?}[{:?}]", left, index))),
        }
        
        Ok(())
    }
}

// Builtin functions for the VM

/// Get the length of a string, array, or hash table
fn builtin_len(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for len: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::String(s) => Ok(Object::Integer(s.len() as i64)),
        Object::Array(elements) => Ok(Object::Integer(elements.len() as i64)),
        Object::HashTable(hash) => Ok(Object::Integer(hash.len() as i64)),
        obj => Err(Error::vm(format!("Argument to len not supported: got {:?}", obj))),
    }
}

/// Get the first element of an array
fn builtin_first(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for first: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::Array(elements) => {
            if elements.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(elements[0].clone())
            }
        },
        obj => Err(Error::vm(format!("Argument to first must be array, got {:?}", obj))),
    }
}

/// Get the last element of an array
fn builtin_last(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for last: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::Array(elements) => {
            if elements.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(elements[elements.len() - 1].clone())
            }
        },
        obj => Err(Error::vm(format!("Argument to last must be array, got {:?}", obj))),
    }
}

/// Get all elements of an array except the first
fn builtin_rest(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for rest: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::Array(elements) => {
            if elements.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(Object::Array(elements[1..].to_vec()))
            }
        },
        obj => Err(Error::vm(format!("Argument to rest must be array, got {:?}", obj))),
    }
}

/// Push an element onto an array
fn builtin_push(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 2 {
        return Err(Error::vm(format!("Wrong number of arguments for push: expected 2, got {}", args.len())));
    }
    
    match &args[0] {
        Object::Array(elements) => {
            let mut new_elements = elements.clone();
            new_elements.push(args[1].clone());
            Ok(Object::Array(new_elements))
        },
        obj => Err(Error::vm(format!("First argument to push must be array, got {:?}", obj))),
    }
}

/// Print an object to stdout
fn builtin_puts(args: Vec<Object>) -> Result<Object, Error> {
    for arg in args {
        println!("{}", arg);
    }
    
    Ok(Object::Null)
}

/// Get the type of an object
fn builtin_type(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for type: expected 1, got {}", args.len())));
    }
    
    let type_name = match &args[0] {
        Object::Integer(_) => "integer",
        Object::Float(_) => "float",
        Object::Boolean(_) => "boolean",
        Object::String(_) => "string",
        Object::Char(_) => "char",
        Object::Array(_) => "array",
        Object::HashTable(_) => "hash",
        Object::CompiledFunction { .. } => "function",
        Object::Closure { .. } => "closure",
        Object::Builtin { .. } => "builtin",
        Object::Struct { .. } => "struct",
        Object::Interface { .. } => "interface",
        Object::Instance { .. } => "instance",
        Object::Method { .. } => "method",
        Object::Error { .. } => "error",
        Object::Null => "null",
    };
    
    Ok(Object::String(type_name.to_string()))
}

/// Check if an object is an integer
fn builtin_is_integer(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for is_integer: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::Integer(_) => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false)),
    }
}

/// Check if an object is a string
fn builtin_is_string(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for is_string: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::String(_) => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false)),
    }
}

/// Check if an object is an array
fn builtin_is_array(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for is_array: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::Array(_) => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false)),
    }
}

/// Check if an object is a hash table
fn builtin_is_hash(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for is_hash: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::HashTable(_) => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false)),
    }
}

/// Check if an object is null
fn builtin_is_null(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("Wrong number of arguments for is_null: expected 1, got {}", args.len())));
    }
    
    match &args[0] {
        Object::Null => Ok(Object::Boolean(true)),
        _ => Ok(Object::Boolean(false)),
    }
}

/// Implementation of vibez.spill - prints arguments followed by a newline
fn builtin_vibez_spill(args: Vec<Object>) -> Result<Object, Error> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    
    Ok(Object::Null)
}