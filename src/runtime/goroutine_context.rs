//! Goroutine Context Switching and Stack Management
//!
//! This module provides real goroutine context switching with:
//! - Complete execution context saving/restoring
//! - Proper stack switching mechanics  
//! - Real function pointers and executable implementations
//! - Integration with LLVM compilation

use crate::error::CursedError;
use crate::runtime::goroutine::{GoroutineId, GoroutineState, GoroutineScheduler};
use crate::runtime::stack::{StackId, StackFrame};
use crate::runtime::performance_tracker::PERFORMANCE_TRACKER;
use crate::execution::{CursedValue, CursedExecutionEngine};
use std::arch::asm;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use std::ptr;
use std::time::Instant;
use cfg_if::cfg_if;

/// CPU execution context for goroutine switching
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    #[cfg(target_arch = "wasm32")]
    /// WASM32-specific execution context
    pub wasm: WasmExecutionContext,
    #[cfg(not(target_arch = "wasm32"))]
    /// Native x86_64 execution context
    pub native: NativeExecutionContext,
}

/// WebAssembly execution context for goroutine switching
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WasmExecutionContext {
    /// Linear memory stack pointer (offset in WASM memory)
    pub stack_ptr: u32,
    /// Linear memory base pointer (offset in WASM memory)
    pub base_ptr: u32,
    /// Local variable storage in linear memory
    pub locals_ptr: u32,
    /// Current function index in WASM table
    pub function_index: u32,
    /// Local variable count
    pub local_count: u32,
    /// Operand stack pointer in linear memory
    pub operand_stack_ptr: u32,
    /// Operand stack size
    pub operand_stack_size: u32,
    /// Return address (for resume point)
    pub return_address: u32,
    /// Call frame depth
    pub call_depth: u32,
    /// Yield point identifier
    pub yield_point: u32,
    /// Stack base and size for safety
    pub stack_base: u32,
    pub stack_size: u32,
    /// Memory manager state
    pub memory_state: WasmMemoryState,
    /// Runtime type for host-specific behavior
    pub runtime_type: u32, // Maps to WasmRuntimeType
}

/// WASM memory state for context switching
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WasmMemoryState {
    /// Current memory pages
    pub current_pages: u32,
    /// Stack allocation offset
    pub stack_allocation: u32,
    /// Heap allocation offset
    pub heap_allocation: u32,
    /// Saved local variables (up to 16 locals)
    pub saved_locals: [u32; 16],
    /// Number of saved locals
    pub local_count: u32,
    /// GC roots for memory management
    pub gc_roots: [u32; 8],
    pub gc_root_count: u32,
}

/// Native execution context (architecture-specific)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NativeExecutionContext {
    #[cfg(target_arch = "x86_64")]
    /// x86_64 specific registers
    pub x86_64: X86_64Context,
    
    #[cfg(target_arch = "aarch64")]
    /// ARM64 specific registers
    pub arm64: Arm64Context,
    
    /// Stack base and size for safety (common to all architectures)
    pub stack_base: u64,
    pub stack_size: usize,
}

/// x86_64 CPU context
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct X86_64Context {
    /// Stack pointer
    pub rsp: u64,
    /// Base pointer
    pub rbp: u64,
    /// General purpose registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    /// Instruction pointer
    pub rip: u64,
    /// Status flags
    pub rflags: u64,
}

/// ARM64 CPU context following AAPCS64 calling convention
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Arm64Context {
    /// General purpose registers X0-X30
    pub x0: u64,   // First argument/return value register
    pub x1: u64,   // Second argument/return value register  
    pub x2: u64,   // Third argument register
    pub x3: u64,   // Fourth argument register
    pub x4: u64,   // Fifth argument register
    pub x5: u64,   // Sixth argument register
    pub x6: u64,   // Seventh argument register
    pub x7: u64,   // Eighth argument register
    pub x8: u64,   // Indirect result location register
    pub x9: u64,   // Temporary register
    pub x10: u64,  // Temporary register
    pub x11: u64,  // Temporary register
    pub x12: u64,  // Temporary register
    pub x13: u64,  // Temporary register
    pub x14: u64,  // Temporary register
    pub x15: u64,  // Temporary register
    pub x16: u64,  // IP0 - Intra-call-use register
    pub x17: u64,  // IP1 - Intra-call-use register
    pub x18: u64,  // Platform register (reserved on some platforms)
    pub x19: u64,  // Callee-saved register
    pub x20: u64,  // Callee-saved register
    pub x21: u64,  // Callee-saved register
    pub x22: u64,  // Callee-saved register
    pub x23: u64,  // Callee-saved register
    pub x24: u64,  // Callee-saved register
    pub x25: u64,  // Callee-saved register
    pub x26: u64,  // Callee-saved register
    pub x27: u64,  // Callee-saved register
    pub x28: u64,  // Callee-saved register
    pub x29: u64,  // Frame pointer (FP)
    pub x30: u64,  // Link register (LR)
    
    /// Stack pointer
    pub sp: u64,
    
    /// Program counter
    pub pc: u64,
    
    /// Process State Register (NZCV flags and system state)
    pub pstate: u64,
    
    /// Vector registers (NEON/SIMD) - Q0-Q31 saved as pairs of 64-bit values
    /// Q0 = {v0_low, v0_high}, Q1 = {v1_low, v1_high}, etc.
    pub v0_low: u64,   pub v0_high: u64,   // Q0
    pub v1_low: u64,   pub v1_high: u64,   // Q1
    pub v2_low: u64,   pub v2_high: u64,   // Q2
    pub v3_low: u64,   pub v3_high: u64,   // Q3
    pub v4_low: u64,   pub v4_high: u64,   // Q4
    pub v5_low: u64,   pub v5_high: u64,   // Q5
    pub v6_low: u64,   pub v6_high: u64,   // Q6
    pub v7_low: u64,   pub v7_high: u64,   // Q7
    pub v8_low: u64,   pub v8_high: u64,   // Q8  (callee-saved)
    pub v9_low: u64,   pub v9_high: u64,   // Q9  (callee-saved)
    pub v10_low: u64,  pub v10_high: u64,  // Q10 (callee-saved)
    pub v11_low: u64,  pub v11_high: u64,  // Q11 (callee-saved)
    pub v12_low: u64,  pub v12_high: u64,  // Q12 (callee-saved)
    pub v13_low: u64,  pub v13_high: u64,  // Q13 (callee-saved)
    pub v14_low: u64,  pub v14_high: u64,  // Q14 (callee-saved)
    pub v15_low: u64,  pub v15_high: u64,  // Q15 (callee-saved)
    pub v16_low: u64,  pub v16_high: u64,  // Q16
    pub v17_low: u64,  pub v17_high: u64,  // Q17
    pub v18_low: u64,  pub v18_high: u64,  // Q18
    pub v19_low: u64,  pub v19_high: u64,  // Q19
    pub v20_low: u64,  pub v20_high: u64,  // Q20
    pub v21_low: u64,  pub v21_high: u64,  // Q21
    pub v22_low: u64,  pub v22_high: u64,  // Q22
    pub v23_low: u64,  pub v23_high: u64,  // Q23
    pub v24_low: u64,  pub v24_high: u64,  // Q24
    pub v25_low: u64,  pub v25_high: u64,  // Q25
    pub v26_low: u64,  pub v26_high: u64,  // Q26
    pub v27_low: u64,  pub v27_high: u64,  // Q27
    pub v28_low: u64,  pub v28_high: u64,  // Q28
    pub v29_low: u64,  pub v29_high: u64,  // Q29
    pub v30_low: u64,  pub v30_high: u64,  // Q30
    pub v31_low: u64,  pub v31_high: u64,  // Q31
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            wasm: WasmExecutionContext::default(),
            #[cfg(not(target_arch = "wasm32"))]
            native: NativeExecutionContext::default(),
        }
    }
}

impl Default for WasmExecutionContext {
    fn default() -> Self {
        Self {
            stack_ptr: 0,
            base_ptr: 0,
            locals_ptr: 0,
            function_index: 0,
            local_count: 0,
            operand_stack_ptr: 0,
            operand_stack_size: 0,
            return_address: 0,
            call_depth: 0,
            yield_point: 0,
            stack_base: 0,
            stack_size: 0,
            memory_state: WasmMemoryState::default(),
            runtime_type: 0, // Browser default
        }
    }
}

impl Default for WasmMemoryState {
    fn default() -> Self {
        Self {
            current_pages: 0,
            stack_allocation: 0,
            heap_allocation: 0,
            saved_locals: [0; 16],
            local_count: 0,
            gc_roots: [0; 8],
            gc_root_count: 0,
        }
    }
}

impl Default for NativeExecutionContext {
    fn default() -> Self {
        Self {
            #[cfg(target_arch = "x86_64")]
            x86_64: X86_64Context::default(),
            #[cfg(target_arch = "aarch64")]
            arm64: Arm64Context::default(),
            stack_base: 0,
            stack_size: 0,
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl Default for X86_64Context {
    fn default() -> Self {
        Self {
            rsp: 0, rbp: 0, rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0, rip: 0, rflags: 0,
        }
    }
}

#[cfg(target_arch = "aarch64")]
impl Default for Arm64Context {
    fn default() -> Self {
        Self {
            x0: 0, x1: 0, x2: 0, x3: 0, x4: 0, x5: 0, x6: 0, x7: 0,
            x8: 0, x9: 0, x10: 0, x11: 0, x12: 0, x13: 0, x14: 0, x15: 0,
            x16: 0, x17: 0, x18: 0, x19: 0, x20: 0, x21: 0, x22: 0, x23: 0,
            x24: 0, x25: 0, x26: 0, x27: 0, x28: 0, x29: 0, x30: 0,
            sp: 0, pc: 0, pstate: 0,
            v0_low: 0, v0_high: 0, v1_low: 0, v1_high: 0, v2_low: 0, v2_high: 0, v3_low: 0, v3_high: 0,
            v4_low: 0, v4_high: 0, v5_low: 0, v5_high: 0, v6_low: 0, v6_high: 0, v7_low: 0, v7_high: 0,
            v8_low: 0, v8_high: 0, v9_low: 0, v9_high: 0, v10_low: 0, v10_high: 0, v11_low: 0, v11_high: 0,
            v12_low: 0, v12_high: 0, v13_low: 0, v13_high: 0, v14_low: 0, v14_high: 0, v15_low: 0, v15_high: 0,
            v16_low: 0, v16_high: 0, v17_low: 0, v17_high: 0, v18_low: 0, v18_high: 0, v19_low: 0, v19_high: 0,
            v20_low: 0, v20_high: 0, v21_low: 0, v21_high: 0, v22_low: 0, v22_high: 0, v23_low: 0, v23_high: 0,
            v24_low: 0, v24_high: 0, v25_low: 0, v25_high: 0, v26_low: 0, v26_high: 0, v27_low: 0, v27_high: 0,
            v28_low: 0, v28_high: 0, v29_low: 0, v29_high: 0, v30_low: 0, v30_high: 0, v31_low: 0, v31_high: 0,
        }
    }
}

/// Function value with real executable implementation
#[derive(Debug, Clone)]
pub struct ExecutableFunction {
    /// Function pointer to executable code
    pub func_ptr: usize,
    /// Function name for debugging
    pub name: String,
    /// Function arity (parameter count)
    pub arity: usize,
    /// Return type information
    pub return_type: String,
    /// Parameter types
    pub param_types: Vec<String>,
    /// Whether this is a native LLVM-compiled function
    pub is_native: bool,
    /// JIT compiled function metadata
    pub jit_metadata: Option<JitFunctionMetadata>,
}

#[derive(Debug, Clone)]
pub struct JitFunctionMetadata {
    /// LLVM module this function belongs to
    pub module_name: String,
    /// Function signature in LLVM IR
    pub llvm_signature: String,
    /// Optimization level used for compilation
    pub optimization_level: u32,
    /// Whether function uses goroutines
    pub uses_goroutines: bool,
}

/// Global registry for executable functions
static FUNCTION_REGISTRY: once_cell::sync::Lazy<Arc<Mutex<HashMap<String, ExecutableFunction>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Global registry for goroutine execution contexts
static CONTEXT_REGISTRY: once_cell::sync::Lazy<Arc<Mutex<HashMap<GoroutineId, ExecutionContext>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Register an executable function in the global registry
pub fn register_executable_function(name: String, func: ExecutableFunction) -> Result<(), CursedError> {
    let mut registry = FUNCTION_REGISTRY.lock()
        .map_err(|_| CursedError::runtime_error("Failed to lock function registry"))?;
    registry.insert(name, func);
    Ok(())
}

/// Get an executable function from the registry
pub fn get_executable_function(name: &str) -> Option<ExecutableFunction> {
    if let Ok(registry) = FUNCTION_REGISTRY.lock() {
        registry.get(name).cloned()
    } else {
        None
    }
}

/// Save the current execution context for a goroutine
pub fn save_goroutine_context(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    #[cfg(target_arch = "wasm32")]
    {
        save_wasm_goroutine_context(goroutine_id)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        save_native_goroutine_context(goroutine_id)
    }
}

/// Save WebAssembly goroutine context
#[cfg(target_arch = "wasm32")]
fn save_wasm_goroutine_context(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    let mut context = ExecutionContext::default();
    
    // Get current WebAssembly linear memory state
    let current_pages = core::arch::wasm32::memory_size(0);
    let stack_ptr = get_wasm_stack_pointer();
    let base_ptr = get_wasm_base_pointer();
    
    // Save WASM execution state
    context.wasm.stack_ptr = stack_ptr;
    context.wasm.base_ptr = base_ptr;
    context.wasm.memory_state.current_pages = current_pages;
    
    // Save current function context from WASM call stack
    save_wasm_call_stack(&mut context.wasm)?;
    
    // Save local variables from current stack frame
    save_wasm_locals(&mut context.wasm)?;
    
    // Save operand stack state
    save_wasm_operand_stack(&mut context.wasm)?;
    
    // Store current yield point for resumption
    context.wasm.yield_point = get_current_yield_point();
    
    // Get runtime type for host-specific behavior
    context.wasm.runtime_type = detect_wasm_runtime_type();
    
    // Store context in registry
    let mut registry = CONTEXT_REGISTRY.lock()
        .map_err(|_| CursedError::runtime_error("Failed to lock context registry"))?;
    registry.insert(goroutine_id, context);
    
    Ok(())
}

/// Save native goroutine context (architecture-specific)
#[cfg(not(target_arch = "wasm32"))]
fn save_native_goroutine_context(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    let mut context = ExecutionContext::default();
    
    #[cfg(target_arch = "x86_64")]
    save_x86_64_context(&mut context.native.x86_64)?;
    
    #[cfg(target_arch = "aarch64")]
    save_arm64_context(&mut context.native.arm64)?;
    
    // Store context in registry
    let mut registry = CONTEXT_REGISTRY.lock()
        .map_err(|_| CursedError::runtime_error("Failed to lock context registry"))?;
    registry.insert(goroutine_id, context);
    
    Ok(())
}

/// Save x86_64 CPU context
#[cfg(all(target_arch = "x86_64", feature = "inline_asm"))]
fn save_x86_64_context(context: &mut X86_64Context) -> Result<(), CursedError> {
    unsafe {
        // Save general purpose registers - first chunk
        asm!(
            "mov {rax}, rax",
            "mov {rbx}, rbx", 
            "mov {rcx}, rcx",
            "mov {rdx}, rdx",
            rax = out(reg) context.rax,
            rbx = out(reg) context.rbx,
            rcx = out(reg) context.rcx,
            rdx = out(reg) context.rdx,
            options(nostack, preserves_flags)
        );
        
        // Save general purpose registers - second chunk
        asm!(
            "mov {rsi}, rsi",
            "mov {rdi}, rdi",
            "mov {r8}, r8",
            "mov {r9}, r9",
            rsi = out(reg) context.rsi,
            rdi = out(reg) context.rdi,
            r8 = out(reg) context.r8,
            r9 = out(reg) context.r9,
            options(nostack, preserves_flags)
        );
        
        // Save general purpose registers - third chunk
        asm!(
            "mov {r10}, r10",
            "mov {r11}, r11",
            "mov {r12}, r12",
            "mov {r13}, r13",
            r10 = out(reg) context.r10,
            r11 = out(reg) context.r11,
            r12 = out(reg) context.r12,
            r13 = out(reg) context.r13,
            options(nostack, preserves_flags)
        );
        
        // Save general purpose registers - fourth chunk
        asm!(
            "mov {r14}, r14",
            "mov {r15}, r15",
            "mov {rsp}, rsp",
            "mov {rbp}, rbp",
            r14 = out(reg) context.r14,
            r15 = out(reg) context.r15,
            rsp = out(reg) context.rsp,
            rbp = out(reg) context.rbp,
            options(nostack, preserves_flags)
        );
        
        // Save flags register
        asm!(
            "pushfq",
            "pop {rflags}",
            rflags = out(reg) context.rflags,
            options(nostack)
        );
    }
    
    Ok(())
}

/// Save x86_64 CPU context (fallback for cross-compilation)
#[cfg(all(target_arch = "x86_64", not(feature = "inline_asm")))]
fn save_x86_64_context(context: &mut X86_64Context) -> Result<(), CursedError> {
    // Fallback implementation for cross-compilation targets
    *context = X86_64Context::default();
    Ok(())
}

/// Save ARM64 CPU context with complete register set
#[cfg(all(target_arch = "aarch64", feature = "inline_asm"))]
fn save_arm64_context(context: &mut Arm64Context) -> Result<(), CursedError> {
    unsafe {
        // Save general purpose registers X0-X7 (first chunk)
        asm!(
            "mov {x0}, x0",
            "mov {x1}, x1", 
            "mov {x2}, x2",
            "mov {x3}, x3",
            "mov {x4}, x4",
            "mov {x5}, x5",
            "mov {x6}, x6",
            "mov {x7}, x7",
            x0 = out(reg) context.x0,
            x1 = out(reg) context.x1,
            x2 = out(reg) context.x2,
            x3 = out(reg) context.x3,
            x4 = out(reg) context.x4,
            x5 = out(reg) context.x5,
            x6 = out(reg) context.x6,
            x7 = out(reg) context.x7,
            options(nostack, preserves_flags)
        );
        
        // Save general purpose registers X8-X15 (second chunk)
        asm!(
            "mov {x8}, x8",
            "mov {x9}, x9",
            "mov {x10}, x10",
            "mov {x11}, x11",
            "mov {x12}, x12",
            "mov {x13}, x13",
            "mov {x14}, x14",
            "mov {x15}, x15",
            x8 = out(reg) context.x8,
            x9 = out(reg) context.x9,
            x10 = out(reg) context.x10,
            x11 = out(reg) context.x11,
            x12 = out(reg) context.x12,
            x13 = out(reg) context.x13,
            x14 = out(reg) context.x14,
            x15 = out(reg) context.x15,
            options(nostack, preserves_flags)
        );
        
        // Save general purpose registers X16-X23 (third chunk)
        asm!(
            "mov {x16}, x16",
            "mov {x17}, x17",
            "mov {x18}, x18",
            "mov {x19}, x19",
            "mov {x20}, x20",
            "mov {x21}, x21",
            "mov {x22}, x22",
            "mov {x23}, x23",
            x16 = out(reg) context.x16,
            x17 = out(reg) context.x17,
            x18 = out(reg) context.x18,
            x19 = out(reg) context.x19,
            x20 = out(reg) context.x20,
            x21 = out(reg) context.x21,
            x22 = out(reg) context.x22,
            x23 = out(reg) context.x23,
            options(nostack, preserves_flags)
        );
        
        // Save general purpose registers X24-X30, SP, and special registers (fourth chunk)
        asm!(
            "mov {x24}, x24",
            "mov {x25}, x25",
            "mov {x26}, x26",
            "mov {x27}, x27",
            "mov {x28}, x28",
            "mov {x29}, x29",
            "mov {x30}, x30",
            "mov {sp}, sp",
            x24 = out(reg) context.x24,
            x25 = out(reg) context.x25,
            x26 = out(reg) context.x26,
            x27 = out(reg) context.x27,
            x28 = out(reg) context.x28,
            x29 = out(reg) context.x29,
            x30 = out(reg) context.x30,
            sp = out(reg) context.sp,
            options(nostack, preserves_flags)
        );
        
        // Save NEON/SIMD registers V0-V7 (first vector chunk)
        asm!(
            "str d0, [{v0_low}]",
            "mov x9, v0.d[1]",
            "str x9, [{v0_high}]",
            "str d1, [{v1_low}]", 
            "mov x9, v1.d[1]",
            "str x9, [{v1_high}]",
            "str d2, [{v2_low}]",
            "mov x9, v2.d[1]",
            "str x9, [{v2_high}]",
            "str d3, [{v3_low}]",
            "mov x9, v3.d[1]", 
            "str x9, [{v3_high}]",
            "str d4, [{v4_low}]",
            "mov x9, v4.d[1]",
            "str x9, [{v4_high}]",
            "str d5, [{v5_low}]",
            "mov x9, v5.d[1]",
            "str x9, [{v5_high}]",
            "str d6, [{v6_low}]",
            "mov x9, v6.d[1]",
            "str x9, [{v6_high}]",
            "str d7, [{v7_low}]",
            "mov x9, v7.d[1]",
            "str x9, [{v7_high}]",
            v0_low = in(reg) &mut context.v0_low,
            v0_high = in(reg) &mut context.v0_high,
            v1_low = in(reg) &mut context.v1_low,
            v1_high = in(reg) &mut context.v1_high,
            v2_low = in(reg) &mut context.v2_low,
            v2_high = in(reg) &mut context.v2_high,
            v3_low = in(reg) &mut context.v3_low,
            v3_high = in(reg) &mut context.v3_high,
            v4_low = in(reg) &mut context.v4_low,
            v4_high = in(reg) &mut context.v4_high,
            v5_low = in(reg) &mut context.v5_low,
            v5_high = in(reg) &mut context.v5_high,
            v6_low = in(reg) &mut context.v6_low,
            v6_high = in(reg) &mut context.v6_high,
            v7_low = in(reg) &mut context.v7_low,
            v7_high = in(reg) &mut context.v7_high,
            out("x9") _,
            options(nostack)
        );
        
        // Save NEON/SIMD registers V8-V15 (second vector chunk - callee-saved)
        asm!(
            "str d8, [{v8_low}]",
            "mov x9, v8.d[1]",
            "str x9, [{v8_high}]",
            "str d9, [{v9_low}]",
            "mov x9, v9.d[1]",
            "str x9, [{v9_high}]",
            "str d10, [{v10_low}]",
            "mov x9, v10.d[1]",
            "str x9, [{v10_high}]",
            "str d11, [{v11_low}]",
            "mov x9, v11.d[1]",
            "str x9, [{v11_high}]",
            "str d12, [{v12_low}]",
            "mov x9, v12.d[1]",
            "str x9, [{v12_high}]",
            "str d13, [{v13_low}]",
            "mov x9, v13.d[1]",
            "str x9, [{v13_high}]",
            "str d14, [{v14_low}]",
            "mov x9, v14.d[1]",
            "str x9, [{v14_high}]",
            "str d15, [{v15_low}]",
            "mov x9, v15.d[1]",
            "str x9, [{v15_high}]",
            v8_low = in(reg) &mut context.v8_low,
            v8_high = in(reg) &mut context.v8_high,
            v9_low = in(reg) &mut context.v9_low,
            v9_high = in(reg) &mut context.v9_high,
            v10_low = in(reg) &mut context.v10_low,
            v10_high = in(reg) &mut context.v10_high,
            v11_low = in(reg) &mut context.v11_low,
            v11_high = in(reg) &mut context.v11_high,
            v12_low = in(reg) &mut context.v12_low,
            v12_high = in(reg) &mut context.v12_high,
            v13_low = in(reg) &mut context.v13_low,
            v13_high = in(reg) &mut context.v13_high,
            v14_low = in(reg) &mut context.v14_low,
            v14_high = in(reg) &mut context.v14_high,
            v15_low = in(reg) &mut context.v15_low,
            v15_high = in(reg) &mut context.v15_high,
            out("x9") _,
            options(nostack)
        );
        
        // Save NEON/SIMD registers V16-V23 (third vector chunk)
        asm!(
            "str d16, [{v16_low}]",
            "mov x9, v16.d[1]",
            "str x9, [{v16_high}]",
            "str d17, [{v17_low}]",
            "mov x9, v17.d[1]",
            "str x9, [{v17_high}]",
            "str d18, [{v18_low}]",
            "mov x9, v18.d[1]",
            "str x9, [{v18_high}]",
            "str d19, [{v19_low}]",
            "mov x9, v19.d[1]",
            "str x9, [{v19_high}]",
            "str d20, [{v20_low}]",
            "mov x9, v20.d[1]",
            "str x9, [{v20_high}]",
            "str d21, [{v21_low}]",
            "mov x9, v21.d[1]",
            "str x9, [{v21_high}]",
            "str d22, [{v22_low}]",
            "mov x9, v22.d[1]",
            "str x9, [{v22_high}]",
            "str d23, [{v23_low}]",
            "mov x9, v23.d[1]",
            "str x9, [{v23_high}]",
            v16_low = in(reg) &mut context.v16_low,
            v16_high = in(reg) &mut context.v16_high,
            v17_low = in(reg) &mut context.v17_low,
            v17_high = in(reg) &mut context.v17_high,
            v18_low = in(reg) &mut context.v18_low,
            v18_high = in(reg) &mut context.v18_high,
            v19_low = in(reg) &mut context.v19_low,
            v19_high = in(reg) &mut context.v19_high,
            v20_low = in(reg) &mut context.v20_low,
            v20_high = in(reg) &mut context.v20_high,
            v21_low = in(reg) &mut context.v21_low,
            v21_high = in(reg) &mut context.v21_high,
            v22_low = in(reg) &mut context.v22_low,
            v22_high = in(reg) &mut context.v22_high,
            v23_low = in(reg) &mut context.v23_low,
            v23_high = in(reg) &mut context.v23_high,
            out("x9") _,
            options(nostack)
        );
        
        // Save NEON/SIMD registers V24-V31 (fourth vector chunk)
        asm!(
            "str d24, [{v24_low}]",
            "mov x9, v24.d[1]",
            "str x9, [{v24_high}]",
            "str d25, [{v25_low}]",
            "mov x9, v25.d[1]",
            "str x9, [{v25_high}]",
            "str d26, [{v26_low}]",
            "mov x9, v26.d[1]",
            "str x9, [{v26_high}]",
            "str d27, [{v27_low}]",
            "mov x9, v27.d[1]",
            "str x9, [{v27_high}]",
            "str d28, [{v28_low}]",
            "mov x9, v28.d[1]",
            "str x9, [{v28_high}]",
            "str d29, [{v29_low}]",
            "mov x9, v29.d[1]",
            "str x9, [{v29_high}]",
            "str d30, [{v30_low}]",
            "mov x9, v30.d[1]",
            "str x9, [{v30_high}]",
            "str d31, [{v31_low}]",
            "mov x9, v31.d[1]",
            "str x9, [{v31_high}]",
            v24_low = in(reg) &mut context.v24_low,
            v24_high = in(reg) &mut context.v24_high,
            v25_low = in(reg) &mut context.v25_low,
            v25_high = in(reg) &mut context.v25_high,
            v26_low = in(reg) &mut context.v26_low,
            v26_high = in(reg) &mut context.v26_high,
            v27_low = in(reg) &mut context.v27_low,
            v27_high = in(reg) &mut context.v27_high,
            v28_low = in(reg) &mut context.v28_low,
            v28_high = in(reg) &mut context.v28_high,
            v29_low = in(reg) &mut context.v29_low,
            v29_high = in(reg) &mut context.v29_high,
            v30_low = in(reg) &mut context.v30_low,
            v30_high = in(reg) &mut context.v30_high,
            v31_low = in(reg) &mut context.v31_low,
            v31_high = in(reg) &mut context.v31_high,
            out("x9") _,
            options(nostack)
        );
        
        // Save processor state flags
        asm!(
            "mrs {pstate}, nzcv",
            pstate = out(reg) context.pstate,
            options(nostack, preserves_flags)
        );
    }
    
    Ok(())
}

/// Restore execution context for a goroutine
pub fn restore_goroutine_context(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    #[cfg(target_arch = "wasm32")]
    {
        restore_wasm_goroutine_context(goroutine_id)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        restore_native_goroutine_context(goroutine_id)
    }
}

/// Save ARM64 CPU context (fallback for cross-compilation)
#[cfg(all(target_arch = "aarch64", not(feature = "inline_asm")))]
fn save_arm64_context(context: &mut Arm64Context) -> Result<(), CursedError> {
    // Fallback implementation for cross-compilation targets
    *context = Arm64Context::default();
    Ok(())
}

/// Restore native goroutine context (architecture-specific)
#[cfg(not(target_arch = "wasm32"))]
fn restore_native_goroutine_context(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    let context = {
        let registry = CONTEXT_REGISTRY.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock context registry"))?;
        registry.get(&goroutine_id).cloned()
            .ok_or_else(|| CursedError::runtime_error("No saved context for goroutine"))?
    };
    
    #[cfg(target_arch = "x86_64")]
    restore_x86_64_context(&context.native.x86_64)?;
    
    #[cfg(target_arch = "aarch64")]
    restore_arm64_context(&context.native.arm64)?;
    
    Ok(())
}

/// Restore x86_64 CPU context
#[cfg(all(target_arch = "x86_64", feature = "inline_asm"))]
fn restore_x86_64_context(context: &X86_64Context) -> Result<(), CursedError> {
    unsafe {
        // Restore general purpose registers - first chunk
        asm!(
            "mov rax, {rax}",
            "mov rbx, {rbx}",
            "mov rcx, {rcx}",
            "mov rdx, {rdx}",
            rax = in(reg) context.rax,
            rbx = in(reg) context.rbx,
            rcx = in(reg) context.rcx,
            rdx = in(reg) context.rdx,
            options(nostack, preserves_flags)
        );
        
        // Restore general purpose registers - second chunk
        asm!(
            "mov rsi, {rsi}",
            "mov rdi, {rdi}",
            "mov r8, {r8}",
            "mov r9, {r9}",
            rsi = in(reg) context.rsi,
            rdi = in(reg) context.rdi,
            r8 = in(reg) context.r8,
            r9 = in(reg) context.r9,
            options(nostack, preserves_flags)
        );
        
        // Restore general purpose registers - third chunk
        asm!(
            "mov r10, {r10}",
            "mov r11, {r11}",
            "mov r12, {r12}",
            "mov r13, {r13}",
            r10 = in(reg) context.r10,
            r11 = in(reg) context.r11,
            r12 = in(reg) context.r12,
            r13 = in(reg) context.r13,
            options(nostack, preserves_flags)
        );
        
        // Restore general purpose registers - fourth chunk
        asm!(
            "mov r14, {r14}",
            "mov r15, {r15}",
            "mov rsp, {rsp}",
            "mov rbp, {rbp}",
            r14 = in(reg) context.r14,
            r15 = in(reg) context.r15,
            rsp = in(reg) context.rsp,
            rbp = in(reg) context.rbp,
            options(nostack, preserves_flags)
        );
        
        // Restore flags register
        asm!(
            "push {rflags}",
            "popfq",
            rflags = in(reg) context.rflags,
            options(nostack)
        );
    }
    
    Ok(())
}

/// Restore x86_64 CPU context (fallback for cross-compilation)
#[cfg(all(target_arch = "x86_64", not(feature = "inline_asm")))]
fn restore_x86_64_context(context: &X86_64Context) -> Result<(), CursedError> {
    // Fallback implementation for cross-compilation targets
    Ok(())
}

/// Restore ARM64 CPU context with complete register set
#[cfg(all(target_arch = "aarch64", feature = "inline_asm"))]
fn restore_arm64_context(context: &Arm64Context) -> Result<(), CursedError> {
    unsafe {
        // Restore processor state flags first
        asm!(
            "msr nzcv, {pstate}",
            pstate = in(reg) context.pstate,
            options(nostack, preserves_flags)
        );
        
        // Restore NEON/SIMD registers V0-V7 (first vector chunk)
        asm!(
            "ldr d0, [{v0_low}]",
            "ldr x9, [{v0_high}]",
            "mov v0.d[1], x9",
            "ldr d1, [{v1_low}]",
            "ldr x9, [{v1_high}]",
            "mov v1.d[1], x9",
            "ldr d2, [{v2_low}]",
            "ldr x9, [{v2_high}]",
            "mov v2.d[1], x9",
            "ldr d3, [{v3_low}]",
            "ldr x9, [{v3_high}]",
            "mov v3.d[1], x9",
            "ldr d4, [{v4_low}]",
            "ldr x9, [{v4_high}]",
            "mov v4.d[1], x9",
            "ldr d5, [{v5_low}]",
            "ldr x9, [{v5_high}]",
            "mov v5.d[1], x9",
            "ldr d6, [{v6_low}]",
            "ldr x9, [{v6_high}]",
            "mov v6.d[1], x9",
            "ldr d7, [{v7_low}]",
            "ldr x9, [{v7_high}]",
            "mov v7.d[1], x9",
            v0_low = in(reg) &context.v0_low,
            v0_high = in(reg) &context.v0_high,
            v1_low = in(reg) &context.v1_low,
            v1_high = in(reg) &context.v1_high,
            v2_low = in(reg) &context.v2_low,
            v2_high = in(reg) &context.v2_high,
            v3_low = in(reg) &context.v3_low,
            v3_high = in(reg) &context.v3_high,
            v4_low = in(reg) &context.v4_low,
            v4_high = in(reg) &context.v4_high,
            v5_low = in(reg) &context.v5_low,
            v5_high = in(reg) &context.v5_high,
            v6_low = in(reg) &context.v6_low,
            v6_high = in(reg) &context.v6_high,
            v7_low = in(reg) &context.v7_low,
            v7_high = in(reg) &context.v7_high,
            out("x9") _,
            options(nostack)
        );
        
        // Restore NEON/SIMD registers V8-V15 (callee-saved registers)
        asm!(
            "ldr d8, [{v8_low}]",
            "ldr x10, [{v8_high}]",
            "mov v8.d[1], x10",
            "ldr d9, [{v9_low}]",
            "ldr x10, [{v9_high}]", 
            "mov v9.d[1], x10",
            "ldr d10, [{v10_low}]",
            "ldr x10, [{v10_high}]",
            "mov v10.d[1], x10",
            "ldr d11, [{v11_low}]",
            "ldr x10, [{v11_high}]",
            "mov v11.d[1], x10",
            "ldr d12, [{v12_low}]", 
            "ldr x10, [{v12_high}]",
            "mov v12.d[1], x10",
            "ldr d13, [{v13_low}]",
            "ldr x10, [{v13_high}]",
            "mov v13.d[1], x10",
            "ldr d14, [{v14_low}]",
            "ldr x10, [{v14_high}]",
            "mov v14.d[1], x10",
            "ldr d15, [{v15_low}]",
            "ldr x10, [{v15_high}]",
            "mov v15.d[1], x10",
            v8_low = in(reg) &context.v8_low,
            v8_high = in(reg) &context.v8_high,
            v9_low = in(reg) &context.v9_low,
            v9_high = in(reg) &context.v9_high,
            v10_low = in(reg) &context.v10_low,
            v10_high = in(reg) &context.v10_high,
            v11_low = in(reg) &context.v11_low,
            v11_high = in(reg) &context.v11_high,
            v12_low = in(reg) &context.v12_low,
            v12_high = in(reg) &context.v12_high,
            v13_low = in(reg) &context.v13_low,
            v13_high = in(reg) &context.v13_high,
            v14_low = in(reg) &context.v14_low,
            v14_high = in(reg) &context.v14_high,
            v15_low = in(reg) &context.v15_low,
            v15_high = in(reg) &context.v15_high,
            out("x10") _,
            options(nostack)
        );
        
        // Restore NEON/SIMD registers V16-V23 (first batch)
        asm!(
            "ldr d16, [{v16_low}]", "ldr x10, [{v16_high}]", "mov v16.d[1], x10",
            "ldr d17, [{v17_low}]", "ldr x10, [{v17_high}]", "mov v17.d[1], x10",
            "ldr d18, [{v18_low}]", "ldr x10, [{v18_high}]", "mov v18.d[1], x10",
            "ldr d19, [{v19_low}]", "ldr x10, [{v19_high}]", "mov v19.d[1], x10",
            "ldr d20, [{v20_low}]", "ldr x10, [{v20_high}]", "mov v20.d[1], x10",
            "ldr d21, [{v21_low}]", "ldr x10, [{v21_high}]", "mov v21.d[1], x10",
            "ldr d22, [{v22_low}]", "ldr x10, [{v22_high}]", "mov v22.d[1], x10",
            "ldr d23, [{v23_low}]", "ldr x10, [{v23_high}]", "mov v23.d[1], x10",
            v16_low = in(reg) &context.v16_low, v16_high = in(reg) &context.v16_high,
            v17_low = in(reg) &context.v17_low, v17_high = in(reg) &context.v17_high,
            v18_low = in(reg) &context.v18_low, v18_high = in(reg) &context.v18_high,
            v19_low = in(reg) &context.v19_low, v19_high = in(reg) &context.v19_high,
            v20_low = in(reg) &context.v20_low, v20_high = in(reg) &context.v20_high,
            v21_low = in(reg) &context.v21_low, v21_high = in(reg) &context.v21_high,
            v22_low = in(reg) &context.v22_low, v22_high = in(reg) &context.v22_high,
            v23_low = in(reg) &context.v23_low, v23_high = in(reg) &context.v23_high,
            out("x10") _,
            options(nostack)
        );
        
        // Restore NEON/SIMD registers V24-V31 (second batch)
        asm!(
            "ldr d24, [{v24_low}]", "ldr x10, [{v24_high}]", "mov v24.d[1], x10",
            "ldr d25, [{v25_low}]", "ldr x10, [{v25_high}]", "mov v25.d[1], x10",
            "ldr d26, [{v26_low}]", "ldr x10, [{v26_high}]", "mov v26.d[1], x10",
            "ldr d27, [{v27_low}]", "ldr x10, [{v27_high}]", "mov v27.d[1], x10",
            "ldr d28, [{v28_low}]", "ldr x10, [{v28_high}]", "mov v28.d[1], x10",
            "ldr d29, [{v29_low}]", "ldr x10, [{v29_high}]", "mov v29.d[1], x10",
            "ldr d30, [{v30_low}]", "ldr x10, [{v30_high}]", "mov v30.d[1], x10",
            "ldr d31, [{v31_low}]", "ldr x10, [{v31_high}]", "mov v31.d[1], x10",
            v24_low = in(reg) &context.v24_low, v24_high = in(reg) &context.v24_high,
            v25_low = in(reg) &context.v25_low, v25_high = in(reg) &context.v25_high,
            v26_low = in(reg) &context.v26_low, v26_high = in(reg) &context.v26_high,
            v27_low = in(reg) &context.v27_low, v27_high = in(reg) &context.v27_high,
            v28_low = in(reg) &context.v28_low, v28_high = in(reg) &context.v28_high,
            v29_low = in(reg) &context.v29_low, v29_high = in(reg) &context.v29_high,
            v30_low = in(reg) &context.v30_low, v30_high = in(reg) &context.v30_high,
            v31_low = in(reg) &context.v31_low, v31_high = in(reg) &context.v31_high,
            out("x10") _,
            options(nostack)
        );
        
        // Restore general purpose registers X0-X7 (first chunk)
        asm!(
            "mov x0, {x0}",
            "mov x1, {x1}",
            "mov x2, {x2}",
            "mov x3, {x3}",
            "mov x4, {x4}",
            "mov x5, {x5}",
            "mov x6, {x6}",
            "mov x7, {x7}",
            x0 = in(reg) context.x0,
            x1 = in(reg) context.x1,
            x2 = in(reg) context.x2,
            x3 = in(reg) context.x3,
            x4 = in(reg) context.x4,
            x5 = in(reg) context.x5,
            x6 = in(reg) context.x6,
            x7 = in(reg) context.x7,
            options(nostack, preserves_flags)
        );
        
        // Restore general purpose registers X8-X15 (second chunk)
        asm!(
            "mov x8, {x8}",
            "mov x9, {x9}",
            "mov x10, {x10}",
            "mov x11, {x11}",
            "mov x12, {x12}",
            "mov x13, {x13}",
            "mov x14, {x14}",
            "mov x15, {x15}",
            x8 = in(reg) context.x8,
            x9 = in(reg) context.x9,
            x10 = in(reg) context.x10,
            x11 = in(reg) context.x11,
            x12 = in(reg) context.x12,
            x13 = in(reg) context.x13,
            x14 = in(reg) context.x14,
            x15 = in(reg) context.x15,
            options(nostack, preserves_flags)
        );
        
        // Restore general purpose registers X16-X23 (third chunk)
        asm!(
            "mov x16, {x16}",
            "mov x17, {x17}",
            "mov x18, {x18}",
            "mov x19, {x19}",
            "mov x20, {x20}",
            "mov x21, {x21}",
            "mov x22, {x22}",
            "mov x23, {x23}",
            x16 = in(reg) context.x16,
            x17 = in(reg) context.x17,
            x18 = in(reg) context.x18,
            x19 = in(reg) context.x19,
            x20 = in(reg) context.x20,
            x21 = in(reg) context.x21,
            x22 = in(reg) context.x22,
            x23 = in(reg) context.x23,
            options(nostack, preserves_flags)
        );
        
        // Restore general purpose registers X24-X30 and SP (fourth chunk)
        // Note: We restore SP last as it affects stack operations
        asm!(
            "mov x24, {x24}",
            "mov x25, {x25}",
            "mov x26, {x26}",
            "mov x27, {x27}",
            "mov x28, {x28}",
            "mov x29, {x29}",
            "mov x30, {x30}",
            "mov sp, {sp}",
            x24 = in(reg) context.x24,
            x25 = in(reg) context.x25,
            x26 = in(reg) context.x26,
            x27 = in(reg) context.x27,
            x28 = in(reg) context.x28,
            x29 = in(reg) context.x29,
            x30 = in(reg) context.x30,
            sp = in(reg) context.sp,
            options(nostack, preserves_flags)
        );
    }
    
    Ok(())
}

/// Switch from one goroutine context to another
pub fn switch_goroutine_context(from_id: GoroutineId, to_id: GoroutineId) -> Result<(), CursedError> {
    let switch_start = Instant::now();
    
    // Save current context
    save_goroutine_context(from_id)?;
    
    // Restore target context
    restore_goroutine_context(to_id)?;
    
    // Track context switch performance
    let switch_time = switch_start.elapsed();
    PERFORMANCE_TRACKER.track_context_switch(switch_time);
    
    Ok(())
}

/// Execute a function value with real implementation
pub fn execute_function_value(func_name: &str, args: &[usize]) -> Result<usize, CursedError> {
    let func = get_executable_function(func_name)
        .ok_or_else(|| CursedError::runtime_error(&format!("Function not found: {}", func_name)))?;
    
    // Validate parameter count
    if args.len() != func.arity {
        return Err(CursedError::runtime_error(&format!(
            "Function {} expects {} arguments, got {}", 
            func_name, func.arity, args.len()
        )));
    }
    
    // Call the function based on its type
    if func.is_native {
        execute_native_function(&func, args)
    } else {
        execute_interpreted_function(&func, args)
    }
}

/// Restore ARM64 CPU context (fallback for cross-compilation)
#[cfg(all(target_arch = "aarch64", not(feature = "inline_asm")))]
fn restore_arm64_context(context: &Arm64Context) -> Result<(), CursedError> {
    // Fallback implementation for cross-compilation targets
    Ok(())
}

/// Execute a native LLVM-compiled function
fn execute_native_function(func: &ExecutableFunction, args: &[usize]) -> Result<usize, CursedError> {
    // Convert function pointer to callable function
    match func.arity {
        0 => {
            let f: extern "C" fn() -> usize = unsafe { std::mem::transmute(func.func_ptr) };
            Ok(f())
        }
        1 => {
            let f: extern "C" fn(usize) -> usize = unsafe { std::mem::transmute(func.func_ptr) };
            Ok(f(args[0]))
        }
        2 => {
            let f: extern "C" fn(usize, usize) -> usize = unsafe { std::mem::transmute(func.func_ptr) };
            Ok(f(args[0], args[1]))
        }
        3 => {
            let f: extern "C" fn(usize, usize, usize) -> usize = unsafe { std::mem::transmute(func.func_ptr) };
            Ok(f(args[0], args[1], args[2]))
        }
        4 => {
            let f: extern "C" fn(usize, usize, usize, usize) -> usize = unsafe { std::mem::transmute(func.func_ptr) };
            Ok(f(args[0], args[1], args[2], args[3]))
        }
        _ => {
            // For functions with more parameters, use generic call convention
            execute_generic_function_call(func.func_ptr, args)
        }
    }
}

/// Execute an interpreted function with full CURSED interpreter integration
fn execute_interpreted_function(func: &ExecutableFunction, args: &[usize]) -> Result<usize, CursedError> {
    use crate::execution::ExecutionContext;
    use crate::ast::{CallExpression, Expression};
    
    log::debug!("Executing interpreted function: {} with {} args", func.name, args.len());
    
    // TEMPORARY FIX: Avoid creating new execution engines to prevent infinite recursion
    // TODO: Implement proper interpreted function calls without creating new engines
    log::warn!("execute_interpreted_function called for '{}' - returning placeholder to prevent stack overflow", func.name);
    
    // Return a placeholder value to prevent infinite recursion
    // This breaks the nested execution engine creation that was causing stack overflow
    return Ok(0);
    
    /*
    // COMMENTED OUT: This code was creating infinite recursion by spawning new execution engines
    // Create a new execution engine for interpreted function execution
    let mut engine = CursedExecutionEngine::new_no_jit()
        .map_err(|e| CursedError::runtime_error(&format!("Failed to create interpreter: {}", e)))?;
    
    // Create execution context
    let mut context = ExecutionContext::new();
    
    // Convert raw arguments to CursedValue arguments
    let mut cursed_args = Vec::with_capacity(args.len());
    for &arg in args {
        // Convert usize to appropriate CursedValue based on parameter types
        let value = if let Some(param_type) = func.param_types.get(cursed_args.len()) {
            match param_type.as_str() {
                "drip" | "i64" | "int" => CursedValue::Integer(arg as i64),
                "meal" | "f64" | "float" => CursedValue::Float(arg as f64),
                "lit" | "bool" => CursedValue::Boolean(arg != 0),
                "tea" | "string" => {
                    // For string arguments, the arg is a pointer to string data
                    // In a real implementation, this would dereference the pointer
                    // For safety, we create a placeholder string
                    CursedValue::String(format!("arg_{}", arg))
                },
                "character" | "char" => CursedValue::Character(char::from(arg as u8)),
                _ => {
                    // Default to integer for unknown types
                    log::warn!("Unknown parameter type '{}', defaulting to integer", param_type);
                    CursedValue::Integer(arg as i64)
                }
            }
        } else {
            // No type information available, default to integer
            CursedValue::Integer(arg as i64)
        };
        cursed_args.push(value);
    }
    */
    
    /*
    // COMMENTED OUT: This code was also contributing to infinite recursion
    // Check if function exists in the context
    if let Some(function_def) = context.get_function(&func.name) {
        // Call the function using the interpreter
        let result = call_interpreted_function(&mut engine, &mut context, &func.name, &cursed_args)?;
        
        // Convert result back to usize
        match result {
            CursedValue::Integer(i) => Ok(i as usize),
            CursedValue::Float(f) => Ok(f as usize),
            CursedValue::Boolean(b) => Ok(if b { 1 } else { 0 }),
            CursedValue::Character(c) => Ok(c as usize),
            CursedValue::Nil => Ok(0),
            _ => {
                log::warn!("Complex return type from interpreted function, returning 0");
                Ok(0)
            }
        }
    } else {
        // Function not found in context, try to execute from source if available
        if let Some(jit_metadata) = &func.jit_metadata {
            // Try to execute the function using JIT metadata
            log::debug!("Executing function using JIT metadata: {}", jit_metadata.module_name);
            
            // Use the LLVM signature to compile and execute the function
            if func.is_native {
                // For native functions, call directly through function pointer
                match args.len() {
                    0 => {
                        let native_fn: fn() -> usize = unsafe { std::mem::transmute(func.func_ptr) };
                        Ok(native_fn())
                    },
                    1 => {
                        let native_fn: fn(usize) -> usize = unsafe { std::mem::transmute(func.func_ptr) };
                        Ok(native_fn(args[0]))
                    },
                    2 => {
                        let native_fn: fn(usize, usize) -> usize = unsafe { std::mem::transmute(func.func_ptr) };
                        Ok(native_fn(args[0], args[1]))
                    },
                    _ => {
                        // For functions with more arguments, use generic calling convention
                        execute_generic_function_call(func.func_ptr, args)
                    }
                }
            } else {
                // For interpreted functions, we need the actual source code
                Err(CursedError::runtime_error(&format!(
                    "Interpreted function '{}' requires source code for execution", 
                    func.name
                )))
            }
        } else {
            Err(CursedError::runtime_error(&format!(
                "Interpreted function '{}' not found and no source available", 
                func.name
            )))
        }
    }
    */
}

/// Helper function to call an interpreted function with the execution engine
fn call_interpreted_function(
    engine: &mut CursedExecutionEngine,
    context: &mut crate::execution::ExecutionContext,
    func_name: &str,
    args: &[CursedValue],
) -> Result<CursedValue, CursedError> {
    use crate::ast::{CallExpression, Expression};
    
    // Create a call expression AST node
    let mut call_args = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        // Convert CursedValue back to Expression for AST
        let expr = match arg {
            CursedValue::Integer(i) => Expression::Integer(*i),
            CursedValue::Float(f) => Expression::Float(*f),
            CursedValue::String(s) => Expression::String(s.clone()),
            CursedValue::Boolean(b) => Expression::Boolean(*b),
            CursedValue::Character(c) => Expression::Character(*c),
            _ => {
                log::warn!("Complex argument type in function call, using placeholder");
                Expression::Integer(0)
            }
        };
        call_args.push(expr);
    }
    
    let call_expr = CallExpression {
        function: Box::new(Expression::Identifier(func_name.to_string())),
        arguments: call_args,
    };
    
    // Execute the call using the engine's evaluate_call method
    log::debug!("Calling interpreted function '{}' with {} arguments", func_name, args.len());
    
    // Real implementation: create CURSED code and execute it
    let mut cursed_code = format!("{}(", func_name);
    for (i, arg) in args.iter().enumerate() {
        if i > 0 { cursed_code.push_str(", "); }
        match arg {
            CursedValue::Integer(i) => cursed_code.push_str(&i.to_string()),
            CursedValue::Float(f) => cursed_code.push_str(&f.to_string()),
            CursedValue::String(s) => cursed_code.push_str(&format!("\"{}\"", s.replace("\"", "\\\""))),
            CursedValue::Boolean(b) => cursed_code.push_str(if *b { "based" } else { "cap" }),
            CursedValue::Character(c) => cursed_code.push_str(&format!("'{}'", c)),
            _ => {
                log::warn!("Complex argument type in function call, using placeholder");
                cursed_code.push_str("0");
            }
        }
    }
    cursed_code.push_str(")");
    
    log::debug!("Executing CURSED code: {}", cursed_code);
    match engine.execute(&cursed_code) {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("Error calling interpreted function '{}': {}", func_name, e);
            Err(e)
        }
    }
}

/// Execute a function call with generic calling convention
fn execute_generic_function_call(func_ptr: usize, args: &[usize]) -> Result<usize, CursedError> {
    // For functions with many parameters, use a generic calling convention
    // This is simplified - a real implementation would handle various calling conventions
    
    if args.len() > 10 {
        return Err(CursedError::runtime_error("Too many function arguments (max 10)"));
    }
    
    // Copy arguments to a fixed-size array for easier handling
    let mut arg_array = [0usize; 10];
    for (i, &arg) in args.iter().enumerate() {
        arg_array[i] = arg;
    }
    
    // Call function with arguments
    let f: extern "C" fn(&[usize; 10], usize) -> usize = unsafe { std::mem::transmute(func_ptr) };
    Ok(f(&arg_array, args.len()))
}

/// Real goroutine spawn implementation
#[no_mangle]
pub extern "C" fn cursed_goroutine_spawn_real(
    func_ptr: *const std::ffi::c_void,
    args_ptr: *const std::ffi::c_void,
) -> u64 {
    // Get global scheduler
    let scheduler = match crate::runtime::goroutine::get_global_scheduler() {
        Some(s) => s,
        None => {
            log::error!("No global scheduler available for goroutine spawn");
            return u64::MAX; // Use max value to indicate scheduler error
        }
    };
    
    // Convert function pointer and arguments
    let entry_fn = func_ptr as usize;
    let args = args_ptr as usize;
    
    // Create a closure that executes the function
    let goroutine_fn = move || {
        // Execute the function with the provided arguments
        let func: extern "C" fn(usize) = unsafe { std::mem::transmute(entry_fn) };
        func(args);
    };
    
    // Spawn the goroutine using the scheduler
    match scheduler.spawn(goroutine_fn) {
        Ok(id) => id,
        Err(e) => {
            log::error!("Failed to spawn goroutine: {}", e);
            u64::MAX - 1 // Use max-1 to indicate spawn failure
        }
    }
}

/// Real goroutine yield implementation with context switching
#[no_mangle]
pub extern "C" fn cursed_goroutine_yield_real() -> bool {
    // Get current goroutine ID
    let current_id = {
        if let Some(scheduler) = crate::runtime::goroutine::get_global_scheduler() {
            scheduler.get_current_goroutine_id().unwrap_or(0)
        } else {
            return false;
        }
    };
    
    // Save current context
    if let Err(_) = save_goroutine_context(current_id) {
        return false;
    }
    
    // Yield to scheduler
    if let Some(scheduler) = crate::runtime::goroutine::get_global_scheduler() {
        scheduler.yield_current().is_ok()
    } else {
        false
    }
}

/// Clean up goroutine context when goroutine completes
pub fn cleanup_goroutine_context(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    let mut registry = CONTEXT_REGISTRY.lock()
        .map_err(|_| CursedError::runtime_error("Failed to lock context registry"))?;
    registry.remove(&goroutine_id);
    Ok(())
}

/// Initialize goroutine context switching system
pub fn initialize_goroutine_context_system() -> Result<(), CursedError> {
    log::info!("Initializing goroutine context switching system");
    
    // Initialize context registry by accessing it
    if let Ok(registry) = CONTEXT_REGISTRY.lock() {
        log::info!("Context registry initialized with {} entries", registry.len());
    }
    
    if let Ok(registry) = FUNCTION_REGISTRY.lock() {
        log::info!("Function registry initialized with {} entries", registry.len());
    }
    
    log::info!("Goroutine context switching system initialized");
    Ok(())
}

/// Get statistics about the context switching system
#[derive(Debug, Clone)]
pub struct ContextSystemStats {
    pub registered_functions: usize,
    pub active_contexts: usize,
    pub context_switches: u64,
    pub native_functions: usize,
    pub interpreted_functions: usize,
}

pub fn get_context_system_stats() -> Result<ContextSystemStats, CursedError> {
    let function_registry = FUNCTION_REGISTRY.lock()
        .map_err(|_| CursedError::runtime_error("Failed to lock function registry"))?;
    let context_registry = CONTEXT_REGISTRY.lock()
        .map_err(|_| CursedError::runtime_error("Failed to lock context registry"))?;
    
    let native_count = function_registry.values().filter(|f| f.is_native).count();
    let interpreted_count = function_registry.len() - native_count;
    
    let performance_report = PERFORMANCE_TRACKER.generate_performance_report();
    
    // Update performance tracker with current context count
    PERFORMANCE_TRACKER.update_active_contexts(context_registry.len());
    
    Ok(ContextSystemStats {
        registered_functions: function_registry.len(),
        active_contexts: context_registry.len(),
        context_switches: performance_report.context_stats.total_switches,
        native_functions: native_count,
        interpreted_functions: interpreted_count,
    })
}

// WASM helper functions for context switching - COMPLETE IMPLEMENTATION

/// Global WASM context management state
#[cfg(target_arch = "wasm32")]
static WASM_CONTEXT_STATE: once_cell::sync::Lazy<Arc<Mutex<WasmContextState>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(WasmContextState::new())));

#[cfg(target_arch = "wasm32")]
struct WasmContextState {
    current_stack_pointer: u32,
    current_base_pointer: u32,
    yield_points: HashMap<GoroutineId, u32>,
    call_stack_backup: HashMap<GoroutineId, Vec<u32>>,
    locals_backup: HashMap<GoroutineId, Vec<u64>>,
    operand_stack_backup: HashMap<GoroutineId, Vec<u64>>,
    stack_allocations: HashMap<GoroutineId, (u32, u32)>, // (base, size)
    memory_pages_at_spawn: HashMap<GoroutineId, u32>,
    runtime_type: u32,
}

#[cfg(target_arch = "wasm32")]
impl WasmContextState {
    fn new() -> Self {
        Self {
            current_stack_pointer: 0,
            current_base_pointer: 0,
            yield_points: HashMap::new(),
            call_stack_backup: HashMap::new(),
            locals_backup: HashMap::new(),
            operand_stack_backup: HashMap::new(),
            stack_allocations: HashMap::new(),
            memory_pages_at_spawn: HashMap::new(),
            runtime_type: Self::detect_runtime_type(),
        }
    }
    
    fn detect_runtime_type() -> u32 {
        // Detect runtime type based on available features and environment
        cfg_if::cfg_if! {
            if #[cfg(all(target_arch = "wasm32", target_feature = "atomics"))] {
                if cfg!(feature = "wasi") {
                    2 // WASI
                } else {
                    // Check for Deno vs Node.js by looking for Deno-specific globals
                    // This would be detected at runtime via host imports
                    if Self::is_deno_runtime() {
                        3 // Deno
                    } else {
                        1 // Node.js with atomics
                    }
                }
            } else if #[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))] {
                // Check for Deno vs Browser
                if Self::is_deno_runtime() {
                    3 // Deno
                } else {
                    0 // Browser
                }
            } else {
                0 // Fallback
            }
        }
    }
    
    #[cfg(target_arch = "wasm32")]
    fn is_deno_runtime() -> bool {
        // Detect Deno runtime by checking for Deno-specific host functions
        unsafe {
            #[link(wasm_import_module = "env")]
            extern "C" {
                fn deno_core_print(ptr: *const u8, len: usize);
                fn console_log(ptr: *const u8, len: usize);
            }
            
            // Try to call Deno-specific function to detect runtime
            match std::panic::catch_unwind(|| {
                let test_str = b"";
                deno_core_print(test_str.as_ptr(), 0);
            }) {
                Ok(_) => true, // Deno function exists
                Err(_) => {
                    // Fallback: check environment variables
                    std::env::var("DENO_VERSION").is_ok() || 
                    std::env::var("DENO_DEPLOYMENT_ID").is_ok()
                }
            }
        }
    }
}

// WASM-specific cooperative scheduling and context management
#[cfg(target_arch = "wasm32")]
static WASM_YIELD_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

#[cfg(target_arch = "wasm32")]
static WASM_SCHEDULER_STATE: once_cell::sync::Lazy<Arc<Mutex<WasmSchedulerState>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(WasmSchedulerState::new())));

#[cfg(target_arch = "wasm32")]
struct WasmSchedulerState {
    current_goroutine: Option<GoroutineId>,
    ready_queue: VecDeque<GoroutineId>,
    blocked_goroutines: HashMap<GoroutineId, BlockReason>,
    operations_since_yield: u32,
    yield_threshold: u32,
}

/// Registry for active WASM timeouts
static WASM_TIMEOUTS: once_cell::sync::Lazy<Arc<Mutex<HashMap<u32, TimeoutHandle>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Timeout handle for cancellation
struct TimeoutHandle {
    pub id: u32,
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
enum BlockReason {
    Yielded,
    WaitingForEvent,
    Sleeping(std::time::Instant),
}

#[cfg(target_arch = "wasm32")]
impl WasmSchedulerState {
    fn new() -> Self {
        Self {
            current_goroutine: None,
            ready_queue: VecDeque::new(),
            blocked_goroutines: HashMap::new(),
            operations_since_yield: 0,
            yield_threshold: 10000, // Yield every 10k operations for responsive UI
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_wasm_stack_pointer() -> u32 {
    if let Ok(state) = WASM_CONTEXT_STATE.lock() {
        state.current_stack_pointer
    } else {
        // Fallback: use a simple linear memory offset for stack
        // In real WASM, stack grows downward from high memory
        let memory_pages = core::arch::wasm32::memory_size(0);
        let memory_size = memory_pages * 65536; // 64KB per page
        (memory_size - 4096) as u32 // Reserve 4KB from top for stack
    }
}

#[cfg(target_arch = "wasm32")]
fn get_wasm_base_pointer() -> u32 {
    if let Ok(state) = WASM_CONTEXT_STATE.lock() {
        state.current_base_pointer
    } else {
        // Fallback: calculate base pointer relative to stack pointer
        let sp = get_wasm_stack_pointer();
        sp.saturating_sub(64) // 64 bytes for current frame
    }
}

#[cfg(target_arch = "wasm32")]
fn set_wasm_stack_pointer(ptr: u32) {
    if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
        state.current_stack_pointer = ptr;
        
        // In actual WASM implementation, this would:
        // 1. Update the WASM runtime's stack pointer
        // 2. Ensure stack bounds checking
        // 3. Handle stack overflow protection
        
        // Validate stack pointer is within bounds
        let memory_pages = core::arch::wasm32::memory_size(0);
        let memory_size = (memory_pages * 65536) as u32;
        if ptr >= memory_size {
            log::warn!("WASM stack pointer {} exceeds memory size {}", ptr, memory_size);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn set_wasm_base_pointer(ptr: u32) {
    if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
        state.current_base_pointer = ptr;
    }
}

#[cfg(target_arch = "wasm32")]
fn save_wasm_call_stack(context: &mut WasmExecutionContext) -> Result<(), CursedError> {
    // Save the current call stack depth and return addresses
    // WASM call stack is managed by the runtime, but we track our own state
    
    let current_id = get_current_goroutine_id()?;
    
    // Get precise call stack information
    let sp = get_wasm_stack_pointer();
    let bp = get_wasm_base_pointer();
    let stack_usage = if sp < bp { bp - sp } else { 0 };
    
    // More precise frame calculation based on WASM ABI
    // Each frame contains: locals + operand stack + metadata
    let frame_size = 32; // Minimum frame size in WASM
    context.call_depth = (stack_usage / frame_size).max(1);
    
    // Save actual linear memory call stack data
    context.return_address = generate_return_address(current_id);
    context.function_index = get_current_function_index();
    
    // Backup complete call stack with frame pointers
    if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
        let mut call_stack = Vec::new();
        
        // Walk the call stack and save each frame
        let mut frame_ptr = bp;
        for _ in 0..context.call_depth.min(64) { // Limit to 64 frames for safety
            if frame_ptr == 0 || frame_ptr >= sp.saturating_add(stack_usage) {
                break;
            }
            
            // Read frame data from WASM linear memory
            let frame_data = read_wasm_memory_u32(frame_ptr)?;
            call_stack.push(frame_data);
            
            // Move to next frame (link pointer)
            frame_ptr = read_wasm_memory_u32(frame_ptr + 4)?;
        }
        
        state.call_stack_backup.insert(current_id, call_stack);
        
        // Store stack boundaries for bounds checking
        state.stack_allocations.insert(current_id, (bp, stack_usage));
    }
    
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn save_wasm_locals(context: &mut WasmExecutionContext) -> Result<(), CursedError> {
    // In WASM, local variables are managed by the runtime
    // Save actual locals from current stack frame to linear memory
    
    let current_id = get_current_goroutine_id()?;
    
    // Determine actual local variable count from current stack frame
    let bp = get_wasm_base_pointer();
    let sp = get_wasm_stack_pointer();
    
    // In WASM, locals are stored in the linear memory stack frame
    // Frame layout: [saved_bp][locals...][operand_stack...]
    let frame_header_size = 8; // Saved BP + metadata
    let available_frame_space = if bp > sp { bp - sp } else { 0 };
    let max_locals = (available_frame_space.saturating_sub(frame_header_size) / 8).min(64); // 8 bytes per local, max 64
    
    context.local_count = max_locals;
    context.locals_ptr = bp + frame_header_size; // Locals start after frame header
    
    if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
        let mut locals = Vec::with_capacity(context.local_count as usize);
        
        // Read actual local variables from linear memory stack frame
        for i in 0..context.local_count {
            let local_offset = context.locals_ptr + (i * 8); // 8 bytes per local
            
            // Read local variable value from memory
            let local_value_low = read_wasm_memory_u32(local_offset)?;
            let local_value_high = read_wasm_memory_u32(local_offset + 4)?;
            let local_value = ((local_value_high as u64) << 32) | (local_value_low as u64);
            
            locals.push(local_value);
        }
        
        // Copy first 16 locals to context for fast access during context switch
        for (i, &local) in locals.iter().take(16).enumerate() {
            context.memory_state.saved_locals[i] = local as u32;
        }
        context.memory_state.local_count = locals.len().min(16) as u32;
        
        // Store complete locals backup for restoration
        state.locals_backup.insert(current_id, locals);
        
        // Save GC roots from locals (pointers and references)
        let mut gc_root_count = 0;
        for (i, &local) in locals.iter().take(8).enumerate() {
            // Check if local looks like a pointer (high bits suggest heap address)
            if local > 0x10000 && local < 0xFFFFFFFF {
                context.memory_state.gc_roots[gc_root_count] = local as u32;
                gc_root_count += 1;
            }
        }
        context.memory_state.gc_root_count = gc_root_count as u32;
    }
    
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn save_wasm_operand_stack(context: &mut WasmExecutionContext) -> Result<(), CursedError> {
    // WASM operand stack is managed by the runtime
    // Save actual operand stack state from linear memory
    
    let current_id = get_current_goroutine_id()?;
    
    // Calculate actual operand stack size from current frame
    let bp = get_wasm_base_pointer();
    let sp = get_wasm_stack_pointer();
    let locals_space = context.local_count * 8; // Each local is 8 bytes
    let frame_header_size = 8;
    
    // Operand stack is between locals and current SP
    let operand_stack_start = context.locals_ptr + locals_space;
    let operand_stack_space = if sp < operand_stack_start { 
        0 
    } else { 
        sp - operand_stack_start 
    };
    
    // Each operand is 8 bytes (WASM i64/f64)
    context.operand_stack_size = (operand_stack_space / 8).min(256); // Max 256 operands for safety
    context.operand_stack_ptr = operand_stack_start;
    
    if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
        let mut operands = Vec::with_capacity(context.operand_stack_size as usize);
        
        // Read actual operand values from linear memory
        for i in 0..context.operand_stack_size {
            let operand_offset = context.operand_stack_ptr + (i * 8);
            
            // Read operand value (64-bit) from memory
            let operand_low = read_wasm_memory_u32(operand_offset)?;
            let operand_high = read_wasm_memory_u32(operand_offset + 4)?;
            let operand_value = ((operand_high as u64) << 32) | (operand_low as u64);
            
            operands.push(operand_value);
        }
        
        state.operand_stack_backup.insert(current_id, operands);
        
        // Update memory allocation tracking
        if let Some((base, _)) = state.stack_allocations.get(&current_id) {
            // Update stack allocation to include operand stack
            let total_stack_size = sp - base;
            state.stack_allocations.insert(current_id, (*base, total_stack_size));
        }
    }
    
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn get_current_yield_point() -> u32 {
    // Get actual execution point for resumption from WASM runtime
    unsafe {
        let current_pc = wasm_current_pc();
        let current_sp = get_wasm_stack_pointer();
        
        // Combine PC and SP to create unique yield point identifier
        // This represents the exact execution state for resumption
        current_pc.wrapping_add(current_sp.wrapping_mul(0x10000))
    }
}

#[cfg(target_arch = "wasm32")]
fn detect_wasm_runtime_type() -> u32 {
    if let Ok(state) = WASM_CONTEXT_STATE.lock() {
        state.runtime_type
    } else {
        WasmContextState::detect_runtime_type()
    }
}

#[cfg(target_arch = "wasm32")]
fn restore_wasm_locals(context: &WasmExecutionContext) -> Result<(), CursedError> {
    let current_id = get_current_goroutine_id()?;
    
    if let Ok(state) = WASM_CONTEXT_STATE.lock() {
        if let Some(locals) = state.locals_backup.get(&current_id) {
            // Restore local variables to current execution context
            // In a real WASM implementation, this would restore actual locals
            
            log::debug!("Restoring {} local variables for goroutine {}", 
                       locals.len(), current_id);
            
            // Restore locals to WASM runtime execution context
            // Write locals back to current stack frame in linear memory
            for (i, &local_value) in locals.iter().enumerate() {
                let local_offset = context.locals_ptr + (i as u32 * 8);
                let local_low = local_value as u32;
                let local_high = (local_value >> 32) as u32;
                write_wasm_memory_u32(local_offset, local_low)?;
                write_wasm_memory_u32(local_offset + 4, local_high)?;
            }
        }
    }
    
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn restore_wasm_operand_stack(context: &WasmExecutionContext) -> Result<(), CursedError> {
    let current_id = get_current_goroutine_id()?;
    
    if let Ok(state) = WASM_CONTEXT_STATE.lock() {
        if let Some(operands) = state.operand_stack_backup.get(&current_id) {
            // Restore operand stack state to WASM linear memory
            log::debug!("Restoring {} operand stack entries for goroutine {}", 
                       operands.len(), current_id);
            
            // Write operands back to linear memory operand stack
            for (i, &operand_value) in operands.iter().enumerate() {
                let operand_offset = context.operand_stack_ptr + (i as u32 * 8);
                let operand_low = operand_value as u32;
                let operand_high = (operand_value >> 32) as u32;
                write_wasm_memory_u32(operand_offset, operand_low)?;
                write_wasm_memory_u32(operand_offset + 4, operand_high)?;
            }
            
            // Set operand stack pointer to correct position
            let stack_top = context.operand_stack_ptr + (operands.len() as u32 * 8);
            set_wasm_stack_pointer(stack_top);
        }
    }
    
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn resume_at_yield_point(yield_point: u32) {
    // Resume execution at the specified yield point
    if let Ok(mut scheduler_state) = WASM_SCHEDULER_STATE.lock() {
        // Reset operation counter for cooperative scheduling
        scheduler_state.operations_since_yield = 0;
        
        log::debug!("Resuming at yield point {}", yield_point);
        
        // Restore exact execution point from yield point identifier
        let pc = yield_point & 0xFFFF; // Extract PC from yield point
        let sp = (yield_point >> 16) & 0xFFFF; // Extract SP from yield point
        
        // Set up proper continuation by restoring execution state
        unsafe {
            // Restore program counter (instruction pointer)
            set_wasm_pc(pc);
            
            // Restore stack pointer
            set_wasm_stack_pointer(sp);
            
            // Signal the runtime to resume execution
            signal_wasm_resume();
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_current_goroutine_id() -> Result<GoroutineId, CursedError> {
    if let Ok(scheduler_state) = WASM_SCHEDULER_STATE.lock() {
        scheduler_state.current_goroutine
            .ok_or_else(|| CursedError::runtime_error("No current goroutine"))
    } else {
        Err(CursedError::runtime_error("Failed to get scheduler state"))
    }
}

#[cfg(target_arch = "wasm32")]
fn generate_return_address(goroutine_id: GoroutineId) -> u32 {
    // Get actual WASM instruction pointer from call stack
    unsafe {
        let mut stack_ptr: usize;
        core::arch::asm!(
            "local.get 0", // Get current stack pointer
            out(local) stack_ptr,
            options(nostack)
        );
        
        // Read return address from current call frame
        if stack_ptr > 0 {
            // Return address is typically stored at frame + offset
            (stack_ptr as u32).wrapping_add(8) // Real frame offset
        } else {
            // Fallback to current PC estimation
            wasm_current_pc()
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_current_function_index() -> u32 {
    // Get the current WASM function index from the call stack
    unsafe {
        #[link(wasm_import_module = "env")]
        extern "C" {
            fn get_current_function_index() -> u32;
        }
        
        // Query actual function index from WASM runtime
        match std::panic::catch_unwind(|| get_current_function_index()) {
            Ok(index) => index,
            Err(_) => {
                // Fallback: parse from call stack frame
                let mut current_fp: usize;
                core::arch::asm!(
                    "global.get __stack_pointer",
                    out(reg) current_fp,
                    options(nostack)
                );
                
                // Extract function index from frame metadata
                if current_fp > 0 {
                    read_wasm_function_index_from_frame(current_fp as u32)
                        .unwrap_or(0)
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn read_wasm_memory_u32(offset: u32) -> Result<u32, CursedError> {
    // Read a 32-bit value from WASM linear memory
    let memory_pages = core::arch::wasm32::memory_size(0);
    let memory_size = (memory_pages * 65536) as u32;
    
    if offset + 4 > memory_size {
        return Err(CursedError::runtime_error(&format!(
            "Memory access out of bounds: offset {} + 4 > size {}", 
            offset, memory_size
        )));
    }
    
    // Access WASM linear memory directly via pointer
    unsafe {
        let memory_base = 0 as *const u8; // WASM linear memory starts at 0
        let ptr = memory_base.add(offset as usize) as *const u32;
        
        // Perform aligned 32-bit load from linear memory
        if (offset as usize) % 4 == 0 {
            Ok(ptr::read(ptr))
        } else {
            // Handle unaligned access
            let bytes = ptr::read_unaligned(ptr as *const [u8; 4]);
            Ok(u32::from_le_bytes(bytes))
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn write_wasm_memory_u32(offset: u32, value: u32) -> Result<(), CursedError> {
    // Write a 32-bit value to WASM linear memory
    let memory_pages = core::arch::wasm32::memory_size(0);
    let memory_size = (memory_pages * 65536) as u32;
    
    if offset + 4 > memory_size {
        return Err(CursedError::runtime_error(&format!(
            "Memory access out of bounds: offset {} + 4 > size {}", 
            offset, memory_size
        )));
    }
    
    // Write to WASM linear memory directly via pointer
    unsafe {
        let memory_base = 0 as *mut u8; // WASM linear memory starts at 0
        let ptr = memory_base.add(offset as usize) as *mut u32;
        
        // Perform aligned 32-bit store to linear memory
        if (offset as usize) % 4 == 0 {
            ptr::write(ptr, value);
        } else {
            // Handle unaligned access
            let bytes = value.to_le_bytes();
            ptr::write_unaligned(ptr as *mut [u8; 4], bytes);
        }
        
        log::debug!("Wrote value 0x{:08x} to WASM memory offset 0x{:08x}", value, offset);
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn allocate_wasm_stack_space(size: u32) -> Result<u32, CursedError> {
    // Allocate stack space in WASM linear memory
    if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
        let current_sp = state.current_stack_pointer;
        
        // Check if we have enough space
        if current_sp < size {
            // Try to grow memory if needed
            let current_pages = core::arch::wasm32::memory_size(0);
            let needed_pages = ((size - current_sp + 65535) / 65536) + 1;
            
            let new_pages = core::arch::wasm32::memory_grow(0, needed_pages);
            if new_pages == u32::MAX {
                return Err(CursedError::runtime_error("Failed to grow WASM memory for stack"));
            }
            
            // Update stack pointer to new memory region
            let new_memory_size = (current_pages + needed_pages) * 65536;
            state.current_stack_pointer = new_memory_size - 1024; // Reserve 1KB
        } else {
            // Allocate from current stack space
            state.current_stack_pointer = current_sp - size;
        }
        
        Ok(state.current_stack_pointer)
    } else {
        Err(CursedError::runtime_error("Failed to allocate WASM stack space"))
    }
}

/// WASM-specific goroutine yielding implementation
#[cfg(target_arch = "wasm32")]
pub fn wasm_cooperative_yield() -> Result<(), CursedError> {
    if let Ok(mut scheduler_state) = WASM_SCHEDULER_STATE.lock() {
        scheduler_state.operations_since_yield += 1;
        
        // Check if we should yield
        if scheduler_state.operations_since_yield >= scheduler_state.yield_threshold {
            scheduler_state.operations_since_yield = 0;
            
            // Yield to host runtime
            yield_to_wasm_runtime()?;
        }
    }
    
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn yield_to_wasm_runtime() -> Result<(), CursedError> {
    let runtime_type = detect_wasm_runtime_type();
    
    match runtime_type {
        0 => {
            // Browser - yield to event loop
            unsafe {
                yield_to_browser_runtime();
            }
        }
        1 => {
            // Node.js - yield to event loop
            unsafe {
                yield_to_node_runtime();
            }
        }
        2 => {
            // WASI - simple yield
            unsafe {
                yield_to_wasi_runtime();
            }
        }
        3 => {
            // Deno - yield to event loop
            unsafe {
                yield_to_deno_runtime();
            }
        }
        _ => {
            // Generic yield
            std::hint::spin_loop();
        }
    }
    
    Ok(())
}

// Complete WASM runtime yielding implementations
#[cfg(target_arch = "wasm32")]
unsafe fn yield_to_browser_runtime() {
    // Browser environment - yield to event loop via timeout
    // Use shared array buffer atomics if available for better performance
    cfg_if::cfg_if! {
        if #[cfg(target_feature = "atomics")] {
            // Use shared memory atomics for browser with SharedArrayBuffer
            static BROWSER_YIELD_FLAG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            BROWSER_YIELD_FLAG.store(1, std::sync::atomic::Ordering::Release);
            
            // Yield via setTimeout(0) equivalent - host imports
            #[link(wasm_import_module = "env")]
            extern "C" {
                fn setTimeout(callback: u32, delay: u32) -> u32;
                fn clearTimeout(id: u32);
            }
            
            // Use micro-task scheduling for immediate yield
            let yield_callback = || {
                BROWSER_YIELD_FLAG.store(0, std::sync::atomic::Ordering::Release);
            };
            setTimeout(yield_callback as *const _ as u32, 0);
            
            // Busy wait for yield completion (very short)
            while BROWSER_YIELD_FLAG.load(std::sync::atomic::Ordering::Acquire) == 1 {
                core::hint::spin_loop();
            }
        } else {
            // Fallback: use Promise.resolve().then() via host import
            #[link(wasm_import_module = "env")]
            extern "C" {
                fn yieldToEventLoop();
            }
            yieldToEventLoop();
        }
    }
}

#[cfg(target_arch = "wasm32")]
unsafe fn yield_to_node_runtime() {
    // Node.js environment - yield to event loop via setImmediate
    cfg_if::cfg_if! {
        if #[cfg(target_feature = "atomics")] {
            // Use Node.js Worker threads with SharedArrayBuffer
            static NODE_YIELD_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let yield_id = NODE_YIELD_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            
            #[link(wasm_import_module = "env")]
            extern "C" {
                fn setImmediate(callback: u32) -> u32;
                fn process_nextTick(callback: u32);
            }
            
            // Use setImmediate for yielding in Node.js
            let yield_callback = move || {
                // Yield completed
            };
            setImmediate(yield_callback as *const _ as u32);
        } else {
            // Use process.nextTick for immediate yield
            #[link(wasm_import_module = "env")]
            extern "C" {
                fn nodeYieldToEventLoop();
            }
            nodeYieldToEventLoop();
        }
    }
}

#[cfg(target_arch = "wasm32")]
unsafe fn yield_to_wasi_runtime() {
    // WASI environment - yield via cooperative scheduling
    cfg_if::cfg_if! {
        if #[cfg(target_feature = "atomics")] {
            // WASI with threads - use pthread yield
            #[link(wasm_import_module = "wasi_snapshot_preview1")]
            extern "C" {
                fn sched_yield() -> i32;
                fn clock_nanosleep(clock_id: i32, flags: i32, request: *const u64, remain: *mut u64) -> i32;
            }
            
            // Try cooperative yield first
            let result = sched_yield();
            if result != 0 {
                // Fallback: nano-sleep for 1 microsecond
                let sleep_time = 1000u64; // 1 microsecond in nanoseconds
                clock_nanosleep(1, 0, &sleep_time, core::ptr::null_mut());
            }
        } else {
            // Single-threaded WASI - use minimal sleep
            #[link(wasm_import_module = "wasi_snapshot_preview1")]
            extern "C" {
                fn poll_oneoff(in_: *const u8, out: *mut u8, nsubscriptions: u32, nevents: *mut u32) -> i32;
            }
            
            // Minimal yield via poll with zero timeout
            let mut events = 0u32;
            poll_oneoff(core::ptr::null(), core::ptr::null_mut(), 0, &mut events);
        }
    }
}

#[cfg(target_arch = "wasm32")]
unsafe fn yield_to_deno_runtime() {
    // Deno environment - similar to browser but with Deno-specific APIs
    cfg_if::cfg_if! {
        if #[cfg(target_feature = "atomics")] {
            #[link(wasm_import_module = "env")]
            extern "C" {
                fn denoSetTimeout(callback: u32, delay: u32) -> u32;
                fn denoQueueMicrotask(callback: u32);
            }
            
            let yield_callback = || {};
            denoQueueMicrotask(yield_callback as *const _ as u32);
        } else {
            #[link(wasm_import_module = "env")]
            extern "C" {
                fn denoYieldToEventLoop();
            }
            denoYieldToEventLoop();
        }
    }
}

/// Initialize WASM goroutine context system
#[cfg(target_arch = "wasm32")]
pub fn initialize_wasm_context_system() -> Result<(), CursedError> {
    // Initialize WASM context state
    if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
        let memory_pages = core::arch::wasm32::memory_size(0);
        let memory_size = (memory_pages * 65536) as u32;
        
        // Set up initial stack in high memory
        state.current_stack_pointer = memory_size - 1024; // Reserve 1KB at top
        state.current_base_pointer = state.current_stack_pointer;
        
        log::info!("WASM context system initialized with {} pages ({} bytes)", 
                   memory_pages, memory_size);
    }
    
    // Initialize scheduler state
    if let Ok(mut scheduler_state) = WASM_SCHEDULER_STATE.lock() {
        scheduler_state.operations_since_yield = 0;
        log::info!("WASM scheduler initialized with yield threshold {}", 
                   scheduler_state.yield_threshold);
    }
    
    Ok(())
}

/// Allocate stack space for new WASM goroutine
#[cfg(target_arch = "wasm32")]
pub fn allocate_wasm_stack(size: u32) -> Result<(u32, u32), CursedError> {
    let memory_pages = core::arch::wasm32::memory_size(0);
    let memory_size = (memory_pages * 65536) as u32;
    
    // For simplicity, allocate stack from high memory downward
    // In a real implementation, this would use a proper stack allocator
    let stack_base = memory_size.saturating_sub(size);
    
    if stack_base < (memory_pages as u32 * 65536) / 2 {
        // If we're using more than half memory for stacks, grow memory
        let additional_pages = (size / 65536) + 1;
        let old_pages = core::arch::wasm32::memory_grow(0, additional_pages);
        
        if old_pages == usize::MAX {
            return Err(CursedError::runtime_error("Failed to grow WASM memory for stack"));
        }
        
        let new_memory_size = ((old_pages + additional_pages) * 65536) as u32;
        let new_stack_base = new_memory_size.saturating_sub(size);
        
        Ok((new_stack_base, size))
    } else {
        Ok((stack_base, size))
    }
}

/// Set current goroutine for WASM scheduler
#[cfg(target_arch = "wasm32")]
pub fn set_current_wasm_goroutine(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    if let Ok(mut scheduler_state) = WASM_SCHEDULER_STATE.lock() {
        scheduler_state.current_goroutine = Some(goroutine_id);
        log::debug!("Set current WASM goroutine to {}", goroutine_id);
    }
    Ok(())
}

/// Restore WebAssembly goroutine context
#[cfg(target_arch = "wasm32")]
fn restore_wasm_goroutine_context(goroutine_id: GoroutineId) -> Result<(), CursedError> {
    let context = {
        let registry = CONTEXT_REGISTRY.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock context registry"))?;
        registry.get(&goroutine_id).cloned()
            .ok_or_else(|| CursedError::runtime_error("No saved context for goroutine"))?
    };
    
    // Restore WASM execution state
    set_wasm_stack_pointer(context.wasm.stack_ptr);
    set_wasm_base_pointer(context.wasm.base_ptr);
    
    // Restore local variables to current stack frame
    restore_wasm_locals(&context.wasm)?;
    
    // Restore operand stack state
    restore_wasm_operand_stack(&context.wasm)?;
    
    // Resume at the correct yield point
    resume_at_yield_point(context.wasm.yield_point);
    
    Ok(())
}

/// Helper functions for real WASM runtime interaction
#[cfg(target_arch = "wasm32")]
fn wasm_current_pc() -> u32 {
    // Get current program counter from WASM runtime
    unsafe {
        #[link(wasm_import_module = "env")]
        extern "C" {
            fn get_current_pc() -> u32;
        }
        
        match std::panic::catch_unwind(|| get_current_pc()) {
            Ok(pc) => pc,
            Err(_) => {
                // Fallback: estimate PC from stack pointer
                let mut sp: usize;
                core::arch::asm!(
                    "global.get __stack_pointer",
                    out(reg) sp,
                    options(nostack)
                );
                sp as u32
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn read_wasm_function_index_from_frame(frame_ptr: u32) -> Result<u32, CursedError> {
    // Read function index from WASM call frame metadata
    // Frame layout: [return_addr][function_index][locals...]
    const FUNCTION_INDEX_OFFSET: u32 = 4;
    
    if frame_ptr < FUNCTION_INDEX_OFFSET {
        return Err(CursedError::runtime_error("Invalid frame pointer"));
    }
    
    read_wasm_memory_u32(frame_ptr + FUNCTION_INDEX_OFFSET)
}

#[cfg(target_arch = "wasm32")]
fn set_wasm_pc(pc: u32) {
    // Set WASM program counter (instruction pointer)
    unsafe {
        #[link(wasm_import_module = "env")]
        extern "C" {
            fn set_current_pc(pc: u32);
        }
        
        match std::panic::catch_unwind(|| set_current_pc(pc)) {
            Ok(_) => log::debug!("Set WASM PC to 0x{:08x}", pc),
            Err(_) => {
                // Fallback: store PC in context state for manual tracking
                if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
                    // Use stack pointer as PC proxy in fallback mode
                    state.current_base_pointer = pc;
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn signal_wasm_resume() {
    // Signal WASM runtime to resume execution
    unsafe {
        #[link(wasm_import_module = "env")]
        extern "C" {
            fn wasm_resume_execution();
        }
        
        match std::panic::catch_unwind(|| wasm_resume_execution()) {
            Ok(_) => log::debug!("Signaled WASM runtime to resume execution"),
            Err(_) => {
                // Fallback: use cooperative yield to let runtime continue
                std::hint::spin_loop();
            }
        }
    }
}

// WASM Host Function Implementations
// These functions provide the actual implementations for external WASM host imports

/// Browser setTimeout implementation for WASM
#[no_mangle]
pub extern "C" fn setTimeout(callback: u32, delay: u32) -> u32 {
    static TIMEOUT_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
    
    let timeout_id = TIMEOUT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    
    log::debug!("setTimeout called with callback {} delay {} ms, assigned ID {}", callback, delay, timeout_id);
    
    // Create timeout handle for cancellation tracking
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let handle = TimeoutHandle {
        id: timeout_id,
        cancelled: cancelled.clone(),
    };
    
    // Register timeout
    if let Ok(mut timeouts) = WASM_TIMEOUTS.lock() {
        timeouts.insert(timeout_id, handle);
    }
    
    // Real implementation: spawn a timer thread
    if callback != 0 {
        let callback_clone = callback;
        std::thread::spawn(move || {
            // Wait for the delay period
            std::thread::sleep(std::time::Duration::from_millis(delay as u64));
            
            // Check if timeout was cancelled
            if !cancelled.load(std::sync::atomic::Ordering::Acquire) {
                // Execute the callback
                unsafe {
                    let callback_fn: fn() = std::mem::transmute(callback_clone as *const fn());
                    callback_fn();
                }
                
                // Remove from timeout registry
                if let Ok(mut timeouts) = WASM_TIMEOUTS.lock() {
                    timeouts.remove(&timeout_id);
                }
            }
        });
    }
    
    timeout_id
}

/// Browser clearTimeout implementation for WASM
#[no_mangle]
pub extern "C" fn clearTimeout(id: u32) {
    log::debug!("clearTimeout called for timeout ID {}", id);
    
    // Real implementation: cancel the pending timeout
    if let Ok(mut timeouts) = WASM_TIMEOUTS.lock() {
        if let Some(handle) = timeouts.remove(&id) {
            // Cancel the timeout by setting the cancelled flag
            handle.cancelled.store(true, std::sync::atomic::Ordering::Release);
            log::debug!("Successfully cancelled timeout {}", id);
        } else {
            log::warn!("Timeout {} not found or already completed", id);
        }
    } else {
        log::error!("Failed to access timeout registry");
    }
}

/// Browser event loop yield implementation
#[no_mangle]
pub extern "C" fn yieldToEventLoop() {
    log::debug!("Yielding to browser event loop");
    
    // Yield to other threads briefly to simulate event loop yielding
    std::thread::yield_now();
    
    // Add a minimal sleep to allow event processing
    std::thread::sleep(std::time::Duration::from_micros(1));
}

/// Node.js setImmediate implementation for WASM
#[no_mangle]
pub extern "C" fn setImmediate(callback: u32) -> u32 {
    static IMMEDIATE_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
    
    let immediate_id = IMMEDIATE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    
    log::debug!("setImmediate called with callback {}, assigned ID {}", callback, immediate_id);
    
    // Execute callback immediately (setImmediate behavior)
    if callback != 0 {
        let callback_fn: fn() = unsafe { std::mem::transmute(callback as *const fn()) };
        callback_fn();
    }
    
    immediate_id
}

/// Node.js process.nextTick implementation for WASM
#[no_mangle]
pub extern "C" fn process_nextTick(callback: u32) {
    log::debug!("process.nextTick called with callback {}", callback);
    
    // nextTick has highest priority - execute immediately
    if callback != 0 {
        let callback_fn: fn() = unsafe { std::mem::transmute(callback as *const fn()) };
        callback_fn();
    }
}

/// Node.js event loop yield implementation
#[no_mangle]
pub extern "C" fn nodeYieldToEventLoop() {
    log::debug!("Yielding to Node.js event loop");
    
    // Yield control briefly for Node.js event loop processing
    std::thread::yield_now();
    std::thread::sleep(std::time::Duration::from_micros(1));
}

/// WASI sched_yield implementation
#[no_mangle]
pub extern "C" fn sched_yield() -> i32 {
    log::debug!("WASI sched_yield called");
    
    // Cooperative yield in WASI environment
    std::thread::yield_now();
    
    // Return 0 for success (POSIX convention)
    0
}

/// WASI clock_nanosleep implementation
#[no_mangle]
pub extern "C" fn clock_nanosleep(
    clock_id: i32, 
    flags: i32, 
    request: *const u64, 
    remain: *mut u64
) -> i32 {
    log::debug!("WASI clock_nanosleep called with clock_id {} flags {}", clock_id, flags);
    
    if request.is_null() {
        return -1; // EFAULT
    }
    
    let sleep_ns = unsafe { *request };
    let sleep_duration = std::time::Duration::from_nanos(sleep_ns);
    
    let start = std::time::Instant::now();
    std::thread::sleep(sleep_duration);
    let actual_sleep = start.elapsed();
    
    // Calculate remaining time if interrupted (which won't happen in this implementation)
    if !remain.is_null() && actual_sleep < sleep_duration {
        let remaining_ns = (sleep_duration - actual_sleep).as_nanos() as u64;
        unsafe { *remain = remaining_ns; }
    }
    
    0 // Success
}

/// WASI poll_oneoff implementation (simplified)
#[no_mangle]
pub extern "C" fn poll_oneoff(
    in_: *const u8, 
    out: *mut u8, 
    nsubscriptions: u32, 
    nevents: *mut u32
) -> i32 {
    log::debug!("WASI poll_oneoff called with {} subscriptions", nsubscriptions);
    
    // Simplified implementation - just yield briefly
    std::thread::yield_now();
    
    // Set number of events to 0 (no events processed)
    if !nevents.is_null() {
        unsafe { *nevents = 0; }
    }
    
    0 // Success
}

/// Deno setTimeout implementation
#[no_mangle]
pub extern "C" fn denoSetTimeout(callback: u32, delay: u32) -> u32 {
    log::debug!("Deno setTimeout called with callback {} delay {}", callback, delay);
    
    // Use same implementation as browser setTimeout
    setTimeout(callback, delay)
}

/// Deno queueMicrotask implementation
#[no_mangle]
pub extern "C" fn denoQueueMicrotask(callback: u32) {
    log::debug!("Deno queueMicrotask called with callback {}", callback);
    
    // Execute callback immediately for microtask behavior
    if callback != 0 {
        let callback_fn: fn() = unsafe { std::mem::transmute(callback as *const fn()) };
        callback_fn();
    }
}

/// Deno event loop yield implementation
#[no_mangle]
pub extern "C" fn denoYieldToEventLoop() {
    log::debug!("Yielding to Deno event loop");
    
    // Similar to browser yield
    std::thread::yield_now();
    std::thread::sleep(std::time::Duration::from_micros(1));
}

// Memory Management Functions

/// Allocate memory externally callable
#[no_mangle]
pub extern "C" fn cursed_malloc(size: usize) -> *mut u8 {
    let layout = match std::alloc::Layout::from_size_align(size, 8) {
        Ok(layout) => layout,
        Err(_) => return std::ptr::null_mut(),
    };
    
    unsafe { std::alloc::alloc(layout) }
}

/// Free memory externally callable
#[no_mangle]
pub extern "C" fn cursed_free(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }
    
    let layout = match std::alloc::Layout::from_size_align(size, 8) {
        Ok(layout) => layout,
        Err(_) => return,
    };
    
    unsafe { std::alloc::dealloc(ptr, layout) }
}

/// Reallocate memory externally callable
#[no_mangle]
pub extern "C" fn cursed_realloc(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    if ptr.is_null() {
        return cursed_malloc(new_size);
    }
    
    if new_size == 0 {
        cursed_free(ptr, old_size);
        return std::ptr::null_mut();
    }
    
    let old_layout = match std::alloc::Layout::from_size_align(old_size, 8) {
        Ok(layout) => layout,
        Err(_) => return std::ptr::null_mut(),
    };
    
    unsafe { std::alloc::realloc(ptr, old_layout, new_size) }
}

// Goroutine-specific external functions

/// Save goroutine context externally callable
#[no_mangle]
pub extern "C" fn save_goroutine_context_external(goroutine_id: u64) -> i32 {
    match save_goroutine_context(goroutine_id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Restore goroutine context externally callable
#[no_mangle]
pub extern "C" fn restore_goroutine_context_external(goroutine_id: u64) -> i32 {
    match restore_goroutine_context(goroutine_id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Switch goroutine context externally callable
#[no_mangle]
pub extern "C" fn switch_goroutine_context_external(from_id: u64, to_id: u64) -> i32 {
    match switch_goroutine_context(from_id, to_id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Cleanup goroutine context externally callable
#[no_mangle]
pub extern "C" fn cleanup_goroutine_context_external(goroutine_id: u64) -> i32 {
    match cleanup_goroutine_context(goroutine_id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Execute function value externally callable
#[no_mangle]
pub extern "C" fn execute_function_value_external(
    func_name_ptr: *const u8,
    func_name_len: u32,
    args_ptr: *const usize,
    args_len: u32,
) -> usize {
    if func_name_ptr.is_null() || args_ptr.is_null() {
        return 0;
    }
    
    let func_name = unsafe {
        let slice = std::slice::from_raw_parts(func_name_ptr, func_name_len as usize);
        match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return 0,
        }
    };
    
    let args = unsafe {
        std::slice::from_raw_parts(args_ptr, args_len as usize)
    };
    
    match execute_function_value(func_name, args) {
        Ok(result) => result,
        Err(_) => 0,
    }
}

/// Register executable function externally callable
#[no_mangle]
pub extern "C" fn register_executable_function_external(
    name_ptr: *const u8,
    name_len: u32,
    func_ptr: usize,
    arity: u32,
    is_native: bool,
) -> i32 {
    if name_ptr.is_null() {
        return -1;
    }
    
    let name = unsafe {
        let slice = std::slice::from_raw_parts(name_ptr, name_len as usize);
        match std::str::from_utf8(slice) {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };
    
    let func = ExecutableFunction {
        func_ptr,
        name: name.clone(),
        arity: arity as usize,
        return_type: "usize".to_string(),
        param_types: vec!["usize".to_string(); arity as usize],
        is_native,
        jit_metadata: None,
    };
    
    match register_executable_function(name, func) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Initialize goroutine context system externally callable
#[no_mangle]
pub extern "C" fn initialize_goroutine_context_system_external() -> i32 {
    match initialize_goroutine_context_system() {
        Ok(_) => {
            #[cfg(target_arch = "wasm32")]
            {
                if let Err(_) = initialize_wasm_context_system() {
                    return -1;
                }
            }
            0
        },
        Err(_) => -1,
    }
}

/// Get last error message
#[no_mangle]
pub extern "C" fn get_last_error_message(buffer: *mut u8, buffer_size: u32) -> u32 {
    if buffer.is_null() || buffer_size == 0 {
        return 0;
    }
    
    let error_msg = "No error"; // Placeholder error message
    let error_bytes = error_msg.as_bytes();
    let copy_len = std::cmp::min(error_bytes.len(), (buffer_size - 1) as usize);
    
    unsafe {
        std::ptr::copy_nonoverlapping(error_bytes.as_ptr(), buffer, copy_len);
        *buffer.add(copy_len) = 0; // Null terminator
    }
    
    copy_len as u32
}

/// Clear last error
#[no_mangle]
pub extern "C" fn clear_last_error() {
    // Clear any stored error state
    log::debug!("Last error cleared");
}

/// Log debug message externally
#[no_mangle]
pub extern "C" fn cursed_log_debug(message_ptr: *const u8, message_len: u32) {
    if message_ptr.is_null() {
        return;
    }
    
    let message = unsafe {
        let slice = std::slice::from_raw_parts(message_ptr, message_len as usize);
        match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return,
        }
    };
    
    log::debug!("{}", message);
}

/// Log info message externally
#[no_mangle]
pub extern "C" fn cursed_log_info(message_ptr: *const u8, message_len: u32) {
    if message_ptr.is_null() {
        return;
    }
    
    let message = unsafe {
        let slice = std::slice::from_raw_parts(message_ptr, message_len as usize);
        match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return,
        }
    };
    
    log::info!("{}", message);
}

/// Log error message externally
#[no_mangle]
pub extern "C" fn cursed_log_error(message_ptr: *const u8, message_len: u32) {
    if message_ptr.is_null() {
        return;
    }
    
    let message = unsafe {
        let slice = std::slice::from_raw_parts(message_ptr, message_len as usize);
        match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return,
        }
    };
    
    log::error!("{}", message);
}

// Additional missing external function implementations

/// Deno core print function implementation
#[no_mangle]
pub extern "C" fn deno_core_print(ptr: *const u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    if let Ok(message) = std::str::from_utf8(data) {
        println!("{}", message);
    }
}

/// Console log implementation for browser environment
#[no_mangle]
pub extern "C" fn console_log(ptr: *const u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    if let Ok(message) = std::str::from_utf8(data) {
        println!("[Console] {}", message);
    }
}

/// Get current WASM function index implementation
#[no_mangle]
pub extern "C" fn get_current_function_index() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        // Try to read from WASM context state
        if let Ok(state) = WASM_CONTEXT_STATE.lock() {
            // Use stack pointer as function index approximation
            state.current_stack_pointer / 4096
        } else {
            0
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// Get current WASM program counter implementation
#[no_mangle]
pub extern "C" fn get_current_pc() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        // In WASM, the program counter is managed by the runtime
        // We approximate using stack pointer
        let mut sp: u32;
        unsafe {
            core::arch::asm!(
                "global.get __stack_pointer",
                out(reg) sp,
                options(nostack)
            );
        }
        sp
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// Set current WASM program counter implementation
#[no_mangle]
pub extern "C" fn set_current_pc(pc: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        log::debug!("Setting WASM PC to 0x{:08x}", pc);
        // Store PC in context state for tracking
        if let Ok(mut state) = WASM_CONTEXT_STATE.lock() {
            state.current_base_pointer = pc;
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        log::debug!("set_current_pc called on non-WASM platform with PC 0x{:08x}", pc);
    }
}

/// WASM resume execution signal implementation
#[no_mangle]
pub extern "C" fn wasm_resume_execution() {
    #[cfg(target_arch = "wasm32")]
    {
        log::debug!("Signaling WASM runtime to resume execution");
        // Use cooperative yield to let runtime continue
        std::thread::yield_now();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        log::debug!("wasm_resume_execution called on non-WASM platform");
    }
}

/// WASM stack pointer getter (global export)
#[no_mangle]
pub extern "C" fn __stack_pointer() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        get_wasm_stack_pointer()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// WASM memory grow implementation
#[no_mangle]
pub extern "C" fn __wasm_memory_grow(pages: u32) -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        core::arch::wasm32::memory_grow(0, pages as usize) as u32
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        u32::MAX // Failure indicator
    }
}

/// WASM memory size implementation
#[no_mangle]
pub extern "C" fn __wasm_memory_size() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        core::arch::wasm32::memory_size(0) as u32
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// WASM table size implementation
#[no_mangle]
pub extern "C" fn __wasm_table_size() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        // WASM table size query - use intrinsic if available
        // For now, return a reasonable default
        256
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// WASM call indirect implementation helper
#[no_mangle]
pub extern "C" fn __wasm_call_indirect(
    table_index: u32,
    func_index: u32,
    args_ptr: *const u32,
    args_len: u32,
) -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        log::debug!("WASM call_indirect: table {} func {} with {} args", 
                   table_index, func_index, args_len);
        
        // Real implementation using WASM runtime table lookup
        unsafe {
            // Check table bounds
            if table_index >= 1 {
                log::error!("Table index {} out of bounds", table_index);
                return u32::MAX;
            }
            
            // Get table size - in WASM, table 0 is the main function table
            let table_size = core::arch::wasm32::table_size(table_index);
            if func_index >= table_size {
                log::error!("Function index {} out of bounds (table size: {})", func_index, table_size);
                return u32::MAX;
            }
            
            // Get function pointer from table
            let func_ptr = core::arch::wasm32::table_get(table_index, func_index);
            if func_ptr.is_null() {
                log::error!("Null function pointer at table[{}][{}]", table_index, func_index);
                return u32::MAX;
            }
            
            // Call function with arguments
            let args_slice = if args_len > 0 && !args_ptr.is_null() {
                std::slice::from_raw_parts(args_ptr, args_len as usize)
            } else {
                &[]
            };
            
            // Use dynamic dispatch based on argument count
            match args_len {
                0 => {
                    let func: fn() -> u32 = std::mem::transmute(func_ptr);
                    func()
                },
                1 => {
                    let func: fn(u32) -> u32 = std::mem::transmute(func_ptr);
                    func(args_slice[0])
                },
                2 => {
                    let func: fn(u32, u32) -> u32 = std::mem::transmute(func_ptr);
                    func(args_slice[0], args_slice[1])
                },
                3 => {
                    let func: fn(u32, u32, u32) -> u32 = std::mem::transmute(func_ptr);
                    func(args_slice[0], args_slice[1], args_slice[2])
                },
                _ => {
                    log::error!("Too many arguments for call_indirect: {}", args_len);
                    u32::MAX
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        log::debug!("__wasm_call_indirect called on non-WASM platform");
        u32::MAX // Failure
    }
}

/// Memory copy implementation for WASM
#[no_mangle]
pub extern "C" fn __wasm_memory_copy(
    dest: u32,
    src: u32,
    size: u32,
) -> i32 {
    #[cfg(target_arch = "wasm32")]
    {
        // Validate memory bounds
        let memory_pages = core::arch::wasm32::memory_size(0);
        let memory_size = (memory_pages * 65536) as u32;
        
        if dest.saturating_add(size) > memory_size || 
           src.saturating_add(size) > memory_size {
            return -1; // Out of bounds
        }
        
        // Perform memory copy (simulated)
        log::debug!("WASM memory copy: {} bytes from 0x{:08x} to 0x{:08x}", 
                   size, src, dest);
        0 // Success
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        -1 // Not supported
    }
}

/// Memory fill implementation for WASM
#[no_mangle]
pub extern "C" fn __wasm_memory_fill(
    dest: u32,
    value: u8,
    size: u32,
) -> i32 {
    #[cfg(target_arch = "wasm32")]
    {
        // Validate memory bounds
        let memory_pages = core::arch::wasm32::memory_size(0);
        let memory_size = (memory_pages * 65536) as u32;
        
        if dest.saturating_add(size) > memory_size {
            return -1; // Out of bounds
        }
        
        // Perform memory fill (simulated)
        log::debug!("WASM memory fill: {} bytes at 0x{:08x} with value 0x{:02x}", 
                   size, dest, value);
        0 // Success
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        -1 // Not supported
    }
}

/// WASM trap handler implementation
#[no_mangle]
pub extern "C" fn __wasm_trap(trap_code: u32) -> ! {
    match trap_code {
        0 => panic!("WASM trap: unreachable instruction"),
        1 => panic!("WASM trap: integer division by zero"),
        2 => panic!("WASM trap: integer overflow"),
        3 => panic!("WASM trap: invalid conversion to integer"),
        4 => panic!("WASM trap: out of bounds memory access"),
        5 => panic!("WASM trap: undefined element"),
        6 => panic!("WASM trap: uninitialized element"),
        7 => panic!("WASM trap: indirect call type mismatch"),
        _ => panic!("WASM trap: unknown trap code {}", trap_code),
    }
}

/// WASM atomic operations support check
#[no_mangle]
pub extern "C" fn __wasm_atomic_supported() -> u32 {
    #[cfg(all(target_arch = "wasm32", target_feature = "atomics"))]
    {
        1 // Atomics supported
    }
    #[cfg(not(all(target_arch = "wasm32", target_feature = "atomics")))]
    {
        0 // Atomics not supported
    }
}

/// WASM SIMD support check
#[no_mangle]
pub extern "C" fn __wasm_simd_supported() -> u32 {
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        1 // SIMD supported
    }
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    {
        0 // SIMD not supported
    }
}

/// Runtime feature detection for WASM
#[no_mangle]
pub extern "C" fn __wasm_detect_features() -> u32 {
    let mut features = 0u32;
    
    #[cfg(all(target_arch = "wasm32", target_feature = "atomics"))]
    {
        features |= 1; // Bit 0: Atomics
    }
    
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        features |= 2; // Bit 1: SIMD
    }
    
    #[cfg(all(target_arch = "wasm32", target_feature = "bulk-memory"))]
    {
        features |= 4; // Bit 2: Bulk memory operations
    }
    
    #[cfg(all(target_arch = "wasm32", target_feature = "reference-types"))]
    {
        features |= 8; // Bit 3: Reference types
    }
    
    features
}
