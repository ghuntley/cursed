// Test just the memory management modules to verify they compile

mod error {
    #[derive(Debug, Clone)]
    pub struct CursedError(String);
    
    impl CursedError {
        pub fn runtime_error(msg: &str) -> Self {
            Self(msg.to_string())
        }
    }
    
    impl std::fmt::Display for CursedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    impl std::error::Error for CursedError {}
}

mod error_types {
    #[derive(Debug)]
    pub enum Error {
        Runtime(String),
    }
}

mod memory {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Tag {
        Object,
        Array,
        Function,
        String,
        Number,
        Boolean,
        Nil,
        Interface,
        Channel,
        Custom(u32),
    }
    
    pub trait Traceable {
        fn trace(&self, visitor: &mut dyn Visitor);
        fn get_tag(&self) -> Tag;
        fn size(&self) -> usize;
    }
    
    pub trait Visitor {
        fn visit(&mut self, obj: &dyn Traceable);
    }
}

mod runtime {
    pub mod stack {
        use crate::error::CursedError;
        use std::sync::{RwLock, Mutex};
        use std::sync::atomic::AtomicUsize;
        use std::collections::HashMap;
        
        pub type StackId = usize;
        
        #[derive(Debug, Clone)]
        pub struct StackSegment {
            pub base: *mut u8,
            pub size: usize,
            pub used: usize,
            pub guard_page: Option<*mut u8>,
        }
        
        unsafe impl Send for StackSegment {}
        unsafe impl Sync for StackSegment {}
        
        #[derive(Debug, Clone)]
        pub struct StackFrame {
            pub id: usize,
            pub function_name: String,
            pub locals: Vec<*mut u8>,
            pub stack_pointer: *mut u8,
            pub frame_pointer: *mut u8,
        }
        
        unsafe impl Send for StackFrame {}
        unsafe impl Sync for StackFrame {}
        
        #[derive(Debug)]
        pub struct RuntimeStack {
            stacks: RwLock<HashMap<StackId, StackSegment>>,
            frames: RwLock<HashMap<StackId, Vec<StackFrame>>>,
            next_id: AtomicUsize,
        }
        
        #[derive(Debug, Clone)]
        pub struct StackInfo {
            pub id: StackId,
            pub size: usize,
            pub used: usize,
            pub frame_count: usize,
            pub has_guard_page: bool,
        }
        
        impl RuntimeStack {
            pub fn new() -> Self {
                Self {
                    stacks: RwLock::new(HashMap::new()),
                    frames: RwLock::new(HashMap::new()),
                    next_id: AtomicUsize::new(1),
                }
            }
            
            pub fn allocate_stack(&self, _size: Option<usize>) -> Result<StackId, CursedError> {
                Ok(1)
            }
            
            pub fn deallocate_stack(&self, _stack_id: StackId) -> Result<(), CursedError> {
                Ok(())
            }
            
            pub fn get_all_gc_roots(&self) -> Vec<*mut u8> {
                Vec::new()
            }
            
            pub fn get_stack_info(&self, stack_id: StackId) -> Result<StackInfo, CursedError> {
                Ok(StackInfo {
                    id: stack_id,
                    size: 1024,
                    used: 0,
                    frame_count: 0,
                    has_guard_page: false,
                })
            }
        }
    }
    
    pub mod channels {
        #[derive(Debug, Clone)]
        pub enum ChannelError {
            AllocationError(String),
        }
        
        impl std::fmt::Display for ChannelError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ChannelError::AllocationError(msg) => write!(f, "Allocation error: {}", msg),
                }
            }
        }
        
        impl std::error::Error for ChannelError {}
    }
}

// Include the actual GC implementation
include!("src/runtime/gc.rs");

fn main() {
    println!("Memory management modules compiled successfully!");
    
    // Basic test
    use std::sync::Arc;
    let stack_manager = Arc::new(runtime::stack::RuntimeStack::new());
    let config = GcConfig::default();
    
    match GarbageCollector::new(config, stack_manager) {
        Ok(gc) => {
            println!("✓ GarbageCollector created successfully");
            println!("  Initial state: {:?}", gc.get_state());
            
            // Test allocation
            match gc.allocate(64, memory::Tag::Object) {
                Ok(handle) => {
                    println!("✓ Allocated object successfully");
                    println!("  Size: {} bytes", unsafe { (*handle.as_ptr()).metadata.size });
                }
                Err(e) => println!("✗ Allocation failed: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to create GarbageCollector: {}", e),
    }
}
