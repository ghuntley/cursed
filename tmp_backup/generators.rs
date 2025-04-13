//! Generators for common data types

use crate::memory::Traceable;
use crate::object::{Object, ObjectType};
use crate::prelude::*;
use crate::stdlib::quick_test::{Generator, Rand};

/// Generate 8-bit integers
pub struct Int8Generator;

impl Generator for Int8Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let range = size.min(i8::MAX as i64) as i8;
        let value = (rand.intn(range as i64) as i8).wrapping_mul(if rand.intn(2) == 0 { 1 } else { -1 });
        ObjectRef::new_int(value as i64)
    }
}

pub fn int8() -> Box<dyn Generator> {
    Box::new(Int8Generator)
}

/// Generate 8-bit integers in a specific range
pub struct Int8RangeGenerator {
    min: i8,
    max: i8,
}

impl Int8RangeGenerator {
    pub fn new(min: i8, max: i8) -> Self {
        Int8RangeGenerator { min, max }
    }
}

impl Generator for Int8RangeGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> ObjectRef {
        let range = (self.max - self.min + 1) as i64;
        let value = self.min + rand.intn(range) as i8;
        ObjectRef::new_int(value as i64)
    }
}

pub fn int8_range(min: i8, max: i8) -> Box<dyn Generator> {
    Box::new(Int8RangeGenerator::new(min, max))
}

/// Generate boolean values
pub struct BoolGenerator;

impl Generator for BoolGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> ObjectRef {
        let value = rand.intn(2) == 1;
        ObjectRef::new_bool(value)
    }
}

pub fn boolean() -> Box<dyn Generator> {
    Box::new(BoolGenerator)
}

/// Generate strings
pub struct StringGenerator {
    min_len: i64,
    max_len: i64,
}

impl StringGenerator {
    pub fn new(min_len: i64, max_len: i64) -> Self {
        StringGenerator { min_len, max_len }
    }
}

impl Generator for StringGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> ObjectRef {
        let max = self.max_len.min(size);
        let len = self.min_len + rand.intn(max - self.min_len + 1);
        
        let mut result = String::with_capacity(len as usize);
        for _ in 0..len {
            // Generate ASCII character between 32 and 126 (printable characters)
            let c = (rand.intn(95) + 32) as u8 as char;
            result.push(c);
        }
        
        ObjectRef::new_string(result)
    }
}

pub fn string() -> Box<dyn Generator> {
    Box::new(StringGenerator::new(0, 100))
}

pub fn string_of_n(min_len: i64, max_len: i64) -> Box<dyn Generator> {
    Box::new(StringGenerator::new(min_len, max_len))
}

/// Generate slices of a specific element type
pub struct SliceGenerator {
    elem_gen: Box<dyn Generator>,
    min_len: i64,
    max_len: i64,
}

impl SliceGenerator {
    pub fn new(elem_gen: Box<dyn Generator>, min_len: i64, max_len: i64) -> Self {
        SliceGenerator { elem_gen, min_len, max_len }
    }
}

impl Generator for SliceGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> ObjectRef {
        let max = self.max_len.min(size);
        let len = self.min_len + rand.intn(max - self.min_len + 1);
        
        let mut elements = Vec::with_capacity(len as usize);
        for _ in 0..len {
            elements.push(self.elem_gen.generate(rand, size / 2));
        }
        
        ObjectRef::new_array(elements)
    }
}

pub fn slice_of(elem_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(SliceGenerator::new(elem_gen, 0, 100))
}

pub fn slice_of_n(min_len: i64, max_len: i64, elem_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(SliceGenerator::new(elem_gen, min_len, max_len))
}

/// Generate one of the provided values
pub struct OneOfGenerator {
    values: Vec<ObjectRef>,
}

impl OneOfGenerator {
    pub fn new(values: Vec<ObjectRef>) -> Self {
        OneOfGenerator { values }
    }
}

impl Generator for OneOfGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> ObjectRef {
        if self.values.is_empty() {
            panic!("OneOfGenerator has no values");
        }
        
        let idx = rand.intn(self.values.len() as i64) as usize;
        self.values[idx].clone()
    }
}

impl Traceable for OneOfGenerator {
    fn trace(&self) {
        for value in &self.values {
            value.trace();
        }
    }
}

pub fn one_of(values: Vec<ObjectRef>) -> Box<dyn Generator> {
    Box::new(OneOfGenerator::new(values))
}