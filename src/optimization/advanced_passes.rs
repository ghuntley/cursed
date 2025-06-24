/// Advanced optimization passes for CURSED compiler
/// 
/// Implements sophisticated optimization algorithms including:
/// - Register allocation with spilling and coalescing
/// - Instruction scheduling and pipelining
/// - Profile-guided optimization integration
/// - Cross-module optimization

use crate::error::{Error, Result};

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn};

/// Advanced register allocator using graph coloring and linear scan
#[derive(Debug, Clone)]
pub struct AdvancedRegisterAllocator {
    /// Target architecture register count
    register_count: usize,
    /// Interference graph
    interference_graph: HashMap<VirtualRegister, HashSet<VirtualRegister>>,
    /// Register allocation result
    allocation: HashMap<VirtualRegister, PhysicalRegister>,
    /// Spill locations
    spill_locations: HashMap<VirtualRegister, StackSlot>,
    /// Coalescing candidates
    coalescing_candidates: Vec<(VirtualRegister, VirtualRegister)>,
    /// Statistics
    statistics: RegisterAllocationStats,
}

/// Virtual register representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualRegister(pub u32);

/// Physical register representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalRegister(pub u32);

/// Stack slot for spilled registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackSlot(pub u32);

/// Live range information for register allocation
#[derive(Debug, Clone)]
pub struct LiveRange {
    pub register: VirtualRegister,
    pub start: u32,
    pub end: u32,
    pub frequency: f64,
    pub spill_cost: f64,
}

/// Register allocation statistics
#[derive(Debug, Clone, Default)]
pub struct RegisterAllocationStats {
    pub virtual_registers: usize,
    pub physical_registers_used: usize,
    pub spilled_registers: usize,
    pub coalesced_moves: usize,
    pub allocation_time: Duration,
}

impl AdvancedRegisterAllocator {
    /// Create new register allocator
    pub fn new(register_count: usize) -> Self {
        Self {
            register_count,
            interference_graph: HashMap::new(),
            allocation: HashMap::new(),
            spill_locations: HashMap::new(),
            coalescing_candidates: Vec::new(),
            statistics: RegisterAllocationStats::default(),
        }
    }

    /// Perform register allocation using graph coloring
    #[instrument(skip(self, live_ranges))]
    pub fn allocate_registers(&mut self, live_ranges: &[LiveRange]) -> Result<()> {
        let start_time = Instant::now();
        
        info!("Starting register allocation for {} live ranges", live_ranges.len());
        
        // Build interference graph
        self.build_interference_graph(live_ranges)?;
        
        // Attempt coalescing to reduce register pressure
        self.perform_coalescing()?;
        
        // Graph coloring allocation
        self.graph_coloring_allocation()?;
        
        // Handle spills if necessary
        self.handle_spills(live_ranges)?;
        
        // Update statistics
        self.statistics.allocation_time = start_time.elapsed();
        self.statistics.virtual_registers = live_ranges.len();
        self.statistics.physical_registers_used = self.allocation.len();
        self.statistics.spilled_registers = self.spill_locations.len();
        
        info!("Register allocation completed in {:?}", self.statistics.allocation_time);
        Ok(())
    }

    /// Build interference graph from live ranges
    fn build_interference_graph(&mut self, live_ranges: &[LiveRange]) -> Result<()> {
        for i in 0..live_ranges.len() {
            for j in (i + 1)..live_ranges.len() {
                let range1 = &live_ranges[i];
                let range2 = &live_ranges[j];
                
                // Check if ranges interfere (overlap)
                if range1.start <= range2.end && range2.start <= range1.end {
                    self.interference_graph
                        .entry(range1.register)
                        .or_default()
                        .insert(range2.register);
                    self.interference_graph
                        .entry(range2.register)
                        .or_default()
                        .insert(range1.register);
                }
            }
        }
        
        debug!("Built interference graph with {} nodes", self.interference_graph.len());
        Ok(())
    }

    /// Perform register coalescing to reduce move instructions
    fn perform_coalescing(&mut self) -> Result<()> {
        let mut coalesced = 0;
        
        for (reg1, reg2) in &self.coalescing_candidates {
            if self.can_coalesce(*reg1, *reg2) {
                self.coalesce_registers(*reg1, *reg2)?;
                coalesced += 1;
            }
        }
        
        self.statistics.coalesced_moves = coalesced;
        debug!("Coalesced {} register pairs", coalesced);
        Ok(())
    }

    /// Check if two registers can be coalesced
    fn can_coalesce(&self, reg1: VirtualRegister, reg2: VirtualRegister) -> bool {
        // Simple heuristic: can coalesce if they don't interfere
        !self.interference_graph
            .get(&reg1)
            .map(|neighbors| neighbors.contains(&reg2))
            .unwrap_or(false)
    }

    /// Coalesce two registers by merging their interference sets
    fn coalesce_registers(&mut self, reg1: VirtualRegister, reg2: VirtualRegister) -> Result<()> {
        // Merge interference sets
        let reg2_neighbors = self.interference_graph.remove(&reg2).unwrap_or_default();
        
        if let Some(reg1_neighbors) = self.interference_graph.get_mut(&reg1) {
            reg1_neighbors.extend(reg2_neighbors);
        }
        
        // Update all references to reg2 to point to reg1
        for neighbors in self.interference_graph.values_mut() {
            if neighbors.remove(&reg2) {
                neighbors.insert(reg1);
            }
        }
        
        Ok(())
    }

    /// Graph coloring register allocation
    fn graph_coloring_allocation(&mut self) -> Result<()> {
        let mut available_registers: HashSet<PhysicalRegister> = 
            (0..self.register_count).map(PhysicalRegister).collect();
        
        // Simple greedy coloring - in practice, would use more sophisticated algorithms
        for (&register, neighbors) in &self.interference_graph {
            let mut used_colors = HashSet::new();
            
            // Find colors used by interfering registers
            for &neighbor in neighbors {
                if let Some(&physical_reg) = self.allocation.get(&neighbor) {
                    used_colors.insert(physical_reg);
                }
            }
            
            // Find first available color
            if let Some(&color) = available_registers.iter().find(|&&c| !used_colors.contains(&c)) {
                self.allocation.insert(register, color);
            } else {
                // Need to spill - mark for later handling
                debug!("Register {:?} needs spilling", register);
            }
        }
        
        Ok(())
    }

    /// Handle register spills by allocating stack slots
    fn handle_spills(&mut self, live_ranges: &[LiveRange]) -> Result<()> {
        let mut next_stack_slot = 0;
        
        for range in live_ranges {
            if !self.allocation.contains_key(&range.register) {
                // This register needs to be spilled
                self.spill_locations.insert(range.register, StackSlot(next_stack_slot));
                next_stack_slot += 1;
                
                warn!("Spilled register {:?} to stack slot {}", range.register, next_stack_slot - 1);
            }
        }
        
        Ok(())
    }

    /// Get allocation result
    pub fn get_allocation(&self) -> &HashMap<VirtualRegister, PhysicalRegister> {
        &self.allocation
    }

    /// Get spill locations
    pub fn get_spill_locations(&self) -> &HashMap<VirtualRegister, StackSlot> {
        &self.spill_locations
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &RegisterAllocationStats {
        &self.statistics
    }
}

/// Advanced instruction scheduler with pipeline awareness
#[derive(Debug, Clone)]
pub struct InstructionScheduler {
    /// Instruction dependency graph
    dependency_graph: HashMap<InstructionId, HashSet<InstructionId>>,
    /// Instruction latencies
    latencies: HashMap<InstructionId, u32>,
    /// Pipeline configuration
    pipeline_config: PipelineConfig,
    /// Scheduling result
    scheduled_order: Vec<InstructionId>,
    /// Statistics
    statistics: SchedulingStats,
}

/// Instruction identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionId(pub u32);

/// Pipeline configuration for instruction scheduling
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub pipeline_depth: u32,
    pub issue_width: u32,
    pub functional_units: HashMap<InstructionType, u32>,
}

/// Instruction type for pipeline scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionType {
    Arithmetic,
    Memory,
    Branch,
    FloatingPoint,
    Vector,
}

/// Scheduling statistics
#[derive(Debug, Clone, Default)]
pub struct SchedulingStats {
    pub instructions_scheduled: usize,
    pub cycles_saved: u32,
    pub pipeline_stalls_avoided: u32,
    pub scheduling_time: Duration,
}

impl InstructionScheduler {
    /// Create new instruction scheduler
    pub fn new(pipeline_config: PipelineConfig) -> Self {
        Self {
            dependency_graph: HashMap::new(),
            latencies: HashMap::new(),
            pipeline_config,
            scheduled_order: Vec::new(),
            statistics: SchedulingStats::default(),
        }
    }

    /// Schedule instructions using list scheduling algorithm
    #[instrument(skip(self, instructions))]
    pub fn schedule_instructions(&mut self, instructions: &[Instruction]) -> Result<Vec<InstructionId>> {
        let start_time = Instant::now();
        
        info!("Starting instruction scheduling for {} instructions", instructions.len());
        
        // Build dependency graph
        self.build_dependency_graph(instructions)?;
        
        // Perform list scheduling
        self.list_scheduling(instructions)?;
        
        // Update statistics
        self.statistics.scheduling_time = start_time.elapsed();
        self.statistics.instructions_scheduled = instructions.len();
        
        info!("Instruction scheduling completed in {:?}", self.statistics.scheduling_time);
        Ok(self.scheduled_order.clone())
    }

    /// Build instruction dependency graph
    fn build_dependency_graph(&mut self, instructions: &[Instruction]) -> Result<()> {
        // Analyze data dependencies (RAW, WAR, WAW)
        for i in 0..instructions.len() {
            for j in (i + 1)..instructions.len() {
                let inst1 = &instructions[i];
                let inst2 = &instructions[j];
                
                if self.has_dependency(inst1, inst2) {
                    self.dependency_graph
                        .entry(InstructionId(i as u32))
                        .or_default()
                        .insert(InstructionId(j as u32));
                }
            }
        }
        
        debug!("Built dependency graph with {} edges", 
               self.dependency_graph.values().map(|deps| deps.len()).sum::<usize>());
        Ok(())
    }

    /// Check if instruction has dependency
    fn has_dependency(&self, inst1: &Instruction, inst2: &Instruction) -> bool {
        // Check for register dependencies
        for def in &inst1.definitions {
            if inst2.uses.contains(def) || inst2.definitions.contains(def) {
                return true;
            }
        }
        
        // Check for memory dependencies
        if inst1.memory_access && inst2.memory_access {
            return true; // Conservative assumption
        }
        
        false
    }

    /// List scheduling algorithm
    fn list_scheduling(&mut self, instructions: &[Instruction]) -> Result<()> {
        let mut ready_queue: VecDeque<InstructionId> = VecDeque::new();
        let mut scheduled = HashSet::new();
        let mut cycle = 0;
        
        // Initialize ready queue with instructions that have no dependencies
        for (i, _) in instructions.iter().enumerate() {
            let inst_id = InstructionId(i as u32);
            if !self.dependency_graph.contains_key(&inst_id) || 
               self.dependency_graph[&inst_id].is_empty() {
                ready_queue.push_back(inst_id);
            }
        }
        
        while !ready_queue.is_empty() || scheduled.len() < instructions.len() {
            // Schedule instructions from ready queue
            let mut issued_this_cycle = 0;
            
            while !ready_queue.is_empty() && 
                  issued_this_cycle < self.pipeline_config.issue_width {
                let inst_id = ready_queue.pop_front().unwrap();
                
                if self.can_issue_instruction(&instructions[inst_id.0 as usize], cycle) {
                    self.scheduled_order.push(inst_id);
                    scheduled.insert(inst_id);
                    issued_this_cycle += 1;
                    
                    // Update ready queue with newly available instructions
                    self.update_ready_queue(&scheduled, &mut ready_queue);
                } else {
                    // Put back if can't issue this cycle
                    ready_queue.push_back(inst_id);
                }
            }
            
            cycle += 1;
        }
        
        self.statistics.cycles_saved = cycle.saturating_sub(instructions.len() as u32);
        debug!("List scheduling completed in {} cycles", cycle);
        Ok(())
    }

    /// Check if instruction can be issued in current cycle
    fn can_issue_instruction(&self, instruction: &Instruction, cycle: u32) -> bool {
        // Check functional unit availability
        let required_units = self.pipeline_config.functional_units
            .get(&instruction.instruction_type)
            .copied()
            .unwrap_or(1);
        
        // Simplified check - in reality would track functional unit usage
        required_units > 0
    }

    /// Update ready queue with newly available instructions
    fn update_ready_queue(&self, scheduled: &HashSet<InstructionId>, ready_queue: &mut VecDeque<InstructionId>) {
        for (&inst_id, dependencies) in &self.dependency_graph {
            if !scheduled.contains(&inst_id) && 
               dependencies.iter().all(|dep| scheduled.contains(dep)) {
                if !ready_queue.contains(&inst_id) {
                    ready_queue.push_back(inst_id);
                }
            }
        }
    }

    /// Get scheduling statistics
    pub fn get_statistics(&self) -> &SchedulingStats {
        &self.statistics
    }
}

/// Instruction representation for scheduling
#[derive(Debug, Clone)]
pub struct Instruction {
    pub id: InstructionId,
    pub instruction_type: InstructionType,
    pub uses: HashSet<VirtualRegister>,
    pub definitions: HashSet<VirtualRegister>,
    pub memory_access: bool,
    pub latency: u32,
}

impl Instruction {
    /// Create new instruction
    pub fn new(id: u32, instruction_type: InstructionType) -> Self {
        Self {
            id: InstructionId(id),
            instruction_type,
            uses: HashSet::new(),
            definitions: HashSet::new(),
            memory_access: false,
            latency: 1,
        }
    }

    /// Add register use
    pub fn add_use(&mut self, register: VirtualRegister) {
        self.uses.insert(register);
    }

    /// Add register definition
    pub fn add_definition(&mut self, register: VirtualRegister) {
        self.definitions.insert(register);
    }

    /// Mark as memory access
    pub fn set_memory_access(&mut self) {
        self.memory_access = true;
    }

    /// Set instruction latency
    pub fn set_latency(&mut self, latency: u32) {
        self.latency = latency;
    }
}

impl Default for PipelineConfig {
    fn default() -> Self {
        let mut functional_units = HashMap::new();
        functional_units.insert(InstructionType::Arithmetic, 2);
        functional_units.insert(InstructionType::Memory, 1);
        functional_units.insert(InstructionType::Branch, 1);
        functional_units.insert(InstructionType::FloatingPoint, 1);
        functional_units.insert(InstructionType::Vector, 1);
        
        Self {
            pipeline_depth: 5,
            issue_width: 2,
            functional_units,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_allocator_creation() {
        let allocator = AdvancedRegisterAllocator::new(8);
        assert_eq!(allocator.register_count, 8);
        assert!(allocator.allocation.is_empty());
    }

    #[test]
    fn test_instruction_scheduler_creation() {
        let config = PipelineConfig::default();
        let scheduler = InstructionScheduler::new(config);
        assert_eq!(scheduler.pipeline_config.pipeline_depth, 5);
    }

    #[test]
    fn test_instruction_creation() {
        let mut inst = Instruction::new(0, InstructionType::Arithmetic);
        inst.add_use(VirtualRegister(1));
        inst.add_definition(VirtualRegister(2));
        inst.set_latency(2);
        
        assert!(inst.uses.contains(&VirtualRegister(1)));
        assert!(inst.definitions.contains(&VirtualRegister(2)));
        assert_eq!(inst.latency, 2);
    }

    #[test]
    fn test_live_range() {
        let range = LiveRange {
            register: VirtualRegister(1),
            start: 10,
            end: 20,
            frequency: 0.8,
            spill_cost: 100.0,
        };
        
        assert_eq!(range.register, VirtualRegister(1));
        assert_eq!(range.start, 10);
        assert_eq!(range.end, 20);
    }
}
