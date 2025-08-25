fr fr channels - Advanced Channel Operations Module
fr fr Extended channel functionality beyond basic concurrenz module
fr fr Provides buffered channels, select operations, and channel utilities

yeet "core"
yeet "concurrenz" 
yeet "sync"

fr fr Channel operation constants
fact CHAN_SEND normie = 1
fact CHAN_RECV normie = 2
fact CHAN_CLOSED normie = 3
fact CHAN_TIMEOUT normie = 4

fact CHAN_SUCCESS normie = 0
fact CHAN_WOULD_BLOCK normie = 1
fact CHAN_BUFFER_FULL normie = 2
fact CHAN_BUFFER_EMPTY normie = 3

fr fr ===== BUFFERED CHANNEL IMPLEMENTATION =====

struct BufferedChannel {
    buffer [tea],           # Message buffer
    capacity normie,        # Maximum buffer size
    head normie,           # Head index for circular buffer
    tail normie,           # Tail index for circular buffer
    count normie,          # Current number of messages
    closed lit,            # Whether channel is closed
    send_waiters normie,   # Number of goroutines waiting to send
    recv_waiters normie,   # Number of goroutines waiting to receive
    mutex sync.Mutex       # Synchronization mutex
}

slay new_buffered_channel(capacity normie) BufferedChannel {
    check capacity <= 0 {
        capacity = 1  # Minimum capacity
    }
    
    sus chan BufferedChannel = BufferedChannel{
        buffer: make_buffer(capacity),
        capacity: capacity,
        head: 0,
        tail: 0,
        count: 0,
        closed: cap,
        send_waiters: 0,
        recv_waiters: 0,
        mutex: sync.new_mutex()
    }
    damn chan
}

slay buffered_send(chan *BufferedChannel, message tea) normie {
    sync.mutex_lock(&chan.mutex)
    
    check chan.closed {
        sync.mutex_unlock(&chan.mutex)
        damn CHAN_CLOSED
    }
    
    # If buffer has space, add message
    check chan.count < chan.capacity {
        chan.buffer[chan.tail] = message
        chan.tail = (chan.tail + 1) % chan.capacity
        chan.count = chan.count + 1
        
        # Wake up waiting receivers
        check chan.recv_waiters > 0 {
            concurrenz.wake_goroutine()
            chan.recv_waiters = chan.recv_waiters - 1
        }
        
        sync.mutex_unlock(&chan.mutex)
        damn CHAN_SUCCESS
    }
    
    # Buffer is full, wait
    chan.send_waiters = chan.send_waiters + 1
    sync.mutex_unlock(&chan.mutex)
    
    # Block until space available
    concurrenz.yield()
    
    # Try again after waking up
    damn buffered_send(chan, message)
}

slay buffered_recv(chan *BufferedChannel) (tea, normie) {
    sync.mutex_lock(&chan.mutex)
    
    # If buffer has messages, return one
    check chan.count > 0 {
        sus message tea = chan.buffer[chan.head]
        chan.head = (chan.head + 1) % chan.capacity
        chan.count = chan.count - 1
        
        # Wake up waiting senders
        check chan.send_waiters > 0 {
            concurrenz.wake_goroutine()
            chan.send_waiters = chan.send_waiters - 1
        }
        
        sync.mutex_unlock(&chan.mutex)
        damn message, CHAN_SUCCESS
    }
    
    # If channel is closed and buffer empty
    check chan.closed {
        sync.mutex_unlock(&chan.mutex)
        damn "", CHAN_CLOSED
    }
    
    # Buffer is empty, wait
    chan.recv_waiters = chan.recv_waiters + 1
    sync.mutex_unlock(&chan.mutex)
    
    # Block until message available
    concurrenz.yield()
    
    # Try again after waking up
    damn buffered_recv(chan)
}

slay buffered_try_send(chan *BufferedChannel, message tea) normie {
    sync.mutex_lock(&chan.mutex)
    
    check chan.closed {
        sync.mutex_unlock(&chan.mutex)
        damn CHAN_CLOSED
    }
    
    check chan.count >= chan.capacity {
        sync.mutex_unlock(&chan.mutex)
        damn CHAN_WOULD_BLOCK
    }
    
    chan.buffer[chan.tail] = message
    chan.tail = (chan.tail + 1) % chan.capacity
    chan.count = chan.count + 1
    
    # Wake up waiting receivers
    check chan.recv_waiters > 0 {
        concurrenz.wake_goroutine()
        chan.recv_waiters = chan.recv_waiters - 1
    }
    
    sync.mutex_unlock(&chan.mutex)
    damn CHAN_SUCCESS
}

slay buffered_try_recv(chan *BufferedChannel) (tea, normie) {
    sync.mutex_lock(&chan.mutex)
    
    check chan.count == 0 {
        check chan.closed {
            sync.mutex_unlock(&chan.mutex)
            damn "", CHAN_CLOSED
        }
        sync.mutex_unlock(&chan.mutex)
        damn "", CHAN_WOULD_BLOCK
    }
    
    sus message tea = chan.buffer[chan.head]
    chan.head = (chan.head + 1) % chan.capacity
    chan.count = chan.count - 1
    
    # Wake up waiting senders
    check chan.send_waiters > 0 {
        concurrenz.wake_goroutine()
        chan.send_waiters = chan.send_waiters - 1
    }
    
    sync.mutex_unlock(&chan.mutex)
    damn message, CHAN_SUCCESS
}

slay buffered_close(chan *BufferedChannel) {
    sync.mutex_lock(&chan.mutex)
    chan.closed = based
    
    # Wake up all waiting goroutines
    bestie chan.send_waiters > 0 {
        concurrenz.wake_goroutine()
        chan.send_waiters = chan.send_waiters - 1
    }
    
    bestie chan.recv_waiters > 0 {
        concurrenz.wake_goroutine()
        chan.recv_waiters = chan.recv_waiters - 1
    }
    
    sync.mutex_unlock(&chan.mutex)
}

fr fr ===== SELECT OPERATION IMPLEMENTATION =====

struct SelectCase {
    channel_ptr normie,     # Pointer to channel (simplified as int)
    operation normie,       # CHAN_SEND or CHAN_RECV
    message tea,           # Message to send (for send operations)
    is_ready lit           # Whether this case is ready
}

struct SelectResult {
    case_index normie,      # Which case was selected (-1 for default/timeout)
    message tea,           # Received message (for recv operations)
    success lit            # Whether operation succeeded
}

slay new_select_case_send(channel_ptr normie, message tea) SelectCase {
    sus case SelectCase = SelectCase{
        channel_ptr: channel_ptr,
        operation: CHAN_SEND,
        message: message,
        is_ready: cap
    }
    damn case
}

slay new_select_case_recv(channel_ptr normie) SelectCase {
    sus case SelectCase = SelectCase{
        channel_ptr: channel_ptr,
        operation: CHAN_RECV,
        message: "",
        is_ready: cap
    }
    damn case
}

slay select_non_blocking(cases []SelectCase) SelectResult {
    sus case_count normie = len_select_cases(cases)
    
    # Check each case for readiness
    sus i normie = 0
    bestie i < case_count {
        check cases[i].operation == CHAN_SEND {
            # Check if send would succeed
            sus status normie = check_send_ready(cases[i].channel_ptr)
            check status == CHAN_SUCCESS {
                # Perform the send
                perform_send(cases[i].channel_ptr, cases[i].message)
                sus result SelectResult = SelectResult{
                    case_index: i,
                    message: "",
                    success: based
                }
                damn result
            }
        } elseif cases[i].operation == CHAN_RECV {
            # Check if recv would succeed
            sus message tea, status normie = check_recv_ready(cases[i].channel_ptr)
            check status == CHAN_SUCCESS {
                sus result SelectResult = SelectResult{
                    case_index: i,
                    message: message,
                    success: based
                }
                damn result
            }
        }
        i = i + 1
    }
    
    # No case ready
    sus result SelectResult = SelectResult{
        case_index: -1,
        message: "",
        success: cap
    }
    damn result
}

slay select_blocking(cases []SelectCase) SelectResult {
    # Keep trying until a case becomes ready
    bestie based {
        sus result SelectResult = select_non_blocking(cases)
        check result.success {
            damn result
        }
        
        # Yield to other goroutines
        concurrenz.yield()
    }
    
    # Should never reach here
    sus empty_result SelectResult = SelectResult{case_index: -1, success: cap}
    damn empty_result
}

slay select_with_timeout(cases []SelectCase, timeout_ms normie) SelectResult {
    sus start_time normie = core.get_timestamp_millis()
    
    bestie based {
        sus result SelectResult = select_non_blocking(cases)
        check result.success {
            damn result
        }
        
        sus current_time normie = core.get_timestamp_millis()
        check current_time - start_time > timeout_ms {
            # Timeout occurred
            sus timeout_result SelectResult = SelectResult{
                case_index: -1,
                message: "",
                success: cap
            }
            damn timeout_result
        }
        
        # Short sleep to avoid busy waiting
        core.sleep_millis(1)
    }
    
    # Should never reach here
    sus empty_result SelectResult = SelectResult{case_index: -1, success: cap}
    damn empty_result
}

fr fr ===== CHANNEL UTILITIES =====

slay channel_len(chan *BufferedChannel) normie {
    sync.mutex_lock(&chan.mutex)
    sus length normie = chan.count
    sync.mutex_unlock(&chan.mutex)
    damn length
}

slay channel_cap(chan *BufferedChannel) normie {
    damn chan.capacity
}

slay channel_is_closed(chan *BufferedChannel) lit {
    sync.mutex_lock(&chan.mutex)
    sus closed lit = chan.closed
    sync.mutex_unlock(&chan.mutex)
    damn closed
}

slay channel_is_full(chan *BufferedChannel) lit {
    sync.mutex_lock(&chan.mutex)
    sus full lit = chan.count >= chan.capacity
    sync.mutex_unlock(&chan.mutex)
    damn full
}

slay channel_is_empty(chan *BufferedChannel) lit {
    sync.mutex_lock(&chan.mutex)
    sus empty lit = chan.count == 0
    sync.mutex_unlock(&chan.mutex)
    damn empty
}

fr fr ===== CHANNEL PATTERNS =====

slay fan_out(input_chan *BufferedChannel, output_chans []BufferedChannel) {
    go {
        bestie based {
            sus message tea, status normie = buffered_recv(input_chan)
            check status == CHAN_CLOSED {
                break
            }
            check status == CHAN_SUCCESS {
                # Send to all output channels
                sus i normie = 0
                bestie i < len_buffered_channels(output_chans) {
                    go {
                        buffered_send(&output_chans[i], message)
                    }
                    i = i + 1
                }
            }
        }
    }
}

slay fan_in(input_chans []BufferedChannel, output_chan *BufferedChannel) {
    sus i normie = 0
    bestie i < len_buffered_channels(input_chans) {
        sus chan_index normie = i
        go {
            bestie based {
                sus message tea, status normie = buffered_recv(&input_chans[chan_index])
                check status == CHAN_CLOSED {
                    break
                }
                check status == CHAN_SUCCESS {
                    buffered_send(output_chan, message)
                }
            }
        }
        i = i + 1
    }
}

slay pipeline_stage(input_chan *BufferedChannel, output_chan *BufferedChannel, 
                    processor func(tea) tea) {
    go {
        bestie based {
            sus message tea, status normie = buffered_recv(input_chan)
            check status == CHAN_CLOSED {
                buffered_close(output_chan)
                break
            }
            check status == CHAN_SUCCESS {
                sus processed tea = processor(message)
                buffered_send(output_chan, processed)
            }
        }
    }
}

fr fr ===== HELPER FUNCTIONS =====

slay make_buffer(size normie) [tea] {
    sus buffer [tea] = []
    sus i normie = 0
    bestie i < size {
        buffer = append_string(buffer, "")
        i = i + 1
    }
    damn buffer
}

slay check_send_ready(channel_ptr normie) normie {
    # Simplified check - in real implementation would check actual channel
    damn CHAN_SUCCESS
}

slay check_recv_ready(channel_ptr normie) (tea, normie) {
    # Simplified check - in real implementation would check actual channel
    damn "test_message", CHAN_SUCCESS
}

slay perform_send(channel_ptr normie, message tea) {
    # Simplified send - in real implementation would send to actual channel
    vibez.spill("Sending: " + message)
}

slay len_select_cases(cases []SelectCase) normie {
    damn 0  # Simplified implementation
}

slay len_buffered_channels(chans []BufferedChannel) normie {
    damn 0  # Simplified implementation
}

slay append_string(arr [tea], str tea) [tea] {
    damn arr  # Simplified implementation
}

fr fr ===== ADVANCED CHANNEL TYPES =====

struct PriorityChannel {
    high_chan BufferedChannel,
    normal_chan BufferedChannel,
    low_chan BufferedChannel
}

slay new_priority_channel(capacity normie) PriorityChannel {
    sus pchan PriorityChannel = PriorityChannel{
        high_chan: new_buffered_channel(capacity),
        normal_chan: new_buffered_channel(capacity),
        low_chan: new_buffered_channel(capacity)
    }
    damn pchan
}

slay priority_send_high(pchan *PriorityChannel, message tea) normie {
    damn buffered_send(&pchan.high_chan, message)
}

slay priority_send_normal(pchan *PriorityChannel, message tea) normie {
    damn buffered_send(&pchan.normal_chan, message)
}

slay priority_send_low(pchan *PriorityChannel, message tea) normie {
    damn buffered_send(&pchan.low_chan, message)
}

slay priority_recv(pchan *PriorityChannel) (tea, normie) {
    # Try high priority first
    sus message tea, status normie = buffered_try_recv(&pchan.high_chan)
    check status == CHAN_SUCCESS {
        damn message, status
    }
    
    # Try normal priority
    message, status = buffered_try_recv(&pchan.normal_chan)
    check status == CHAN_SUCCESS {
        damn message, status
    }
    
    # Try low priority (blocking)
    damn buffered_recv(&pchan.low_chan)
}

fr fr ===== MODULE INITIALIZATION =====

slay init_channels() {
    vibez.spill("channels module initialized")
}

slay get_channels_info() tea {
    damn "channels v1.0 - Advanced Channel Operations for Concurrent CURSED"
}
