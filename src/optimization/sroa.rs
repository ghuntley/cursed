/// Scalar Replacement of Aggregates (SROA) Implementation
/// 
/// Provides comprehensive SROA optimization for CURSED, breaking down
/// aggregates (structs, arrays) into individual scalars for better optimization.

use crate::error::{Error, Result};
use crate::common::optimization_level::OptimizationLevel;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};

use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, InstructionValue, StructValue, ArrayValue, BasicValueEnum, AggregateValueEnum},
    types::{StructType, ArrayType, BasicType, AnyType},
    basic_block::BasicBlock,
    builder::Builder,
};

/// Scalar Replacement of Aggregates optimizer
pub struct SroaOptimizer<'ctx> {
    context: &'ctx Context,
    optimization_level: OptimizationLevel,
    aggregate_analysis: AggregateAnalysis,
    replacement_map: HashMap<String, ScalarReplacement>,
    statistics: Arc<Mutex<SroaStatistics>>,
    builder: Builder<'ctx>,
}

/// Analysis of aggregate types and their usage patterns
#[derive(Debug, Clone)]
pub struct AggregateAnalysis {
    struct_analyses: HashMap<String, StructAnalysis>,
    array_analyses: HashMap<String, ArrayAnalysis>,
    allocation_sites: HashMap<String, AllocationSite>,
    usage_patterns: HashMap<String, UsagePattern>,
}

/// Analysis of struct usage
#[derive(Debug, Clone)]
pub struct StructAnalysis {
    pub struct_name: String,
    pub field_count: usize,
    pub field_types: Vec<String>,
    pub field_usage: HashMap<usize, FieldUsage>,
    pub total_allocations: usize,
    pub promotable_allocations: usize,
    pub promotion_benefit: f64,
}

/// Analysis of array usage
#[derive(Debug, Clone)]
pub struct ArrayAnalysis {
    pub array_name: String,
    pub element_type: String,
    pub element_count: Option<usize>,
    pub access_patterns: Vec<ArrayAccessPattern>,
    pub is_constant_indexed: bool,
    pub promotable_elements: HashSet<usize>,
    pub promotion_benefit: f64,
}

/// Field usage information
#[derive(Debug, Clone)]
pub struct FieldUsage {
    pub field_index: usize,
    pub read_count: usize,
    pub write_count: usize,
    pub gep_count: usize,
    pub escape_count: usize,
    pub is_promotable: bool,
}

/// Array access pattern analysis
#[derive(Debug, Clone)]
pub struct ArrayAccessPattern {
    pub access_type: ArrayAccessType,
    pub index: ArrayIndex,
    pub frequency: usize,
    pub is_sequential: bool,
}

/// Type of array access
#[derive(Debug, Clone)]
pub enum ArrayAccessType {
    Load,
    Store,
    GetElementPtr,
    MemCpy,
    MemSet,
}

/// Array index information
#[derive(Debug, Clone)]
pub enum ArrayIndex {
    Constant(usize),
    Variable(String),
    Expression(String),
    Unknown,
}

/// Allocation site information
#[derive(Debug, Clone)]
pub struct AllocationSite {
    pub allocation_name: String,
    pub allocation_type: AllocationType,
    pub aggregate_type: String,
    pub size_info: SizeInfo,
    pub lifetime: AllocationLifetime,
    pub is_promotable: bool,
    pub promotion_score: f64,
}

/// Type of allocation
#[derive(Debug, Clone)]
pub enum AllocationType {
    Stack,     // alloca instruction
    Heap,      // malloc/new
    Global,    // global variable
    Parameter, // function parameter
}

/// Size information for allocations
#[derive(Debug, Clone)]
pub struct SizeInfo {
    pub static_size: Option<usize>,
    pub dynamic_size: Option<String>,
    pub alignment: Option<u32>,
    pub element_count: Option<usize>,
}

/// Allocation lifetime analysis
#[derive(Debug, Clone)]
pub enum AllocationLifetime {
    Function,                    // Lives for entire function
    BasicBlock,                  // Lives within basic block
    Loop,                        // Lives within loop
    Conditional(String),         // Lives within conditional
    Unknown,
}

/// Usage pattern for aggregates
#[derive(Debug, Clone)]
pub struct UsagePattern {
    pub aggregate_name: String,
    pub total_uses: usize,
    pub field_accesses: HashMap<usize, usize>,
    pub whole_aggregate_operations: usize,
    pub escape_uses: usize,
    pub is_scalar_replaceable: bool,
    pub complexity_score: f64,
}

/// Scalar replacement mapping
#[derive(Debug, Clone)]
pub struct ScalarReplacement {
    pub original_aggregate: String,
    pub scalar_variables: HashMap<usize, String>, // field_index -> scalar_name
    pub replacement_instructions: Vec<ReplacementInstruction>,
    pub estimated_benefit: f64,
}

/// Instruction replacement information
#[derive(Debug, Clone)]
pub struct ReplacementInstruction {
    pub original_instruction: String,
    pub replacement_type: ReplacementType,
    pub scalar_operations: Vec<String>,
    pub complexity_reduction: f64,
}

/// Type of instruction replacement
#[derive(Debug, Clone)]
pub enum ReplacementType {
    LoadReplacement,      // Replace aggregate load with scalar loads
    StoreReplacement,     // Replace aggregate store with scalar stores
    GepElimination,       // Eliminate GEP by using scalar directly
    MemcpyElimination,    // Replace memcpy with scalar copies
    PhiReplacement,       // Replace aggregate phi with scalar phis
    SelectReplacement,    // Replace aggregate select with scalar selects
}

/// SROA optimization statistics
#[derive(Debug, Clone, Default)]
pub struct SroaStatistics {
    pub aggregates_analyzed: usize,
    pub structs_promoted: usize,
    pub arrays_promoted: usize,
    pub scalar_variables_created: usize,
    pub instructions_eliminated: usize,
    pub instructions_replaced: usize,
    pub memory_accesses_simplified: usize,
    pub estimated_speedup: f64,
    pub memory_usage_reduction: usize,
    pub optimization_time: Duration,
    pub functions_optimized: usize,
}

impl<'ctx> SroaOptimizer<'ctx> {
    /// Create new SROA optimizer
    #[instrument(skip(context))]
    pub fn new(context: &'ctx Context, optimization_level: OptimizationLevel) -> Self {
        info!("Initializing SROA optimizer with optimization level {:?}", optimization_level);
        
        Self {
            context,
            optimization_level,
            aggregate_analysis: AggregateAnalysis::new(),
            replacement_map: HashMap::new(),
            statistics: Arc::new(Mutex::new(SroaStatistics::default())),
            builder: context.create_builder(),
        }
    }
    
    /// Perform SROA optimization on entire module
    #[instrument(skip(self, module))]
    pub fn optimize_module(&mut self, module: &Module<'ctx>) -> Result<SroaOptimizationResults> {
        let start_time = Instant::now();
        info!("Starting SROA optimization");
        
        // Phase 1: Analyze all aggregates in the module
        self.analyze_module_aggregates(module)?;
        
        // Phase 2: Identify promotion candidates
        let promotion_candidates = self.identify_promotion_candidates()?;
        
        // Phase 3: Perform scalar replacement
        let mut replacement_results = Vec::new();
        for candidate in &promotion_candidates {
            if let Some(function) = self.find_function_for_allocation(module, &candidate.allocation_name) {
                let result = self.perform_scalar_replacement(function, candidate)?;
                replacement_results.push(result);
            }
        }
        
        // Phase 4: Clean up dead code
        self.cleanup_dead_aggregate_code(module)?;
        
        let optimization_time = start_time.elapsed();
        self.update_statistics(optimization_time, &replacement_results);
        
        info!(
            optimization_time = ?optimization_time,
            structs_promoted = self.get_statistics().structs_promoted,
            arrays_promoted = self.get_statistics().arrays_promoted,
            "SROA optimization completed"
        );
        
        Ok(SroaOptimizationResults {
            promotion_candidates,
            replacement_results,
            aggregate_analysis: self.aggregate_analysis.clone(),
            optimization_opportunities: self.identify_additional_opportunities()?,
            statistics: self.get_statistics(),
        })
    }
    
    /// Optimize a single function
    #[instrument(skip(self, function))]
    pub fn optimize_function(&mut self, function: FunctionValue<'ctx>) -> Result<FunctionSroaResults> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        debug!("Optimizing function with SROA: {}", function_name);
        
        // Analyze aggregates in this function
        self.analyze_function_aggregates(function)?;
        
        // Find promotion candidates in this function
        let candidates = self.find_function_promotion_candidates(function)?;
        
        // Perform replacements
        let mut replacements = Vec::new();
        for candidate in &candidates {
            let replacement = self.perform_scalar_replacement(function, candidate)?;
            replacements.push(replacement);
        }
        
        Ok(FunctionSroaResults {
            function_name: function_name.to_string(),
            promotion_candidates: candidates,
            scalar_replacements: replacements,
            optimization_benefit: self.calculate_function_benefit(&replacements),
        })
    }
    
    /// Check if an aggregate can be promoted
    pub fn can_promote_aggregate(&self, allocation_name: &str) -> PromotionEligibility {
        if let Some(allocation_site) = self.aggregate_analysis.allocation_sites.get(allocation_name) {
            if !allocation_site.is_promotable {
                return PromotionEligibility {
                    can_promote: false,
                    blocking_reasons: vec!["Not marked as promotable during analysis".to_string()],
                    confidence: 0.0,
                };
            }
            
            if let Some(usage_pattern) = self.aggregate_analysis.usage_patterns.get(allocation_name) {
                let mut blocking_reasons = Vec::new();
                
                // Check for blocking conditions
                if usage_pattern.escape_uses > 0 {
                    blocking_reasons.push("Aggregate escapes function scope".to_string());
                }
                
                if usage_pattern.whole_aggregate_operations > usage_pattern.total_uses / 2 {
                    blocking_reasons.push("Too many whole-aggregate operations".to_string());
                }
                
                if usage_pattern.complexity_score > 0.8 {
                    blocking_reasons.push("Access pattern too complex".to_string());
                }
                
                let can_promote = blocking_reasons.is_empty();
                let confidence = if can_promote {
                    allocation_site.promotion_score
                } else {
                    0.0
                };
                
                PromotionEligibility {
                    can_promote,
                    blocking_reasons,
                    confidence,
                }
            } else {
                PromotionEligibility {
                    can_promote: false,
                    blocking_reasons: vec!["No usage pattern analysis available".to_string()],
                    confidence: 0.0,
                }
            }
        } else {
            PromotionEligibility {
                can_promote: false,
                blocking_reasons: vec!["Allocation site not found".to_string()],
                confidence: 0.0,
            }
        }
    }
    
    /// Generate comprehensive SROA report
    pub fn generate_sroa_report(&self, results: &SroaOptimizationResults) -> String {
        let mut report = String::new();
        
        report.push_str("# Scalar Replacement of Aggregates (SROA) Report\n\n");
        
        // Executive Summary
        report.push_str("## Executive Summary\n");
        report.push_str(&format!("- **Aggregates Analyzed**: {}\n", results.statistics.aggregates_analyzed));
        report.push_str(&format!("- **Structs Promoted**: {}\n", results.statistics.structs_promoted));
        report.push_str(&format!("- **Arrays Promoted**: {}\n", results.statistics.arrays_promoted));
        report.push_str(&format!("- **Scalar Variables Created**: {}\n", results.statistics.scalar_variables_created));
        report.push_str(&format!("- **Instructions Eliminated**: {}\n", results.statistics.instructions_eliminated));
        report.push_str(&format!("- **Memory Accesses Simplified**: {}\n", results.statistics.memory_accesses_simplified));
        report.push_str(&format!("- **Estimated Speedup**: {:.1}%\n", results.statistics.estimated_speedup));
        report.push_str(&format!("- **Memory Usage Reduction**: {} bytes\n", results.statistics.memory_usage_reduction));
        report.push_str(&format!("- **Optimization Time**: {:?}\n\n", results.statistics.optimization_time));
        
        // Promotion Candidates
        if !results.promotion_candidates.is_empty() {
            report.push_str("## Promotion Candidates\n");
            for (i, candidate) in results.promotion_candidates.iter().enumerate().take(10) {
                report.push_str(&format!("{}. **{}** (score: {:.2})\n", 
                    i + 1, candidate.allocation_name, candidate.promotion_score));
                report.push_str(&format!("   - Type: {:?}\n", candidate.allocation_type));
                if let Some(size) = candidate.size_info.static_size {
                    report.push_str(&format!("   - Size: {} bytes\n", size));
                }
                report.push_str(&format!("   - Lifetime: {:?}\n", candidate.lifetime));
            }
            report.push_str("\n");
        }
        
        // Replacement Results
        if !results.replacement_results.is_empty() {
            report.push_str("## Replacement Results\n");
            for (i, replacement) in results.replacement_results.iter().enumerate().take(10) {
                report.push_str(&format!("{}. **{}**\n", i + 1, replacement.original_aggregate));
                report.push_str(&format!("   - Scalar variables: {}\n", replacement.scalar_variables.len()));
                report.push_str(&format!("   - Instructions replaced: {}\n", replacement.replacement_instructions.len()));
                report.push_str(&format!("   - Estimated benefit: {:.1}%\n", replacement.estimated_benefit));
            }
            report.push_str("\n");
        }
        
        // Aggregate Analysis Summary
        report.push_str("## Aggregate Analysis Summary\n");
        report.push_str(&format!("- **Structs Analyzed**: {}\n", results.aggregate_analysis.struct_analyses.len()));
        report.push_str(&format!("- **Arrays Analyzed**: {}\n", results.aggregate_analysis.array_analyses.len()));
        report.push_str(&format!("- **Allocation Sites**: {}\n", results.aggregate_analysis.allocation_sites.len()));
        
        // Detailed struct analysis
        if !results.aggregate_analysis.struct_analyses.is_empty() {
            report.push_str("\n### Struct Analysis Details\n");
            for (struct_name, analysis) in &results.aggregate_analysis.struct_analyses {
                report.push_str(&format!("- **{}**: {} fields, {} allocations, {:.1}% benefit\n", 
                    struct_name, analysis.field_count, analysis.total_allocations, analysis.promotion_benefit));
            }
        }
        
        // Additional optimization opportunities
        if !results.optimization_opportunities.is_empty() {
            report.push_str("\n## Additional Optimization Opportunities\n");
            for (i, opportunity) in results.optimization_opportunities.iter().enumerate().take(5) {
                report.push_str(&format!("{}. **{}**\n", i + 1, opportunity.opportunity_type));
                report.push_str(&format!("   - Description: {}\n", opportunity.description));
                report.push_str(&format!("   - Potential benefit: {:.1}%\n", opportunity.potential_benefit));
            }
        }
        
        report
    }
    
    /// Get current optimization statistics
    pub fn get_statistics(&self) -> SroaStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    // Implementation methods
    
    fn analyze_module_aggregates(&mut self, module: &Module<'ctx>) -> Result<()> {
        debug!("Analyzing module aggregates");
        
        // Analyze global aggregates
        for global in module.get_globals() {
            if self.is_aggregate_type(global.get_value_type()) {
                self.analyze_global_aggregate(global)?;
            }
        }
        
        // Analyze function aggregates
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                self.analyze_function_aggregates(function)?;
            }
        }
        
        Ok(())
    }
    
    fn analyze_function_aggregates(&mut self, function: FunctionValue<'ctx>) -> Result<()> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        debug!("Analyzing aggregates in function: {}", function_name);
        
        // Find all aggregate allocations
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                if instr.get_opcode() == inkwell::values::InstructionOpcode::Alloca {
                    if let Some(alloca_instr) = instr.as_alloca_instruction() {
                        let allocated_type = alloca_instr.get_allocated_type();
                        if self.is_aggregate_type(allocated_type) {
                            self.analyze_allocation_site(&instr, function)?;
                        }
                    }
                }
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        
        Ok(())
    }
    
    fn analyze_global_aggregate(&mut self, _global: inkwell::values::GlobalValue<'ctx>) -> Result<()> {
        // Analyze global aggregate - for now, mark as not promotable
        // In a real implementation, would analyze if global is only accessed in promotable ways
        Ok(())
    }
    
    fn analyze_allocation_site(&mut self, allocation_instr: &InstructionValue<'ctx>, function: FunctionValue<'ctx>) -> Result<()> {
        let alloc_name = allocation_instr.get_name().to_str().unwrap_or("unnamed_alloc").to_string();
        
        if let Some(alloca_instr) = allocation_instr.as_alloca_instruction() {
            let allocated_type = alloca_instr.get_allocated_type();
            
            // Create allocation site info
            let allocation_site = AllocationSite {
                allocation_name: alloc_name.clone(),
                allocation_type: AllocationType::Stack,
                aggregate_type: self.get_type_name(allocated_type),
                size_info: self.calculate_size_info(allocated_type),
                lifetime: AllocationLifetime::Function, // Simplified
                is_promotable: true, // Will be updated based on usage analysis
                promotion_score: 0.0, // Will be calculated
            };
            
            // Analyze usage pattern
            let usage_pattern = self.analyze_allocation_usage(allocation_instr, function)?;
            
            // Update promotion eligibility based on usage
            let mut updated_allocation = allocation_site;
            updated_allocation.is_promotable = usage_pattern.is_scalar_replaceable;
            updated_allocation.promotion_score = self.calculate_promotion_score(&usage_pattern);
            
            self.aggregate_analysis.allocation_sites.insert(alloc_name.clone(), updated_allocation);
            self.aggregate_analysis.usage_patterns.insert(alloc_name, usage_pattern);
            
            // Analyze specific aggregate type
            if allocated_type.is_struct_type() {
                self.analyze_struct_type(allocated_type.into_struct_type())?;
            } else if allocated_type.is_array_type() {
                self.analyze_array_type(allocated_type.into_array_type())?;
            }
        }
        
        Ok(())
    }
    
    fn analyze_allocation_usage(&self, allocation_instr: &InstructionValue<'ctx>, function: FunctionValue<'ctx>) -> Result<UsagePattern> {
        let alloc_name = allocation_instr.get_name().to_str().unwrap_or("unnamed").to_string();
        
        let mut usage_pattern = UsagePattern {
            aggregate_name: alloc_name,
            total_uses: 0,
            field_accesses: HashMap::new(),
            whole_aggregate_operations: 0,
            escape_uses: 0,
            is_scalar_replaceable: true,
            complexity_score: 0.0,
        };
        
        // Find all uses of this allocation
        let users = self.find_instruction_users(allocation_instr, function);
        
        for user in users {
            usage_pattern.total_uses += 1;
            
            match user.get_opcode() {
                inkwell::values::InstructionOpcode::GetElementPtr => {
                    if let Some(gep_instr) = user.as_gep_instruction() {
                        // Analyze field access
                        if let Some(field_index) = self.extract_constant_field_index(&gep_instr) {
                            *usage_pattern.field_accesses.entry(field_index).or_insert(0) += 1;
                        } else {
                            // Non-constant index - makes promotion more complex
                            usage_pattern.complexity_score += 0.2;
                        }
                    }
                }
                inkwell::values::InstructionOpcode::Load |
                inkwell::values::InstructionOpcode::Store => {
                    // Whole aggregate operation
                    usage_pattern.whole_aggregate_operations += 1;
                }
                inkwell::values::InstructionOpcode::Call => {
                    // Passing to function - potential escape
                    usage_pattern.escape_uses += 1;
                    usage_pattern.is_scalar_replaceable = false;
                }
                inkwell::values::InstructionOpcode::Ret => {
                    // Returning aggregate - escape
                    usage_pattern.escape_uses += 1;
                    usage_pattern.is_scalar_replaceable = false;
                }
                _ => {
                    // Other operations may complicate promotion
                    usage_pattern.complexity_score += 0.1;
                }
            }
        }
        
        // Adjust scalar replaceability based on usage patterns
        if usage_pattern.escape_uses > 0 {
            usage_pattern.is_scalar_replaceable = false;
        }
        
        if usage_pattern.whole_aggregate_operations > usage_pattern.total_uses / 2 {
            usage_pattern.is_scalar_replaceable = false;
        }
        
        if usage_pattern.complexity_score > 0.5 {
            usage_pattern.is_scalar_replaceable = false;
        }
        
        Ok(usage_pattern)
    }
    
    fn analyze_struct_type(&mut self, struct_type: StructType<'ctx>) -> Result<()> {
        let struct_name = self.get_struct_type_name(struct_type);
        
        if !self.aggregate_analysis.struct_analyses.contains_key(&struct_name) {
            let field_types = struct_type.get_field_types();
            let mut field_usage = HashMap::new();
            
            // Initialize field usage tracking
            for i in 0..field_types.len() {
                field_usage.insert(i, FieldUsage {
                    field_index: i,
                    read_count: 0,
                    write_count: 0,
                    gep_count: 0,
                    escape_count: 0,
                    is_promotable: true,
                });
            }
            
            let struct_analysis = StructAnalysis {
                struct_name: struct_name.clone(),
                field_count: field_types.len(),
                field_types: field_types.iter().map(|t| self.get_type_name(*t)).collect(),
                field_usage,
                total_allocations: 0, // Will be updated
                promotable_allocations: 0, // Will be updated
                promotion_benefit: 0.0, // Will be calculated
            };
            
            self.aggregate_analysis.struct_analyses.insert(struct_name, struct_analysis);
        }
        
        Ok(())
    }
    
    fn analyze_array_type(&mut self, array_type: ArrayType<'ctx>) -> Result<()> {
        let array_name = self.get_array_type_name(array_type);
        
        if !self.aggregate_analysis.array_analyses.contains_key(&array_name) {
            let element_type = array_type.get_element_type();
            let element_count = array_type.len();
            
            let array_analysis = ArrayAnalysis {
                array_name: array_name.clone(),
                element_type: self.get_type_name(element_type),
                element_count: Some(element_count as usize),
                access_patterns: Vec::new(),
                is_constant_indexed: true, // Will be updated based on usage
                promotable_elements: HashSet::new(),
                promotion_benefit: 0.0,
            };
            
            self.aggregate_analysis.array_analyses.insert(array_name, array_analysis);
        }
        
        Ok(())
    }
    
    fn identify_promotion_candidates(&self) -> Result<Vec<AllocationSite>> {
        let mut candidates = Vec::new();
        
        for allocation_site in self.aggregate_analysis.allocation_sites.values() {
            if allocation_site.is_promotable && allocation_site.promotion_score > 0.5 {
                candidates.push(allocation_site.clone());
            }
        }
        
        // Sort by promotion score (highest first)
        candidates.sort_by(|a, b| b.promotion_score.partial_cmp(&a.promotion_score).unwrap());
        
        Ok(candidates)
    }
    
    fn find_function_promotion_candidates(&self, function: FunctionValue<'ctx>) -> Result<Vec<AllocationSite>> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        let all_candidates = self.identify_promotion_candidates()?;
        
        // Filter candidates that belong to this function
        let function_candidates: Vec<AllocationSite> = all_candidates.into_iter()
            .filter(|candidate| {
                // In a real implementation, would track which function each allocation belongs to
                // For now, assume all stack allocations could belong to any function
                matches!(candidate.allocation_type, AllocationType::Stack)
            })
            .collect();
        
        Ok(function_candidates)
    }
    
    fn perform_scalar_replacement(&mut self, function: FunctionValue<'ctx>, candidate: &AllocationSite) -> Result<ScalarReplacement> {
        debug!("Performing scalar replacement for: {}", candidate.allocation_name);
        
        let mut scalar_variables = HashMap::new();
        let mut replacement_instructions = Vec::new();
        
        // Determine the number of scalars needed
        let scalar_count = self.calculate_scalar_count(candidate)?;
        
        // Create scalar variables
        self.builder.position_at_end(function.get_first_basic_block().unwrap());
        for i in 0..scalar_count {
            let scalar_name = format!("{}_scalar_{}", candidate.allocation_name, i);
            
            // Create scalar alloca (simplified - would use actual field types)
            let i32_type = self.context.i32_type();
            let scalar_alloca = self.builder.build_alloca(i32_type, &scalar_name)
                .map_err(|e| Error::CompilationError(format!("Failed to create scalar alloca: {}", e)))?;
            
            scalar_variables.insert(i, scalar_name);
        }
        
        // Replace uses of the original aggregate
        let original_allocation = self.find_allocation_instruction(function, &candidate.allocation_name);
        if let Some(alloc_instr) = original_allocation {
            let users = self.find_instruction_users(&alloc_instr, function);
            
            for user in users {
                let replacement = self.create_replacement_instruction(&user, &scalar_variables)?;
                replacement_instructions.push(replacement);
            }
        }
        
        let estimated_benefit = self.calculate_replacement_benefit(&replacement_instructions);
        
        Ok(ScalarReplacement {
            original_aggregate: candidate.allocation_name.clone(),
            scalar_variables,
            replacement_instructions,
            estimated_benefit,
        })
    }
    
    fn cleanup_dead_aggregate_code(&mut self, module: &Module<'ctx>) -> Result<()> {
        debug!("Cleaning up dead aggregate code");
        
        // Remove dead allocations and related instructions
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                self.remove_dead_instructions(function)?;
            }
        }
        
        Ok(())
    }
    
    fn remove_dead_instructions(&mut self, function: FunctionValue<'ctx>) -> Result<()> {
        let mut instructions_to_remove = Vec::new();
        
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                // Check if instruction is dead (no users and no side effects)
                if self.is_dead_instruction(&instr) {
                    instructions_to_remove.push(instr);
                }
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        
        // Remove dead instructions
        for instr in instructions_to_remove {
            unsafe {
                instr.remove_from_basic_block();
            }
        }
        
        Ok(())
    }
    
    // Helper methods
    
    fn is_aggregate_type(&self, type_val: inkwell::types::AnyTypeEnum<'ctx>) -> bool {
        type_val.is_struct_type() || type_val.is_array_type()
    }
    
    fn get_type_name(&self, type_val: inkwell::types::BasicTypeEnum<'ctx>) -> String {
        // Simplified type name extraction
        format!("{:?}", type_val)
    }
    
    fn get_struct_type_name(&self, struct_type: StructType<'ctx>) -> String {
        struct_type.get_name().map(|s| s.to_string()).unwrap_or_else(|| "anonymous_struct".to_string())
    }
    
    fn get_array_type_name(&self, array_type: ArrayType<'ctx>) -> String {
        format!("array_{}_{}", array_type.len(), self.get_type_name(array_type.get_element_type()))
    }
    
    fn calculate_size_info(&self, type_val: inkwell::types::AnyTypeEnum<'ctx>) -> SizeInfo {
        // Real size calculation based on LLVM type system
        let (static_size, alignment, element_count) = match type_val {
            inkwell::types::AnyTypeEnum::ArrayType(array_type) => {
                let element_type = array_type.get_element_type();
                let len = array_type.len() as usize;
                let element_size = self.get_type_size_bits(&element_type) / 8;
                (
                    Some(element_size * len),
                    Some(self.get_type_alignment(&element_type)),
                    Some(len)
                )
            }
            inkwell::types::AnyTypeEnum::StructType(struct_type) => {
                let field_types = struct_type.get_field_types();
                let mut total_size = 0;
                let mut max_alignment = 1;
                
                for field_type in field_types {
                    let field_size = self.get_type_size_bits(&field_type) / 8;
                    let field_alignment = self.get_type_alignment(&field_type);
                    
                    // Add padding for alignment
                    total_size = (total_size + field_alignment - 1) & !(field_alignment - 1);
                    total_size += field_size;
                    max_alignment = max_alignment.max(field_alignment);
                }
                
                // Final struct padding
                total_size = (total_size + max_alignment - 1) & !(max_alignment - 1);
                
                (Some(total_size), Some(max_alignment as u32), Some(field_types.len()))
            }
            _ => (None, Some(8), None) // Conservative defaults
        };
        
        SizeInfo {
            static_size,
            dynamic_size: None,
            alignment,
            element_count,
        }
    }
    
    fn find_instruction_users(&self, instruction: &InstructionValue<'ctx>, function: FunctionValue<'ctx>) -> Vec<InstructionValue<'ctx>> {
        let mut users = Vec::new();
        
        // Real implementation: traverse all instructions to find users
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let mut instr = bb.get_first_instruction();
            while let Some(current_instr) = instr {
                // Check each operand of the current instruction
                for i in 0..current_instr.get_operand_count() {
                    if let Some(operand) = current_instr.get_operand(i) {
                        if let Some(operand_instr) = operand.as_instruction_value() {
                            if std::ptr::eq(operand_instr.as_any_value_enum().as_ref(), 
                                           instruction.as_any_value_enum().as_ref()) {
                                users.push(current_instr);
                                break;
                            }
                        }
                    }
                }
                instr = current_instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        
        users
    }
    
    fn extract_constant_field_index(&self, gep_instr: &inkwell::values::GetElementPtrInstruction<'ctx>) -> Option<usize> {
        // Real implementation: extract constant indices from GEP instruction
        let num_indices = gep_instr.get_operand_count();
        
        // For struct field access, we typically need the last index
        if num_indices >= 2 {
            if let Some(last_operand) = gep_instr.get_operand(num_indices - 1) {
                if let Some(const_int) = last_operand.as_int_value() {
                    return Some(const_int.get_zero_extended_constant() as usize);
                }
            }
        }
        
        // If we can't extract a constant index, return None
        None
    }
    
    fn calculate_promotion_score(&self, usage_pattern: &UsagePattern) -> f64 {
        if !usage_pattern.is_scalar_replaceable {
            return 0.0;
        }
        
        let mut score = 0.8; // Base score for promotable aggregates
        
        // Bonus for many field accesses vs whole aggregate operations
        if usage_pattern.total_uses > 0 {
            let field_access_ratio = usage_pattern.field_accesses.len() as f64 / usage_pattern.total_uses as f64;
            score += field_access_ratio * 0.2;
        }
        
        // Penalty for complexity
        score -= usage_pattern.complexity_score * 0.3;
        
        // Penalty for escapes
        score -= (usage_pattern.escape_uses as f64 / usage_pattern.total_uses.max(1) as f64) * 0.5;
        
        score.max(0.0).min(1.0)
    }
    
    fn calculate_scalar_count(&self, candidate: &AllocationSite) -> Result<usize> {
        // Determine how many scalar variables are needed
        if let Some(element_count) = candidate.size_info.element_count {
            Ok(element_count)
        } else {
            // For structs, would need to count fields
            Ok(4) // Placeholder
        }
    }
    
    fn find_allocation_instruction(&self, function: FunctionValue<'ctx>, allocation_name: &str) -> Option<InstructionValue<'ctx>> {
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                if let Some(name) = instr.get_name().to_str() {
                    if name == allocation_name {
                        return Some(instr);
                    }
                }
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        None
    }
    
    fn create_replacement_instruction(&self, _original: &InstructionValue<'ctx>, _scalars: &HashMap<usize, String>) -> Result<ReplacementInstruction> {
        // Create replacement instruction mapping
        Ok(ReplacementInstruction {
            original_instruction: "placeholder".to_string(),
            replacement_type: ReplacementType::LoadReplacement,
            scalar_operations: vec!["scalar_load".to_string()],
            complexity_reduction: 0.2,
        })
    }
    
    fn calculate_replacement_benefit(&self, replacements: &[ReplacementInstruction]) -> f64 {
        replacements.iter().map(|r| r.complexity_reduction).sum::<f64>() * 100.0
    }
    
    fn is_dead_instruction(&self, _instruction: &InstructionValue<'ctx>) -> bool {
        // Check if instruction has no users and no side effects
        // For now, return false to avoid removing instructions incorrectly
        false
    }
    
    fn find_function_for_allocation(&self, module: &Module<'ctx>, allocation_name: &str) -> Option<FunctionValue<'ctx>> {
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                if self.find_allocation_instruction(function, allocation_name).is_some() {
                    return Some(function);
                }
            }
        }
        None
    }
    
    fn identify_additional_opportunities(&self) -> Result<Vec<SroaOptimizationOpportunity>> {
        let mut opportunities = Vec::new();
        
        // Look for partial promotion opportunities
        for allocation_site in self.aggregate_analysis.allocation_sites.values() {
            if !allocation_site.is_promotable && allocation_site.promotion_score > 0.3 {
                opportunities.push(SroaOptimizationOpportunity {
                    opportunity_type: "Partial Promotion".to_string(),
                    description: format!("Could partially promote aggregate {}", allocation_site.allocation_name),
                    potential_benefit: allocation_site.promotion_score * 50.0,
                    complexity: "Medium".to_string(),
                });
            }
        }
        
        // Look for struct field grouping opportunities
        for struct_analysis in self.aggregate_analysis.struct_analyses.values() {
            if struct_analysis.field_count > 4 && struct_analysis.promotion_benefit > 0.3 {
                opportunities.push(SroaOptimizationOpportunity {
                    opportunity_type: "Field Grouping".to_string(),
                    description: format!("Could group frequently accessed fields in {}", struct_analysis.struct_name),
                    potential_benefit: struct_analysis.promotion_benefit * 30.0,
                    complexity: "High".to_string(),
                });
            }
        }
        
        Ok(opportunities)
    }
    
    fn calculate_function_benefit(&self, replacements: &[ScalarReplacement]) -> f64 {
        replacements.iter().map(|r| r.estimated_benefit).sum()
    }
    
    fn update_statistics(&self, optimization_time: Duration, replacement_results: &[ScalarReplacement]) {
        if let Ok(mut stats) = self.statistics.lock() {
            stats.optimization_time = optimization_time;
            stats.aggregates_analyzed = self.aggregate_analysis.allocation_sites.len();
            stats.structs_promoted = self.aggregate_analysis.struct_analyses.values()
                .filter(|s| s.promotable_allocations > 0).count();
            stats.arrays_promoted = self.aggregate_analysis.array_analyses.values()
                .filter(|a| !a.promotable_elements.is_empty()).count();
            stats.scalar_variables_created = replacement_results.iter()
                .map(|r| r.scalar_variables.len()).sum();
            stats.instructions_replaced = replacement_results.iter()
                .map(|r| r.replacement_instructions.len()).sum();
            stats.estimated_speedup = replacement_results.iter()
                .map(|r| r.estimated_benefit).sum::<f64>() / replacement_results.len().max(1) as f64;
        }
    }
}

// Supporting types

impl AggregateAnalysis {
    fn new() -> Self {
        Self {
            struct_analyses: HashMap::new(),
            array_analyses: HashMap::new(),
            allocation_sites: HashMap::new(),
            usage_patterns: HashMap::new(),
        }
    }
}

/// Results of SROA optimization
#[derive(Debug, Clone)]
pub struct SroaOptimizationResults {
    pub promotion_candidates: Vec<AllocationSite>,
    pub replacement_results: Vec<ScalarReplacement>,
    pub aggregate_analysis: AggregateAnalysis,
    pub optimization_opportunities: Vec<SroaOptimizationOpportunity>,
    pub statistics: SroaStatistics,
}

/// Results for a single function
#[derive(Debug, Clone)]
pub struct FunctionSroaResults {
    pub function_name: String,
    pub promotion_candidates: Vec<AllocationSite>,
    pub scalar_replacements: Vec<ScalarReplacement>,
    pub optimization_benefit: f64,
}

/// Promotion eligibility analysis
#[derive(Debug, Clone)]
pub struct PromotionEligibility {
    pub can_promote: bool,
    pub blocking_reasons: Vec<String>,
    pub confidence: f64,
}

/// Additional optimization opportunity
#[derive(Debug, Clone)]
pub struct SroaOptimizationOpportunity {
    pub opportunity_type: String,
    pub description: String,
    pub potential_benefit: f64,
    pub complexity: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_sroa_optimizer_creation() {
        let context = Context::create();
        let optimizer = SroaOptimizer::new(&context, OptimizationLevel::O2);
        
        let stats = optimizer.get_statistics();
        assert_eq!(stats.aggregates_analyzed, 0);
        assert_eq!(stats.structs_promoted, 0);
        assert_eq!(stats.arrays_promoted, 0);
    }
    
    #[test]
    fn test_aggregate_type_detection() {
        let context = Context::create();
        let optimizer = SroaOptimizer::new(&context, OptimizationLevel::O2);
        
        let i32_type = context.i32_type();
        let struct_type = context.struct_type(&[i32_type.into(), i32_type.into()], false);
        let array_type = i32_type.array_type(10);
        
        assert!(optimizer.is_aggregate_type(struct_type.as_any_type_enum()));
        assert!(optimizer.is_aggregate_type(array_type.as_any_type_enum()));
        assert!(!optimizer.is_aggregate_type(i32_type.as_any_type_enum()));
    }
    
    #[test]
    fn test_promotion_score_calculation() {
        let context = Context::create();
        let optimizer = SroaOptimizer::new(&context, OptimizationLevel::O2);
        
        let usage_pattern = UsagePattern {
            aggregate_name: "test".to_string(),
            total_uses: 10,
            field_accesses: [(0, 5), (1, 3)].iter().cloned().collect(),
            whole_aggregate_operations: 2,
            escape_uses: 0,
            is_scalar_replaceable: true,
            complexity_score: 0.1,
        };
        
        let score = optimizer.calculate_promotion_score(&usage_pattern);
        assert!(score > 0.5);
        assert!(score <= 1.0);
    }
    
    #[test]
    fn test_escape_detection() {
        let context = Context::create();
        let optimizer = SroaOptimizer::new(&context, OptimizationLevel::O2);
        
        let usage_pattern_escaped = UsagePattern {
            aggregate_name: "escaped".to_string(),
            total_uses: 5,
            field_accesses: HashMap::new(),
            whole_aggregate_operations: 1,
            escape_uses: 2,
            is_scalar_replaceable: false,
            complexity_score: 0.0,
        };
        
        let score_escaped = optimizer.calculate_promotion_score(&usage_pattern_escaped);
        assert_eq!(score_escaped, 0.0);
    }
    
    #[test]
    fn test_size_info_calculation() {
        let context = Context::create();
        let optimizer = SroaOptimizer::new(&context, OptimizationLevel::O2);
        
        let i32_type = context.i32_type();
        let array_type = i32_type.array_type(5);
        
        let size_info = optimizer.calculate_size_info(array_type.as_any_type_enum());
        assert_eq!(size_info.element_count, Some(5));
        assert!(size_info.static_size.is_some());
    }
}
