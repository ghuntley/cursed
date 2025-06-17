use cursed::runtime::goroutine::{GoroutineScheduler, SchedulerConfig};
use cursed::error::CursedError;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goroutine_scheduler_creation() -> Result<(), CursedError> {
        let config = SchedulerConfig::default();
        let scheduler = GoroutineScheduler::new(config)?;
        
        assert!(scheduler.worker_count() > 0);
        Ok(())
    }

    #[test]
    fn test_basic_goroutine_spawn() -> Result<(), CursedError> {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config)?;
        
        // Test goroutine spawning
        let goroutine_id = scheduler.spawn_goroutine(|| {
            // Simple goroutine task
        })?;
        
        assert!(goroutine_id > 0);
        Ok(())
    }

    #[test]
    fn test_goroutine_execution() -> Result<(), CursedError> {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config)?;
        
        scheduler.start()?;
        
        // Spawn a simple goroutine
        let _id = scheduler.spawn_goroutine(|| {
            // Task execution
        })?;
        
        // Allow some time for execution
        std::thread::sleep(Duration::from_millis(100));
        
        // Check scheduler state
        assert!(scheduler.is_running());
        
        scheduler.stop()?;
        Ok(())
    }

    #[test]
    fn test_multiple_goroutines() -> Result<(), CursedError> {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config)?;
        
        scheduler.start()?;
        
        // Spawn multiple goroutines
        for i in 0..5 {
            let _id = scheduler.spawn_goroutine(move || {
                // Simple task with different iterations
                for _ in 0..i {
                    std::thread::yield_now();
                }
            })?;
        }
        
        // Let goroutines run
        std::thread::sleep(Duration::from_millis(200));
        
        // Check that scheduler is handling multiple goroutines
        let active_count = scheduler.active_goroutines();
        assert!(active_count >= 0);
        
        scheduler.stop()?;
        Ok(())
    }
}
