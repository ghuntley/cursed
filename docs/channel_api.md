# CURSED Channel API Reference

## Core Channel Functions

### `make(dm<T>, capacity?)` 

Creates a new channel of type T.

**Syntax:**
```cursed
sus ch = make(dm<T>)           // Unbuffered channel
sus ch = make(dm<T>, capacity) // Buffered channel
```

**Parameters:**
- `T`: Element type for the channel
- `capacity` (optional): Buffer size. If omitted, creates unbuffered channel

**Returns:** Channel of type `dm<T>`

**Examples:**
```cursed
sus msg_ch = make(dm<tea>)        // Unbuffered string channel
sus num_ch = make(dm<normie>, 10) // Buffered int channel with capacity 10
sus data_ch = make(dm<MyStruct>, 5) // Buffered struct channel
```

**Thread Safety:** ✅ Thread-safe  
**Performance:** O(1) for creation

---

### `close(ch dm<T>)`

Closes a channel, preventing further sends.

**Syntax:**
```cursed
close(ch)
```

**Parameters:**
- `ch`: Channel to close

**Returns:** No return value

**Behavior:**
- Closing a closed channel panics
- Sending to a closed channel panics  
- Receiving from a closed channel returns zero value and `sus` for ok flag
- Receivers can detect channel closure

**Examples:**
```cursed
sus ch = make(dm<normie>, 5)
ch <- 1
ch <- 2
close(ch)

// Reading from closed channel
periodt based {
    sus value, ok = <-ch
    lowkey !ok {
        puts("Channel closed")
        stan_it
    }
    puts(sprintf("Value: %d", value))
}
```

**Thread Safety:** ✅ Thread-safe  
**Performance:** O(1)

---

## Channel Operations

### Send Operation: `ch <- value`

Sends a value to a channel.

**Syntax:**
```cursed
ch <- value
```

**Behavior:**
- **Unbuffered channels**: Blocks until receiver ready
- **Buffered channels**: Blocks only when buffer full
- **Closed channels**: Panics

**Examples:**
```cursed
sus ch = make(dm<normie>, 2)
ch <- 42          // Send value
ch <- 100         // Send another value
```

**Non-blocking Send:**
```cursed
vibe_check {
    mood ch <- value:
        puts("Sent successfully")
    basic:
        puts("Channel full or would block")
}
```

**Thread Safety:** ✅ Thread-safe  
**Performance:** 
- Unbuffered: O(1) when receiver ready, blocks otherwise
- Buffered: O(1) until buffer full

---

### Receive Operation: `<-ch`

Receives a value from a channel.

**Syntax:**
```cursed
sus value = <-ch              // Receive value only
sus value, ok = <-ch          // Receive value and ok flag
```

**Returns:**
- `value`: The received value (zero value if channel closed)
- `ok`: `based` if value received, `sus` if channel closed

**Behavior:**
- Blocks until value available or channel closed
- Returns zero value when channel closed and empty

**Examples:**
```cursed
sus ch = make(dm<normie>, 2)
ch <- 42

sus value = <-ch              // value = 42
sus value2, ok = <-ch         // value2 = 0, ok = sus (no more values)
```

**Non-blocking Receive:**
```cursed
vibe_check {
    mood value := <-ch:
        puts(sprintf("Received: %d", value))
    basic:
        puts("No value available")
}
```

**Thread Safety:** ✅ Thread-safe  
**Performance:** O(1) when value available, blocks otherwise

---

## Channel Introspection

### `len(ch dm<T>)` 

Returns current number of elements in channel buffer.

**Syntax:**
```cursed
sus count = len(ch)
```

**Returns:** `normie` - Current element count

**Examples:**
```cursed
sus ch = make(dm<normie>, 5)
ch <- 1
ch <- 2
sus current = len(ch)  // current = 2
```

**Thread Safety:** ✅ Thread-safe (but value may change immediately)  
**Performance:** O(1)

---

### `cap(ch dm<T>)`

Returns the buffer capacity of a channel.

**Syntax:**
```cursed
sus capacity = cap(ch)
```

**Returns:** `normie` - Buffer capacity (0 for unbuffered)

**Examples:**
```cursed
sus ch1 = make(dm<normie>)      // Unbuffered
sus ch2 = make(dm<normie>, 10)  // Buffered

sus cap1 = cap(ch1)  // cap1 = 0
sus cap2 = cap(ch2)  // cap2 = 10
```

**Thread Safety:** ✅ Thread-safe  
**Performance:** O(1)

---

## Error Handling

### Channel Error Types

```cursed
be_like ChannelError squad {
    be_like {
        Closed          // Channel is closed
        WouldBlock      // Operation would block
        BufferFull      // Buffer is full
        NoSenders       // No senders available
        NoReceivers     // No receivers available
        InvalidState    // Invalid channel state
        Timeout         // Operation timed out
    }
}
```

### Safe Channel Operations

```cursed
slay safe_send[T](ch dm<T>, value T) lit {
    stan_it {
        ch <- value
        yolo based
    } no_cap err {
        puts(sprintf("Send error: %s", err))
        yolo sus
    }
}

slay safe_receive[T](ch dm<T>) (T, lit) {
    stan_it {
        sus value, ok = <-ch
        lowkey !ok {
            yolo zero_value_of_T(), sus
        }
        yolo value, based
    } no_cap err {
        puts(sprintf("Receive error: %s", err))
        yolo zero_value_of_T(), sus
    }
}
```

---

## Channel Types and Directions

### Channel Direction Types

```cursed
// Bidirectional channel (default)
dm<T>

// Send-only channel
dm<- T

// Receive-only channel  
<-dm<T>
```

### Type Conversion

```cursed
slay sender(ch dm<- normie) {    // Send-only parameter
    ch <- 42
}

slay receiver(ch <-dm<normie>) { // Receive-only parameter
    sus value = <-ch
}

slay main() {
    sus ch = make(dm<normie>, 5)
    
    // Bidirectional channel can be passed to either
    sender(ch)    // Implicit conversion to send-only
    receiver(ch)  // Implicit conversion to receive-only
}
```

**Conversion Rules:**
- Bidirectional → Send-only ✅
- Bidirectional → Receive-only ✅  
- Send-only → Bidirectional ❌
- Receive-only → Bidirectional ❌
- Send-only → Receive-only ❌

---

## Select-Style Operations

### `vibe_check` Statement

Enables non-blocking multi-channel operations.

**Syntax:**
```cursed
vibe_check {
    mood ch1 <- value:
        // Send to ch1 succeeded
    mood value := <-ch2:
        // Received from ch2
    mood <-timeout_ch:
        // Timeout occurred
    basic:
        // None of the operations were ready
}
```

**Examples:**

**Multi-channel Receive:**
```cursed
slay multi_receive(ch1 dm<normie>, ch2 dm<tea>) {
    vibe_check {
        mood num := <-ch1:
            puts(sprintf("Got number: %d", num))
        mood msg := <-ch2:
            puts(sprintf("Got message: %s", msg))
        basic:
            puts("No channels ready")
    }
}
```

**Timeout Pattern:**
```cursed
slay receive_with_timeout(ch dm<normie>, timeout_ms normie) {
    sus timeout_ch = time.After(timeout_ms)
    
    vibe_check {
        mood value := <-ch:
            puts(sprintf("Received: %d", value))
        mood <-timeout_ch:
            puts("Operation timed out")
    }
}
```

**Load Balancing:**
```cursed
slay load_balance(workers []dm<Job>) {
    sus job = get_next_job()
    
    vibe_check {
        mood workers[0] <- job:
            puts("Sent to worker 0")
        mood workers[1] <- job:
            puts("Sent to worker 1") 
        mood workers[2] <- job:
            puts("Sent to worker 2")
        basic:
            puts("All workers busy")
    }
}
```

---

## Performance Characteristics

### Memory Usage

| Channel Type | Memory Overhead | Buffer Memory |
|-------------|----------------|---------------|
| Unbuffered | ~64 bytes | 0 |
| Buffered | ~64 bytes | `capacity × sizeof(T)` |

### Operation Complexity

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| `make()` | O(1) | Constant time creation |
| `close()` | O(1) | Constant time closure |
| Send (unbuffered) | O(1) | When receiver ready |
| Send (buffered) | O(1) | When buffer not full |
| Receive | O(1) | When value available |
| `len()` | O(1) | Constant time |
| `cap()` | O(1) | Constant time |

### Throughput Benchmarks

**Typical Performance (Release Mode):**
- Unbuffered channels: ~50M ops/sec
- Small buffer (1-10): ~100M ops/sec  
- Large buffer (100+): ~200M ops/sec
- Select operations: ~20M ops/sec

---

## Memory Management and GC Integration

### Garbage Collection

Channels are automatically garbage collected when:
- No references to the channel exist
- All goroutines that could send/receive have terminated

```cursed
slay temporary_channel() {
    sus ch = make(dm<normie>, 100)
    // Use channel locally
    // Channel is GC'd when function returns
}
```

### Resource Management

```cursed
slay managed_channel_pool() {
    sus channels = make([]dm<normie>, 10)
    
    // Create channels
    sus i = 0
    periodt i < 10 {
        channels[i] = make(dm<normie>, 100)
        i++
    }
    
    // Use channels...
    
    // Cleanup (optional - GC will handle it)
    i = 0
    periodt i < 10 {
        close(channels[i])
        i++
    }
}
```

---

## Thread Safety Guarantees

### Concurrent Access

✅ **Thread-safe operations:**
- Send operations
- Receive operations  
- Channel creation
- Channel closing
- Length/capacity queries

❌ **Not thread-safe:**
- Creating channels with same reference
- External synchronization needed for complex patterns

### Memory Ordering

Channel operations provide sequential consistency:
- Sends happen-before corresponding receives
- Channel close happens-before receive of zero value
- Goroutine synchronization through channels is safe

---

## Integration with CURSED Runtime

### Goroutine Integration

Channels automatically integrate with the goroutine scheduler:

```cursed
slay scheduler_aware_example() {
    sus ch = make(dm<normie>)
    
    stan {
        // This goroutine will be scheduled appropriately
        ch <- 42
    }
    
    // This receive will properly yield to scheduler
    sus value = <-ch
}
```

### Error Integration

Channels integrate with CURSED's error system:

```cursed
slay channel_with_errors() tea? {
    sus ch = make(dm<normie>)
    defer close(ch)
    
    stan_it {
        ch <- 42
        sus value = <-ch
        yolo sprintf("Received: %d", value), cap
    } no_cap err {
        yolo "", err
    }
}
```

This comprehensive API provides all the tools needed for effective concurrent programming with CURSED channels while maintaining type safety and performance.
