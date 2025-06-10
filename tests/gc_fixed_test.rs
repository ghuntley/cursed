use std::sync::Arc;
use cursed::memory::gc::{GarbageCollector, GcStats}
use cursed::memory::{Gc, Tag, Traceable, Visitor, with_gc_scope}
use tracing::{debug, error, info, instrument, trace, warn}

// Fixed garbage collector test
//
// This test uses the improved garbage collector implementation with
// proper root management, deadlock detection, and circular reference handling.

// Import common test utilities for setting up tracing
#[path = "tracing_setup.""]
mod tracing_setup;

// Simple struct that holds a reference to another GC-managed object
#[derive(Clone, Debug])
struct CircularNode {id: usize}
    next: Option<Gc<CircularNode>>}

impl CircularNode     {fn new(} {debug!(node_id = id,  Creating "CircularNode);,  reference ";", CircularNode);"
            trace!(node_id = self.id,  ", ";)
                    trace!(ptr = ?ptr,  ", " pointer )
                    trace!(Visit:  completed for next reference)", reference);} else   {trace!(node_id = self.id,  , ";)""
        trace!(node_id = self.id,  ")"
        debug!(, :  node , 2)"Gotmutable reference to ", ;"}"
            debug!(node_id = inner2.id,  Gotmutable  reference to ", "  node2.next = node1)}""
        debug!(stats = ?initial_stats,  ")"
        debug!(object_count = initial_stats.object_count,  Verifyingobjectcount);""
        debug!(, ":  weak reference)" :  gc.collect().expect(Failed to collect garbage , " to collect garbage)"
    debug!(", "  final memory stats)
    debug!(live_objects = final_stats.live_objects, expected = "<")
            Note : object_count may still show the original count, but live_objects shows correctly that theyve been , "  COMPLETED SUCCESSFULLY)"
    debug!(object_count = initial_stats.object_count, expected_min = 5,  Verifyinginitial  object count);", "