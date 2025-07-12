#!/usr/bin/env python3
"""
Stdlib Dependency Graph Analyzer
Analyzes dependencies between stdlib modules and creates dependency graphs.
"""

import os
import re
import json
from pathlib import Path
from typing import Dict, List, Set, Tuple
from collections import defaultdict, deque
try:
    import graphviz
    HAS_GRAPHVIZ = True
except ImportError:
    HAS_GRAPHVIZ = False

class StdlibDependencyAnalyzer:
    def __init__(self, stdlib_dir: str = "stdlib"):
        self.stdlib_dir = Path(stdlib_dir)
        self.modules = {}
        self.dependency_graph = defaultdict(set)
        self.reverse_dependency_graph = defaultdict(set)
        
    def discover_modules(self) -> Dict[str, Dict]:
        """Discover all stdlib modules and their dependencies."""
        modules = {}
        
        for module_dir in self.stdlib_dir.iterdir():
            if module_dir.is_dir() and not module_dir.name.startswith('.'):
                module_name = module_dir.name
                modules[module_name] = {
                    'path': module_dir,
                    'dependencies': set(),
                    'dependents': set(),
                    'functions': [],
                    'exports': [],
                    'test_files': []
                }
                
                # Analyze mod.csd file
                mod_file = module_dir / 'mod.csd'
                if mod_file.exists():
                    with open(mod_file, 'r') as f:
                        content = f.read()
                        
                        # Extract dependencies (yeet statements)
                        deps = re.findall(r'yeet\s+"([^"]+)"', content)
                        modules[module_name]['dependencies'] = set(deps)
                        
                        # Extract function definitions
                        functions = re.findall(r'slay\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(', content)
                        modules[module_name]['functions'] = functions
                        
                        # Extract exports (vibes statements)
                        exports = re.findall(r'vibes\s+([a-zA-Z_][a-zA-Z0-9_]*)', content)
                        modules[module_name]['exports'] = exports
                
                # Find test files
                test_files = list(module_dir.glob('test_*.csd')) + list(module_dir.glob('*_test.csd'))
                modules[module_name]['test_files'] = [f.name for f in test_files]
        
        return modules
    
    def build_dependency_graph(self):
        """Build forward and reverse dependency graphs."""
        for module_name, module_info in self.modules.items():
            for dep in module_info['dependencies']:
                if dep in self.modules:  # Only include stdlib dependencies
                    self.dependency_graph[module_name].add(dep)
                    self.reverse_dependency_graph[dep].add(module_name)
    
    def find_circular_dependencies(self) -> List[List[str]]:
        """Find circular dependencies in the module graph."""
        visited = set()
        rec_stack = set()
        cycles = []
        
        def dfs(node, path):
            if node in rec_stack:
                # Found a cycle
                cycle_start = path.index(node)
                cycle = path[cycle_start:] + [node]
                cycles.append(cycle)
                return
            
            if node in visited:
                return
            
            visited.add(node)
            rec_stack.add(node)
            
            for neighbor in self.dependency_graph[node]:
                dfs(neighbor, path + [node])
            
            rec_stack.remove(node)
        
        for module in self.modules:
            if module not in visited:
                dfs(module, [])
        
        return cycles
    
    def calculate_module_metrics(self) -> Dict[str, Dict]:
        """Calculate various metrics for each module."""
        metrics = {}
        
        for module_name, module_info in self.modules.items():
            metrics[module_name] = {
                'direct_dependencies': len(module_info['dependencies']),
                'direct_dependents': len(self.reverse_dependency_graph[module_name]),
                'function_count': len(module_info['functions']),
                'export_count': len(module_info['exports']),
                'test_file_count': len(module_info['test_files']),
                'has_tests': len(module_info['test_files']) > 0,
                'dependency_depth': self.calculate_dependency_depth(module_name),
                'fan_out': len(module_info['dependencies']),
                'fan_in': len(self.reverse_dependency_graph[module_name])
            }
        
        return metrics
    
    def calculate_dependency_depth(self, module_name: str) -> int:
        """Calculate the maximum dependency depth for a module."""
        visited = set()
        
        def dfs(node):
            if node in visited:
                return 0
            
            visited.add(node)
            max_depth = 0
            
            for dep in self.dependency_graph[node]:
                depth = dfs(dep)
                max_depth = max(max_depth, depth + 1)
            
            return max_depth
        
        return dfs(module_name)
    
    def find_critical_modules(self) -> List[str]:
        """Find modules that are dependencies of many other modules."""
        critical_threshold = 5  # Modules with 5+ dependents are critical
        
        critical_modules = []
        for module_name in self.modules:
            dependent_count = len(self.reverse_dependency_graph[module_name])
            if dependent_count >= critical_threshold:
                critical_modules.append((module_name, dependent_count))
        
        return sorted(critical_modules, key=lambda x: x[1], reverse=True)
    
    def find_leaf_modules(self) -> List[str]:
        """Find modules that don't depend on any other stdlib modules."""
        leaf_modules = []
        for module_name, module_info in self.modules.items():
            stdlib_deps = [dep for dep in module_info['dependencies'] if dep in self.modules]
            if not stdlib_deps:
                leaf_modules.append(module_name)
        
        return sorted(leaf_modules)
    
    def find_root_modules(self) -> List[str]:
        """Find modules that no other modules depend on."""
        root_modules = []
        for module_name in self.modules:
            if not self.reverse_dependency_graph[module_name]:
                root_modules.append(module_name)
        
        return sorted(root_modules)
    
    def topological_sort(self) -> List[str]:
        """Return modules in topological order (dependencies first)."""
        in_degree = {module: 0 for module in self.modules}
        
        for module in self.modules:
            for dep in self.dependency_graph[module]:
                in_degree[dep] += 1
        
        queue = deque([module for module in self.modules if in_degree[module] == 0])
        result = []
        
        while queue:
            module = queue.popleft()
            result.append(module)
            
            for dependent in self.reverse_dependency_graph[module]:
                in_degree[dependent] -= 1
                if in_degree[dependent] == 0:
                    queue.append(dependent)
        
        return result
    
    def generate_dependency_visualization(self, output_file: str = "stdlib_dependencies"):
        """Generate a visual dependency graph."""
        if not HAS_GRAPHVIZ:
            print("Graphviz not available. Generating text-based visualization...")
            self.generate_text_visualization(output_file)
            return
            
        try:
            dot = graphviz.Digraph(comment='CURSED Stdlib Dependencies')
            dot.attr(rankdir='TB', size='20,20')
            
            # Add nodes
            critical_modules = {name for name, _ in self.find_critical_modules()}
            leaf_modules = set(self.find_leaf_modules())
            root_modules = set(self.find_root_modules())
            
            for module_name in self.modules:
                if module_name in critical_modules:
                    dot.node(module_name, module_name, style='filled', fillcolor='lightcoral')
                elif module_name in leaf_modules:
                    dot.node(module_name, module_name, style='filled', fillcolor='lightgreen')
                elif module_name in root_modules:
                    dot.node(module_name, module_name, style='filled', fillcolor='lightblue')
                else:
                    dot.node(module_name, module_name)
            
            # Add edges
            for module_name in self.modules:
                for dep in self.dependency_graph[module_name]:
                    dot.edge(module_name, dep)
            
            # Save graph
            dot.render(output_file, format='svg', cleanup=True)
            dot.render(output_file, format='png', cleanup=True)
            
            print(f"Dependency graph saved as {output_file}.svg and {output_file}.png")
            
        except Exception as e:
            print(f"Failed to generate visualization: {e}")
            print("Install graphviz: pip install graphviz")
    
    def generate_text_visualization(self, output_file: str):
        """Generate a text-based dependency visualization."""
        with open(f"{output_file}.txt", "w") as f:
            f.write("CURSED Stdlib Dependency Graph (Text Format)\n")
            f.write("=" * 50 + "\n\n")
            
            for module_name in sorted(self.modules.keys()):
                f.write(f"{module_name}\n")
                
                # Dependencies
                deps = sorted(self.dependency_graph[module_name])
                if deps:
                    f.write("  Dependencies:\n")
                    for dep in deps:
                        f.write(f"    └── {dep}\n")
                
                # Dependents
                dependents = sorted(self.reverse_dependency_graph[module_name])
                if dependents:
                    f.write("  Used by:\n")
                    for dependent in dependents:
                        f.write(f"    └── {dependent}\n")
                
                f.write("\n")
        
        print(f"Text dependency graph saved as {output_file}.txt")
    
    def generate_test_dependency_order(self) -> List[str]:
        """Generate optimal test order based on dependencies."""
        # Test leaf modules first, then work up the dependency chain
        topo_order = self.topological_sort()
        return list(reversed(topo_order))  # Reverse to test dependencies first
    
    def generate_integration_test_pairs(self) -> List[Tuple[str, str]]:
        """Generate pairs of modules that should be integration tested together."""
        pairs = []
        
        for module_name in self.modules:
            for dep in self.dependency_graph[module_name]:
                pairs.append((module_name, dep))
        
        return pairs
    
    def generate_dependency_report(self) -> str:
        """Generate a comprehensive dependency analysis report."""
        metrics = self.calculate_module_metrics()
        cycles = self.find_circular_dependencies()
        critical_modules = self.find_critical_modules()
        leaf_modules = self.find_leaf_modules()
        root_modules = self.find_root_modules()
        
        report = f"""# CURSED Stdlib Dependency Analysis Report

## Overview
- **Total Modules**: {len(self.modules)}
- **Total Dependencies**: {sum(len(deps) for deps in self.dependency_graph.values())}
- **Circular Dependencies**: {len(cycles)}
- **Critical Modules**: {len(critical_modules)}
- **Leaf Modules**: {len(leaf_modules)}
- **Root Modules**: {len(root_modules)}

## Critical Modules (High Fan-In)
These modules are depended upon by many other modules:
"""
        
        for module_name, dependent_count in critical_modules:
            report += f"- **{module_name}**: {dependent_count} dependents\n"
        
        report += f"""
## Leaf Modules (No Dependencies)
These modules don't depend on other stdlib modules:
"""
        for module in leaf_modules:
            report += f"- {module}\n"
        
        report += f"""
## Root Modules (No Dependents)
These modules are not used by other stdlib modules:
"""
        for module in root_modules:
            report += f"- {module}\n"
        
        if cycles:
            report += f"""
## Circular Dependencies ⚠️
The following circular dependencies were found:
"""
            for i, cycle in enumerate(cycles, 1):
                report += f"{i}. {' → '.join(cycle)}\n"
        
        report += f"""
## Module Metrics
| Module | Dependencies | Dependents | Functions | Tests | Depth |
|--------|-------------|------------|-----------|-------|-------|
"""
        
        for module_name in sorted(self.modules.keys()):
            m = metrics[module_name]
            report += f"| {module_name} | {m['direct_dependencies']} | {m['direct_dependents']} | {m['function_count']} | {'✅' if m['has_tests'] else '❌'} | {m['dependency_depth']} |\n"
        
        report += f"""
## Recommendations
1. **Critical Modules**: Focus testing efforts on critical modules with high fan-in
2. **Circular Dependencies**: {"Resolve circular dependencies" if cycles else "No circular dependencies found ✅"}
3. **Test Coverage**: Ensure all modules have comprehensive tests
4. **Integration Testing**: Test critical dependency pairs together
5. **Module Isolation**: Consider reducing dependencies in high-depth modules

## Test Order Recommendation
Based on dependency analysis, test modules in this order:
"""
        
        test_order = self.generate_test_dependency_order()
        for i, module in enumerate(test_order, 1):
            report += f"{i}. {module}\n"
        
        return report
    
    def run_analysis(self):
        """Run complete dependency analysis."""
        print("Starting stdlib dependency analysis...")
        
        # Discover modules
        self.modules = self.discover_modules()
        print(f"Discovered {len(self.modules)} modules")
        
        # Build dependency graph
        self.build_dependency_graph()
        print("Built dependency graph")
        
        # Generate reports
        dependency_report = self.generate_dependency_report()
        with open("stdlib_dependency_report.md", "w") as f:
            f.write(dependency_report)
        
        # Generate visualization
        self.generate_dependency_visualization()
        
        # Save raw data
        analysis_data = {
            'modules': {name: {
                'dependencies': list(info['dependencies']),
                'dependents': list(self.reverse_dependency_graph[name]),
                'functions': info['functions'],
                'exports': info['exports'],
                'test_files': info['test_files']
            } for name, info in self.modules.items()},
            'metrics': self.calculate_module_metrics(),
            'circular_dependencies': self.find_circular_dependencies(),
            'critical_modules': self.find_critical_modules(),
            'leaf_modules': self.find_leaf_modules(),
            'root_modules': self.find_root_modules(),
            'test_order': self.generate_test_dependency_order(),
            'integration_pairs': self.generate_integration_test_pairs()
        }
        
        with open("stdlib_dependency_analysis.json", "w") as f:
            json.dump(analysis_data, f, indent=2)
        
        print("Analysis complete!")
        print("Generated:")
        print("- stdlib_dependency_report.md")
        print("- stdlib_dependency_analysis.json")
        print("- stdlib_dependencies.svg")
        print("- stdlib_dependencies.png")

if __name__ == "__main__":
    analyzer = StdlibDependencyAnalyzer()
    analyzer.run_analysis()
