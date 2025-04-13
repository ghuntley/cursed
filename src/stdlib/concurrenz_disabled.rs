//! Temporarily disabled synchronization primitives for concurrent CURSED programs
//! This is a temporary version to allow compilation while we fix weak references.

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;

/// Dummy new_mutex function that always returns null
pub fn new_mutex(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy mutex_lock function that always returns null
pub fn mutex_lock(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy mutex_unlock function that always returns null
pub fn mutex_unlock(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy new_rwmutex function that always returns null
pub fn new_rwmutex(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy rwmutex_rlock function that always returns null
pub fn rwmutex_rlock(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy rwmutex_runlock function that always returns null
pub fn rwmutex_runlock(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy rwmutex_lock function that always returns null
pub fn rwmutex_lock(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy rwmutex_unlock function that always returns null
pub fn rwmutex_unlock(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy new_waitgroup function that always returns null
pub fn new_waitgroup(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy waitgroup_add function that always returns null
pub fn waitgroup_add(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy waitgroup_done function that always returns null
pub fn waitgroup_done(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy waitgroup_wait function that always returns null
pub fn waitgroup_wait(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy new_once function that always returns null
pub fn new_once(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}

/// Dummy once_do function that always returns null
pub fn once_do(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    Ok(Rc::new(Object::Null))
}