//! Benchmark scenarios for testing various components of the CURSED language

use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, trace, warn, instrument};

use crate::memory::{GarbageCollector, Gc, Traceable, Visitor, Tag};
use crate::memory::thread_safe_gc::ThreadSafeGc;
use crate::object::Object;
use crate::object_thread_safe::ThreadSafeValue;

use super::harness::{Benchmark, BenchmarkSuite, BenchmarkConfig};
use super::metrics::{Metric, TimingMetric, MemoryMetric, ThroughputMetric};

/// Create a standard benchmark suite for overall language performance
pub fn standard_suite() -> BenchmarkSuite {
    let mut suite = BenchmarkSuite::new(
        "standard",
        "Standard benchmark suite for overall language performance",
    );
    
    // Add basic object allocation benchmark
    suite.add_benchmark(Benchmark::new(
        "object_allocation",
        "Basic object allocation performance",
        || {
            #[derive(Debug, Clone)]
            struct TestObject {
                id: usize,
                value: String,
            }
            
            impl Traceable for TestObject {
                fn trace(&self, _visitor: &mut dyn Visitor) {
                    // No references to trace
                }
                
                fn size(&self) -> usize {
                    std::mem::size_of::<Self>() + self.value.len()
                }
                
                fn tag(&self) -> Tag {
                    Tag::Object
                }
                
                fn finalize(&mut self) {
                    // Nothing to finalize
                }
            }
            
            // Benchmark function
            let gc = Arc::new(GarbageCollector::new());
            let start = Instant::now();
            
            // Allocate a bunch of objects
            const NUM_OBJECTS: usize = 10_000;
            let mut objects = Vec::with_capacity(NUM_OBJECTS);
            
            for i in 0..NUM_OBJECTS {
                let obj = TestObject {
                    id: i,
                    value: format!("Object #{}", i),
                };
                objects.push(gc.allocate(obj));
            }
            
            let elapsed = start.elapsed();
            debug!(elapsed_ms = ?elapsed.as_millis(), objects = NUM_OBJECTS, "Object allocation test complete");
            
            // Create metrics
            let mut metrics: Vec<Box<dyn Metric>> = Vec::new();
            
            metrics.push(Box::new(TimingMetric {
                name: "allocation_time".to_string(),
                duration: elapsed,
            }));
            
            metrics.push(Box::new(ThroughputMetric {
                name: "allocation_throughput".to_string(),
                operations: NUM_OBJECTS as u64,
                duration: elapsed,
                operation_unit: "objects".to_string(),
            }));
            
            // Memory metrics
            let mem_stats = gc.stats();
            metrics.push(Box::new(MemoryMetric {
                name: "memory_usage".to_string(),
                before_object_count: 0,
                after_object_count: mem_stats.object_count,
                before_total_size: 0,
                after_total_size: mem_stats.total_size,
                allocated: mem_stats.total_size,
                collected: mem_stats.total_collected,
                collection_time_ms: mem_stats.total_gc_time_ms,
            }));
            
            // Clean up
            drop(objects);
            gc.collect_garbage();
            
            metrics
        },
    ));
    
    suite.add_benchmark(Benchmark::new(
        "object_traversal",
        "Traversing object references",
        || {
            #[derive(Debug, Clone)]
            struct Node {
                id: usize,
                children: Vec<Gc<Node>>,
            }
            
            impl Traceable for Node {
                fn trace(&self, visitor: &mut dyn Visitor) {
                    for child in &self.children {
                        visitor.visit_ptr(child.id(), Tag::Object);
                    }
                }
                
                fn size(&self) -> usize {
                    std::mem::size_of::<Self>() + self.children.capacity() * std::mem::size_of::<Gc<Node>>()
                }
                
                fn tag(&self) -> Tag {
                    Tag::Object
                }
                
                fn finalize(&mut self) {
                    // Nothing to finalize
                }
            }
            
            // Create a tree structure
            const DEPTH: usize = 5;
            const BRANCHING: usize = 4;
            
            let gc = Arc::new(GarbageCollector::new());
            
            // Helper to create a tree of given depth
            fn create_tree(depth: usize, id: &mut usize, gc: &Arc<GarbageCollector>, branching: usize) -> Gc<Node> {
                let current_id = *id;
                *id += 1;
                
                let mut node = Node {
                    id: current_id,
                    children: Vec::new(),
                };
                
                if depth > 0 {
                    for _ in 0..branching {
                        node.children.push(create_tree(depth - 1, id, gc, branching));
                    }
                }
                
                gc.allocate(node)
            }
            
            // Create the tree
            let mut next_id = 0;
            let start_create = Instant::now();
            let root = create_tree(DEPTH, &mut next_id, &gc, BRANCHING);
            let create_elapsed = start_create.elapsed();
            
            // Now traverse the tree
            fn traverse(node: &Gc<Node>) -> usize {
                let mut count = 1;
                if let Some(inner) = node.inner() {
                    for child in &inner.children {
                        count += traverse(child);
                    }
                }
                count
            }
            
            let start_traverse = Instant::now();
            let nodes_visited = traverse(&root);
            let traverse_elapsed = start_traverse.elapsed();
            
            debug!(create_ms = ?create_elapsed.as_millis(), traverse_ms = ?traverse_elapsed.as_millis(), 
                  nodes = nodes_visited, "Tree traversal test complete");
            
            // Create metrics
            let mut metrics: Vec<Box<dyn Metric>> = Vec::new();
            
            metrics.push(Box::new(TimingMetric {
                name: "create_time".to_string(),
                duration: create_elapsed,
            }));
            
            metrics.push(Box::new(TimingMetric {
                name: "traverse_time".to_string(),
                duration: traverse_elapsed,
            }));
            
            metrics.push(Box::new(ThroughputMetric {
                name: "traverse_throughput".to_string(),
                operations: nodes_visited as u64,
                duration: traverse_elapsed,
                operation_unit: "nodes".to_string(),
            }));
            
            // Clean up
            drop(root);
            gc.collect_garbage();
            
            metrics
        },
    ));
    
    suite
}

/// Create a benchmark suite focused on garbage collector performance
pub fn gc_suite() -> BenchmarkSuite {
    let mut suite = BenchmarkSuite::new(
        "gc",
        "Benchmark suite focused on garbage collector performance",
    );
    
    // Add GC cycle detection benchmark
    suite.add_benchmark(Benchmark::new(
        "gc_cycle_detection",
        "Circular reference detection and collection",
        || {
            #[derive(Debug, Clone)]
            struct CircularNode {
                id: usize,
                next: Option<Gc<CircularNode>>,
            }
            
            impl Traceable for CircularNode {
                fn trace(&self, visitor: &mut dyn Visitor) {
                    if let Some(next) = &self.next {
                        visitor.visit_ptr(next.id(), Tag::Object);
                    }
                }
                
                fn size(&self) -> usize {
                    std::mem::size_of::<Self>()
                }
                
                fn tag(&self) -> Tag {
                    Tag::Object
                }
                
                fn finalize(&mut self) {
                    // Nothing to finalize
                }
            }
            
            let gc = Arc::new(GarbageCollector::new());
            
            // Create a circular chain with many nodes
            const NUM_NODES: usize = 10_000;
            
            let start_create = Instant::now();
            let mut nodes = Vec::with_capacity(NUM_NODES);
            
            // First create all nodes
            for i in 0..NUM_NODES {
                nodes.push(gc.allocate(CircularNode {
                    id: i,
                    next: None,
                }));
            }
            
            // Now connect them in a circular chain
            for i in 0..NUM_NODES {
                let next_idx = (i + 1) % NUM_NODES;
                if let Some(node) = nodes[i].inner_mut() {
                    node.next = Some(nodes[next_idx].clone());
                }
            }
            
            let create_elapsed = start_create.elapsed();
            
            // Get initial memory stats
            let before_stats = gc.stats();
            
            // Drop all references except the ones in the circular structure
            drop(nodes);
            
            // Force collection
            let start_collect = Instant::now();
            gc.collect_garbage();
            
            // Give GC a moment to finish any background work
            std::thread::sleep(Duration::from_millis(50));
            let collect_elapsed = start_collect.elapsed();
            
            // Get final memory stats
            let after_stats = gc.stats();
            
            debug!(create_ms = ?create_elapsed.as_millis(), collect_ms = ?collect_elapsed.as_millis(),
                   before_objects = before_stats.object_count, after_objects = after_stats.object_count,
                   "Cycle detection test complete");
            
            // Create metrics
            let mut metrics: Vec<Box<dyn Metric>> = Vec::new();
            
            metrics.push(Box::new(TimingMetric {
                name: "create_time".to_string(),
                duration: create_elapsed,
            }));
            
            metrics.push(Box::new(TimingMetric {
                name: "collect_time".to_string(),
                duration: collect_elapsed,
            }));
            
            metrics.push(Box::new(MemoryMetric {
                name: "memory_usage".to_string(),
                before_object_count: before_stats.object_count,
                after_object_count: after_stats.object_count,
                before_total_size: before_stats.total_size,
                after_total_size: after_stats.total_size,
                allocated: before_stats.total_size,
                collected: before_stats.total_size - after_stats.total_size,
                collection_time_ms: after_stats.total_gc_time_ms - before_stats.total_gc_time_ms,
            }));
            
            metrics
        },
    ));
    
    // Add large object collection benchmark
    suite.add_benchmark(Benchmark::new(
        "gc_large_object_collection",
        "Collection of large objects",
        || {
            #[derive(Debug, Clone)]
            struct LargeObject {
                id: usize,
                // Large buffer to make this object consume significant memory
                buffer: Vec<u8>,
            }
            
            impl Traceable for LargeObject {
                fn trace(&self, _visitor: &mut dyn Visitor) {
                    // No references to trace
                }
                
                fn size(&self) -> usize {
                    std::mem::size_of::<Self>() + self.buffer.capacity()
                }
                
                fn tag(&self) -> Tag {
                    Tag::Object
                }
                
                fn finalize(&mut self) {
                    // Nothing to finalize
                }
            }
            
            let gc = Arc::new(GarbageCollector::new());
            
            // Create some large objects
            const NUM_OBJECTS: usize = 100;
            const BUFFER_SIZE: usize = 100_000; // 100 KB per object
            
            let start_create = Instant::now();
            let mut objects = Vec::with_capacity(NUM_OBJECTS);
            
            for i in 0..NUM_OBJECTS {
                let mut buffer = Vec::with_capacity(BUFFER_SIZE);
                buffer.resize(BUFFER_SIZE, 0);
                
                objects.push(gc.allocate(LargeObject {
                    id: i,
                    buffer,
                }));
            }
            
            let create_elapsed = start_create.elapsed();
            
            // Get initial memory stats
            let before_stats = gc.stats();
            
            // Drop half the objects
            objects.truncate(NUM_OBJECTS / 2);
            
            // Force collection
            let start_collect = Instant::now();
            gc.collect_garbage();
            let collect_elapsed = start_collect.elapsed();
            
            // Get final memory stats
            let after_stats = gc.stats();
            
            debug!(create_ms = ?create_elapsed.as_millis(), collect_ms = ?collect_elapsed.as_millis(),
                   before_objects = before_stats.object_count, after_objects = after_stats.object_count,
                   "Large object collection test complete");
            
            // Create metrics
            let mut metrics: Vec<Box<dyn Metric>> = Vec::new();
            
            metrics.push(Box::new(TimingMetric {
                name: "create_time".to_string(),
                duration: create_elapsed,
            }));
            
            metrics.push(Box::new(TimingMetric {
                name: "collect_time".to_string(),
                duration: collect_elapsed,
            }));
            
            metrics.push(Box::new(MemoryMetric {
                name: "memory_usage".to_string(),
                before_object_count: before_stats.object_count,
                after_object_count: after_stats.object_count,
                before_total_size: before_stats.total_size,
                after_total_size: after_stats.total_size,
                allocated: before_stats.total_size,
                collected: before_stats.total_size - after_stats.total_size,
                collection_time_ms: after_stats.total_gc_time_ms - before_stats.total_gc_time_ms,
            }));
            
            metrics
        },
    ));
    
    suite
}

/// Create a benchmark suite focused on concurrency performance
pub fn concurrency_suite() -> BenchmarkSuite {
    let mut suite = BenchmarkSuite::new(
        "concurrency",
        "Benchmark suite focused on concurrency performance",
    );
    
    // Add channel benchmark
    suite.add_benchmark(Benchmark::new(
        "channel_throughput",
        "Channel send/receive throughput",
        || {
            use std::thread;
            use std::sync::mpsc;
            
            const NUM_MESSAGES: usize = 10_000;
            
            // Create a standard Rust channel instead of CURSED channel for thread safety
            let (tx, rx) = mpsc::channel();
            
            let start = Instant::now();
            
            // Spawn a sender thread
            let sender = thread::spawn(move || {
                for i in 0..NUM_MESSAGES {
                    tx.send(ThreadSafeValue::Integer(i as i64)).unwrap();
                }
            });
            
            // Receive messages
            let mut received = 0;
            for _ in 0..NUM_MESSAGES {
                let _ = rx.recv().unwrap();
                received += 1;
            }
            
            // Wait for sender to finish
            sender.join().unwrap();
            
            let elapsed = start.elapsed();
            
            debug!(elapsed_ms = ?elapsed.as_millis(), messages = NUM_MESSAGES, 
                   "Channel throughput test complete");
            
            // Create metrics
            let mut metrics: Vec<Box<dyn Metric>> = Vec::new();
            
            metrics.push(Box::new(TimingMetric {
                name: "channel_time".to_string(),
                duration: elapsed,
            }));
            
            metrics.push(Box::new(ThroughputMetric {
                name: "channel_throughput".to_string(),
                operations: NUM_MESSAGES as u64,
                duration: elapsed,
                operation_unit: "messages".to_string(),
            }));
            
            metrics
        },
    ));
    
    // Add concurrent GC benchmark
    suite.add_benchmark(Benchmark::new(
        "concurrent_gc",
        "Garbage collection with concurrent threads",
        || {
            use std::thread;
            use std::sync::Barrier;
            
            #[derive(Debug, Clone)]
            struct TestObject {
                id: usize,
                value: String,
            }
            
            impl Traceable for TestObject {
                fn trace(&self, _visitor: &mut dyn Visitor) {
                    // No references to trace
                }
                
                fn size(&self) -> usize {
                    std::mem::size_of::<Self>() + self.value.len()
                }
                
                fn tag(&self) -> Tag {
                    Tag::Object
                }
                
                fn finalize(&mut self) {
                    // Nothing to finalize
                }
            }
            
            // Create a shared GC
            let gc = Arc::new(GarbageCollector::new());
            
            const NUM_THREADS: usize = 4;
            const OBJECTS_PER_THREAD: usize = 5_000;
            
            // Set up a barrier to synchronize threads
            let barrier = Arc::new(Barrier::new(NUM_THREADS + 1)); // +1 for main thread
            
            let start = Instant::now();
            
            // Spawn threads to allocate objects
            let mut handles = Vec::with_capacity(NUM_THREADS);
            
            for thread_id in 0..NUM_THREADS {
                let thread_gc = gc.clone();
                let thread_barrier = barrier.clone();
                
                handles.push(thread::spawn(move || {
                    let mut objects = Vec::with_capacity(OBJECTS_PER_THREAD);
                    
                    // Allocate objects
                    for i in 0..OBJECTS_PER_THREAD {
                        let obj = TestObject {
                            id: thread_id * OBJECTS_PER_THREAD + i,
                            value: format!("Thread {} Object #{}", thread_id, i),
                        };
                        objects.push(thread_gc.allocate(obj));
                    }
                    
                    // Wait for all threads to finish allocation
                    thread_barrier.wait();
                    
                    // Hold references for a bit
                    std::thread::sleep(Duration::from_millis(10));
                    
                    // Drop references
                    drop(objects);
                    
                    // Wait for all threads to drop references
                    thread_barrier.wait();
                }));
            }
            
            // Wait for all threads to finish allocation
            barrier.wait();
            
            // Get stats after allocation
            let after_alloc_stats = gc.stats();
            
            // Wait for all threads to drop references
            barrier.wait();
            
            // Force collection
            let collect_start = Instant::now();
            gc.collect_garbage();
            let collect_elapsed = collect_start.elapsed();
            
            // Wait for all threads to finish
            for handle in handles {
                handle.join().unwrap();
            }
            
            let total_elapsed = start.elapsed();
            
            // Get final stats
            let final_stats = gc.stats();
            
            debug!(total_ms = ?total_elapsed.as_millis(), collect_ms = ?collect_elapsed.as_millis(),
                   after_alloc_objects = after_alloc_stats.object_count, final_objects = final_stats.object_count,
                   "Concurrent GC test complete");
            
            // Create metrics
            let mut metrics: Vec<Box<dyn Metric>> = Vec::new();
            
            metrics.push(Box::new(TimingMetric {
                name: "total_time".to_string(),
                duration: total_elapsed,
            }));
            
            metrics.push(Box::new(TimingMetric {
                name: "collect_time".to_string(),
                duration: collect_elapsed,
            }));
            
            metrics.push(Box::new(MemoryMetric {
                name: "memory_usage".to_string(),
                before_object_count: after_alloc_stats.object_count,
                after_object_count: final_stats.object_count,
                before_total_size: after_alloc_stats.total_size,
                after_total_size: final_stats.total_size,
                allocated: after_alloc_stats.total_size,
                collected: after_alloc_stats.total_size - final_stats.total_size,
                collection_time_ms: final_stats.total_gc_time_ms - after_alloc_stats.total_gc_time_ms,
            }));
            
            metrics
        },
    ));
    
    suite
}