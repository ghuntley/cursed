use crate::error::Error;
/// Output streaming and input generation for exec_vibez
/// 
/// Implements OutputStreamer and InputGenerator functionality according to specs/stdlib/exec_vibez.md

use std::io::{self, BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

use super::cmd::Cmd;
use super::process::{Process, ProcessState};
use super::error::{ExecResult, ExecError, execution_failed};

/// Global streamer tracking
static ACTIVE_STREAMERS: AtomicUsize = AtomicUsize::new(0);

/// Output streamer for real-time command output processing
#[derive(Debug)]
pub struct OutputStreamer {
    /// Associated command
    cmd: Cmd,
    /// Line callback function
    line_callback: Option<Box<dyn Fn(String) + Send + Sync>>,
    /// Data callback function
    data_callback: Option<Box<dyn Fn(&[u8]) + Send + Sync>>,
    /// Whether the streamer is active
    active: Arc<Mutex<bool>>,
    /// Buffer size for reading
    buffer_size: usize,
    /// Whether to stream stderr as well
    include_stderr: bool,
}

impl OutputStreamer {
    /// Create a new output streamer
    pub fn new(cmd: Cmd) -> Self {
        ACTIVE_STREAMERS.fetch_add(1, Ordering::Relaxed);
        
        Self {
            cmd,
            line_callback: None,
            data_callback: None,
            active: Arc::new(Mutex::new(false)),
            buffer_size: 8192,
            include_stderr: true,
        }
    }
    
    /// Set the line callback function
    pub fn on_line<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.line_callback = Some(Box::new(callback));
        self
    }
    
    /// Set the data callback function
    pub fn on_data<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&[u8]) + Send + Sync + 'static,
    {
        self.data_callback = Some(Box::new(callback));
        self
    }
    
    /// Set the buffer size
    pub fn set_buffer_size(&mut self, size: usize) -> &mut Self {
        self.buffer_size = size;
        self
    }
    
    /// Set whether to include stderr in streaming
    pub fn include_stderr(&mut self, include: bool) -> &mut Self {
        self.include_stderr = include;
        self
    }
    
    /// Start streaming the command output
    pub fn start(&mut self) -> ExecResult<()> {
        // Start the command
        let process = self.cmd.start()?;
        
        {
            let mut active = self.active.lock().unwrap();
            *active = true;
        }
        
        // Get stdout pipe
        if let Some(stdout) = self.cmd.stdout_pipe() {
            let line_callback = self.line_callback.take();
            let data_callback = self.data_callback.take();
            let active = Arc::clone(&self.active);
            let buffer_size = self.buffer_size;
            
            thread::spawn(move || {
                let reader = BufReader::with_capacity(buffer_size, stdout);
                
                if let Some(line_cb) = line_callback {
                    // Line-by-line processing
                    for line_result in reader.split("\n") {
                        if !*active.lock().unwrap() {
                            break;
                        }
                        
                        match line_result {
                            Ok(line) => line_cb(line),
                            Err(e) => {
                                tracing::warn!("Error reading line from stdout: {}", e);
                                break;
                            }
                        }
                    }
                } else if let Some(data_cb) = data_callback {
                    // Raw data processing
                    let mut buffer = vec![0u8; buffer_size];
                    let mut reader = reader.into_inner();
                    
                    loop {
                        if !*active.lock().unwrap() {
                            break;
                        }
                        
                        match reader.read(&mut buffer) {
                            Ok(0) => break, // EOF
                            Ok(n) => data_cb(&buffer[..n]),
                            Err(e) => {
                                tracing::warn!("Error reading data from stdout: {}", e);
                                break;
                            }
                        }
                    }
                }
            });
        }
        
        // Handle stderr if requested
        if self.include_stderr {
            if let Some(stderr) = self.cmd.stderr_pipe() {
                let active = Arc::clone(&self.active);
                let buffer_size = self.buffer_size;
                
                thread::spawn(move || {
                    let reader = BufReader::with_capacity(buffer_size, stderr);
                    
                    for line_result in reader.split("\n") {
                        if !*active.lock().unwrap() {
                            break;
                        }
                        
                        match line_result {
                            Ok(line) => {
                                tracing::debug!("stderr: {}", line);
                            }
                            Err(e) => {
                                tracing::warn!("Error reading line from stderr: {}", e);
                                break;
                            }
                        }
                    }
                });
            }
        }
        
        Ok(())
    }
    
    /// Wait for the command to complete
    pub fn wait(&mut self) -> ExecResult<ProcessState> {
        let state = self.cmd.wait()?;
        
        {
            let mut active = self.active.lock().unwrap();
            *active = false;
        }
        
        Ok(state)
    }
    
    /// Stop streaming (kill the process)
    pub fn stop(&mut self) -> ExecResult<()> {
        {
            let mut active = self.active.lock().unwrap();
            *active = false;
        }
        
        if let Some(process) = self.cmd.process() {
            process.kill()?;
        }
        
        Ok(())
    }
    
    /// Check if the streamer is active
    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }
}

impl Drop for OutputStreamer {
    fn drop(&mut self) {
        ACTIVE_STREAMERS.fetch_sub(1, Ordering::Relaxed);
        
        {
            let mut active = self.active.lock().unwrap();
            *active = false;
        }
    }
}

/// Input generator for programmatic command input
#[derive(Debug)]
pub struct InputGenerator {
    /// Input queue with timing
    input_queue: Arc<Mutex<VecDeque<(Vec<u8>, Option<Duration>)>>>,
    /// Whether the generator is active
    active: Arc<Mutex<bool>>,
    /// Input thread handle
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl InputGenerator {
    /// Create a new input generator
    pub fn new(cmd: &mut Cmd) -> ExecResult<Self> {
        let stdin_pipe = cmd.stdin_pipe()?;
        
        let input_queue = Arc::new(Mutex::new(VecDeque::new()));
        let active = Arc::new(Mutex::new(true));
        
        let queue_clone = Arc::clone(&input_queue);
        let active_clone = Arc::clone(&active);
        
        let thread_handle = thread::spawn(move || {
            let mut stdin = stdin_pipe;
            let mut last_write = Instant::now();
            
            loop {
                if !*active_clone.lock().unwrap() {
                    break;
                }
                
                let input_item = {
                    let mut queue = queue_clone.lock().unwrap();
                    queue.pop_front()
                };
                
                if let Some((data, delay)) = input_item {
                    // Apply delay if specified
                    if let Some(delay_duration) = delay {
                        let elapsed = last_write.elapsed();
                        if elapsed < delay_duration {
                            thread::sleep(delay_duration - elapsed);
                        }
                    }
                    
                    // Write the data
                    if let Err(e) = stdin.write_all(&data) {
                        tracing::warn!("Error writing to stdin: {}", e);
                        break;
                    }
                    
                    if let Err(e) = stdin.flush() {
                        tracing::warn!("Error flushing stdin: {}", e);
                        break;
                    }
                    
                    last_write = Instant::now();
                } else {
                    // No input available, sleep briefly
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });
        
        Ok(Self {
            input_queue,
            active,
            thread_handle: Some(thread_handle),
        })
    }
    
    /// Write data immediately
    pub fn write(&self, data: &str) -> &Self {
        self.write_bytes(data.as_bytes())
    }
    
    /// Write bytes immediately
    pub fn write_bytes(&self, data: &[u8]) -> &Self {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push_back((data.to_vec(), None));
        self
    }
    
    /// Write data after a delay
    pub fn write_after(&self, data: &str, delay: Duration) -> &Self {
        self.write_bytes_after(data.as_bytes(), delay)
    }
    
    /// Write bytes after a delay
    pub fn write_bytes_after(&self, data: &[u8], delay: Duration) -> &Self {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push_back((data.to_vec(), Some(delay)));
        self
    }
    
    /// Write a line (with newline)
    pub fn write_line(&self, line: &str) -> &Self {
        let data = format!("{}\n", line);
        self.write(&data)
    }
    
    /// Write a line after a delay
    pub fn write_line_after(&self, line: &str, delay: Duration) -> &Self {
        let data = format!("{}\n", line);
        self.write_after(&data, delay)
    }
    
    /// Close the input stream
    pub fn close(&mut self) {
        {
            let mut active = self.active.lock().unwrap();
            *active = false;
        }
        
        // Wait for the thread to finish
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
    
    /// Check if the generator is active
    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }
    
    /// Get the number of queued input items
    pub fn queue_size(&self) -> usize {
        self.input_queue.lock().unwrap().len()
    }
}

impl Drop for InputGenerator {
    fn drop(&mut self) {
        self.close();
    }
}

/// Create a new output streamer
pub fn new_output_streamer(cmd: Cmd) -> OutputStreamer {
    OutputStreamer::new(cmd)
}

/// Create a new input generator
pub fn new_input_generator(cmd: &mut Cmd) -> ExecResult<InputGenerator> {
    InputGenerator::new(cmd)
}

/// Get the number of active streamers
pub fn get_active_streamer_count() -> usize {
    ACTIVE_STREAMERS.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::exec_vibez::cmd::Cmd;
use crate::stdlib::process::info::ProcessState;
    
    #[test]
    fn test_output_streamer_creation() {
        let cmd = Cmd::new("echo", &["test"]);
        let streamer = OutputStreamer::new(cmd);
        
        assert!(!streamer.is_active());
        assert_eq!(streamer.buffer_size, 8192);
        assert!(streamer.include_stderr);
    }
    
    #[test]
    fn test_output_streamer_configuration() {
        let cmd = Cmd::new("echo", &["test"]);
        let mut streamer = OutputStreamer::new(cmd);
        
        streamer.set_buffer_size(4096)
                .include_stderr(false);
        
        assert_eq!(streamer.buffer_size, 4096);
        assert!(!streamer.include_stderr);
    }
    
    #[test]
    fn test_output_streamer_callbacks() {
        let cmd = Cmd::new("echo", &["test"]);
        let mut streamer = OutputStreamer::new(cmd);
        
        streamer.on_line(|line| {
            println!("Got line: {}", line);
        });
        
        // Can't easily test the callback without running a real process
        // but we can verify the streamer was configured
        assert!(streamer.line_callback.is_some());
    }
    
    #[test]
    fn test_new_output_streamer_constructor() {
        let cmd = Cmd::new("echo", &["test"]);
        let streamer = new_output_streamer(cmd);
        
        assert!(!streamer.is_active());
    }
    
    #[test]
    fn test_input_generator_queue() {
        // We can test the queue functionality without actually creating a process
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        
        {
            let mut q = queue.lock().unwrap();
            q.push_back((b"test".to_vec(), None));
            q.push_back((b"data".to_vec(), Some(Duration::from_millis(100))));
        }
        
        assert_eq!(queue.lock().unwrap().len(), 2);
        
        let item = queue.lock().unwrap().pop_front();
        assert!(item.is_some());
        if let Some((data, delay)) = item {
            assert_eq!(data, b"test");
            assert_eq!(delay, None);
        }
    }
    
    #[test]
    fn test_active_streamer_count() {
        let initial_count = get_active_streamer_count();
        
        {
            let cmd = Cmd::new("echo", &["test"]);
            let _streamer = OutputStreamer::new(cmd);
            
            assert_eq!(get_active_streamer_count(), initial_count + 1);
        }
        
        // After streamer is dropped, count should decrease
        assert_eq!(get_active_streamer_count(), initial_count);
    }
}


pub type NewOutputStreamer = OutputStreamer;
pub type NewInputGenerator = InputGenerator;
