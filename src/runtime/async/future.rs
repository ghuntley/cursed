use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

use crate::error::CursedError;

/// A future that is immediately ready with a value
pub struct ReadyFuture<T> {
    value: Option<T>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for ReadyFuture<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadyFuture")
            .field("value", &self.value)
            .finish()
    }
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
            results: (0..len).map(|_| None).collect(),
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

/// State for async future chaining
#[derive(Debug)]
pub enum ChainState {
    WaitingForFirst,
    WaitingForSecond,
    Completed,
}

/// A future that combines two futures sequentially with proper type safety
pub struct AndThenFuture<F, Func, T, U> {
    state: ChainState,
    first: Option<F>,
    chain_func: Option<Func>,
    second: Option<Pin<Box<dyn Future<Output = U> + Send>>>,
    _phantom: std::marker::PhantomData<(T, U)>,
}

impl<F, Func, T, U> AndThenFuture<F, Func, T, U>
where
    F: Future<Output = T>,
    Func: FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>>,
    T: Send + 'static,
    U: Send + 'static,
{
    pub fn new(first: F, chain_func: Func) -> Self {
        Self {
            state: ChainState::WaitingForFirst,
            first: Some(first),
            chain_func: Some(chain_func),
            second: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, Func, T, U> Future for AndThenFuture<F, Func, T, U>
where
    F: Future<Output = T>,
    Func: FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>>,
    T: Send + 'static,
    U: Send + 'static,
{
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        match this.state {
            ChainState::WaitingForFirst => {
                if let Some(ref mut first) = this.first {
                    let first_pin = unsafe { Pin::new_unchecked(first) };
                    if let Poll::Ready(result) = first_pin.poll(cx) {
                        this.first = None;
                        // Use the result to create the second future
                        if let Some(chain_func) = this.chain_func.take() {
                            this.second = Some(chain_func(result));
                            this.state = ChainState::WaitingForSecond;
                        } else {
                            // Should never happen - chain function was already taken
                            panic!("AndThenFuture chain function already consumed");
                        }
                    } else {
                        return Poll::Pending;
                    }
                }
            }
            ChainState::WaitingForSecond => {
                if let Some(ref mut second) = this.second {
                    let result = second.as_mut().poll(cx);
                    if result.is_ready() {
                        this.state = ChainState::Completed;
                    }
                    return result;
                }
            }
            ChainState::Completed => {
                panic!("AndThenFuture polled after completion");
            }
        }

        Poll::Pending
    }
}

/// A safer MapFuture that maps the output of a future
pub struct MapFuture<F, Func, T, U> {
    future: Option<F>,
    map_func: Option<Func>,
    _phantom: std::marker::PhantomData<(T, U)>,
}

impl<F, Func, T, U> MapFuture<F, Func, T, U>
where
    F: Future<Output = T>,
    Func: FnOnce(T) -> U,
{
    pub fn new(future: F, map_func: Func) -> Self {
        Self {
            future: Some(future),
            map_func: Some(map_func),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, Func, T, U> Future for MapFuture<F, Func, T, U>
where
    F: Future<Output = T>,
    Func: FnOnce(T) -> U,
{
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        if let Some(ref mut future) = this.future {
            let future_pin = unsafe { Pin::new_unchecked(future) };
            if let Poll::Ready(result) = future_pin.poll(cx) {
                this.future = None;
                if let Some(map_func) = this.map_func.take() {
                    return Poll::Ready(map_func(result));
                } else {
                    panic!("MapFuture polled after completion");
                }
            }
        }
        
        Poll::Pending
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

    /// Chain two futures together with proper type safety
    pub fn and_then<F, Func, T, U>(
        future: F,
        chain_func: Func,
    ) -> AndThenFuture<F, Func, T, U>
    where
        F: Future<Output = T>,
        Func: FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>>,
        T: Send + 'static,
        U: Send + 'static,
    {
        AndThenFuture::new(future, chain_func)
    }

    /// Map the output of a future
    pub fn map<F, Func, T, U>(future: F, map_func: Func) -> MapFuture<F, Func, T, U>
    where
        F: Future<Output = T>,
        Func: FnOnce(T) -> U,
    {
        MapFuture::new(future, map_func)
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
            return Poll::Ready(Err(CursedError::runtime_error("Future timed out")));
        }
        
        Poll::Pending
    }
}

/// Extension trait for futures with additional combinators
pub trait FutureExt: Future {
    /// Add a timeout to this future
    fn timeout(self, duration: Duration) -> TimeoutFuture<Self>
    where
        Self: Sized,
    {
        TimeoutFuture::new(self, duration)
    }

    /// Map the output of this future
    fn map<F, U>(self, f: F) -> MapFuture<Self, F, Self::Output, U>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> U,
    {
        MapFuture::new(self, f)
    }

    /// Chain this future with another
    fn and_then<F, Fut, U>(self, f: F) -> AndThenFuture<Self, F, Self::Output, U>
    where
        Self: Sized,
        Self::Output: Send + 'static,
        F: FnOnce(Self::Output) -> Pin<Box<dyn Future<Output = U> + Send>>,
        U: Send + 'static,
    {
        AndThenFuture::new(self, f)
    }
}

impl<F: Future> FutureExt for F {}

/// Result type for async operations
#[derive(Debug, Clone, PartialEq)]
pub enum AsyncResult<T> {
    Success(T),
    Error(String),
    Timeout,
}

impl<T> AsyncResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            AsyncResult::Success(value) => value,
            AsyncResult::Error(err) => panic!("AsyncResult error: {}", err),
            AsyncResult::Timeout => panic!("AsyncResult timeout"),
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, AsyncResult::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, AsyncResult::Error(_))
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, AsyncResult::Timeout)
    }
}

impl<T> From<Result<T, CursedError>> for AsyncResult<T> {
    fn from(result: Result<T, CursedError>) -> Self {
        match result {
            Ok(value) => AsyncResult::Success(value),
            Err(err) => AsyncResult::Error(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_ready_future() {
        let future = ReadyFuture::new(42);
        let result = future.await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_delay_future() {
        let start = Instant::now();
        let future = DelayFuture::new(Duration::from_millis(10));
        future.await;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[tokio::test]
    async fn test_yield_future() {
        let future = YieldFuture::new();
        future.await;
        // If we get here, the future completed successfully
    }

    #[tokio::test]
    async fn test_map_future() {
        let future = ReadyFuture::new(42);
        let mapped = future.map(|x| x * 2);
        let result = mapped.await;
        assert_eq!(result, 84);
    }

    #[tokio::test]
    async fn test_and_then_future() {
        let future = ReadyFuture::new(42);
        let chained = util::and_then(future, |x| {
            Box::pin(async move { x * 2 }) as Pin<Box<dyn Future<Output = i32> + Send>>
        });
        let result = chained.await;
        assert_eq!(result, 84);
    }

    #[tokio::test]
    async fn test_timeout_future_success() {
        let future = ReadyFuture::new(42);
        let timeout_future = future.timeout(Duration::from_millis(100));
        let result = timeout_future.await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_timeout_future_timeout() {
        let future = PendingFuture::<i32>::new();
        let timeout_future = future.timeout(Duration::from_millis(10));
        let result = timeout_future.await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_join_future() {
        let futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> = vec![
            Box::pin(ReadyFuture::new(1)),
            Box::pin(ReadyFuture::new(2)),
            Box::pin(ReadyFuture::new(3)),
        ];
        let join_future = JoinFuture::new(futures);
        let results = join_future.await;
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_select_future() {
        let futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> = vec![
            Box::pin(ReadyFuture::new(1)),
            Box::pin(PendingFuture::new()),
            Box::pin(ReadyFuture::new(3)),
        ];
        let select_future = SelectFuture::new(futures);
        let (result, index) = select_future.await;
        // Should return the first ready future (index 0 with value 1)
        assert_eq!(result, 1);
        assert_eq!(index, 0);
    }

    #[tokio::test]
    async fn test_lazy_future() {
        let lazy = LazyFuture::new(|| {
            Box::pin(async { 42 }) as Pin<Box<dyn Future<Output = i32> + Send>>
        });
        let result = lazy.await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_async_result() {
        let success = AsyncResult::Success(42);
        assert!(success.is_success());
        assert!(!success.is_error());
        assert!(!success.is_timeout());

        let error: AsyncResult<i32> = AsyncResult::Error("test error".to_string());
        assert!(!error.is_success());
        assert!(error.is_error());
        assert!(!error.is_timeout());

        let timeout = AsyncResult::<i32>::Timeout;
        assert!(!timeout.is_success());
        assert!(!timeout.is_error());
        assert!(timeout.is_timeout());
    }

    #[tokio::test]
    async fn test_buffered_future() {
        let mut buffered = BufferedFuture::new(3);
        
        assert!(buffered.push(1).is_ok());
        assert!(buffered.push(2).is_ok());
        assert!(buffered.push(3).is_ok());
        assert!(buffered.push(4).is_err()); // Buffer full
        
        assert_eq!(buffered.pop(), Some(1));
        assert_eq!(buffered.pop(), Some(2));
        assert_eq!(buffered.pop(), Some(3));
        assert_eq!(buffered.pop(), None);
    }
}
