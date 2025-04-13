# VibeLock (sync package)

## Overview
VibeLock provides synchronization primitives for coordinating concurrent access to shared resources. It's inspired by Go's sync package but with enhanced features and a focus on maintaining the proper "vibe" (state) across goroutines.

## Mutex Primitives

### `VibeMutex`
A mutual exclusion lock for coordinating concurrent access to shared resources.

```go
type VibeMutex struct {}

// Methods
func (m *VibeMutex) Lock()
func (m *VibeMutex) Unlock()
func (m *VibeMutex) TryLock() bool
```

### `VibeRWMutex`
A reader/writer mutual exclusion lock, allowing multiple readers or a single writer.

```go
type VibeRWMutex struct {}

// Methods
func (rw *VibeRWMutex) Lock()
func (rw *VibeRWMutex) Unlock()
func (rw *VibeRWMutex) RLock()
func (rw *VibeRWMutex) RUnlock()
func (rw *VibeRWMutex) TryLock() bool
func (rw *VibeRWMutex) TryRLock() bool
func (rw *VibeRWMutex) RLocker() VibeLocker
```

### `VibeLocker`
Interface for objects that can be locked and unlocked.

```go
type VibeLocker interface {
    Lock()
    Unlock()
}
```

## Semaphore Primitives

### `FullVibe`
A counting semaphore with Acquire/Release operations.

```go
type FullVibe struct {}

// Constructor
func NewFullVibe(n int) *FullVibe

// Methods
func (s *FullVibe) Acquire(n int)
func (s *FullVibe) TryAcquire(n int) bool
func (s *FullVibe) Release(n int)
func (s *FullVibe) Available() int
```

## Synchronization Primitives

### `VibeWaitGroup`
Used to wait for a collection of goroutines to finish executing.

```go
type VibeWaitGroup struct {}

// Methods
func (wg *VibeWaitGroup) Add(delta int)
func (wg *VibeWaitGroup) Done()
func (wg *VibeWaitGroup) Wait()
```

### `VibeOnce`
Ensures a function is called exactly once, regardless of how many goroutines attempt to call it.

```go
type VibeOnce struct {}

// Methods
func (o *VibeOnce) Do(f func())
func (o *VibeOnce) Done() bool
```

### `VibeCond`
Implements a condition variable to signal events to waiting goroutines.

```go
type VibeCond struct {}

// Constructor
func NewVibeCond(l VibeLocker) *VibeCond

// Methods
func (c *VibeCond) Wait()
func (c *VibeCond) Signal()
func (c *VibeCond) Broadcast()
```

## Atomic Operations

### `VibeValue`
Provides atomic operations for values of various types.

```go
type VibeValue[T any] struct {}

// Methods
func (v *VibeValue[T]) Load() T
func (v *VibeValue[T]) Store(val T)
func (v *VibeValue[T]) Swap(new T) (old T)
func (v *VibeValue[T]) CompareAndSwap(old, new T) bool
```

### Atomic Functions

```go
func AddInt32(addr *int32, delta int32) int32
func AddInt64(addr *int64, delta int64) int64
func AddUint32(addr *uint32, delta uint32) uint32
func AddUint64(addr *uint64, delta uint64) uint64

func CompareAndSwapInt32(addr *int32, old, new int32) bool
func CompareAndSwapInt64(addr *int64, old, new int64) bool
func CompareAndSwapUint32(addr *uint32, old, new uint32) bool
func CompareAndSwapUint64(addr *uint64, old, new uint64) bool
func CompareAndSwapPointer(addr *unsafe.Pointer, old, new unsafe.Pointer) bool

func LoadInt32(addr *int32) int32
func LoadInt64(addr *int64) int64
func LoadUint32(addr *uint32) uint32
func LoadUint64(addr *uint64) uint64
func LoadPointer(addr *unsafe.Pointer) unsafe.Pointer

func StoreInt32(addr *int32, val int32)
func StoreInt64(addr *int64, val int64)
func StoreUint32(addr *uint32, val uint32)
func StoreUint64(addr *uint64, val uint64)
func StorePointer(addr *unsafe.Pointer, val unsafe.Pointer)

func SwapInt32(addr *int32, new int32) int32
func SwapInt64(addr *int64, new int64) int64
func SwapUint32(addr *uint32, new uint32) uint32
func SwapUint64(addr *uint64, new uint64) uint64
func SwapPointer(addr *unsafe.Pointer, new unsafe.Pointer) unsafe.Pointer
```

## Pooling

### `VibePool`
A pool for reusing objects to reduce allocation overhead.

```go
type VibePool[T any] struct {
    New func() T
}

// Methods
func (p *VibePool[T]) Get() T
func (p *VibePool[T]) Put(x T)
```

## Maps and Data Structures

### `VibeMap`
A concurrent-safe map implementation.

```go
type VibeMap[K comparable, V any] struct {}

// Methods
func (m *VibeMap[K, V]) Load(key K) (V, bool)
func (m *VibeMap[K, V]) Store(key K, value V)
func (m *VibeMap[K, V]) Delete(key K)
func (m *VibeMap[K, V]) LoadOrStore(key K, value V) (actual V, loaded bool)
func (m *VibeMap[K, V]) LoadAndDelete(key K) (value V, loaded bool)
func (m *VibeMap[K, V]) Range(f func(key K, value V) bool)
func (m *VibeMap[K, V]) Len() int
func (m *VibeMap[K, V]) Clear()
```

## Enhanced Concurrency Patterns

### `WorkerPool`
A pool of worker goroutines that can process tasks concurrently.

```go
type WorkerPool struct {}

// Constructor
func NewWorkerPool(numWorkers int) *WorkerPool

// Methods
func (p *WorkerPool) Submit(task func())
func (p *WorkerPool) SubmitWithResult(task func() interface{}) <-chan interface{}
func (p *WorkerPool) Shutdown()
func (p *WorkerPool) ShutdownAndWait() bool
func (p *WorkerPool) IsShutdown() bool
func (p *WorkerPool) WorkerCount() int
func (p *WorkerPool) PendingTasks() int
```

### `RateLimiter`
Limits the rate at which operations can be performed.

```go
type RateLimiter struct {}

// Constructor
func NewRateLimiter(rate int, interval time.Duration) *RateLimiter

// Methods
func (r *RateLimiter) Allow() bool
func (r *RateLimiter) Wait()
func (r *RateLimiter) SetRate(rate int, interval time.Duration)
func (r *RateLimiter) GetRate() (int, time.Duration)
```

### `VibeBarrier`
Synchronization point where multiple goroutines meet before proceeding.

```go
type VibeBarrier struct {}

// Constructor
func NewVibeBarrier(n int) *VibeBarrier

// Methods
func (b *VibeBarrier) Wait() (int, bool)
func (b *VibeBarrier) Reset()
```

### `DebounceFn`
Limits how often a function can be called.

```go
type DebounceFn struct {}

// Constructor
func NewDebounceFn(interval time.Duration, fn func()) *DebounceFn

// Methods
func (d *DebounceFn) Call()
func (d *DebounceFn) Cancel()
func (d *DebounceFn) Flush()
```

## Usage Example

```go
// Using VibeMutex
var mu vibe_lock.VibeMutex
var count int

func increment() {
    mu.Lock()
    defer mu.Unlock()
    count++
}

// Using VibeRWMutex
var rwMu vibe_lock.VibeRWMutex
var data map[string]string = make(map[string]string)

func readData(key string) string {
    rwMu.RLock()
    defer rwMu.RUnlock()
    return data[key]
}

func writeData(key, value string) {
    rwMu.Lock()
    defer rwMu.Unlock()
    data[key] = value
}

// Using VibeWaitGroup
func processItems(items []string) {
    var wg vibe_lock.VibeWaitGroup
    
    for _, item := range items {
        wg.Add(1)
        go func(item string) {
            defer wg.Done()
            // Process item
            vibez.spill("Processing", item)
        }(item)
    }
    
    wg.Wait()
    vibez.spill("All items processed")
}

// Using VibeOnce
var initOnce vibe_lock.VibeOnce
var instance *Service

func GetInstance() *Service {
    initOnce.Do(func() {
        instance = &Service{}
        instance.Init()
    })
    return instance
}

// Using VibeMap
var userCache vibe_lock.VibeMap[string, User]

func getUser(id string) (User, bool) {
    if user, ok := userCache.Load(id); ok {
        return user, true
    }
    
    user := fetchUserFromDatabase(id)
    userCache.Store(id, user)
    return user, false
}

// Using WorkerPool
func processLargeDataSet(items []Item) {
    pool := vibe_lock.NewWorkerPool(10) // 10 workers
    results := make([]<-chan interface{}, len(items))
    
    for i, item := range items {
        itemCopy := item // Capture loop variable
        results[i] = pool.SubmitWithResult(func() interface{} {
            return processItem(itemCopy)
        })
    }
    
    // Collect results
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