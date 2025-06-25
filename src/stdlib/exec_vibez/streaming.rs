use crate::error::CursedError;
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
    /// Line callback function
    /// Data callback function
    /// Whether the streamer is active
    /// Buffer size for reading
    /// Whether to stream stderr as well
impl OutputStreamer {
    /// Create a new output streamer
    pub fn new(cmd: Cmd) -> Self {
        ACTIVE_STREAMERS.fetch_add(1, Ordering::Relaxed);
        
        Self {
        }
    }
    
    /// Set the line callback function
    pub fn on_line<F>(&mut self, callback: F) -> &mut Self
    where
    {
        self.line_callback = Some(Box::new(callback));
        self
    /// Set the data callback function
    pub fn on_data<F>(&mut self, callback: F) -> &mut Self
    where
    {
        self.data_callback = Some(Box::new(callback));
        self
    /// Set the buffer size
    pub fn set_buffer_size(&mut self, size: usize) -> &mut Self {
        self.buffer_size = size;
        self
    /// Set whether to include stderr in streaming
    pub fn include_stderr(&mut self, include: bool) -> &mut Self {
        self.include_stderr = include;
        self
    /// Start streaming the command output
    pub fn start(&mut self) -> ExecResult<()> {
        // Start the command
        let process = self.cmd.start()?;
        
        {
            let mut active = self.active.lock().unwrap();
            *active = true;
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
                        match line_result {
                            Err(e) => {
                                tracing::warn!("CursedError reading line from stdout: {}", e);
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
                        match reader.read(&mut buffer) {
                            Ok(0) => break, // EOF
                            Err(e) => {
                                tracing::warn!("CursedError reading data from stdout: {}", e);
                                break;
                            }
                        }
                    }
                }
            });
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
                        match line_result {
                            Ok(line) => {
                                tracing::debug!("stderr: {}", line);
                            }
                            Err(e) => {
                                tracing::warn!("CursedError reading line from stderr: {}", e);
                                break;
                            }
                        }
                    }
                });
            }
        }
        
        Ok(())
    /// Wait for the command to complete
    pub fn wait(&mut self) -> ExecResult<ProcessState> {
        let state = self.cmd.wait()?;
        
        {
            let mut active = self.active.lock().unwrap();
            *active = false;
        Ok(state)
    /// Stop streaming (kill the process)
    pub fn stop(&mut self) -> ExecResult<()> {
        {
            let mut active = self.active.lock().unwrap();
            *active = false;
        if let Some(process) = self.cmd.process() {
            process.kill()?;
        Ok(())
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
/// Input generator for programmatic command input
#[derive(Debug)]
pub struct InputGenerator {
    /// Input queue with timing
    /// Whether the generator is active
    /// Input thread handle
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
                let input_item = {
                    let mut queue = queue_clone.lock().unwrap();
                    queue.pop_front()
                
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
                        tracing::warn!("CursedError writing to stdin: {}", e);
                        break;
                    if let Err(e) = stdin.flush() {
                        tracing::warn!("CursedError flushing stdin: {}", e);
                        break;
                    last_write = Instant::now();
                } else {
                    // No input available, sleep briefly
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });
        
        Ok(Self {
        })
    /// Write data immediately
    pub fn write(&self, data: &str) -> &Self {
        self.write_bytes(data.as_bytes())
    /// Write bytes immediately
    pub fn write_bytes(&self, data: &[u8]) -> &Self {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push_back((data.to_vec(), None));
        self
    /// Write data after a delay
    pub fn write_after(&self, data: &str, delay: Duration) -> &Self {
        self.write_bytes_after(data.as_bytes(), delay)
    /// Write bytes after a delay
    pub fn write_bytes_after(&self, data: &[u8], delay: Duration) -> &Self {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push_back((data.to_vec(), Some(delay)));
        self
    /// Write a line (with newline)
    pub fn write_line(&self, line: &str) -> &Self {
        let data = format!("{}\n", line);
        self.write(&data)
    /// Write a line after a delay
    pub fn write_line_after(&self, line: &str, delay: Duration) -> &Self {
        let data = format!("{}\n", line);
        self.write_after(&data, delay)
    /// Close the input stream
    pub fn close(&mut self) {
        {
            let mut active = self.active.lock().unwrap();
            *active = false;
        // Wait for the thread to finish
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
    
    /// Check if the generator is active
    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
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
/// Create a new input generator
pub fn new_input_generator(cmd: &mut Cmd) -> ExecResult<InputGenerator> {
    InputGenerator::new(cmd)
/// Get the number of active streamers
pub fn get_active_streamer_count() -> usize {
    ACTIVE_STREAMERS.load(Ordering::Relaxed)
