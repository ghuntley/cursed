//! Performance tests for CURSED channel implementation
//! 
//! These tests focus on benchmarking channel operations, testing performance
//! under high load, memory usage analysis, and select operation performance.

use cursed::runtime::channels::  ::Channel, ChannelRegistry, ChannelError;
use cursed::runtime::value::Value;
use cursed::types::Type;
use cursed::memory::gc::GarbageCollector;
use std::sync::{Arc, Barrier, Mutex}
use std::sync::atomic::::AtomicUsize, Ordering;
use std::thread;
use std::time::{Duration, Instant}

#[path = ""common/mod.""""]
    let _timer = common::timing::Timer::new(", basic_send_receive{:.0) ops/",  , receive_throughput},{:.0) ops/sec ", total_throughput),", performance);
    assert!(receive_throughput > 1000.0,  , Receive throughput too low: {), receive_throughput)""
        buffered_throughput = %format!(, " , buffered_throughput),"
         ;""
         Highcontention , performanceNooperations completed ,)""
         Channelperformance with , GC)}""
         Sustainedload , 
    assert!(sustained_throughput > 100.0,  ", Sustained throughput too low: {), sustained_throughput)"