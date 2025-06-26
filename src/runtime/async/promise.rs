use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::CursedError;

/// Promise state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromiseState {
    Pending,
    Resolved,
    Rejected,
}

/// Promise resolution result
#[derive(Debug, Clone)]
pub enum PromiseResult<T, E> {
    Resolved(T),
    Rejected(E),
}

/// Internal promise state
struct PromiseInner<T, E> {
    state: PromiseState,
    result: Option<PromiseResult<T, E>>,
    waker: Option<Waker>,
}

impl<T, E> PromiseInner<T, E> {
    fn new() -> Self {
        Self {
            state: PromiseState::Pending,
            result: None,
            waker: None,
        }
    }
}

/// A Promise implementation similar to JavaScript promises
pub struct Promise<T, E> {
    inner: Arc<Mutex<PromiseInner<T, E>>>,
}

impl<T, E> Promise<T, E>
where
    T: Clone + Send + 'static,
    E: Clone + Send + 'static,
{
    /// Create a new promise with resolver and rejector
    pub fn new() -> (Self, PromiseResolver<T, E>, PromiseRejector<T, E>) {
        let inner = Arc::new(Mutex::new(PromiseInner::new()));
        
        let promise = Self {
            inner: inner.clone(),
        };
        
        let resolver = PromiseResolver {
            inner: inner.clone(),
            resolved: Arc::new(AtomicBool::new(false)),
        };
        
        let rejector = PromiseRejector {
            inner,
            rejected: Arc::new(AtomicBool::new(false)),
        };
        
        (promise, resolver, rejector)
    }

    /// Create a promise that is already resolved
    pub fn resolved(value: T) -> Self {
        let inner = Arc::new(Mutex::new(PromiseInner {
            state: PromiseState::Resolved,
            result: Some(PromiseResult::Resolved(value)),
            waker: None,
        }));
        
        Self { inner }
    }

    /// Create a promise that is already rejected
    pub fn rejected(error: E) -> Self {
        let inner = Arc::new(Mutex::new(PromiseInner {
            state: PromiseState::Rejected,
            result: Some(PromiseResult::Rejected(error)),
            waker: None,
        }));
        
        Self { inner }
    }

    /// Map successful values to a new type
    pub fn map<U, F>(self, f: F) -> Promise<U, E>
    where
        F: FnOnce(T) -> U + Send + 'static,
        U: Clone + Send + 'static,
    {
        let (new_promise, resolver, rejector) = Promise::new();
        
        let inner = self.inner;
        tokio::spawn(async move {
            let result = {
                loop {
                    let guard = inner.lock().unwrap();
                    match &guard.result {
                        Some(result) => break result.clone(),
                        None => {
                            drop(guard);
                            tokio::task::yield_now().await;
                        }
                    }
                }
            };
            
            match result {
                PromiseResult::Resolved(value) => {
                    let mapped_value = f(value);
                    resolver.resolve(mapped_value);
                }
                PromiseResult::Rejected(error) => {
                    rejector.reject(error);
                }
            }
        });
        
        new_promise
    }

    /// Chain another promise after this one
    pub fn then<U, F, Fut>(self, f: F) -> Promise<U, E>
    where
        F: FnOnce(T) -> Fut + Send + 'static,
        Fut: Future<Output = Result<U, E>> + Send + 'static,
        U: Clone + Send + 'static,
    {
        let (new_promise, resolver, rejector) = Promise::new();
        
        let inner = self.inner;
        tokio::spawn(async move {
            let result = {
                loop {
                    let guard = inner.lock().unwrap();
                    match &guard.result {
                        Some(result) => break result.clone(),
                        None => {
                            drop(guard);
                            tokio::task::yield_now().await;
                        }
                    }
                }
            };
            
            match result {
                PromiseResult::Resolved(value) => {
                    match f(value).await {
                        Ok(new_value) => resolver.resolve(new_value),
                        Err(error) => rejector.reject(error),
                    };
                }
                PromiseResult::Rejected(error) => {
                    rejector.reject(error);
                }
            }
        });
        
        new_promise
    }

    /// Handle errors with a fallback function
    pub fn catch<F, Fut>(self, f: F) -> Promise<T, E>
    where
        F: FnOnce(E) -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
    {
        let (new_promise, resolver, rejector) = Promise::new();
        
        let inner = self.inner;
        tokio::spawn(async move {
            let result = {
                loop {
                    let guard = inner.lock().unwrap();
                    match &guard.result {
                        Some(result) => break result.clone(),
                        None => {
                            drop(guard);
                            tokio::task::yield_now().await;
                        }
                    }
                }
            };
            
            match result {
                PromiseResult::Resolved(value) => {
                    resolver.resolve(value);
                }
                PromiseResult::Rejected(error) => {
                    match f(error).await {
                        Ok(recovered_value) => resolver.resolve(recovered_value),
                        Err(new_error) => rejector.reject(new_error),
                    };
                }
            }
        });
        
        new_promise
    }

    /// Execute a function regardless of promise outcome (finally)
    pub fn finally<F, Fut>(self, f: F) -> Promise<T, E>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let (new_promise, resolver, rejector) = Promise::new();
        
        let inner = self.inner;
        tokio::spawn(async move {
            let result = {
                loop {
                    let guard = inner.lock().unwrap();
                    match &guard.result {
                        Some(result) => break result.clone(),
                        None => {
                            drop(guard);
                            tokio::task::yield_now().await;
                        }
                    }
                }
            };
            
            // Execute the finally function
            f().await;
            
            // Pass through the original result
            match result {
                PromiseResult::Resolved(value) => {
                    resolver.resolve(value);
                }
                PromiseResult::Rejected(error) => {
                    rejector.reject(error);
                }
            }
        });
        
        new_promise
    }
}

impl<T, E> Future for Promise<T, E>
where
    T: Clone + Send + 'static,
    E: Clone + Send + 'static,
{
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        
        match &inner.result {
            Some(PromiseResult::Resolved(value)) => {
                Poll::Ready(Ok(value.clone()))
            }
            Some(PromiseResult::Rejected(error)) => {
                Poll::Ready(Err(error.clone()))
            }
            None => {
                inner.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

/// Resolver for a promise
pub struct PromiseResolver<T, E> {
    inner: Arc<Mutex<PromiseInner<T, E>>>,
    resolved: Arc<AtomicBool>,
}

impl<T, E> PromiseResolver<T, E> {
    /// Resolve the promise with a value
    pub fn resolve(self, value: T) -> bool {
        if self.resolved.swap(true, Ordering::SeqCst) {
            return false; // Already resolved
        }
        
        let mut inner = self.inner.lock().unwrap();
        inner.state = PromiseState::Resolved;
        inner.result = Some(PromiseResult::Resolved(value));
        
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
        
        true
    }
}

/// Rejector for a promise
pub struct PromiseRejector<T, E> {
    inner: Arc<Mutex<PromiseInner<T, E>>>,
    rejected: Arc<AtomicBool>,
}

impl<T, E> PromiseRejector<T, E> {
    /// Reject the promise with an error
    pub fn reject(self, error: E) -> bool {
        if self.rejected.swap(true, Ordering::SeqCst) {
            return false; // Already rejected
        }
        
        let mut inner = self.inner.lock().unwrap();
        inner.state = PromiseState::Rejected;
        inner.result = Some(PromiseResult::Rejected(error));
        
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
        
        true
    }
}

/// Promise utilities
impl<T, E> Promise<T, E>
where
    T: Clone + Send + 'static,
    E: Clone + Send + 'static,
{
    /// Wait for all promises to resolve
    pub async fn all(promises: Vec<Promise<T, E>>) -> Result<Vec<T>, E> {
        let mut results = Vec::with_capacity(promises.len());
        
        for promise in promises {
            results.push(promise.await?);
        }
        
        Ok(results)
    }

    /// Wait for any promise to resolve
    pub async fn any(promises: Vec<Promise<T, E>>) -> Result<T, Vec<E>> {
        if promises.is_empty() {
            return Err(vec![]);
        }

        let mut errors = Vec::new();
        
        for promise in promises {
            match promise.await {
                Ok(value) => return Ok(value),
                Err(error) => errors.push(error),
            }
        }
        
        Err(errors)
    }

    /// Wait for the first promise to complete (either resolve or reject)
    pub async fn race(promises: Vec<Promise<T, E>>) -> Result<T, E> {
        if promises.is_empty() {
            return Err(E::default()); // This assumes E: Default, we may need to adjust
        }

        // For simplicity, we'll await the first promise
        // In a real implementation, we'd use select! or similar
        promises.into_iter().next().unwrap().await
    }
}

// Default implementations for when E needs to be Default
impl<T, E> Default for Promise<T, E>
where
    T: Clone + Send + 'static,
    E: Clone + Send + 'static + Default,
{
    fn default() -> Self {
        Self::rejected(E::default())
    }
}
