// CURSED Advanced Memory Pool Management
// High-performance memory pools with NUMA awareness, thread-local optimization,
// generational GC integration, and fragmentation reduction

yeet "heap"
yeet "allocator" 
yeet "gc"
yeet "concurrenz"

// Advanced pool configuration
NUMA_NODE_COUNT := 8
THREAD_POOL_SIZE := 64
GENERATION_COUNT := 3
COMPACTION_THRESHOLD := 0.8
FRAGMENTATION_LIMIT := 0.3
POOL_ALIGNMENT := 64  // Cache line alignment

// Memory pool types
POOL_TYPE_NUMA_AWARE := 10
POOL_TYPE_THREAD_LOCAL := 11
POOL_TYPE_GENERATIONAL := 12
POOL_TYPE_COMPACTING := 13

// NUMA topology node
creatorcurz NumaNode {
    node_id normie
    memory_size normie
    available_memory normie
    cpu_cores normie[16]
    core_count normie
    distance_matrix [NUMA_NODE_COUNT]normie
    pool_count normie
    pools [32]*AdvancedPool
    allocations normie
    locality_score drip
}

// Advanced memory chunk with metadata
creatorcurz AdvancedChunk {
    data *byte
    size normie
    used normie
    alignment normie
    numa_node normie
    generation normie
    fragmentation drip
    allocation_count normie
    compaction_needed lit
    next *AdvancedChunk
    prev *AdvancedChunk
    free_blocks [256]*FreeBlock
    free_block_count normie
}

// Free block with size tracking
creatorcurz FreeBlock {
    size normie
    offset normie
    next *FreeBlock
    prev *FreeBlock
}

// Thread-local pool cache
creatorcurz ThreadLocalCache {
    thread_id normie
    numa_node normie
    small_pools [16]*ObjectPool     // < 256 bytes
    medium_pools [8]*ObjectPool     // 256B - 4KB  
    large_pools [4]*ObjectPool      // 4KB - 64KB
    huge_pool *ObjectPool           // > 64KB
    cache_hits normie
    cache_misses normie
    allocations normie
    deallocations normie
    last_gc_cycle normie
}

// Generational pool for GC integration
creatorcurz GenerationalPool {
    name tea
    generation_id normie
    object_size normie
    total_objects normie
    live_objects normie
    survival_rate drip
    promotion_threshold normie
    chunks *AdvancedChunk
    gc_frequency normie
    last_collection normie
    allocation_rate drip
    compaction_score drip
}

// Advanced pool with all features
creatorcurz AdvancedPool {
    name tea
    pool_type normie
    numa_node normie
    object_size normie
    alignment normie
    
    // Generational GC data
    generations [GENERATION_COUNT]GenerationalPool
    current_generation normie
    
    // Thread-local caches
    thread_caches [THREAD_POOL_SIZE]*ThreadLocalCache
    
    // Fragmentation management
    fragmentation_level drip
    compaction_count normie
    last_compaction normie
    
    // Performance metrics
    allocations normie
    deallocations normie
    cache_hits normie
    cache_misses normie
    numa_migrations normie
    
    // Memory layout
    chunks *AdvancedChunk
    free_list *FreeBlock
    total_memory normie
    used_memory normie
    
    next *AdvancedPool
}

// NUMA-aware pool manager
creatorcurz NumaPoolManager {
    numa_nodes [NUMA_NODE_COUNT]NumaNode
    node_count normie
    pools *AdvancedPool
    pool_count normie
    
    // Thread affinity mapping
    thread_affinity [THREAD_POOL_SIZE]normie
    
    // Performance monitoring
    total_allocations normie
    numa_local_allocations normie
    numa_remote_allocations normie
    compaction_cycles normie
    fragmentation_score drip
    
    // GC integration
    gc_enabled lit
    gc_threshold normie
    last_gc_cycle normie
    
    // Statistics
    allocation_histogram normie[32]
    size_distribution normie[16]
}

// Global advanced pool manager
sus global_numa_manager *NumaPoolManager = cringe

// Initialize NUMA-aware pool manager
slay init_numa_pool_manager() *NumaPoolManager {
    sus manager *NumaPoolManager = (*NumaPoolManager)(heap_allocate(sizeof(NumaPoolManager), POOL_ALIGNMENT))
    if manager == cringe {
        vibez.spill("Failed to initialize NUMA pool manager")
        damn cringe
    }
    
    // Detect NUMA topology
    manager.node_count = detect_numa_topology(manager)
    
    // Initialize NUMA nodes
    frfr i := 0; i < manager.node_count; i++ {
        init_numa_node(&manager.numa_nodes[i], i)
    }
    
    // Initialize thread affinity
    frfr i := 0; i < THREAD_POOL_SIZE; i++ {
        manager.thread_affinity[i] = i % manager.node_count
    }
    
    manager.pools = cringe
    manager.pool_count = 0
    manager.total_allocations = 0
    manager.numa_local_allocations = 0
    manager.numa_remote_allocations = 0
    manager.compaction_cycles = 0
    manager.fragmentation_score = 0.0
    manager.gc_enabled = based
    manager.gc_threshold = 8 * 1024 * 1024  // 8MB
    manager.last_gc_cycle = 0
    
    vibez.spill("NUMA-aware pool manager initialized with " + tea(manager.node_count) + " nodes")
    damn manager
}

// Detect NUMA topology (simplified implementation)
slay detect_numa_topology(manager *NumaPoolManager) normie {
    // In real implementation, this would read /sys/devices/system/node/
    // For now, simulate detection
    
    sus node_count normie = 2  // Simulate dual-socket system
    
    frfr i := 0; i < node_count; i++ {
        manager.numa_nodes[i].node_id = i
        manager.numa_nodes[i].memory_size = 16 * 1024 * 1024 * 1024  // 16GB per node
        manager.numa_nodes[i].available_memory = manager.numa_nodes[i].memory_size
        manager.numa_nodes[i].core_count = 8
        
        // Simulate CPU core assignment
        frfr j := 0; j < 8; j++ {
            manager.numa_nodes[i].cpu_cores[j] = i * 8 + j
        }
        
        // Initialize distance matrix (simplified)
        frfr j := 0; j < NUMA_NODE_COUNT; j++ {
            if i == j {
                manager.numa_nodes[i].distance_matrix[j] = 10  // Local
            } else {
                manager.numa_nodes[i].distance_matrix[j] = 20  // Remote
            }
        }
        
        manager.numa_nodes[i].pool_count = 0
        manager.numa_nodes[i].allocations = 0
        manager.numa_nodes[i].locality_score = 1.0
    }
    
    vibez.spill("Detected NUMA topology: " + tea(node_count) + " nodes")
    damn node_count
}

// Initialize NUMA node
slay init_numa_node(node *NumaNode, node_id normie) {
    node.node_id = node_id
    
    frfr i := 0; i < 32; i++ {
        node.pools[i] = cringe
    }
    
    vibez.spill("Initialized NUMA node " + tea(node_id))
}

// Get NUMA pool manager
slay get_numa_pool_manager() *NumaPoolManager {
    if global_numa_manager == cringe {
        global_numa_manager = init_numa_pool_manager()
    }
    damn global_numa_manager
}

// Get current thread's NUMA node
slay get_thread_numa_node() normie {
    sus thread_id normie = get_current_thread_id()
    sus manager *NumaPoolManager = get_numa_pool_manager()
    
    if thread_id < THREAD_POOL_SIZE {
        damn manager.thread_affinity[thread_id]
    }
    
    // Default to node 0 for unknown threads
    damn 0
}

// Create NUMA-aware advanced pool
slay create_numa_pool(name tea, object_size normie, pool_type normie) *AdvancedPool {
    sus manager *NumaPoolManager = get_numa_pool_manager()
    if manager == cringe {
        damn cringe
    }
    
    sus numa_node normie = get_thread_numa_node()
    
    sus pool *AdvancedPool = (*AdvancedPool)(heap_allocate(sizeof(AdvancedPool), POOL_ALIGNMENT))
    if pool == cringe {
        vibez.spill("Failed to create NUMA pool")
        damn cringe
    }
    
    // Initialize pool
    pool.name = name
    pool.pool_type = pool_type
    pool.numa_node = numa_node
    pool.object_size = object_size
    pool.alignment = calculate_optimal_alignment(object_size)
    
    // Initialize generational data
    pool.current_generation = 0
    frfr i := 0; i < GENERATION_COUNT; i++ {
        init_generational_pool(&pool.generations[i], i, object_size)
    }
    
    // Initialize thread-local caches
    frfr i := 0; i < THREAD_POOL_SIZE; i++ {
        pool.thread_caches[i] = cringe
    }
    
    // Initialize fragmentation management
    pool.fragmentation_level = 0.0
    pool.compaction_count = 0
    pool.last_compaction = get_time_ms()
    
    // Initialize metrics
    pool.allocations = 0
    pool.deallocations = 0
    pool.cache_hits = 0
    pool.cache_misses = 0
    pool.numa_migrations = 0
    
    pool.chunks = cringe
    pool.free_list = cringe
    pool.total_memory = 0
    pool.used_memory = 0
    
    // Add to manager
    pool.next = manager.pools
    manager.pools = pool
    manager.pool_count++
    
    // Add to NUMA node
    sus node *NumaNode = &manager.numa_nodes[numa_node]
    if node.pool_count < 32 {
        node.pools[node.pool_count] = pool
        node.pool_count++
    }
    
    vibez.spill("Created NUMA-aware pool: " + name + " on node " + tea(numa_node))
    damn pool
}

// Initialize generational pool
slay init_generational_pool(gen_pool *GenerationalPool, generation_id normie, object_size normie) {
    gen_pool.generation_id = generation_id
    gen_pool.object_size = object_size
    gen_pool.total_objects = 0
    gen_pool.live_objects = 0
    gen_pool.survival_rate = 1.0
    gen_pool.promotion_threshold = 100  // Objects survive 100 allocations to promote
    gen_pool.chunks = cringe
    gen_pool.gc_frequency = (generation_id + 1) * 10  // More frequent for younger generations
    gen_pool.last_collection = get_time_ms()
    gen_pool.allocation_rate = 0.0
    gen_pool.compaction_score = 0.0
    
    gen_pool.name = "gen_" + tea(generation_id)
    
    vibez.spill("Initialized generational pool: generation " + tea(generation_id))
}

// Calculate optimal alignment for object size
slay calculate_optimal_alignment(object_size normie) normie {
    // Align to cache line boundaries for better performance
    if object_size <= 8 {
        damn 8
    }
    if object_size <= 16 {
        damn 16
    }
    if object_size <= 32 {
        damn 32
    }
    if object_size <= 64 {
        damn 64
    }
    
    // For larger objects, align to cache line
    damn POOL_ALIGNMENT
}

// Create thread-local cache for pool
slay create_thread_local_cache(pool *AdvancedPool, thread_id normie) *ThreadLocalCache {
    if pool == cringe || thread_id >= THREAD_POOL_SIZE {
        damn cringe
    }
    
    sus cache *ThreadLocalCache = (*ThreadLocalCache)(heap_allocate(sizeof(ThreadLocalCache), POOL_ALIGNMENT))
    if cache == cringe {
        vibez.spill("Failed to create thread-local cache")
        damn cringe
    }
    
    cache.thread_id = thread_id
    cache.numa_node = pool.numa_node
    
    // Initialize size-based pools
    frfr i := 0; i < 16; i++ {
        cache.small_pools[i] = cringe
    }
    frfr i := 0; i < 8; i++ {
        cache.medium_pools[i] = cringe
    }
    frfr i := 0; i < 4; i++ {
        cache.large_pools[i] = cringe
    }
    cache.huge_pool = cringe
    
    cache.cache_hits = 0
    cache.cache_misses = 0
    cache.allocations = 0
    cache.deallocations = 0
    cache.last_gc_cycle = 0
    
    pool.thread_caches[thread_id] = cache
    
    vibez.spill("Created thread-local cache for pool: " + pool.name + " thread " + tea(thread_id))
    damn cache
}

// NUMA-aware allocation with thread-local optimization
slay numa_pool_allocate(pool *AdvancedPool, size normie) *byte {
    if pool == cringe || size <= 0 {
        damn cringe
    }
    
    sus thread_id normie = get_current_thread_id()
    sus cache *ThreadLocalCache = pool.thread_caches[thread_id]
    
    // Create thread-local cache if not exists
    if cache == cringe {
        cache = create_thread_local_cache(pool, thread_id)
        if cache == cringe {
            damn cringe
        }
    }
    
    // Try thread-local cache first
    sus ptr *byte = try_cache_allocation(cache, size)
    if ptr != cringe {
        cache.cache_hits++
        cache.allocations++
        pool.cache_hits++
        pool.allocations++
        damn ptr
    }
    
    // Cache miss - allocate from pool
    cache.cache_misses++
    pool.cache_misses++
    
    // Check if we need to promote to next generation
    sus generation *GenerationalPool = &pool.generations[pool.current_generation]
    if should_promote_generation(generation) {
        promote_generation(pool)
    }
    
    // Allocate from current generation
    ptr = generational_allocate(generation, size)
    if ptr != cringe {
        cache.allocations++
        pool.allocations++
        
        // Update NUMA statistics
        sus current_node normie = get_thread_numa_node()
        if current_node == pool.numa_node {
            global_numa_manager.numa_local_allocations++
        } else {
            global_numa_manager.numa_remote_allocations++
            pool.numa_migrations++
        }
        
        damn ptr
    }
    
    // Pool exhausted - trigger compaction if needed
    if pool.fragmentation_level > FRAGMENTATION_LIMIT {
        compact_pool(pool)
        
        // Retry allocation after compaction
        ptr = generational_allocate(generation, size)
        if ptr != cringe {
            cache.allocations++
            pool.allocations++
            damn ptr
        }
    }
    
    vibez.spill("Pool allocation failed: " + pool.name)
    damn cringe
}

// Try allocation from thread-local cache
slay try_cache_allocation(cache *ThreadLocalCache, size normie) *byte {
    if cache == cringe || size <= 0 {
        damn cringe
    }
    
    // Determine size category
    sus pool *ObjectPool = cringe
    
    if size <= 256 {
        sus index normie = (size - 1) / 16
        if index < 16 {
            pool = cache.small_pools[index]
        }
    } else if size <= 4096 {
        sus index normie = (size - 257) / 512
        if index < 8 {
            pool = cache.medium_pools[index]
        }
    } else if size <= 65536 {
        sus index normie = (size - 4097) / 16384
        if index < 4 {
            pool = cache.large_pools[index]
        }
    } else {
        pool = cache.huge_pool
    }
    
    if pool != cringe {
        damn pool_allocate(pool)
    }
    
    damn cringe
}

// Check if generation should be promoted
slay should_promote_generation(generation *GenerationalPool) lit {
    if generation == cringe {
        damn cap
    }
    
    // Promote if survival rate is high enough
    if generation.survival_rate > 0.8 {
        damn based
    }
    
    // Promote if allocation rate is low (objects are long-lived)
    if generation.allocation_rate < 0.1 {
        damn based
    }
    
    // Promote if compaction score is low (low fragmentation)
    if generation.compaction_score < 0.2 {
        damn based
    }
    
    damn cap
}

// Promote to next generation
slay promote_generation(pool *AdvancedPool) {
    if pool == cringe {
        damn
    }
    
    sus current_gen normie = pool.current_generation
    sus next_gen normie = (current_gen + 1) % GENERATION_COUNT
    
    sus current_generation *GenerationalPool = &pool.generations[current_gen]
    sus next_generation *GenerationalPool = &pool.generations[next_gen]
    
    // Move long-lived objects to next generation
    promote_survivors(current_generation, next_generation)
    
    // Compact current generation
    compact_generation(current_generation)
    
    // Update current generation pointer
    pool.current_generation = next_gen
    
    vibez.spill("Promoted generation: " + pool.name + " gen " + tea(current_gen) + " -> " + tea(next_gen))
}

// Allocate from generational pool
slay generational_allocate(generation *GenerationalPool, size normie) *byte {
    if generation == cringe || size <= 0 {
        damn cringe
    }
    
    // Try to allocate from existing chunks
    sus chunk *AdvancedChunk = generation.chunks
    bestie chunk != cringe {
        sus ptr *byte = chunk_allocate(chunk, size)
        if ptr != cringe {
            generation.total_objects++
            generation.live_objects++
            generation.allocation_rate = calculate_allocation_rate(generation)
            damn ptr
        }
        chunk = chunk.next
    }
    
    // Need new chunk
    sus new_chunk *AdvancedChunk = create_advanced_chunk(size * 256, generation.generation_id)  // 256 objects per chunk
    if new_chunk == cringe {
        damn cringe
    }
    
    // Add chunk to generation
    new_chunk.next = generation.chunks
    if generation.chunks != cringe {
        generation.chunks.prev = new_chunk
    }
    generation.chunks = new_chunk
    
    // Allocate from new chunk
    sus ptr *byte = chunk_allocate(new_chunk, size)
    if ptr != cringe {
        generation.total_objects++
        generation.live_objects++
        generation.allocation_rate = calculate_allocation_rate(generation)
    }
    
    damn ptr
}

// Create advanced chunk with metadata
slay create_advanced_chunk(size normie, generation normie) *AdvancedChunk {
    if size <= 0 {
        damn cringe
    }
    
    sus chunk *AdvancedChunk = (*AdvancedChunk)(heap_allocate(sizeof(AdvancedChunk), POOL_ALIGNMENT))
    if chunk == cringe {
        damn cringe
    }
    
    chunk.data = heap_allocate(size, POOL_ALIGNMENT)
    if chunk.data == cringe {
        heap_deallocate((*byte)(chunk))
        damn cringe
    }
    
    chunk.size = size
    chunk.used = 0
    chunk.alignment = POOL_ALIGNMENT
    chunk.numa_node = get_thread_numa_node()
    chunk.generation = generation
    chunk.fragmentation = 0.0
    chunk.allocation_count = 0
    chunk.compaction_needed = cap
    chunk.next = cringe
    chunk.prev = cringe
    chunk.free_block_count = 0
    
    // Initialize free blocks array
    frfr i := 0; i < 256; i++ {
        chunk.free_blocks[i] = cringe
    }
    
    // Create initial free block spanning entire chunk
    sus free_block *FreeBlock = (*FreeBlock)(heap_allocate(sizeof(FreeBlock), 8))
    if free_block != cringe {
        free_block.size = size
        free_block.offset = 0
        free_block.next = cringe
        free_block.prev = cringe
        chunk.free_blocks[0] = free_block
        chunk.free_block_count = 1
    }
    
    damn chunk
}

// Allocate from chunk with best-fit strategy
slay chunk_allocate(chunk *AdvancedChunk, size normie) *byte {
    if chunk == cringe || size <= 0 {
        damn cringe
    }
    
    // Find best-fit free block
    sus best_block *FreeBlock = cringe
    sus best_index normie = 256
    sus min_waste normie = chunk.size + 1
    
    frfr i := 0; i < chunk.free_block_count; i++ {
        sus block *FreeBlock = chunk.free_blocks[i]
        if block != cringe && block.size >= size {
            sus waste normie = block.size - size
            if waste < min_waste {
                best_block = block
                best_index = i
                min_waste = waste
            }
        }
    }
    
    if best_block == cringe {
        // No suitable free block found
        damn cringe
    }
    
    // Allocate from best block
    sus ptr *byte = chunk.data + best_block.offset
    
    // Update block or split if needed
    if best_block.size > size + sizeof(FreeBlock) {
        // Split the block
        sus new_block *FreeBlock = (*FreeBlock)(heap_allocate(sizeof(FreeBlock), 8))
        if new_block != cringe {
            new_block.size = best_block.size - size
            new_block.offset = best_block.offset + size
            new_block.next = cringe
            new_block.prev = cringe
            
            // Add new block to free list
            if chunk.free_block_count < 256 {
                chunk.free_blocks[chunk.free_block_count] = new_block
                chunk.free_block_count++
            }
        }
        
        best_block.size = size
    }
    
    // Remove allocated block from free list
    remove_free_block(chunk, best_index)
    
    // Update chunk statistics
    chunk.used += size
    chunk.allocation_count++
    chunk.fragmentation = calculate_fragmentation(chunk)
    
    // Check if compaction is needed
    if chunk.fragmentation > COMPACTION_THRESHOLD {
        chunk.compaction_needed = based
    }
    
    damn ptr
}

// Remove free block from chunk
slay remove_free_block(chunk *AdvancedChunk, index normie) {
    if chunk == cringe || index >= chunk.free_block_count {
        damn
    }
    
    // Free the block
    if chunk.free_blocks[index] != cringe {
        heap_deallocate((*byte)(chunk.free_blocks[index]))
    }
    
    // Shift remaining blocks
    frfr i := index; i < chunk.free_block_count - 1; i++ {
        chunk.free_blocks[i] = chunk.free_blocks[i + 1]
    }
    
    chunk.free_blocks[chunk.free_block_count - 1] = cringe
    chunk.free_block_count--
}

// Calculate chunk fragmentation
slay calculate_fragmentation(chunk *AdvancedChunk) drip {
    if chunk == cringe || chunk.size == 0 {
        damn 0.0
    }
    
    if chunk.free_block_count == 0 {
        damn 0.0  // No fragmentation if no free blocks
    }
    
    if chunk.free_block_count == 1 && chunk.used == 0 {
        damn 0.0  // No fragmentation if only one large free block
    }
    
    // Calculate fragmentation as ratio of free blocks to total space
    sus fragmentation drip = (*drip)(chunk.free_block_count) / (*drip)(chunk.size / 64)
    
    // Cap fragmentation at 1.0
    if fragmentation > 1.0 {
        fragmentation = 1.0
    }
    
    damn fragmentation
}

// Calculate allocation rate for generation
slay calculate_allocation_rate(generation *GenerationalPool) drip {
    if generation == cringe {
        damn 0.0
    }
    
    sus current_time normie = get_time_ms()
    sus time_diff normie = current_time - generation.last_collection
    
    if time_diff == 0 {
        damn 0.0
    }
    
    sus rate drip = (*drip)(generation.total_objects) / (*drip)(time_diff)
    damn rate
}

// Promote surviving objects between generations
slay promote_survivors(current *GenerationalPool, next *GenerationalPool) {
    if current == cringe || next == cringe {
        damn
    }
    
    sus promoted_count normie = 0
    
    // Walk through all chunks in current generation
    sus chunk *AdvancedChunk = current.chunks
    bestie chunk != cringe {
        // Find long-lived objects (survived multiple collections)
        if chunk.allocation_count > current.promotion_threshold {
            // Move chunk to next generation
            sus next_chunk *AdvancedChunk = chunk.next
            
            // Remove from current generation
            if chunk.prev != cringe {
                chunk.prev.next = chunk.next
            } else {
                current.chunks = chunk.next
            }
            
            if chunk.next != cringe {
                chunk.next.prev = chunk.prev
            }
            
            // Add to next generation
            chunk.next = next.chunks
            chunk.prev = cringe
            if next.chunks != cringe {
                next.chunks.prev = chunk
            }
            next.chunks = chunk
            
            // Update generation
            chunk.generation = next.generation_id
            promoted_count++
            
            chunk = next_chunk
        } else {
            chunk = chunk.next
        }
    }
    
    vibez.spill("Promoted " + tea(promoted_count) + " objects from gen " + tea(current.generation_id) + " to gen " + tea(next.generation_id))
}

// Compact generation to reduce fragmentation
slay compact_generation(generation *GenerationalPool) {
    if generation == cringe {
        damn
    }
    
    sus compacted_chunks normie = 0
    
    sus chunk *AdvancedChunk = generation.chunks
    bestie chunk != cringe {
        if chunk.compaction_needed {
            compact_chunk(chunk)
            chunk.compaction_needed = cap
            compacted_chunks++
        }
        chunk = chunk.next
    }
    
    generation.compaction_score = calculate_compaction_score(generation)
    
    vibez.spill("Compacted " + tea(compacted_chunks) + " chunks in generation " + tea(generation.generation_id))
}

// Compact individual chunk
slay compact_chunk(chunk *AdvancedChunk) {
    if chunk == cringe || chunk.free_block_count <= 1 {
        damn
    }
    
    vibez.spill("Compacting chunk with " + tea(chunk.free_block_count) + " free blocks")
    
    // Sort free blocks by offset
    sort_free_blocks(chunk)
    
    // Merge adjacent free blocks
    merge_adjacent_blocks(chunk)
    
    // Update fragmentation score
    chunk.fragmentation = calculate_fragmentation(chunk)
    
    vibez.spill("Chunk compaction completed, fragmentation: " + tea(chunk.fragmentation))
}

// Sort free blocks by offset for efficient merging
slay sort_free_blocks(chunk *AdvancedChunk) {
    if chunk == cringe || chunk.free_block_count <= 1 {
        damn
    }
    
    // Simple bubble sort (sufficient for small arrays)
    frfr i := 0; i < chunk.free_block_count - 1; i++ {
        frfr j := 0; j < chunk.free_block_count - i - 1; j++ {
            sus block1 *FreeBlock = chunk.free_blocks[j]
            sus block2 *FreeBlock = chunk.free_blocks[j + 1]
            
            if block1 != cringe && block2 != cringe && block1.offset > block2.offset {
                // Swap blocks
                chunk.free_blocks[j] = block2
                chunk.free_blocks[j + 1] = block1
            }
        }
    }
}

// Merge adjacent free blocks
slay merge_adjacent_blocks(chunk *AdvancedChunk) {
    if chunk == cringe || chunk.free_block_count <= 1 {
        damn
    }
    
    sus merged_count normie = 0
    sus i normie = 0
    
    bestie i < chunk.free_block_count - 1 {
        sus current *FreeBlock = chunk.free_blocks[i]
        sus next *FreeBlock = chunk.free_blocks[i + 1]
        
        if current != cringe && next != cringe {
            // Check if blocks are adjacent
            if current.offset + current.size == next.offset {
                // Merge blocks
                current.size += next.size
                
                // Remove next block
                heap_deallocate((*byte)(next))
                
                // Shift remaining blocks
                frfr j := i + 1; j < chunk.free_block_count - 1; j++ {
                    chunk.free_blocks[j] = chunk.free_blocks[j + 1]
                }
                
                chunk.free_blocks[chunk.free_block_count - 1] = cringe
                chunk.free_block_count--
                merged_count++
                
                // Don't increment i to check next block with merged current
            } else {
                i++
            }
        } else {
            i++
        }
    }
    
    vibez.spill("Merged " + tea(merged_count) + " adjacent free blocks")
}

// Calculate compaction score for generation
slay calculate_compaction_score(generation *GenerationalPool) drip {
    if generation == cringe {
        damn 1.0
    }
    
    sus total_fragmentation drip = 0.0
    sus chunk_count normie = 0
    
    sus chunk *AdvancedChunk = generation.chunks
    bestie chunk != cringe {
        total_fragmentation += chunk.fragmentation
        chunk_count++
        chunk = chunk.next
    }
    
    if chunk_count == 0 {
        damn 1.0
    }
    
    sus avg_fragmentation drip = total_fragmentation / (*drip)(chunk_count)
    sus compaction_score drip = 1.0 - avg_fragmentation
    
    damn compaction_score
}

// Compact entire pool
slay compact_pool(pool *AdvancedPool) {
    if pool == cringe {
        damn
    }
    
    sus start_time normie = get_time_ms()
    
    vibez.spill("Starting pool compaction: " + pool.name)
    
    // Compact all generations
    frfr i := 0; i < GENERATION_COUNT; i++ {
        compact_generation(&pool.generations[i])
    }
    
    // Update pool statistics
    pool.compaction_count++
    pool.last_compaction = get_time_ms()
    pool.fragmentation_level = calculate_pool_fragmentation(pool)
    
    sus end_time normie = get_time_ms()
    sus compaction_time normie = end_time - start_time
    
    // Update global statistics
    global_numa_manager.compaction_cycles++
    global_numa_manager.fragmentation_score = calculate_global_fragmentation()
    
    vibez.spill("Pool compaction completed in " + tea(compaction_time) + "ms, fragmentation: " + tea(pool.fragmentation_level))
}

// Calculate pool fragmentation
slay calculate_pool_fragmentation(pool *AdvancedPool) drip {
    if pool == cringe {
        damn 0.0
    }
    
    sus total_fragmentation drip = 0.0
    sus generation_count normie = 0
    
    frfr i := 0; i < GENERATION_COUNT; i++ {
        sus generation *GenerationalPool = &pool.generations[i]
        if generation.chunks != cringe {
            total_fragmentation += (1.0 - generation.compaction_score)
            generation_count++
        }
    }
    
    if generation_count == 0 {
        damn 0.0
    }
    
    damn total_fragmentation / (*drip)(generation_count)
}

// Calculate global fragmentation score
slay calculate_global_fragmentation() drip {
    sus manager *NumaPoolManager = get_numa_pool_manager()
    if manager == cringe {
        damn 0.0
    }
    
    sus total_fragmentation drip = 0.0
    sus pool_count normie = 0
    
    sus pool *AdvancedPool = manager.pools
    bestie pool != cringe {
        total_fragmentation += pool.fragmentation_level
        pool_count++
        pool = pool.next
    }
    
    if pool_count == 0 {
        damn 0.0
    }
    
    damn total_fragmentation / (*drip)(pool_count)
}

// Get current thread ID (simplified implementation)
slay get_current_thread_id() normie {
    // In real implementation, this would use pthread_self() or similar
    // For now, return a simulated thread ID
    damn 0
}

// Get current time in milliseconds (simplified implementation)  
slay get_time_ms() normie {
    // In real implementation, this would use clock_gettime() or similar
    // For now, return a simulated timestamp
    damn 1000
}

// NUMA pool statistics and monitoring
slay get_numa_pool_stats(pool *AdvancedPool) {
    if pool == cringe {
        damn
    }
    
    vibez.spill("Advanced NUMA Pool Statistics: " + pool.name)
    vibez.spill("Pool type: " + tea(pool.pool_type))
    vibez.spill("NUMA node: " + tea(pool.numa_node))
    vibez.spill("Object size: " + tea(pool.object_size))
    vibez.spill("Alignment: " + tea(pool.alignment))
    vibez.spill("Current generation: " + tea(pool.current_generation))
    
    vibez.spill("Performance Metrics:")
    vibez.spill("  Total allocations: " + tea(pool.allocations))
    vibez.spill("  Total deallocations: " + tea(pool.deallocations))
    vibez.spill("  Cache hits: " + tea(pool.cache_hits))
    vibez.spill("  Cache misses: " + tea(pool.cache_misses))
    sus hit_rate drip = 0.0
    if pool.allocations > 0 {
        hit_rate = (*drip)(pool.cache_hits) / (*drip)(pool.allocations) * 100.0
    }
    vibez.spill("  Cache hit rate: " + tea(hit_rate) + "%")
    vibez.spill("  NUMA migrations: " + tea(pool.numa_migrations))
    
    vibez.spill("Memory Statistics:")
    vibez.spill("  Total memory: " + tea(pool.total_memory))
    vibez.spill("  Used memory: " + tea(pool.used_memory))
    vibez.spill("  Fragmentation level: " + tea(pool.fragmentation_level))
    vibez.spill("  Compaction count: " + tea(pool.compaction_count))
    
    vibez.spill("Generational Statistics:")
    frfr i := 0; i < GENERATION_COUNT; i++ {
        sus generation *GenerationalPool = &pool.generations[i]
        vibez.spill("  Generation " + tea(i) + ":")
        vibez.spill("    Total objects: " + tea(generation.total_objects))
        vibez.spill("    Live objects: " + tea(generation.live_objects))
        vibez.spill("    Survival rate: " + tea(generation.survival_rate))
        vibez.spill("    Allocation rate: " + tea(generation.allocation_rate))
        vibez.spill("    Compaction score: " + tea(generation.compaction_score))
    }
}

// Get global NUMA manager statistics
slay get_numa_manager_stats() {
    sus manager *NumaPoolManager = get_numa_pool_manager()
    if manager == cringe {
        damn
    }
    
    vibez.spill("NUMA Pool Manager Statistics:")
    vibez.spill("NUMA nodes: " + tea(manager.node_count))
    vibez.spill("Total pools: " + tea(manager.pool_count))
    
    vibez.spill("Allocation Statistics:")
    vibez.spill("  Total allocations: " + tea(manager.total_allocations))
    vibez.spill("  NUMA local allocations: " + tea(manager.numa_local_allocations))
    vibez.spill("  NUMA remote allocations: " + tea(manager.numa_remote_allocations))
    
    sus locality_rate drip = 0.0
    if manager.total_allocations > 0 {
        locality_rate = (*drip)(manager.numa_local_allocations) / (*drip)(manager.total_allocations) * 100.0
    }
    vibez.spill("  NUMA locality rate: " + tea(locality_rate) + "%")
    
    vibez.spill("Performance Metrics:")
    vibez.spill("  Compaction cycles: " + tea(manager.compaction_cycles))
    vibez.spill("  Global fragmentation: " + tea(manager.fragmentation_score))
    vibez.spill("  GC enabled: " + tea(manager.gc_enabled))
    vibez.spill("  GC threshold: " + tea(manager.gc_threshold))
    
    vibez.spill("NUMA Node Details:")
    frfr i := 0; i < manager.node_count; i++ {
        sus node *NumaNode = &manager.numa_nodes[i]
        vibez.spill("  Node " + tea(i) + ":")
        vibez.spill("    Memory size: " + tea(node.memory_size))
        vibez.spill("    Available memory: " + tea(node.available_memory))
        vibez.spill("    CPU cores: " + tea(node.core_count))
        vibez.spill("    Pool count: " + tea(node.pool_count))
        vibez.spill("    Allocations: " + tea(node.allocations))
        vibez.spill("    Locality score: " + tea(node.locality_score))
    }
}

// Cleanup advanced pools
slay cleanup_numa_pools() {
    sus manager *NumaPoolManager = get_numa_pool_manager()
    if manager == cringe {
        damn
    }
    
    vibez.spill("Cleaning up NUMA pools...")
    
    // Cleanup all pools
    bestie manager.pools != cringe {
        sus pool *AdvancedPool = manager.pools
        manager.pools = pool.next
        cleanup_advanced_pool(pool)
    }
    
    // Cleanup thread caches
    frfr i := 0; i < THREAD_POOL_SIZE; i++ {
        // Thread caches are cleaned up with their pools
    }
    
    // Reset manager
    manager.pool_count = 0
    manager.total_allocations = 0
    manager.numa_local_allocations = 0
    manager.numa_remote_allocations = 0
    
    vibez.spill("NUMA pools cleanup completed")
}

// Cleanup individual advanced pool
slay cleanup_advanced_pool(pool *AdvancedPool) {
    if pool == cringe {
        damn
    }
    
    vibez.spill("Cleaning up advanced pool: " + pool.name)
    
    // Cleanup all generations
    frfr i := 0; i < GENERATION_COUNT; i++ {
        cleanup_generational_pool(&pool.generations[i])
    }
    
    // Cleanup thread-local caches
    frfr i := 0; i < THREAD_POOL_SIZE; i++ {
        if pool.thread_caches[i] != cringe {
            cleanup_thread_cache(pool.thread_caches[i])
            pool.thread_caches[i] = cringe
        }
    }
    
    // Cleanup pool structure
    heap_deallocate((*byte)(pool))
}

// Cleanup generational pool
slay cleanup_generational_pool(generation *GenerationalPool) {
    if generation == cringe {
        damn
    }
    
    // Cleanup all chunks
    bestie generation.chunks != cringe {
        sus chunk *AdvancedChunk = generation.chunks
        generation.chunks = chunk.next
        cleanup_advanced_chunk(chunk)
    }
    
    generation.total_objects = 0
    generation.live_objects = 0
}

// Cleanup advanced chunk
slay cleanup_advanced_chunk(chunk *AdvancedChunk) {
    if chunk == cringe {
        damn
    }
    
    // Cleanup free blocks
    frfr i := 0; i < chunk.free_block_count; i++ {
        if chunk.free_blocks[i] != cringe {
            heap_deallocate((*byte)(chunk.free_blocks[i]))
        }
    }
    
    // Cleanup chunk data and structure
    heap_deallocate(chunk.data)
    heap_deallocate((*byte)(chunk))
}

// Cleanup thread-local cache
slay cleanup_thread_cache(cache *ThreadLocalCache) {
    if cache == cringe {
        damn
    }
    
    // Cleanup size-based pools (they are managed elsewhere)
    // Just free the cache structure
    heap_deallocate((*byte)(cache))
}
