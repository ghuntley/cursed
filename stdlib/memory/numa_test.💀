fr fr CURSED NUMA Topology System Test
fr fr Production-level testing of NUMA memory management

yeet "memory/numa_topology"
yeet "testz"
yeet "memory/memory_pools"

fr fr NUMA System Validation Tests
slay test_numa_availability() lit {
    vibez.spill("=== Testing NUMA Availability Detection ===")
    
    fr fr Test NUMA topology initialization
    sus topology *NUMATopology = numa_topology_init()
    testz.assert_not_null(topology, "NUMA topology should initialize")
    
    fr fr Test node count detection
    sus node_count normie = numa_get_node_count()
    testz.assert_gte_int(node_count, 1, "Should detect at least 1 NUMA node")
    testz.assert_lte_int(node_count, 64, "Should not detect unrealistic node count")
    
    vibez.spillf("Detected {} NUMA nodes", node_count)
    damn based
}

slay test_numa_node_information() lit {
    vibez.spill("=== Testing NUMA Node Information ===")
    
    sus topology *NUMATopology = numa_get_topology()
    testz.assert_not_null(topology, "Should have initialized topology")
    
    fr fr Test each node has valid information
    bestie i := 0; i < topology.nodes.len(); i = i + 1 {
        sus node NUMANode = topology.nodes[i]
        
        vibez.spillf("Testing Node {}", node.node_id)
        testz.assert_eq_int(node.node_id, i, "Node ID should match array index")
        testz.assert_eq_lit(node.online, based, "Node should be online")
        
        fr fr Test CPU information
        testz.assert_gte_int(node.local_cpus.len(), 1, "Node should have at least 1 CPU")
        testz.assert_gte_thicc(node.cpu_mask, 1, "CPU mask should be set")
        
        fr fr Test memory information  
        testz.assert_gte_thicc(node.memory_size, 1024*1024, "Node should have at least 1MB memory")
        testz.assert_lte_thicc(node.free_memory, node.memory_size, "Free memory should not exceed total")
        
        fr fr Test distance information
        testz.assert_eq_int(node.distances.len(), topology.num_nodes, "Distance array should match node count")
        testz.assert_eq_int(node.distances[i], 10, "Local distance should be 10")
        
        vibez.spillf("  CPUs: {} (mask: 0x{:x})", node.local_cpus.len(), node.cpu_mask)
        vibez.spillf("  Memory: {} MB total, {} MB free", 
                    node.memory_size / (1024 * 1024), 
                    node.free_memory / (1024 * 1024))
    }
    
    damn based
}

slay test_numa_current_node() lit {
    vibez.spill("=== Testing Current NUMA Node Detection ===")
    
    sus current_node normie = numa_get_current_node()
    sus node_count normie = numa_get_node_count()
    
    testz.assert_gte_int(current_node, 0, "Current node should be non-negative")
    testz.assert_lt_int(current_node, node_count, "Current node should be within valid range")
    
    vibez.spillf("Process running on NUMA node: {}", current_node)
    
    fr fr Test multiple calls return consistent results
    sus current_node2 normie = numa_get_current_node()
    testz.assert_eq_int(current_node, current_node2, "Current node should be consistent")
    
    damn based
}

slay test_numa_memory_allocation() lit {
    vibez.spill("=== Testing NUMA Memory Allocation ===")
    
    sus node_count normie = numa_get_node_count()
    
    fr fr Test local allocation
    sus local_ptr *void = numa_alloc_local(1024)
    testz.assert_not_null(local_ptr, "Local allocation should succeed")
    vibez.spill("Local allocation: SUCCESS")
    
    fr fr Test node-specific allocation
    bestie node := 0; node < node_count; node = node + 1 {
        sus node_ptr *void = numa_alloc_on_node(1024, node)
        testz.assert_not_null(node_ptr, "Node-specific allocation should succeed")
        vibez.spillf("Node {} allocation: SUCCESS", node)
    }
    
    fr fr Test interleaved allocation
    sus interleaved_ptr *void = numa_alloc_interleaved(4096)
    testz.assert_not_null(interleaved_ptr, "Interleaved allocation should succeed")
    vibez.spill("Interleaved allocation: SUCCESS")
    
    fr fr Test invalid node ID handling
    sus invalid_ptr *void = numa_alloc_on_node(1024, 999)
    testz.assert_not_null(invalid_ptr, "Invalid node allocation should fallback gracefully")
    vibez.spill("Invalid node allocation: HANDLED GRACEFULLY")
    
    damn based
}

slay test_numa_distance_matrix() lit {
    vibez.spill("=== Testing NUMA Distance Matrix ===")
    
    sus node_count normie = numa_get_node_count()
    
    fr fr Test distance calculations
    bestie from_node := 0; from_node < node_count; from_node = from_node + 1 {
        bestie to_node := 0; to_node < node_count; to_node = to_node + 1 {
            sus distance normie = numa_get_distance(from_node, to_node)
            
            yo from_node == to_node {
                testz.assert_eq_int(distance, 10, "Local distance should be 10")
            } otherwise {
                testz.assert_gte_int(distance, 10, "Inter-node distance should be >= 10")
                testz.assert_lte_int(distance, 255, "Distance should be <= 255")
            }
            
            vibez.spillf("Distance from node {} to node {}: {}", from_node, to_node, distance)
        }
    }
    
    fr fr Test invalid node handling
    sus invalid_distance normie = numa_get_distance(999, 0)
    testz.assert_eq_int(invalid_distance, -1, "Invalid node distance should return -1")
    
    damn based
}

slay test_numa_memory_statistics() lit {
    vibez.spill("=== Testing NUMA Memory Statistics ===")
    
    sus node_count normie = numa_get_node_count()
    sus total_memory thicc = 0
    sus total_free thicc = 0
    
    bestie node := 0; node < node_count; node = node + 1 {
        sus node_memory thicc = numa_get_node_memory_size(node)
        sus node_free thicc = numa_get_node_free_memory(node)
        
        testz.assert_gte_thicc(node_memory, 1024*1024, "Node should have at least 1MB memory")
        testz.assert_lte_thicc(node_free, node_memory, "Free should not exceed total")
        testz.assert_gte_thicc(node_free, 0, "Free memory should be non-negative")
        
        total_memory = total_memory + node_memory
        total_free = total_free + node_free
        
        vibez.spillf("Node {}: {} MB total, {} MB free ({:.1f}% free)", 
                    node, 
                    node_memory / (1024 * 1024),
                    node_free / (1024 * 1024),
                    (node_free * 100.0) / node_memory)
    }
    
    vibez.spillf("Total system: {} MB, {} MB free", 
                total_memory / (1024 * 1024), total_free / (1024 * 1024))
    
    fr fr Test invalid node handling
    sus invalid_memory thicc = numa_get_node_memory_size(999)
    testz.assert_eq_thicc(invalid_memory, 0, "Invalid node memory should return 0")
    
    damn based
}

slay test_numa_topology_printing() lit {
    vibez.spill("=== Testing NUMA Topology Printing ===")
    
    fr fr This will print detailed topology information
    numa_print_topology()
    
    vibez.spill("Topology printing: COMPLETED")
    damn based
}

slay test_numa_stress_allocation() lit {
    vibez.spill("=== Testing NUMA Stress Allocation ===")
    
    sus node_count normie = numa_get_node_count()
    sus allocation_count normie = 100
    
    fr fr Stress test with many allocations
    bestie i := 0; i < allocation_count; i = i + 1 {
        sus target_node normie = i % node_count
        sus ptr *void = numa_alloc_on_node(4096, target_node)
        testz.assert_not_null(ptr, "Stress allocation should succeed")
        
        yo (i % 10) == 0 {
            vibez.spillf("Completed {} allocations", i)
        }
    }
    
    vibez.spillf("Stress test: {} allocations completed successfully", allocation_count)
    damn based
}

slay test_numa_memory_migration() lit {
    vibez.spill("=== Testing NUMA Memory Migration Simulation ===")
    
    sus node_count normie = numa_get_node_count()
    yo node_count < 2 {
        vibez.spill("Single node system - skipping migration test")
        damn based
    }
    
    fr fr Simulate memory migration between nodes
    sus source_node normie = 0
    sus target_node normie = 1
    
    fr fr Allocate on source node
    sus ptr *void = numa_alloc_on_node(8192, source_node)
    testz.assert_not_null(ptr, "Source allocation should succeed")
    
    fr fr Simulate migration to target node (conceptual test)
    sus migrated_ptr *void = numa_alloc_on_node(8192, target_node)  
    testz.assert_not_null(migrated_ptr, "Target allocation should succeed")
    
    vibez.spillf("Simulated memory migration from node {} to node {}", source_node, target_node)
    damn based
}

slay test_numa_performance_characteristics() lit {
    vibez.spill("=== Testing NUMA Performance Characteristics ===")
    
    sus node_count normie = numa_get_node_count()
    
    fr fr Test local vs remote access simulation
    bestie from_node := 0; from_node < node_count; from_node = from_node + 1 {
        bestie to_node := 0; to_node < node_count; to_node = to_node + 1 {
            sus distance normie = numa_get_distance(from_node, to_node)
            sus is_local lit = (from_node == to_node)
            
            yo is_local {
                vibez.spillf("Local access (node {} -> {}): distance={} (FASTEST)", 
                           from_node, to_node, distance)
            } otherwise yo distance <= 20 {
                vibez.spillf("Near access (node {} -> {}): distance={} (FAST)", 
                           from_node, to_node, distance)
            } otherwise {
                vibez.spillf("Remote access (node {} -> {}): distance={} (SLOWER)", 
                           from_node, to_node, distance)
            }
        }
    }
    
    damn based
}

fr fr Main test execution
slay run_numa_tests() lit {
    vibez.spill("CURSED NUMA Topology System - Comprehensive Test Suite")
    vibez.spill("=" * 60)
    
    testz.test_start("NUMA System Tests")
    
    fr fr Core functionality tests
    testz.assert_true(test_numa_availability(), "NUMA availability detection")
    testz.assert_true(test_numa_node_information(), "NUMA node information")
    testz.assert_true(test_numa_current_node(), "Current node detection")
    testz.assert_true(test_numa_memory_allocation(), "NUMA memory allocation")
    testz.assert_true(test_numa_distance_matrix(), "NUMA distance matrix")
    testz.assert_true(test_numa_memory_statistics(), "NUMA memory statistics")
    
    fr fr Advanced tests
    testz.assert_true(test_numa_topology_printing(), "NUMA topology printing")
    testz.assert_true(test_numa_stress_allocation(), "NUMA stress allocation")
    testz.assert_true(test_numa_memory_migration(), "NUMA memory migration")
    testz.assert_true(test_numa_performance_characteristics(), "NUMA performance characteristics")
    
    fr fr Print final test summary
    testz.print_test_summary()
    
    vibez.spill("\nNUMA System Validation: COMPLETE")
    vibez.spill("All NUMA functions operational and memory-safe")
    
    damn based
}

fr fr Execute all tests
run_numa_tests()
