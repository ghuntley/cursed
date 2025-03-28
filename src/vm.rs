// Virtual Machine implementation for CURSED
use crate::compiler::{Bytecode, Instructions, Opcode};
use crate::error::{Error, SourceLocation};
use crate::object::Object;
use std::collections::HashMap;
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
}

impl VM {
    /// Create a new VM with the given constants and instructions
    pub fn new() -> Self {
        // Initialize empty globals array with capacity
        let mut globals = Vec::with_capacity(constants::GLOBALS_SIZE);
        for _ in 0..constants::GLOBALS_SIZE {
            globals.push(Rc::new(Object::Null));
        }
        
        VM {
            constants: Vec::new(),
            stack: Vec::with_capacity(constants::STACK_SIZE),
            globals,
            frames: Vec::with_capacity(constants::MAX_FRAMES),
            frame_index: 0,
            sp: 0,
        }
    }
    
    /// Create a new VM with the given bytecode
    pub fn with_bytecode(bytecode: Bytecode) -> Self {
        let mut vm = Self::new();
        vm.constants = bytecode.constants.into_iter().map(|obj| Rc::new(obj)).collect();
        
        // Create the main frame with the bytecode instructions
        let main_frame = Frame::new(bytecode.instructions, 0);
        vm.frames.push(main_frame);
        vm.frame_index = 0;
        
        vm
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
            _ => {
                return Err(Error::vm(format!("Calling non-function: {}", fn_obj)));
            }
        }
        
        Ok(())
    }
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
    fn test_function_call() {
        // Create a simple CompiledFunction that returns a constant
        let mut function_instructions = Vec::new();
        
        // Push constant 42 onto the stack (the constant at index 1 in the constants array)
        function_instructions.push(Opcode::Constant as u8);
        function_instructions.push(0); // const index hi byte 
        function_instructions.push(1); // const index lo byte - changed from 0 to 1
        
        // Return the value
        function_instructions.push(Opcode::ReturnValue as u8);
        
        // Create the compiled function object
        let compiled_fn = crate::compiler::CompiledFunction {
            instructions: function_instructions,
            num_locals: 0,
            num_parameters: 0,
            name: Some("test_function".to_string()),
        };
        
        // Create the main program bytecode
        let mut main_instructions = Vec::new();
        
        // Push the function onto the stack (constant at index 0)
        main_instructions.push(Opcode::Constant as u8);
        main_instructions.push(0); // const index hi byte
        main_instructions.push(0); // const index lo byte
        
        // Call the function with 0 arguments
        main_instructions.push(Opcode::Call as u8);
        main_instructions.push(0); // num args hi byte
        main_instructions.push(0); // num args lo byte
        
        // Create the constants for the main program (includes the compiled function)
        let constants = vec![
            Object::CompiledFunction(Rc::new(compiled_fn)),
            Object::Integer(42),
        ];
        
        let bytecode = Bytecode { 
            instructions: main_instructions, 
            constants 
        };
        
        // Create the VM with the bytecode
        let mut vm = VM::with_bytecode(bytecode);
        
        // Run the VM
        match vm.run() {
            Ok(result) => {
                // Verify the result is 42 from the function call
                assert_eq!(*result, Object::Integer(42));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_nested_function_calls() {
        // Create an inner function that returns 42
        let mut inner_function_instructions = Vec::new();
        
        // Push constant 42 onto the stack (constant index 1)
        inner_function_instructions.push(Opcode::Constant as u8);
        inner_function_instructions.push(0); // const index hi byte 
        inner_function_instructions.push(1); // const index lo byte (42)
        
        // Return the value
        inner_function_instructions.push(Opcode::ReturnValue as u8);
        
        let inner_fn = crate::compiler::CompiledFunction {
            instructions: inner_function_instructions,
            num_locals: 0,
            num_parameters: 0,
            name: Some("inner_function".to_string()),
        };
        
        // Create an outer function that calls the inner function
        let mut outer_function_instructions = Vec::new();
        
        // Push the inner function onto the stack (constant at index 0)
        outer_function_instructions.push(Opcode::Constant as u8);
        outer_function_instructions.push(0); // const index hi byte 
        outer_function_instructions.push(0); // const index lo byte (inner function)
        
        // Call the inner function with 0 arguments
        outer_function_instructions.push(Opcode::Call as u8);
        outer_function_instructions.push(0); // num args hi byte
        outer_function_instructions.push(0); // num args lo byte
        
        // Return the value from inner function
        outer_function_instructions.push(Opcode::ReturnValue as u8);
        
        let outer_fn = crate::compiler::CompiledFunction {
            instructions: outer_function_instructions,
            num_locals: 0,
            num_parameters: 0,
            name: Some("outer_function".to_string()),
        };
        
        // Create the main program bytecode
        let mut main_instructions = Vec::new();
        
        // Push the outer function onto the stack (constant at index 2)
        main_instructions.push(Opcode::Constant as u8);
        main_instructions.push(0); // const index hi byte
        main_instructions.push(2); // const index lo byte (outer function)
        
        // Call the outer function with 0 arguments
        main_instructions.push(Opcode::Call as u8);
        main_instructions.push(0); // num args hi byte
        main_instructions.push(0); // num args lo byte
        
        // Create the constants for the main program
        let constants = vec![
            Object::CompiledFunction(Rc::new(inner_fn)),
            Object::Integer(42),
            Object::CompiledFunction(Rc::new(outer_fn)),
        ];
        
        let bytecode = Bytecode { 
            instructions: main_instructions, 
            constants 
        };
        
        // Create the VM with the bytecode
        let mut vm = VM::with_bytecode(bytecode);
        
        // Run the VM
        match vm.run() {
            Ok(result) => {
                // Verify the result is 42 from the nested function calls
                assert_eq!(*result, Object::Integer(42));
            },
            Err(e) => {
                println!("VM execution error: {:?}", e);
                panic!("VM execution failed: {:?}", e);
            }
        }
    }
    
    // Additional tests can be added here
} 