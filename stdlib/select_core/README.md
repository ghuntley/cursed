# select_core

Pure CURSED implementation of select/channel operations providing Go-style concurrency primitives. Features non-blocking channel operations, timeout handling, and comprehensive select statement support for concurrent programming.

## Overview

The `select_core` module provides:
- Complete select statement implementation
- Channel creation and management
- Non-blocking channel operations
- Timeout channel support
- Comprehensive testing framework
- Pure CURSED concurrency primitives

## Core Concepts

### Select Operations

Select statements enable non-blocking communication on multiple channels simultaneously, choosing the first available operation.

### Channel Types

- **Buffered Channels**: Channels with internal buffer for storing messages
- **Unbuffered Channels**: Synchronous channels requiring sender/receiver coordination
- **Timeout Channels**: Special channels for implementing timeouts

## Constants and Types

### Operation Types

```cursed
sus SELECT_RECEIVE := 0     // Receive operation
sus SELECT_SEND := 1        // Send operation  
sus SELECT_DEFAULT := -1    // Default case
```

### Result Codes

```cursed
sus SELECT_COMPLETED := 0          // Operation completed successfully
sus SELECT_DEFAULT_EXECUTED := -1  // Default case executed
sus SELECT_TIMEOUT := -2           // Operation timed out
sus SELECT_ALL_CLOSED := -3        // All channels closed
sus SELECT_ERROR := -4             // Error occurred
```

## Core Functions

### Select Operations

#### `select_prepare(num_cases: normie) -> normie`
Prepares a new select operation context.

**Parameters:**
- `num_cases`: Number of cases in the select statement

**Returns:** Select ID for subsequent operations

**Example:**
```cursed
yeet "select_core"

// Prepare select with 3 cases (2 channels + default)
sus select_id normie = select_prepare(3)
```

#### `select_add_case(select_id: normie, channel_id: normie, operation_type: normie, value: tea) -> normie`
Adds a case to the select operation.

**Parameters:**
- `select_id`: Select operation identifier
- `channel_id`: Channel to operate on
- `operation_type`: SELECT_RECEIVE, SELECT_SEND, or SELECT_DEFAULT
- `value`: Value to send (for send operations)

**Returns:** Case index or -1 on error

#### `select_execute(select_id: normie, has_default: lit) -> normie`
Executes the select operation, returning the ready case.

**Parameters:**
- `select_id`: Select operation identifier
- `has_default`: Whether select has a default case

**Returns:** Index of ready case or special result code

#### `select_execute_with_timeout(select_id: normie, has_default: lit, timeout_ms: normie) -> normie`
Executes select with timeout support.

**Parameters:**
- `timeout_ms`: Timeout in milliseconds

**Returns:** Case index or SELECT_TIMEOUT

#### `select_cleanup(select_id: normie)`
Cleans up select operation resources.

### Channel Operations

#### `channel_create(buffer_size: normie) -> normie`
Creates a new channel with specified buffer size.

**Parameters:**
- `buffer_size`: Buffer size (0 for unbuffered channel)

**Returns:** Channel ID

**Example:**
```cursed
// Create buffered channel with capacity 5
sus buffered_ch normie = channel_create(5)

// Create unbuffered (synchronous) channel
sus sync_ch normie = channel_create(0)
```

#### `channel_send(channel_id: normie, value: tea) -> normie`
Sends value to channel (non-blocking).

**Parameters:**
- `channel_id`: Target channel
- `value`: Value to send

**Returns:** 0 on success, -1 if closed, -2 if would block

#### `channel_recv(channel_id: normie) -> tea`
Receives value from channel.

**Returns:** Received value or empty string if would block/closed

#### `channel_try_recv(channel_id: normie) -> tea`
Non-blocking receive attempt.

**Returns:** Received value or empty string if no data available

#### `channel_close(channel_id: normie)`
Closes channel for further sends.

#### `channel_destroy(channel_id: normie)`
Destroys channel and frees resources.

### Utility Functions

#### `channel_has_data(channel_id: normie) -> lit`
Checks if channel has data available for receiving.

#### `channel_can_send(channel_id: normie) -> lit`
Checks if channel can accept data for sending.

#### `create_timeout_channel(timeout_ms: normie) -> normie`
Creates a channel that receives a timeout signal after specified delay.

## Usage Examples

### Basic Select Statement

```cursed
yeet "select_core"

slay basic_select_example() {
    // Create channels
    sus ch1 normie = channel_create(1)
    sus ch2 normie = channel_create(1)
    
    // Send data to first channel
    channel_send(ch1, "message from ch1")
    
    // Prepare select operation
    sus select_id normie = select_prepare(2)
    
    // Add receive cases
    select_add_case(select_id, ch1, SELECT_RECEIVE, "")
    select_add_case(select_id, ch2, SELECT_RECEIVE, "")
    
    // Execute select
    sus result normie = select_execute(select_id, cringe)
    
    match result {
        0 -> {
            sus value tea = select_get_receive_value(select_id, 0)
            vibez.spill("Received from ch1: " + value)
        }
        1 -> {
            sus value tea = select_get_receive_value(select_id, 1)
            vibez.spill("Received from ch2: " + value)
        }
        SELECT_TIMEOUT -> {
            vibez.spill("No channels ready")
        }
    }
    
    // Cleanup
    select_cleanup(select_id)
    channel_destroy(ch1)
    channel_destroy(ch2)
}
```

### Select with Default Case

```cursed
slay select_with_default() {
    sus ch normie = channel_create(1)
    
    // Prepare select with default
    sus select_id normie = select_prepare(2)
    select_add_case(select_id, ch, SELECT_RECEIVE, "")
    select_add_case(select_id, 0, SELECT_DEFAULT, "")
    
    // Execute - will hit default since channel is empty
    sus result normie = select_execute(select_id, based)
    
    lowkey result == SELECT_DEFAULT_EXECUTED {
        vibez.spill("Default case executed - no data available")
    }
    
    select_cleanup(select_id)
    channel_destroy(ch)
}
```

### Select with Send Operations

```cursed
slay select_send_example() {
    sus ch1 normie = channel_create(1)  // Buffer size 1
    sus ch2 normie = channel_create(0)  // Unbuffered
    
    sus select_id normie = select_prepare(2)
    
    // Add send cases
    select_add_case(select_id, ch1, SELECT_SEND, "data for ch1")
    select_add_case(select_id, ch2, SELECT_SEND, "data for ch2")
    
    // Execute - ch1 should be ready (has buffer space)
    sus result normie = select_execute(select_id, cringe)
    
    lowkey result == 0 {
        vibez.spill("Successfully sent to ch1")
        
        // Verify data was sent
        sus received tea = channel_recv(ch1)
        vibez.spill("Confirmed: " + received)
    }
    
    select_cleanup(select_id)
    channel_destroy(ch1)
    channel_destroy(ch2)
}
```

### Timeout Handling

```cursed
slay timeout_example() {
    sus ch normie = channel_create(1)
    sus timeout_ch normie = create_timeout_channel(1000)  // 1 second timeout
    
    sus select_id normie = select_prepare(2)
    select_add_case(select_id, ch, SELECT_RECEIVE, "")
    select_add_case(select_id, timeout_ch, SELECT_RECEIVE, "")
    
    // Execute with timeout
    sus result normie = select_execute_with_timeout(select_id, cringe, 1000)
    
    match result {
        0 -> {
            vibez.spill("Received data from main channel")
        }
        1 -> {
            vibez.spill("Operation timed out")
        }
        SELECT_TIMEOUT -> {
            vibez.spill("Select timed out")
        }
    }
    
    select_cleanup(select_id)
    channel_destroy(ch)
    channel_destroy(timeout_ch)
}
```

### Producer-Consumer Pattern

```cursed
slay producer_consumer_example() {
    sus work_queue normie = channel_create(10)  // Buffer for work items
    sus done_ch normie = channel_create(1)      // Completion signal
    
    // Simulate producer
    bestie i := 0; i < 5; i = i + 1 {
        sus work_item tea = "task_" + string(i)
        channel_send(work_queue, work_item)
    }
    
    // Simulate consumer with select
    sus processed_count normie = 0
    bestie processed_count < 5 {
        sus select_id normie = select_prepare(2)
        select_add_case(select_id, work_queue, SELECT_RECEIVE, "")
        select_add_case(select_id, done_ch, SELECT_RECEIVE, "")
        
        sus result normie = select_execute(select_id, cringe)
        
        match result {
            0 -> {
                sus task tea = select_get_receive_value(select_id, 0)
                vibez.spill("Processing: " + task)
                processed_count = processed_count + 1
            }
            1 -> {
                vibez.spill("Received done signal")
                select_cleanup(select_id)
                break
            }
            SELECT_TIMEOUT -> {
                vibez.spill("No work available")
            }
        }
        
        select_cleanup(select_id)
    }
    
    channel_destroy(work_queue)
    channel_destroy(done_ch)
}
```

### Fan-out Pattern

```cursed
slay fan_out_example() {
    sus input_ch normie = channel_create(5)
    sus output1_ch normie = channel_create(5)
    sus output2_ch normie = channel_create(5)
    sus output3_ch normie = channel_create(5)
    
    // Send input data
    bestie i := 0; i < 3; i = i + 1 {
        channel_send(input_ch, "data_" + string(i))
    }
    
    // Fan out to multiple channels
    bestie channel_has_data(input_ch) {
        sus data tea = channel_recv(input_ch)
        
        // Select which output channel to use
        sus select_id normie = select_prepare(3)
        select_add_case(select_id, output1_ch, SELECT_SEND, data)
        select_add_case(select_id, output2_ch, SELECT_SEND, data)
        select_add_case(select_id, output3_ch, SELECT_SEND, data)
        
        sus result normie = select_execute(select_id, cringe)
        
        match result {
            0 -> vibez.spill("Sent to output1: " + data)
            1 -> vibez.spill("Sent to output2: " + data)
            2 -> vibez.spill("Sent to output3: " + data)
        }
        
        select_cleanup(select_id)
    }
    
    // Cleanup
    channel_destroy(input_ch)
    channel_destroy(output1_ch)
    channel_destroy(output2_ch)
    channel_destroy(output3_ch)
}
```

## Advanced Features

### Channel State Management

```cursed
// Channel state information
squad ChannelState {
    spill channel_id normie
    spill buffer_size normie
    spill buffer_count normie
    spill closed lit
    spill buffer []tea
}

// Get channel state for debugging
slay get_channel_state(channel_id normie) ChannelState {
    // Access internal channel state
    // Return debugging information
    damn state
}
```

### Performance Monitoring

```cursed
// Track select operation performance
squad SelectMetrics {
    spill total_operations normie
    spill timeout_count normie
    spill default_executions normie
    spill average_wait_time normie
}

slay collect_select_metrics() SelectMetrics {
    // Gather performance statistics
    // Track operation patterns
    damn metrics
}
```

### Error Recovery

```cursed
// Robust error handling for channels
slay safe_channel_operation(operation tea, channel_id normie, value tea) (tea, tea) {
    // Validate channel ID
    lowkey !is_valid_channel(channel_id) {
        damn "", "invalid channel ID"
    }
    
    // Attempt operation with error handling
    match operation {
        "send" -> {
            sus result normie = channel_send(channel_id, value)
            match result {
                0 -> damn "success", ""
                -1 -> damn "", "channel closed"
                -2 -> damn "", "would block"
                default -> damn "", "unknown error"
            }
        }
        "recv" -> {
            sus received tea = channel_recv(channel_id)
            lowkey string_length(received) > 0 {
                damn received, ""
            } else {
                damn "", "no data available"
            }
        }
        default -> {
            damn "", "unsupported operation"
        }
    }
}
```

## Testing Framework

### Comprehensive Test Suite

The module includes extensive tests for all functionality:

#### `test_select_preparation()`
Tests select context preparation and cleanup.

#### `test_channel_creation()`
Tests channel creation with various buffer sizes.

#### `test_channel_send_receive()`
Tests basic send/receive operations.

#### `test_channel_buffering()`
Tests buffer overflow and underflow conditions.

#### `test_channel_closing()`
Tests channel closing behavior.

#### `test_select_operations()`
Tests select with multiple channels.

#### `test_select_with_default()`
Tests default case execution.

#### `test_select_send_operations()`
Tests select with send operations.

#### `test_timeout_channel()`
Tests timeout channel functionality.

#### `test_select_timeout()`
Tests select timeout behavior.

### Running Tests

```bash
# Run select core tests
zig build test
./zig-out/bin/cursed-zig stdlib/select_core/test_select_core.csd

# Run specific test
./zig-out/bin/cursed-zig -c "
yeet 'select_core'
test_channel_send_receive()
"
```

### Test Examples

```cursed
// Example test implementation
slay test_custom_scenario() {
    test_start("Custom Select Scenario")
    
    // Create test channels
    sus ch1 normie = channel_create(2)
    sus ch2 normie = channel_create(0)
    
    // Test scenario setup
    channel_send(ch1, "test_data")
    
    // Create select operation
    sus select_id normie = select_prepare(2)
    sus case1 normie = select_add_case(select_id, ch1, SELECT_RECEIVE, "")
    sus case2 normie = select_add_case(select_id, ch2, SELECT_RECEIVE, "")
    
    // Execute and verify
    sus result normie = select_execute(select_id, cringe)
    assert_eq_int(result, 0)  // Should select ch1
    
    sus value tea = select_get_receive_value(select_id, 0)
    assert_eq_string(value, "test_data")
    
    // Cleanup
    select_cleanup(select_id)
    channel_destroy(ch1)
    channel_destroy(ch2)
    
    print_test_summary()
}
```

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| select_prepare | O(1) | Constant time initialization |
| select_add_case | O(1) | Add case to array |
| select_execute | O(n) | Check n cases for readiness |
| channel_send | O(1) | Array append operation |
| channel_recv | O(n) | Array shift operation |
| channel_create | O(1) | Initialize channel state |

### Space Complexity

- **Select contexts**: O(n) where n is number of cases
- **Channel buffers**: O(b) where b is buffer size
- **Global state**: O(c) where c is number of channels

### Performance Optimizations

```cursed
// Optimize channel operations for common cases
slay optimize_channel_recv(channel_id normie) tea {
    // Fast path for buffered channels
    lowkey channel_has_data(channel_id) {
        damn channel_recv(channel_id)
    }
    
    // Slow path for empty channels
    damn ""
}

// Optimize select for single channel
slay optimize_single_channel_select(channel_id normie, timeout_ms normie) tea {
    // Skip full select overhead for single channel
    lowkey channel_has_data(channel_id) {
        damn channel_recv(channel_id)
    }
    
    // Use simplified timeout logic
    damn wait_for_single_channel(channel_id, timeout_ms)
}
```

## Dependencies

```cursed
yeet "testz"  // Testing framework only
```

**Note:** The module is self-contained with no runtime dependencies.

## Integration Examples

### With Goroutines (Future)

```cursed
// Integration with goroutine system (planned)
slay spawn_worker(work_ch normie, result_ch normie) {
    stan {  // Go-style goroutine
        bestie based {
            sus select_id normie = select_prepare(1)
            select_add_case(select_id, work_ch, SELECT_RECEIVE, "")
            
            sus result normie = select_execute(select_id, cringe)
            lowkey result == 0 {
                sus work tea = select_get_receive_value(select_id, 0)
                sus processed tea = process_work(work)
                channel_send(result_ch, processed)
            }
            
            select_cleanup(select_id)
        }
    }
}
```

### With Event Systems

```cursed
// Event-driven programming with select
squad EventSystem {
    spill event_ch normie
    spill stop_ch normie
    spill handlers map[tea]function
}

slay (es *EventSystem) run_event_loop() {
    bestie based {
        sus select_id normie = select_prepare(2)
        select_add_case(select_id, es.event_ch, SELECT_RECEIVE, "")
        select_add_case(select_id, es.stop_ch, SELECT_RECEIVE, "")
        
        sus result normie = select_execute(select_id, cringe)
        
        match result {
            0 -> {
                sus event tea = select_get_receive_value(select_id, 0)
                es.handle_event(event)
            }
            1 -> {
                vibez.spill("Stopping event loop")
                select_cleanup(select_id)
                break
            }
        }
        
        select_cleanup(select_id)
    }
}
```

## Architecture

### Modular Design

1. **Core Layer**: Basic select and channel operations
2. **State Layer**: Context and channel state management  
3. **Utility Layer**: Helper functions and optimizations
4. **Test Layer**: Comprehensive validation framework

### Extension Points

- **Custom channel types**: Specialized channel implementations
- **Alternative schedulers**: Different scheduling algorithms
- **Performance monitoring**: Detailed metrics collection
- **Integration hooks**: Callbacks for external systems

The `select_core` module provides a solid foundation for concurrent programming in CURSED with Go-style channel semantics and select statements.
