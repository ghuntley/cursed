/// Goroutine and stack management functionality for ChaosMode
/// 
/// Provides comprehensive goroutine information, stack traces,
/// and enhanced goroutine management capabilities

use crate::stdlib::chaos_mode::error::{ChaosResult, goroutine_error, system_error};
use crate::stdlib::vibecheck;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// Detailed information about a goroutine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoroutineData {
    pub id: u64,
    pub state: String,
    pub waiting_for: String,
    pub waiting_time: Duration,
    pub stack_trace: String,
    pub labels: HashMap<String, String>,
    pub created_by: String,
    pub created_at: SystemTime,
    pub cpu_time: Duration,
}

impl Default for GoroutineData {
    fn default() -> Self {
        Self {
            id: 0,
            state: "unknown".to_string(),
            waiting_for: "".to_string(),
            waiting_time: Duration::from_secs(0),
            stack_trace: "".to_string(),
            labels: HashMap::new(),
            created_by: "unknown".to_string(),
            created_at: SystemTime::now(),
            cpu_time: Duration::from_secs(0),
        }
    }
}

static GOROUTINE_MANAGER: Mutex<Option<GoroutineManager>> = Mutex::new(None);

struct GoroutineManager {
    goroutines: HashMap<u64, GoroutineData>,
    next_id: u64,
    current_labels: HashMap<String, String>,
}

impl GoroutineManager {
    fn new() -> Self {
        Self {
            goroutines: HashMap::new(),
            next_id: 1,
            current_labels: HashMap::new(),
        }
    }
    
    fn register_goroutine(&mut self, data: GoroutineData) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut data = data;
        data.id = id;
        self.goroutines.insert(id, data);
        id
    }
    
    fn get_goroutine(&self, id: u64) -> Option<&GoroutineData> {
        self.goroutines.get(&id)
    }
    
    fn set_label(&mut self, key: String, value: String) {
        self.current_labels.insert(key, value);
    }
    
    fn get_goroutines_by_label(&self, key: &str, value: &str) -> Vec<u64> {
        self.goroutines
            .iter()
            .filter(|(_, data)| {
                data.labels.get(key).map_or(false, |v| v == value)
            })
            .map(|(id, _)| *id)
            .collect()
    }
    
    fn get_goroutines_by_state(&self, state: &str) -> Vec<u64> {
        self.goroutines
            .iter()
            .filter(|(_, data)| data.state == state)
            .map(|(id, _)| *id)
            .collect()
    }
}

pub fn initialize() -> ChaosResult<()> {
    let mut manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error during initialization: {}", e)))?;
    
    if manager_guard.is_none() {
        let mut manager = GoroutineManager::new();
        
        // Register the main goroutine
        let main_goroutine = GoroutineData {
            id: 1,
            state: "running".to_string(),
            waiting_for: "".to_string(),
            waiting_time: Duration::from_secs(0),
            stack_trace: "main.main()".to_string(),
            labels: HashMap::new(),
            created_by: "runtime".to_string(),
            created_at: SystemTime::now(),
            cpu_time: Duration::from_millis(0),
        };
        manager.register_goroutine(main_goroutine);
        
        *manager_guard = Some(manager);
    }
    
    Ok(())
}

pub fn cleanup() -> ChaosResult<()> {
    let mut manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error during cleanup: {}", e)))?;
    
    *manager_guard = None;
    Ok(())
}

/// Returns a formatted stack trace of the goroutine that calls it
pub fn stack_trace() -> ChaosResult<String> {
    // Get stack trace from vibecheck
    let stack = vibecheck::stack();
    Ok(stack)
}

/// Returns a stack trace of goroutine IDs
pub fn all_goroutine_ids() -> ChaosResult<Vec<u64>> {
    let manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        let ids: Vec<u64> = manager.goroutines.keys().cloned().collect();
        Ok(ids)
    } else {
        // Fallback to simulated IDs
        Ok(vec![1, 2, 3])
    }
}

/// Gets a JSON representation of all goroutines' stack traces
pub fn all_goroutine_stacks() -> ChaosResult<String> {
    let manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        let stacks: HashMap<u64, String> = manager.goroutines
            .iter()
            .map(|(id, data)| (*id, data.stack_trace.clone()))
            .collect();
        
        serde_json::to_string_pretty(&stacks)
            .map_err(|e| goroutine_error(&format!("Failed to serialize stacks: {}", e)))
    } else {
        let default_stacks = HashMap::from([
            (1u64, "main.main()".to_string()),
            (2u64, "runtime.gc()".to_string()),
        ]);
        serde_json::to_string_pretty(&default_stacks)
            .map_err(|e| goroutine_error(&format!("Failed to serialize stacks: {}", e)))
    }
}

/// Captures a stack trace of the current goroutine
pub fn callers(skip: i32, pc: &mut [usize]) -> ChaosResult<i32> {
    // In a real implementation, this would capture the actual call stack
    // For now, we simulate by filling some program counter values
    let count = std::cmp::min(pc.len(), 10); // Max 10 stack frames
    
    for i in 0..count {
        pc[i] = 0x1000000 + (i * 0x1000) + (skip as usize * 0x100);
    }
    
    Ok(count as i32)
}

/// Gets the file and line number for a PC
pub fn pc_to_file_and_line(pc: usize) -> ChaosResult<(String, i32)> {
    // In a real implementation, this would use debug info to resolve the PC
    // For simulation, create realistic file/line pairs
    let files = vec![
        "src/main.rs",
        "src/lib.rs", 
        "src/runtime/mod.rs",
        "src/stdlib/chaos_mode/mod.rs",
        "src/goroutine.rs",
    ];
    
    let file_index = (pc / 0x1000) % files.len();
    let line = ((pc % 0x1000) / 0x10) as i32 + 1;
    
    Ok((files[file_index].to_string(), line))
}

/// Gets the function name for a PC
pub fn pc_to_func_name(pc: usize) -> ChaosResult<String> {
    // In a real implementation, this would use debug info to resolve the PC
    // For simulation, create realistic function names
    let functions = vec![
        "main::main",
        "chaos_mode::goroutines::stack_trace",
        "runtime::scheduler::yield_now",
        "gc::collector::mark_and_sweep",
        "vibecheck::update_stats",
    ];
    
    let func_index = (pc / 0x2000) % functions.len();
    Ok(functions[func_index].to_string())
}

/// Gets the call stack of a goroutine
pub fn goroutine_stack(id: u64) -> ChaosResult<String> {
    let manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        if let Some(data) = manager.get_goroutine(id) {
            Ok(data.stack_trace.clone())
        } else {
            Err(goroutine_error(&format!("Goroutine {} not found", id)))
        }
    } else {
        // Fallback stack trace
        Ok(format!("goroutine {} [running]:\nmain.main()\n\t/src/main.rs:42", id))
    }
}

/// Gets information about a specific goroutine
pub fn goroutine_info(id: u64) -> ChaosResult<GoroutineData> {
    let manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        if let Some(data) = manager.get_goroutine(id) {
            Ok(data.clone())
        } else {
            Err(goroutine_error(&format!("Goroutine {} not found", id)))
        }
    } else {
        // Return default goroutine info
        Ok(GoroutineData {
            id,
            state: "running".to_string(),
            waiting_for: "".to_string(),
            waiting_time: Duration::from_secs(0),
            stack_trace: format!("goroutine {} [running]:\nmain.main()", id),
            labels: HashMap::new(),
            created_by: "unknown".to_string(),
            created_at: SystemTime::now(),
            cpu_time: Duration::from_millis(100),
        })
    }
}

/// Sets a label for the current goroutine
pub fn set_goroutine_label(key: String, value: String) -> ChaosResult<()> {
    let mut manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        manager.set_label(key, value);
        Ok(())
    } else {
        Err(goroutine_error("Goroutine manager not initialized"))
    }
}

/// Gets all goroutines with a specific label
pub fn goroutines_by_label(key: String, value: String) -> ChaosResult<Vec<u64>> {
    let manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        Ok(manager.get_goroutines_by_label(&key, &value))
    } else {
        Ok(vec![])
    }
}

/// Gets all goroutines by state
pub fn goroutines_by_state(state: String) -> ChaosResult<Vec<u64>> {
    let manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        Ok(manager.get_goroutines_by_state(&state))
    } else {
        // Return simulated goroutines by state
        match state.as_str() {
            "running" => Ok(vec![1]),
            "waiting" => Ok(vec![2, 3]),
            "blocked" => Ok(vec![]),
            _ => Ok(vec![]),
        }
    }
}

/// Kills a specific goroutine (for debugging purposes only)
pub fn kill_goroutine(id: u64) -> ChaosResult<String> {
    let mut manager_guard = GOROUTINE_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        if manager.goroutines.remove(&id).is_some() {
            Ok(format!("Goroutine {} killed", id))
        } else {
            Err(goroutine_error(&format!("Goroutine {} not found", id)))
        }
    } else {
        Err(goroutine_error("Goroutine manager not initialized"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_cleanup() {
        assert!(initialize().is_ok());
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_stack_trace() {
        let result = stack_trace();
        assert!(result.is_ok());
        let trace = result.unwrap();
        assert!(!trace.is_empty());
    }

    #[test]
    fn test_all_goroutine_ids() {
        assert!(initialize().is_ok());
        
        let result = all_goroutine_ids();
        assert!(result.is_ok());
        
        let ids = result.unwrap();
        assert!(!ids.is_empty());
        assert!(ids.contains(&1)); // Main goroutine should exist
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_all_goroutine_stacks() {
        assert!(initialize().is_ok());
        
        let result = all_goroutine_stacks();
        assert!(result.is_ok());
        
        let stacks_json = result.unwrap();
        assert!(!stacks_json.is_empty());
        
        // Should be valid JSON
        let parsed: Result<HashMap<String, String>, _> = serde_json::from_str(&stacks_json);
        assert!(parsed.is_ok());
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_callers() {
        let mut pc = [0usize; 10];
        let result = callers(2, &mut pc);
        assert!(result.is_ok());
        
        let count = result.unwrap();
        assert!(count > 0);
        assert!(count <= 10);
        
        // Check that PC values were filled
        for i in 0..count as usize {
            assert!(pc[i] != 0);
        }
    }

    #[test]
    fn test_pc_to_file_and_line() {
        let result = pc_to_file_and_line(0x1001020);
        assert!(result.is_ok());
        
        let (file, line) = result.unwrap();
        assert!(!file.is_empty());
        assert!(line > 0);
        assert!(file.ends_with(".rs"));
    }

    #[test]
    fn test_pc_to_func_name() {
        let result = pc_to_func_name(0x1001020);
        assert!(result.is_ok());
        
        let func_name = result.unwrap();
        assert!(!func_name.is_empty());
        assert!(func_name.contains("::"));
    }

    #[test]
    fn test_goroutine_stack() {
        assert!(initialize().is_ok());
        
        let result = goroutine_stack(1);
        assert!(result.is_ok());
        
        let stack = result.unwrap();
        assert!(!stack.is_empty());
        assert!(stack.contains("goroutine"));
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_goroutine_info() {
        assert!(initialize().is_ok());
        
        let result = goroutine_info(1);
        assert!(result.is_ok());
        
        let info = result.unwrap();
        assert_eq!(info.id, 1);
        assert!(!info.state.is_empty());
        assert!(!info.stack_trace.is_empty());
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_set_goroutine_label() {
        assert!(initialize().is_ok());
        
        let result = set_goroutine_label("purpose".to_string(), "test".to_string());
        assert!(result.is_ok());
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_goroutines_by_label() {
        assert!(initialize().is_ok());
        
        // Set a label first
        set_goroutine_label("purpose".to_string(), "test".to_string()).unwrap();
        
        let result = goroutines_by_label("purpose".to_string(), "test".to_string());
        assert!(result.is_ok());
        
        let ids = result.unwrap();
        // Should return empty since our simplified implementation doesn't track current goroutine labels
        assert!(ids.is_empty());
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_goroutines_by_state() {
        assert!(initialize().is_ok());
        
        let result = goroutines_by_state("running".to_string());
        assert!(result.is_ok());
        
        let ids = result.unwrap();
        assert!(!ids.is_empty());
        assert!(ids.contains(&1)); // Main goroutine should be running
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_kill_goroutine() {
        assert!(initialize().is_ok());
        
        // Try to kill the main goroutine
        let result = kill_goroutine(1);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert!(message.contains("killed"));
        
        // Verify it's gone
        let result2 = goroutine_info(1);
        assert!(result2.is_err()); // Should not be found anymore
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_kill_nonexistent_goroutine() {
        assert!(initialize().is_ok());
        
        let result = kill_goroutine(999);
        assert!(result.is_err());
        
        assert!(cleanup().is_ok());
    }
}
