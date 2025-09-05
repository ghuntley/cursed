# CURSED Channel Examples

This directory contains comprehensive examples demonstrating the CURSED channel system for concurrent programming. Each example builds on the previous ones, starting with basic concepts and progressing to advanced patterns.

## Quick Start

To run any example:

```bash
# Build the CURSED compiler first
make build

# Run an example
./target/debug/cursed examples/channels/hello_channels.💀
```

## Example Overview

### 1. Basic Channel Operations

**File:** [`hello_channels.💀`](hello_channels.💀)  
**Concepts:** Channel creation, send/receive, buffered vs unbuffered, closing channels

This example introduces the fundamental concepts of channels in CURSED:
- Creating unbuffered and buffered channels with `make(dm<T>)`
- Basic send (`ch <- value`) and receive (`value = <-ch`) operations
- Understanding blocking vs non-blocking behavior
- Channel closing and detecting closure

**Key Takeaways:**
- Unbuffered channels provide synchronization points
- Buffered channels allow asynchronous communication
- Always close channels when done sending
- Use the two-value receive to detect channel closure

### 2. Producer-Consumer Pattern

**File:** [`producer_consumer.💀`](producer_consumer.💀)  
**Concepts:** Work distribution, multiple producers/consumers, inventory monitoring

Demonstrates the classic producer-consumer pattern with:
- Multiple producers creating products concurrently
- Multiple consumers processing products
- Channel buffering for performance optimization
- Monitoring and statistics collection
- Buffer size impact on throughput

**Key Takeaways:**
- Use buffered channels to decouple producer and consumer timing
- Multiple consumers can share work from a single channel
- Monitor channel performance with different buffer sizes
- Proper channel closing prevents consumer hangs

### 3. Worker Pool Pattern

**File:** [`worker_pool.💀`](worker_pool.💀)  
**Concepts:** Parallel job processing, error handling, priority queues

Shows how to implement scalable parallel processing:
- Fixed number of workers processing variable workload
- Different job types with varying processing requirements
- Error handling and result collection
- Priority-based work distribution
- Performance monitoring and statistics

**Key Takeaways:**
- Worker pools provide controlled concurrency
- Use structured job and result types for complex workflows
- Implement priority queues for critical work
- Collect performance metrics for optimization

### 4. Pipeline Processing

**File:** [`pipeline.💀`](pipeline.💀)  
**Concepts:** Multi-stage processing, data transformation, parallel pipelines

Demonstrates data processing pipelines with:
- Sequential processing stages (generation → validation → transformation → enrichment → output)
- Error propagation through pipeline stages
- Parallel processing branches
- Result merging from multiple processors
- Pipeline monitoring and statistics

**Key Takeaways:**
- Pipelines break complex processing into manageable stages
- Each stage can have different performance characteristics
- Use parallel branches for independent processing
- Implement comprehensive error tracking

### 5. Fan-in and Fan-out Patterns

**File:** [`fan_in_out.💀`](fan_in_out.💀)  
**Concepts:** Work distribution/aggregation, load balancing, scatter-gather

Advanced patterns for work distribution and collection:
- **Fan-out**: Distribute work from one source to multiple workers
- **Fan-in**: Merge results from multiple sources
- **Load balancing**: Send work to first available worker
- **Priority fan-in**: Merge with priority handling
- **Scatter-gather**: Send queries to multiple services and collect results

**Key Takeaways:**
- Fan-out enables parallel processing of identical work
- Fan-in aggregates results while preserving order/priority
- Load balancing maximizes resource utilization
- Scatter-gather patterns enable redundancy and performance

### 6. Advanced Select Operations

**File:** [`channel_select.💀`](channel_select.💀)  
**Concepts:** Multi-channel coordination, timeouts, priority handling, rate limiting

Complex channel coordination patterns using `vibe_check` (select):
- **Event coordination**: Handle multiple event streams with priorities
- **Service monitoring**: Health checks with timeouts and status aggregation
- **Priority dispatching**: Process work based on priority levels
- **Rate limiting**: Control request processing rates with token buckets

**Key Takeaways:**
- `vibe_check` enables non-blocking multi-channel operations
- Combine channels with timeouts for robust service operations
- Implement priority systems with multiple channels
- Use token bucket pattern for rate limiting

## Running the Examples

### Prerequisites

1. Build the CURSED compiler:
```bash
make build
```

2. Ensure the channel runtime is properly linked:
```bash
./fix_linking.sh cargo build
```

### Individual Examples

```bash
# Basic channel operations
./target/debug/cursed examples/channels/hello_channels.💀

# Producer-consumer pattern
./target/debug/cursed examples/channels/producer_consumer.💀

# Worker pool processing
./target/debug/cursed examples/channels/worker_pool.💀

# Pipeline processing
./target/debug/cursed examples/channels/pipeline.💀

# Fan-in/fan-out patterns
./target/debug/cursed examples/channels/fan_in_out.💀

# Advanced select operations
./target/debug/cursed examples/channels/channel_select.💀
```

### Run All Examples

```bash
# Run all channel examples in sequence
for example in examples/channels/*.💀; do
    echo "Running $example..."
    ./target/debug/cursed "$example"
    echo "Completed $example"
    echo "---"
done
```

## Example Output

Each example produces detailed output showing:
- Channel operations and message flow
- Goroutine coordination and timing
- Performance statistics and metrics
- Error handling and edge cases
- Pattern-specific behavior demonstrations

Example output from `hello_channels.💀`:
```
=== CURSED Channel Hello World ===
Creating channels...
Main: Waiting for greeting...
Sender goroutine: Sending greeting...
Main: Received greeting: Hello from CURSED channels!
Main: Waiting for number...
Sender goroutine: Sending number...
Main: Received number: 42
Sender goroutine: Done sending
=== Basic Channel Demo Complete ===
```

## Code Organization

Each example follows consistent patterns:

### Structure
- **Type definitions**: Custom structs for data flowing through channels
- **Worker functions**: Goroutines that process channel data
- **Coordinator functions**: Manage channel lifecycle and coordination
- **Demo functions**: Demonstrate specific patterns
- **Main function**: Orchestrates the example execution

### Naming Conventions
- **Channels**: `work_ch`, `result_ch`, `status_ch`, etc.
- **Goroutine functions**: `worker()`, `producer()`, `consumer()`, etc.
- **Demo functions**: `basic_demo()`, `advanced_demo()`, etc.
- **Types**: `Job`, `Result`, `Status`, `Message`, etc.

### Error Handling
Examples demonstrate various error handling patterns:
- Channel closure detection with two-value receive
- Timeout handling with `time.After()`
- Graceful shutdown with dedicated channels
- Error propagation through result structures

## Performance Considerations

### Buffer Sizing Guidelines

| Use Case | Recommended Buffer Size | Rationale |
|----------|------------------------|-----------|
| Synchronization | 0 (unbuffered) | Ensures coordination |
| Low latency | 1-5 | Minimal buffering |
| Moderate throughput | 10-100 | Balance latency/throughput |
| High throughput | 100-1000 | Maximize throughput |
| Batch processing | 1000+ | Handle bursts |

### Performance Tips

1. **Choose appropriate buffer sizes** based on your use case
2. **Use worker pools** for CPU-intensive tasks
3. **Implement backpressure** to prevent memory exhaustion
4. **Monitor channel lengths** to detect bottlenecks
5. **Use select with timeouts** for robust service operations

## Common Patterns Summary

| Pattern | Use Case | Key Benefits | Example File |
|---------|----------|--------------|--------------|
| **Producer-Consumer** | Work distribution | Decoupling, load balancing | `producer_consumer.💀` |
| **Worker Pool** | Parallel processing | Resource control, scalability | `worker_pool.💀` |
| **Pipeline** | Sequential processing | Modularity, parallelism | `pipeline.💀` |
| **Fan-out** | Work distribution | Parallel execution | `fan_in_out.💀` |
| **Fan-in** | Result aggregation | Data merging, coordination | `fan_in_out.💀` |
| **Select** | Multi-channel coordination | Non-blocking, timeouts | `channel_select.💀` |

## Troubleshooting

### Common Issues

1. **Deadlock**: Usually caused by unbuffered channels without proper goroutine coordination
   - **Solution**: Use buffered channels or ensure receiver is ready

2. **Goroutine hangs**: Often due to channels not being closed
   - **Solution**: Always close channels when done sending

3. **Memory leaks**: Goroutines waiting indefinitely on channels
   - **Solution**: Implement timeouts and graceful shutdown

4. **Performance issues**: Inappropriate buffer sizes or blocking operations
   - **Solution**: Profile channel usage and adjust buffer sizes

### Debugging Tips

1. **Add logging** to track channel operations
2. **Use channel length** (`len(ch)`) to monitor buffer usage
3. **Implement timeouts** to detect stuck operations
4. **Monitor goroutine counts** to detect leaks

## Next Steps

After working through these examples:

1. **Read the documentation**:
   - [Channel System Overview](../../docs/channels.md)
   - [Channel API Reference](../../docs/channel_api.md)
   - [Channel Tutorial](../../docs/channel_tutorial.md)
   - [Implementation Guide](../../docs/channel_implementation.md)

2. **Experiment with patterns**:
   - Modify buffer sizes and observe behavior
   - Combine patterns for complex workflows
   - Add error handling and monitoring

3. **Build your own examples**:
   - Create domain-specific channel patterns
   - Implement real-world use cases
   - Contribute examples back to the project

4. **Performance testing**:
   - Benchmark channel operations
   - Profile memory usage
   - Optimize for your specific use cases

These examples provide a solid foundation for understanding and using CURSED channels effectively in concurrent programs. The patterns demonstrated here are building blocks for more complex concurrent systems.
