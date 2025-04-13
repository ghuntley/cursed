# atomic_drip (sync/atomic)

## Overview
The `atomic_drip` module provides low-level atomic memory operations for synchronization across goroutines. These operations ensure that concurrent modifications to shared memory are performed without race conditions.

## Core Types and Interfaces

### Basic Types
Atomic versions of basic types.

```csd
type Int32 struct {
  // fields not directly accessible
}

type Int64 struct {
  // fields not directly accessible
}

type Uint32 struct {
  // fields not directly accessible
}

type Uint64 struct {
  // fields not directly accessible
}

type Bool struct {
  // fields not directly accessible
}

type Float32 struct {
  // fields not directly accessible
}

type Float64 struct {
  // fields not directly accessible
}

type String struct {
  // fields not directly accessible
}

type Pointer[T any] struct {
  // fields not directly accessible
}
```

### Value
Generic atomic value container.

```csd
type Value[T any] struct {
  // fields not directly accessible
}

func (v *Value[T]) Load() T
func (v *Value[T]) Store(val T)
func (v *Value[T]) Swap(new T) T
func (v *Value[T]) CompareAndSwap(old, new T) bool
```

## Core Methods

### Int32/Int64 Methods

```csd
func (i *Int32) Load() int32
func (i *Int32) Store(val int32)
func (i *Int32) Add(delta int32) int32
func (i *Int32) Swap(new int32) int32
func (i *Int32) CompareAndSwap(old, new int32) bool

func (i *Int64) Load() int64
func (i *Int64) Store(val int64)
func (i *Int64) Add(delta int64) int64
func (i *Int64) Swap(new int64) int64
func (i *Int64) CompareAndSwap(old, new int64) bool
```

### Uint32/Uint64 Methods

```csd
func (u *Uint32) Load() uint32
func (u *Uint32) Store(val uint32)
func (u *Uint32) Add(delta uint32) uint32
func (u *Uint32) Swap(new uint32) uint32
func (u *Uint32) CompareAndSwap(old, new uint32) bool

func (u *Uint64) Load() uint64
func (u *Uint64) Store(val uint64)
func (u *Uint64) Add(delta uint64) uint64
func (u *Uint64) Swap(new uint64) uint64
func (u *Uint64) CompareAndSwap(old, new uint64) bool
```

### Bool Methods

```csd
func (b *Bool) Load() bool
func (b *Bool) Store(val bool)
func (b *Bool) Swap(new bool) bool
func (b *Bool) CompareAndSwap(old, new bool) bool
```

### Float32/Float64 Methods

```csd
func (f *Float32) Load() float32
func (f *Float32) Store(val float32)
func (f *Float32) Swap(new float32) float32
func (f *Float32) CompareAndSwap(old, new float32) bool

func (f *Float64) Load() float64
func (f *Float64) Store(val float64)
func (f *Float64) Swap(new float64) float64
func (f *Float64) CompareAndSwap(old, new float64) bool
```

### String Methods

```csd
func (s *String) Load() string
func (s *String) Store(val string)
func (s *String) Swap(new string) string
func (s *String) CompareAndSwap(old, new string) bool
```

### Pointer Methods

```csd
func (p *Pointer[T]) Load() *T
func (p *Pointer[T]) Store(val *T)
func (p *Pointer[T]) Swap(new *T) *T
func (p *Pointer[T]) CompareAndSwap(old, new *T) bool
```

## Enhanced Features

- **Wait Groups**: Atomic wait groups for goroutine synchronization
  ```csd
  group := atomic_drip.NewWaitGroup()
  group.Add(1)
  go func() {
    defer group.Done()
    // do work
  }()
  group.Wait()
  ```

- **Atomic Bitfield**: Efficient bit operations on atomic values
  ```csd
  bits := atomic_drip.NewBitfield32(0)
  bits.SetBits(0x3)  // Set bits 0 and 1
  val := bits.TestBit(1) // Check if bit 1 is set
  ```

- **Atomic Collections**: Thread-safe collections with atomic operations
  ```csd
  queue := atomic_drip.NewQueue[string]()
  queue.Push("item")
  item := queue.Pop()
  ```

- **Atomic Flags**: Specialized boolean flags with extended operations
  ```csd
  flag := atomic_drip.NewFlag(false)
  if flag.SetIfUnset() {
    // First goroutine to set the flag
  }
  ```

- **MemoryOrder Options**: Control memory ordering constraints
  ```csd
  counter.Store(10, atomic_drip.MemoryOrderRelease)
  value := counter.Load(atomic_drip.MemoryOrderAcquire)
  ```

## Usage Examples

```csd
// Basic atomic integer operations
counter := atomic_drip.Int64{}

// Store a value
counter.Store(10)

// Add to the counter from multiple goroutines
for i := 0; i < 10; i++ {
  go func() {
    for j := 0; j < 1000; j++ {
      counter.Add(1)
    }
  }()
}

// Wait for goroutines to finish (in real code, use proper synchronization)
timez.Sleep(timez.Second)

// Load the final value
finalValue := counter.Load()
vibez.spill("Final counter value: %d", finalValue)

// Compare and swap
current := counter.Load()
for {
  // Try to update the counter if it's still equal to current
  if counter.CompareAndSwap(current, current+1) {
    break
  }
  // Someone else modified it, get the new value and try again
  current = counter.Load()
}

vibez.spill("Counter after CAS: %d", counter.Load())

// Atomic boolean
done := atomic_drip.Bool{}
done.Store(false)

go func() {
  // Simulate work
  timez.Sleep(timez.Millisecond * 500)
  // Signal completion
  done.Store(true)
}()

// Wait for completion
for !done.Load() {
  timez.Sleep(timez.Millisecond * 10)
}

vibez.spill("Work completed!")

// Atomic pointer
type Data struct {
  Value int
  Name  string
}

ptr := atomic_drip.Pointer[Data]{}

// Store a pointer to new data
newData := &Data{Value: 42, Name: "answer"}
ptr.Store(newData)

// Access the data safely from multiple goroutines
data := ptr.Load()
vibez.spill("Data: %+v", data)

// Atomic Value (type-safe generic container)
configValue := atomic_drip.Value[map[string]string]{}

// Initialize with a config
config := map[string]string{
  "host": "localhost",
  "port": "8080",
}
configValue.Store(config)

// Access it from another goroutine
go func() {
  cfg := configValue.Load()
  vibez.spill("Config from goroutine: %v", cfg)
}()

// Update the config atomically
updatedConfig := map[string]string{
  "host": "example.com",
  "port": "9090",
  "protocol": "https",
}
configValue.Store(updatedConfig)

// Wait for goroutine
timez.Sleep(timez.Millisecond * 100)

// Atomic Float64
temperature := atomic_drip.Float64{}
temperature.Store(98.6)

go func() {
  // Update temperature
  for i := 0; i < 10; i++ {
    current := temperature.Load()
    temperature.Store(current + 0.1)
    timez.Sleep(timez.Millisecond * 10)
  }
}()

// Monitor temperature changes
for i := 0; i < 5; i++ {
  vibez.spill("Temperature: %.1f", temperature.Load())
  timez.Sleep(timez.Millisecond * 30)
}

// Atomic string
message := atomic_drip.String{}
message.Store("Hello")

go func() {
  oldMsg := message.Swap("World")
  vibez.spill("Swapped '%s' with 'World'", oldMsg)
}()

// Wait for swap
timez.Sleep(timez.Millisecond * 50)
vibez.spill("Current message: %s", message.Load())

// Using atomic bitfield
flags := atomic_drip.NewBitfield32(0)
flags.SetBit(0)   // Set first bit
flags.SetBit(2)   // Set third bit

value := flags.Load()
vibez.spill("Flags value: %b", value)
vibez.spill("Bit 0 set: %v", flags.TestBit(0))
vibez.spill("Bit 1 set: %v", flags.TestBit(1))
vibez.spill("Bit 2 set: %v", flags.TestBit(2))

flags.ClearBit(0) // Clear first bit
vibez.spill("After clearing bit 0: %b", flags.Load())
```

## Implementation Guidelines

- Implement all operations using hardware atomic instructions
- Ensure correct memory ordering semantics
- Optimize for common architectures (x86, ARM, etc.)
- Provide fallbacks for architectures without direct atomic support
- Document memory ordering guarantees for each operation
- Handle 32-bit vs 64-bit alignment issues properly
- Ensure consistent behavior across different platforms
- Minimize overhead for atomic operations
- Support efficient busy-waiting techniques
- Ensure proper memory barriers for cross-thread visibility