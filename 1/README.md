# CURSED Topological Sort Demo

This demo showcases graph algorithms implemented in the CURSED programming language, specifically focusing on topological sorting of directed acyclic graphs (DAGs).

## Overview

Topological sorting is a linear ordering of vertices in a directed acyclic graph such that for every directed edge (u, v), vertex u comes before vertex v in the ordering. This demo implements Kahn's algorithm for topological sorting.

## Files

- **`simple_topo_sort.csd`** - Basic demo showing step-by-step topological sorting
- **`advanced_topo_sort.csd`** - More sophisticated implementation with analysis
- **`topological_sort.csd`** - Full implementation (requires stdlib features)
- **`input.txt`** - Sample input data in edge-list format
- **`Makefile`** - Build and test automation

## Input Format

The input represents a directed graph as a list of edges:
```
A B
B C
B D
C E
D E
```

Each line `X Y` means there's a directed edge from node X to node Y.

## Building and Running

### Prerequisites
Ensure the CURSED compiler is built in the parent directory:
```bash
cd .. && make build
```

### Quick Start
```bash
# Show available commands
make help

# Build and run the demo
make run

# Run tests
make test

# Build release version (if available)
make build-release

# Show project information
make info
```

### Manual Execution
```bash
# Run with debug compiler
../target/debug/cursed simple_topo_sort.csd

# Run with release compiler (if available)
../target/release/cursed simple_topo_sort.csd
```

## Algorithm Details

The implementation demonstrates:

1. **Kahn's Algorithm** - Uses in-degree counting and queue processing
2. **Graph Representation** - Adjacency list structure
3. **Cycle Detection** - Identifies when topological sort is impossible
4. **Complexity Analysis** - O(V + E) time, O(V) space

### Step-by-Step Process

1. **Initialize**: Calculate in-degree for each node
2. **Queue Setup**: Add all nodes with in-degree 0 to queue
3. **Process**: While queue is not empty:
   - Remove node from queue
   - Add to result
   - Decrease in-degree of neighbors
   - Add neighbors with in-degree 0 to queue
4. **Validate**: Check if all nodes were processed (no cycles)

## Example Output

```
=== CURSED Topological Sort Demo ===
Input edges:
A -> B
B -> C
B -> D
C -> E
D -> E

Processing topological sort...
Step 1: Find nodes with no incoming edges
Initial queue: [A]
...
Topological order: A B C D E
Alternative valid order: A B D C E

Graph statistics:
Nodes: 5
Edges: 5
Graph is connected
```

## Test Cases

The demo includes several test cases:

1. **Linear Chain**: A→B→C→D (simple case)
2. **Diamond Pattern**: A→{B,C}→D (multiple valid orderings)
3. **Complex Graph**: Multiple components and dependencies

## Learning Objectives

This demo illustrates:

- Graph algorithm implementation in CURSED
- Data structure usage (arrays, queues)
- Control flow (`lowkey`/`highkey` conditionals)
- Function definition (`slay` functions)
- Variable declaration (`sus` variables)
- Output operations (`print`, `yolo`)

## CURSED Language Features Demonstrated

- **Functions**: `slay functionName() { ... }`
- **Variables**: `sus variableName = value`
- **Conditionals**: `lowkey (condition) { ... } highkey { ... }`
- **Loops**: `while (condition) { ... }` and `for item in array { ... }`
- **Output**: `print("text")` and `yolo "text"`
- **Arrays**: `["item1", "item2"]`
- **Dictionaries**: `{"key": "value"}`

## Performance Notes

- Time Complexity: O(V + E) where V = vertices, E = edges
- Space Complexity: O(V) for in-degree array and result storage
- The algorithm is optimal for topological sorting

## Extensions

Possible enhancements:
- File I/O for reading edge lists from files
- Interactive input parsing
- Visualization output
- Parallel processing for independent components
- Support for weighted graphs

## Troubleshooting

If you encounter issues:

1. Ensure CURSED compiler is built: `make verify-compiler`
2. Check compiler version: `../target/debug/cursed --version`
3. Verify input format matches expected edge list format
4. For complex graphs, ensure no cycles exist

---

This demo showcases the expressiveness and capability of the CURSED programming language for implementing fundamental computer science algorithms.
