// VM Constants for stack size, frame count, and memory allocation
pub const STACK_SIZE: usize = 2048;
pub const MAX_FRAMES: usize = 1024;
pub const DEFAULT_MEMORY_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const HEAP_SIZE: usize = 4 * 1024 * 1024; // 4MB
pub const GC_SIZE: usize = 1 * 1024 * 1024; // 1MB 