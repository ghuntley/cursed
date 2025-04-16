// The code module for CURSED language
// This will contain bytecode definitions in the future

/// Bytecode format for the CURSED VM
pub struct Code {
    pub instructions: Vec<u8>,
}

impl Code {
    /// Create a new code object
    #[tracing::instrument(level = "debug")]
    pub fn new() -> Self {
        tracing::debug!("Creating new Code object");
        Code {
            instructions: Vec::new(),
        }
    }
}

/// Opcode for the VM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0,
    // Actual opcodes will be added in future versions
}
