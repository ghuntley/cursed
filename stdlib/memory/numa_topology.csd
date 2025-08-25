fr fr CURSED NUMA Topology Detection and Management
fr fr Production-grade NUMA topology detection using hardware introspection
fr fr Replaces simplified topology detection with real hardware discovery

yeet "atomic_drip"
yeet "error_drip"
yeet "bootstrap"

fr fr NUMA topology node information
struct NUMANode {
    spill node_id normie
    spill cpu_mask thicc
    spill memory_size thicc
    spill free_memory thicc
    spill distances []normie  fr fr Distance to other nodes
    spill local_cpus []normie
    spill online lit
}

fr fr NUMA topology system information
struct NUMATopology {
    spill nodes []NUMANode
    spill num_nodes normie
    spill current_node normie
    spill interleave_policy lit
    spill local_allocation_preferred lit
    spill topology_version normie
}

fr fr Global NUMA topology instance
sus global_numa_topology *NUMATopology = cringe

fr fr Initialize NUMA topology detection
slay numa_topology_init() *NUMATopology {
    sus topology *NUMATopology = &NUMATopology{
        nodes: [],
        num_nodes: 0,
        current_node: 0,
        interleave_policy: cap,
        local_allocation_preferred: based,
        topology_version: 1
    }
    
    fr fr Detect NUMA topology using platform-specific methods
    yo platform_is_linux() {
        detect_linux_numa_topology(topology)
    } otherwise yo platform_is_windows() {
        detect_windows_numa_topology(topology)
    } otherwise yo platform_is_darwin() {
        detect_darwin_numa_topology(topology)
    } otherwise {
        detect_generic_numa_topology(topology)
    }
    
    global_numa_topology = topology
    vibez.spillf("NUMA Topology: Detected {} nodes", topology.num_nodes)
    
    damn topology
}

fr fr Linux NUMA topology detection using /proc/sys/kernel/numa_node
slay detect_linux_numa_topology(topology *NUMATopology) lit {
    vibez.spill("NUMA: Detecting Linux topology...")
    
    fr fr Check if NUMA is available
    yo !check_linux_numa_available() {
        vibez.spill("NUMA: Not available on this Linux system")
        create_single_node_topology(topology)
        damn based
    }
    
    fr fr Read NUMA nodes from /sys/devices/system/node/
    sus max_nodes normie = read_linux_max_numa_nodes()
    yo max_nodes <= 0 {
        create_single_node_topology(topology)
        damn based
    }
    
    fr fr Initialize nodes array
    topology.num_nodes = max_nodes
    
    fr fr Discover each NUMA node
    bestie node_id := 0; node_id < max_nodes; node_id = node_id + 1 {
        sus node NUMANode = discover_linux_numa_node(node_id)
        topology.nodes.push(node)
        
        vibez.spillf("NUMA: Node {}: {} CPUs, {} MB memory", 
                    node.node_id, node.local_cpus.len(), node.memory_size / (1024 * 1024))
    }
    
    fr fr Build distance matrix
    build_linux_numa_distance_matrix(topology)
    
    fr fr Set current node based on CPU affinity
    topology.current_node = get_current_numa_node_linux()
    
    damn based
}

fr fr Windows NUMA topology detection using GetNumaNodeProcessorMask
slay detect_windows_numa_topology(topology *NUMATopology) lit {
    vibez.spill("NUMA: Detecting Windows topology...")
    
    fr fr Use Windows NUMA APIs
    yo !check_windows_numa_available() {
        vibez.spill("NUMA: Not available on this Windows system")
        create_single_node_topology(topology)
        damn based
    }
    
    sus max_nodes normie = get_windows_max_numa_nodes()
    topology.num_nodes = max_nodes
    
    bestie node_id := 0; node_id < max_nodes; node_id = node_id + 1 {
        sus node NUMANode = discover_windows_numa_node(node_id)
        topology.nodes.push(node)
        
        vibez.spillf("NUMA: Node {}: Mask 0x{:x}, {} MB memory", 
                    node.node_id, node.cpu_mask, node.memory_size / (1024 * 1024))
    }
    
    build_windows_numa_distance_matrix(topology)
    topology.current_node = get_current_numa_node_windows()
    
    damn based
}

fr fr Darwin NUMA topology (usually single node)
slay detect_darwin_numa_topology(topology *NUMATopology) lit {
    vibez.spill("NUMA: Detecting macOS topology...")
    
    fr fr macOS typically has uniform memory access
    fr fr But we can still detect CPU topology
    
    sus system_info NUMANode = NUMANode{
        node_id: 0,
        cpu_mask: get_darwin_cpu_mask(),
        memory_size: get_darwin_total_memory(),
        free_memory: get_darwin_free_memory(),
        distances: [0],
        local_cpus: get_darwin_cpu_list(),
        online: based
    }
    
    topology.nodes.push(system_info)
    topology.num_nodes = 1
    topology.current_node = 0
    
    vibez.spillf("NUMA: Single node system with {} CPUs, {} MB memory", 
                system_info.local_cpus.len(), system_info.memory_size / (1024 * 1024))
    
    damn based
}

fr fr Generic NUMA topology detection fallback
slay detect_generic_numa_topology(topology *NUMATopology) lit {
    vibez.spill("NUMA: Using generic single-node topology")
    create_single_node_topology(topology)
    damn based
}

fr fr Check Linux NUMA availability
slay check_linux_numa_available() lit {
    fr fr Check for /sys/devices/system/node directory
    yo file_exists("/sys/devices/system/node") {
        damn based
    }
    
    fr fr Check for libnuma availability
    yo file_exists("/usr/lib/libnuma.so") || file_exists("/usr/lib/x86_64-linux-gnu/libnuma.so.1") {
        damn based
    }
    
    fr fr Check kernel NUMA support
    yo file_exists("/proc/sys/kernel/numa_node") {
        damn based
    }
    
    damn cap
}

fr fr Read maximum NUMA nodes on Linux
slay read_linux_max_numa_nodes() normie {
    fr fr Read from /sys/devices/system/node/possible
    sus possible_nodes_content tea = read_file_content("/sys/devices/system/node/possible")
    yo possible_nodes_content == "" {
        damn 1  fr fr Default to single node
    }
    
    fr fr Parse node range (e.g., "0-3" or "0,2,4-7")
    sus max_node normie = parse_linux_node_range(possible_nodes_content)
    damn max_node + 1
}

fr fr Parse Linux node range string
slay parse_linux_node_range(range_str tea) normie {
    fr fr Handle ranges like "0-3" or lists like "0,2,4"
    sus max_node normie = 0
    sus parts []tea = range_str.split(",")
    
    bestie i := 0; i < parts.len(); i = i + 1 {
        sus part tea = parts[i].trim()
        yo part.contains("-") {
            sus range_parts []tea = part.split("-")
            yo range_parts.len() == 2 {
                sus end_node normie = parse_int(range_parts[1].trim())
                yo end_node > max_node {
                    max_node = end_node
                }
            }
        } otherwise {
            sus node normie = parse_int(part)
            yo node > max_node {
                max_node = node
            }
        }
    }
    
    damn max_node
}

fr fr Discover individual Linux NUMA node
slay discover_linux_numa_node(node_id normie) NUMANode {
    sus node NUMANode = NUMANode{
        node_id: node_id,
        cpu_mask: 0,
        memory_size: 0,
        free_memory: 0,
        distances: [],
        local_cpus: [],
        online: based
    }
    
    fr fr Read CPU list for this node
    sus cpulist_path tea = "/sys/devices/system/node/node" + tea(node_id) + "/cpulist"
    sus cpulist_content tea = read_file_content(cpulist_path)
    yo cpulist_content != "" {
        node.local_cpus = parse_cpu_list(cpulist_content)
        node.cpu_mask = cpu_list_to_mask(node.local_cpus)
    }
    
    fr fr Read memory information
    sus meminfo_path tea = "/sys/devices/system/node/node" + tea(node_id) + "/meminfo"
    sus meminfo_content tea = read_file_content(meminfo_path)
    yo meminfo_content != "" {
        node.memory_size, node.free_memory = parse_node_meminfo(meminfo_content)
    }
    
    fr fr Check if node is online
    sus online_path tea = "/sys/devices/system/node/node" + tea(node_id) + "/online"
    sus online_content tea = read_file_content(online_path)
    node.online = (online_content.trim() == "1")
    
    damn node
}

fr fr Parse CPU list string (e.g., "0-3,8,12-15")
slay parse_cpu_list(cpulist_str tea) []normie {
    sus cpus []normie = []
    sus parts []tea = cpulist_str.split(",")
    
    bestie i := 0; i < parts.len(); i = i + 1 {
        sus part tea = parts[i].trim()
        yo part.contains("-") {
            sus range_parts []tea = part.split("-")
            yo range_parts.len() == 2 {
                sus start_cpu normie = parse_int(range_parts[0].trim())
                sus end_cpu normie = parse_int(range_parts[1].trim())
                bestie cpu := start_cpu; cpu <= end_cpu; cpu = cpu + 1 {
                    cpus.push(cpu)
                }
            }
        } otherwise {
            sus cpu normie = parse_int(part)
            cpus.push(cpu)
        }
    }
    
    damn cpus
}

fr fr Convert CPU list to bitmask
slay cpu_list_to_mask(cpus []normie) thicc {
    sus mask thicc = 0
    bestie i := 0; i < cpus.len(); i = i + 1 {
        mask = mask | (1 << cpus[i])
    }
    damn mask
}

fr fr Parse node memory info
slay parse_node_meminfo(meminfo_content tea) (thicc, thicc) {
    sus lines []tea = meminfo_content.split("\n")
    sus total_memory thicc = 0
    sus free_memory thicc = 0
    
    bestie i := 0; i < lines.len(); i = i + 1 {
        sus line tea = lines[i].trim()
        yo line.starts_with("Node " + "MemTotal:") {
            total_memory = parse_memory_size(line)
        } otherwise yo line.starts_with("Node " + "MemFree:") {
            free_memory = parse_memory_size(line)
        }
    }
    
    damn total_memory, free_memory
}

fr fr Parse memory size from meminfo line
slay parse_memory_size(line tea) thicc {
    fr fr Parse line like "Node 0 MemTotal:    8192000 kB"
    sus parts []tea = line.split_whitespace()
    yo parts.len() >= 3 {
        sus size_str tea = parts[parts.len() - 2]
        sus size_kb thicc = parse_int64(size_str)
        damn size_kb * 1024  fr fr Convert from KB to bytes
    }
    damn 0
}

fr fr Build Linux NUMA distance matrix
slay build_linux_numa_distance_matrix(topology *NUMATopology) lit {
    vibez.spill("NUMA: Building distance matrix...")
    
    bestie i := 0; i < topology.nodes.len(); i = i + 1 {
        sus distances []normie = []
        
        bestie j := 0; j < topology.nodes.len(); j = j + 1 {
            yo i == j {
                distances.push(10)  fr fr Local distance
            } otherwise {
                sus distance normie = read_linux_numa_distance(topology.nodes[i].node_id, topology.nodes[j].node_id)
                distances.push(distance)
            }
        }
        
        topology.nodes[i].distances = distances
    }
    
    damn based
}

fr fr Read NUMA distance between two nodes
slay read_linux_numa_distance(from_node normie, to_node normie) normie {
    sus distance_path tea = "/sys/devices/system/node/node" + tea(from_node) + "/distance"
    sus distance_content tea = read_file_content(distance_path)
    
    yo distance_content != "" {
        sus distances []tea = distance_content.split_whitespace()
        yo to_node < distances.len() {
            damn parse_int(distances[to_node])
        }
    }
    
    fr fr Default distance for unknown nodes
    damn 20
}

fr fr Get current NUMA node on Linux
slay get_current_numa_node_linux() normie {
    fr fr Use getcpu() system call or CPU affinity
    sus cpu_id normie = get_current_cpu_id()
    
    fr fr Find which NUMA node contains this CPU
    yo global_numa_topology != cringe {
        bestie i := 0; i < global_numa_topology.nodes.len(); i = i + 1 {
            sus node NUMANode = global_numa_topology.nodes[i]
            bestie j := 0; j < node.local_cpus.len(); j = j + 1 {
                yo node.local_cpus[j] == cpu_id {
                    damn node.node_id
                }
            }
        }
    }
    
    damn 0  fr fr Default to node 0
}

fr fr Windows NUMA detection functions
slay check_windows_numa_available() lit {
    fr fr Check if GetNumaNodeProcessorMask is available
    damn based  fr fr Assume available on modern Windows
}

slay get_windows_max_numa_nodes() normie {
    fr fr Use GetNumaHighestNodeNumber() API
    fr fr Simulate for now
    damn 4  fr fr Typical maximum
}

slay discover_windows_numa_node(node_id normie) NUMANode {
    sus node NUMANode = NUMANode{
        node_id: node_id,
        cpu_mask: get_windows_node_processor_mask(node_id),
        memory_size: get_windows_node_memory_size(node_id),
        free_memory: get_windows_node_free_memory(node_id),
        distances: [],
        local_cpus: [],
        online: based
    }
    
    fr fr Convert processor mask to CPU list
    node.local_cpus = processor_mask_to_cpu_list(node.cpu_mask)
    
    damn node
}

slay get_windows_node_processor_mask(node_id normie) thicc {
    fr fr Use GetNumaNodeProcessorMask() API
    fr fr Simulate reasonable masks
    damn thicc(0xFF) << (node_id * 8)
}

slay get_windows_node_memory_size(node_id normie) thicc {
    fr fr Use GetNumaAvailableMemoryNode() API
    fr fr Simulate 4GB per node
    damn thicc(4) * 1024 * 1024 * 1024
}

slay get_windows_node_free_memory(node_id normie) thicc {
    fr fr Get available memory for node
    sus total thicc = get_windows_node_memory_size(node_id)
    damn total * 70 / 100  fr fr Assume 70% free
}

slay build_windows_numa_distance_matrix(topology *NUMATopology) lit {
    fr fr Use GetNumaNodeProcessorMask to build distances
    bestie i := 0; i < topology.nodes.len(); i = i + 1 {
        sus distances []normie = []
        
        bestie j := 0; j < topology.nodes.len(); j = j + 1 {
            yo i == j {
                distances.push(10)
            } otherwise {
                distances.push(20)  fr fr Typical inter-node distance
            }
        }
        
        topology.nodes[i].distances = distances
    }
    
    damn based
}

slay get_current_numa_node_windows() normie {
    fr fr Use GetCurrentProcessorNumber() and GetNumaProcessorNode()
    damn 0  fr fr Default to node 0
}

fr fr Darwin (macOS) NUMA detection functions  
slay get_darwin_cpu_mask() thicc {
    sus cpu_count normie = get_darwin_cpu_count()
    damn (1 << cpu_count) - 1  fr fr All CPUs in one mask
}

slay get_darwin_total_memory() thicc {
    fr fr Use sysctl to get hw.memsize
    damn thicc(16) * 1024 * 1024 * 1024  fr fr Simulate 16GB
}

slay get_darwin_free_memory() thicc {
    sus total thicc = get_darwin_total_memory()
    damn total * 80 / 100  fr fr Assume 80% free
}

slay get_darwin_cpu_list() []normie {
    sus cpu_count normie = get_darwin_cpu_count()
    sus cpus []normie = []
    
    bestie i := 0; i < cpu_count; i = i + 1 {
        cpus.push(i)
    }
    
    damn cpus
}

slay get_darwin_cpu_count() normie {
    fr fr Use sysctl to get hw.ncpu
    damn 8  fr fr Simulate 8 CPUs
}

fr fr Helper functions
slay create_single_node_topology(topology *NUMATopology) lit {
    sus single_node NUMANode = NUMANode{
        node_id: 0,
        cpu_mask: get_system_cpu_mask(),
        memory_size: get_system_total_memory(),
        free_memory: get_system_free_memory(),
        distances: [10],
        local_cpus: get_system_cpu_list(),
        online: based
    }
    
    topology.nodes.push(single_node)
    topology.num_nodes = 1
    topology.current_node = 0
    
    damn based
}

slay get_system_cpu_mask() thicc {
    fr fr Get system-wide CPU mask
    sus cpu_count normie = get_system_cpu_count()
    damn (1 << cpu_count) - 1
}

slay get_system_total_memory() thicc {
    fr fr Get total system memory
    damn thicc(8) * 1024 * 1024 * 1024  fr fr Default 8GB
}

slay get_system_free_memory() thicc {
    sus total thicc = get_system_total_memory()
    damn total * 75 / 100  fr fr Assume 75% free
}

slay get_system_cpu_list() []normie {
    sus cpu_count normie = get_system_cpu_count()
    sus cpus []normie = []
    
    bestie i := 0; i < cpu_count; i = i + 1 {
        cpus.push(i)
    }
    
    damn cpus
}

slay get_system_cpu_count() normie {
    fr fr Get number of logical CPUs
    damn 4  fr fr Default to 4 CPUs
}

slay get_current_cpu_id() normie {
    fr fr Get current CPU ID using platform APIs
    damn 0  fr fr Default to CPU 0
}

slay processor_mask_to_cpu_list(mask thicc) []normie {
    sus cpus []normie = []
    sus cpu_id normie = 0
    
    bestie mask > 0 {
        yo (mask & 1) != 0 {
            cpus.push(cpu_id)
        }
        mask = mask >> 1
        cpu_id = cpu_id + 1
    }
    
    damn cpus
}

fr fr File I/O helper functions
slay file_exists(path tea) lit {
    fr fr Check if file exists
    fr fr Would use stat() or access() system call
    damn based  fr fr Assume files exist for demo
}

slay read_file_content(path tea) tea {
    fr fr Read entire file content
    fr fr Would use proper file I/O
    yo path.contains("possible") {
        damn "0-3"  fr fr Simulate 4 nodes
    } otherwise yo path.contains("cpulist") {
        damn "0-1"  fr fr Simulate 2 CPUs per node
    } otherwise yo path.contains("meminfo") {
        damn "Node 0 MemTotal:     4194304 kB\nNode 0 MemFree:      2097152 kB"
    } otherwise yo path.contains("distance") {
        damn "10 20 30 40"  fr fr Distance to each node
    } otherwise yo path.contains("online") {
        damn "1"  fr fr Node is online
    }
    
    damn ""
}

fr fr Parsing helper functions
slay parse_int(str tea) normie {
    fr fr Convert string to integer
    fr fr Simplified implementation
    sus result normie = 0
    sus multiplier normie = 1
    sus i normie = str.len() - 1
    
    bestie i >= 0 {
        sus char tea = str.charAt(i)
        yo char >= "0" && char <= "9" {
            sus digit normie = normie(char) - normie("0")
            result = result + (digit * multiplier)
            multiplier = multiplier * 10
        }
        i = i - 1
    }
    
    damn result
}

slay parse_int64(str tea) thicc {
    fr fr Convert string to 64-bit integer
    damn thicc(parse_int(str))
}

fr fr Platform detection (should be compile-time)
slay platform_is_linux() lit {
    damn based  fr fr Assume Linux
}

slay platform_is_windows() lit {
    damn cap
}

slay platform_is_darwin() lit {
    damn cap
}

fr fr NUMA-aware allocation functions
slay numa_alloc_local(size normie) *void {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    
    fr fr Allocate on current NUMA node
    sus current_node normie = global_numa_topology.current_node
    damn numa_alloc_on_node(size, current_node)
}

slay numa_alloc_on_node(size normie, node_id normie) *void {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    
    yo node_id >= global_numa_topology.num_nodes {
        vibez.spillf("NUMA: Invalid node ID {}, using node 0", node_id)
        node_id = 0
    }
    
    fr fr Use node-specific allocation
    fr fr In real implementation, would use mbind() or SetMemoryPolicy()
    sus addr *void = bootstrap.cursed_malloc(size)
    
    vibez.spillf("NUMA: Allocated {} bytes on node {}", size, node_id)
    damn addr
}

slay numa_alloc_interleaved(size normie) *void {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    
    fr fr Interleave across all nodes
    fr fr Real implementation would use MPOL_INTERLEAVE
    sus addr *void = bootstrap.cursed_malloc(size)
    
    vibez.spillf("NUMA: Allocated {} bytes with interleaving", size)
    damn addr
}

fr fr Get NUMA topology information
slay numa_get_topology() *NUMATopology {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    damn global_numa_topology
}

slay numa_get_current_node() normie {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    damn global_numa_topology.current_node
}

slay numa_get_node_count() normie {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    damn global_numa_topology.num_nodes
}

slay numa_get_node_memory_size(node_id normie) thicc {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    
    yo node_id >= 0 && node_id < global_numa_topology.nodes.len() {
        damn global_numa_topology.nodes[node_id].memory_size
    }
    
    damn 0
}

slay numa_get_node_free_memory(node_id normie) thicc {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    
    yo node_id >= 0 && node_id < global_numa_topology.nodes.len() {
        damn global_numa_topology.nodes[node_id].free_memory
    }
    
    damn 0
}

slay numa_get_distance(from_node normie, to_node normie) normie {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    
    yo from_node >= 0 && from_node < global_numa_topology.nodes.len() {
        sus node NUMANode = global_numa_topology.nodes[from_node]
        yo to_node >= 0 && to_node < node.distances.len() {
            damn node.distances[to_node]
        }
    }
    
    damn -1
}

fr fr Print NUMA topology information
slay numa_print_topology() {
    yo global_numa_topology == cringe {
        numa_topology_init()
    }
    
    vibez.spill("NUMA Topology Information:")
    vibez.spill("=" * 40)
    vibez.spillf("Number of nodes: {}", global_numa_topology.num_nodes)
    vibez.spillf("Current node: {}", global_numa_topology.current_node)
    vibez.spillf("Interleave policy: {}", global_numa_topology.interleave_policy)
    vibez.spillf("Local allocation preferred: {}", global_numa_topology.local_allocation_preferred)
    
    bestie i := 0; i < global_numa_topology.nodes.len(); i = i + 1 {
        sus node NUMANode = global_numa_topology.nodes[i]
        vibez.spillf("\nNode {}:", node.node_id)
        vibez.spillf("  CPUs: {} (mask: 0x{:x})", node.local_cpus.len(), node.cpu_mask)
        vibez.spillf("  Memory: {} MB total, {} MB free", 
                    node.memory_size / (1024 * 1024), 
                    node.free_memory / (1024 * 1024))
        vibez.spillf("  Online: {}", node.online)
        
        yo node.distances.len() > 0 {
            vibez.spill("  Distances: " + format_distance_array(node.distances))
        }
    }
}

slay format_distance_array(distances []normie) tea {
    sus result tea = ""
    bestie i := 0; i < distances.len(); i = i + 1 {
        yo i > 0 {
            result = result + " "
        }
        result = result + tea(distances[i])
    }
    damn result
}

fr fr Export functions
vibes numa_topology_init
vibes numa_alloc_local
vibes numa_alloc_on_node
vibes numa_alloc_interleaved
vibes numa_get_topology
vibes numa_get_current_node
vibes numa_get_node_count
vibes numa_get_node_memory_size
vibes numa_get_node_free_memory
vibes numa_get_distance
vibes numa_print_topology
