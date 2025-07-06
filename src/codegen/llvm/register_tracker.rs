//! Register tracking to ensure consecutive numbering

use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct RegisterTracker {
    allocated: HashSet<usize>,
    next_expected: usize,
}

impl RegisterTracker {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn allocate_register(&mut self) -> String {
        let reg = self.next_expected;
        self.allocated.insert(reg);
        self.next_expected += 1;
        format!("%{}", reg)
    }
    
    pub fn validate(&self) -> Result<(), String> {
        for i in 0..self.next_expected {
            if !self.allocated.contains(&i) {
                return Err(format!("Missing register %{}", i));
            }
        }
        Ok(())
    }
    
    pub fn get_next_number(&self) -> usize {
        self.next_expected
    }
}
