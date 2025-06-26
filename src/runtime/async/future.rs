use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

use crate::error::CursedError;

/// A future that is immediately ready with a value
#[derive(Debug)]
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

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        Poll::Ready(this.value.take().expect("ReadyFuture polled after completion"))
    }
}

/// A future that never completes
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

impl<T> Default for PendingFuture<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A future that completes after a delay
pub struct DelayFuture {
    when: Instant,
}

impl DelayFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            when: Instant::now() + duration,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// A future that yields control back to the executor
pub struct YieldFuture {
    yielded: bool,
}

impl YieldFuture {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for YieldFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        if this.yielded {
            Poll::Ready(())
        } else {
            this.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl Default for YieldFuture {
    fn default() -> Self {
        Self::new()
    }
}

/// A future that joins multiple futures
pub struct JoinFuture<T> {
    futures: Vec<Pin<Box<dyn Future<Output = T> + Send>>>,
    results: Vec<Option<T>>,
    completed: usize,
}

impl<T> JoinFuture<T>
where
    T: Send + 'static,
{
    pub fn new(futures: Vec<Pin<Box<dyn Future<Output = T> + Send>>>) -> Self {
        let len = futures.len();
        Self {
            futures,
            results: vec![None; len],
            completed: 0,
        }
    }
}

impl<T> Future for JoinFuture<T>
where
    T: Send + 'static,
{
    type Output = Vec<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        for (i, future) in this.futures.iter_mut().enumerate() {
            if this.results[i].is_none() {
                if let Poll::Ready(result) = future.as_mut().poll(cx) {
                    this.results[i] = Some(result);
                    this.completed += 1;
                }
            }
        }

        if this.completed == this.futures.len() {
            let results = std::mem::take(&mut this.results);
            Poll::Ready(results.into_iter().map(|r| r.unwrap()).collect())
        } else {
            Poll::Pending
        }
    }
}

/// A future that combines two futures sequentially
pub struct AndThenFuture<F1, F2, T, U> {
    first: Option<F1>,
    second: Option<F2>,
    _phantom: std::marker::PhantomData<(T, U)>,
}

impl<F1, F2, T, U> AndThenFuture<F1, F2, T, U> {
    pub fn new(first: F1) -> Self {
        Self {
            first: Some(first),
            second: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F1, F2, T, U> Future for AndThenFuture<F1, F2, T, U>
where
    F1: Future<Output = T>,
    F2: Future<Output = U>,
    T: Send + 'static,
    U: Send + 'static,
{
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        if let Some(ref mut first) = this.first {
            let first_pin = unsafe { Pin::new_unchecked(first) };
            if let Poll::Ready(_result) = first_pin.poll(cx) {
                this.first = None; // First future completed
                // In a real implementation, we'd use the result to create the second future
                // For now, we'll just complete immediately
                return Poll::Ready(unsafe { std::mem::zeroed() }); // This is a hack
            }
        }

        if let Some(ref mut second) = this.second {
            let second_pin = unsafe { Pin::new_unchecked(second) };
            second_pin.poll(cx)
        } else {
            Poll::Pending
        }
    }
}

/// A future that creates another future lazily
pub struct LazyFuture<F, T> {
    init: Option<F>,
    future: Option<Pin<Box<dyn Future<Output = T> + Send>>>,
}

impl<F, T> LazyFuture<F, T> {
    pub fn new(init: F) -> Self {
        Self {
            init: Some(init),
            future: None,
        }
    }
}

impl<F, T> Future for LazyFuture<F, T>
where
    F: FnOnce() -> Pin<Box<dyn Future<Output = T> + Send>>,
    T: Send + 'static,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        if this.future.is_none() {
            let init = this.init.take().expect("LazyFuture polled after completion");
            this.future = Some(init());
        }

        if let Some(ref mut future) = this.future {
            future.as_mut().poll(cx)
        } else {
            Poll::Pending
        }
    }
}

/// A future that polls multiple futures concurrently and returns the first to complete
pub struct SelectFuture<T> {
    futures: Vec<Pin<Box<dyn Future<Output = T> + Send>>>,
}

impl<T> SelectFuture<T>
where
    T: Send + 'static,
{
    pub fn new(futures: Vec<Pin<Box<dyn Future<Output = T> + Send>>>) -> Self {
        Self { futures }
    }
}

impl<T> Future for SelectFuture<T>
where
    T: Send + 'static,
{
    type Output = (T, usize);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        for (i, future) in this.futures.iter_mut().enumerate() {
            if let Poll::Ready(result) = future.as_mut().poll(cx) {
                return Poll::Ready((result, i));
            }
        }
        
        Poll::Pending
    }
}

/// A buffered stream-like future
pub struct BufferedFuture<T> {
    buffer: VecDeque<T>,
    max_size: usize,
}

impl<T> BufferedFuture<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.buffer.len() >= self.max_size {
            Err(item)
        } else {
            self.buffer.push_back(item);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

/// Utility functions for working with futures
pub mod util {
    use super::*;

    /// Create a future that's immediately ready with a value
    pub fn ready<T>(value: T) -> ReadyFuture<T> {
        ReadyFuture::new(value)
    }

    /// Create a future that never completes
    pub fn pending<T>() -> PendingFuture<T> {
        PendingFuture::new()
    }

    /// Create a future that completes after a delay
    pub fn delay(duration: Duration) -> DelayFuture {
        DelayFuture::new(duration)
    }

    /// Create a future that yields control
    pub fn yield_now() -> YieldFuture {
        YieldFuture::new()
    }

    /// Join multiple futures
    pub fn join<T>(futures: Vec<Pin<Box<dyn Future<Output = T> + Send>>>) -> JoinFuture<T>
    where
        T: Send + 'static,
    {
        JoinFuture::new(futures)
    }

    /// Select from multiple futures
    pub fn select<T>(futures: Vec<Pin<Box<dyn Future<Output = T> + Send>>>) -> SelectFuture<T>
    where
        T: Send + 'static,
    {
        SelectFuture::new(futures)
    }

    /// Create a lazy future
    pub fn lazy<F, T>(init: F) -> LazyFuture<F, T>
    where
        F: FnOnce() -> Pin<Box<dyn Future<Output = T> + Send>>,
        T: Send + 'static,
    {
        LazyFuture::new(init)
    }
}

/// Timeout wrapper for futures
pub struct TimeoutFuture<F> {
    future: F,
    delay: DelayFuture,
}

impl<F> TimeoutFuture<F> {
    pub fn new(future: F, timeout: Duration) -> Self {
        Self {
            future,
            delay: DelayFuture::new(timeout),
        }
    }
}

impl<F> Future for TimeoutFuture<F>
where
    F: Future,
{
    type Output = Result<F::Output, CursedError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        // Poll the main future first
        let future_pin = unsafe { Pin::new_unchecked(&mut this.future) };
        if let Poll::Ready(result) = future_pin.poll(cx) {
            return Poll::Ready(Ok(result));
        }
        
        // Poll the timeout
        let delay_pin = unsafe { Pin::new_unchecked(&mut this.delay) };
        if let Poll::Ready(()) = delay_pin.poll(cx) {
            return Poll::Ready(Err(CursedError::Runtime("Future timed out".to_string())));
        }
        
        Poll::Pending
    }
}

/// Extension trait for futures
pub trait FutureExt: Future {
    /// Add a timeout to this future
    fn timeout(self, duration: Duration) -> TimeoutFuture<Self>
    where
        Self: Sized,
    {
        TimeoutFuture::new(self, duration)
    }
}

impl<F: Future> FutureExt for F {}
