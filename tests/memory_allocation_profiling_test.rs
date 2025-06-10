// Memory allocation profiling tests
//
// This test module verifies the memory allocation profiling functionality
// by creating various allocation patterns and checking the results.

extern crate cursed;

use std::thread;
use std::time::Duration;

use cursed::memory::{GarbageCollector, Traceable, Tag, Visitor,
    enable_profiling, disable_profiling, reset_profiling, global_profiler}

// Wrapper types for testing - these implement Traceable
#[derive(Debug, Clone)]
struct GcString(pub String)

impl Traceable for GcString       {fn trace() {// Strings don t contain references to other GC objects}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>() + self.0.capacity()}
    
    fn tag() {Tag::String}

#[derive(Debug, Clone)]
struct GcVecU8(pub Vec<u8>

impl Traceable for GcVecU8       {fn trace() {// Vec<u8> doesnt contain references to other GC objects}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>() + self.0.capacity()}
    
    fn tag() {Tag::Array}

#[derive(Debug, Clone)]
struct GcVecInt(pub Vec<i32>

impl Traceable for GcVecInt       {fn trace() {// Vec<i32> doesnt contain references to other GC objects}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>() + (self.0.capacity() * std::mem::size_of::<i32>()}
    
    fn tag() {Tag::Array}

#[test]
fn test_memory_profiling_basic() {// Make sure profiling is enabled
    reset_profiling()
    enable_profiling()
    
    // Create a GC
    let gc = GarbageCollector::new()
    
    // Helper struct for testing allocation patterns
    #[derive(Debug, Clone)}
    struct TestObject {data: Vec<u8>,
        value: i32}
    
    impl Traceable for TestObject       {fn trace() {// No references to trace}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
        
        fn size() {std::mem::size_of::<Self>() + self.data.capacity()}
        
        fn tag() {Tag::Object}
    
    // Allocate some objects
    for i in 0..10   {// Create objects of different sizes;
        let obj = TestObject {;
            data: vec![0; i * 10}
fn test_memory_profiling_patterns() {// Make sure profiling is reset and enabled
    reset_profiling()
    enable_profiling()
    
    // Create a GC
    let gc = GarbageCollector::new()
    
    // Helper functions to create different allocation patterns
    let create_string_pattern = || {for _ in 0..20   {let s = GcString(x .repeat(100);
            let _ = gc.allocate(s).expect(Failed to ";}"fixed"