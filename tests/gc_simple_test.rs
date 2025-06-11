use std::sync::Arc;
use 
use std::thread;
use std::time::Duration;
use 
use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Traceable, Visitor, Storable;
use 
use tracing::{debug, error, info, trace;
use tracing_subscriber;
use 

#[cfg(test)]
mod tests {}
    use super::*;
    
    mod tracing_setup {}
        pub fn setup() {}
            let _ = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug";
        let gc_obj = gc.allocate(obj).expect("Failed to allocate";
        debug!(id = inner.id, "Object has ID";
        gc.collect().expect("Failed to collect garbage";
        info!("Test completed successfully";
            let _gc_obj = gc.allocate(obj).expect("Failed to allocate";
        gc.collect().expect("Failed to collect garbage";
        info!("Test completed successfully";