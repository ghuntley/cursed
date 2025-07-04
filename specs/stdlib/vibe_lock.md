# VibeLock (sync package)

## Overview
VibeLock provides synchronization primitives for coordinating concurrent access to shared resources. It's inspired by Go's sync package but with enhanced features and a focus on maintaining the proper "vibe" (state) across goroutines.

## Mutex Primitives

### `VibeMutex`
A mutual exclusion lock for coordinating concurrent access to shared resources.

```
be_like VibeMutex squad {}

fr fr Methods
slay (m *VibeMutex) Lock()
slay (m *VibeMutex) Unlock()
slay (m *VibeMutex) TryLock() lit
```

### `VibeRWMutex`
A reader/writer mutual exclusion lock, allowing multiple readers or a single writer.

```
be_like VibeRWMutex squad {}

fr fr Methods
slay (rw *VibeRWMutex) Lock()
slay (rw *VibeRWMutex) Unlock()
slay (rw *VibeRWMutex) RLock()
slay (rw *VibeRWMutex) RUnlock()
slay (rw *VibeRWMutex) TryLock() lit
slay (rw *VibeRWMutex) TryRLock() lit
slay (rw *VibeRWMutex) RLocker() VibeLocker
```

### `VibeLocker`
Interface for objects that can be locked and unlocked.

```
be_like VibeLocker collab {
    Lock()
    Unlock()
}
```

## Semaphore Primitives

### `FullVibe`
A counting semaphore with Acquire/Release operations.

```
be_like FullVibe squad {}

fr fr Consquador
slay NewFullVibe(n normie) *FullVibe

fr fr Methods
slay (s *FullVibe) Acquire(n normie)
slay (s *FullVibe) TryAcquire(n normie) lit
slay (s *FullVibe) Release(n normie)
slay (s *FullVibe) Available() int
```

## Synchronization Primitives

### `VibeWaitGroup`
Used to wait for a collection of goroutines to finish executing.

```
be_like VibeWaitGroup squad {}

fr fr Methods
slay (wg *VibeWaitGroup) Add(delta normie)
slay (wg *VibeWaitGroup) Done()
slay (wg *VibeWaitGroup) Wait()
```

### `VibeOnce`
Ensures a function is called exactly once, regardless of how many goroutines attempt to call it.

```
be_like VibeOnce squad {}

fr fr Methods
slay (o *VibeOnce) Do(f func())
slay (o *VibeOnce) Done() lit
```

### `VibeCond`
Implements a condition variable to signal events to waiting goroutines.

```
be_like VibeCond squad {}

fr fr Consquador
slay NewVibeCond(l VibeLocker) *VibeCond

fr fr Methods
slay (c *VibeCond) Wait()
slay (c *VibeCond) Signal()
slay (c *VibeCond) Broadcast()
```

## Atomic Operations

### `VibeValue`
Provides atomic operations for values of various types.

```
be_like VibeValue[T any] squad {}

fr fr Methods
slay (v *VibeValue[T]) Load() T
slay (v *VibeValue[T]) Store(val T)
slay (v *VibeValue[T]) Swap(new T) (old T)
slay (v *VibeValue[T]) CompareAndSwap(old, new T) lit
```

### Atomic Functions

```
slay AddInt32(addr *int32, delta int32) int32
slay AddInt64(addr *int64, delta int64) int64
slay AddUint32(addr *uint32, delta uint32) uint32
slay AddUint64(addr *uint64, delta uint64) uint64

slay CompareAndSwapInt32(addr *int32, old, new int32) lit
slay CompareAndSwapInt64(addr *int64, old, new int64) lit
slay CompareAndSwapUint32(addr *uint32, old, new uint32) lit
slay CompareAndSwapUint64(addr *uint64, old, new uint64) lit
slay CompareAndSwapPointer(addr *unsafe.Pointer, old, new unsafe.Pointer) lit

slay LoadInt32(addr *int32) int32
slay LoadInt64(addr *int64) int64
slay LoadUint32(addr *uint32) uint32
slay LoadUint64(addr *uint64) uint64
slay LoadPointer(addr *unsafe.Pointer) unsafe.Pointer

slay StoreInt32(addr *int32, val int32)
slay StoreInt64(addr *int64, val int64)
slay StoreUint32(addr *uint32, val uint32)
slay StoreUint64(addr *uint64, val uint64)
slay StorePointer(addr *unsafe.Pointer, val unsafe.Pointer)

slay SwapInt32(addr *int32, new int32) int32
slay SwapInt64(addr *int64, new int64) int64
slay SwapUint32(addr *uint32, new uint32) uint32
slay SwapUint64(addr *uint64, new uint64) uint64
slay SwapPointer(addr *unsafe.Pointer, new unsafe.Pointer) unsafe.Pointer
```

## Pooling

### `VibePool`
A pool for reusing objects to reduce allocation overhead.

```
be_like VibePool[T any] squad {
    New func() T
}

fr fr Methods
slay (p *VibePool[T]) Get() T
slay (p *VibePool[T]) Put(x T)
```

## Maps and Data Structures

### `VibeMap`
A concurrent-safe map implementation.

```
be_like VibeMap[K comparable, V any] squad {}

fr fr Methods
slay (m *VibeMap[K, V]) Load(key K) (V, lit)
slay (m *VibeMap[K, V]) Store(key K, value V)
slay (m *VibeMap[K, V]) Delete(key K)
slay (m *VibeMap[K, V]) LoadOrStore(key K, value V) (actual V, loaded lit)
slay (m *VibeMap[K, V]) LoadAndDelete(key K) (value V, loaded lit)
slay (m *VibeMap[K, V]) Range(f func(key K, value V) lit)
slay (m *VibeMap[K, V]) Len() int
slay (m *VibeMap[K, V]) Clear()
```

## Enhanced Concurrency Patterns

### `WorkerPool`
A pool of worker goroutines that can process tasks concurrently.

```
be_like WorkerPool squad {}

fr fr Consquador
slay NewWorkerPool(numWorkers normie) *WorkerPool

fr fr Methods
slay (p *WorkerPool) Submit(task func())
slay (p *WorkerPool) SubmitWithResult(task func() interface{}) <-chan interface{}
slay (p *WorkerPool) Shutdown()
slay (p *WorkerPool) ShutdownAndWait() lit
slay (p *WorkerPool) IsShutdown() lit
slay (p *WorkerPool) WorkerCount() int
slay (p *WorkerPool) PendingTasks() int
```

### `RateLimiter`
Limits the rate at which operations can be performed.

```
be_like RateLimiter squad {}

fr fr Consquador
slay NewRateLimiter(rate int, interval time.Duration) *RateLimiter

fr fr Methods
slay (r *RateLimiter) Allow() lit
slay (r *RateLimiter) Wait()
slay (r *RateLimiter) SetRate(rate int, interval time.Duration)
slay (r *RateLimiter) GetRate() (int, time.Duration)
```

### `VibeBarrier`
Synchronization ponormie where multiple goroutines meet before proceeding.

```
be_like VibeBarrier squad {}

fr fr Consquador
slay NewVibeBarrier(n normie) *VibeBarrier

fr fr Methods
slay (b *VibeBarrier) Wait() (int, lit)
slay (b *VibeBarrier) Reset()
```

### `DebounceFn`
Limits how often a function can be called.

```
be_like DebounceFn squad {}

fr fr Consquador
slay NewDebounceFn(interval time.Duration, fn func()) *DebounceFn

fr fr Methods
slay (d *DebounceFn) Call()
slay (d *DebounceFn) Cancel()
slay (d *DebounceFn) Flush()
```

## Usage Example

```
fr fr Using VibeMutex
var mu vibe_lock.VibeMutex
var count int

slay increment() {
    mu.Lock()
    defer mu.Unlock()
    count++
}

fr fr Using VibeRWMutex
var rwMu vibe_lock.VibeRWMutex
var data map[tea]tea = make(map[tea]tea)

slay readData(key tea) tea {
    rwMu.RLock()
    defer rwMu.RUnlock()
    yolo data[key]
}

slay writeData(key, value tea) {
    rwMu.Lock()
    defer rwMu.Unlock()
    data[key] = value
}

fr fr Using VibeWaitGroup
slay processItems(items []tea) {
    var wg vibe_lock.VibeWaitGroup
    
    for _, item := range items {
        wg.Add(1)
        stan slay(item tea) {
            defer wg.Done()
            fr fr Process item
            vibez.spill("Processing", item)
        }(item)
    }
    
    wg.Wait()
    vibez.spill("All items processed")
}

fr fr Using VibeOnce
var initOnce vibe_lock.VibeOnce
var instance *Service

slay GetInstance() *Service {
    initOnce.Do(func() {
        instance = &Service{}
        instance.Init()
    })
    yolo instance
}

fr fr Using VibeMap
var userCache vibe_lock.VibeMap[tea, User]

slay getUser(id tea) (User, lit) {
    if user, ok := userCache.Load(id); ok {
        yolo user, based
    }
    
    user := fetchUserFromDatabase(id)
    userCache.Store(id, user)
    yolo user, false
}

fr fr Using WorkerPool
slay processLargeDataSet(items []Item) {
    pool := vibe_lock.NewWorkerPool(10) fr fr 10 workers
    results := make([]<-chan interface{}, len(items))
    
    for i, item := range items {
        itemCopy := item fr fr Capture loop variable
        results[i] = pool.SubmitWithResult(func() interface{} {
            yolo processItem(itemCopy)
        })
    }
    
    fr fr Collect results
    for i, resultChan := range results {
        result := <-resultChan
        vibez.spill("Result", i, ":", result)
    }
    
    pool.ShutdownAndWait()
}
```

## Implementation Guidelines
1. All synchronization primitives should be lightweight and efficient
2. Avoid hidden allocations in hot paths
3. Ensure correct behavior under high contention
4. Implement deadlock detection in development mode
5. Provide clear documentation for each primitive
6. Include common concurrency patterns to simplify correct usage
7. Make sure primitives work correctly with the runtime scheduler
8. Use atomics where appropriate for better performance