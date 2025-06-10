/// fr fr Constant time operations stub
use super::errors::*;

#[derive(Debug, Clone)]
pub struct ConstantTimeOps;

pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    a == b // Stub implementation
}

pub fn constant_time_select(condition: bool, a: &[u8], b: &[u8]) -> Vec<u8> {
    if condition { a.to_vec() } else { b.to_vec() }
}

pub fn constant_time_copy(src: &[u8], dst: &mut [u8]) {
    dst.copy_from_slice(src);
}

pub fn timing_safe_equal(a: &[u8], b: &[u8]) -> bool {
    constant_time_compare(a, b)
}
