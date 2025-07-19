# Channel Core Module

Pure CURSED implementation of Go-style channels for goroutine communication, replacing `src/runtime/channels/` with zero FFI dependencies.

## Overview

The Channel Core module provides the foundation for inter-goroutine communication in CURSED. It implements unbuffered and buffered channels, select statements, and channel lifecycle management entirely in pure CURSED.

## Key Features

- **Go-Style Channels**: Unbuffered and buffered channel types
- **Select Statements**: Multi-channel operations with timeout support
- **Channel Lifecycle**: Creation, usage, and proper cleanup
- **Blocking Operations**: Cooperative blocking with goroutine integration
- **Type Safety**: Element type preservation across channel operations
- **Performance**: Optimized for high-throughput communication

## Channel Types

```cursed
sus CHANNEL_UNBUFFERED normie = 0   # Synchronous communication
sus CHANNEL_BUFFERED normie = 1     # Asynchronous with buffer
sus CHANNEL_CLOSED normie = 2       # Closed channel state
```

## Channel Operations

```cursed
sus CHAN_OP_SEND normie = 1      # Send operation
sus CHAN_OP_RECEIVE normie = 2   # Receive operation
sus CHAN_OP_SELECT normie = 3    # Select operation
```

## Core Types

### Channel
```cursed
vibe Channel = smash {
    id normie,
    channel_type normie,
    buffer_size normie,
    element_type tea,
    buffer []tea,
    buffer_head normie,
    buffer_tail normie,
    is_closed lit,
    send_waiters []normie,     # Waiting goroutines for send
    recv_waiters []normie,     # Waiting goroutines for receive
    created_at normie,
    total_sends normie,
    total_receives normie
}
```

### ChannelResult
```cursed
vibe ChannelResult = smash {
    success lit,
    value tea,
    channel_closed lit,
    would_block lit
}
```

### SelectCase
```cursed
vibe SelectCase = smash {
    channel_id normie,
    operation normie,
    send_value tea,
    case_index normie
}
```

## Key Functions

### Channel Management
- `init_channel_system()` - Initialize channel system
- `make_channel(buffer_size, element_type)` - Create new channel
- `close_channel(id)` - Close channel (no more sends)
- `channel_exists(id)` - Check if channel exists
- `get_channel_info(id)` - Retrieve channel information

### Communication Operations
- `channel_send(id, value)` - Send value to channel
- `channel_receive(id)` - Receive value from channel
- `send_unbuffered(id, value)` - Direct synchronous send
- `send_buffered(id, value)` - Buffered asynchronous send
- `receive_unbuffered(id)` - Direct synchronous receive
- `receive_buffered(id)` - Buffered receive

### Select Statements
- `channel_select(cases, default_case)` - Multi-channel select
- Support for send, receive, and default cases
- Non-blocking operation detection

### Statistics and Monitoring
- `get_channel_stats()` - Detailed channel statistics
- `channel_health_check()` - System health monitoring
- Performance metrics and usage tracking

## Channel Semantics

### Unbuffered Channels
- **Synchronous**: Send blocks until receive
- **Direct Transfer**: No intermediate buffering
- **Goroutine Coordination**: Perfect synchronization primitive

```cursed
sus sync_chan normie = make_channel(0, "tea")
# Sending blocks until someone receives
sus result ChannelResult = channel_send(sync_chan, "message")
```

### Buffered Channels
- **Asynchronous**: Send succeeds if buffer has space
- **FIFO Buffer**: First-in, first-out message ordering
- **Capacity Control**: Fixed buffer size prevents overflow

```cursed
sus async_chan normie = make_channel(10, "normie")
# Sends succeed until buffer is full
bestie i < 10 {
    channel_send(async_chan, stringz.itoa(i))
}
```

### Channel Closure
- **Send Prevention**: No new sends after closure
- **Receive Completion**: Existing buffer can be drained
- **Waiter Notification**: All blocked goroutines woken

```cursed
close_channel(chan_id)
# Subsequent sends fail with channel_closed = true
sus result ChannelResult = channel_send(chan_id, "fails")
assert_true(result.channel_closed)
```

## Select Statement Implementation

### Multi-Channel Operations
```cursed
sus cases []SelectCase = []

# Add receive case
sus recv_case SelectCase
recv_case.channel_id = chan1
recv_case.operation = CHAN_OP_RECEIVE
cases = append(cases, recv_case)

# Add send case  
sus send_case SelectCase
send_case.channel_id = chan2
send_case.operation = CHAN_OP_SEND
send_case.send_value = "data"
cases = append(cases, send_case)

# Execute select
sus selected normie = channel_select(cases, based)  # with default
```

### Selection Algorithm
1. **Immediate Check**: Test all cases for immediate availability
2. **Default Case**: Execute if no cases ready and default provided
3. **Blocking**: Wait for first case to become ready (future enhancement)

## Integration Points

### Goroutine System
- **Cooperative Blocking**: Channel operations yield to scheduler
- **Waiter Queues**: Blocked goroutines managed efficiently
- **Panic Safety**: Channel operations don't leak on panic

### Memory Management
- **Buffer Allocation**: Channel buffers managed by `memory_core`
- **Message Lifecycle**: Automatic cleanup of queued messages
- **GC Integration**: Proper cleanup of channel resources

### Runtime Core
- **Value Transport**: Messages use `CursedValue` for type safety
- **Serialization**: Values serialized for channel transport
- **Type Preservation**: Element types maintained across operations

## Performance Characteristics

- **Low Latency**: Minimal overhead for unbuffered channels
- **High Throughput**: Efficient buffered channel implementation
- **Memory Efficient**: Compact channel representation
- **Scalable**: Handles thousands of channels efficiently

## Testing

Run comprehensive tests with:
```bash
cargo run --bin cursed stdlib/channel_core/test_channel_core.csd
```

The test suite covers:
- Channel creation and lifecycle management
- Send and receive operations for both channel types
- Buffer overflow protection and blocking behavior
- Channel closure and error handling
- Select statement functionality
- Statistics and health monitoring
- Multiple goroutine scenarios
- Edge cases and error conditions

## Channel Statistics

Example statistics output:
```cursed
{
    "total_created": 100,
    "active_channels": 85,
    "total_sends": 50000,
    "total_receives": 48000,
    "unbuffered_channels": 20,
    "buffered_channels": 65,
    "total_waiters": 15
}
```

## Self-Hosting Impact

This module is **critical for self-hosting** as it enables:

1. **Compiler Pipeline**: Parallel compilation stages communication
2. **Tool Integration**: LSP, debugger, profiler coordination
3. **Background Tasks**: GC, I/O, monitoring task communication
4. **Error Propagation**: Clean error handling across goroutines

## Migration Status

- ✅ **Complete**: Core channel implementation
- ✅ **Complete**: Unbuffered and buffered channel types
- ✅ **Complete**: Send and receive operations
- ✅ **Complete**: Channel closure and lifecycle
- ✅ **Complete**: Basic select statement support
- ✅ **Complete**: Statistics and monitoring
- ✅ **Complete**: Comprehensive test coverage
- 🔄 **Enhancement**: Advanced select with timeout
- 🔄 **Integration**: Full goroutine scheduler integration

## Usage Examples

### Producer-Consumer Pattern
```cursed
# Create buffered channel for work items
sus work_chan normie = make_channel(100, "tea")

# Producer goroutine sends work
bestie i < 1000 {
    channel_send(work_chan, "work_item_" + stringz.itoa(i))
}
close_channel(work_chan)

# Consumer goroutine processes work
bestie {
    sus result ChannelResult = channel_receive(work_chan)
    lowkey result.channel_closed {
        ghosted  # No more work
    }
    process_work(result.value)
}
```

### Fan-Out Pattern
```cursed
sus input_chan normie = make_channel(0, "tea")
sus output_chan1 normie = make_channel(10, "tea")
sus output_chan2 normie = make_channel(10, "tea")

# Fan-out to multiple consumers
bestie {
    sus data ChannelResult = channel_receive(input_chan)
    lowkey data.success {
        channel_send(output_chan1, data.value)
        channel_send(output_chan2, data.value)
    }
}
```

This module successfully replaces `src/runtime/channels/` and provides the foundation for concurrent communication in the CURSED compiler.
