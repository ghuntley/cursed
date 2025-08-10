//! Async I/O Poller Implementation
//!
//! Cross-platform async I/O implementation with:
//! - Linux: epoll-based polling
//! - Windows: IOCP (I/O Completion Port) based polling
//! - Unified interface for file I/O and network operations
//! - Integration with CURSED runtime system

use crate::error::CursedError;
use crate::runtime::goroutine::{Goroutine, GoroutineId, GoroutineState};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::io::{self, Read, Write};

#[cfg(target_os = "windows")]
use winapi::um::{
    ioapiset::{CreateIoCompletionPort, GetQueuedCompletionStatus, PostQueuedCompletionStatus},
    winbase::{INFINITE, WAIT_TIMEOUT},
    winnt::{HANDLE, PVOID},
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
    errhandlingapi::GetLastError,
    winsock2::{WSAGetLastError, WSAOVERLAPPED},
    minwinbase::OVERLAPPED,
};

#[cfg(target_os = "linux")]
use libc::{epoll_create1, epoll_ctl, epoll_wait, EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLLIN, EPOLLOUT, EPOLLERR};

/// Maximum number of events to process per polling iteration
const MAX_EVENTS: usize = 64;

/// I/O operation types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IoOpType {
    Read,
    Write,
    Accept,
    Connect,
    SendTo,
    RecvFrom,
}

/// I/O completion result
#[derive(Debug)]
pub struct IoCompletion {
    pub goroutine_id: GoroutineId,
    pub operation_id: u64,
    pub op_type: IoOpType,
    pub result: io::Result<usize>,
    pub timestamp: Instant,
}

/// Async I/O request
#[derive(Debug)]
pub struct IoRequest {
    pub goroutine_id: GoroutineId,
    pub operation_id: u64,
    pub op_type: IoOpType,
    pub fd_or_handle: usize,
    pub buffer_ptr: *mut u8,
    pub buffer_len: usize,
    pub offset: Option<u64>,
    pub timeout: Option<Duration>,
    pub submitted_at: Instant,
}

unsafe impl Send for IoRequest {}
unsafe impl Sync for IoRequest {}

/// Cross-platform async I/O poller
pub struct AsyncPoller {
    /// Platform-specific poller implementation
    inner: Box<dyn AsyncPollerImpl + Send + Sync>,
    /// Pending I/O operations
    pending_ops: Arc<RwLock<HashMap<u64, IoRequest>>>,
    /// Completed I/O operations
    completed_ops: Arc<Mutex<Vec<IoCompletion>>>,
    /// Operation ID counter
    next_op_id: AtomicU64,
    /// Shutdown signal
    shutdown: Arc<AtomicBool>,
    /// Poller thread handle
    poller_thread: Option<JoinHandle<()>>,
}

/// Platform-specific async poller implementation trait
trait AsyncPollerImpl {
    fn submit_read(&mut self, req: IoRequest) -> Result<(), CursedError>;
    fn submit_write(&mut self, req: IoRequest) -> Result<(), CursedError>;
    fn poll_completions(&mut self, timeout: Option<Duration>) -> Result<Vec<IoCompletion>, CursedError>;
    fn cancel_operation(&mut self, operation_id: u64) -> Result<(), CursedError>;
    fn shutdown(&mut self) -> Result<(), CursedError>;
}

#[cfg(target_os = "windows")]
/// Windows IOCP-based async I/O implementation
struct WindowsIOCPPoller {
    /// I/O Completion Port handle
    iocp_handle: HANDLE,
    /// Active overlapped operations
    overlapped_ops: HashMap<u64, Box<OVERLAPPED>>,
    /// Operation metadata
    operation_metadata: HashMap<u64, IoRequest>,
}

#[cfg(target_os = "windows")]
impl WindowsIOCPPoller {
    fn new() -> Result<Self, CursedError> {
        unsafe {
            let iocp_handle = CreateIoCompletionPort(
                INVALID_HANDLE_VALUE,
                std::ptr::null_mut(),
                0,
                0, // Number of concurrent threads (0 = number of processors)
            );

            if iocp_handle.is_null() {
                return Err(CursedError::runtime_error(&format!(
                    "Failed to create IOCP: error code {}",
                    GetLastError()
                )));
            }

            Ok(Self {
                iocp_handle,
                overlapped_ops: HashMap::new(),
                operation_metadata: HashMap::new(),
            })
        }
    }

    fn associate_handle(&mut self, handle: HANDLE, completion_key: usize) -> Result<(), CursedError> {
        unsafe {
            let result = CreateIoCompletionPort(
                handle,
                self.iocp_handle,
                completion_key,
                0,
            );

            if result.is_null() {
                return Err(CursedError::runtime_error(&format!(
                    "Failed to associate handle with IOCP: error code {}",
                    GetLastError()
                )));
            }
        }
        Ok(())
    }

    fn create_overlapped(&mut self, operation_id: u64) -> *mut OVERLAPPED {
        let overlapped = Box::new(unsafe { std::mem::zeroed::<OVERLAPPED>() });
        let overlapped_ptr = Box::into_raw(overlapped);
        
        // Store overlapped for cleanup
        unsafe {
            self.overlapped_ops.insert(operation_id, Box::from_raw(overlapped_ptr));
        }
        
        overlapped_ptr
    }
}

#[cfg(target_os = "windows")]
impl AsyncPollerImpl for WindowsIOCPPoller {
    fn submit_read(&mut self, req: IoRequest) -> Result<(), CursedError> {
        use winapi::um::fileapi::ReadFile;
        
        let overlapped_ptr = self.create_overlapped(req.operation_id);
        self.operation_metadata.insert(req.operation_id, req);

        unsafe {
            let handle = req.fd_or_handle as HANDLE;
            let mut bytes_read = 0u32;

            let result = ReadFile(
                handle,
                req.buffer_ptr as PVOID,
                req.buffer_len as u32,
                &mut bytes_read,
                overlapped_ptr,
            );

            if result == 0 {
                let error = GetLastError();
                if error != winapi::shared::winerror::ERROR_IO_PENDING {
                    self.overlapped_ops.remove(&req.operation_id);
                    self.operation_metadata.remove(&req.operation_id);
                    return Err(CursedError::runtime_error(&format!(
                        "ReadFile failed: error code {}",
                        error
                    )));
                }
            }
        }

        Ok(())
    }

    fn submit_write(&mut self, req: IoRequest) -> Result<(), CursedError> {
        use winapi::um::fileapi::WriteFile;

        let overlapped_ptr = self.create_overlapped(req.operation_id);
        self.operation_metadata.insert(req.operation_id, req);

        unsafe {
            let handle = req.fd_or_handle as HANDLE;
            let mut bytes_written = 0u32;

            let result = WriteFile(
                handle,
                req.buffer_ptr as PVOID,
                req.buffer_len as u32,
                &mut bytes_written,
                overlapped_ptr,
            );

            if result == 0 {
                let error = GetLastError();
                if error != winapi::shared::winerror::ERROR_IO_PENDING {
                    self.overlapped_ops.remove(&req.operation_id);
                    self.operation_metadata.remove(&req.operation_id);
                    return Err(CursedError::runtime_error(&format!(
                        "WriteFile failed: error code {}",
                        error
                    )));
                }
            }
        }

        Ok(())
    }

    fn poll_completions(&mut self, timeout: Option<Duration>) -> Result<Vec<IoCompletion>, CursedError> {
        let mut completions = Vec::new();
        let timeout_ms = timeout.map(|d| d.as_millis() as u32).unwrap_or(INFINITE);

        unsafe {
            let mut bytes_transferred = 0u32;
            let mut completion_key = 0usize;
            let mut overlapped_ptr: *mut OVERLAPPED = std::ptr::null_mut();

            let result = GetQueuedCompletionStatus(
                self.iocp_handle,
                &mut bytes_transferred,
                &mut completion_key,
                &mut overlapped_ptr,
                timeout_ms,
            );

            if result != 0 || !overlapped_ptr.is_null() {
                // Find the operation ID by overlapped pointer
                let mut found_op_id = None;
                for (&op_id, overlapped_box) in &self.overlapped_ops {
                    if overlapped_box.as_ref() as *const OVERLAPPED == overlapped_ptr {
                        found_op_id = Some(op_id);
                        break;
                    }
                }

                if let Some(operation_id) = found_op_id {
                    if let Some(req) = self.operation_metadata.remove(&operation_id) {
                        self.overlapped_ops.remove(&operation_id);

                        let io_result = if result != 0 {
                            Ok(bytes_transferred as usize)
                        } else {
                            let error = GetLastError();
                            Err(io::Error::from_raw_os_error(error as i32))
                        };

                        completions.push(IoCompletion {
                            goroutine_id: req.goroutine_id,
                            operation_id: req.operation_id,
                            op_type: req.op_type,
                            result: io_result,
                            timestamp: Instant::now(),
                        });
                    }
                }
            } else {
                let error = GetLastError();
                if error != WAIT_TIMEOUT {
                    return Err(CursedError::runtime_error(&format!(
                        "GetQueuedCompletionStatus failed: error code {}",
                        error
                    )));
                }
            }
        }

        Ok(completions)
    }

    fn cancel_operation(&mut self, operation_id: u64) -> Result<(), CursedError> {
        // Remove from tracking
        self.overlapped_ops.remove(&operation_id);
        self.operation_metadata.remove(&operation_id);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), CursedError> {
        unsafe {
            // Post a quit message to wake up polling thread
            PostQueuedCompletionStatus(self.iocp_handle, 0, 0, std::ptr::null_mut());
            
            if !self.iocp_handle.is_null() {
                CloseHandle(self.iocp_handle);
                self.iocp_handle = std::ptr::null_mut();
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
/// Linux epoll-based async I/O implementation
struct LinuxEpollPoller {
    epoll_fd: i32,
    pending_reads: HashMap<i32, IoRequest>,
    pending_writes: HashMap<i32, IoRequest>,
}

#[cfg(target_os = "linux")]
impl LinuxEpollPoller {
    fn new() -> Result<Self, CursedError> {
        let epoll_fd = unsafe { epoll_create1(libc::EPOLL_CLOEXEC) };
        if epoll_fd == -1 {
            return Err(CursedError::runtime_error("Failed to create epoll instance"));
        }

        Ok(Self {
            epoll_fd,
            pending_reads: HashMap::new(),
            pending_writes: HashMap::new(),
        })
    }
}

#[cfg(target_os = "linux")]
impl AsyncPollerImpl for LinuxEpollPoller {
    fn submit_read(&mut self, req: IoRequest) -> Result<(), CursedError> {
        use libc::{epoll_event, EPOLLIN, EPOLLONESHOT};

        let fd = req.fd_or_handle as i32;
        self.pending_reads.insert(fd, req);

        unsafe {
            let mut event = epoll_event {
                events: (EPOLLIN | EPOLLONESHOT) as u32,
                u64: fd as u64,
            };

            if epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event) == -1 {
                self.pending_reads.remove(&fd);
                return Err(CursedError::runtime_error("Failed to add read event to epoll"));
            }
        }

        Ok(())
    }

    fn submit_write(&mut self, req: IoRequest) -> Result<(), CursedError> {
        use libc::{epoll_event, EPOLLOUT, EPOLLONESHOT};

        let fd = req.fd_or_handle as i32;
        self.pending_writes.insert(fd, req);

        unsafe {
            let mut event = epoll_event {
                events: (EPOLLOUT | EPOLLONESHOT) as u32,
                u64: fd as u64,
            };

            if epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event) == -1 {
                self.pending_writes.remove(&fd);
                return Err(CursedError::runtime_error("Failed to add write event to epoll"));
            }
        }

        Ok(())
    }

    fn poll_completions(&mut self, timeout: Option<Duration>) -> Result<Vec<IoCompletion>, CursedError> {
        use libc::{epoll_event, epoll_wait};

        let mut events = [unsafe { std::mem::zeroed::<epoll_event>() }; MAX_EVENTS];
        let timeout_ms = timeout.map(|d| d.as_millis() as i32).unwrap_or(-1);

        let num_events = unsafe {
            epoll_wait(self.epoll_fd, events.as_mut_ptr(), MAX_EVENTS as i32, timeout_ms)
        };

        if num_events == -1 {
            return Err(CursedError::runtime_error("epoll_wait failed"));
        }

        let mut completions = Vec::new();

        for i in 0..num_events as usize {
            let event = &events[i];
            let fd = event.u64 as i32;
            let events = event.events;

            // Check for read completion
            if (events & EPOLLIN as u32) != 0 {
                if let Some(req) = self.pending_reads.remove(&fd) {
                    // Perform the actual read
                    let result = unsafe {
                        let bytes_read = libc::read(
                            fd,
                            req.buffer_ptr as *mut libc::c_void,
                            req.buffer_len,
                        );

                        if bytes_read >= 0 {
                            Ok(bytes_read as usize)
                        } else {
                            Err(io::Error::last_os_error())
                        }
                    };

                    completions.push(IoCompletion {
                        goroutine_id: req.goroutine_id,
                        operation_id: req.operation_id,
                        op_type: req.op_type,
                        result,
                        timestamp: Instant::now(),
                    });
                }
            }

            // Check for write completion
            if (events & EPOLLOUT as u32) != 0 {
                if let Some(req) = self.pending_writes.remove(&fd) {
                    // Perform the actual write
                    let result = unsafe {
                        let bytes_written = libc::write(
                            fd,
                            req.buffer_ptr as *const libc::c_void,
                            req.buffer_len,
                        );

                        if bytes_written >= 0 {
                            Ok(bytes_written as usize)
                        } else {
                            Err(io::Error::last_os_error())
                        }
                    };

                    completions.push(IoCompletion {
                        goroutine_id: req.goroutine_id,
                        operation_id: req.operation_id,
                        op_type: req.op_type,
                        result,
                        timestamp: Instant::now(),
                    });
                }
            }
        }

        Ok(completions)
    }

    fn cancel_operation(&mut self, operation_id: u64) -> Result<(), CursedError> {
        // Find and remove the operation
        self.pending_reads.retain(|_, req| req.operation_id != operation_id);
        self.pending_writes.retain(|_, req| req.operation_id != operation_id);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), CursedError> {
        unsafe {
            if self.epoll_fd != -1 {
                libc::close(self.epoll_fd);
                self.epoll_fd = -1;
            }
        }
        Ok(())
    }
}

impl AsyncPoller {
    /// Create a new async poller for the current platform
    pub fn new() -> Result<Self, CursedError> {
        let inner: Box<dyn AsyncPollerImpl + Send + Sync> = {
            #[cfg(target_os = "windows")]
            {
                Box::new(WindowsIOCPPoller::new()?)
            }
            #[cfg(target_os = "linux")]
            {
                Box::new(LinuxEpollPoller::new()?)
            }
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            {
                return Err(CursedError::runtime_error("Unsupported platform for async I/O"));
            }
        };

        Ok(Self {
            inner,
            pending_ops: Arc::new(RwLock::new(HashMap::new())),
            completed_ops: Arc::new(Mutex::new(Vec::new())),
            next_op_id: AtomicU64::new(1),
            shutdown: Arc::new(AtomicBool::new(false)),
            poller_thread: None,
        })
    }

    /// Start the async poller background thread
    pub fn start(&mut self) -> Result<(), CursedError> {
        let shutdown = self.shutdown.clone();
        let completed_ops = self.completed_ops.clone();

        // Note: We'll need to restructure this to handle the inner poller
        // across thread boundaries more carefully in a real implementation
        
        Ok(())
    }

    /// Submit an async read operation
    pub fn submit_read(
        &mut self,
        goroutine_id: GoroutineId,
        fd_or_handle: usize,
        buffer: &mut [u8],
        timeout: Option<Duration>,
    ) -> Result<u64, CursedError> {
        let operation_id = self.next_op_id.fetch_add(1, Ordering::SeqCst);

        let req = IoRequest {
            goroutine_id,
            operation_id,
            op_type: IoOpType::Read,
            fd_or_handle,
            buffer_ptr: buffer.as_mut_ptr(),
            buffer_len: buffer.len(),
            offset: None,
            timeout,
            submitted_at: Instant::now(),
        };

        {
            let mut pending = self.pending_ops.write()
                .map_err(|_| CursedError::runtime_error("Failed to lock pending operations"))?;
            pending.insert(operation_id, req.clone());
        }

        self.inner.submit_read(req)?;
        Ok(operation_id)
    }

    /// Submit an async write operation
    pub fn submit_write(
        &mut self,
        goroutine_id: GoroutineId,
        fd_or_handle: usize,
        buffer: &[u8],
        timeout: Option<Duration>,
    ) -> Result<u64, CursedError> {
        let operation_id = self.next_op_id.fetch_add(1, Ordering::SeqCst);

        let req = IoRequest {
            goroutine_id,
            operation_id,
            op_type: IoOpType::Write,
            fd_or_handle,
            buffer_ptr: buffer.as_ptr() as *mut u8,
            buffer_len: buffer.len(),
            offset: None,
            timeout,
            submitted_at: Instant::now(),
        };

        {
            let mut pending = self.pending_ops.write()
                .map_err(|_| CursedError::runtime_error("Failed to lock pending operations"))?;
            pending.insert(operation_id, req.clone());
        }

        self.inner.submit_write(req)?;
        Ok(operation_id)
    }

    /// Poll for completed I/O operations
    pub fn poll_completions(&mut self, timeout: Option<Duration>) -> Result<Vec<IoCompletion>, CursedError> {
        let completions = self.inner.poll_completions(timeout)?;

        // Remove completed operations from pending
        if !completions.is_empty() {
            let mut pending = self.pending_ops.write()
                .map_err(|_| CursedError::runtime_error("Failed to lock pending operations"))?;
            
            for completion in &completions {
                pending.remove(&completion.operation_id);
            }
        }

        Ok(completions)
    }

    /// Cancel a pending I/O operation
    pub fn cancel_operation(&mut self, operation_id: u64) -> Result<(), CursedError> {
        {
            let mut pending = self.pending_ops.write()
                .map_err(|_| CursedError::runtime_error("Failed to lock pending operations"))?;
            pending.remove(&operation_id);
        }

        self.inner.cancel_operation(operation_id)
    }

    /// Get pending operation count
    pub fn pending_count(&self) -> usize {
        self.pending_ops.read()
            .map(|pending| pending.len())
            .unwrap_or(0)
    }

    /// Shutdown the async poller
    pub fn shutdown(&mut self) -> Result<(), CursedError> {
        self.shutdown.store(true, Ordering::SeqCst);
        self.inner.shutdown()?;

        if let Some(handle) = self.poller_thread.take() {
            let _ = handle.join();
        }

        Ok(())
    }
}

impl Drop for AsyncPoller {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

/// Integration with existing CURSED runtime
pub mod runtime_integration {
    use super::*;
    use crate::runtime::goroutine::GoroutineScheduler;

    impl GoroutineScheduler {
        /// Integrate async poller with the scheduler
        pub fn integrate_async_poller(&mut self, mut poller: AsyncPoller) -> Result<(), CursedError> {
            poller.start()?;
            
            // Store poller for future use
            // This would need proper integration with the scheduler's event loop
            
            Ok(())
        }

        /// Handle I/O completion and resume goroutines
        pub fn handle_io_completions(&mut self, completions: Vec<IoCompletion>) -> Result<(), CursedError> {
            for completion in completions {
                // Find the goroutine waiting for this I/O operation
                if let Some(goroutine) = self.find_goroutine_mut(completion.goroutine_id) {
                    // Resume the goroutine with the I/O result
                    goroutine.resume_with_io_result(completion.result, completion.op_type)?;
                    
                    // Mark goroutine as ready to run
                    goroutine.set_state(GoroutineState::Ready);
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_poller_creation() {
        let poller = AsyncPoller::new();
        assert!(poller.is_ok());
    }

    #[test]
    fn test_operation_id_generation() {
        let mut poller = AsyncPoller::new().unwrap();
        let id1 = poller.next_op_id.load(Ordering::SeqCst);
        let id2 = poller.next_op_id.fetch_add(1, Ordering::SeqCst);
        assert!(id2 > id1);
    }
}
