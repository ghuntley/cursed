use crate::compiler::{Bytecode, CompiledFunction, Instructions, Object, Opcode};
use crate::error::{Error, ErrorReporter};
use crate::memory::{MemoryManager};
use crate::memory::gc::{Traceable, Visitor};
use crate::memory::tagged::Tag;
use crate::prelude::{VecExt, CloneableVecExt};
use std::fmt;
use std::rc::Rc;
use std::alloc::Layout;
use std::ptr::NonNull;
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
}

impl Frame {
    /// Create a new frame for a function
    pub fn new(function: Rc<CompiledFunction>, base_pointer: usize) -> Self {
        Frame {
            instructions: function.instructions.clone(),
            ip: 0,
            base_pointer,
            function,
            closure: None,
        }
    }
    
    /// Create a new frame for a closure
    pub fn new_with_closure(closure: Rc<Closure>, base_pointer: usize) -> Self {
        Frame {
            instructions: closure.function.instructions.clone(),
            ip: 0,
            base_pointer,
            function: Rc::clone(&closure.function),
            closure: Some(closure),
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

/// The virtual machine that executes the bytecode
#[derive(Clone)]
pub struct VM {
    /// Constants from the bytecode
    pub constants: Vec<Object>,
    /// Instructions from the bytecode
    pub instructions: Instructions,
    
    /// The stack for storing values
    pub stack: Vec<Object>,
    /// The stack pointer (points to the next available slot)
    pub sp: usize,
    
    /// Global variables store
    pub globals: Vec<Object>,
    
    /// Call frames for function execution
    pub frames: Vec<Frame>,
    /// Current frame index
    pub frame_index: usize,
    
    /// Memory manager for custom allocation
    pub memory_manager: MemoryManager,
    
    /// Counter for periodic GC
    pub instruction_counter: usize,
}

impl VecExt<Object> for VM {
    fn push(&mut self, item: Object) {
        if self.stack.len() < STACK_SIZE {
            self.stack.push(item);
        }
    }
    
    fn len(&self) -> usize {
        self.stack.len()
    }
    
    fn capacity(&self) -> usize {
        self.stack.capacity()
    }
    
    fn clear(&mut self) {
        self.stack.clear();
    }
    
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    
    fn swap_remove(&mut self, index: usize) -> Object {
        self.stack.swap_remove(index)
    }
    
    fn iter(&self) -> Iter<'_, Object> {
        self.stack.iter()
    }
    
    fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, Object> {
        self.stack.drain(range)
    }
    
    fn reserve(&mut self, additional: usize) {
        self.stack.reserve(additional);
    }
    
    fn reverse(&mut self) {
        self.stack.reverse();
    }
}

// Special methods for VM not covered by VecExt
impl VM {
    fn pop(&mut self) -> Object {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp -= 1;
        self.stack[self.sp].clone()
    }
    
    fn push_frame(&mut self, frame: Frame) -> Result<(), Error> {
        if self.frame_index >= MAX_FRAMES - 1 {
            return Err(Error::Runtime("Stack overflow: too many function calls".to_string()));
        }
        
        self.frames.push(frame);
        self.frame_index += 1;
        
        Ok(())
    }
    
    /// Create a new VM with the provided bytecode
    pub fn new(bytecode: Bytecode) -> Result<Self, Error> {
        let main_fn = Rc::new(CompiledFunction {
            instructions: bytecode.instructions.clone(),
            locals_count: 0,
            parameters_count: 0,
            free_variables: vec![],
        });
        
        let main_closure = Rc::new(Closure {
            function: main_fn,
            free_variables: vec![],
        });
        
        let main_frame = Frame::new_with_closure(main_closure, 0);
        
        // Initialize the VM
        let mut vm = Self {
            constants: bytecode.constants.clone(),
            instructions: bytecode.instructions.clone(),
            stack: Vec::with_capacity(STACK_SIZE),
            sp: 0,
            globals: Vec::new(),
            frames: vec![main_frame],
            frame_index: 0,
            memory_manager: MemoryManager::new(DEFAULT_MEMORY_SIZE)?,
            instruction_counter: 0,
        };
        
        // Pre-allocate the stack
        vm.stack.resize(STACK_SIZE, Object::Null);
        
        // Pre-allocate globals
        vm.globals.resize(GLOBALS_SIZE, Object::Null);
        
        Ok(vm)
    }
    
    /// Get the current frame
    pub fn current_frame(&self) -> &Frame {
        &self.frames[self.frame_index]
    }
    
    /// Get a mutable reference to the current frame
    pub fn current_frame_mut(&mut self) -> &mut Frame {
        &mut self.frames[self.frame_index]
    }
    
    /// Pop a frame from the call stack
    pub fn pop_frame(&mut self) -> Frame {
        self.frame_index -= 1;
        self.frames.pop().unwrap()
    }
    
    /// Read a uint8 from the instructions at the current position
    pub fn read_uint8(&mut self) -> Result<u8, Error> {
        let frame = self.current_frame_mut();
        let ip = frame.ip;
        
        if ip >= frame.instructions.len() {
            return Err(Error::Runtime("Instruction pointer out of bounds".to_string()));
        }
        
        let value = frame.instructions[ip];
        frame.ip += 1;
        
        Ok(value)
    }
    
    /// Read a uint16 from the instructions at the current position
    pub fn read_uint16(&mut self) -> Result<u16, Error> {
        let frame = self.current_frame_mut();
        let ip = frame.ip;
        
        if ip + 1 >= frame.instructions.len() {
            return Err(Error::Runtime("Instruction pointer out of bounds".to_string()));
        }
        
        let value = ((frame.instructions[ip] as u16) << 8) | (frame.instructions[ip + 1] as u16);
        frame.ip += 2;
        
        Ok(value)
    }
    
    /// Execute a function call
    pub fn execute_call(&mut self, num_args: usize) -> Result<(), Error> {
        let callee = self.stack[self.sp - 1 - num_args].clone();
        
        match callee {
            Object::Closure(closure) => {
                // Check that number of arguments matches function parameter count
                if num_args != closure.function.parameters_count {
                    return Err(Error::Runtime(format!(
                        "Wrong number of arguments: got {}, want {}",
                        num_args,
                        closure.function.parameters_count
                    )));
                }
                
                // Create a new frame for the function call
                let frame = Frame::new_with_closure(closure, self.sp - num_args);
                self.push_frame(frame)?;
                
                // Make space for the function's local variables
                self.sp += closure.function.locals_count;
                
                Ok(())
            },
            _ => {
                Err(Error::Runtime(format!("Calling non-function: {:?}", callee)))
            }
        }
    }
    
    /// Execute building an array
    pub fn execute_build_array(&mut self, num_elements: usize) -> Result<(), Error> {
        let mut elements = Vec::with_capacity(num_elements);
        for i in (0..num_elements).rev() {
            if self.sp >= i + 1 {
                elements.push(self.stack[self.sp - i - 1].clone());
            } else {
                return Err(Error::from_str("Stack underflow"));
            }
        }
        
        // Remove the elements from the stack
        self.sp -= num_elements;
        
        // Push the array onto the stack
        let array_obj = Object::Array(elements);
        self.push(array_obj);
        
        Ok(())
    }
    
    /// Execute building a hash
    pub fn execute_build_hash(&mut self, num_pairs: usize) -> Result<(), Error> {
        let mut hash = HashMap::new();
        for i in (0..num_pairs).rev() {
            let value_index = self.sp - 2 * i - 1;
            let key_index = self.sp - 2 * i - 2;
            
            if key_index < 0 || value_index < 0 || key_index >= self.stack.len() || value_index >= self.stack.len() {
                return Err(Error::from_str("Stack underflow"));
            }
            
            let value = self.stack[value_index].clone();
            if let Object::String(key) = &self.stack[key_index] {
                hash.insert(key.clone(), value);
            } else {
                return Err(Error::from_str("Hash key must be a string"));
            }
        }
        
        // Remove the elements from the stack
        self.sp -= 2 * num_pairs;
        
        // Push the hash onto the stack
        self.push(Object::HashTable(hash));
        
        Ok(())
    }
    
    /// Execute an index operation
    pub fn execute_index_operation(&mut self) -> Result<(), Error> {
        let index = self.pop();
        let left = self.pop();
        
        match (left, index) {
            (Object::Array(array), Object::Integer(index)) => {
                let i = index as usize;
                if i < array.len() {
                    self.push(array[i].clone());
                    Ok(())
                } else {
                    Err(Error::from_str("Index out of bounds"))
                }
            },
            (Object::HashTable(hash), Object::String(key)) => {
                if let Some(value) = hash.get(&key) {
                    self.push(value.clone());
                    Ok(())
                } else {
                    Err(Error::from_str("Key not found in hash"))
                }
            },
            _ => Err(Error::from_str("Invalid index operation")),
        }
    }
    
    /// Push a closure
    pub fn push_closure(&mut self, const_index: usize, num_free: usize) -> Result<(), Error> {
        if const_index >= self.constants.len() {
            return Err(Error::from_str("Invalid constant index"));
        }
        
        let mut free_vars = Vec::with_capacity(num_free);
        for i in 0..num_free {
            free_vars.push(self.stack[self.sp - num_free + i].clone());
        }
        
        self.sp -= num_free;
        
        if let Object::CompiledFunction(func) = &self.constants[const_index] {
            let closure = Rc::new(Closure {
                function: func.clone(),
                free_variables: free_vars,
            });
            self.push(Object::Closure(closure));
            Ok(())
        } else {
            Err(Error::from_str("Not a function"))
        }
    }
    
    /// Run the VM on the current bytecode
    pub fn run(&mut self) -> Result<(), Error> {
        // Number of instructions to execute before checking if GC should run
        const GC_CHECK_INTERVAL: usize = 1000;
        // Threshold for triggering GC (e.g., if heap is 70% full)
        const GC_THRESHOLD_PERCENTAGE: f32 = 0.7;
        
        while self.current_frame().ip < self.current_frame().instructions.len() {
            // Increment instruction counter
            self.instruction_counter += 1;
            
            // Periodically check if garbage collection should run
            if self.instruction_counter % GC_CHECK_INTERVAL == 0 {
                match self.memory_manager.garbage_collector.try_borrow() {
                    Ok(gc) => {
                        let heap_usage = gc.stats.heap_usage;
                        let heap_capacity = gc.stats.heap_capacity;
                        
                        if heap_capacity > 0 && (heap_usage as f32 / heap_capacity as f32) > GC_THRESHOLD_PERCENTAGE {
                            // Heap is above threshold, trigger garbage collection
                            drop(gc); // Drop borrow before mutably borrowing
                            self.collect_garbage()?;
                        }
                    },
                    Err(_) => {
                        // Skip GC this time if already borrowed
                    }
                }
            }
            
            let op_code = Opcode::from(self.current_frame_mut().instructions[self.current_frame().ip]);
            self.current_frame_mut().ip += 1;
            
            match op_code {
                Opcode::Constant => {
                    let const_index = self.read_uint16()? as usize;
                    if const_index < self.constants.len() {
                        let _ = self.push(self.constants[const_index].clone());
                    } else {
                        return Err(Error::Runtime(format!("Constant index out of bounds: {}", const_index)));
                    }
                },
                Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => {
                    let _ = self.execute_binary_operation(op_code);
                },
                Opcode::Equal | Opcode::NotEqual | Opcode::GreaterThan |
                Opcode::GreaterThanEqual | Opcode::LessThan | Opcode::LessThanEqual => {
                    let _ = self.execute_comparison(op_code);
                },
                Opcode::Minus | Opcode::Bang => {
                    let _ = self.execute_prefix_operation(op_code);
                },
                Opcode::Pop => {
                    self.pop();
                },
                Opcode::True => {
                    let _ = self.push(Object::Boolean(true));
                },
                Opcode::False => {
                    let _ = self.push(Object::Boolean(false));
                },
                Opcode::JumpIfFalse => {
                    let pos = self.read_uint16()?;
                    let condition = self.pop();
                    
                    if let Object::Boolean(false) = condition {
                        self.current_frame_mut().ip = pos as usize;
                    }
                },
                Opcode::Jump => {
                    let pos = self.read_uint16()?;
                    self.current_frame_mut().ip = pos as usize;
                },
                Opcode::Null => {
                    let _ = self.push(Object::Null);
                },
                Opcode::GetGlobal => {
                    let global_index = self.read_uint16()? as usize;
                    if global_index >= self.globals.len() {
                        return Err(Error::Runtime(format!("Global index out of bounds: {}", global_index)));
                    }
                    let _ = self.push(self.globals[global_index].clone());
                },
                Opcode::SetGlobal => {
                    let global_index = self.read_uint16()? as usize;
                    if global_index >= self.globals.len() {
                        self.globals.resize(global_index + 1, Object::Null);
                    }
                    self.globals[global_index] = self.pop();
                },
                Opcode::GetLocal => {
                    let local_index = self.read_uint8()? as usize;
                    let base_pointer = self.current_frame().base_pointer;
                    if base_pointer + local_index >= self.sp {
                        return Err(Error::Runtime(format!("Local index out of bounds: {}", local_index)));
                    }
                    let _ = self.push(self.stack[base_pointer + local_index].clone());
                },
                Opcode::SetLocal => {
                    let local_index = self.read_uint8()? as usize;
                    let base_pointer = self.current_frame().base_pointer;
                    if base_pointer + local_index >= self.sp {
                        return Err(Error::Runtime(format!("Local index out of bounds: {}", local_index)));
                    }
                    self.stack[base_pointer + local_index] = self.pop();
                },
                Opcode::Call => {
                    let num_args = self.read_uint8()? as usize;
                    let _ = self.execute_call(num_args);
                },
                Opcode::ReturnValue => {
                    let return_value = self.pop();
                    let frame = self.pop_frame();
                    self.sp = frame.base_pointer;
                    let _ = self.push(return_value);
                },
                Opcode::Return => {
                    let frame = self.pop_frame();
                    self.sp = frame.base_pointer;
                    let _ = self.push(Object::Null);
                },
                Opcode::Array => {
                    let num_elements = self.read_uint16()? as usize;
                    let _ = self.execute_build_array(num_elements);
                },
                Opcode::Hash => {
                    let num_pairs = self.read_uint16()? as usize;
                    let _ = self.execute_build_hash(num_pairs);
                },
                Opcode::Index => {
                    let _ = self.execute_index_operation();
                },
                Opcode::Closure => {
                    let const_index = self.read_uint16()? as usize;
                    let num_free = self.read_uint8()? as usize;
                    let _ = self.push_closure(const_index, num_free);
                },
                Opcode::GetFree => {
                    let free_index = self.read_uint8()? as usize;
                    let current_closure = match &self.current_frame().closure {
                        Some(closure) => closure,
                        None => return Err(Error::Runtime("No closure found in current frame".to_string())),
                    };
                    
                    if free_index >= current_closure.free_variables.len() {
                        return Err(Error::Runtime(format!("Free variable index out of bounds: {}", free_index)));
                    }
                    
                    let _ = self.push(current_closure.free_variables[free_index].clone());
                },
                _ => return Err(Error::Runtime(format!("Unknown opcode: {:?}", op_code))),
            }
        }
        
        Ok(())
    }
    
    /// Execute a binary operation
    pub fn execute_binary_operation(&mut self, op_code: Opcode) -> Result<(), Error> {
        let right = self.pop();
        let left = self.pop();
        
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => {
                match op_code {
                    Opcode::Add => self.push(Object::Integer(l + r))?,
                    Opcode::Sub => self.push(Object::Integer(l - r))?,
                    Opcode::Mul => self.push(Object::Integer(l * r))?,
                    Opcode::Div => {
                        if r == 0 {
                            return Err(Error::Runtime("Division by zero".to_string()));
                        }
                        self.push(Object::Integer(l / r))?;
                    },
                    _ => return Err(Error::Runtime(format!("Unknown integer operation: {:?}", op_code))),
                }
            },
            (Object::Float(l), Object::Float(r)) => {
                match op_code {
                    Opcode::Add => self.push(Object::Float(l + r))?,
                    Opcode::Sub => self.push(Object::Float(l - r))?,
                    Opcode::Mul => self.push(Object::Float(l * r))?,
                    Opcode::Div => {
                        if r == 0.0 {
                            return Err(Error::Runtime("Division by zero".to_string()));
                        }
                        self.push(Object::Float(l / r))?;
                    },
                    _ => return Err(Error::Runtime(format!("Unknown float operation: {:?}", op_code))),
                }
            },
            (Object::String(l), Object::String(r)) => {
                match op_code {
                    Opcode::Add => self.push(Object::String(format!("{}{}", l, r)))?,
                    _ => return Err(Error::Runtime(format!("Unsupported string operation: {:?}", op_code))),
                }
            },
            _ => {
                return Err(Error::Runtime(
                    format!("Unsupported types for binary operation: {:?} and {:?}", left, right)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Execute a comparison operation
    pub fn execute_comparison(&mut self, op_code: Opcode) -> Result<(), Error> {
        let right = self.pop();
        let left = self.pop();
        
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => {
                match op_code {
                    Opcode::Equal => self.push(Object::Boolean(l == r))?,
                    Opcode::NotEqual => self.push(Object::Boolean(l != r))?,
                    Opcode::GreaterThan => self.push(Object::Boolean(l > r))?,
                    Opcode::GreaterThanEqual => self.push(Object::Boolean(l >= r))?,
                    Opcode::LessThan => self.push(Object::Boolean(l < r))?,
                    Opcode::LessThanEqual => self.push(Object::Boolean(l <= r))?,
                    _ => return Err(Error::Runtime(format!("Unknown integer comparison: {:?}", op_code))),
                }
            },
            (Object::Float(l), Object::Float(r)) => {
                match op_code {
                    Opcode::Equal => self.push(Object::Boolean(l == r))?,
                    Opcode::NotEqual => self.push(Object::Boolean(l != r))?,
                    Opcode::GreaterThan => self.push(Object::Boolean(l > r))?,
                    Opcode::GreaterThanEqual => self.push(Object::Boolean(l >= r))?,
                    Opcode::LessThan => self.push(Object::Boolean(l < r))?,
                    Opcode::LessThanEqual => self.push(Object::Boolean(l <= r))?,
                    _ => return Err(Error::Runtime(format!("Unknown float comparison: {:?}", op_code))),
                }
            },
            (Object::Boolean(l), Object::Boolean(r)) => {
                match op_code {
                    Opcode::Equal => self.push(Object::Boolean(l == r))?,
                    Opcode::NotEqual => self.push(Object::Boolean(l != r))?,
                    _ => return Err(Error::Runtime(format!("Unsupported boolean comparison: {:?}", op_code))),
                }
            },
            (Object::String(l), Object::String(r)) => {
                match op_code {
                    Opcode::Equal => self.push(Object::Boolean(l == r))?,
                    Opcode::NotEqual => self.push(Object::Boolean(l != r))?,
                    _ => return Err(Error::Runtime(format!("Unsupported string comparison: {:?}", op_code))),
                }
            },
            _ => return Err(Error::Runtime(format!("Unsupported types for comparison: {:?} and {:?}", left, right))),
        }
        
        Ok(())
    }

    /// Execute a prefix operation
    pub fn execute_prefix_operation(&mut self, op_code: Opcode) -> Result<(), Error> {
        let operand = self.pop();
        
        match op_code {
            Opcode::Minus => {
                match operand {
                    Object::Integer(value) => self.push(Object::Integer(-value))?,
                    Object::Float(value) => self.push(Object::Float(-value))?,
                    _ => return Err(Error::Runtime(format!("Unsupported type for negation: {:?}", operand))),
                }
            },
            Opcode::Bang => {
                match operand {
                    Object::Boolean(value) => self.push(Object::Boolean(!value))?,
                    _ => return Err(Error::Runtime(format!("Unsupported type for logical NOT: {:?}", operand))),
                }
            },
            _ => return Err(Error::Runtime(format!("Unknown prefix operation: {:?}", op_code))),
        }
        
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
    
    /// Get the global variables as a vector
    pub fn global_values(&self) -> Vec<(usize, Object)> {
        self.globals.iter().enumerate()
            .filter(|(_, obj)| !matches!(obj, Object::Null))
            .map(|(i, obj)| (i, obj.clone()))
            .collect()
    }
    
    /// Print the current state of the VM (for debugging purposes)
    pub fn print_state(&self) -> String {
        let mut output = String::new();
        
        output.push_str("VM State:\n");
        output.push_str(&format!("  Instruction pointer: {}\n", self.current_frame().ip));
        output.push_str(&format!("  Stack pointer: {}\n", self.sp));
        output.push_str(&format!("  Frame index: {}\n", self.frame_index));
        
        output.push_str("  Stack:\n");
        for (i, obj) in self.stack[0..self.sp].iter().enumerate() {
            output.push_str(&format!("    [{}]: {:?}\n", i, obj));
        }
        
        output.push_str("  Globals:\n");
        for (i, obj) in self.globals.iter().enumerate() {
            if !matches!(obj, Object::Null) {
                output.push_str(&format!("    [{}]: {:?}\n", i, obj));
            }
        }
        
        output
    }
    
    /// Execute a single instruction and return the current instruction
    pub fn step(&mut self) -> Result<(Opcode, Option<Object>), Error> {
        if self.current_frame().ip >= self.current_frame().instructions.len() {
            return Err(Error::Runtime("End of bytecode".to_string()));
        }
        
        let op_code = Opcode::from(self.current_frame_mut().instructions[self.current_frame().ip]);
        let old_sp = self.sp;
        
        // Execute single instruction using the run method's pattern matching
        self.current_frame_mut().ip += 1;
        
        match op_code {
            Opcode::Constant => {
                let const_index = self.read_uint16()?;
                self.push(self.constants[const_index].clone())?;
            },
            Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => {
                self.execute_binary_operation(op_code)?;
            },
            Opcode::Equal | Opcode::NotEqual | Opcode::GreaterThan |
            Opcode::GreaterThanEqual | Opcode::LessThan | Opcode::LessThanEqual => {
                self.execute_comparison(op_code)?;
            },
            Opcode::Minus | Opcode::Bang => {
                self.execute_prefix_operation(op_code)?;
            },
            Opcode::Pop => {
                self.pop();
            },
            Opcode::True => {
                self.push(Object::Boolean(true))?;
            },
            Opcode::False => {
                self.push(Object::Boolean(false))?;
            },
            Opcode::JumpIfFalse => {
                let pos = self.read_uint16()?;
                let condition = self.pop();
                
                if let Object::Boolean(false) = condition {
                    self.current_frame_mut().ip = pos as usize;
                }
            },
            Opcode::Jump => {
                let pos = self.read_uint16()?;
                self.current_frame_mut().ip = pos as usize;
            },
            Opcode::Null => {
                self.push(Object::Null)?;
            },
            Opcode::GetGlobal => {
                let global_index = self.read_uint16()?;
                if global_index >= self.globals.len() {
                    return Err(Error::Runtime(format!("Global index out of bounds: {}", global_index)));
                }
                self.push(self.globals[global_index].clone())?;
            },
            Opcode::SetGlobal => {
                let global_index = self.read_uint16()?;
                if global_index >= self.globals.len() {
                    self.globals.resize(global_index + 1, Object::Null);
                }
                self.globals[global_index] = self.pop();
            },
            Opcode::GetLocal => {
                let local_index = self.read_uint8()?;
                let base_pointer = self.current_frame().base_pointer;
                if base_pointer + local_index as usize >= self.sp {
                    return Err(Error::Runtime(format!("Local index out of bounds: {}", local_index)));
                }
                self.push(self.stack[base_pointer + local_index as usize].clone())?;
            },
            Opcode::SetLocal => {
                let local_index = self.read_uint8()?;
                let base_pointer = self.current_frame().base_pointer;
                if base_pointer + local_index as usize >= self.sp {
                    return Err(Error::Runtime(format!("Local index out of bounds: {}", local_index)));
                }
                self.stack[base_pointer + local_index as usize] = self.pop();
            },
            Opcode::Call => {
                let num_args = self.read_uint8()?;
                self.execute_call(num_args as usize)?;
            },
            Opcode::ReturnValue => {
                let return_value = self.pop();
                let frame = self.pop_frame();
                self.sp = frame.base_pointer;
                self.push(return_value)?;
            },
            Opcode::Return => {
                let frame = self.pop_frame();
                self.sp = frame.base_pointer;
                self.push(Object::Null)?;
            },
            Opcode::Array => {
                let num_elements = self.read_uint16()? as usize;
                self.execute_build_array(num_elements)?;
            },
            Opcode::Hash => {
                let num_pairs = self.read_uint16()? as usize;
                self.execute_build_hash(num_pairs)?;
            },
            Opcode::Index => {
                self.execute_index_operation()?;
            },
            Opcode::Closure => {
                let const_index = self.read_uint16()? as usize;
                let num_free = self.read_uint8()? as usize;
                self.push_closure(const_index, num_free)?;
            },
            Opcode::GetFree => {
                let free_index = self.read_uint8()? as usize;
                let current_closure = match &self.current_frame().closure {
                    Some(closure) => closure,
                    None => return Err(Error::Runtime("No closure found in current frame".to_string())),
                };
                
                if free_index >= current_closure.free_variables.len() {
                    return Err(Error::Runtime(format!("Free variable index out of bounds: {}", free_index)));
                }
                
                self.push(current_closure.free_variables[free_index].clone())?;
            },
            _ => return Err(Error::Runtime(format!("Unknown opcode: {:?}", op_code))),
        }
        
        // Return the executed opcode and any new value that was pushed to the stack
        let result = if self.sp > old_sp {
            Some(self.stack[self.sp - 1].clone())
        } else {
            None
        };
        
        Ok((op_code, result))
    }

    /// Allocate an object in memory
    pub fn allocate_object<T: Sized + Clone>(&mut self, obj: T) -> Result<NonNull<T>, Error> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let layout = Layout::from_size_align(size, align)
            .map_err(|_| Error::Runtime(format!("Invalid layout for object of size {}", size)))?;
        
        // Allocate memory using the memory manager
        let ptr = self.memory_manager.allocate(layout)?;
        
        // Cast to the right type and initialize
        let typed_ptr = ptr.as_ptr() as *mut T;
        unsafe {
            *typed_ptr = obj.clone();
        }
        
        // Convert to NonNull
        Ok(NonNull::new(typed_ptr)
            .ok_or_else(|| Error::Runtime("Failed to create NonNull pointer".to_string()))?)
    }

    /// Create a new VM with a program string
    pub fn with_program(program: &str) -> Result<Self, Error> {
        let lexer = Lexer::new(program);
        let mut parser = Parser::new(lexer);
        
        let program = parser.parse_program()?;
        
        let mut compiler = Compiler::new();
        let bytecode = compiler.compile(program)?;
        
        Self::new(bytecode)
    }

    /// Execute a program string directly and return the result
    pub fn execute(program: &str) -> Result<Option<Object>, Error> {
        let lexer = Lexer::new(program);
        let mut parser = Parser::new(lexer);
        
        let program = parser.parse_program()?;
        
        let mut compiler = Compiler::new();
        let bytecode = compiler.compile(program)?;
        
        let mut vm = Self::new(bytecode)?;
        vm.run()?;
        
        Ok(vm.last_popped_value())
    }

    /// Collect garbage by running the garbage collector
    pub fn collect_garbage(&mut self) -> Result<(), Error> {
        // Get all roots from the stack
        let mut roots = Vec::new();
        
        // Add local variables from current frame
        for i in 0..self.sp {
            // Extract closures from stack objects
            match &self.stack[i] {
                Object::Closure(closure) => {
                    roots.push(closure.clone());
                },
                _ => {}, // Ignore other object types
            }
        }
        
        // Add globals
        for obj in &self.globals {
            // Extract closures from global objects
            match obj {
                Object::Closure(closure) => {
                    roots.push(closure.clone());
                },
                _ => {}, // Ignore other object types
            }
        }
        
        // Run garbage collection
        self.memory_manager.collect_garbage(&roots)?;
        
        Ok(())
    }

    /// Push an object onto the stack
    pub fn push(&mut self, item: Object) -> Result<(), Error> {
        if self.sp < STACK_SIZE {
            self.stack[self.sp] = item;
            self.sp += 1;
            Ok(())
        } else {
            Err(Error::Runtime("Stack overflow".to_string()))
        }
    }
    
    /// Pop an object from the stack
    pub fn pop(&mut self) -> Object {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp -= 1;
        self.stack[self.sp].clone()
    }
}

/// GC-managed array object
struct GcArray {
    values: Vec<Object>,
}

impl Traceable for GcArray {
    fn trace(&self, _visitor: &mut dyn crate::memory::gc::Visitor) {
        // We don't trace the individual Object values yet
        // This would require making Object traceable
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + self.values.capacity() * std::mem::size_of::<Object>()
    }
}

/// GC-managed string object
struct GcString {
    value: String,
}

impl Traceable for GcString {
    fn trace(&self, _visitor: &mut dyn crate::memory::gc::Visitor) {
        // Strings don't have references to other objects
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + self.value.capacity()
    }
}

impl Traceable for Frame {
    fn trace(&self, _visitor: &mut dyn crate::memory::gc::Visitor) {
        // Implementation details
    }
    
    // ... existing size method
}

impl Traceable for VM {
    fn trace(&self, _visitor: &mut dyn crate::memory::gc::Visitor) {
        // Implementation details 
    }
    
    // ... existing size method
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{self, Expression, Node, Program, Statement};
    use crate::compiler::{Bytecode, Compiler, Object};
    use crate::lexer::Lexer;
    use crate::parser_impl::Parser;
    
    // Test the VM for integer arithmetic expressions
    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5", 10),
            ("5 - 5", 0),
            ("5 * 5", 25),
            ("5 / 5", 1),
            ("5 + 5 * 5", 30),
            ("5 * (5 + 5)", 50),
        ];
        
        for (input, expected) in tests {
            let result = VM::execute(input).expect("VM execution failed");
            
            assert!(result.is_some());
            
            let obj = result.unwrap();
            match obj {
                Object::Integer(i) => assert_eq!(i, expected),
                _ => panic!("Expected integer, got {:?}", obj),
            }
        }
    }
    
    // Test function calls
    #[test]
    fn test_function_calls() {
        let input = r#"
        let add = fn(a, b) { a + b; };
        let result = add(5, 10);
        result;
        "#;
        
        let result = VM::execute(input).expect("VM execution failed");
        assert!(result.is_some());
        
        let obj = result.unwrap();
        match obj {
            Object::Integer(i) => assert_eq!(i, 15),
            _ => panic!("Expected integer, got {:?}", obj),
        }
    }
    
    // Test nested function calls
    #[test]
    fn test_nested_function_calls() {
        let input = r#"
        let add = fn(a, b) { a + b; };
        let apply = fn(f, x, y) { f(x, y); };
        let result = apply(add, 5, 10);
        result;
        "#;
        
        let result = VM::execute(input).expect("VM execution failed");
        assert!(result.is_some());
        
        let obj = result.unwrap();
        match obj {
            Object::Integer(i) => assert_eq!(i, 15),
            _ => panic!("Expected integer, got {:?}", obj),
        }
    }
    
    // Test closures
    #[test]
    fn test_closures() {
        let input = r#"
        let newAdder = fn(x) {
            fn(y) { x + y };
        };
        
        let addTwo = newAdder(2);
        addTwo(3);
        "#;
        
        let result = VM::execute(input).expect("VM execution failed");
        assert!(result.is_some());
        
        let obj = result.unwrap();
        match obj {
            Object::Integer(i) => assert_eq!(i, 5),
            _ => panic!("Expected integer, got {:?}", obj),
        }
    }
    
    // Test array literals and indexing
    #[test]
    fn test_arrays() {
        let input = r#"
        let arr = [1, 2, 3, 4, 5];
        arr[2];
        "#;
        
        let result = VM::execute(input).expect("VM execution failed");
        assert!(result.is_some());
        
        let obj = result.unwrap();
        match obj {
            Object::Integer(i) => assert_eq!(i, 3),
            _ => panic!("Expected integer, got {:?}", obj),
        }
    }
    
    // Test hash map literals and indexing
    #[test]
    fn test_hash_maps() {
        let input = r#"
        let map = {"one": 1, "two": 2, "three": 3};
        map["two"];
        "#;
        
        let result = VM::execute(input).expect("VM execution failed");
        assert!(result.is_some());
        
        let obj = result.unwrap();
        match obj {
            Object::Integer(i) => assert_eq!(i, 2),
            _ => panic!("Expected integer, got {:?}", obj),
        }
    }
    
    // Test comparison operations
    #[test]
    fn test_comparisons() {
        let tests = vec![
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("1 >= 1", true),
            ("1 <= 1", true),
            ("1 > 1", false),
            ("1 < 1", false),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];
        
        for (input, expected) in tests {
            let result = VM::execute(input).expect("VM execution failed");
            
            assert!(result.is_some());
            
            let obj = result.unwrap();
            match obj {
                Object::Boolean(b) => assert_eq!(b, expected, "Failed for input: {}", input),
                _ => panic!("Expected boolean, got {:?}", obj),
            }
        }
    }
    
    // Test conditionals
    #[test]
    fn test_conditionals() {
        let tests = vec![
            ("if (true) { 10 } else { 20 }", 10),
            ("if (false) { 10 } else { 20 }", 20),
            ("if (1 < 2) { 10 } else { 20 }", 10),
            ("if (1 > 2) { 10 } else { 20 }", 20),
            ("if (1 > 2) { 10 }", 0), // No value (null) evaluates to 0 in this test
            ("if (1 < 2) { 10 }", 10),
            ("if (if (false) { 10 } else { 20 } > 15) { 1 } else { 0 }", 1),
        ];
        
        for (input, expected) in tests {
            let result = VM::execute(input).expect("VM execution failed");
            
            assert!(result.is_some());
            
            let obj = result.unwrap();
            match obj {
                Object::Integer(i) => assert_eq!(i, expected, "Failed for input: {}", input),
                Object::Null => assert_eq!(expected, 0, "Failed for input: {}", input),
                _ => panic!("Expected integer or null, got {:?}", obj),
            }
        }
    }
    
    // Test string operations
    #[test]
    fn test_string_operations() {
        let input = r#"
        let firstName = "John";
        let lastName = "Doe";
        let fullName = firstName + " " + lastName;
        fullName;
        "#;
        
        let result = VM::execute(input).expect("VM execution failed");
        assert!(result.is_some());
        
        let obj = result.unwrap();
        match obj {
            Object::String(s) => assert_eq!(s, "John Doe"),
            _ => panic!("Expected string, got {:?}", obj),
        }
    }
    
    // Test error handling
    #[test]
    fn test_error_handling() {
        let tests = vec![
            "5 / 0",                // Division by zero
            "5[0]",                 // Index operation on non-array
            "[1, 2, 3][5]",         // Index out of bounds
            "{\"a\": 1}[5]",        // Invalid hash key
            "fn() { return 1; }(1)", // Wrong number of arguments
        ];
        
        for input in tests {
            let result = VM::execute(input);
            assert!(result.is_err(), "Expected error for input: {}", input);
        }
    }
} 