/// Future trait and implementations for async/await support
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::future::Future as StdFuture;
use std::sync::{Arc, Mutex};
use crate::error::CursedError;

/// Core Future trait for CURSED async operations
pub trait Future {
    type Output;

    /// Poll the future to check if it's ready
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

/// Type alias for boxed futures
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Type alias for non-Send futures
pub type LocalFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// Future state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum FutureState {
    /// Future is still pending
    Pending,
    /// Future completed successfully
    Ready,
    /// Future completed with an error
    CursedError,
    /// Future was cancelled
    Cancelled,
}

/// Result type for futures
pub type FutureResult<T> = std::result::Result<T, FutureError>;

/// CursedError types specific to futures
#[derive(Debug, Clone)]
pub enum FutureError {
    /// Future was cancelled before completion
    Cancelled,
    /// Future timed out
    Timeout,
    /// Future failed with an error
    Failed(String),
    /// Future is in an invalid state
    InvalidState,
    /// Runtime error occurred
    Runtime(CursedError),
}

// impl std::fmt::Display for FutureError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             FutureError::Cancelled => write!(f, "Future was cancelled"),
//             FutureError::Timeout => write!(f, "Future timed out"),
//             FutureError::Failed(msg) => write!(f, "Future failed: {}", msg),
//             FutureError::InvalidState => write!(f, "Future is in invalid state"),
//             FutureError::Runtime(err) => write!(f, "Runtime error: {}", err),
//         }
//     }
// }

// impl std::error::CursedError for FutureError {}
// 
/// Convert from CURSED CursedError to FutureError
// impl From<CursedError> for FutureError {
//     fn from(err: CursedError) -> Self {
//         FutureError::Runtime(err)
//     }
// }

/// Basic completed future implementation
pub struct ReadyFuture<T> {
    value: Option<T>,
}

impl<T> ReadyFuture<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T> Future for ReadyFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(value) = self.value.take() {
            Poll::Ready(value)
        } else {
            Poll::Pending
        }
    }
}

// Implement standard Future trait for ReadyFuture to support .await syntax
impl<T> StdFuture for ReadyFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Delegate to the custom Future implementation
        Future::poll(self, cx)
    }
}

/// Future that yields control once before completing
pub struct YieldNowFuture {
    yielded: bool,
}

impl YieldNowFuture {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for YieldNowFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// Implement standard Future trait for YieldNowFuture to support .await syntax
impl StdFuture for YieldNowFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Delegate to the custom Future implementation
        Future::poll(self, cx)
    }
}

/// Create a future that yields once
pub fn yield_now() -> YieldNowFuture {
    YieldNowFuture::new()
}

/// Create a ready future
pub fn ready<T>(value: T) -> ReadyFuture<T> {
    ReadyFuture::new(value)
}

/// Create a pending future that never completes
pub fn pending<T>() -> PendingFuture<T> {
    PendingFuture::new()
}

/// Future that never completes
pub struct PendingFuture<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PendingFuture<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Future for PendingFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

// Implement standard Future trait for PendingFuture to support .await syntax
impl<T> StdFuture for PendingFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Delegate to the custom Future implementation
        Future::poll(self, cx)
    }
}

/// Shared future that can be cloned and polled from multiple places
pub struct SharedFuture<T> {
    inner: Arc<Mutex<SharedFutureInner<T>>>,
}

struct SharedFutureInner<T> {
    state: FutureState,
    result: Option<T>,
    wakers: Vec<Waker>,
}

impl<T: Clone> SharedFuture<T> {
    pub fn new() -> (Self, SharedFutureResolver<T>) {
        let inner = Arc::new(Mutex::new(SharedFutureInner {
            state: FutureState::Pending,
            result: None,
            wakers: Vec::new(),
        }));

        let future = SharedFuture {
            inner: inner.clone(),
        };

        let resolver = SharedFutureResolver { inner };

        (future, resolver)
    }

    pub fn state(&self) -> FutureState {
        self.inner.lock().unwrap().state.clone()
    }
}

impl<T: Clone> Clone for SharedFuture<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Clone> Future for SharedFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        
        match inner.state {
            FutureState::Ready => {
                if let Some(result) = &inner.result {
                    Poll::Ready(result.clone())
                } else {
                    Poll::Pending
                }
            }
            FutureState::Pending => {
                // Add waker to be notified when ready
                inner.wakers.push(cx.waker().clone());
                Poll::Pending
            }
            _ => Poll::Pending,
        }
    }
}

/// Resolver for SharedFuture
pub struct SharedFutureResolver<T> {
    inner: Arc<Mutex<SharedFutureInner<T>>>,
}

impl<T: Clone> SharedFutureResolver<T> {
    pub fn resolve(self, value: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.state = FutureState::Ready;
        inner.result = Some(value);
        
        // Wake all waiting tasks
        for waker in inner.wakers.drain(..) {
            waker.wake();
        }
    }

    pub fn cancel(self) {
        let mut inner = self.inner.lock().unwrap();
        inner.state = FutureState::Cancelled;
        
        // Wake all waiting tasks
        for waker in inner.wakers.drain(..) {
            waker.wake();
        }
    }
}

/// Join multiple futures and wait for all to complete
pub struct JoinFuture<T> {
    futures: Vec<BoxFuture<'static, T>>,
    results: Vec<Option<T>>,
}

impl<T: Send + 'static> JoinFuture<T> {
    pub fn new(futures: Vec<BoxFuture<'static, T>>) -> Self {
        let len = futures.len();
        Self {
            futures,
            results: vec![None; len],
        }
    }
}

impl<T: Send + 'static> Future for JoinFuture<T> {
    type Output = Vec<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut all_ready = true;

        for (i, future) in self.futures.iter_mut().enumerate() {
            if self.results[i].is_none() {
                match Pin::new(future).poll(cx) {
                    Poll::Ready(value) => {
                        self.results[i] = Some(value);
                    }
                    Poll::Pending => {
                        all_ready = false;
                    }
                }
            }
        }

        if all_ready {
            let mut results = Vec::new();
            for result in self.results.iter_mut() {
                if let Some(value) = result.take() {
                    results.push(value);
                }
            }
            Poll::Ready(results)
        } else {
            Poll::Pending
        }
    }
}

impl<T: Send + 'static> StdFuture for JoinFuture<T> {
    type Output = Vec<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(self, cx)
    }
}

/// Select the first future to complete
pub struct SelectFuture<T> {
    futures: Vec<BoxFuture<'static, T>>,
}

impl<T: Send + 'static> SelectFuture<T> {
    pub fn new(futures: Vec<BoxFuture<'static, T>>) -> Self {
        Self { futures }
    }
}

impl<T: Send + 'static> Future for SelectFuture<T> {
    type Output = (T, usize);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        for (i, future) in self.futures.iter_mut().enumerate() {
            if let Poll::Ready(value) = Pin::new(future).poll(cx) {
                return Poll::Ready((value, i));
            }
        }
        Poll::Pending
    }
}

impl<T: Send + 'static> StdFuture for SelectFuture<T> {
    type Output = (T, usize);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(self, cx)
    }
}

/// Map the result of a future
pub struct MapFuture<F, T, U> {
    future: F,
    map_fn: Option<fn(T) -> U>,
}

impl<F, T, U> MapFuture<F, T, U>
where
    F: Future<Output = T>,
{
    pub fn new(future: F, map_fn: fn(T) -> U) -> Self {
        Self {
            future,
            map_fn: Some(map_fn),
        }
    }
}

impl<F, T, U> Future for MapFuture<F, T, U>
where
    F: Future<Output = T>,
{
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        let future = unsafe { Pin::new_unchecked(&mut this.future) };
        
        match future.poll(cx) {
            Poll::Ready(value) => {
                if let Some(map_fn) = this.map_fn.take() {
                    Poll::Ready(map_fn(value))
                } else {
                    Poll::Pending
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Join two futures together
pub fn join<F1, F2>(f1: F1, f2: F2) -> JoinTwoFuture<F1, F2>
where
    F1: Future,
    F2: Future,
{
    JoinTwoFuture::new(f1, f2)
}

pub struct JoinTwoFuture<F1, F2>
where
    F1: Future,
    F2: Future,
{
    future1: Option<F1>,
    future2: Option<F2>,
    result1: Option<F1::Output>,
    result2: Option<F2::Output>,
}

impl<F1, F2> JoinTwoFuture<F1, F2>
where
    F1: Future,
    F2: Future,
{
    pub fn new(f1: F1, f2: F2) -> Self {
        Self {
            future1: Some(f1),
            future2: Some(f2),
            result1: None,
            result2: None,
        }
    }
}

impl<F1, F2> Future for JoinTwoFuture<F1, F2>
where
    F1: Future,
    F2: Future,
{
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        // Poll first future
        if this.result1.is_none() {
            if let Some(future1) = &mut this.future1 {
                let future1 = unsafe { Pin::new_unchecked(future1) };
                if let Poll::Ready(result) = future1.poll(cx) {
                    this.result1 = Some(result);
                    this.future1 = None;
                }
            }
        }

        // Poll second future
        if this.result2.is_none() {
            if let Some(future2) = &mut this.future2 {
                let future2 = unsafe { Pin::new_unchecked(future2) };
                if let Poll::Ready(result) = future2.poll(cx) {
                    this.result2 = Some(result);
                    this.future2 = None;
                }
            }
        }

        // Check if both are ready
        if let (Some(result1), Some(result2)) = (this.result1.take(), this.result2.take()) {
            Poll::Ready((result1, result2))
        } else {
            Poll::Pending
        }
    }
}

impl<F1, F2> StdFuture for JoinTwoFuture<F1, F2>
where
    F1: Future,
    F2: Future,
{
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(self, cx)
    }
}

/// Utility functions for future manipulation
pub mod utils {
    use super::*;

    /// Box a future for type erasure
    pub fn boxed<F>(future: F) -> BoxFuture<'static, F::Output>
    where
        F: Future + Send + 'static,
    {
        Box::pin(future)
    }

    /// Create a future that completes immediately with a value
    pub fn ok<T>(value: T) -> ReadyFuture<crate::error::Result<()>> {
        ReadyFuture::new(Ok(value))
    }

    /// Create a future that completes immediately with an error
    pub fn err<T>(error: FutureError) -> ReadyFuture<crate::error::Result<()>> {
        ReadyFuture::new(Err(error))
    }

    /// Map a future's result
    pub fn map<F, T, U>(future: F, map_fn: fn(T) -> U) -> MapFuture<F, T, U>
    where
        F: Future<Output = T>,
    {
        MapFuture::new(future, map_fn)
    }
}

/// Yield control back to the async executor (alternative implementation)
pub fn yield_control() -> YieldNow {
    YieldNow { yielded: false }
}

/// Future that yields control once and then completes
pub struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl StdFuture for YieldNow {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(self, cx)
    }
}
