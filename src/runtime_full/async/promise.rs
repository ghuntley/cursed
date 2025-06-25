use crate::error::CursedError;
/// Promise implementation for CURSED async operations
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};
use std::future::Future as StdFuture;
use crate::runtime::r#async::{Future, FutureError, FutureResult, FutureState};

/// Promise state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum PromiseState {
    /// Promise is pending resolution
    Pending,
    /// Promise was resolved with a value
    Resolved,
    /// Promise was rejected with an error
    Rejected,
}

/// Promise type that can be resolved or rejected from anywhere
pub struct Promise<T> {
    inner: Arc<Mutex<PromiseInner<T>>>,
}

struct PromiseInner<T> {
    state: PromiseState,
    result: Option<crate::error::Result<()>>,
    wakers: Vec<Waker>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Promise<T> {
    /// Create a new promise with its resolver and rejecter
    pub fn new() -> (Self, PromiseResolver<T>, PromiseRejecter<T>) {
        let inner = Arc::new(Mutex::new(PromiseInner {
            state: PromiseState::Pending,
            result: None,
            wakers: Vec::new(),
            _phantom: std::marker::PhantomData,
        }));

        let promise = Promise {
            inner: inner.clone(),
        };

        let resolver = PromiseResolver {
            inner: inner.clone(),
        };

        let rejecter = PromiseRejecter { inner };

        (promise, resolver, rejecter)
    }

    /// Create a promise that is already resolved
    pub fn resolved(value: T) -> Self {
        let inner = Arc::new(Mutex::new(PromiseInner {
            state: PromiseState::Resolved,
            result: Some(Ok(value)),
            wakers: Vec::new(),
        }));

        Promise { inner }
    }

    /// Create a promise that is already rejected
    pub fn rejected(error: FutureError) -> Self {
        let inner = Arc::new(Mutex::new(PromiseInner {
            state: PromiseState::Rejected,
            result: Some(Err(error)),
            wakers: Vec::new(),
        }));

        Promise { inner }
    }

    /// Get the current state of the promise
    pub fn state(&self) -> PromiseState {
        self.inner.lock().unwrap().state.clone()
    }

    /// Check if the promise is resolved
    pub fn is_resolved(&self) -> bool {
        matches!(self.state(), PromiseState::Resolved)
    }

    /// Check if the promise is rejected
    pub fn is_rejected(&self) -> bool {
        matches!(self.state(), PromiseState::Rejected)
    }

    /// Check if the promise is pending
    pub fn is_pending(&self) -> bool {
        matches!(self.state(), PromiseState::Pending)
    }

    /// Try to get the result without waiting (returns None if still pending)
    pub fn try_result(&self) -> Option<FutureResult<T>>
    where
        T: Clone,
    {
        let inner = self.inner.lock().unwrap();
        match inner.state {
            PromiseState::Pending => None,
            _ => inner.result.clone(),
        }
    }
}

impl<T> Clone for Promise<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Future for Promise<T>
where
    T: Clone,
{
    type Output = FutureResult<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();

        match inner.state {
            PromiseState::Pending => {
                // Add waker to be notified when resolved/rejected
                inner.wakers.push(cx.waker().clone());
                Poll::Pending
            }
            PromiseState::Resolved | PromiseState::Rejected => {
                if let Some(result) = &inner.result {
                    Poll::Ready(result.clone())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

// Implement standard Future trait for Promise to support .await syntax
impl<T> StdFuture for Promise<T>
where
    T: Clone + Send,
{
    type Output = FutureResult<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Delegate to the custom Future implementation
        Future::poll(self, cx)
    }
}

/// Resolver for a promise - can resolve the promise with a value
pub struct PromiseResolver<T> {
    inner: Arc<Mutex<PromiseInner<T>>>,
}

impl<T> PromiseResolver<T> {
    /// Resolve the promise with a value
    pub fn resolve(self, value: T) -> crate::error::Result<()> {
        let mut inner = self.inner.lock().unwrap();

        if inner.state != PromiseState::Pending {
            return Err(FutureError::InvalidState);
        }

        inner.state = PromiseState::Resolved;
        inner.result = Some(Ok(value));

        // Wake all waiting tasks
        for waker in inner.wakers.drain(..) {
            waker.wake();
        }

        Ok(())
    }

    /// Try to resolve the promise, returning false if already resolved/rejected
    pub fn try_resolve(self, value: T) -> bool {
        self.resolve(value).is_ok()
    }
}

/// Rejecter for a promise - can reject the promise with an error
pub struct PromiseRejecter<T> {
    inner: Arc<Mutex<PromiseInner<T>>>,
}

impl<T> PromiseRejecter<T> {
    /// Reject the promise with an error
    pub fn reject(self, error: FutureError) -> crate::error::Result<()> {
        let mut inner = self.inner.lock().unwrap();

        if inner.state != PromiseState::Pending {
            return Err(FutureError::InvalidState);
        }

        inner.state = PromiseState::Rejected;
        inner.result = Some(Err(error));

        // Wake all waiting tasks
        for waker in inner.wakers.drain(..) {
            waker.wake();
        }

        Ok(())
    }

    /// Try to reject the promise, returning false if already resolved/rejected
    pub fn try_reject(self, error: FutureError) -> bool {
        self.reject(error).is_ok()
    }

    /// Reject with a simple message
    pub fn reject_with_message(self, message: &str) -> crate::error::Result<()> {
        self.reject(FutureError::Failed(message.to_string()))
    }
}

/// All promise - resolves when all promises resolve, rejects if any reject
pub struct AllPromise<T> {
    promises: Vec<Promise<T>>,
    results: Vec<Option<T>>,
}

impl<T: Clone> AllPromise<T> {
    pub fn new(promises: Vec<Promise<T>>) -> Self {
        let len = promises.len();
        Self {
            promises,
            results: vec![None; len],
        }
    }
}

impl<T: Clone> Future for AllPromise<T> {
    type Output = FutureResult<Vec<T>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut all_ready = true;

        for (i, promise) in self.promises.iter_mut().enumerate() {
            if self.results[i].is_none() {
                match Pin::new(promise).poll(cx) {
                    Poll::Ready(Ok(value)) => {
                        self.results[i] = Some(value);
                    }
                    Poll::Ready(Err(error)) => {
                        // If any promise fails, the whole thing fails
                        return Poll::Ready(Err(error));
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
            Poll::Ready(Ok(results))
        } else {
            Poll::Pending
        }
    }
}

impl<T: Clone> StdFuture for AllPromise<T> {
    type Output = FutureResult<Vec<T>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(self, cx)
    }
}

/// Race promise - resolves with the first promise to complete
pub struct RacePromise<T> {
    promises: Vec<Promise<T>>,
}

impl<T: Clone> RacePromise<T> {
    pub fn new(promises: Vec<Promise<T>>) -> Self {
        Self { promises }
    }
}

impl<T: Clone> Future for RacePromise<T> {
    type Output = FutureResult<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        for promise in self.promises.iter_mut() {
            if let Poll::Ready(result) = Pin::new(promise).poll(cx) {
                return Poll::Ready(result);
            }
        }
        Poll::Pending
    }
}

impl<T: Clone> StdFuture for RacePromise<T> {
    type Output = FutureResult<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(self, cx)
    }
}

/// Any promise - resolves when any promise resolves, only fails if all fail
pub struct AnyPromise<T> {
    promises: Vec<Promise<T>>,
    failed_count: usize,
    errors: Vec<FutureError>,
}

impl<T: Clone> AnyPromise<T> {
    pub fn new(promises: Vec<Promise<T>>) -> Self {
        Self {
            promises,
            failed_count: 0,
            errors: Vec::new(),
        }
    }
}

impl<T: Clone> Future for AnyPromise<T> {
    type Output = FutureResult<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        for promise in self.promises.iter_mut() {
            match Pin::new(promise).poll(cx) {
                Poll::Ready(Ok(value)) => {
                    // First success wins
                    return Poll::Ready(Ok(value));
                }
                Poll::Ready(Err(error)) => {
                    self.failed_count += 1;
                    self.errors.push(error);
                    
                    // If all promises failed, return the first error
                    if self.failed_count == self.promises.len() {
                        let first_error = self.errors.into_iter().next()
                            .unwrap_or(FutureError::Failed("All promises failed".to_string()));
                        return Poll::Ready(Err(first_error));
                    }
                }
                Poll::Pending => {}
            }
        }
        Poll::Pending
    }
}

impl<T: Clone> StdFuture for AnyPromise<T> {
    type Output = FutureResult<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(self, cx)
    }
}

/// Utility functions for promise manipulation
pub mod utils {
    use super::*;

    /// Create a promise that resolves immediately
    pub fn resolve<T>(value: T) -> Promise<T> {
        Promise::resolved(value)
    }

    /// Create a promise that rejects immediately
    pub fn reject<T>(error: FutureError) -> Promise<T> {
        Promise::rejected(error)
    }

    /// Create a promise that rejects with a message
    pub fn reject_with_message<T>(message: &str) -> Promise<T> {
        Promise::rejected(FutureError::Failed(message.to_string()))
    }

    /// Wait for all promises to resolve
    pub fn all<T: Clone>(promises: Vec<Promise<T>>) -> AllPromise<T> {
        AllPromise::new(promises)
    }

    /// Race multiple promises (first to complete wins)
    pub fn race<T: Clone>(promises: Vec<Promise<T>>) -> RacePromise<T> {
        RacePromise::new(promises)
    }

    /// Wait for any promise to resolve (only fails if all fail)
    pub fn any<T: Clone>(promises: Vec<Promise<T>>) -> AnyPromise<T> {
        AnyPromise::new(promises)
    }

    /// Convert a future to a promise
    pub fn from_future<F>(future: F) -> Promise<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let (promise, resolver, rejecter) = Promise::new();

        // Spawn the future to run it
        if let Some(runtime) = crate::runtime::r#async::get_async_runtime() {
            runtime.spawn_with_resolver(future, resolver, rejecter);
        }

        promise
    }

    /// Create a promise from a closure that runs in a background task
    pub fn spawn<F, T>(f: F) -> Promise<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (promise, resolver, _rejecter) = Promise::new();

        // Spawn a task to run the closure
        std::thread::spawn(move || {
            let result = f();
            let _ = resolver.resolve(result);
        });

        promise
    }
}

/// Promise chain operations
impl<T> Promise<T>
where
    T: Clone + Send + 'static,
{
    /// Chain this promise with another operation
    pub fn then<F, U>(self, f: F) -> Promise<U>
    where
        F: FnOnce(T) -> U + Send + 'static,
        U: Send + 'static,
    {
        let (new_promise, resolver, rejecter) = Promise::new();

        if let Some(runtime) = crate::runtime::r#async::get_async_runtime() {
            runtime.spawn_chain(self, f, resolver, rejecter);
        }

        new_promise
    }

    /// Catch errors in this promise
    pub fn catch<F>(self, f: F) -> Promise<T>
    where
        F: FnOnce(FutureError) -> T + Send + 'static,
    {
        let (new_promise, resolver, rejecter) = Promise::new();

        if let Some(runtime) = crate::runtime::r#async::get_async_runtime() {
            runtime.spawn_catch(self, f, resolver, rejecter);
        }

        new_promise
    }

    /// Transform this promise with a function that can fail
    pub fn map<F, U>(self, f: F) -> Promise<U>
    where
        F: FnOnce(T) -> FutureResult<U> + Send + 'static,
        U: Send + 'static,
    {
        let (new_promise, resolver, rejecter) = Promise::new();

        if let Some(runtime) = crate::runtime::r#async::get_async_runtime() {
            runtime.spawn_map(self, f, resolver, rejecter);
        }

        new_promise
    }
}
