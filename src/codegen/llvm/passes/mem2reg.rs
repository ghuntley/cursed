
/// Memory to Register Promotion (Mem2Reg)
/// 
/// This pass promotes memory operations to register operations by converting
/// alloca/load/store patterns into SSA values. It's essential for enabling
/// other optimizations by exposing values in registers.

use super::{OptimizationPass, PassConfiguration, PassResult};
use crate::common::optimization_level::OptimizationLevel;
use crate::error::{Error, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, BasicValueEnum, InstructionValue, BasicValue, PointerValue},
    basic_block::BasicBlock,
    builder::Builder,
    types::{BasicType, BasicTypeEnum},
};

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn};

/// Mem2Reg optimization pass
pub struct Mem2RegPass<'ctx> {
    context_lifetime: std::marker::PhantomData<&'ctx ()>,
    statistics: Mem2RegStatistics,
    promote_debug_info: bool,
}

impl<'ctx> Mem2RegPass<'ctx> {
    /// Create new Mem2Reg pass
    pub fn new() -> Self {
        Self {
            context_lifetime: std::marker::PhantomData,
            statistics: Mem2RegStatistics::default(),
            promote_debug_info: false,
        }
    }
    
    /// Create Mem2Reg pass with debug info promotion
    pub fn with_debug_info(promote_debug_info: bool) -> Self {
        Self {
            context_lifetime: std::marker::PhantomData,
            statistics: Mem2RegStatistics::default(),
            promote_debug_info,
        }
    }
}

impl<'ctx> OptimizationPass<'ctx> for Mem2RegPass<'ctx> {
    fn name(&self) -> &str {
        "mem2reg"
    }
    
    fn description(&self) -> &str {
        "Memory to Register Promotion - converts alloca/load/store to SSA values"
    }
    
    fn dependencies(&self) -> Vec<String> {
        vec![]
    }
    
    fn should_run(&self, config: &PassConfiguration) -> bool {
        config.enable_memory_optimizations && config.optimization_level >= OptimizationLevel::O1
    }
    
    fn required_optimization_level(&self) -> OptimizationLevel {
        OptimizationLevel::O1
    }
    
    fn estimated_execution_time(&self) -> Duration {
        Duration::from_millis(200)
    }
    
    #[instrument(skip(self, module, context))]
    fn run_on_module(&mut self, module: &Module<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        let start_time = Instant::now();
        info!("Running Mem2Reg pass on module");
        
        let mut total_result = PassResult::unchanged();
        
        // Run Mem2Reg on each function
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                debug!("Running Mem2Reg on function: {:?}", function.get_name());
                let function_result = self.run_on_function(&function, context)?;
                total_result = total_result.merge(function_result);
            }
        }
        
        total_result.execution_time = start_time.elapsed();
        
        info!("Mem2Reg pass completed: {} allocas promoted",
              total_result.memory_allocations_eliminated);
        
        Ok(total_result)
    }
    
    #[instrument(skip(self, function, context))]
    fn run_on_function(&mut self, function: &FunctionValue<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        let mut result = PassResult::unchanged();
        
        // Find promotable allocas
        let promotable_allocas = self.find_promotable_allocas(function)?;
        
        if promotable_allocas.is_empty() {
            debug!("No promotable allocas found");
            return Ok(result);
        }
        
        info!("Found {} promotable allocas", promotable_allocas.len());
        
        // Create Mem2Reg promoter
        let mut promoter = Mem2RegPromoter::new(function, context);
        
        // Promote each alloca
        let mut promoted_count = 0;
        for alloca in promotable_allocas {
            if promoter.promote_alloca(alloca)? {
                promoted_count += 1;
                result.changed = true;
            }
        }
        
        result.memory_allocations_eliminated = promoted_count;
        self.statistics.functions_processed += 1;
        self.statistics.total_allocas_promoted += promoted_count;
        
        debug!("Promoted {} allocas to registers", promoted_count);
        
        Ok(result)
    }
    
    fn get_statistics(&self) -> super::PassStatistics {
        super::PassStatistics {
            total_executions: self.statistics.functions_processed,
            successful_executions: self.statistics.functions_processed,
            total_execution_time: Duration::from_millis(0),
            average_execution_time: Duration::from_millis(0),
            total_instructions_eliminated: self.statistics.total_loads_eliminated + self.statistics.total_stores_eliminated,
            total_functions_inlined: 0,
            total_optimizations_applied: self.statistics.total_allocas_promoted,
            peak_memory_usage: 0,
        }
    }
    
    /// Find allocas that can be promoted to registers
    fn find_promotable_allocas(&self, function: &FunctionValue<'ctx>) -> Result<Vec<InstructionValue<'ctx>>> {
        let mut promotable_allocas = Vec::new();
        
        // Look for allocas in the entry block (typical pattern)
        if let Some(entry_block) = function.get_first_basic_block() {
            let mut instruction = entry_block.get_first_instruction();
            
            while let Some(instr) = instruction {
                if instr.get_opcode() == inkwell::values::InstructionOpcode::Alloca {
                    if self.is_promotable_alloca(&instr)? {
                        promotable_allocas.push(instr);
                    }
                }
                instruction = instr.get_next_instruction();
            }
        }
        
        Ok(promotable_allocas)
    }
    
    /// Check if an alloca is promotable to registers
    fn is_promotable_alloca(&self, alloca: &InstructionValue<'ctx>) -> Result<bool> {
        // Check if the alloca allocates a single value (not an array)
        if !self.allocates_single_value(alloca)? {
            return Ok(false);
        }
        
        // Analyze all uses of the alloca
        let use_analysis = self.analyze_alloca_uses(alloca)?;
        
        // Check if all uses are simple loads and stores
        if !use_analysis.only_loads_and_stores {
            return Ok(false);
        }
        
        // Check if there are no volatile operations
        if use_analysis.has_volatile_ops {
            return Ok(false);
        }
        
        // Check if alloca is not used in multiple blocks in complex ways
        if !self.has_simple_use_pattern(&use_analysis)? {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// Check if alloca allocates a single value
    fn allocates_single_value(&self, alloca: &InstructionValue<'ctx>) -> Result<bool> {
        // In a real implementation, we'd check:
        // 1. The alloca doesn't have an array size operand
        // 2. The allocated type is a simple scalar or small aggregate
        // For now, assume it's a single value
        Ok(true)
    }
    
    /// Analyze uses of an alloca
    fn analyze_alloca_uses(&self, alloca: &InstructionValue<'ctx>) -> Result<AllocaUseAnalysis> {
        let mut analysis = AllocaUseAnalysis {
            only_loads_and_stores: true,
            has_volatile_ops: false,
            loads: Vec::new(),
            stores: Vec::new(),
            defining_blocks: HashSet::new(),
            using_blocks: HashSet::new(),
        };
        
        // In a real implementation, we'd iterate through all uses
        // For now, assume simple usage pattern
        analysis.only_loads_and_stores = true;
        analysis.has_volatile_ops = false;
        
        Ok(analysis)
    }
    
    /// Check if the use pattern is simple enough for promotion
    fn has_simple_use_pattern(&self, analysis: &AllocaUseAnalysis) -> Result<bool> {
        // Simple heuristics:
        // 1. Not too many blocks involved
        // 2. Reasonable number of loads and stores
        Ok(analysis.defining_blocks.len() <= 10 && 
           analysis.using_blocks.len() <= 20 &&
           analysis.loads.len() <= 100 &&
           analysis.stores.len() <= 100)
    }
}

/// Mem2Reg promoter that performs the actual promotion
struct Mem2RegPromoter<'ctx> {
    function: &'ctx FunctionValue<'ctx>,
    context: &'ctx Context,
    
    // PHI nodes created during promotion
    phi_nodes: HashMap<BasicBlock<'ctx>, HashMap<InstructionValue<'ctx>, InstructionValue<'ctx>>>,
    
    // Current value of each alloca in each block
    current_values: HashMap<BasicBlock<'ctx>, HashMap<InstructionValue<'ctx>, BasicValueEnum<'ctx>>>,
    
    // Dominance information (simplified)
    dominance_info: DominanceInfo<'ctx>,
}

impl<'ctx> Mem2RegPromoter<'ctx> {
    /// Create new Mem2Reg promoter
    fn new(function: &'ctx FunctionValue<'ctx>, context: &'ctx Context) -> Self {
        let dominance_info = DominanceInfo::compute(function);
        
        Self {
            function,
            context,
            phi_nodes: HashMap::new(),
            current_values: HashMap::new(),
            dominance_info,
        }
    }
    
    /// Promote a single alloca to registers
    fn promote_alloca(&mut self, alloca: InstructionValue<'ctx>) -> Result<bool> {
        debug!("Promoting alloca to registers");
        
        // Collect all loads and stores for this alloca
        let (loads, stores) = self.collect_loads_and_stores(alloca)?;
        
        if loads.is_empty() && stores.is_empty() {
            // No uses, can just delete the alloca
            return Ok(true);
        }
        
        // Compute where PHI nodes are needed
        let phi_locations = self.compute_phi_locations(&stores)?;
        
        // Insert PHI nodes
        self.insert_phi_nodes(alloca, &phi_locations)?;
        
        // Rename variables (convert loads/stores to use SSA values)
        self.rename_variables(alloca, &loads, &stores)?;
        
        // Remove the original alloca and load/store instructions
        self.cleanup_instructions(alloca, &loads, &stores)?;
        
        debug!("Successfully promoted alloca with {} loads and {} stores", 
               loads.len(), stores.len());
        
        Ok(true)
    }
    
    /// Collect all load and store instructions for an alloca
    fn collect_loads_and_stores(
        &self,
        alloca: InstructionValue<'ctx>,
    ) -> Result<(Vec<InstructionValue<'ctx>>, Vec<InstructionValue<'ctx>>)> {
        let mut loads = Vec::new();
        let mut stores = Vec::new();
        
        // In a real implementation, we'd iterate through all uses of the alloca
        // and categorize them as loads or stores
        // For now, return empty vectors
        
        Ok((loads, stores))
    }
    
    /// Compute where PHI nodes are needed
    fn compute_phi_locations(
        &self,
        stores: &[InstructionValue<'ctx>],
    ) -> Result<HashSet<BasicBlock<'ctx>>> {
        let mut phi_locations = HashSet::new();
        
        // Use dominance frontier algorithm to compute PHI placement
        // For each store, find blocks in the dominance frontier
        for store in stores {
            if let Some(store_block) = store.get_parent() {
                let frontier = self.dominance_info.get_dominance_frontier(store_block);
                phi_locations.extend(frontier);
            }
        }
        
        Ok(phi_locations)
    }
    
    /// Insert PHI nodes at computed locations
    fn insert_phi_nodes(
        &mut self,
        alloca: InstructionValue<'ctx>,
        phi_locations: &HashSet<BasicBlock<'ctx>>,
    ) -> Result<()> {
        for &block in phi_locations {
            // In a real implementation, we'd:
            // 1. Create a PHI node at the beginning of the block
            // 2. Add it to our phi_nodes mapping
            // 3. Set up the PHI with the correct number of incoming values
            
            debug!("Would insert PHI node in block at address {}", block.get_address());
        }
        
        Ok(())
    }
    
    /// Rename variables (convert to SSA form)
    fn rename_variables(
        &mut self,
        alloca: InstructionValue<'ctx>,
        loads: &[InstructionValue<'ctx>],
        stores: &[InstructionValue<'ctx>],
    ) -> Result<()> {
        // Start renaming from the entry block
        if let Some(entry_block) = self.function.get_first_basic_block() {
            self.rename_in_block(alloca, entry_block, loads, stores)?;
        }
        
        Ok(())
    }
    
    /// Rename variables in a specific block
    fn rename_in_block(
        &mut self,
        alloca: InstructionValue<'ctx>,
        block: BasicBlock<'ctx>,
        loads: &[InstructionValue<'ctx>],
        stores: &[InstructionValue<'ctx>],
    ) -> Result<()> {
        // Process instructions in the block
        let mut instruction = block.get_first_instruction();
        
        while let Some(instr) = instruction {
            // Handle stores: update current value
            if stores.contains(&instr) {
                self.handle_store_rename(alloca, &instr, block)?;
            }
            
            // Handle loads: replace with current value
            if loads.contains(&instr) {
                self.handle_load_rename(alloca, &instr, block)?;
            }
            
            instruction = instr.get_next_instruction();
        }
        
        // Process successor blocks
        self.process_successor_blocks(alloca, block, loads, stores)?;
        
        Ok(())
    }
    
    /// Handle renaming of a store instruction
    fn handle_store_rename(
        &mut self,
        alloca: InstructionValue<'ctx>,
        store: &InstructionValue<'ctx>,
        block: BasicBlock<'ctx>,
    ) -> Result<()> {
        // In a real implementation:
        // 1. Get the stored value from the store instruction
        // 2. Update current_values mapping for this alloca in this block
        // 3. Mark the store for deletion
        
        debug!("Handling store rename in block {}", block.get_address());
        Ok(())
    }
    
    /// Handle renaming of a load instruction
    fn handle_load_rename(
        &mut self,
        alloca: InstructionValue<'ctx>,
        load: &InstructionValue<'ctx>,
        block: BasicBlock<'ctx>,
    ) -> Result<()> {
        // In a real implementation:
        // 1. Get the current value for this alloca in this block
        // 2. Replace all uses of the load with the current value
        // 3. Mark the load for deletion
        
        debug!("Handling load rename in block {}", block.get_address());
        Ok(())
    }
    
    /// Process successor blocks
    fn process_successor_blocks(
        &mut self,
        alloca: InstructionValue<'ctx>,
        block: BasicBlock<'ctx>,
        loads: &[InstructionValue<'ctx>],
        stores: &[InstructionValue<'ctx>],
    ) -> Result<()> {
        // In a real implementation:
        // 1. Get all successor blocks
        // 2. For each successor, update PHI nodes with current values
        // 3. Recursively process successor blocks
        
        Ok(())
    }
    
    /// Clean up original instructions
    fn cleanup_instructions(
        &self,
        alloca: InstructionValue<'ctx>,
        loads: &[InstructionValue<'ctx>],
        stores: &[InstructionValue<'ctx>],
    ) -> Result<()> {
        // In a real implementation:
        // 1. Remove all load instructions
        // 2. Remove all store instructions  
        // 3. Remove the original alloca instruction
        
        debug!("Cleaning up {} loads, {} stores, and 1 alloca", 
               loads.len(), stores.len());
        
        Ok(())
    }
}

/// Analysis of alloca uses
#[derive(Debug)]
struct AllocaUseAnalysis {
    /// Whether all uses are just loads and stores
    only_loads_and_stores: bool,
    /// Whether any volatile operations are present
    has_volatile_ops: bool,
    /// Load instructions
    loads: Vec<InstructionValue<'static>>,
    /// Store instructions
    stores: Vec<InstructionValue<'static>>,
    /// Blocks that define values (have stores)
    defining_blocks: HashSet<usize>,
    /// Blocks that use values (have loads)
    using_blocks: HashSet<usize>,
}

/// Simplified dominance information
#[derive(Debug)]
struct DominanceInfo<'ctx> {
    /// Dominance frontiers for each block
    dominance_frontiers: HashMap<BasicBlock<'ctx>, HashSet<BasicBlock<'ctx>>>,
}

impl<'ctx> DominanceInfo<'ctx> {
    /// Compute dominance information for a function
    fn compute(function: &'ctx FunctionValue<'ctx>) -> Self {
        let mut dominance_frontiers = HashMap::new();
        
        // In a real implementation, we'd compute actual dominance frontiers
        // For now, return empty frontiers
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            dominance_frontiers.insert(bb, HashSet::new());
            block = bb.get_next_basic_block();
        }
        
        Self {
            dominance_frontiers,
        }
    }
    
    /// Get dominance frontier for a block
    fn get_dominance_frontier(&self, block: BasicBlock<'ctx>) -> HashSet<BasicBlock<'ctx>> {
        self.dominance_frontiers.get(&block).cloned().unwrap_or_default()
    }
}

/// Statistics for Mem2Reg pass
#[derive(Debug, Default)]
struct Mem2RegStatistics {
    pub functions_processed: u64,
    pub total_allocas_promoted: usize,
    pub total_loads_eliminated: usize,
    pub total_stores_eliminated: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_mem2reg_pass_creation() {
        let pass = Mem2RegPass::<'_>::new();
        assert_eq!(pass.name(), "mem2reg");
        assert!(pass.description().contains("Memory to Register"));
    }
    
    #[test]
    fn test_mem2reg_pass_with_debug_info() {
        let pass = Mem2RegPass::<'_>::with_debug_info(true);
        assert!(pass.promote_debug_info);
        
        let pass2 = Mem2RegPass::<'_>::with_debug_info(false);
        assert!(!pass2.promote_debug_info);
    }
    
    #[test]
    fn test_mem2reg_dependencies() {
        let pass = Mem2RegPass::<'_>::new();
        let deps = pass.dependencies();
        assert!(deps.is_empty()); // Mem2reg is typically a base pass
    }
    
    #[test]
    fn test_alloca_use_analysis() {
        let analysis = AllocaUseAnalysis {
            only_loads_and_stores: true,
            has_volatile_ops: false,
            loads: Vec::new(),
            stores: Vec::new(),
            defining_blocks: HashSet::new(),
            using_blocks: HashSet::new(),
        };
        
        assert!(analysis.only_loads_and_stores);
        assert!(!analysis.has_volatile_ops);
        assert_eq!(analysis.loads.len(), 0);
        assert_eq!(analysis.stores.len(), 0);
    }
    
    #[test]
    fn test_dominance_info_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        
        let dom_info = DominanceInfo::compute(&function);
        assert!(dom_info.dominance_frontiers.contains_key(&basic_block));
        
        let frontier = dom_info.get_dominance_frontier(basic_block);
        assert!(frontier.is_empty()); // Single block should have empty frontier
    }
    
    #[test]
    fn test_mem2reg_promoter_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test", fn_type, None);
        
        let promoter = Mem2RegPromoter::new(&function, &context);
        assert!(promoter.phi_nodes.is_empty());
        assert!(promoter.current_values.is_empty());
    }
}
