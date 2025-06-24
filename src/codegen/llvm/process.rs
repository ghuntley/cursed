// Process compilation support for LLVM codegen
use std::collections::HashMap;

/// Process compilation context
#[derive(Debug)]
pub struct ProcessCompilation<'ctx> {
    pub context: &'ctx inkwell::context::Context,
    pub processes: HashMap<String, ProcessInfo>,
}

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub name: String,
    pub entry_point: String,
    pub control_ops: Vec<ProcessControlOp>,
}

/// Process control operations
#[derive(Debug, Clone)]
pub enum ProcessControlOp {
    Start,
    Stop,
    Pause,
    Resume,
    Signal(i32),
    IpcChannel(IpcChannelType),
    SharedMemory(SharedMemoryOp),
}

/// IPC channel types
#[derive(Debug, Clone)]
pub enum IpcChannelType {
    Pipe,
    Socket,
    MessageQueue,
    SharedMemory,
}

/// Shared memory operations
#[derive(Debug, Clone)]
pub enum SharedMemoryOp {
    Create(usize),
    Attach,
    Detach,
    Read(usize, usize),
    Write(usize, Vec<u8>),
}

/// Signal operations
#[derive(Debug, Clone)]
pub enum SignalOp {
    Send(i32, i32),
    Handle(i32),
    Ignore(i32),
    Default(i32),
}

impl<'ctx> ProcessCompilation<'ctx> {
    pub fn new(context: &'ctx inkwell::context::Context) -> Self {
        Self {
            context,
            processes: HashMap::new(),
        }
    }
    
    pub fn register_process(&mut self, name: String, info: ProcessInfo) {
        self.processes.insert(name, info);
    }
    
    pub fn compile_process(&self, _name: &str) -> Result<(), ProcessError> {
        // Stub implementation
        Ok(())
    }
}

/// Process compilation error
#[derive(Debug)]
pub struct ProcessError {
    pub message: String,
}

impl ProcessError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Process error: {}", self.message)
    }
}

impl std::error::Error for ProcessError {}
