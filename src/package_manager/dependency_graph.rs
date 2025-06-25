// Dependency graph management
use crate::error_types::CursedError;
use std::collections::{HashMap, HashSet, VecDeque};

/// Dependency graph for package resolution
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, package: String, node: DependencyNode) {
        self.nodes.insert(package.clone(), node);
        self.edges.entry(package).or_insert_with(Vec::new);
    }

    pub fn add_dependency(&mut self, package: String, dependency: String) {
        self.edges.entry(package).or_default().push(dependency);
    }

    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in self.nodes.keys() {
            if !visited.contains(node) {
                if let Some(cycle) = self.dfs_cycle_detect(node, &mut visited, &mut rec_stack, &mut Vec::new()) {
                    cycles.push(cycle);
                }
            }
        }

        cycles
    }

    fn dfs_cycle_detect(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(dependencies) = self.edges.get(node) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_cycle_detect(dep, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Found cycle
                    let cycle_start = path.iter().position(|x| x == dep).unwrap();
                    return Some(path[cycle_start..].to_vec());
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        None
    }

    pub fn topological_sort(&self) -> crate::error_types::Result<Vec<String>> {
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Calculate in-degrees
        for node in self.nodes.keys() {
            in_degree.insert(node.clone(), 0);
        }

        for dependencies in self.edges.values() {
            for dep in dependencies {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // Find nodes with no incoming edges
        for (node, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node.clone());
            }
        }

        // Process nodes
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(dependencies) = self.edges.get(&node) {
                for dep in dependencies {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err(CursedError::Parse("Circular dependency detected".to_string()));
        }

        Ok(result)
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Node in the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub package: String,
    pub version: crate::package_manager::Version,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
}

impl DependencyNode {
    pub fn new(package: String, version: crate::package_manager::Version) -> Self {
        Self {
            package,
            version,
            dependencies: Vec::new(),
            dev_dependencies: Vec::new(),
            optional_dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: String) {
        self.dependencies.push(dep);
    }

    pub fn add_dev_dependency(&mut self, dep: String) {
        self.dev_dependencies.push(dep);
    }

    pub fn add_optional_dependency(&mut self, dep: String) {
        self.optional_dependencies.push(dep);
    }
}
