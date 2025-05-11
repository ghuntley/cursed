//! Disabled version of concurrenz module for backward compatibility
//! This provides stubs for the old API that can be used when the new API is not available

use crate::error::Error;
use std::sync::{Mutex, RwLock};

// Mutex stubs
pub fn create_mutex() -> Mutex<()> {
    Mutex::new(())
}

pub fn lock_mutex(_mutex: &Mutex<()>) -> Result<(), Error> {
    Ok(())
}

pub fn unlock_mutex(_mutex: &Mutex<()>) -> Result<(), Error> {
    Ok(())
}

// RwMutex stubs
pub fn create_rwmutex() -> RwLock<()> {
    RwLock::new(())
}

pub fn rlock_rwmutex(_rwmutex: &RwLock<()>) -> Result<(), Error> {
    Ok(())
}

pub fn wlock_rwmutex(_rwmutex: &RwLock<()>) -> Result<(), Error> {
    Ok(())
}

pub fn runlock_rwmutex(_rwmutex: &RwLock<()>) -> Result<(), Error> {
    Ok(())
}

pub fn wunlock_rwmutex(_rwmutex: &RwLock<()>) -> Result<(), Error> {
    Ok(())
}

// WaitGroup stubs
pub struct BasicWaitGroup {}

pub fn create_waitgroup() -> BasicWaitGroup {
    BasicWaitGroup {}
}

pub fn add_waitgroup(_wg: &BasicWaitGroup, _delta: i32) -> Result<(), Error> {
    Ok(())
}

pub fn done_waitgroup(_wg: &BasicWaitGroup) -> Result<(), Error> {
    Ok(())
}

pub fn wait_waitgroup(_wg: &BasicWaitGroup) -> Result<(), Error> {
    Ok(())
}

// Once stubs
pub struct BasicOnce {}

pub fn create_once() -> BasicOnce {
    BasicOnce {}
}

pub fn do_once(_once: &BasicOnce, _f: fn() -> Result<(), Error>) -> Result<(), Error> {
    Ok(())
}