//! Package-level documentation generation for CURSED
//!
//! Generates package-level documentation, handles cross-package linking,
//! module hierarchy navigation, and standard library documentation integration.

use crate::docs::{DocError, DocResult, DocumentationItem, ItemType, CommentParser};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument};

/// Module information
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    /// Module name
    pub name: String,
    /// Module path
    pub path: PathBuf,
    /// Module documentation
    pub documentation: Option<String>,
    /// Exported items
    pub exports: Vec<DocumentationItem>,
    /// Submodules
    pub submodules: Vec<ModuleInfo>,
    /// Dependencies
    pub dependencies: Vec<String>,
}

impl ModuleInfo {
    /// Create a new module info
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            documentation: None,
            exports: Vec::new(),
            submodules: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    /// Set module documentation
    pub fn with_documentation(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Add an exported item
    pub fn add_export(mut self, item: DocumentationItem) -> Self {
        self.exports.push(item);
        self
    }

    /// Add a submodule
    pub fn add_submodule(mut self, module: ModuleInfo) -> Self {
        self.submodules.push(module);
        self
    }

    /// Add a dependency
    pub fn add_dependency(mut self, dependency: String) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Get all items recursively
    pub fn all_items(&self) -> Vec<&DocumentationItem> {
        let mut items = Vec::new();
        
        // Add exports from this module
        items.extend(&self.exports);
        
        // Add items from submodules
        for submodule in &self.submodules {
            items.extend(submodule.all_items());
        }
        
        items
    }

    /// Count total items
    pub fn item_count(&self) -> usize {
        let mut count = self.exports.len();
        for submodule in &self.submodules {
            count += submodule.item_count();
        }
        count
    }
}

/// Package documentation generator
pub struct PackageDocumentation {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: Option<String>,
    /// Root module
    pub root_module: ModuleInfo,
    /// Cross-references
    pub cross_references: HashMap<String, Vec<String>>,
    /// External dependencies
    pub external_dependencies: Vec<String>,
}

impl PackageDocumentation {
    /// Create a new package documentation
    pub fn new(name: String, version: String) -> Self {
        Self {
            name: name.clone(),
            version,
            description: None,
            root_module: ModuleInfo::new(name, PathBuf::from(".")),
            cross_references: HashMap::new(),
            external_dependencies: Vec::new(),
        }
    }

    /// Set package description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Generate package documentation from source directory
    #[instrument(skip(self, source_dir))]
    pub fn generate_from_directory<P: AsRef<Path>>(&mut self, source_dir: P) -> DocResult<()> {
        let source_path = source_dir.as_ref();
        info!("Generating package documentation from: {}", source_path.display());

        // Scan directory for CURSED files
        let cursed_files = self.find_cursed_files(source_path)?;
        debug!("Found {} CURSED files", cursed_files.len());

        // Process each file
        for file_path in cursed_files {
            self.process_file(&file_path)?;
        }

        // Build cross-references
        self.build_cross_references()?;

        Ok(())
    }

    /// Find all CURSED files in directory
    fn find_cursed_files(&self, dir: &Path) -> DocResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in std::fs::read_dir(dir)
            .map_err(|e| DocError::IoError(format!("Failed to read directory {}: {}", dir.display(), e)))? 
        {
            let entry = entry
                .map_err(|e| DocError::IoError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_file() && self.is_cursed_file(&path) {
                files.push(path);
            } else if path.is_dir() && !self.should_ignore_directory(&path) {
                let mut subdir_files = self.find_cursed_files(&path)?;
                files.append(&mut subdir_files);
            }
        }
        
        Ok(files)
    }

    /// Check if a file is a CURSED source file
    fn is_cursed_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("csd"))
            .unwrap_or(false)
    }

    /// Check if a directory should be ignored
    fn should_ignore_directory(&self, path: &Path) -> bool {
        let ignore_dirs = ["target", "build", ".git", "node_modules", "tmp"];
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| ignore_dirs.contains(&name))
            .unwrap_or(false)
    }

    /// Process a single CURSED file
    fn process_file(&mut self, file_path: &Path) -> DocResult<()> {
        debug!("Processing file: {}", file_path.display());

        // Read file content
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| DocError::IoError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

        // Parse comments
        let comment_parser = CommentParser::new()?;
        let comments = comment_parser.parse_comments(&content)?;

        // AST extraction disabled for now
        // TODO: Parse AST from content - this would require integration with the CURSED parser
        let items = Vec::new();
        
        // Extract module name 
        let module_name = self.extract_module_name(file_path)?;

        // Find or create module
        let module = self.find_or_create_module(&module_name, file_path);
        
        // Add items to module
        for item in items {
            module.exports.push(item);
        }

        Ok(())
    }

    /// Extract module name from file path
    fn extract_module_name(&self, file_path: &Path) -> DocResult<String> {
        let stem = file_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| DocError::ParseError(format!("Invalid file path: {}", file_path.display())))?;
        
        // Convert file path to module path (e.g., src/utils/math.csd -> utils::math)
        let parent_dirs: Vec<String> = if let Some(parent) = file_path.parent() {
            parent.components()
                .filter_map(|c| {
                    let name = c.as_os_str().to_string_lossy();
                    // Filter out common directory names
                    if ["src", "lib", "."].contains(&name.as_ref()) {
                        None
                    } else {
                        Some(name.into_owned())
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        
        let mut module_path = parent_dirs;
        module_path.push(stem.into());
        
        let result = if module_path.is_empty() {
            "main".to_string()
        } else {
            module_path.join("::")
        };
        
        Ok(result)
    }

    /// Find or create a module
    fn find_or_create_module(&mut self, module_name: &str, file_path: &Path) -> &mut ModuleInfo {
        // This is a simplified implementation
        // In a real implementation, you'd handle nested modules properly
        &mut self.root_module
    }

    /// Build cross-references between items
    fn build_cross_references(&mut self) -> DocResult<()> {
        let all_items = self.root_module.all_items();
        
        for item in &all_items {
            let mut references = Vec::new();
            
            // Look for references in description
            if let Some(description) = item.description() {
                references.extend(self.find_references_in_text(description, &all_items));
            }
            
            // Look for references in parameter types
            for param in &item.parameters {
                references.extend(self.find_references_in_text(&param.param_type, &all_items));
            }
            
            // Look for references in return type
            if let Some(return_type) = &item.return_type {
                references.extend(self.find_references_in_text(return_type, &all_items));
            }
            
            if !references.is_empty() {
                self.cross_references.insert(item.name.clone(), references);
            }
        }
        
        Ok(())
    }

    /// Find references to other items in text
    fn find_references_in_text(&self, text: &str, all_items: &[&DocumentationItem]) -> Vec<String> {
        let mut references = Vec::new();
        
        for item in all_items {
            if text.contains(&item.name) {
                references.push(item.name.clone());
            }
        }
        
        references
    }

    /// Generate module hierarchy
    pub fn generate_module_hierarchy(&self) -> String {
        let mut hierarchy = String::new();
        self.generate_module_hierarchy_recursive(&self.root_module, 0, &mut hierarchy);
        hierarchy
    }

    /// Generate module hierarchy recursively
    fn generate_module_hierarchy_recursive(&self, module: &ModuleInfo, depth: usize, output: &mut String) {
        let indent = "  ".repeat(depth);
        
        output.push_str(&format!("{}{}/ ({} items)\n", indent, module.name, module.exports.len()));
        
        for submodule in &module.submodules {
            self.generate_module_hierarchy_recursive(submodule, depth + 1, output);
        }
    }

    /// Generate dependency graph
    pub fn generate_dependency_graph(&self) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        
        // Build dependency graph from modules
        self.build_dependency_graph_recursive(&self.root_module, &mut graph);
        
        graph
    }

    /// Build dependency graph recursively
    fn build_dependency_graph_recursive(&self, module: &ModuleInfo, graph: &mut HashMap<String, Vec<String>>) {
        graph.insert(module.name.clone(), module.dependencies.clone());
        
        for submodule in &module.submodules {
            self.build_dependency_graph_recursive(submodule, graph);
        }
    }

    /// Get package statistics
    pub fn get_statistics(&self) -> PackageStatistics {
        let all_items = self.root_module.all_items();
        
        let function_count = all_items.iter().filter(|i| i.item_type == ItemType::Function).count();
        let squad_count = all_items.iter().filter(|i| i.item_type == ItemType::Squad).count();
        let collab_count = all_items.iter().filter(|i| i.item_type == ItemType::Collab).count();
        let total_lines_of_docs = all_items.iter()
            .filter_map(|i| i.description())
            .map(|desc| desc.lines().count())
            .sum();
        
        PackageStatistics {
            total_modules: self.count_modules(&self.root_module),
            total_items: all_items.len(),
            function_count,
            squad_count,
            collab_count,
            cross_reference_count: self.cross_references.len(),
            total_lines_of_documentation: total_lines_of_docs,
        }
    }

    /// Count modules recursively
    fn count_modules(&self, module: &ModuleInfo) -> usize {
        1 + module.submodules.iter().map(|m| self.count_modules(m)).sum::<usize>()
    }

    /// Export package information as JSON
    pub fn export_metadata(&self) -> DocResult<String> {
        let metadata = serde_json::json!({
            "name": self.name,
            "version": self.version,
            "description": self.description,
            "statistics": self.get_statistics(),
            "modules": self.root_module.name,
            "dependencies": self.external_dependencies
        });
        
        serde_json::to_string_pretty(&metadata)
            .map_err(|e| DocError::IoError(format!("Failed to serialize metadata: {}", e)))
    }
}

/// Package statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct PackageStatistics {
    pub total_modules: usize,
    pub total_items: usize,
    pub function_count: usize,
    pub squad_count: usize,
    pub collab_count: usize,
    pub cross_reference_count: usize,
    pub total_lines_of_documentation: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_info_creation() {
        let module = ModuleInfo::new("test".to_string(), PathBuf::from("test.csd"));
        
        assert_eq!(module.name, "test");
        assert_eq!(module.path, PathBuf::from("test.csd"));
        assert_eq!(module.exports.len(), 0);
        assert_eq!(module.submodules.len(), 0);
    }

    #[test]
    fn test_package_documentation_creation() {
        let pkg = PackageDocumentation::new("test_package".to_string(), "1.0.0".to_string());
        
        assert_eq!(pkg.name, "test_package");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.root_module.name, "test_package");
    }

    #[test]
    fn test_is_cursed_file() {
        let pkg = PackageDocumentation::new("test".to_string(), "1.0.0".to_string());
        
        assert!(pkg.is_cursed_file(Path::new("test.csd")));
        assert!(pkg.is_cursed_file(Path::new("TEST.CSD")));
        assert!(!pkg.is_cursed_file(Path::new("test.rs")));
        assert!(!pkg.is_cursed_file(Path::new("test.txt")));
    }

    #[test]
    fn test_should_ignore_directory() {
        let pkg = PackageDocumentation::new("test".to_string(), "1.0.0".to_string());
        
        assert!(pkg.should_ignore_directory(Path::new("target")));
        assert!(pkg.should_ignore_directory(Path::new("build")));
        assert!(pkg.should_ignore_directory(Path::new(".git")));
        assert!(!pkg.should_ignore_directory(Path::new("src")));
        assert!(!pkg.should_ignore_directory(Path::new("lib")));
    }

    #[test]
    fn test_extract_module_name() {
        let pkg = PackageDocumentation::new("test".to_string(), "1.0.0".to_string());
        
        let module_name = pkg.extract_module_name(Path::new("src/utils/math.csd")).unwrap();
        assert_eq!(module_name, "utils::math");
        
        let module_name = pkg.extract_module_name(Path::new("lib.csd")).unwrap();
        assert_eq!(module_name, "lib");
    }

    #[test]
    fn test_module_item_count() {
        let mut module = ModuleInfo::new("test".to_string(), PathBuf::from("test.csd"));
        
        // Add some items
        module = module.add_export(DocumentationItem::new("func1".to_string(), ItemType::Function, 1));
        module = module.add_export(DocumentationItem::new("func2".to_string(), ItemType::Function, 2));
        
        // Add submodule with items
        let mut submodule = ModuleInfo::new("sub".to_string(), PathBuf::from("sub.csd"));
        submodule = submodule.add_export(DocumentationItem::new("func3".to_string(), ItemType::Function, 3));
        module = module.add_submodule(submodule);
        
        assert_eq!(module.item_count(), 3);
        assert_eq!(module.all_items().len(), 3);
    }
}
