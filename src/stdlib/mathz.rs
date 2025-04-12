//! Mathematical functions and constants for CURSED programs
//!
//! The mathz package provides mathematical operations, functions, and constants
//! similar to Go's math package. It includes common operations like absolute value,
//! rounding, exponentiation, trigonometric functions, and mathematical constants.
//!
//! Key functions include:
//!
//! - Basic operations: `abs`, `max`, `min`
//! - Rounding: `ceil`, `floor`, `round`
//! - Powers: `pow`, `sqrt`
//!
//! Constants:
//! - `PI`: The mathematical constant π (3.141592...)
//! - `E`: The mathematical constant e (2.718281...)

use std::rc::Rc;
use crate::object::Object;
use crate::error::Error;

// Constants
pub const PI: f64 = std::f64::consts::PI;
pub const E: f64 = std::f64::consts::E;

/// Returns the absolute value of a number
///
/// This function computes the absolute value of the input number,
/// returning the magnitude regardless of sign.
///
/// # Arguments
///
/// * `args` - A slice with one Object reference (Integer or Float)
///
/// # Returns
///
/// Result<Rc<Object>, Error> - The absolute value result or an error
pub fn abs(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("abs requires 1 argument".to_string()));
    }
    
    match &*args[0] {
        Object::Integer(i) => Ok(Rc::new(Object::Integer(i.abs()))),
        Object::Float(f) => Ok(Rc::new(Object::Float(f.abs()))),
        _ => Err(Error::Runtime("Argument to abs must be a number".to_string())),
    }
}

/// Ceiling function (smallest integer >= x)
pub fn ceil(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("ceil requires 1 argument".to_string()));
    }
    
    match &*args[0] {
        Object::Integer(i) => Ok(Rc::new(Object::Integer(*i))),
        Object::Float(f) => Ok(Rc::new(Object::Float(f.ceil()))),
        _ => Err(Error::Runtime("Argument to ceil must be a number".to_string())),
    }
}

/// Floor function (largest integer <= x)
pub fn floor(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("floor requires 1 argument".to_string()));
    }
    
    match &*args[0] {
        Object::Integer(i) => Ok(Rc::new(Object::Integer(*i))),
        Object::Float(f) => Ok(Rc::new(Object::Float(f.floor()))),
        _ => Err(Error::Runtime("Argument to floor must be a number".to_string())),
    }
}

/// Maximum of x and y
pub fn max(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("max requires 2 arguments".to_string()));
    }
    
    match (&*args[0], &*args[1]) {
        (Object::Integer(x), Object::Integer(y)) => Ok(Rc::new(Object::Integer(std::cmp::max(*x, *y)))),
        (Object::Float(x), Object::Float(y)) => Ok(Rc::new(Object::Float(x.max(*y)))),
        (Object::Integer(x), Object::Float(y)) => Ok(Rc::new(Object::Float((*x as f64).max(*y)))),
        (Object::Float(x), Object::Integer(y)) => Ok(Rc::new(Object::Float(x.max(*y as f64)))),
        _ => Err(Error::Runtime("Arguments to max must be numbers".to_string())),
    }
}

/// Minimum of x and y
pub fn min(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("min requires 2 arguments".to_string()));
    }
    
    match (&*args[0], &*args[1]) {
        (Object::Integer(x), Object::Integer(y)) => Ok(Rc::new(Object::Integer(std::cmp::min(*x, *y)))),
        (Object::Float(x), Object::Float(y)) => Ok(Rc::new(Object::Float(x.min(*y)))),
        (Object::Integer(x), Object::Float(y)) => Ok(Rc::new(Object::Float((*x as f64).min(*y)))),
        (Object::Float(x), Object::Integer(y)) => Ok(Rc::new(Object::Float(x.min(*y as f64)))),
        _ => Err(Error::Runtime("Arguments to min must be numbers".to_string())),
    }
}

/// x^y
pub fn pow(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("pow requires 2 arguments".to_string()));
    }
    
    let x = match &*args[0] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("First argument to pow must be a number".to_string())),
    };
    
    let y = match &*args[1] {
        Object::Integer(i) => *i as f64,
        Object::Float(f) => *f,
        _ => return Err(Error::Runtime("Second argument to pow must be a number".to_string())),
    };
    
    // Check if we can return an integer
    let result = x.powf(y);
    if result.fract() == 0.0 && result >= (i64::MIN as f64) && result <= (i64::MAX as f64) {
        Ok(Rc::new(Object::Integer(result as i64)))
    } else {
        Ok(Rc::new(Object::Float(result)))
    }
}

/// Computes the square root of a non-negative number
///
/// This function calculates the square root of the input number. The input
/// must be non-negative, or an error will be returned. The result will be
/// an integer if the square root is an exact integer, otherwise a float.
///
/// # Arguments
///
/// * `args` - A slice with one Object reference (non-negative Integer or Float)
///
/// # Returns
///
/// Result<Rc<Object>, Error> - The square root result or an error
pub fn sqrt(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("sqrt requires 1 argument".to_string()));
    }
    
    let x = match &*args[0] {
        Object::Integer(i) => {
            if *i < 0 {
                return Err(Error::Runtime("Cannot take square root of negative number".to_string()));
            }
            *i as f64
        },
        Object::Float(f) => {
            if *f < 0.0 {
                return Err(Error::Runtime("Cannot take square root of negative number".to_string()));
            }
            *f
        },
        _ => return Err(Error::Runtime("Argument to sqrt must be a number".to_string())),
    };
    
    let result = x.sqrt();
    if result.fract() == 0.0 {
        Ok(Rc::new(Object::Integer(result as i64)))
    } else {
        Ok(Rc::new(Object::Float(result)))
    }
}

/// Round to nearest integer
pub fn round(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("round requires 1 argument".to_string()));
    }
    
    match &*args[0] {
        Object::Integer(i) => Ok(Rc::new(Object::Integer(*i))),
        Object::Float(f) => Ok(Rc::new(Object::Float(f.round()))),
        _ => Err(Error::Runtime("Argument to round must be a number".to_string())),
    }
}