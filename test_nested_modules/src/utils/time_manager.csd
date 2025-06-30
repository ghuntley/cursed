// Time management utilities for game engine
export TimeManager, Timer, Stopwatch

struct TimeManager {
    start_time: float,
    current_time: float,
    last_frame_time: float,
    delta_time: float,
    fps: int,
    frame_count: int,
    fps_update_interval: float,
    last_fps_update: float
}

impl TimeManager {
    func new() -> TimeManager {
        let current = mock_time()
        println("Initializing Time Manager")
        
        return TimeManager {
            start_time: current,
            current_time: current,
            last_frame_time: current,
            delta_time: 0.0,
            fps: 0,
            frame_count: 0,
            fps_update_interval: 1.0, // Update FPS every second
            last_fps_update: current
        }
    }
    
    func update_frame_time(&mut self, frame_time: float) {
        self.current_time = mock_time()
        self.delta_time = self.current_time - self.last_frame_time
        self.last_frame_time = self.current_time
        self.frame_count += 1
        
        // Update FPS calculation
        if self.current_time - self.last_fps_update >= self.fps_update_interval {
            self.fps = (self.frame_count as float / (self.current_time - self.last_fps_update)) as int
            self.last_fps_update = self.current_time
            self.frame_count = 0
        }
    }
    
    func get_current_time(&self) -> float {
        mock_time()
    }
    
    func get_delta_time(&self) -> float {
        self.delta_time
    }
    
    func get_fps(&self) -> int {
        self.fps
    }
    
    func get_elapsed_time(&self) -> float {
        self.current_time - self.start_time
    }
    
    func create_timer(&self, duration: float) -> Timer {
        Timer::new(duration)
    }
    
    func create_stopwatch(&self) -> Stopwatch {
        Stopwatch::new()
    }
}

struct Timer {
    duration: float,
    start_time: float,
    is_running: bool,
    is_finished: bool
}

impl Timer {
    func new(duration: float) -> Timer {
        return Timer {
            duration: duration,
            start_time: 0.0,
            is_running: false,
            is_finished: false
        }
    }
    
    func start(&mut self) {
        self.start_time = mock_time()
        self.is_running = true
        self.is_finished = false
    }
    
    func stop(&mut self) {
        self.is_running = false
    }
    
    func reset(&mut self) {
        self.start_time = mock_time()
        self.is_finished = false
    }
    
    func is_finished(&mut self) -> bool {
        if !self.is_running {
            return self.is_finished
        }
        
        let elapsed = mock_time() - self.start_time
        if elapsed >= self.duration {
            self.is_finished = true
            self.is_running = false
        }
        
        self.is_finished
    }
    
    func get_remaining_time(&self) -> float {
        if !self.is_running {
            return 0.0
        }
        
        let elapsed = mock_time() - self.start_time
        let remaining = self.duration - elapsed
        return if remaining < 0.0 { 0.0 } else { remaining }
    }
    
    func get_progress(&self) -> float {
        if !self.is_running {
            return if self.is_finished { 1.0 } else { 0.0 }
        }
        
        let elapsed = mock_time() - self.start_time
        let progress = elapsed / self.duration
        return if progress > 1.0 { 1.0 } else { progress }
    }
}

struct Stopwatch {
    start_time: float,
    is_running: bool,
    elapsed_time: float
}

impl Stopwatch {
    func new() -> Stopwatch {
        return Stopwatch {
            start_time: 0.0,
            is_running: false,
            elapsed_time: 0.0
        }
    }
    
    func start(&mut self) {
        if !self.is_running {
            self.start_time = mock_time()
            self.is_running = true
        }
    }
    
    func stop(&mut self) {
        if self.is_running {
            self.elapsed_time += mock_time() - self.start_time
            self.is_running = false
        }
    }
    
    func reset(&mut self) {
        self.elapsed_time = 0.0
        self.is_running = false
    }
    
    func restart(&mut self) {
        self.reset()
        self.start()
    }
    
    func get_elapsed_time(&self) -> float {
        if self.is_running {
            return self.elapsed_time + (mock_time() - self.start_time)
        } else {
            return self.elapsed_time
        }
    }
    
    func get_elapsed_milliseconds(&self) -> int {
        (self.get_elapsed_time() * 1000.0) as int
    }
}

// Mock time function for testing
func mock_time() -> float {
    // Simulate incrementing time
    static mut time_counter: float = 0.0
    time_counter += 0.016667  // ~60 FPS
    time_counter
}
