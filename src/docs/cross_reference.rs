/// Advanced Cross-Reference System
/// 
/// Provides sophisticated cross-referencing capabilities with semantic analysis,
/// dependency visualization, and intelligent relationship detection.

use crate::error::{Error, SourceLocation};
use crate::docs::generator::{ExtractedDocumentation, DocumentationItem, ItemKind};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::{Parser, ParsedProgram};
use crate::ast::{AstNode, Statement, Expression};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::fs;

/// Advanced cross-reference analyzer
#[derive(Debug)]
pub struct CrossReferenceAnalyzer {
    /// Configuration for cross-reference analysis
    config: CrossReferenceConfig,
    /// Semantic analysis engine
    semantic_analyzer: SemanticAnalyzer,
    /// Dependency graph builder
    dependency_graph: DependencyGraph,
    /// Type relationship tracker
    type_relationships: TypeRelationshipMap,
    /// Reference index
    reference_index: ReferenceIndex,
}

/// Configuration for cross-reference analysis
#[derive(Debug, Clone)]
pub struct CrossReferenceConfig {
    /// Enable semantic analysis for better linking
    pub enable_semantic_analysis: bool,
    /// Generate dependency graph visualization
    pub generate_dependency_graph: bool,
    /// Maximum depth for relationship discovery
    pub max_relationship_depth: usize,
    /// Enable type relationship analysis
    pub enable_type_relationships: bool,
    /// Include internal/private references
    pub include_private_refs: bool,
    /// Generate concept suggestions
    pub generate_concept_suggestions: bool,
    /// Cross-reference output formats
    pub output_formats: HashSet<CrossRefFormat>,
}

impl Default for CrossReferenceConfig {
    fn default() -> Self {
        let mut formats = HashSet::new();
        formats.insert(CrossRefFormat::Html);
        formats.insert(CrossRefFormat::Json);
        formats.insert(CrossRefFormat::GraphViz);

        Self {
            enable_semantic_analysis: true,
            generate_dependency_graph: true,
            max_relationship_depth: 3,
            enable_type_relationships: true,
            include_private_refs: false,
            generate_concept_suggestions: true,
            output_formats: formats,
        }
    }
}

/// Cross-reference output formats
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CrossRefFormat {
    Html,
    Json,
    GraphViz,
    Mermaid,
    PlantUml,
    Cypher,
}

/// Semantic analysis engine
#[derive(Debug)]
pub struct SemanticAnalyzer {
    /// Symbol table for semantic analysis
    symbol_table: SymbolTable,
    /// Type information database
    type_database: TypeDatabase,
    /// Scope analysis results
    scope_analysis: ScopeAnalysis,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self {
            symbol_table: SymbolTable::default(),
            type_database: TypeDatabase::default(),
            scope_analysis: ScopeAnalysis::default(),
        }
    }
}

/// Symbol table for tracking definitions and uses
#[derive(Debug, Default)]
pub struct SymbolTable {
    /// Symbols by name
    symbols: HashMap<String, Symbol>,
    /// Scopes hierarchy
    scopes: Vec<Scope>,
    /// Current scope index
    current_scope: usize,
}

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub definition_location: SourceLocation,
    pub uses: Vec<SourceLocation>,
    pub scope_id: usize,
    pub visibility: Visibility,
    pub documentation: Option<String>,
}

/// Types of symbols
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    Function,
    Variable,
    Constant,
    Type,
    Interface,
    Module,
    Field,
    Parameter,
    Generic,
}

/// Symbol visibility
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// Scope information
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: usize,
    pub parent: Option<usize>,
    pub scope_type: ScopeType,
    pub symbols: HashSet<String>,
    pub location: SourceLocation,
}

/// Types of scopes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeType {
    Global,
    Module,
    Function,
    Block,
    Interface,
    Struct,
}

/// Type information database
#[derive(Debug, Default)]
pub struct TypeDatabase {
    /// Type definitions
    types: HashMap<String, TypeInfo>,
    /// Type relationships
    relationships: HashMap<String, Vec<TypeRelationship>>,
    /// Generic type constraints
    constraints: HashMap<String, Vec<TypeConstraint>>,
}

/// Type information
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub type_kind: TypeKind,
    pub definition_location: SourceLocation,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub generic_parameters: Vec<String>,
    pub documentation: Option<String>,
}

/// Kinds of types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Struct,
    Interface,
    Enum,
    Alias,
    Generic,
    Primitive,
    Function,
}

/// Field information
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub location: SourceLocation,
    pub visibility: Visibility,
    pub documentation: Option<String>,
}

/// Method information
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: Option<String>,
    pub parameters: Vec<ParameterInfo>,
    pub location: SourceLocation,
    pub visibility: Visibility,
    pub documentation: Option<String>,
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: String,
    pub is_optional: bool,
    pub default_value: Option<String>,
}

/// Type relationships
#[derive(Debug, Clone)]
pub struct TypeRelationship {
    pub relationship_type: RelationshipType,
    pub target_type: String,
    pub location: SourceLocation,
    pub strength: f64, // 0.0 to 1.0
}

/// Types of relationships between types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipType {
    Implements,
    Extends,
    Uses,
    Contains,
    References,
    Similar,
    Conflicting,
}

/// Type constraints
#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub constraint_type: ConstraintType,
    pub target: String,
    pub location: SourceLocation,
}

/// Types of constraints
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintType {
    Implements,
    Extends,
    Equals,
    Compatible,
}

/// Scope analysis results
#[derive(Debug, Default)]
pub struct ScopeAnalysis {
    /// Scope hierarchy
    scope_tree: Vec<ScopeNode>,
    /// Variable lifetime analysis
    lifetimes: HashMap<String, VariableLifetime>,
    /// Shadowing analysis
    shadowing: Vec<ShadowingIssue>,
}

/// Scope tree node
#[derive(Debug, Clone)]
pub struct ScopeNode {
    pub scope: Scope,
    pub children: Vec<usize>,
    pub depth: usize,
}

/// Variable lifetime information
#[derive(Debug, Clone)]
pub struct VariableLifetime {
    pub variable_name: String,
    pub definition: SourceLocation,
    pub first_use: Option<SourceLocation>,
    pub last_use: Option<SourceLocation>,
    pub scope_id: usize,
}

/// Shadowing issue
#[derive(Debug, Clone)]
pub struct ShadowingIssue {
    pub shadowed_symbol: String,
    pub original_location: SourceLocation,
    pub shadowing_location: SourceLocation,
    pub severity: ShadowingSeverity,
}

/// Shadowing severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShadowingSeverity {
    Warning,
    Error,
    Info,
}

/// Dependency graph
#[derive(Debug, Default)]
pub struct DependencyGraph {
    /// Nodes in the dependency graph
    nodes: HashMap<String, DependencyNode>,
    /// Edges representing dependencies
    edges: Vec<DependencyEdge>,
    /// Strongly connected components
    components: Vec<Vec<String>>,
    /// Topological ordering
    topological_order: Vec<String>,
}

/// Dependency graph node
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub id: String,
    pub node_type: DependencyNodeType,
    pub location: SourceLocation,
    pub metadata: HashMap<String, String>,
}

/// Types of dependency nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyNodeType {
    Module,
    Function,
    Type,
    Interface,
    External,
}

/// Dependency graph edge
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub edge_type: DependencyType,
    pub weight: f64,
    pub location: SourceLocation,
}

/// Types of dependencies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyType {
    Import,
    FunctionCall,
    TypeUsage,
    Inheritance,
    Implementation,
    Reference,
}

/// Type relationship map
#[derive(Debug, Default)]
pub struct TypeRelationshipMap {
    /// Direct relationships
    direct_relationships: HashMap<String, Vec<TypeRelationship>>,
    /// Transitive relationships
    transitive_relationships: HashMap<String, Vec<TypeRelationship>>,
    /// Similarity scores
    similarity_matrix: HashMap<(String, String), f64>,
}

/// Reference index for fast lookups
#[derive(Debug, Default)]
pub struct ReferenceIndex {
    /// Forward references (from -> to)
    forward_refs: HashMap<String, Vec<Reference>>,
    /// Backward references (to -> from)
    backward_refs: HashMap<String, Vec<Reference>>,
    /// Reference by type
    by_type: HashMap<ReferenceType, Vec<Reference>>,
    /// Reference by location
    by_location: HashMap<PathBuf, Vec<Reference>>,
}

/// Reference information
#[derive(Debug, Clone)]
pub struct Reference {
    pub from: String,
    pub to: String,
    pub reference_type: ReferenceType,
    pub location: SourceLocation,
    pub context: ReferenceContext,
    pub confidence: f64, // 0.0 to 1.0
}

/// Types of references
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReferenceType {
    Definition,
    Usage,
    Call,
    TypeAnnotation,
    Inheritance,
    Implementation,
    Import,
    Documentation,
}

/// Reference context
#[derive(Debug, Clone)]
pub struct ReferenceContext {
    pub scope: String,
    pub surrounding_code: String,
    pub line_context: String,
}

/// Cross-reference analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReferenceResult {
    /// Total references found
    pub total_references: usize,
    /// References by type
    pub references_by_type: HashMap<String, usize>,
    /// Dependency graph summary
    pub dependency_summary: DependencySummary,
    /// Type relationship summary
    pub type_relationship_summary: TypeRelationshipSummary,
    /// Concept suggestions
    pub concept_suggestions: Vec<ConceptSuggestion>,
    /// Analysis metadata
    pub analysis_metadata: AnalysisMetadata,
}

/// Dependency graph summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySummary {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub strongly_connected_components: usize,
    pub cyclic_dependencies: usize,
    pub max_depth: usize,
    pub complexity_score: f64,
}

/// Type relationship summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRelationshipSummary {
    pub total_types: usize,
    pub inheritance_chains: usize,
    pub interface_implementations: usize,
    pub type_usage_count: usize,
    pub complexity_metrics: TypeComplexityMetrics,
}

/// Type complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeComplexityMetrics {
    pub average_inheritance_depth: f64,
    pub average_interface_implementations: f64,
    pub coupling_coefficient: f64,
    pub cohesion_score: f64,
}

/// Concept suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptSuggestion {
    pub suggestion_type: SuggestionType,
    pub primary_concept: String,
    pub related_concepts: Vec<String>,
    pub confidence: f64,
    pub description: String,
    pub examples: Vec<String>,
}

/// Types of concept suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    SimilarTypes,
    RelatedFunctions,
    UsagePatterns,
    DesignPatterns,
    Refactoring,
    Documentation,
}

/// Analysis metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub analysis_time: String,
    pub files_analyzed: usize,
    pub lines_of_code: usize,
    pub analysis_duration: f64, // seconds
    pub config_used: String,
}

impl CrossReferenceAnalyzer {
    /// Create a new cross-reference analyzer
    pub fn new(config: CrossReferenceConfig) -> Self {
        Self {
            config,
            semantic_analyzer: SemanticAnalyzer::default(),
            dependency_graph: DependencyGraph::default(),
            type_relationships: TypeRelationshipMap::default(),
            reference_index: ReferenceIndex::default(),
        }
    }

    /// Analyze cross-references in documentation
    pub fn analyze_cross_references(
        &mut self,
        documentation: &ExtractedDocumentation,
        source_files: &[PathBuf],
    ) -> Result<CrossReferenceResult, Error> {
        // Phase 1: Build semantic model
        if self.config.enable_semantic_analysis {
            self.build_semantic_model(source_files)?;
        }

        // Phase 2: Analyze dependencies
        if self.config.generate_dependency_graph {
            self.build_dependency_graph(documentation, source_files)?;
        }

        // Phase 3: Analyze type relationships
        if self.config.enable_type_relationships {
            self.analyze_type_relationships(documentation)?;
        }

        // Phase 4: Build reference index
        self.build_reference_index(documentation)?;

        // Phase 5: Generate concept suggestions
        let concept_suggestions = if self.config.generate_concept_suggestions {
            self.generate_concept_suggestions()?
        } else {
            Vec::new()
        };

        // Phase 6: Compile results
        let result = CrossReferenceResult {
            total_references: self.count_total_references(),
            references_by_type: self.count_references_by_type(),
            dependency_summary: self.generate_dependency_summary(),
            type_relationship_summary: self.generate_type_relationship_summary(),
            concept_suggestions,
            analysis_metadata: self.generate_analysis_metadata(source_files.len()),
        };

        Ok(result)
    }

    /// Build semantic model from source files
    fn build_semantic_model(&mut self, source_files: &[PathBuf]) -> Result<(), Error> {
        for file_path in source_files {
            let content = fs::read_to_string(file_path)
                .map_err(|e| Error::SystemError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

            // Parse the file
            let mut lexer = Lexer::new(&content);
            let tokens = lexer.tokenize()
                .map_err(|e| Error::SystemError(format!("Failed to tokenize {}: {:?}", file_path.display(), e)))?;

            let mut parser = Parser::new(tokens);
            let program = parser.parse()
                .map_err(|e| Error::SystemError(format!("Failed to parse {}: {:?}", file_path.display(), e)))?;

            // Analyze semantics
            self.analyze_program_semantics(&program, file_path)?;
        }

        // Build scope hierarchy
        self.build_scope_hierarchy()?;

        // Analyze variable lifetimes
        self.analyze_variable_lifetimes()?;

        // Detect shadowing issues
        self.detect_shadowing_issues()?;

        Ok(())
    }

    /// Analyze semantics of a parsed program
    fn analyze_program_semantics(&mut self, program: &ParsedProgram, file_path: &Path) -> Result<(), Error> {
        // Enter global scope
        self.semantic_analyzer.symbol_table.enter_scope(ScopeType::Global, SourceLocation {
            line: 1,
            column: 1,
            file: Some(file_path.to_path_buf()),
        });

        // Process statements
        for statement in &program.statements {
            self.analyze_statement_semantics(statement)?;
        }

        // Exit global scope
        self.semantic_analyzer.symbol_table.exit_scope();

        Ok(())
    }

    /// Analyze semantics of a statement
    fn analyze_statement_semantics(&mut self, statement: &dyn Statement) -> Result<(), Error> {
        match statement {
            Statement::FunctionDeclaration { name, location, .. } => {
                self.semantic_analyzer.symbol_table.add_symbol(Symbol {
                    name: name.clone(),
                    symbol_type: SymbolType::Function,
                    definition_location: location.clone(),
                    uses: Vec::new(),
                    scope_id: self.semantic_analyzer.symbol_table.current_scope,
                    visibility: Visibility::Public, // Default assumption
                    documentation: None,
                });
            }
            Statement::StructDeclaration { name, location, .. } => {
                self.semantic_analyzer.symbol_table.add_symbol(Symbol {
                    name: name.clone(),
                    symbol_type: SymbolType::Type,
                    definition_location: location.clone(),
                    uses: Vec::new(),
                    scope_id: self.semantic_analyzer.symbol_table.current_scope,
                    visibility: Visibility::Public,
                    documentation: None,
                });

                // Add type information
                self.semantic_analyzer.type_database.add_type(TypeInfo {
                    name: name.clone(),
                    type_kind: TypeKind::Struct,
                    definition_location: location.clone(),
                    fields: Vec::new(), // Would be extracted from actual struct definition
                    methods: Vec::new(),
                    generic_parameters: Vec::new(),
                    documentation: None,
                });
            }
            Statement::InterfaceDeclaration { name, location, .. } => {
                self.semantic_analyzer.symbol_table.add_symbol(Symbol {
                    name: name.clone(),
                    symbol_type: SymbolType::Interface,
                    definition_location: location.clone(),
                    uses: Vec::new(),
                    scope_id: self.semantic_analyzer.symbol_table.current_scope,
                    visibility: Visibility::Public,
                    documentation: None,
                });

                self.semantic_analyzer.type_database.add_type(TypeInfo {
                    name: name.clone(),
                    type_kind: TypeKind::Interface,
                    definition_location: location.clone(),
                    fields: Vec::new(),
                    methods: Vec::new(),
                    generic_parameters: Vec::new(),
                    documentation: None,
                });
            }
            Statement::VariableDeclaration { name, location, .. } => {
                self.semantic_analyzer.symbol_table.add_symbol(Symbol {
                    name: name.clone(),
                    symbol_type: SymbolType::Variable,
                    definition_location: location.clone(),
                    uses: Vec::new(),
                    scope_id: self.semantic_analyzer.symbol_table.current_scope,
                    visibility: Visibility::Private,
                    documentation: None,
                });
            }
            _ => {
                // Handle other statement types
            }
        }

        Ok(())
    }

    /// Build dependency graph
    fn build_dependency_graph(&mut self, documentation: &ExtractedDocumentation, source_files: &[PathBuf]) -> Result<(), Error> {
        // Add nodes for each documentation item
        for item in &documentation.items {
            let node_type = match item.kind {
                ItemKind::Function => DependencyNodeType::Function,
                ItemKind::Struct | ItemKind::Enum | ItemKind::Type => DependencyNodeType::Type,
                ItemKind::Interface => DependencyNodeType::Interface,
                ItemKind::Module => DependencyNodeType::Module,
                _ => DependencyNodeType::Type,
            };

            self.dependency_graph.add_node(DependencyNode {
                id: item.name.clone(),
                node_type,
                location: item.source_info.clone(),
                metadata: HashMap::new(),
            });
        }

        // Analyze dependencies from source files
        for file_path in source_files {
            self.analyze_file_dependencies(file_path)?;
        }

        // Compute strongly connected components
        self.compute_strongly_connected_components();

        // Compute topological ordering
        self.compute_topological_ordering();

        Ok(())
    }

    /// Analyze dependencies in a file
    fn analyze_file_dependencies(&mut self, file_path: &Path) -> Result<(), Error> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| Error::SystemError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

        // Simple dependency extraction (would be more sophisticated in practice)
        let lines: Vec<&str> = content.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Look for import statements
            if trimmed.starts_with("import ") {
                if let Some(imported_module) = self.extract_import_target(trimmed) {
                    let location = SourceLocation {
                        line: line_num + 1,
                        column: 1,
                        file: Some(file_path.to_path_buf()),
                    };

                    self.dependency_graph.add_edge(DependencyEdge {
                        from: file_path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                        to: imported_module,
                        edge_type: DependencyType::Import,
                        weight: 1.0,
                        location,
                    });
                }
            }

            // Look for function calls
            if let Some(function_call) = self.extract_function_call(trimmed) {
                let location = SourceLocation {
                    line: line_num + 1,
                    column: 1,
                    file: Some(file_path.to_path_buf()),
                };

                self.dependency_graph.add_edge(DependencyEdge {
                    from: file_path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                    to: function_call,
                    edge_type: DependencyType::FunctionCall,
                    weight: 0.5,
                    location,
                });
            }
        }

        Ok(())
    }

    /// Extract import target from import statement
    fn extract_import_target(&self, import_line: &str) -> Option<String> {
        // Simple extraction - would be more sophisticated in practice
        if let Some(quote_start) = import_line.find('"') {
            if let Some(quote_end) = import_line[quote_start + 1..].find('"') {
                return Some(import_line[quote_start + 1..quote_start + 1 + quote_end].to_string());
            }
        }
        None
    }

    /// Extract function call from a line
    fn extract_function_call(&self, line: &str) -> Option<String> {
        // Simple pattern matching for function calls
        if let Some(paren_pos) = line.find('(') {
            let before_paren = &line[..paren_pos];
            if let Some(space_pos) = before_paren.rfind(' ') {
                let function_name = before_paren[space_pos + 1..].trim();
                if function_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(function_name.to_string());
                }
            } else if before_paren.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Some(before_paren.to_string());
            }
        }
        None
    }

    /// Analyze type relationships
    fn analyze_type_relationships(&mut self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        // Build similarity matrix
        self.build_type_similarity_matrix(documentation)?;

        // Detect inheritance relationships
        self.detect_inheritance_relationships(documentation)?;

        // Detect interface implementations
        self.detect_interface_implementations(documentation)?;

        // Compute transitive relationships
        self.compute_transitive_relationships()?;

        Ok(())
    }

    /// Build type similarity matrix
    fn build_type_similarity_matrix(&mut self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        let type_items: Vec<&DocumentationItem> = documentation.items.iter()
            .filter(|item| matches!(item.kind, ItemKind::Struct | ItemKind::Interface | ItemKind::Type))
            .collect();

        for i in 0..type_items.len() {
            for j in i + 1..type_items.len() {
                let similarity = self.calculate_type_similarity(type_items[i], type_items[j]);
                let key = (type_items[i].name.clone(), type_items[j].name.clone());
                self.type_relationships.similarity_matrix.insert(key, similarity);
            }
        }

        Ok(())
    }

    /// Calculate similarity between two types
    fn calculate_type_similarity(&self, type1: &DocumentationItem, type2: &DocumentationItem) -> f64 {
        let mut similarity = 0.0;
        let mut factors = 0;

        // Name similarity
        let name_similarity = self.calculate_string_similarity(&type1.name, &type2.name);
        similarity += name_similarity * 0.3;
        factors += 1;

        // Description similarity
        let desc_similarity = self.calculate_string_similarity(&type1.description, &type2.description);
        similarity += desc_similarity * 0.4;
        factors += 1;

        // Parameter similarity (for functions)
        if !type1.parameters.is_empty() && !type2.parameters.is_empty() {
            let param_similarity = self.calculate_parameter_similarity(&type1.parameters, &type2.parameters);
            similarity += param_similarity * 0.3;
            factors += 1;
        }

        if factors > 0 {
            similarity / factors as f64
        } else {
            0.0
        }
    }

    /// Calculate string similarity using simple algorithm
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }

        let s1_words: HashSet<&str> = s1.split_whitespace().collect();
        let s2_words: HashSet<&str> = s2.split_whitespace().collect();

        let intersection = s1_words.intersection(&s2_words).count();
        let union = s1_words.union(&s2_words).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Calculate parameter similarity
    fn calculate_parameter_similarity(&self, params1: &[crate::docs::generator::Parameter], params2: &[crate::docs::generator::Parameter]) -> f64 {
        if params1.is_empty() && params2.is_empty() {
            return 1.0;
        }

        let mut matches = 0;
        for p1 in params1 {
            for p2 in params2 {
                if p1.name == p2.name {
                    matches += 1;
                    break;
                }
            }
        }

        matches as f64 / params1.len().max(params2.len()) as f64
    }

    /// Detect inheritance relationships
    fn detect_inheritance_relationships(&mut self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        // Simple pattern matching for inheritance keywords
        for item in &documentation.items {
            if item.description.contains("extends") || item.description.contains("inherits") {
                // Extract inheritance information (simplified)
                if let Some(parent_type) = self.extract_parent_type(&item.description) {
                    self.type_relationships.direct_relationships
                        .entry(item.name.clone())
                        .or_insert_with(Vec::new)
                        .push(TypeRelationship {
                            relationship_type: RelationshipType::Extends,
                            target_type: parent_type,
                            location: item.source_info.clone(),
                            strength: 0.9,
                        });
                }
            }
        }

        Ok(())
    }

    /// Extract parent type from description
    fn extract_parent_type(&self, description: &str) -> Option<String> {
        // Simple pattern matching - would be more sophisticated in practice
        if let Some(extends_pos) = description.find("extends") {
            let after_extends = &description[extends_pos + 7..];
            if let Some(word_end) = after_extends.find(|c: char| c.is_whitespace() || c == ',' || c == '.') {
                let parent_type = after_extends[..word_end].trim();
                if !parent_type.is_empty() {
                    return Some(parent_type.to_string());
                }
            }
        }
        None
    }

    /// Detect interface implementations
    fn detect_interface_implementations(&mut self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        for item in &documentation.items {
            if item.description.contains("implements") {
                if let Some(interface_type) = self.extract_interface_type(&item.description) {
                    self.type_relationships.direct_relationships
                        .entry(item.name.clone())
                        .or_insert_with(Vec::new)
                        .push(TypeRelationship {
                            relationship_type: RelationshipType::Implements,
                            target_type: interface_type,
                            location: item.source_info.clone(),
                            strength: 0.8,
                        });
                }
            }
        }

        Ok(())
    }

    /// Extract interface type from description
    fn extract_interface_type(&self, description: &str) -> Option<String> {
        if let Some(implements_pos) = description.find("implements") {
            let after_implements = &description[implements_pos + 10..];
            if let Some(word_end) = after_implements.find(|c: char| c.is_whitespace() || c == ',' || c == '.') {
                let interface_type = after_implements[..word_end].trim();
                if !interface_type.is_empty() {
                    return Some(interface_type.to_string());
                }
            }
        }
        None
    }

    /// Compute transitive relationships
    fn compute_transitive_relationships(&mut self) -> Result<(), Error> {
        // Use Floyd-Warshall-like algorithm to compute transitive closure
        let types: Vec<String> = self.type_relationships.direct_relationships.keys().cloned().collect();

        for k in &types {
            for i in &types {
                for j in &types {
                    if let Some(i_to_k) = self.get_relationship_strength(i, k) {
                        if let Some(k_to_j) = self.get_relationship_strength(k, j) {
                            let transitive_strength = (i_to_k * k_to_j * 0.8).min(1.0);
                            
                            if transitive_strength > 0.3 {
                                self.type_relationships.transitive_relationships
                                    .entry(i.clone())
                                    .or_insert_with(Vec::new)
                                    .push(TypeRelationship {
                                        relationship_type: RelationshipType::References,
                                        target_type: j.clone(),
                                        location: SourceLocation { line: 0, column: 0, file: None },
                                        strength: transitive_strength,
                                    });
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get relationship strength between two types
    fn get_relationship_strength(&self, from: &str, to: &str) -> Option<f64> {
        if let Some(relationships) = self.type_relationships.direct_relationships.get(from) {
            for rel in relationships {
                if rel.target_type == to {
                    return Some(rel.strength);
                }
            }
        }
        None
    }

    /// Build reference index
    fn build_reference_index(&mut self, documentation: &ExtractedDocumentation) -> Result<(), Error> {
        for item in &documentation.items {
            // Create definition reference
            let def_ref = Reference {
                from: item.name.clone(),
                to: item.name.clone(),
                reference_type: ReferenceType::Definition,
                location: item.source_info.clone(),
                context: ReferenceContext {
                    scope: "global".to_string(),
                    surrounding_code: item.description.clone(),
                    line_context: item.description.clone(),
                },
                confidence: 1.0,
            };

            self.reference_index.add_reference(def_ref);

            // Find usage references in descriptions and examples
            for other_item in &documentation.items {
                if other_item.name != item.name {
                    if other_item.description.contains(&item.name) {
                        let usage_ref = Reference {
                            from: other_item.name.clone(),
                            to: item.name.clone(),
                            reference_type: ReferenceType::Usage,
                            location: other_item.source_info.clone(),
                            context: ReferenceContext {
                                scope: "global".to_string(),
                                surrounding_code: other_item.description.clone(),
                                line_context: other_item.description.clone(),
                            },
                            confidence: 0.7,
                        };

                        self.reference_index.add_reference(usage_ref);
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate concept suggestions
    fn generate_concept_suggestions(&self) -> Result<Vec<ConceptSuggestion>, Error> {
        let mut suggestions = Vec::new();

        // Similar types suggestions
        for ((type1, type2), similarity) in &self.type_relationships.similarity_matrix {
            if *similarity > 0.6 {
                suggestions.push(ConceptSuggestion {
                    suggestion_type: SuggestionType::SimilarTypes,
                    primary_concept: type1.clone(),
                    related_concepts: vec![type2.clone()],
                    confidence: *similarity,
                    description: format!("Types '{}' and '{}' share similar characteristics", type1, type2),
                    examples: vec![
                        format!("Both {} and {} have similar structure", type1, type2),
                    ],
                });
            }
        }

        // Related functions suggestions based on naming patterns
        suggestions.extend(self.generate_related_function_suggestions()?);

        // Usage pattern suggestions
        suggestions.extend(self.generate_usage_pattern_suggestions()?);

        Ok(suggestions)
    }

    /// Generate related function suggestions
    fn generate_related_function_suggestions(&self) -> Result<Vec<ConceptSuggestion>, Error> {
        let mut suggestions = Vec::new();

        // Group functions by naming patterns
        let mut function_groups: HashMap<String, Vec<String>> = HashMap::new();
        
        for symbol_name in self.semantic_analyzer.symbol_table.symbols.keys() {
            if let Some(symbol) = self.semantic_analyzer.symbol_table.symbols.get(symbol_name) {
                if symbol.symbol_type == SymbolType::Function {
                    let prefix = self.extract_function_prefix(&symbol.name);
                    function_groups.entry(prefix).or_insert_with(Vec::new).push(symbol.name.clone());
                }
            }
        }

        for (prefix, functions) in function_groups {
            if functions.len() > 1 {
                for function in &functions {
                    let related: Vec<String> = functions.iter()
                        .filter(|f| *f != function)
                        .cloned()
                        .collect();

                    if !related.is_empty() {
                        suggestions.push(ConceptSuggestion {
                            suggestion_type: SuggestionType::RelatedFunctions,
                            primary_concept: function.clone(),
                            related_concepts: related,
                            confidence: 0.8,
                            description: format!("Functions with '{}' prefix are related", prefix),
                            examples: vec![format!("Consider grouping functions: {}", functions.join(", "))],
                        });
                    }
                }
            }
        }

        Ok(suggestions)
    }

    /// Extract function prefix for grouping
    fn extract_function_prefix(&self, function_name: &str) -> String {
        // Simple prefix extraction - take first word or camelCase prefix
        if let Some(underscore_pos) = function_name.find('_') {
            function_name[..underscore_pos].to_string()
        } else {
            // CamelCase splitting (simplified)
            let mut prefix = String::new();
            for (i, ch) in function_name.char_indices() {
                if i > 0 && ch.is_uppercase() {
                    break;
                }
                prefix.push(ch.to_lowercase().next().unwrap_or(ch));
            }
            if prefix.len() < function_name.len() {
                prefix
            } else {
                function_name.to_string()
            }
        }
    }

    /// Generate usage pattern suggestions
    fn generate_usage_pattern_suggestions(&self) -> Result<Vec<ConceptSuggestion>, Error> {
        let mut suggestions = Vec::new();

        // Find common usage patterns
        let mut usage_patterns: HashMap<String, usize> = HashMap::new();
        
        for references in self.reference_index.forward_refs.values() {
            for reference in references {
                let pattern = format!("{:?}", reference.reference_type);
                *usage_patterns.entry(pattern).or_insert(0) += 1;
            }
        }

        for (pattern, count) in usage_patterns {
            if count > 5 {
                suggestions.push(ConceptSuggestion {
                    suggestion_type: SuggestionType::UsagePatterns,
                    primary_concept: pattern.clone(),
                    related_concepts: vec!["common_pattern".to_string()],
                    confidence: 0.6,
                    description: format!("Pattern '{}' is used {} times", pattern, count),
                    examples: vec![format!("This pattern appears {} times in the codebase", count)],
                });
            }
        }

        Ok(suggestions)
    }

    /// Generate various output formats
    pub fn generate_cross_reference_outputs(&self, result: &CrossReferenceResult, output_dir: &Path) -> Result<(), Error> {
        fs::create_dir_all(output_dir)
            .map_err(|e| Error::SystemError(format!("Failed to create output directory: {}", e)))?;

        for format in &self.config.output_formats {
            match format {
                CrossRefFormat::Html => self.generate_html_output(result, output_dir)?,
                CrossRefFormat::Json => self.generate_json_output(result, output_dir)?,
                CrossRefFormat::GraphViz => self.generate_graphviz_output(output_dir)?,
                CrossRefFormat::Mermaid => self.generate_mermaid_output(output_dir)?,
                CrossRefFormat::PlantUml => self.generate_plantuml_output(output_dir)?,
                CrossRefFormat::Cypher => self.generate_cypher_output(output_dir)?,
            }
        }

        Ok(())
    }

    /// Generate HTML cross-reference output
    fn generate_html_output(&self, result: &CrossReferenceResult, output_dir: &Path) -> Result<(), Error> {
        let html_content = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED Cross-Reference Analysis</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; line-height: 1.6; }}
        .header {{ background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 30px; }}
        .section {{ margin: 30px 0; }}
        .section h2 {{ color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }}
        .stat-card {{ background: white; border: 1px solid #e9ecef; border-radius: 8px; padding: 20px; text-align: center; }}
        .stat-number {{ font-size: 2em; font-weight: bold; color: #007bff; }}
        .stat-label {{ color: #6c757d; }}
        .suggestions {{ background: #f8f9fa; border-left: 4px solid #007bff; padding: 15px; margin: 10px 0; }}
        .suggestion {{ margin: 10px 0; }}
        .confidence {{ background: #28a745; color: white; padding: 2px 6px; border-radius: 3px; font-size: 0.8em; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>CURSED Cross-Reference Analysis</h1>
        <p>Comprehensive analysis of code relationships, dependencies, and cross-references.</p>
        <p>Generated: {}</p>
    </div>

    <div class="section">
        <h2>Summary Statistics</h2>
        <div class="stats">
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Total References</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Dependency Nodes</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Type Relationships</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Concept Suggestions</div>
            </div>
        </div>
    </div>

    <div class="section">
        <h2>Dependency Analysis</h2>
        <p><strong>Total Nodes:</strong> {}</p>
        <p><strong>Total Edges:</strong> {}</p>
        <p><strong>Strongly Connected Components:</strong> {}</p>
        <p><strong>Complexity Score:</strong> {:.2}</p>
    </div>

    <div class="section">
        <h2>Type Relationships</h2>
        <p><strong>Total Types:</strong> {}</p>
        <p><strong>Inheritance Chains:</strong> {}</p>
        <p><strong>Interface Implementations:</strong> {}</p>
        <p><strong>Coupling Coefficient:</strong> {:.2}</p>
    </div>

    <div class="section">
        <h2>Concept Suggestions</h2>
        {}
    </div>
</body>
</html>"#,
            result.analysis_metadata.analysis_time,
            result.total_references,
            result.dependency_summary.total_nodes,
            result.type_relationship_summary.total_types,
            result.concept_suggestions.len(),
            result.dependency_summary.total_nodes,
            result.dependency_summary.total_edges,
            result.dependency_summary.strongly_connected_components,
            result.dependency_summary.complexity_score,
            result.type_relationship_summary.total_types,
            result.type_relationship_summary.inheritance_chains,
            result.type_relationship_summary.interface_implementations,
            result.type_relationship_summary.complexity_metrics.coupling_coefficient,
            self.generate_suggestions_html(&result.concept_suggestions)
        );

        fs::write(output_dir.join("cross_references.html"), html_content)
            .map_err(|e| Error::SystemError(format!("Failed to write HTML output: {}", e)))?;

        Ok(())
    }

    /// Generate suggestions HTML
    fn generate_suggestions_html(&self, suggestions: &[ConceptSuggestion]) -> String {
        suggestions.iter()
            .map(|suggestion| {
                format!(
                    r#"<div class="suggestions">
                        <div class="suggestion">
                            <strong>{:?}</strong>: {} 
                            <span class="confidence">{:.0}% confidence</span>
                            <br>
                            <small>{}</small>
                            <br>
                            <em>Related: {}</em>
                        </div>
                    </div>"#,
                    suggestion.suggestion_type,
                    suggestion.primary_concept,
                    suggestion.confidence * 100.0,
                    suggestion.description,
                    suggestion.related_concepts.join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Generate JSON output
    fn generate_json_output(&self, result: &CrossReferenceResult, output_dir: &Path) -> Result<(), Error> {
        let json_content = serde_json::to_string_pretty(result)
            .map_err(|e| Error::SystemError(format!("Failed to serialize result: {}", e)))?;

        fs::write(output_dir.join("cross_references.json"), json_content)
            .map_err(|e| Error::SystemError(format!("Failed to write JSON output: {}", e)))?;

        Ok(())
    }

    /// Generate GraphViz output
    fn generate_graphviz_output(&self, output_dir: &Path) -> Result<(), Error> {
        let mut dot_content = String::from("digraph CrossReferences {\n");
        dot_content.push_str("    rankdir=TB;\n");
        dot_content.push_str("    node [shape=box, style=filled, fillcolor=lightblue];\n");

        // Add nodes
        for (node_id, node) in &self.dependency_graph.nodes {
            let label = format!("{}\\n{:?}", node_id, node.node_type);
            dot_content.push_str(&format!("    \"{}\" [label=\"{}\"];\n", node_id, label));
        }

        // Add edges
        for edge in &self.dependency_graph.edges {
            let label = format!("{:?}", edge.edge_type);
            dot_content.push_str(&format!(
                "    \"{}\" -> \"{}\" [label=\"{}\", weight={}];\n",
                edge.from, edge.to, label, edge.weight
            ));
        }

        dot_content.push_str("}\n");

        fs::write(output_dir.join("dependencies.dot"), dot_content)
            .map_err(|e| Error::SystemError(format!("Failed to write GraphViz output: {}", e)))?;

        Ok(())
    }

    /// Generate Mermaid diagram output
    fn generate_mermaid_output(&self, output_dir: &Path) -> Result<(), Error> {
        let mut mermaid_content = String::from("graph TD\n");

        // Add nodes and edges
        for edge in &self.dependency_graph.edges {
            mermaid_content.push_str(&format!(
                "    {}[{}] --> {}[{}]\n",
                edge.from.replace(" ", "_"),
                edge.from,
                edge.to.replace(" ", "_"),
                edge.to
            ));
        }

        fs::write(output_dir.join("dependencies.mmd"), mermaid_content)
            .map_err(|e| Error::SystemError(format!("Failed to write Mermaid output: {}", e)))?;

        Ok(())
    }

    /// Generate PlantUML output
    fn generate_plantuml_output(&self, output_dir: &Path) -> Result<(), Error> {
        let mut plantuml_content = String::from("@startuml\n");
        plantuml_content.push_str("!theme blueprint\n");

        // Add relationships
        for edge in &self.dependency_graph.edges {
            let arrow = match edge.edge_type {
                DependencyType::Inheritance => "--|>",
                DependencyType::Implementation => "..|>",
                DependencyType::Import => "-->",
                _ => "..>",
            };

            plantuml_content.push_str(&format!(
                "{} {} {} : {:?}\n",
                edge.from, arrow, edge.to, edge.edge_type
            ));
        }

        plantuml_content.push_str("@enduml\n");

        fs::write(output_dir.join("relationships.puml"), plantuml_content)
            .map_err(|e| Error::SystemError(format!("Failed to write PlantUML output: {}", e)))?;

        Ok(())
    }

    /// Generate Cypher query output for Neo4j
    fn generate_cypher_output(&self, output_dir: &Path) -> Result<(), Error> {
        let mut cypher_content = String::new();

        // Create nodes
        cypher_content.push_str("// Create nodes\n");
        for (node_id, node) in &self.dependency_graph.nodes {
            cypher_content.push_str(&format!(
                "CREATE ({}:{}{{name: '{}', type: '{:?}'}})\n",
                node_id.replace(" ", "_"),
                match node.node_type {
                    DependencyNodeType::Module => "Module",
                    DependencyNodeType::Function => "Function",
                    DependencyNodeType::Type => "Type",
                    DependencyNodeType::Interface => "Interface",
                    DependencyNodeType::External => "External",
                },
                node_id,
                node.node_type
            ));
        }

        // Create relationships
        cypher_content.push_str("\n// Create relationships\n");
        for edge in &self.dependency_graph.edges {
            cypher_content.push_str(&format!(
                "MATCH (a{{name: '{}'}}), (b{{name: '{}'}}) CREATE (a)-[:{:?}{{weight: {}}}]->(b)\n",
                edge.from, edge.to, edge.edge_type, edge.weight
            ));
        }

        fs::write(output_dir.join("graph.cypher"), cypher_content)
            .map_err(|e| Error::SystemError(format!("Failed to write Cypher output: {}", e)))?;

        Ok(())
    }

    /// Helper methods for counting and summarizing
    fn count_total_references(&self) -> usize {
        self.reference_index.forward_refs.values()
            .map(|refs| refs.len())
            .sum()
    }

    fn count_references_by_type(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for refs in self.reference_index.by_type.values() {
            for reference in refs {
                *counts.entry(format!("{:?}", reference.reference_type)).or_insert(0) += 1;
            }
        }
        counts
    }

    fn generate_dependency_summary(&self) -> DependencySummary {
        DependencySummary {
            total_nodes: self.dependency_graph.nodes.len(),
            total_edges: self.dependency_graph.edges.len(),
            strongly_connected_components: self.dependency_graph.components.len(),
            cyclic_dependencies: self.count_cyclic_dependencies(),
            max_depth: self.calculate_max_dependency_depth(),
            complexity_score: self.calculate_complexity_score(),
        }
    }

    fn generate_type_relationship_summary(&self) -> TypeRelationshipSummary {
        TypeRelationshipSummary {
            total_types: self.semantic_analyzer.type_database.types.len(),
            inheritance_chains: self.count_inheritance_chains(),
            interface_implementations: self.count_interface_implementations(),
            type_usage_count: self.count_type_usage(),
            complexity_metrics: self.calculate_type_complexity_metrics(),
        }
    }

    fn generate_analysis_metadata(&self, files_count: usize) -> AnalysisMetadata {
        AnalysisMetadata {
            analysis_time: chrono::Utc::now().to_rfc3339(),
            files_analyzed: files_count,
            lines_of_code: 0, // Would be calculated during analysis
            analysis_duration: 0.0, // Would be measured during analysis
            config_used: format!("{:?}", self.config),
        }
    }

    fn count_cyclic_dependencies(&self) -> usize {
        self.dependency_graph.components.iter()
            .filter(|component| component.len() > 1)
            .count()
    }

    fn calculate_max_dependency_depth(&self) -> usize {
        // Simplified calculation - would use proper graph traversal
        10 // Placeholder
    }

    fn calculate_complexity_score(&self) -> f64 {
        let nodes = self.dependency_graph.nodes.len() as f64;
        let edges = self.dependency_graph.edges.len() as f64;
        if nodes > 0.0 {
            edges / nodes
        } else {
            0.0
        }
    }

    fn count_inheritance_chains(&self) -> usize {
        self.type_relationships.direct_relationships.values()
            .flat_map(|rels| rels.iter())
            .filter(|rel| rel.relationship_type == RelationshipType::Extends)
            .count()
    }

    fn count_interface_implementations(&self) -> usize {
        self.type_relationships.direct_relationships.values()
            .flat_map(|rels| rels.iter())
            .filter(|rel| rel.relationship_type == RelationshipType::Implements)
            .count()
    }

    fn count_type_usage(&self) -> usize {
        self.reference_index.by_type.get(&ReferenceType::TypeAnnotation)
            .map(|refs| refs.len())
            .unwrap_or(0)
    }

    fn calculate_type_complexity_metrics(&self) -> TypeComplexityMetrics {
        TypeComplexityMetrics {
            average_inheritance_depth: 2.0, // Placeholder
            average_interface_implementations: 1.5, // Placeholder
            coupling_coefficient: 0.3, // Placeholder
            cohesion_score: 0.7, // Placeholder
        }
    }

    fn build_scope_hierarchy(&mut self) -> Result<(), Error> {
        // Build scope hierarchy from symbol table
        Ok(())
    }

    fn analyze_variable_lifetimes(&mut self) -> Result<(), Error> {
        // Analyze variable lifetimes
        Ok(())
    }

    fn detect_shadowing_issues(&mut self) -> Result<(), Error> {
        // Detect variable shadowing
        Ok(())
    }

    fn compute_strongly_connected_components(&mut self) {
        // Implement Tarjan's algorithm for SCC detection
        // Placeholder implementation
    }

    fn compute_topological_ordering(&mut self) {
        // Implement topological sorting
        // Placeholder implementation
    }
}

// Implementation methods for helper structs
impl SymbolTable {
    fn enter_scope(&mut self, scope_type: ScopeType, location: SourceLocation) {
        let scope_id = self.scopes.len();
        let parent = if self.current_scope == 0 { None } else { Some(self.current_scope) };
        
        self.scopes.push(Scope {
            id: scope_id,
            parent,
            scope_type,
            symbols: HashSet::new(),
            location,
        });
        
        self.current_scope = scope_id;
    }

    fn exit_scope(&mut self) {
        if let Some(scope) = self.scopes.get(self.current_scope) {
            if let Some(parent_id) = scope.parent {
                self.current_scope = parent_id;
            }
        }
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol.clone());
        if let Some(scope) = self.scopes.get_mut(self.current_scope) {
            scope.symbols.insert(symbol.name);
        }
    }
}

impl TypeDatabase {
    fn add_type(&mut self, type_info: TypeInfo) {
        self.types.insert(type_info.name.clone(), type_info);
    }
}

impl DependencyGraph {
    fn add_node(&mut self, node: DependencyNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    fn add_edge(&mut self, edge: DependencyEdge) {
        self.edges.push(edge);
    }
}

impl ReferenceIndex {
    fn add_reference(&mut self, reference: Reference) {
        // Add to forward references
        self.forward_refs.entry(reference.from.clone())
            .or_insert_with(Vec::new)
            .push(reference.clone());

        // Add to backward references
        self.backward_refs.entry(reference.to.clone())
            .or_insert_with(Vec::new)
            .push(reference.clone());

        // Add to type index
        self.by_type.entry(reference.reference_type.clone())
            .or_insert_with(Vec::new)
            .push(reference.clone());

        // Add to location index
        if let Some(ref file) = reference.location.file {
            self.by_location.entry(file.clone())
                .or_insert_with(Vec::new)
                .push(reference);
        }
    }
}
