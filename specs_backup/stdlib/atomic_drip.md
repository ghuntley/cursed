# atomic_drip (sync/atomic)

## Overview
The `atomic_drip` module provides low-level atomic memory operations for synchronization across goroutines. These operations ensure that concurrent modifications to shared memory are performed without race conditions.

## Core Types and Interfaces

### Basic Types
Atomic versions of basic types.

```csd
be_like Int32 squad {
  fr fr fields not directly accessible
}

be_like Int64 squad {
  fr fr fields not directly accessible
}

be_like Uint32 squad {
  fr fr fields not directly accessible
}

be_like Uint64 squad {
  fr fr fields not directly accessible
}

be_like Bool squad {
  fr fr fields not directly accessible
}

be_like Float32 squad {
  fr fr fields not directly accessible
}

be_like Float64 squad {
  fr fr fields not directly accessible
}

be_like String squad {
  fr fr fields not directly accessible
}

be_like Pointer[T any] squad {
  fr fr fields not directly accessible
}
```

### Value
Generic atomic value container.

```csd
be_like Value[T any] squad {
  fr fr fields not directly accessible
}

slay (v *Value[T]) Load() T
slay (v *Value[T]) Store(val T)
slay (v *Value[T]) Swap(new T) T
slay (v *Value[T]) CompareAndSwap(old, new T) lit
```

## Core Methods

### Int32/Int64 Methods

```csd
slay (i *Int32) Load() int32
slay (i *Int32) Store(val int32)
slay (i *Int32) Add(delta int32) int32
slay (i *Int32) Swap(new int32) int32
slay (i *Int32) CompareAndSwap(old, new int32) lit

slay (i *Int64) Load() int64
slay (i *Int64) Store(val int64)
slay (i *Int64) Add(delta int64) int64
slay (i *Int64) Swap(new int64) int64
slay (i *Int64) CompareAndSwap(old, new int64) lit
```

### Uint32/Uint64 Methods

```csd
slay (u *Uint32) Load() uint32
slay (u *Uint32) Store(val uint32)
slay (u *Uint32) Add(delta uint32) uint32
slay (u *Uint32) Swap(new uint32) uint32
slay (u *Uint32) CompareAndSwap(old, new uint32) lit

slay (u *Uint64) Load() uint64
slay (u *Uint64) Store(val uint64)
slay (u *Uint64) Add(delta uint64) uint64
slay (u *Uint64) Swap(new uint64) uint64
slay (u *Uint64) CompareAndSwap(old, new uint64) lit
```

### Bool Methods

```csd
slay (b *Bool) Load() lit
slay (b *Bool) Store(val lit)
slay (b *Bool) Swap(new lit) lit
slay (b *Bool) CompareAndSwap(old, new lit) lit
```

### Float32/Float64 Methods

```csd
slay (f *Float32) Load() float32
slay (f *Float32) Store(val float32)
slay (f *Float32) Swap(new float32) float32
slay (f *Float32) CompareAndSwap(old, new float32) lit

slay (f *Float64) Load() float64
slay (f *Float64) Store(val float64)
slay (f *Float64) Swap(new float64) float64
slay (f *Float64) CompareAndSwap(old, new float64) lit
```

### String Methods

```csd
slay (s *String) Load() tea
slay (s *String) Store(val tea)
slay (s *String) Swap(new tea) tea
slay (s *String) CompareAndSwap(old, new tea) lit
```

### Pointer Methods

```csd
slay (p *Pointer[T]) Load() *T
slay (p *Pointer[T]) Store(val *T)
slay (p *Pointer[T]) Swap(new *T) *T
slay (p *Pointer[T]) CompareAndSwap(old, new *T) lit
```

## Enhanced Features

- **Wait Groups**: Atomic wait groups for goroutine synchronization
  ```csd
  group := atomic_drip.NewWaitGroup()
  group.Add(1)
  stan slay() {
    defer group.Done()
    fr fr do work
  }()
  group.Wait()
  ```

- **Atomic Bitfield**: Efficient bit operations on atomic values
  ```csd
  bits := atomic_drip.NewBitfield32(0)
  bits.SetBits(0x3)  fr fr Set bits 0 and 1
  val := bits.TestBit(1) fr fr Check if bit 1 is set
  ```

- **Atomic Collections**: Thread-safe collections with atomic operations
  ```csd
  queue := atomic_drip.NewQueue[tea]()
  queue.Push("item")
  item := queue.Pop()
  ```

- **Atomic Flags**: Specialized litean flags with extended operations
  ```csd
  flag := atomic_drip.NewFlag(false)
  if flag.SetIfUnset() {
    fr fr First goroutine to set the flag
  }
  ```

- **MemoryOrder Options**: Control memory ordering constraints
  ```csd
  counter.Store(10, atomic_drip.MemoryOrderRelease)
  value := counter.Load(atomic_drip.MemoryOrderAcquire)
  ```

## Usage Examples

```csd
fr fr Basic atomic integer operations
counter := atomic_drip.Int64{}

fr fr Store a value
counter.Store(10)

fr fr Add to the counter from multiple goroutines
for i := 0; i < 10; i++ {
  stan slay() {
    for j := 0; j < 1000; j++ {
      counter.Add(1)
    }
  }()
}

fr fr Wait for goroutines to finish (in real code, use proper synchronization)
timez.Sleep(timez.Second)

fr fr Load the final value
finalValue := counter.Load()
vibez.spill("Final counter value: %d", finalValue)

fr fr Compare and swap
current := counter.Load()
for {
  fr fr Try to update the counter if it's still equal to current
  if counter.CompareAndSwap(current, current+1) {
    break
  }
  fr fr Someone else modified it, get the new value and try again
  current = counter.Load()
}

vibez.spill("Counter after CAS: %d", counter.Load())

fr fr Atomic litean
done := atomic_drip.Bool{}
done.Store(false)

stan slay() {
  fr fr Simulate work
  timez.Sleep(timez.Millisecond * 500)
  fr fr Signal completion
  done.Store(based)
}()

fr fr Wait for completion
for !done.Load() {
  timez.Sleep(timez.Millisecond * 10)
}

vibez.spill("Work completed!")

fr fr Atomic pointer
be_like Data squad {
  Value int
  Name  tea
}

ptr := atomic_drip.Pointer[Data]{}

fr fr Store a pointer to new data
newData := &Data{Value: 42, Name: "answer"}
ptr.Store(newData)

fr fr Access the data safely from multiple goroutines
data := ptr.Load()
vibez.spill("Data: %+v", data)

fr fr Atomic Value (type-safe generic container)
configValue := atomic_drip.Value[map[tea]tea]{}

fr fr Initialize with a config
config := map[tea]tea{
  "host": "localhost",
  "port": "8080",
}
configValue.Store(config)

fr fr Access it from another goroutine
stan slay() {
  cfg := configValue.Load()
  vibez.spill("Config from goroutine: %v", cfg)
}()

fr fr Update the config atomically
updatedConfig := map[tea]tea{
  "host": "example.com",
  "port": "9090",
  "protocol": "https",
}
configValue.Store(updatedConfig)

fr fr Wait for goroutine
timez.Sleep(timez.Millisecond * 100)

fr fr Atomic Float64
temperature := atomic_drip.Float64{}
temperature.Store(98.6)

stan slay() {
  fr fr Update temperature
  for i := 0; i < 10; i++ {
    current := temperature.Load()
    temperature.Store(current + 0.1)
    timez.Sleep(timez.Millisecond * 10)
  }
}()

fr fr Monitor temperature changes
for i := 0; i < 5; i++ {
  vibez.spill("Temperature: %.1f", temperature.Load())
  timez.Sleep(timez.Millisecond * 30)
}

fr fr Atomic tea
message := atomic_drip.String{}
message.Store("Hello")

stan slay() {
  oldMsg := message.Swap("World")
  vibez.spill("Swapped '%s' with 'World'", oldMsg)
}()

fr fr Wait for swap
timez.Sleep(timez.Millisecond * 50)
vibez.spill("Current message: %s", message.Load())

fr fr Using atomic bitfield
flags := atomic_drip.NewBitfield32(0)
flags.SetBit(0)   fr fr Set first bit
flags.SetBit(2)   fr fr Set third bit

value := flags.Load()
vibez.spill("Flags value: %b", value)
vibez.spill("Bit 0 set: %v", flags.TestBit(0))
vibez.spill("Bit 1 set: %v", flags.TestBit(1))
vibez.spill("Bit 2 set: %v", flags.TestBit(2))

flags.ClearBit(0) fr fr Clear first bit
vibez.spill("After clearing bit 0: %b", flags.Load())
```

## Implementation Guidelines

- Implement all operations using hardware atomic insquadions
- Ensure correct memory ordering semantics
- Optimize for common architectures (x86, ARM, etc.)
- Provide fallbacks for architectures without direct atomic support
- Document memory ordering guarantees for each operation
- Handle 32-bit vs 64-bit alignment issues properly
- Ensure consistent behavior across different platforms
- Minimize overhead for atomic operations
- Support efficient busy-waiting techniques
- Ensure proper memory barriers for cross-thread visibility