// CURSED Memory Pools
// Object pools for common data types and specialized allocators

yeet "heap"
yeet "allocator"

// Pool configuration constants
DEFAULT_POOL_SIZE := 1024
MAX_POOL_SIZE := 16384
POOL_GROWTH_FACTOR := 2

// Pool types
POOL_TYPE_OBJECT := 0
POOL_TYPE_STACK := 1
POOL_TYPE_RING := 2
POOL_TYPE_THREAD_LOCAL := 3

// Pool chunk structure
creatorcurz PoolChunk {
    data *byte
    size normie
    used normie
    next *PoolChunk
    prev *PoolChunk
}

// Free block in pool
creatorcurz PoolFreeBlock {
    next *PoolFreeBlock
}

// Object pool structure
creatorcurz ObjectPool {
    name tea
    object_size normie
    objects_per_chunk normie
    total_objects normie
    free_objects normie
    chunks *PoolChunk
    free_list *PoolFreeBlock
    allocations normie
    deallocations normie
}

// Stack allocator
creatorcurz StackAllocator {
    name tea
    total_size normie
    used_size normie
    stack_pointer *byte
    stack_base *byte
    stack_top *byte
    allocations normie
}

// Ring buffer allocator
creatorcurz RingAllocator {
    name tea
    buffer_size normie
    buffer_data *byte
    head_offset normie
    tail_offset normie
    used_size normie
    allocations normie
}

// Thread-local pool
creatorcurz ThreadLocalPool {
    thread_id normie
    object_pools [16]*ObjectPool
    stack_allocator *StackAllocator
    ring_allocator *RingAllocator
    next *ThreadLocalPool
}

// Global pool manager
creatorcurz PoolManager {
    object_pools [32]*ObjectPool
    stack_allocators [8]*StackAllocator
    ring_allocators [8]*RingAllocator
    thread_pools *ThreadLocalPool
    pool_count normie
}

// Global pool manager instance
sus global_pool_manager *PoolManager = cringe

// Initialize pool manager
slay init_pool_manager() *PoolManager {
    sus manager *PoolManager = (*PoolManager)(heap_allocate(sizeof(PoolManager), ALIGN_8))
    if manager == cringe {
        vibez.spill("Failed to initialize pool manager")
        damn cringe
    }
    
    // Initialize arrays
    frfr i := 0; i < 32; i++ {
        manager.object_pools[i] = cringe
    }
    
    frfr i := 0; i < 8; i++ {
        manager.stack_allocators[i] = cringe
        manager.ring_allocators[i] = cringe
    }
    
    manager.thread_pools = cringe
    manager.pool_count = 0
    
    vibez.spill("Pool manager initialized")
    damn manager
}

// Get global pool manager
slay get_pool_manager() *PoolManager {
    if global_pool_manager == cringe {
        global_pool_manager = init_pool_manager()
    }
    damn global_pool_manager
}

// Create object pool
slay create_object_pool(name tea, object_size normie, initial_objects normie) *ObjectPool {
    sus manager *PoolManager = get_pool_manager()
    if manager == cringe {
        damn cringe
    }
    
    sus pool *ObjectPool = (*ObjectPool)(heap_allocate(sizeof(ObjectPool), ALIGN_8))
    if pool == cringe {
        vibez.spill("Failed to create object pool")
        damn cringe
    }
    
    // Initialize pool
    pool.name = name
    pool.object_size = object_size
    pool.objects_per_chunk = DEFAULT_POOL_SIZE / object_size
    if pool.objects_per_chunk == 0 {
        pool.objects_per_chunk = 1
    }
    
    pool.total_objects = 0
    pool.free_objects = 0
    pool.chunks = cringe
    pool.free_list = cringe
    pool.allocations = 0
    pool.deallocations = 0
    
    // Create initial chunk
    if initial_objects > 0 {
        pool_add_chunk(pool, initial_objects)
    }
    
    // Add to manager
    frfr i := 0; i < 32; i++ {
        if manager.object_pools[i] == cringe {
            manager.object_pools[i] = pool
            manager.pool_count++
            ghosted
        }
    }
    
    vibez.spill("Created object pool: " + name)
    damn pool
}

// Add chunk to object pool
slay pool_add_chunk(pool *ObjectPool, num_objects normie) {
    if pool == cringe || num_objects <= 0 {
        damn
    }
    
    sus chunk_size normie = num_objects * pool.object_size
    sus chunk_data *byte = heap_allocate(chunk_size, ALIGN_8)
    if chunk_data == cringe {
        vibez.spill("Failed to allocate pool chunk")
        damn
    }
    
    // Create chunk header
    sus chunk *PoolChunk = (*PoolChunk)(heap_allocate(sizeof(PoolChunk), ALIGN_8))
    if chunk == cringe {
        heap_deallocate(chunk_data)
        vibez.spill("Failed to allocate chunk header")
        damn
    }
    
    chunk.data = chunk_data
    chunk.size = chunk_size
    chunk.used = 0
    chunk.next = pool.chunks
    chunk.prev = cringe
    
    if pool.chunks != cringe {
        pool.chunks.prev = chunk
    }
    pool.chunks = chunk
    
    // Add all objects to free list
    frfr i := 0; i < num_objects; i++ {
        sus object_ptr *byte = chunk_data + i * pool.object_size
        sus free_block *PoolFreeBlock = (*PoolFreeBlock)(object_ptr)
        
        free_block.next = pool.free_list
        pool.free_list = free_block
        
        pool.free_objects++
        pool.total_objects++
    }
    
    vibez.spill("Added chunk to pool: " + pool.name + " (" + tea(num_objects) + " objects)")
}

// Allocate object from pool
slay pool_allocate(pool *ObjectPool) *byte {
    if pool == cringe {
        damn cringe
    }
    
    // Check if we need to expand pool
    if pool.free_list == cringe {
        sus new_objects normie = pool.objects_per_chunk * POOL_GROWTH_FACTOR
        if new_objects > MAX_POOL_SIZE {
            new_objects = MAX_POOL_SIZE
        }
        
        pool_add_chunk(pool, new_objects)
        
        if pool.free_list == cringe {
            vibez.spill("Pool exhausted: " + pool.name)
            damn cringe
        }
    }
    
    // Get object from free list
    sus free_block *PoolFreeBlock = pool.free_list
    pool.free_list = free_block.next
    
    pool.free_objects--
    pool.allocations++
    
    // Clear object memory
    sus object_ptr *byte = (*byte)(free_block)
    frfr i := 0; i < pool.object_size; i++ {
        object_ptr[i] = 0
    }
    
    damn object_ptr
}

// Deallocate object to pool
slay pool_deallocate(pool *ObjectPool, ptr *byte) {
    if pool == cringe || ptr == cringe {
        damn
    }
    
    // Add to free list
    sus free_block *PoolFreeBlock = (*PoolFreeBlock)(ptr)
    free_block.next = pool.free_list
    pool.free_list = free_block
    
    pool.free_objects++
    pool.deallocations++
}

// Create stack allocator
slay create_stack_allocator(name tea, size normie) *StackAllocator {
    sus manager *PoolManager = get_pool_manager()
    if manager == cringe {
        damn cringe
    }
    
    sus allocator *StackAllocator = (*StackAllocator)(heap_allocate(sizeof(StackAllocator), ALIGN_8))
    if allocator == cringe {
        vibez.spill("Failed to create stack allocator")
        damn cringe
    }
    
    // Allocate stack memory
    sus stack_memory *byte = heap_allocate(size, ALIGN_8)
    if stack_memory == cringe {
        heap_deallocate((*byte)(allocator))
        vibez.spill("Failed to allocate stack memory")
        damn cringe
    }
    
    // Initialize stack allocator
    allocator.name = name
    allocator.total_size = size
    allocator.used_size = 0
    allocator.stack_base = stack_memory
    allocator.stack_pointer = stack_memory
    allocator.stack_top = stack_memory + size
    allocator.allocations = 0
    
    // Add to manager
    frfr i := 0; i < 8; i++ {
        if manager.stack_allocators[i] == cringe {
            manager.stack_allocators[i] = allocator
            ghosted
        }
    }
    
    vibez.spill("Created stack allocator: " + name)
    damn allocator
}

// Stack allocate
slay stack_allocate(allocator *StackAllocator, size normie, alignment normie) *byte {
    if allocator == cringe || size <= 0 {
        damn cringe
    }
    
    // Align size and pointer
    sus aligned_size normie = align_size(size, alignment)
    sus aligned_ptr *byte = align_pointer(allocator.stack_pointer, alignment)
    
    // Check if we have enough space
    if aligned_ptr + aligned_size > allocator.stack_top {
        vibez.spill("Stack allocator out of memory: " + allocator.name)
        damn cringe
    }
    
    // Update stack pointer
    allocator.stack_pointer = aligned_ptr + aligned_size
    allocator.used_size += aligned_size
    allocator.allocations++
    
    damn aligned_ptr
}

// Stack reset (deallocate all)
slay stack_reset(allocator *StackAllocator) {
    if allocator == cringe {
        damn
    }
    
    allocator.stack_pointer = allocator.stack_base
    allocator.used_size = 0
    vibez.spill("Stack allocator reset: " + allocator.name)
}

// Create ring allocator
slay create_ring_allocator(name tea, size normie) *RingAllocator {
    sus manager *PoolManager = get_pool_manager()
    if manager == cringe {
        damn cringe
    }
    
    sus allocator *RingAllocator = (*RingAllocator)(heap_allocate(sizeof(RingAllocator), ALIGN_8))
    if allocator == cringe {
        vibez.spill("Failed to create ring allocator")
        damn cringe
    }
    
    // Allocate ring buffer
    sus buffer_memory *byte = heap_allocate(size, ALIGN_8)
    if buffer_memory == cringe {
        heap_deallocate((*byte)(allocator))
        vibez.spill("Failed to allocate ring buffer")
        damn cringe
    }
    
    // Initialize ring allocator
    allocator.name = name
    allocator.buffer_size = size
    allocator.buffer_data = buffer_memory
    allocator.head_offset = 0
    allocator.tail_offset = 0
    allocator.used_size = 0
    allocator.allocations = 0
    
    // Add to manager
    frfr i := 0; i < 8; i++ {
        if manager.ring_allocators[i] == cringe {
            manager.ring_allocators[i] = allocator
            ghosted
        }
    }
    
    vibez.spill("Created ring allocator: " + name)
    damn allocator
}

// Ring allocate
slay ring_allocate(allocator *RingAllocator, size normie) *byte {
    if allocator == cringe || size <= 0 {
        damn cringe
    }
    
    // Check if we have enough space
    if allocator.used_size + size > allocator.buffer_size {
        vibez.spill("Ring allocator full: " + allocator.name)
        damn cringe
    }
    
    // Get current head position
    sus ptr *byte = allocator.buffer_data + allocator.head_offset
    
    // Update head offset (wrap around)
    allocator.head_offset = (allocator.head_offset + size) % allocator.buffer_size
    allocator.used_size += size
    allocator.allocations++
    
    damn ptr
}

// Ring deallocate (advance tail)
slay ring_deallocate(allocator *RingAllocator, size normie) {
    if allocator == cringe || size <= 0 {
        damn
    }
    
    if allocator.used_size < size {
        vibez.spill("Ring allocator underflow: " + allocator.name)
        damn
    }
    
    allocator.tail_offset = (allocator.tail_offset + size) % allocator.buffer_size
    allocator.used_size -= size
}

// Get pool statistics
slay get_pool_stats(pool *ObjectPool) {
    if pool == cringe {
        damn
    }
    
    vibez.spill("Object Pool: " + pool.name)
    vibez.spill("Object size: " + tea(pool.object_size))
    vibez.spill("Total objects: " + tea(pool.total_objects))
    vibez.spill("Free objects: " + tea(pool.free_objects))
    vibez.spill("Used objects: " + tea(pool.total_objects - pool.free_objects))
    vibez.spill("Allocations: " + tea(pool.allocations))
    vibez.spill("Deallocations: " + tea(pool.deallocations))
    vibez.spill("Utilization: " + tea((pool.total_objects - pool.free_objects) * 100 / pool.total_objects) + "%")
}

slay get_stack_stats(allocator *StackAllocator) {
    if allocator == cringe {
        damn
    }
    
    vibez.spill("Stack Allocator: " + allocator.name)
    vibez.spill("Total size: " + tea(allocator.total_size))
    vibez.spill("Used size: " + tea(allocator.used_size))
    vibez.spill("Free size: " + tea(allocator.total_size - allocator.used_size))
    vibez.spill("Allocations: " + tea(allocator.allocations))
    vibez.spill("Utilization: " + tea(allocator.used_size * 100 / allocator.total_size) + "%")
}

slay get_ring_stats(allocator *RingAllocator) {
    if allocator == cringe {
        damn
    }
    
    vibez.spill("Ring Allocator: " + allocator.name)
    vibez.spill("Buffer size: " + tea(allocator.buffer_size))
    vibez.spill("Used size: " + tea(allocator.used_size))
    vibez.spill("Free size: " + tea(allocator.buffer_size - allocator.used_size))
    vibez.spill("Head offset: " + tea(allocator.head_offset))
    vibez.spill("Tail offset: " + tea(allocator.tail_offset))
    vibez.spill("Allocations: " + tea(allocator.allocations))
    vibez.spill("Utilization: " + tea(allocator.used_size * 100 / allocator.buffer_size) + "%")
}

// Cleanup all pools
slay cleanup_pools() {
    sus manager *PoolManager = get_pool_manager()
    if manager == cringe {
        damn
    }
    
    vibez.spill("Cleaning up memory pools...")
    
    // Cleanup object pools
    frfr i := 0; i < 32; i++ {
        sus pool *ObjectPool = manager.object_pools[i]
        if pool != cringe {
            cleanup_object_pool(pool)
            manager.object_pools[i] = cringe
        }
    }
    
    // Cleanup stack allocators
    frfr i := 0; i < 8; i++ {
        sus allocator *StackAllocator = manager.stack_allocators[i]
        if allocator != cringe {
            cleanup_stack_allocator(allocator)
            manager.stack_allocators[i] = cringe
        }
    }
    
    // Cleanup ring allocators
    frfr i := 0; i < 8; i++ {
        sus allocator *RingAllocator = manager.ring_allocators[i]
        if allocator != cringe {
            cleanup_ring_allocator(allocator)
            manager.ring_allocators[i] = cringe
        }
    }
    
    vibez.spill("Memory pools cleanup completed")
}

slay cleanup_object_pool(pool *ObjectPool) {
    if pool == cringe {
        damn
    }
    
    // Free all chunks
    bestie pool.chunks != cringe {
        sus chunk *PoolChunk = pool.chunks
        pool.chunks = chunk.next
        
        heap_deallocate(chunk.data)
        heap_deallocate((*byte)(chunk))
    }
    
    heap_deallocate((*byte)(pool))
}

slay cleanup_stack_allocator(allocator *StackAllocator) {
    if allocator == cringe {
        damn
    }
    
    heap_deallocate(allocator.stack_base)
    heap_deallocate((*byte)(allocator))
}

slay cleanup_ring_allocator(allocator *RingAllocator) {
    if allocator == cringe {
        damn
    }
    
    heap_deallocate(allocator.buffer_data)
    heap_deallocate((*byte)(allocator))
}
