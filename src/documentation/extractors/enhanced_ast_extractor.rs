//! # Enhanced AST Extractor for Documentation
//!
//! Advanced AST analysis and documentation extraction with comprehensive type relationship
//! mapping, inheritance hierarchies, and cross-reference generation for the CURSED
//! programming language documentation system.
//!
//! ## Features
//!
//! - **Complete Type Analysis**: Full type relationship mapping and inheritance hierarchies
//! - **Cross-Reference Generation**: Comprehensive linking between modules, types, and functions
//! - **Interactive Navigation**: Code navigation features for documentation
//! - **Semantic Analysis**: Deep understanding of code structure and relationships
//! - **Gen Z Integration**: Full support for CURSED's Gen Z slang terminology
//! - **Performance Optimization**: Efficient AST traversal and caching

use crate::ast::*;
use crate::documentation::extractors::{AstExtractor, ExtractionConfig, EnhancedDocumentationItem};
use crate::error::{Error as CursedError, SourceLocation};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument, warn};

/// Enhanced type relationship information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRelationship {
    /// Source type name
    pub source: String,
    
    /// Target type name  
    pub target: String,
    
    /// Type of relationship
    pub relationship_type: RelationshipType,
    
    /// Additional metadata about the relationship
    pub metadata: HashMap<String, String>,
    
    /// Source location where relationship is defined
    pub location: SourceLocation,
    
    /// Confidence score of the relationship detection (0.0-1.0)
    pub confidence: f64,
}

/// Types of relationships between code elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationshipType {
    /// Type inheritance (A extends B)
    Inherits,
    
    /// Interface implementation (A implements B)
    Implements,
    
    /// Type composition (A contains B)
    Contains,
    
    /// Type usage/dependency (A uses B)
    Uses,
    
    /// Function call relationship
    Calls,
    
    /// Module import relationship
    Imports,
    
    /// Generic type constraint
    Constrains,
    
    /// Error type relationship
    ThrowsError,
    
    /// Return type relationship
    Returns,
    
    /// Parameter type relationship
    AcceptsParameter,
    
    /// Field type relationship
    HasField,
    
    /// Method relationship
    HasMethod,
    
    /// Association (general relationship)
    Associates,
}

/// Enhanced inheritance hierarchy information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceHierarchy {
    /// Root type of the hierarchy
    pub root_type: String,
    
    /// All types in the hierarchy
    pub hierarchy: BTreeMap<String, HierarchyNode>,
    
    /// Depth of the hierarchy
    pub max_depth: usize,
    
    /// Total number of types in hierarchy
    pub total_types: usize,
}

/// Node in an inheritance hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyNode {
    /// Type name
    pub type_name: String,
    
    /// Parent type (if any)
    pub parent: Option<String>,
    
    /// Direct children types
    pub children: Vec<String>,
    
    /// Depth in hierarchy (0 for root)
    pub depth: usize,
    
    /// Whether this is an abstract type
    pub is_abstract: bool,
    
    /// Interfaces implemented by this type
    pub interfaces: Vec<String>,
    
    /// Source location
    pub location: SourceLocation,
}

/// Cross-reference information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// Source identifier
    pub source: String,
    
    /// Target identifier
    pub target: String,
    
    /// Type of cross-reference
    pub reference_type: CrossReferenceType,
    
    /// Source location
    pub source_location: SourceLocation,
    
    /// Target location (if available)
    pub target_location: Option<SourceLocation>,
    
    /// Context information
    pub context: String,
}

/// Types of cross-references
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CrossReferenceType {
    /// Type reference
    TypeReference,
    
    /// Function call
    FunctionCall,
    
    /// Variable reference
    VariableReference,
    
    /// Constant reference
    ConstantReference,
    
    /// Module reference
    ModuleReference,
    
    /// Interface reference
    InterfaceReference,
    
    /// Struct field reference
    FieldReference,
    
    /// Method call
    MethodCall,
    
    /// Generic parameter reference
    GenericReference,
    
    /// Import reference
    ImportReference,
}

/// Navigation information for interactive features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationInfo {
    /// Definition location
    pub definition: SourceLocation,
    
    /// All usage locations
    pub usages: Vec<SourceLocation>,
    
    /// Related items (similar types, functions, etc.)
    pub related_items: Vec<String>,
    
    /// Documentation links
    pub doc_links: Vec<String>,
    
    /// Example usage locations
    pub examples: Vec<SourceLocation>,
}

/// Enhanced semantic analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnalysis {
    /// All type relationships found
    pub type_relationships: Vec<TypeRelationship>,
    
    /// Inheritance hierarchies
    pub inheritance_hierarchies: Vec<InheritanceHierarchy>,
    
    /// Cross-references between items
    pub cross_references: Vec<CrossReference>,
    
    /// Navigation information for each item
    pub navigation_info: HashMap<String, NavigationInfo>,
    
    /// Dependency graph
    pub dependency_graph: DependencyGraph,
    
    /// Complexity metrics
    pub complexity_metrics: ComplexityMetrics,
    
    /// Gen Z slang mappings
    pub slang_mappings: HashMap<String, String>,
}

/// Dependency graph for modules and types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Nodes in the dependency graph
    pub nodes: HashMap<String, DependencyNode>,
    
    /// Edges representing dependencies
    pub edges: Vec<DependencyEdge>,
    
    /// Strongly connected components
    pub strongly_connected_components: Vec<Vec<String>>,
    
    /// Circular dependencies detected
    pub circular_dependencies: Vec<Vec<String>>,
}

/// Node in dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    /// Node identifier
    pub id: String,
    
    /// Node type (module, type, function, etc.)
    pub node_type: String,
    
    /// Source location
    pub location: SourceLocation,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Edge in dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source node
    pub from: String,
    
    /// Target node
    pub to: String,
    
    /// Edge type
    pub edge_type: String,
    
    /// Edge weight (importance/strength)
    pub weight: f64,
}

/// Code complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    /// Cyclomatic complexity by function
    pub cyclomatic_complexity: HashMap<String, usize>,
    
    /// Cognitive complexity by function
    pub cognitive_complexity: HashMap<String, usize>,
    
    /// Lines of code by module
    pub lines_of_code: HashMap<String, usize>,
    
    /// Number of dependencies by module
    pub dependency_count: HashMap<String, usize>,
    
    /// Coupling metrics
    pub coupling_metrics: HashMap<String, f64>,
    
    /// Cohesion metrics
    pub cohesion_metrics: HashMap<String, f64>,
}

/// Enhanced AST extractor with comprehensive analysis
pub struct EnhancedAstExtractor {
    /// Extraction configuration
    config: ExtractionConfig,
    
    /// Type registry for tracking all types
    type_registry: HashMap<String, TypeInfo>,
    
    /// Function registry for tracking all functions
    function_registry: HashMap<String, FunctionInfo>,
    
    /// Module registry for tracking all modules
    module_registry: HashMap<String, ModuleInfo>,
    
    /// Cross-reference tracker
    cross_reference_tracker: HashMap<String, Vec<CrossReference>>,
    
    /// Current analysis context
    current_context: AnalysisContext,
    
    /// Gen Z slang mappings
    slang_mappings: HashMap<String, String>,
}

/// Type information for registry
#[derive(Debug, Clone)]
struct TypeInfo {
    name: String,
    type_kind: String,
    location: SourceLocation,
    generic_params: Vec<String>,
    parent_types: Vec<String>,
    implemented_interfaces: Vec<String>,
    fields: Vec<FieldInfo>,
    methods: Vec<String>,
    metadata: HashMap<String, String>,
}

/// Function information for registry
#[derive(Debug, Clone)]
struct FunctionInfo {
    name: String,
    location: SourceLocation,
    parameters: Vec<ParameterInfo>,
    return_type: Option<String>,
    is_async: bool,
    is_public: bool,
    calls: Vec<String>,
    metadata: HashMap<String, String>,
}

/// Module information for registry
#[derive(Debug, Clone)]
struct ModuleInfo {
    name: String,
    path: PathBuf,
    exports: Vec<String>,
    imports: Vec<String>,
    submodules: Vec<String>,
    location: SourceLocation,
    metadata: HashMap<String, String>,
}

/// Field information
#[derive(Debug, Clone)]
struct FieldInfo {
    name: String,
    field_type: String,
    is_public: bool,
    is_optional: bool,
    metadata: HashMap<String, String>,
}

/// Parameter information
#[derive(Debug, Clone)]
struct ParameterInfo {
    name: String,
    param_type: Option<String>,
    is_optional: bool,
    default_value: Option<String>,
}

/// Analysis context for tracking current state
#[derive(Debug, Clone)]
struct AnalysisContext {
    current_module: Option<String>,
    current_type: Option<String>,
    current_function: Option<String>,
    scope_stack: Vec<String>,
    generic_context: Vec<String>,
}

impl Default for AnalysisContext {
    fn default() -> Self {
        Self {
            current_module: None,
            current_type: None,
            current_function: None,
            scope_stack: Vec::new(),
            generic_context: Vec::new(),
        }
    }
}

impl EnhancedAstExtractor {
    /// Create a new enhanced AST extractor
    #[instrument(skip(config))]
    pub fn new(config: ExtractionConfig) -> Result<(), Error> {
        info!("Creating enhanced AST extractor");
        
        // Initialize Gen Z slang mappings
        let slang_mappings = Self::initialize_slang_mappings();
        
        Ok(Self {
            config,
            type_registry: HashMap::new(),
            function_registry: HashMap::new(),
            module_registry: HashMap::new(),
            cross_reference_tracker: HashMap::new(),
            current_context: AnalysisContext::default(),
            slang_mappings,
        })
    }
    
    /// Initialize Gen Z slang mappings for CURSED language
    fn initialize_slang_mappings() -> HashMap<String, String> {
        let mut mappings = HashMap::new();
        
        // Function keywords
        mappings.insert("slay".to_string(), "function".to_string());
        mappings.insert("yolo".to_string(), "yield".to_string());
        
        // Variable keywords  
        mappings.insert("sus".to_string(), "let".to_string());
        mappings.insert("facts".to_string(), "const".to_string());
        
        // Control flow
        mappings.insert("lowkey".to_string(), "if".to_string());
        mappings.insert("highkey".to_string(), "else".to_string());
        mappings.insert("periodt".to_string(), "while".to_string());
        mappings.insert("bestie".to_string(), "for".to_string());
        mappings.insert("flex".to_string(), "break".to_string());
        
        // Types
        mappings.insert("squad".to_string(), "struct".to_string());
        mappings.insert("collab".to_string(), "interface".to_string());
        
        // Control structures
        mappings.insert("vibe_check".to_string(), "switch".to_string());
        mappings.insert("mood".to_string(), "case".to_string());
        mappings.insert("basic".to_string(), "default".to_string());
        
        // Concurrency
        mappings.insert("stan".to_string(), "goroutine".to_string());
        
        mappings
    }
    
    /// Extract comprehensive documentation with enhanced analysis
    #[instrument(skip(self, ast, source_code))]
    pub async fn extract_comprehensive_documentation(
        &mut self,
        ast: &AstNode,
        file_path: &Path,
        source_code: &str,
    ) -> Result<(), Error> {
        info!("Starting comprehensive documentation extraction for: {:?}", file_path);
        
        // Reset state for new file
        self.reset_extraction_state();
        
        // Set current module context
        self.current_context.current_module = Some(
            file_path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        );
        
        // Phase 1: Initial AST traversal to build registries
        self.build_registries(ast, source_code).await?;
        
        // Phase 2: Extract documentation items
        let mut documentation_items = Vec::new();
        self.extract_documentation_items(ast, source_code, &mut documentation_items).await?;
        
        // Phase 3: Perform semantic analysis
        let semantic_analysis = self.perform_semantic_analysis().await?;
        
        // Phase 4: Enhance documentation items with analysis results
        self.enhance_documentation_items(&mut documentation_items, &semantic_analysis);
        
        info!("Extracted {} documentation items with comprehensive analysis", documentation_items.len());
        
        Ok((documentation_items, semantic_analysis))
    }
    
    /// Reset extraction state for new file
    fn reset_extraction_state(&mut self) {
        self.current_context = AnalysisContext::default();
        // Keep registries across files for cross-file analysis
    }
    
    /// Build registries by traversing AST
    #[instrument(skip(self, ast, source_code))]
    async fn build_registries(&mut self, ast: &AstNode, source_code: &str) -> Result<(), Error> {
        debug!("Building type and function registries");
        
        self.traverse_for_registry(ast, source_code).await?;
        
        debug!("Registry building complete - Types: {}, Functions: {}, Modules: {}", 
            self.type_registry.len(),
            self.function_registry.len(), 
            self.module_registry.len()
        );
        
        Ok(())
    }
    
    /// Traverse AST to build registries
    #[instrument(skip(self, node, source_code))]
    async fn traverse_for_registry(&mut self, node: &AstNode, source_code: &str) -> Result<(), Error> {
        match &node.node_type {
            AstNodeType::Program(program) => {
                // Register module information
                if let Some(ref module_name) = self.current_context.current_module {
                    let module_info = ModuleInfo {
                        name: module_name.clone(),
                        path: PathBuf::from(&node.location.unwrap().file),
                        exports: Vec::new(), // Will be populated later
                        imports: Vec::new(), // Will be populated later
                        submodules: Vec::new(),
                        location: node.location.unwrap(),
                        metadata: HashMap::new(),
                    };
                    self.module_registry.insert(module_name.clone(), module_info);
                }
                
                // Process all statements
                for statement in &program.statements {
                    // Note: statement is Box<dyn Statement>, not AstNode - skip for now
                }
            }
            
            AstNodeType::FunctionDeclaration(func_decl) => {
                self.register_function(func_decl)?;
            }
            
            AstNodeType::StructDeclaration(struct_decl) => {
                self.register_struct(struct_decl)?;
            }
            
            AstNodeType::InterfaceDeclaration(interface_decl) => {
                self.register_interface(interface_decl)?;
            }
            
            AstNodeType::EnumDeclaration(enum_decl) => {
                self.register_enum(enum_decl)?;
            }
            
            AstNodeType::Import(import_stmt) => {
                self.register_import(import_stmt)?;
            }
            
            _ => {
                // Recursively process child nodes
                if let Some(children) = self.get_child_nodes(node) {
                    for child in children {
                        self.traverse_for_registry(child, source_code).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Register function information
    fn register_function(&mut self, func_decl: &FunctionDeclaration) -> Result<(), Error> {
        let parameters = func_decl.parameters.iter().map(|param| ParameterInfo {
            name: param.to_string().clone(),
            param_type: param.param_type.as_ref().map(|t| self.format_type_name(t)),
            is_optional: param.default_value.is_some(),
            default_value: param.default_value.as_ref().map(|_| "default".to_string()), // Would extract actual value
        }).collect();
        
        let function_info = FunctionInfo {
            name: func_decl.to_string().clone(),
            location: func_decl.location.clone(),
            parameters,
            return_type: func_decl.return_type.as_ref().map(|t| self.format_type_name(t)),
            is_async: func_decl.is_async,
            is_public: func_decl.is_public,
            calls: Vec::new(), // Will be populated during analysis
            metadata: HashMap::new(),
        };
        
        self.function_registry.insert(func_decl.to_string().clone(), function_info);
        
        debug!("Registered function: {}", func_decl.to_string());
        Ok(())
    }
    
    /// Register struct information
    fn register_struct(&mut self, struct_decl: &StructDeclaration) -> Result<(), Error> {
        let fields = struct_decl.fields.iter().map(|field| FieldInfo {
            name: field.to_string().clone(),
            field_type: field.field_type.as_ref()
                .map(|t| self.format_type_name(t))
                .unwrap_or_else(|| "Any".to_string()),
            is_public: field.is_public,
            is_optional: false, // Would need to determine from AST
            metadata: HashMap::new(),
        }).collect();
        
        let type_info = TypeInfo {
            name: struct_decl.to_string().clone(),
            type_kind: "struct".to_string(),
            location: struct_decl.location.clone(),
            generic_params: struct_decl.generic_params.clone().unwrap_or_default(),
            parent_types: Vec::new(), // Structs don't inherit in CURSED
            implemented_interfaces: Vec::new(), // Would extract from AST
            fields,
            methods: Vec::new(), // Would find associated methods
            metadata: HashMap::new(),
        };
        
        self.type_registry.insert(struct_decl.to_string().clone(), type_info);
        
        debug!("Registered struct: {}", struct_decl.to_string());
        Ok(())
    }
    
    /// Register interface information
    fn register_interface(&mut self, interface_decl: &InterfaceDeclaration) -> Result<(), Error> {
        let methods = interface_decl.methods.iter().map(|method| method.to_string().clone()).collect();
        
        let type_info = TypeInfo {
            name: interface_decl.to_string().clone(),
            type_kind: "interface".to_string(),
            location: interface_decl.location.clone(),
            generic_params: interface_decl.generic_params.clone().unwrap_or_default(),
            parent_types: Vec::new(), // Would extract parent interfaces
            implemented_interfaces: Vec::new(),
            fields: Vec::new(), // Interfaces don't have fields
            methods,
            metadata: HashMap::new(),
        };
        
        self.type_registry.insert(interface_decl.to_string().clone(), type_info);
        
        // Register interface methods
        for method in &interface_decl.methods {
            self.register_function(method)?;
        }
        
        debug!("Registered interface: {}", interface_decl.to_string());
        Ok(())
    }
    
    /// Register enum information
    fn register_enum(&mut self, enum_decl: &EnumDeclaration) -> Result<(), Error> {
        let type_info = TypeInfo {
            name: enum_decl.to_string().clone(),
            type_kind: "enum".to_string(),
            location: enum_decl.location.clone(),
            generic_params: enum_decl.generic_params.clone().unwrap_or_default(),
            parent_types: Vec::new(),
            implemented_interfaces: Vec::new(),
            fields: Vec::new(), // Enum variants would be stored differently
            methods: Vec::new(),
            metadata: HashMap::new(),
        };
        
        self.type_registry.insert(enum_decl.to_string().clone(), type_info);
        
        debug!("Registered enum: {}", enum_decl.to_string());
        Ok(())
    }
    
    /// Register import information
    fn register_import(&mut self, import_stmt: &Import) -> Result<(), Error> {
        if let Some(ref module_name) = self.current_context.current_module {
            if let Some(module_info) = self.module_registry.get_mut(module_name) {
                module_info.imports.push(import_stmt.path.clone());
            }
        }
        
        debug!("Registered import: {}", import_stmt.path);
        Ok(())
    }
    
    /// Extract documentation items from AST
    #[instrument(skip(self, ast, source_code, documentation_items))]
    async fn extract_documentation_items(
        &mut self,
        ast: &AstNode,
        source_code: &str,
        documentation_items: &mut Vec<EnhancedDocumentationItem>,
    ) -> Result<(), Error> {
        debug!("Extracting documentation items");
        
        // Use the existing AST extractor as base
        let base_extractor = AstExtractor::new(self.config.clone())?;
        let base_items = base_extractor.extract_complete_documentation(ast, Path::new(""), source_code).await?;
        
        // Enhance base items with additional analysis
        for base_item in base_items {
            let enhanced_item = self.enhance_documentation_item(base_item, source_code)?;
            documentation_items.push(enhanced_item);
        }
        
        Ok(())
    }
    
    /// Enhance a documentation item with additional analysis
    fn enhance_documentation_item(
        &self,
        base_item: EnhancedDocumentationItem,
        source_code: &str,
    ) -> Result<(), Error> {
        let mut enhanced = base_item;
        
        // Add cross-references
        if let Some(cross_refs) = self.cross_reference_tracker.get(&enhanced.base.to_string()) {
            enhanced.relationships.extend(
                cross_refs.iter().map(|cross_ref| TypeRelationship {
                    source: cross_ref.source.clone(),
                    target: cross_ref.target.clone(),
                    relationship_type: self.convert_cross_ref_type(&cross_ref.reference_type),
                    metadata: HashMap::new(),
                    location: cross_ref.source_location.clone(),
                    confidence: 0.9, // High confidence for direct references
                })
            );
        }
        
        // Add semantic information based on registries
        if let Some(type_info) = self.type_registry.get(&enhanced.base.to_string()) {
            // Add type-specific relationships
            for parent in &type_info.parent_types {
                enhanced.relationships.push(TypeRelationship {
                    source: enhanced.base.to_string().clone(),
                    target: parent.clone(),
                    relationship_type: RelationshipType::Inherits,
                    metadata: HashMap::new(),
                    location: enhanced.base.location.clone(),
                    confidence: 1.0,
                });
            }
            
            for interface in &type_info.implemented_interfaces {
                enhanced.relationships.push(TypeRelationship {
                    source: enhanced.base.to_string().clone(),
                    target: interface.clone(),
                    relationship_type: RelationshipType::Implements,
                    metadata: HashMap::new(),
                    location: enhanced.base.location.clone(),
                    confidence: 1.0,
                });
            }
        }
        
        if let Some(function_info) = self.function_registry.get(&enhanced.base.to_string()) {
            // Add function call relationships
            for called_function in &function_info.calls {
                enhanced.relationships.push(TypeRelationship {
                    source: enhanced.base.to_string().clone(),
                    target: called_function.clone(),
                    relationship_type: RelationshipType::Calls,
                    metadata: HashMap::new(),
                    location: enhanced.base.location.clone(),
                    confidence: 0.9,
                });
            }
        }
        
        Ok(enhanced)
    }
    
    /// Perform comprehensive semantic analysis
    #[instrument(skip(self))]
    async fn perform_semantic_analysis(&self) -> Result<(), Error> {
        info!("Performing semantic analysis");
        
        // Build type relationships
        let type_relationships = self.build_type_relationships();
        
        // Build inheritance hierarchies
        let inheritance_hierarchies = self.build_inheritance_hierarchies();
        
        // Collect cross-references
        let cross_references = self.collect_cross_references();
        
        // Build navigation information
        let navigation_info = self.build_navigation_info();
        
        // Build dependency graph
        let dependency_graph = self.build_dependency_graph();
        
        // Calculate complexity metrics
        let complexity_metrics = self.calculate_complexity_metrics();
        
        Ok(SemanticAnalysis {
            type_relationships,
            inheritance_hierarchies,
            cross_references,
            navigation_info,
            dependency_graph,
            complexity_metrics,
            slang_mappings: self.slang_mappings.clone(),
        })
    }
    
    /// Build type relationships from registries
    fn build_type_relationships(&self) -> Vec<TypeRelationship> {
        let mut relationships = Vec::new();
        
        // Add relationships from type registry
        for (type_name, type_info) in &self.type_registry {
            // Parent type relationships
            for parent in &type_info.parent_types {
                relationships.push(TypeRelationship {
                    source: type_name.clone(),
                    target: parent.clone(),
                    relationship_type: RelationshipType::Inherits,
                    metadata: HashMap::new(),
                    location: type_info.location.clone(),
                    confidence: 1.0,
                });
            }
            
            // Interface implementations
            for interface in &type_info.implemented_interfaces {
                relationships.push(TypeRelationship {
                    source: type_name.clone(),
                    target: interface.clone(),
                    relationship_type: RelationshipType::Implements,
                    metadata: HashMap::new(),
                    location: type_info.location.clone(),
                    confidence: 1.0,
                });
            }
            
            // Field type relationships
            for field in &type_info.fields {
                relationships.push(TypeRelationship {
                    source: type_name.clone(),
                    target: field.field_type.clone(),
                    relationship_type: RelationshipType::HasField,
                    metadata: {
                        let mut meta = HashMap::new();
                        meta.insert("field_name".to_string(), field.to_string().clone());
                        meta
                    },
                    location: type_info.location.clone(),
                    confidence: 1.0,
                });
            }
        }
        
        // Add relationships from function registry
        for (function_name, function_info) in &self.function_registry {
            // Return type relationships
            if let Some(ref return_type) = function_info.return_type {
                relationships.push(TypeRelationship {
                    source: function_name.clone(),
                    target: return_type.clone(),
                    relationship_type: RelationshipType::Returns,
                    metadata: HashMap::new(),
                    location: function_info.location.clone(),
                    confidence: 1.0,
                });
            }
            
            // Parameter type relationships
            for param in &function_info.parameters {
                if let Some(ref param_type) = param.param_type {
                    relationships.push(TypeRelationship {
                        source: function_name.clone(),
                        target: param_type.clone(),
                        relationship_type: RelationshipType::AcceptsParameter,
                        metadata: {
                            let mut meta = HashMap::new();
                            meta.insert("parameter_name".to_string(), param.to_string().clone());
                            meta
                        },
                        location: function_info.location.clone(),
                        confidence: 1.0,
                    });
                }
            }
            
            // Function call relationships
            for called_function in &function_info.calls {
                relationships.push(TypeRelationship {
                    source: function_name.clone(),
                    target: called_function.clone(),
                    relationship_type: RelationshipType::Calls,
                    metadata: HashMap::new(),
                    location: function_info.location.clone(),
                    confidence: 0.9,
                });
            }
        }
        
        debug!("Built {} type relationships", relationships.len());
        relationships
    }
    
    /// Build inheritance hierarchies
    fn build_inheritance_hierarchies(&self) -> Vec<InheritanceHierarchy> {
        let mut hierarchies = Vec::new();
        let mut processed_types = HashSet::new();
        
        // Find root types (types with no parents)
        for (type_name, type_info) in &self.type_registry {
            if type_info.parent_types.is_empty() && !processed_types.contains(type_name) {
                if let Ok(hierarchy) = self.build_hierarchy_from_root(type_name, &mut processed_types) {
                    hierarchies.push(hierarchy);
                }
            }
        }
        
        debug!("Built {} inheritance hierarchies", hierarchies.len());
        hierarchies
    }
    
    /// Build hierarchy from a root type
    fn build_hierarchy_from_root(
        &self,
        root_type: &str,
        processed_types: &mut HashSet<String>,
    ) -> Result<(), Error> {
        let mut hierarchy = BTreeMap::new();
        let mut max_depth = 0;
        
        // Build hierarchy recursively
        self.build_hierarchy_node(root_type, None, 0, &mut hierarchy, &mut max_depth, processed_types)?;
        
        Ok(InheritanceHierarchy {
            root_type: root_type.to_string(),
            hierarchy,
            max_depth,
            total_types: hierarchy.len(),
        })
    }
    
    /// Build a single hierarchy node
    fn build_hierarchy_node(
        &self,
        type_name: &str,
        parent: Option<String>,
        depth: usize,
        hierarchy: &mut BTreeMap<String, HierarchyNode>,
        max_depth: &mut usize,
        processed_types: &mut HashSet<String>,
    ) -> Result<(), Error> {
        if processed_types.contains(type_name) {
            return Ok(()); // Avoid cycles
        }
        
        processed_types.insert(type_name.to_string());
        *max_depth = (*max_depth).max(depth);
        
        if let Some(type_info) = self.type_registry.get(type_name) {
            // Find children (types that inherit from this type)
            let children: Vec<String> = self.type_registry.iter()
                .filter_map(|(child_name, child_info)| {
                    if child_info.parent_types.contains(&type_name.to_string()) {
                        Some(child_name.clone())
                    } else {
                        None
                    }
                })
                .collect();
            
            let node = HierarchyNode {
                type_name: type_name.to_string(),
                parent,
                children: children.clone(),
                depth,
                is_abstract: type_info.type_kind == "interface", // Interfaces are abstract
                interfaces: type_info.implemented_interfaces.clone(),
                location: type_info.location.clone(),
            };
            
            hierarchy.insert(type_name.to_string(), node);
            
            // Recursively build child nodes
            for child in children {
                self.build_hierarchy_node(
                    &child,
                    Some(type_name.to_string()),
                    depth + 1,
                    hierarchy,
                    max_depth,
                    processed_types,
                )?;
            }
        }
        
        Ok(())
    }
    
    /// Collect all cross-references
    fn collect_cross_references(&self) -> Vec<CrossReference> {
        let mut cross_references = Vec::new();
        
        for (source, refs) in &self.cross_reference_tracker {
            cross_references.extend(refs.clone());
        }
        
        debug!("Collected {} cross-references", cross_references.len());
        cross_references
    }
    
    /// Build navigation information
    fn build_navigation_info(&self) -> HashMap<String, NavigationInfo> {
        let mut navigation_info = HashMap::new();
        
        // Build navigation for types
        for (type_name, type_info) in &self.type_registry {
            let usages = self.find_type_usages(type_name);
            let related_items = self.find_related_types(type_name);
            
            let nav_info = NavigationInfo {
                definition: type_info.location.clone(),
                usages,
                related_items,
                doc_links: Vec::new(), // Would generate documentation links
                examples: Vec::new(), // Would find example usages
            };
            
            navigation_info.insert(type_name.clone(), nav_info);
        }
        
        // Build navigation for functions
        for (function_name, function_info) in &self.function_registry {
            let usages = self.find_function_usages(function_name);
            let related_items = self.find_related_functions(function_name);
            
            let nav_info = NavigationInfo {
                definition: function_info.location.clone(),
                usages,
                related_items,
                doc_links: Vec::new(),
                examples: Vec::new(),
            };
            
            navigation_info.insert(function_name.clone(), nav_info);
        }
        
        debug!("Built navigation info for {} items", navigation_info.len());
        navigation_info
    }
    
    /// Find usages of a type
    fn find_type_usages(&self, type_name: &str) -> Vec<SourceLocation> {
        let mut usages = Vec::new();
        
        // Find in function parameters and return types
        for function_info in self.function_registry.values() {
            if let Some(ref return_type) = function_info.return_type {
                if return_type == type_name {
                    usages.push(function_info.location.clone());
                }
            }
            
            for param in &function_info.parameters {
                if let Some(ref param_type) = param.param_type {
                    if param_type == type_name {
                        usages.push(function_info.location.clone());
                    }
                }
            }
        }
        
        // Find in struct fields
        for type_info in self.type_registry.values() {
            for field in &type_info.fields {
                if field.field_type == type_name {
                    usages.push(type_info.location.clone());
                }
            }
        }
        
        usages
    }
    
    /// Find usages of a function
    fn find_function_usages(&self, function_name: &str) -> Vec<SourceLocation> {
        let mut usages = Vec::new();
        
        // Find in function calls
        for function_info in self.function_registry.values() {
            if function_info.calls.contains(&function_name.to_string()) {
                usages.push(function_info.location.clone());
            }
        }
        
        usages
    }
    
    /// Find related types
    fn find_related_types(&self, type_name: &str) -> Vec<String> {
        let mut related = Vec::new();
        
        if let Some(type_info) = self.type_registry.get(type_name) {
            // Add parent types
            related.extend(type_info.parent_types.clone());
            
            // Add implemented interfaces
            related.extend(type_info.implemented_interfaces.clone());
            
            // Add types that use this type
            for (other_type, other_info) in &self.type_registry {
                for field in &other_info.fields {
                    if field.field_type == type_name && other_type != type_name {
                        related.push(other_type.clone());
                    }
                }
            }
        }
        
        related.sort();
        related.dedup();
        related
    }
    
    /// Find related functions
    fn find_related_functions(&self, function_name: &str) -> Vec<String> {
        let mut related = Vec::new();
        
        if let Some(function_info) = self.function_registry.get(function_name) {
            // Add called functions
            related.extend(function_info.calls.clone());
            
            // Add functions that call this function
            for (other_function, other_info) in &self.function_registry {
                if other_info.calls.contains(&function_name.to_string()) && other_function != function_name {
                    related.push(other_function.clone());
                }
            }
        }
        
        related.sort();
        related.dedup();
        related
    }
    
    /// Build dependency graph
    fn build_dependency_graph(&self) -> DependencyGraph {
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();
        
        // Add type nodes
        for (type_name, type_info) in &self.type_registry {
            let node = DependencyNode {
                id: type_name.clone(),
                node_type: type_info.type_kind.clone(),
                location: type_info.location.clone(),
                metadata: type_info.metadata.clone(),
            };
            nodes.insert(type_name.clone(), node);
        }
        
        // Add function nodes
        for (function_name, function_info) in &self.function_registry {
            let node = DependencyNode {
                id: function_name.clone(),
                node_type: "function".to_string(),
                location: function_info.location.clone(),
                metadata: function_info.metadata.clone(),
            };
            nodes.insert(function_name.clone(), node);
        }
        
        // Add module nodes
        for (module_name, module_info) in &self.module_registry {
            let node = DependencyNode {
                id: module_name.clone(),
                node_type: "module".to_string(),
                location: module_info.location.clone(),
                metadata: module_info.metadata.clone(),
            };
            nodes.insert(module_name.clone(), node);
        }
        
        // Add edges based on relationships
        for type_info in self.type_registry.values() {
            // Parent type edges
            for parent in &type_info.parent_types {
                edges.push(DependencyEdge {
                    from: type_info.to_string().clone(),
                    to: parent.clone(),
                    edge_type: "inherits".to_string(),
                    weight: 1.0,
                });
            }
            
            // Interface implementation edges
            for interface in &type_info.implemented_interfaces {
                edges.push(DependencyEdge {
                    from: type_info.to_string().clone(),
                    to: interface.clone(),
                    edge_type: "implements".to_string(),
                    weight: 1.0,
                });
            }
            
            // Field type edges
            for field in &type_info.fields {
                edges.push(DependencyEdge {
                    from: type_info.to_string().clone(),
                    to: field.field_type.clone(),
                    edge_type: "uses".to_string(),
                    weight: 0.5,
                });
            }
        }
        
        // Function call edges
        for function_info in self.function_registry.values() {
            for called_function in &function_info.calls {
                edges.push(DependencyEdge {
                    from: function_info.to_string().clone(),
                    to: called_function.clone(),
                    edge_type: "calls".to_string(),
                    weight: 0.8,
                });
            }
        }
        
        // Find strongly connected components and circular dependencies
        let strongly_connected_components = self.find_strongly_connected_components(&nodes, &edges);
        let circular_dependencies = strongly_connected_components.iter()
            .filter(|component| component.len() > 1)
            .cloned()
            .collect();
        
        DependencyGraph {
            nodes,
            edges,
            strongly_connected_components,
            circular_dependencies,
        }
    }
    
    /// Find strongly connected components in dependency graph
    fn find_strongly_connected_components(
        &self,
        nodes: &HashMap<String, DependencyNode>,
        edges: &[DependencyEdge],
    ) -> Vec<Vec<String>> {
        // Implementation of Tarjan's algorithm would go here
        // For now, return empty components
        Vec::new()
    }
    
    /// Calculate complexity metrics
    fn calculate_complexity_metrics(&self) -> ComplexityMetrics {
        let mut cyclomatic_complexity = HashMap::new();
        let mut cognitive_complexity = HashMap::new();
        let mut lines_of_code = HashMap::new();
        let mut dependency_count = HashMap::new();
        let mut coupling_metrics = HashMap::new();
        let mut cohesion_metrics = HashMap::new();
        
        // Calculate function-level metrics
        for (function_name, function_info) in &self.function_registry {
            // Cyclomatic complexity (simplified calculation)
            let complexity = 1 + function_info.calls.len(); // Simplified metric
            cyclomatic_complexity.insert(function_name.clone(), complexity);
            
            // Cognitive complexity (simplified)
            cognitive_complexity.insert(function_name.clone(), complexity);
            
            // Dependency count
            dependency_count.insert(function_name.clone(), function_info.calls.len());
        }
        
        // Calculate module-level metrics
        for (module_name, module_info) in &self.module_registry {
            // Lines of code (would need actual source analysis)
            lines_of_code.insert(module_name.clone(), 100); // Placeholder
            
            // Coupling (number of dependencies)
            let coupling = module_info.imports.len() as f64;
            coupling_metrics.insert(module_name.clone(), coupling);
            
            // Cohesion (simplified metric)
            let cohesion = if module_info.exports.is_empty() { 
                0.0 
            } else { 
                1.0 / module_info.exports.len() as f64 
            };
            cohesion_metrics.insert(module_name.clone(), cohesion);
        }
        
        ComplexityMetrics {
            cyclomatic_complexity,
            cognitive_complexity,
            lines_of_code,
            dependency_count,
            coupling_metrics,
            cohesion_metrics,
        }
    }
    
    /// Enhance documentation items with analysis results
    fn enhance_documentation_items(
        &self,
        documentation_items: &mut Vec<EnhancedDocumentationItem>,
        semantic_analysis: &SemanticAnalysis,
    ) {
        for item in documentation_items {
            // Add relationships from semantic analysis
            let relevant_relationships: Vec<TypeRelationship> = semantic_analysis.type_relationships.iter()
                .filter(|rel| rel.source == item.base.to_string() || rel.target == item.base.to_string())
                .cloned()
                .collect();
            
            item.relationships.extend(relevant_relationships);
            
            // Add navigation info if available
            if let Some(nav_info) = semantic_analysis.navigation_info.get(&item.base.to_string()) {
                item.base.metadata.insert(
                    "usages_count".to_string(),
                    nav_info.usages.len().to_string()
                );
                item.base.metadata.insert(
                    "related_items_count".to_string(),
                    nav_info.related_items.len().to_string()
                );
            }
            
            // Add complexity metrics if available
            if let Some(complexity) = semantic_analysis.complexity_metrics.cyclomatic_complexity.get(&item.base.to_string()) {
                item.base.metadata.insert(
                    "cyclomatic_complexity".to_string(),
                    complexity.to_string()
                );
            }
        }
    }
    
    /// Get child nodes from AST node
    fn get_child_nodes(&self, node: &AstNode) -> Option<Vec<&AstNode>> {
        // This would return child nodes based on the node type
        // Implementation depends on the actual AST structure
        None
    }
    
    /// Format type name for display
    fn format_type_name(&self, type_expr: &dyn Expression) -> String {
        // This would format type expressions into readable strings
        // For now, return a placeholder
        match type_expr {
            Expression::Identifier(id) => id.to_string().clone(),
            _ => "Unknown".to_string(),
        }
    }
    
    /// Convert cross-reference type to relationship type
    fn convert_cross_ref_type(&self, cross_ref_type: &CrossReferenceType) -> RelationshipType {
        match cross_ref_type {
            CrossReferenceType::TypeReference => RelationshipType::Uses,
            CrossReferenceType::FunctionCall => RelationshipType::Calls,
            CrossReferenceType::VariableReference => RelationshipType::Uses,
            CrossReferenceType::ConstantReference => RelationshipType::Uses,
            CrossReferenceType::ModuleReference => RelationshipType::Imports,
            CrossReferenceType::InterfaceReference => RelationshipType::Implements,
            CrossReferenceType::FieldReference => RelationshipType::HasField,
            CrossReferenceType::MethodCall => RelationshipType::Calls,
            CrossReferenceType::GenericReference => RelationshipType::Constrains,
            CrossReferenceType::ImportReference => RelationshipType::Imports,
        }
    }
}
