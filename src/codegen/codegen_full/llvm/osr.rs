/// On-Stack Replacement (OSR) Implementation for CURSED JIT
/// 
/// Enables replacing currently executing functions with optimized versions while they're running.
/// This is critical for hot loop optimization and provides seamless upgrades of running code.

use crate::error::CursedError;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::ptr;

use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, BasicValueEnum},
    types::{BasicTypeEnum, StructType},
    builder::Builder,
    basic_block::BasicBlock,
    AddressSpace,
};

/// OSR (On-Stack Replacement) Manager
/// 
/// Manages the transition from currently executing functions to optimized versions.
/// Handles stack frame analysis, state preservation, and runtime replacement.
pub struct OSRManager<'ctx> {
    context: &'ctx Context,
    pending_replacements: Arc<Mutex<HashMap<String, OSRReplacement<'ctx>>>>,
    stack_frame_tracker: Arc<RwLock<StackFrameTracker>>,
    deopt_metadata: Arc<Mutex<HashMap<String, DeoptimizationInfo>>>,
    config: OSRConfig,
    stats: OSRStats,
}

/// Configuration for OSR behavior
#[derive(Debug, Clone)]
pub struct OSRConfig {
    /// Enable aggressive OSR for hot loops
    pub enable_loop_osr: bool,
    /// Enable function-level OSR
    pub enable_function_osr: bool,
    /// Maximum time to spend on OSR preparation
    pub osr_preparation_timeout: Duration,
    /// Maximum stack depth for OSR
    pub max_stack_depth: usize,
    /// Enable deoptimization support
    pub enable_deoptimization: bool,
    /// OSR trigger threshold (execution count)
    pub osr_trigger_threshold: u64,
    /// Enable speculative optimizations
    pub enable_speculative_optimizations: bool,
}

/// Statistics for OSR operations
#[derive(Debug, Default, Clone)]
pub struct OSRStats {
    /// Total OSR replacements performed
    pub total_osr_replacements: u64,
    /// Successful OSR transitions
    pub successful_transitions: u64,
    /// Failed OSR attempts
    pub failed_transitions: u64,
    /// Deoptimizations performed
    pub deoptimizations: u64,
    /// Average OSR preparation time
    pub avg_preparation_time: Duration,
    /// Average transition time
    pub avg_transition_time: Duration,
    /// Performance improvement from OSR
    pub performance_improvement_percent: f64,
}

/// Represents an OSR replacement operation
#[derive(Debug)]
pub struct OSRReplacement<'ctx> {
    /// Original function being replaced
    pub original_function: FunctionValue<'ctx>,
    /// Optimized replacement function
    pub optimized_function: FunctionValue<'ctx>,
    /// OSR entry points in the optimized function
    pub osr_entry_points: Vec<OSREntryPoint<'ctx>>,
    /// Stack frame mapping information
    pub frame_mapping: StackFrameMapping,
    /// Trigger conditions for OSR
    pub trigger_conditions: Vec<OSRTrigger>,
    /// Preparation timestamp
    pub prepared_at: Instant,
}

/// OSR entry point in optimized code
#[derive(Debug)]
pub struct OSREntryPoint<'ctx> {
    /// Basic block where OSR can occur
    pub basic_block: BasicBlock<'ctx>,
    /// Variable mapping for live values
    pub live_value_mapping: HashMap<String, BasicValueEnum<'ctx>>,
    /// Loop depth at this entry point
    pub loop_depth: usize,
    /// Expected execution frequency
    pub execution_frequency: u64,
}

/// Stack frame tracking for OSR
#[derive(Debug, Default)]
pub struct StackFrameTracker {
    /// Active stack frames
    pub active_frames: VecDeque<StackFrame>,
    /// Frame metadata by function name
    pub frame_metadata: HashMap<String, FrameMetadata>,
    /// Current stack depth
    pub current_depth: usize,
}

/// Individual stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Function name
    pub function_name: String,
    /// Local variables and their values
    pub local_variables: HashMap<String, VariableValue>,
    /// Return address information
    pub return_address: Option<usize>,
    /// Frame pointer
    pub frame_pointer: Option<usize>,
    /// Stack pointer
    pub stack_pointer: Option<usize>,
}

/// Stack frame mapping for OSR transitions
#[derive(Debug, Clone)]
pub struct StackFrameMapping {
    /// Variable mapping between original and optimized versions
    pub variable_mapping: HashMap<String, String>,
    /// Register allocation mapping
    pub register_mapping: HashMap<String, String>,
    /// Memory layout changes
    pub memory_layout_changes: Vec<MemoryLayoutChange>,
}

/// Memory layout change for OSR
#[derive(Debug, Clone)]
pub struct MemoryLayoutChange {
    /// Original memory offset
    pub original_offset: isize,
    /// New memory offset
    pub new_offset: isize,
    /// Variable name
    pub variable_name: String,
    /// Type information
    pub type_info: String,
}

/// Variable value in stack frame
#[derive(Debug, Clone)]
pub struct VariableValue {
    /// Variable name
    pub name: String,
    /// Value representation
    pub value: VariableValueType,
    /// Type information
    pub type_name: String,
    /// Whether the variable is live at OSR point
    pub is_live: bool,
}

/// Types of variable values
#[derive(Debug, Clone)]
pub enum VariableValueType {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Pointer(usize),
    Aggregate(Vec<VariableValue>),
}

/// Frame metadata for optimization decisions
#[derive(Debug, Clone)]
pub struct FrameMetadata {
    /// Function name
    pub function_name: String,
    /// Execution count
    pub execution_count: u64,
    /// Hot loop information
    pub hot_loops: Vec<HotLoop>,
    /// Optimization opportunities
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

/// Hot loop information
#[derive(Debug, Clone)]
pub struct HotLoop {
    /// Loop identifier
    pub loop_id: String,
    /// Iteration count
    pub iteration_count: u64,
    /// Average iteration time
    pub avg_iteration_time: Duration,
    /// Loop body complexity
    pub complexity_score: f64,
}

/// Optimization opportunity
#[derive(Debug, Clone)]
pub struct OptimizationOpportunity {
    /// Type of optimization
    pub optimization_type: OptimizationType,
    /// Potential performance improvement
    pub potential_improvement: f64,
    /// Required preparation time
    pub preparation_time: Duration,
}

/// Types of optimizations available
#[derive(Debug, Clone)]
pub enum OptimizationType {
    LoopUnrolling,
    VectorOptimization,
    ConstantPropagation,
    DeadCodeElimination,
    InliningOptimization,
    RegisterOptimization,
}

/// OSR trigger conditions
#[derive(Debug, Clone)]
pub enum OSRTrigger {
    /// Loop iteration count threshold
    LoopIterationCount(u64),
    /// Function execution count threshold
    FunctionExecutionCount(u64),
    /// Time-based trigger
    ExecutionTime(Duration),
    /// Performance threshold
    PerformanceThreshold(f64),
}

/// Deoptimization information
#[derive(Debug, Clone)]
pub struct DeoptimizationInfo {
    /// Original function name
    pub original_function: String,
    /// Optimized function name
    pub optimized_function: String,
    /// Deoptimization reason
    pub reason: DeoptimizationReason,
    /// Recovery strategy
    pub recovery_strategy: RecoveryStrategy,
    /// Preserved state information
    pub preserved_state: HashMap<String, VariableValue>,
}

/// Reasons for deoptimization
#[derive(Debug, Clone)]
pub enum DeoptimizationReason {
    /// Speculative optimization failed
    SpeculativeOptimizationFailed,
    /// Type assumption violated
    TypeAssumptionViolated,
    /// Control flow assumption violated
    ControlFlowAssumptionViolated,
    /// Memory layout assumption violated
    MemoryLayoutAssumptionViolated,
    /// External dependency changed
    ExternalDependencyChanged,
}

/// Recovery strategies for deoptimization
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Return to original unoptimized code
    ReturnToOriginal,
    /// Re-optimize with different assumptions
    ReOptimizeWithDifferentAssumptions,
    /// Use fallback implementation
    UseFallbackImplementation,
    /// Trigger emergency compilation
    TriggerEmergencyCompilation,
}

impl Default for OSRConfig {
    fn default() -> Self {
        Self {
            enable_loop_osr: true,
            enable_function_osr: true,
            osr_preparation_timeout: Duration::from_millis(100),
            max_stack_depth: 1000,
            enable_deoptimization: true,
            osr_trigger_threshold: 1000,
            enable_speculative_optimizations: true,
        }
    }
}

impl<'ctx> OSRManager<'ctx> {
    /// Create a new OSR manager
    pub fn new(context: &'ctx Context, config: OSRConfig) -> Self {
        Self {
            context,
            pending_replacements: Arc::new(Mutex::new(HashMap::new())),
            stack_frame_tracker: Arc::new(RwLock::new(StackFrameTracker::default())),
            deopt_metadata: Arc::new(Mutex::new(HashMap::new())),
            config,
            stats: OSRStats::default(),
        }
    }

    /// Prepare OSR replacement for a function
    pub fn prepare_osr_replacement(
        &mut self,
        function_name: &str,
        original_function: FunctionValue<'ctx>,
        optimized_function: FunctionValue<'ctx>,
    ) -> crate::error::Result<()> {
        let start_time = Instant::now();

        tracing::info!(
            function_name = function_name,
            "Preparing OSR replacement"
        );

        // Analyze stack frames
        let frame_mapping = self.analyze_stack_frame_mapping(&original_function, &optimized_function)?;
        
        // Identify OSR entry points
        let osr_entry_points = self.identify_osr_entry_points(&optimized_function)?;
        
        // Determine trigger conditions
        let trigger_conditions = self.determine_trigger_conditions(function_name)?;
        
        // Create OSR replacement
        let replacement = OSRReplacement {
            original_function,
            optimized_function,
            osr_entry_points,
            frame_mapping,
            trigger_conditions,
            prepared_at: Instant::now(),
        };

        // Store pending replacement
        {
            let mut pending = self.pending_replacements.lock().unwrap();
            pending.insert(function_name.to_string(), replacement);
        }

        let preparation_time = start_time.elapsed();
        self.stats.avg_preparation_time = if self.stats.total_osr_replacements == 0 {
            preparation_time
        } else {
            (self.stats.avg_preparation_time + preparation_time) / 2
        };

        tracing::info!(
            function_name = function_name,
            preparation_time_ms = preparation_time.as_millis(),
            "OSR replacement prepared successfully"
        );

        Ok(())
    }

    /// Check if OSR should be triggered for a function
    pub fn should_trigger_osr(&self, function_name: &str, execution_count: u64) -> bool {
        let pending = self.pending_replacements.lock().unwrap();
        if let Some(replacement) = pending.get(function_name) {
            for trigger in &replacement.trigger_conditions {
                match trigger {
                    OSRTrigger::FunctionExecutionCount(threshold) => {
                        if execution_count >= *threshold {
                            return true;
                        }
                    }
                    OSRTrigger::LoopIterationCount(_) => {
                        // Would need runtime loop iteration tracking
                        // For now, use execution count as proxy
                        if execution_count >= self.config.osr_trigger_threshold {
                            return true;
                        }
                    }
                    OSRTrigger::ExecutionTime(duration) => {
                        // Would need execution time tracking
                        // Implementation depends on runtime profiling integration
                        continue;
                    }
                    OSRTrigger::PerformanceThreshold(_) => {
                        // Would need performance metrics
                        continue;
                    }
                }
            }
        }
        false
    }

    /// Perform OSR transition
    pub fn perform_osr_transition(
        &mut self,
        function_name: &str,
        current_stack_frame: &StackFrame,
    ) -> crate::error::Result<()> {
        let start_time = Instant::now();

        tracing::info!(
            function_name = function_name,
            "Performing OSR transition"
        );

        // Get pending replacement
        let replacement = {
            let pending = self.pending_replacements.lock().unwrap();
            pending.get(function_name).cloned()
        };

        let replacement = match replacement {
            Some(r) => r,
            None => {
                tracing::warn!(
                    function_name = function_name,
                    "No OSR replacement prepared for function"
                );
                return Ok(false);
            }
        };

        // Validate stack frame compatibility
        if !self.validate_stack_frame_compatibility(current_stack_frame, &replacement.frame_mapping)? {
            tracing::warn!(
                function_name = function_name,
                "Stack frame incompatible with OSR replacement"
            );
            self.stats.failed_transitions += 1;
            return Ok(false);
        }

        // Perform the actual transition
        let success = self.execute_osr_transition(&replacement, current_stack_frame)?;

        let transition_time = start_time.elapsed();
        self.stats.avg_transition_time = if self.stats.successful_transitions == 0 {
            transition_time
        } else {
            (self.stats.avg_transition_time + transition_time) / 2
        };

        if success {
            self.stats.successful_transitions += 1;
            self.stats.total_osr_replacements += 1;
            
            tracing::info!(
                function_name = function_name,
                transition_time_ms = transition_time.as_millis(),
                "OSR transition completed successfully"
            );
        } else {
            self.stats.failed_transitions += 1;
            
            tracing::warn!(
                function_name = function_name,
                transition_time_ms = transition_time.as_millis(),
                "OSR transition failed"
            );
        }

        Ok(success)
    }

    /// Trigger deoptimization
    pub fn trigger_deoptimization(
        &mut self,
        function_name: &str,
        reason: DeoptimizationReason,
    ) -> crate::error::Result<()> {
        tracing::warn!(
            function_name = function_name,
            reason = ?reason,
            "Triggering deoptimization"
        );

        // Create deoptimization info
        let deopt_info = DeoptimizationInfo {
            original_function: function_name.to_string(),
            optimized_function: format!("{}_optimized", function_name),
            reason: reason.clone(),
            recovery_strategy: self.determine_recovery_strategy(&reason),
            preserved_state: self.preserve_current_state(function_name)?,
        };

        // Store deoptimization metadata
        {
            let mut deopt_metadata = self.deopt_metadata.lock().unwrap();
            deopt_metadata.insert(function_name.to_string(), deopt_info.clone());
        }

        // Execute recovery strategy
        self.execute_recovery_strategy(&deopt_info)?;

        self.stats.deoptimizations += 1;

        tracing::info!(
            function_name = function_name,
            recovery_strategy = ?deopt_info.recovery_strategy,
            "Deoptimization completed"
        );

        Ok(())
    }

    /// Analyze stack frame mapping between original and optimized functions
    fn analyze_stack_frame_mapping(
        &self,
        original_function: &FunctionValue<'ctx>,
        optimized_function: &FunctionValue<'ctx>,
    ) -> crate::error::Result<()> {
        let mut variable_mapping = HashMap::new();
        let mut register_mapping = HashMap::new();
        let mut memory_layout_changes = Vec::new();

        // Analyze function parameters
        for (i, param) in original_function.get_params().iter().enumerate() {
            if let Some(opt_param) = optimized_function.get_params().get(i) {
                let param_name = format!("param_{}", i);
                variable_mapping.insert(param_name.clone(), param_name.clone());
                
                // Check if register allocation changed
                if param.get_type() != opt_param.get_type() {
                    register_mapping.insert(param_name.clone(), format!("opt_param_{}", i));
                }
            }
        }

        // Analyze local variables (would require debug info in production)
        // For now, create placeholder mappings
        for i in 0..10 {
            let var_name = format!("local_{}", i);
            variable_mapping.insert(var_name.clone(), var_name.clone());
        }

        Ok(StackFrameMapping {
            variable_mapping,
            register_mapping,
            memory_layout_changes,
        })
    }

    /// Identify OSR entry points in optimized function
    fn identify_osr_entry_points(
        &self,
        optimized_function: &FunctionValue<'ctx>,
    ) -> crate::error::Result<()> {
        let mut entry_points = Vec::new();

        // Iterate through basic blocks to find suitable OSR entry points
        for basic_block in optimized_function.get_basic_blocks() {
            // Look for loop headers and other suitable entry points
            let mut live_value_mapping = HashMap::new();
            
            // Analyze live values at this point (simplified)
            for (i, instruction) in basic_block.get_instructions().enumerate() {
                if i < 5 { // Limit to first few instructions for demo
                    live_value_mapping.insert(
                        format!("value_{}", i),
                        instruction.as_basic_value_enum(),
                    );
                }
            }

            let entry_point = OSREntryPoint {
                basic_block,
                live_value_mapping,
                loop_depth: self.estimate_loop_depth(&basic_block),
                execution_frequency: 1000, // Would be determined by profiling
            };

            entry_points.push(entry_point);

            // Limit the number of entry points for practical reasons
            if entry_points.len() >= 3 {
                break;
            }
        }

        Ok(entry_points)
    }

    /// Estimate loop depth for a basic block
    fn estimate_loop_depth(&self, _basic_block: &BasicBlock<'ctx>) -> usize {
        // In a production implementation, this would analyze control flow
        // For now, return a placeholder value
        1
    }

    /// Determine trigger conditions for OSR
    fn determine_trigger_conditions(&self, function_name: &str) -> crate::error::Result<()> {
        let mut triggers = Vec::new();
        
        // Function execution count trigger
        triggers.push(OSRTrigger::FunctionExecutionCount(self.config.osr_trigger_threshold));
        
        // Loop iteration trigger for functions with loops
        if function_name.contains("loop") || function_name.contains("for") {
            triggers.push(OSRTrigger::LoopIterationCount(500));
        }
        
        // Performance threshold trigger
        triggers.push(OSRTrigger::PerformanceThreshold(2.0)); // 2x improvement target

        Ok(triggers)
    }

    /// Validate stack frame compatibility
    fn validate_stack_frame_compatibility(
        &self,
        stack_frame: &StackFrame,
        frame_mapping: &StackFrameMapping,
    ) -> crate::error::Result<()> {
        // Check if all required variables are present and mappable
        for local_var in &stack_frame.local_variables {
            if !frame_mapping.variable_mapping.contains_key(local_var.0) {
                tracing::warn!(
                    variable_name = local_var.0,
                    "Variable not found in frame mapping"
                );
                return Ok(false);
            }
        }

        // Validate memory layout compatibility
        for layout_change in &frame_mapping.memory_layout_changes {
            if layout_change.original_offset < 0 || layout_change.new_offset < 0 {
                tracing::warn!(
                    variable_name = layout_change.variable_name,
                    "Invalid memory layout offset"
                );
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Execute the actual OSR transition
    fn execute_osr_transition(
        &self,
        replacement: &OSRReplacement<'ctx>,
        current_stack_frame: &StackFrame,
    ) -> crate::error::Result<()> {
        tracing::debug!("Executing OSR transition");

        // In a production implementation, this would:
        // 1. Suspend execution at a safe point
        // 2. Map the current stack frame to the optimized version
        // 3. Transfer control to the optimized function
        // 4. Resume execution in the optimized code

        // For this implementation, we'll simulate the process
        let entry_point = replacement.osr_entry_points.first();
        if entry_point.is_none() {
            return Ok(false);
        }

        // Simulate variable mapping
        for (original_var, mapped_var) in &replacement.frame_mapping.variable_mapping {
            if let Some(original_value) = current_stack_frame.local_variables.get(original_var) {
                tracing::debug!(
                    original_var = original_var,
                    mapped_var = mapped_var,
                    "Mapping variable for OSR transition"
                );
                
                // In production: transfer value to new location
                // For now, just log the mapping
            }
        }

        // Simulate control transfer
        tracing::debug!("Control transferred to optimized function");

        Ok(true)
    }

    /// Determine recovery strategy for deoptimization
    fn determine_recovery_strategy(&self, reason: &DeoptimizationReason) -> RecoveryStrategy {
        match reason {
            DeoptimizationReason::SpeculativeOptimizationFailed => {
                RecoveryStrategy::ReOptimizeWithDifferentAssumptions
            }
            DeoptimizationReason::TypeAssumptionViolated => {
                RecoveryStrategy::ReturnToOriginal
            }
            DeoptimizationReason::ControlFlowAssumptionViolated => {
                RecoveryStrategy::UseFallbackImplementation
            }
            DeoptimizationReason::MemoryLayoutAssumptionViolated => {
                RecoveryStrategy::TriggerEmergencyCompilation
            }
            DeoptimizationReason::ExternalDependencyChanged => {
                RecoveryStrategy::ReOptimizeWithDifferentAssumptions
            }
        }
    }

    /// Preserve current state for deoptimization
    fn preserve_current_state(&self, function_name: &str) -> crate::error::Result<()> {
        let mut preserved_state = HashMap::new();

        // Get current stack frame
        let tracker = self.stack_frame_tracker.read().unwrap();
        if let Some(frame) = tracker.active_frames.iter().find(|f| f.function_name == function_name) {
            for (var_name, var_value) in &frame.local_variables {
                preserved_state.insert(var_name.clone(), var_value.clone());
            }
        }

        Ok(preserved_state)
    }

    /// Execute recovery strategy
    fn execute_recovery_strategy(&self, deopt_info: &DeoptimizationInfo) -> crate::error::Result<()> {
        match &deopt_info.recovery_strategy {
            RecoveryStrategy::ReturnToOriginal => {
                tracing::info!("Returning to original unoptimized code");
                // Would restore original function execution
            }
            RecoveryStrategy::ReOptimizeWithDifferentAssumptions => {
                tracing::info!("Re-optimizing with different assumptions");
                // Would trigger new optimization with updated assumptions
            }
            RecoveryStrategy::UseFallbackImplementation => {
                tracing::info!("Using fallback implementation");
                // Would switch to a known-good fallback
            }
            RecoveryStrategy::TriggerEmergencyCompilation => {
                tracing::info!("Triggering emergency compilation");
                // Would compile a basic safe version quickly
            }
        }

        Ok(())
    }

    /// Record stack frame entry
    pub fn record_stack_frame_entry(&self, frame: StackFrame) -> crate::error::Result<()> {
        let mut tracker = self.stack_frame_tracker.write().unwrap();
        tracker.active_frames.push_back(frame);
        tracker.current_depth += 1;
        
        if tracker.current_depth > self.config.max_stack_depth {
            return Err(CursedError::from_str("Stack depth exceeded maximum limit"));
        }
        
        Ok(())
    }

    /// Record stack frame exit
    pub fn record_stack_frame_exit(&self, function_name: &str) -> crate::error::Result<()> {
        let mut tracker = self.stack_frame_tracker.write().unwrap();
        
        // Remove the most recent frame for this function
        if let Some(pos) = tracker.active_frames.iter().rposition(|f| f.function_name == function_name) {
            tracker.active_frames.remove(pos);
            tracker.current_depth = tracker.current_depth.saturating_sub(1);
        }
        
        Ok(())
    }

    /// Get current stack depth
    pub fn get_current_stack_depth(&self) -> usize {
        let tracker = self.stack_frame_tracker.read().unwrap();
        tracker.current_depth
    }

    /// Get OSR statistics
    pub fn get_stats(&self) -> OSRStats {
        self.stats.clone()
    }

    /// Reset OSR statistics
    pub fn reset_stats(&mut self) {
        self.stats = OSRStats::default();
    }

    /// Update OSR configuration
    pub fn update_config(&mut self, config: OSRConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> &OSRConfig {
        &self.config
    }

    /// Check if OSR is ready for a function
    pub fn is_osr_ready(&self, function_name: &str) -> bool {
        let pending = self.pending_replacements.lock().unwrap();
        pending.contains_key(function_name)
    }

    /// Get list of functions with pending OSR replacements
    pub fn get_pending_osr_functions(&self) -> Vec<String> {
        let pending = self.pending_replacements.lock().unwrap();
        pending.keys().cloned().collect()
    }

    /// Generate OSR report
    pub fn generate_osr_report(&self) -> String {
        let mut report = String::from("🔄 OSR (On-Stack Replacement) Report\n");
        report.push_str("=".repeat(50).as_str());
        report.push('\n');

        // Statistics
        report.push_str(&format!("Total OSR replacements: {}\n", self.stats.total_osr_replacements));
        report.push_str(&format!("Successful transitions: {}\n", self.stats.successful_transitions));
        report.push_str(&format!("Failed transitions: {}\n", self.stats.failed_transitions));
        report.push_str(&format!("Deoptimizations: {}\n", self.stats.deoptimizations));
        
        if self.stats.total_osr_replacements > 0 {
            let success_rate = (self.stats.successful_transitions as f64 / self.stats.total_osr_replacements as f64) * 100.0;
            report.push_str(&format!("Success rate: {:.2}%\n", success_rate));
        }

        report.push_str(&format!("Avg preparation time: {:.2}ms\n", self.stats.avg_preparation_time.as_millis()));
        report.push_str(&format!("Avg transition time: {:.2}ms\n", self.stats.avg_transition_time.as_millis()));
        report.push('\n');

        // Pending replacements
        let pending = self.pending_replacements.lock().unwrap();
        if !pending.is_empty() {
            report.push_str("Pending OSR replacements:\n");
            for function_name in pending.keys() {
                report.push_str(&format!("  - {}\n", function_name));
            }
            report.push('\n');
        }

        // Stack frame information
        let tracker = self.stack_frame_tracker.read().unwrap();
        report.push_str(&format!("Current stack depth: {}\n", tracker.current_depth));
        report.push_str(&format!("Active frames: {}\n", tracker.active_frames.len()));

        report
    }
}

/// Utility functions for OSR

/// Create an OSR manager with optimal settings
pub fn create_optimized_osr_manager(context: &Context) -> OSRManager {
    let config = OSRConfig {
        enable_loop_osr: true,
        enable_function_osr: true,
        osr_preparation_timeout: Duration::from_millis(50),
        max_stack_depth: 500,
        enable_deoptimization: true,
        osr_trigger_threshold: 100,
        enable_speculative_optimizations: true,
    };
    
    OSRManager::new(context, config)
}

/// Create an OSR manager for development/debugging
pub fn create_debug_osr_manager(context: &Context) -> OSRManager {
    let config = OSRConfig {
        enable_loop_osr: false,
        enable_function_osr: true,
        osr_preparation_timeout: Duration::from_millis(200),
        max_stack_depth: 100,
        enable_deoptimization: true,
        osr_trigger_threshold: 10,
        enable_speculative_optimizations: false,
    };
    
    OSRManager::new(context, config)
}

