/// Runtime system for CURSED
pub mod goroutine;
pub mod channels;
pub mod unicode_char;

pub use goroutine::{
    GoroutineScheduler, Goroutine, GoroutineState, GoroutineStack,
    SchedulerConfig, GcCoordinator, SafePoint,
    cursed_spawn_goroutine, cursed_yield_goroutine, cursed_safe_point, cursed_gc_requested
};
pub use channels::*;
pub use unicode_char::*;
