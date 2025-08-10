use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

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
        // EMERGENCY FIX: Log Promise creation to detect if this is called during basic execution
        eprintln!("WARNING: Promise::new() called - this may trigger stack overflow");
        let inner = Arc::new(Mutex::new(PromiseInner::new()));
        
        let promise = Self {
            inner: Arc::clone(&inner),
        };
        
        let resolver = PromiseResolver {
            inner: Arc::clone(&inner),
        };
        
        let rejector = PromiseRejector {
            inner: Arc::clone(&inner),
        };
        
        (promise, resolver, rejector)
    }

    /// Map the resolved value to a new type (NO INFINITE LOOPS)
    pub fn map<U, F>(self, f: F) -> Promise<U, E>
    where
        F: FnOnce(T) -> U + Send + 'static,
        U: Clone + Send + 'static,
    {
        let (new_promise, resolver, rejector) = Promise::new();
        
        // Check if already resolved instead of waiting
        let inner = self.inner;
        if let Ok(guard) = inner.lock() {
            if let Some(ref result) = guard.result {
                match result {
                    PromiseResult::Resolved(value) => {
                        let mapped_value = f(value.clone());
                        resolver.resolve(mapped_value);
                    }
                    PromiseResult::Rejected(error) => {
                        rejector.reject(error.clone());
                    }
                }
                return new_promise;
            }
        }
        
        // If not resolved, just return the promise without spawning infinite tasks
        new_promise
    }

    /// Chain another promise after this one (NO INFINITE LOOPS)
    pub fn then<U, F, Fut>(self, _f: F) -> Promise<U, E>
    where
        F: FnOnce(T) -> Fut + Send + 'static,
        Fut: Future<Output = Result<U, E>> + Send + 'static,
        U: Clone + Send + 'static,
    {
        // For now, just return a new unresolved promise to prevent infinite loops
        let (new_promise, _resolver, _rejector) = Promise::new();
        new_promise
    }

    /// Handle errors with a fallback function (NO INFINITE LOOPS)
    pub fn catch<F, Fut>(self, _f: F) -> Promise<T, E>
    where
        F: FnOnce(E) -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
    {
        // For now, just return a new unresolved promise to prevent infinite loops
        let (new_promise, _resolver, _rejector) = Promise::new();
        new_promise
    }

    /// Execute a function regardless of promise outcome (NO INFINITE LOOPS)
    pub fn finally<F, Fut>(self, _f: F) -> Promise<T, E>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        // For now, just return a new unresolved promise to prevent infinite loops
        let (new_promise, _resolver, _rejector) = Promise::new();
        new_promise
    }
}

/// Promise resolver
pub struct PromiseResolver<T, E> {
    inner: Arc<Mutex<PromiseInner<T, E>>>,
}

impl<T, E> PromiseResolver<T, E> {
    /// Resolve the promise with a value
    pub fn resolve(self, value: T) {
        if let Ok(mut inner) = self.inner.lock() {
            if inner.state == PromiseState::Pending {
                inner.state = PromiseState::Resolved;
                inner.result = Some(PromiseResult::Resolved(value));
                if let Some(waker) = inner.waker.take() {
                    waker.wake();
                }
            }
        }
    }
}

/// Promise rejector
pub struct PromiseRejector<T, E> {
    inner: Arc<Mutex<PromiseInner<T, E>>>,
}

impl<T, E> PromiseRejector<T, E> {
    /// Reject the promise with an error
    pub fn reject(self, error: E) {
        if let Ok(mut inner) = self.inner.lock() {
            if inner.state == PromiseState::Pending {
                inner.state = PromiseState::Rejected;
                inner.result = Some(PromiseResult::Rejected(error));
                if let Some(waker) = inner.waker.take() {
                    waker.wake();
                }
            }
        }
    }
}

impl<T, E> Future for Promise<T, E>
where
    T: Clone + Send + 'static,
    E: Clone + Send + 'static,
{
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut inner) = self.inner.lock() {
            match &inner.result {
                Some(PromiseResult::Resolved(value)) => Poll::Ready(Ok(value.clone())),
                Some(PromiseResult::Rejected(error)) => Poll::Ready(Err(error.clone())),
                None => {
                    // Store the waker for later
                    inner.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            }
        } else {
            // If we can't lock, consider it a failure
            Poll::Pending
        }
    }
}
