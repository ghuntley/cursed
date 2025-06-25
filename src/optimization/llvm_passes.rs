//! LLVM passes integration

pub struct LlvmPassManager;
pub struct LtoManager;
pub struct PgoManager;

impl LlvmPassManager {
    pub fn new() -> Self { LlvmPassManager }
}

impl LtoManager {
    pub fn new() -> Self { LtoManager }
}

impl PgoManager {
    pub fn new() -> Self { PgoManager }
}
