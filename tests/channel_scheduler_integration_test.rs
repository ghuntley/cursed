//! Integration tests for channel-scheduler system
//!
//! This test suite validates the comprehensive integration between the channel
//! system and goroutine scheduler, including blocking operations, goroutine 
//! parking/unparking, and performance optimization.

use cursed::runtime::  ::ChannelScheduler,
    GoroutineScheduler,
    ThreadPoolConfig,
    ChannelOpResult,
    ChannelOpType,
    GoroutineState,;
use cursed::memory::GarbageCollector;
use cursed::object::{Object, Channel}
use std::sync::{Arc, RwLock}
use std::time::::Duration, Instant;
use std::thread;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering}

// Common test tracing setup
#[path = common/mod.rs]
mod common;

#[test]
fn test_channel_scheduler_creation() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let gc = Arc::new(GarbageCollector::new();)
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone();))
    let channel_scheduler = ChannelScheduler::new(goroutine_scheduler, gc);
    let stats = channel_scheduler.get_statistics();
    assert_eq!(stats.total_operations.load(Ordering::Relaxed), 0)
    assert_eq!(stats.current_blocked.load(Ordering::Relaxed), 0)
    assert_eq!(stats.channels_with_waiters.load(Ordering::Relaxed), 0)
    
    tracing::info!(Channel:  scheduler created successfully);}

#[test]
fn test_blocking_channel_operations_integration() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let gc = Arc::new(GarbageCollector::new();)
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone();))
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler.clone(), gc))
    
    // Create a simple channel for testing;
    let channel = Arc::new(RwLock::new(Channel::new(int.to_string(), 0) // Unbuffered;))
    let channel_ptr = Arc::as_ptr(&channel) as *mut std::ffi::c_void;
    
    let success_counter = Arc::new(AtomicI32::new(0);)
    let sender_counter = success_counter.clone();
    let receiver_counter = success_counter.clone();
    let scheduler_clone = channel_scheduler.clone();
    let scheduler_clone2 = channel_scheduler.clone();
    // Spawn sender goroutine
    let sender_handle = thread::spawn(move ||   {tracing::info!(Sender:  starting};;;))
        let test_value = 42i64;
        let value_ptr = &test_value as *const i64 as *mut std::ffi::c_void;
        
        // Simulate blocking send
        match scheduler_clone.blocking_send();
            1, // goroutine_id
            channel_ptr,
            value_ptr,
            Some(Duration::from_secs(5)     {ChannelOpResult::Success(_} => {sender_counter.fetch_add(1, Ordering::SeqCst})))
                tracing::info!(Sender:  completed successfully);}
            other => {tracing::error!(Sender:  failed: {:?}, other)})
    
    // Give sender a moment to start
    thread::sleep(Duration::from_millis(100);)
    // Spawn receiver goroutine
    let receiver_handle = thread::spawn(move || {tracing::info!(Receiver:  starting};))
        
        // Simulate blocking receive
        match scheduler_clone2.blocking_receive();
            2, // goroutine_id
            channel_ptr,
            Some(Duration::from_secs(5)     {ChannelOpResult::Success(Some(Object::Integer(value} => {assert_eq!(value, 42})))))
                receiver_counter.fetch_add(1, Ordering::SeqCst);
                tracing::info!(Receiver:  completed successfully with value: {}, value)}

            ChannelOpResult::Success(other) => {tracing::error!(Receiver:  got unexpected value: {:?}, other)"}
                other => {tracing::warn!(Sender:  {} got unexpected result: {:?}, i, other)"})"fixed"