// Virtual Machine implementation for CURSED
use crate::compiler::{Bytecode, Instructions, Opcode};
use crate::error::{Error, SourceLocation};
use crate::object::Object;
use std::rc::Rc;

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
    fn build_closure(&mut self, const_idx: usize, num_free_vars: usize) -> Result<Rc<Object>, Error> {
        // Get the compiled function from constants
        let function_obj = self.constants.get(const_idx).ok_or_else(|| {
            Error::vm(format!("Invalid constant index for closure: {}", const_idx))
        })?.clone();
        
        // Verify it's a compiled function
        if let Object::CompiledFunction(function) = &*function_obj {
            // Collect free variables from the stack
            let mut free_vars = Vec::with_capacity(num_free_vars);
            
            // Free variables are on the stack before the closure is created
            // Start from sp - num_free_vars
            let start_idx = self.sp.checked_sub(num_free_vars).ok_or_else(|| {
                Error::vm(format!("Stack underflow when collecting free variables"))
            })?;
            
            // Collect the free variables
            for i in 0..num_free_vars {
                let free_var = self.stack[start_idx + i].clone();
                free_vars.push((*free_var).clone());
            }
            
            // Remove free variables from the stack
            self.sp -= num_free_vars;
            
            // Create and return the closure
            Ok(Rc::new(Object::Closure {
                function: function.clone(),
                free_vars,
            }))
        } else {
            Err(Error::vm(format!("Not a compiled function: {:?}", function_obj)))
        }
    }
    
    /// Execute a function call with the given number of arguments
    fn execute_call(&mut self, num_args: usize) -> Result<(), Error> {
        // The function is at stack[sp - 1 - num_args]
        let fn_idx = self.sp - 1 - num_args;
        
        if fn_idx >= self.stack.len() {
            return Err(Error::vm("Function index out of bounds".to_string()));
        }
        
        let fn_obj = self.stack[fn_idx].clone();
        
        match &*fn_obj {
            Object::CompiledFunction(compiled_fn) => {
                // Verify that the number of arguments matches the number of parameters
                if num_args != compiled_fn.num_parameters as usize {
                    return Err(Error::vm(format!(
                        "Wrong number of arguments: got {}, want {}",
                        num_args, compiled_fn.num_parameters
                    )));
                }
                
                // Create a new frame for this function call
                let frame = Frame::new(compiled_fn.instructions.clone(), self.sp - num_args);
                
                // Check if we have too many frames
                if self.frame_index + 1 >= constants::MAX_FRAMES {
                    return Err(Error::stack_overflow("Stack overflow: too many frames".to_string()));
                }
                
                // Add the frame to the call stack
                self.frame_index += 1;
                if self.frame_index < self.frames.len() {
                    self.frames[self.frame_index] = frame;
                } else {
                    self.frames.push(frame);
                }
                
                // Reserve space for local variables on the stack
                let num_locals = compiled_fn.num_locals as usize;
                let base_pointer = self.frames[self.frame_index].base_pointer;
                
                // Make sure we have enough space on the stack
                let required_stack_size = base_pointer + num_locals;
                if required_stack_size > constants::STACK_SIZE {
                    return Err(Error::stack_overflow("Stack overflow for locals".to_string()));
                }
                
                // Initialize locals to null
                let current_stack_size = self.sp;
                for _ in 0..num_locals.saturating_sub(current_stack_size - base_pointer) {
                    self.push(Rc::new(Object::Null))?;
                }
                
                // Set the stack pointer to point past the function's local values
                self.sp = base_pointer + num_locals;
            },
            Object::Closure { function, free_vars } => {
                // Verify that the number of arguments matches the number of parameters
                if num_args != function.num_parameters as usize {
                    return Err(Error::vm(format!(
                        "Wrong number of arguments: got {}, want {}",
                        num_args, function.num_parameters
                    )));
                }
                
                // Create a new frame for this function call
                let frame = Frame {
                    instructions: function.instructions.clone(),
                    ip: 0,
                    base_pointer: self.sp - num_args,
                    free_vars: free_vars.iter().map(|obj| Rc::new(obj.clone())).collect(),
                };
                
                // Check if we have too many frames
                if self.frame_index + 1 >= constants::MAX_FRAMES {
                    return Err(Error::stack_overflow("Stack overflow: too many frames".to_string()));
                }
                
                // Add the frame to the call stack
                self.frame_index += 1;
                if self.frame_index < self.frames.len() {
                    self.frames[self.frame_index] = frame;
                } else {
                    self.frames.push(frame);
                }
                
                // Reserve space for local variables on the stack
                let num_locals = function.num_locals as usize;
                let base_pointer = self.frames[self.frame_index].base_pointer;
                
                // Make sure we have enough space on the stack
                let required_stack_size = base_pointer + num_locals;
                if required_stack_size > constants::STACK_SIZE {
                    return Err(Error::stack_overflow("Stack overflow for locals".to_string()));
                }
                
                // Initialize locals to null
                let current_stack_size = self.sp;
                for _ in 0..num_locals.saturating_sub(current_stack_size - base_pointer) {
                    self.push(Rc::new(Object::Null))?;
                }
                
                // Set the stack pointer to point past the function's local values
                self.sp = base_pointer + num_locals;
            },
            Object::Builtin { function, .. } => {
                // Extract arguments from the stack
                let start_idx = self.sp - num_args;
                let args: Vec<Object> = self.stack[start_idx..self.sp]
                    .iter()
                    .map(|obj| (**obj).clone())
                    .collect();
                
                // Reset the stack pointer to before the arguments
                self.sp = fn_idx;
                
                // Call the builtin function
                match function(args) {
                    Ok(result) => {
                        // Push the result onto the stack
                        self.push(Rc::new(result))?;
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            _ => {
                return Err(Error::vm(format!("Calling non-function: {}", fn_obj)));
            }
        }
        
        Ok(())
    }
    
    /// Execute an index operation (array[index] or hash[key])
    fn execute_index_operation(&mut self, left: Rc<Object>, index: Rc<Object>) -> Result<(), Error> {
        match (&*left, &*index) {
            // Array indexing
            (Object::Array(array), Object::Integer(idx)) => {
                let idx = *idx as usize;
                if idx >= array.len() {
                    // Out of bounds access returns null
                    self.push(Rc::new(Object::Null))?;
                } else {
                    self.push(Rc::new(array[idx].clone()))?;
                }
            },
            // Hash table lookup with String key
            (Object::HashTable(hash), Object::String(key)) => {
                // String keys can be used directly
                match hash.get(key) {
                    Some(value) => self.push(Rc::new(value.clone()))?,
                    None => self.push(Rc::new(Object::Null))?,
                }
            },
            // Hash table lookup with Integer key
            (Object::HashTable(hash), Object::Integer(i)) => {
                // Convert integer to string for hash lookup
                let key = i.to_string();
                match hash.get(&key) {
                    Some(value) => self.push(Rc::new(value.clone()))?,
                    None => self.push(Rc::new(Object::Null))?,
                }
            },
            // Hash table lookup with Boolean key
            (Object::HashTable(hash), Object::Boolean(b)) => {
                // Convert boolean to string for hash lookup
                let key = b.to_string();
                match hash.get(&key) {
                    Some(value) => self.push(Rc::new(value.clone()))?,
                    None => self.push(Rc::new(Object::Null))?,
                }
            },
            // Any other unhashable type
            (Object::HashTable(_), index) => {
                return Err(Error::type_error(
                    format!("Unhashable type used as hash key: {}", index),
                    SourceLocation::default(),
                ))
            },
            // Not a hashable or indexable type
            _ => return Err(Error::type_error(
                format!("Index operator not supported: {} {}", left, index),
                SourceLocation::default(),
            )),
        }
        Ok(())
    }
    
    /// Build an array from elements on the stack
    fn build_array(&mut self, num_elements: usize) -> Result<(), Error> {
        // Check if we have enough elements on the stack
        if num_elements > self.sp {
            return Err(Error::vm(format!("Not enough elements on stack for array: need {}, have {}", 
                                         num_elements, self.sp)));
        }
        
        let start_idx = self.sp - num_elements;
        let elements: Vec<Object> = self.stack[start_idx..self.sp]
            .iter()
            .map(|obj| (**obj).clone())
            .collect();
        
        // Reset the stack pointer to remove the array elements
        self.sp = start_idx;
        
        // Push the array onto the stack
        self.push(Rc::new(Object::Array(elements)))
    }
    
    /// Build a hash map from key-value pairs on the stack
    fn build_hash(&mut self, num_pairs: usize) -> Result<(), Error> {
        // For a hash, we need key-value pairs, so there should be num_pairs * 2 elements
        let total_elements = num_pairs * 2;
        
        // Check if we have enough elements on the stack
        if total_elements > self.sp {
            return Err(Error::vm(format!("Not enough elements on stack for hash: need {}, have {}", 
                                         total_elements, self.sp)));
        }
        
        let mut hash_map = std::collections::HashMap::new();
        
        // Start at the first key
        let mut stack_idx = self.sp - total_elements;
        
        // Extract key-value pairs from the stack
        for _ in 0..num_pairs {
            let key_obj = self.stack[stack_idx].clone();
            stack_idx += 1;
            let value_obj = self.stack[stack_idx].clone();
            stack_idx += 1;
            
            // Convert key to string for hash map
            let key = match &*key_obj {
                Object::String(s) => s.clone(),
                Object::Integer(i) => i.to_string(),
                Object::Boolean(b) => b.to_string(),
                _ => return Err(Error::type_error(
                    format!("Unhashable type used as hash key: {}", key_obj),
                    SourceLocation::default(),
                )),
            };
            
            hash_map.insert(key, (*value_obj).clone());
        }
        
        // Reset the stack pointer to remove the hash elements
        self.sp -= total_elements;
        
        // Push the hash map onto the stack
        self.push(Rc::new(Object::HashTable(hash_map)))
    }
}

// Builtin functions for the CURSED language
// These are defined outside the VM struct for clarity

/// Built-in function: len
/// Returns the length of a string, array, or hash
pub fn builtin_len(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for len: got {}, want 1", args.len())));
    }
    
    match &args[0] {
        Object::String(s) => Ok(Object::Integer(s.len() as i64)),
        Object::Array(arr) => Ok(Object::Integer(arr.len() as i64)),
        Object::HashTable(hash) => Ok(Object::Integer(hash.len() as i64)),
        _ => Err(Error::type_error(
            format!("argument to 'len' not supported, got {}", args[0]),
            SourceLocation::default(),
        )),
    }
}

/// Built-in function: first
/// Returns the first element of an array or the first character of a string
pub fn builtin_first(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for first: got {}, want 1", args.len())));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(arr[0].clone())
            }
        },
        Object::String(s) => {
            if s.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(Object::Char(s.chars().next().unwrap()))
            }
        },
        _ => Err(Error::type_error(
            format!("argument to 'first' must be array or string, got {}", args[0]),
            SourceLocation::default(),
        )),
    }
}

/// Built-in function: last
/// Returns the last element of an array or the last character of a string
pub fn builtin_last(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for last: got {}, want 1", args.len())));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(arr[arr.len() - 1].clone())
            }
        },
        Object::String(s) => {
            if s.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(Object::Char(s.chars().last().unwrap()))
            }
        },
        _ => Err(Error::type_error(
            format!("argument to 'last' must be array or string, got {}", args[0]),
            SourceLocation::default(),
        )),
    }
}

/// Built-in function: rest
/// Returns all elements of an array except the first, or all characters of a string except the first
pub fn builtin_rest(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for rest: got {}, want 1", args.len())));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                Ok(Object::Null)
            } else {
                let rest = arr[1..].to_vec();
                Ok(Object::Array(rest))
            }
        },
        Object::String(s) => {
            if s.is_empty() {
                Ok(Object::Null)
            } else {
                let rest = s.chars().skip(1).collect::<String>();
                Ok(Object::String(rest))
            }
        },
        _ => Err(Error::type_error(
            format!("argument to 'rest' must be array or string, got {}", args[0]),
            SourceLocation::default(),
        )),
    }
}

/// Built-in function: push
/// Adds an element to the end of an array
pub fn builtin_push(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 2 {
        return Err(Error::vm(format!("wrong number of arguments for push: got {}, want 2", args.len())));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            let mut new_array = arr.clone();
            new_array.push(args[1].clone());
            Ok(Object::Array(new_array))
        },
        _ => Err(Error::type_error(
            format!("argument to 'push' must be array, got {}", args[0]),
            SourceLocation::default(),
        )),
    }
}

/// Built-in function: puts
/// Prints a value to the console
pub fn builtin_puts(args: Vec<Object>) -> Result<Object, Error> {
    for arg in args {
        println!("{}", arg);
    }
    
    // puts returns null
    Ok(Object::Null)
}

/// Built-in function: type
/// Returns the type of a value as a string
pub fn builtin_type(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for type: got {}, want 1", args.len())));
    }
    
    let type_name = match &args[0] {
        Object::Integer(_) => "integer",
        Object::Float(_) => "float",
        Object::Boolean(_) => "boolean",
        Object::String(_) => "string",
        Object::Char(_) => "char",
        Object::Array(_) => "array",
        Object::HashTable(_) => "hash",
        Object::CompiledFunction(_) => "function",
        Object::Closure { .. } => "closure",
        Object::Builtin { .. } => "builtin",
        Object::Struct { .. } => "struct",
        Object::Instance { .. } => "instance",
        Object::Error { .. } => "error",
        Object::Null => "null",
    };
    
    Ok(Object::String(type_name.to_string()))
}

/// Built-in function: is_integer
/// Returns true if the value is an integer, false otherwise
pub fn builtin_is_integer(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for is_integer: got {}, want 1", args.len())));
    }
    
    Ok(Object::Boolean(matches!(&args[0], Object::Integer(_))))
}

/// Built-in function: is_string
/// Returns true if the value is a string, false otherwise
pub fn builtin_is_string(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for is_string: got {}, want 1", args.len())));
    }
    
    Ok(Object::Boolean(matches!(&args[0], Object::String(_))))
}

/// Built-in function: is_array
/// Returns true if the value is an array, false otherwise
pub fn builtin_is_array(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for is_array: got {}, want 1", args.len())));
    }
    
    Ok(Object::Boolean(matches!(&args[0], Object::Array(_))))
}

/// Built-in function: is_hash
/// Returns true if the value is a hash, false otherwise
pub fn builtin_is_hash(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for is_hash: got {}, want 1", args.len())));
    }
    
    Ok(Object::Boolean(matches!(&args[0], Object::HashTable(_))))
}

/// Built-in function: is_null
/// Returns true if the value is null, false otherwise
pub fn builtin_is_null(args: Vec<Object>) -> Result<Object, Error> {
    if args.len() != 1 {
        return Err(Error::vm(format!("wrong number of arguments for is_null: got {}, want 1", args.len())));
    }
    
    Ok(Object::Boolean(matches!(&args[0], Object::Null)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    
    #[test]
    fn test_stack_operations() {
        let mut vm = VM::new();
        
        // Test pushing objects onto the stack
        let obj1 = Rc::new(Object::Integer(42));
        let obj2 = Rc::new(Object::Boolean(true));
        
        vm.push(obj1.clone()).unwrap();
        assert_eq!(vm.sp, 1);
        
        vm.push(obj2.clone()).unwrap();
        assert_eq!(vm.sp, 2);
        
        // Test peeking at objects
        let peek_result = vm.peek().unwrap();
        assert_eq!(*peek_result, Object::Boolean(true));
        assert_eq!(vm.sp, 2); // Stack pointer shouldn't change
        
        let peek_at_result = vm.peek_at(1).unwrap();
        assert_eq!(*peek_at_result, Object::Integer(42));
        
        // Test popping objects
        let pop_result1 = vm.pop().unwrap();
        assert_eq!(*pop_result1, Object::Boolean(true));
        assert_eq!(vm.sp, 1);
        
        let pop_result2 = vm.pop().unwrap();
        assert_eq!(*pop_result2, Object::Integer(42));
        assert_eq!(vm.sp, 0);
        
        // Test stack underflow
        let pop_result3 = vm.pop();
        assert!(pop_result3.is_err());
        match pop_result3 {
            Err(Error::VMError(_)) => (),
            _ => panic!("Expected VMError"),
        }
    }
    
    #[test]
    fn test_local_variable_operations() {
        // Create bytecode for setting and getting local variables
        let mut instructions = Vec::new();
        
        // Push constant 42
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        
        // Set local at index 0
        instructions.push(Opcode::SetLocal as u8);
        instructions.push(0); // local index hi byte
        instructions.push(0); // local index lo byte
        
        // Push constant 99
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(1); // const index lo byte
        
        // Set local at index 1
        instructions.push(Opcode::SetLocal as u8);
        instructions.push(0); // local index hi byte
        instructions.push(1); // local index lo byte
        
        // Get local at index 0
        instructions.push(Opcode::GetLocal as u8);
        instructions.push(0); // local index hi byte
        instructions.push(0); // local index lo byte
        
        // Get local at index 1
        instructions.push(Opcode::GetLocal as u8);
        instructions.push(0); // local index hi byte
        instructions.push(1); // local index lo byte
        
        // Add the two local values
        instructions.push(Opcode::Add as u8);
        
        let constants = vec![Object::Integer(42), Object::Integer(99)];
        let bytecode = Bytecode { instructions, constants };
        
        // Create the VM with the bytecode
        let mut vm = VM::with_bytecode(bytecode);
        
        // Local variables are on the stack at the base pointer index
        // Set the base pointer to 0 for this test
        let frame = &mut vm.frames[0];
        frame.base_pointer = 0;
        
        // Make room on the stack for two local variables
        vm.stack.push(Rc::new(Object::Null));
        vm.stack.push(Rc::new(Object::Null));
        vm.sp = 2;
        
        // Run the VM
        match vm.run() {
            Ok(result) => {
                // Verify the result is 42 + 99 = 141
                assert_eq!(*result, Object::Integer(141));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_array_operations() {
        // Create a program that builds an array [1, 2, 3] and gets the element at index 1
        let mut instructions = Vec::new();
        
        // Push constants onto the stack
        for i in 0..3 {
            instructions.push(Opcode::Constant as u8);
            instructions.push(0); // const index hi byte
            instructions.push(i); // const index lo byte
        }
        
        // Create array with 3 elements
        instructions.push(Opcode::Array as u8);
        instructions.push(0); // num elements hi byte  
        instructions.push(3); // num elements lo byte
        
        // Get element at index 1
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(3); // const index lo byte (index value = 1)
        
        instructions.push(Opcode::Index as u8);
        
        let constants = vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(1), // index value
        ];
        
        let bytecode = Bytecode { instructions, constants };
        
        // Run the VM
        let mut vm = VM::with_bytecode(bytecode);
        
        match vm.run() {
            Ok(result) => {
                // The result should be 2 (the value at index 1)
                assert_eq!(*result, Object::Integer(2));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_hash_operations() {
        // Create a program that builds a hash {"one": 1, "two": 2} and gets the value for key "two"
        let mut instructions = Vec::new();
        
        // Push key "one"
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        
        // Push value 1
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(1); // const index lo byte
        
        // Push key "two"
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(2); // const index lo byte
        
        // Push value 2
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(3); // const index lo byte
        
        // Create hash with 2 pairs
        instructions.push(Opcode::Hash as u8);
        instructions.push(0); // num pairs hi byte
        instructions.push(2); // num pairs lo byte
        
        // Push key for lookup
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(2); // const index lo byte (key "two")
        
        // Get the value
        instructions.push(Opcode::Index as u8);
        
        let constants = vec![
            Object::String("one".to_string()),
            Object::Integer(1),
            Object::String("two".to_string()),
            Object::Integer(2),
        ];
        
        let bytecode = Bytecode { instructions, constants };
        
        // Run the VM
        let mut vm = VM::with_bytecode(bytecode);
        
        match vm.run() {
            Ok(result) => {
                // The result should be 2 (the value for key "two")
                assert_eq!(*result, Object::Integer(2));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_nested_hash_operations() {
        // Create a program that:
        // 1. Creates a hash {"inner": 42}
        // 2. Accesses the hash with key "inner" to get 42
        let mut instructions = Vec::new();
        
        // Create a hash {"inner": 42}
        // Key "inner"
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        
        // Value 42
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(1); // const index lo byte
        
        // Create hash
        instructions.push(Opcode::Hash as u8);
        instructions.push(0); // num pairs hi byte
        instructions.push(1); // num pairs lo byte
        
        // Access the hash with key "inner"
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        
        instructions.push(Opcode::Index as u8);
        
        // Constants for the program
        let constants = vec![
            Object::String("inner".to_string()),
            Object::Integer(42),
        ];
        
        let bytecode = Bytecode { instructions, constants };
        
        // Create the VM and run the program
        let mut vm = VM::with_bytecode(bytecode);
        
        // Run the VM
        match vm.run() {
            Ok(result) => {
                // The result should be 42
                assert_eq!(*result, Object::Integer(42));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_complex_array_operations() {
        // Create a nested array structure [[1, 2], [3, 4]] and access [1][0] == 3
        let mut instructions = Vec::new();
        
        // Push constants 1, 2 for first inner array
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(1); // const index lo byte
        
        // Create first inner array [1, 2]
        instructions.push(Opcode::Array as u8);
        instructions.push(0); // num elements hi byte
        instructions.push(2); // num elements lo byte
        
        // Push constants 3, 4 for second inner array
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(2); // const index lo byte
        
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(3); // const index lo byte
        
        // Create second inner array [3, 4]
        instructions.push(Opcode::Array as u8);
        instructions.push(0); // num elements hi byte
        instructions.push(2); // num elements lo byte
        
        // Create outer array [[1, 2], [3, 4]]
        instructions.push(Opcode::Array as u8);
        instructions.push(0); // num elements hi byte
        instructions.push(2); // num elements lo byte
        
        // Get the second inner array (index 1)
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(4); // const index lo byte (index value = 1)
        
        instructions.push(Opcode::Index as u8);
        
        // Get the first element of the second inner array (index 0)
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(5); // const index lo byte (index value = 0)
        
        instructions.push(Opcode::Index as u8);
        
        let constants = vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(4),
            Object::Integer(1), // Outer array index (1)
            Object::Integer(0), // Inner array index (0)
        ];
        
        let bytecode = Bytecode { instructions, constants };
        
        // Run the VM
        let mut vm = VM::with_bytecode(bytecode);
        
        match vm.run() {
            Ok(result) => {
                // The result should be 3 (the value at outer[1][0])
                assert_eq!(*result, Object::Integer(3));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_closure_without_free_vars() {
        // Create a simple closure with no free variables
        // The closure just returns a constant value (42)
        
        // Create a constant for the compiled function
        let compiled_fn = Object::CompiledFunction(Rc::new(crate::compiler::CompiledFunction {
            instructions: vec![
                Opcode::Constant as u8, 0, 1, // Push constant (42) which is at index 1 in the constants array
                Opcode::ReturnValue as u8,    // Return the value
            ],
            num_locals: 0,
            num_parameters: 0,
            name: Some("test_closure".to_string()),
        }));
        
        // Instructions for creating and calling the closure
        let mut instructions = Vec::new();
        
        // Create the closure (compiled_fn is at constant index 0, with 0 free vars)
        instructions.push(Opcode::Closure as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        instructions.push(0); // num free vars hi byte
        instructions.push(0); // num free vars lo byte
        
        // Call the closure with 0 arguments
        instructions.push(Opcode::Call as u8);
        instructions.push(0); // num args hi byte
        instructions.push(0); // num args lo byte
        
        // Set up bytecode with the instructions and constants
        // Note: we need both the compiled function AND the integer 42 in the constants array
        let constants = vec![
            compiled_fn,
            Object::Integer(42),
        ];
        let bytecode = Bytecode { instructions, constants };
        
        // Create and run the VM
        let mut vm = VM::with_bytecode(bytecode);
        
        match vm.run() {
            Ok(result) => {
                // The result should be 42
                assert_eq!(*result, Object::Integer(42));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_closure_with_free_vars() {
        // Create a closure that captures a free variable and returns it
        // The free variable is 42
        
        // Create a constant for the compiled function that uses a free variable
        let compiled_fn = Object::CompiledFunction(Rc::new(crate::compiler::CompiledFunction {
            instructions: vec![
                Opcode::GetFree as u8, 0, 0, // Get free variable at index 0
                Opcode::ReturnValue as u8,   // Return the value
            ],
            num_locals: 0,
            num_parameters: 0,
            name: Some("test_closure_with_free_vars".to_string()),
        }));
        
        // Instructions for creating and calling the closure
        let mut instructions = Vec::new();
        
        // Push the value 42 onto the stack (will become a free variable)
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte 
        instructions.push(1); // const index lo byte (42 is at index 1)
        
        // Create the closure (compiled_fn is at constant index 0, with 1 free var)
        instructions.push(Opcode::Closure as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        instructions.push(0); // num free vars hi byte
        instructions.push(1); // num free vars lo byte
        
        // Call the closure with 0 arguments
        instructions.push(Opcode::Call as u8);
        instructions.push(0); // num args hi byte
        instructions.push(0); // num args lo byte
        
        // Set up bytecode with the instructions and constants
        let constants = vec![
            compiled_fn,
            Object::Integer(42),
        ];
        let bytecode = Bytecode { instructions, constants };
        
        // Create and run the VM
        let mut vm = VM::with_bytecode(bytecode);
        
        match vm.run() {
            Ok(result) => {
                // The result should be 42 (the free variable)
                assert_eq!(*result, Object::Integer(42));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_nested_closures() {
        // Create nested closures where an outer closure returns another closure
        // and the inner closure captures a free variable from the outer scope
        
        // Inner function (returns the free variable at index 0)
        let inner_fn = Object::CompiledFunction(Rc::new(crate::compiler::CompiledFunction {
            instructions: vec![
                Opcode::GetFree as u8, 0, 0, // Get free variable at index 0
                Opcode::ReturnValue as u8,   // Return the value
            ],
            num_locals: 0,
            num_parameters: 0,
            name: Some("inner_closure".to_string()),
        }));
        
        // Outer function (creates and returns the inner closure)
        let outer_fn = Object::CompiledFunction(Rc::new(crate::compiler::CompiledFunction {
            instructions: vec![
                Opcode::GetLocal as u8, 0, 0, // Get local variable at index 0 (the argument 42)
                Opcode::Closure as u8, 0, 1, 0, 1, // Create closure (inner_fn) with 1 free var
                Opcode::ReturnValue as u8,    // Return the closure
            ],
            num_locals: 1,
            num_parameters: 1,
            name: Some("outer_closure".to_string()),
        }));
        
        // Instructions for creating the outer closure, calling it with arg 42, 
        // then calling the returned inner closure
        let mut instructions = Vec::new();
        
        // Create the outer closure (compiled_fn is at constant index 0, with 0 free vars)
        instructions.push(Opcode::Closure as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        instructions.push(0); // num free vars hi byte
        instructions.push(0); // num free vars lo byte
        
        // Push argument 42 for the outer closure
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(2); // const index lo byte (42 is at index 2)
        
        // Call the outer closure with 1 argument
        instructions.push(Opcode::Call as u8);
        instructions.push(0); // num args hi byte
        instructions.push(1); // num args lo byte
        
        // Call the inner closure (returned by the outer closure) with 0 arguments
        instructions.push(Opcode::Call as u8);
        instructions.push(0); // num args hi byte
        instructions.push(0); // num args lo byte
        
        // Set up bytecode with the instructions and constants
        let constants = vec![
            outer_fn,
            inner_fn,
            Object::Integer(42),
        ];
        let bytecode = Bytecode { instructions, constants };
        
        // Create and run the VM
        let mut vm = VM::with_bytecode(bytecode);
        
        match vm.run() {
            Ok(result) => {
                // The result should be 42 (the captured value)
                assert_eq!(*result, Object::Integer(42));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_builtin_functions() {
        // Test the builtin len function with a string
        let result = builtin_len(vec![Object::String("hello".to_string())]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Integer(5));
        
        // Test the builtin first function with an array
        let result = builtin_first(vec![Object::Array(vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
        ])]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Integer(1));
        
        // Test the builtin type function
        let result = builtin_type(vec![Object::Integer(42)]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::String("integer".to_string()));
        
        // Test the builtin is_integer function
        let result = builtin_is_integer(vec![Object::Integer(42)]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(true));
        
        // Test the builtin is_string function with a non-string
        let result = builtin_is_string(vec![Object::Integer(42)]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Boolean(false));
    }
    
    #[test]
    fn test_type_declarations() {
        // Test defining a struct/type and adding fields
        let mut instructions = Vec::new();
        let mut constants = Vec::new();
        
        // Add constants for the type name and field names/types
        constants.push(Object::String("Person".to_string())); // 0: type name
        constants.push(Object::String("name".to_string()));   // 1: field1 name
        constants.push(Object::String("tea".to_string()));    // 2: field1 type
        constants.push(Object::String("age".to_string()));    // 3: field2 name
        constants.push(Object::String("normie".to_string())); // 4: field2 type
        
        // Load the type name and create a struct type with 2 fields
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(0); // const index lo byte
        
        instructions.push(Opcode::DefineType as u8);
        instructions.push(0); // num fields hi byte
        instructions.push(2); // num fields lo byte
        
        // Define first field (name: tea)
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(1); // const index lo byte (field name)
        
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(2); // const index lo byte (field type)
        
        instructions.push(Opcode::DefineField as u8);
        
        // Define second field (age: normie)
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(3); // const index lo byte (field name)
        
        instructions.push(Opcode::Constant as u8);
        instructions.push(0); // const index hi byte
        instructions.push(4); // const index lo byte (field type)
        
        instructions.push(Opcode::DefineField as u8);
        
        // Store in global variable
        instructions.push(Opcode::SetGlobal as u8);
        instructions.push(0); // global index hi byte
        instructions.push(0); // global index lo byte
        
        // Get the struct definition back
        instructions.push(Opcode::GetGlobal as u8);
        instructions.push(0); // global index hi byte
        instructions.push(0); // global index lo byte
        
        let bytecode = Bytecode { instructions, constants };
        let mut vm = VM::new();
        vm.with_bytecode_and_state(bytecode, Vec::new());
        
        let result = vm.run().unwrap();
        
        // Verify the result is a struct with the correct fields
        match &*result {
            Object::Struct { name, fields } => {
                assert_eq!(name, "Person");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, "name");
                assert_eq!(fields[0].1, "tea");
                assert_eq!(fields[1].0, "age");
                assert_eq!(fields[1].1, "normie");
            },
            _ => panic!("Expected struct, got {:?}", result),
        }
    }
} 