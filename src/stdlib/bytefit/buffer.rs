use crate::error::CursedError;
/// Enhanced Buffer type for efficient byte manipulation
use super::{ByteFitError, ByteFitResult, buffer_overflow, invalid_input};
use std::sync::{Arc, Mutex};
use std::io::{self, Write, Read};

/// FitBuffer is an enhanced buffer type for efficient byte slice manipulation.
#[derive(Debug, Clone)]
pub struct FitBuffer {
#[derive(Debug)]
struct BufferInner {
    off: usize, // read offset
impl FitBuffer {
    /// Create a new FitBuffer with optional initial data
    pub fn new(buf: Option<Vec<u8>>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BufferInner {
        }
    }

    /// Returns the contents of the buffer as a byte slice
    pub fn bytes(&self) -> Vec<u8> {
        let inner = self.inner.lock().unwrap();
        inner.buf[inner.off..].to_vec()
    /// Returns the contents of the buffer as a string
    pub fn string(&self) -> String {
        String::from_utf8_lossy(&self.bytes()).to_string()
    /// Returns the number of bytes available for reading
    pub fn len(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.buf.len() - inner.off
    /// Returns the capacity of the buffer
    pub fn cap(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.buf.capacity()
    /// Truncates the buffer to n bytes
    pub fn truncate(&self, n: usize) {
        let mut inner = self.inner.lock().unwrap();
        let available = inner.buf.len() - inner.off;
        if n < available {
            inner.buf.truncate(inner.off + n);
        }
    }

    /// Resets the buffer to be empty
    pub fn reset(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.buf.clear();
        inner.off = 0;
    /// Grows the buffer's capacity by n bytes
    pub fn grow(&self, n: usize) {
        let mut inner = self.inner.lock().unwrap();
        inner.buf.reserve(n);
    /// Writes data to the buffer
    pub fn write(&self, p: &[u8]) -> ByteFitResult<usize> {
        let mut inner = self.inner.lock().unwrap();
        inner.buf.extend_from_slice(p);
        Ok(p.len())
    /// Writes a string to the buffer
    pub fn write_string(&self, s: &str) -> ByteFitResult<usize> {
        self.write(s.as_bytes())
    /// Writes a single byte to the buffer
    pub fn write_byte(&self, c: u8) -> ByteFitResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.buf.push(c);
        Ok(())
    /// Writes a rune (Unicode code point) to the buffer
    pub fn write_rune(&self, r: char) -> ByteFitResult<usize> {
        let s = r.to_string();
        self.write(s.as_bytes())
    /// Reads data from the buffer into p
    pub fn read(&self, p: &mut [u8]) -> ByteFitResult<usize> {
        let mut inner = self.inner.lock().unwrap();
        let available = &inner.buf[inner.off..];
        let n = std::cmp::min(p.len(), available.len());
        p[..n].copy_from_slice(&available[..n]);
        inner.off += n;
        Ok(n)
    /// Reads the next byte from the buffer
    pub fn read_byte(&self) -> ByteFitResult<u8> {
        let mut inner = self.inner.lock().unwrap();
        if inner.off >= inner.buf.len() {
            return Err(invalid_input("EOF"));
        }
        let byte = inner.buf[inner.off];
        inner.off += 1;
        Ok(byte)
    /// Reads the next rune from the buffer
    pub fn read_rune(&self) -> ByteFitResult<(char, usize)> {
        let bytes = self.bytes();
        if bytes.is_empty() {
            return Err(invalid_input("EOF"));
        match std::str::from_utf8(&bytes) {
            Ok(s) => {
                if let Some(c) = s.chars().next() {
                    let char_len = c.len_utf8();
                    let mut inner = self.inner.lock().unwrap();
                    inner.off += char_len;
                    Ok((c, char_len))
                } else {
                    Err(invalid_input("EOF"))
                }
            }
        }
    }

    /// Unreads the last rune
    pub fn unread_rune(&self) -> ByteFitResult<()> {
        let mut inner = self.inner.lock().unwrap();
        if inner.off == 0 {
            return Err(invalid_input("Cannot unread from empty buffer"));
        // Find the start of the last UTF-8 character
        let mut pos = inner.off - 1;
        while pos > 0 && (inner.buf[pos] & 0x80) != 0 && (inner.buf[pos] & 0x40) == 0 {
            pos -= 1;
        }
        inner.off = pos;
        Ok(())
    /// Unreads the last byte
    pub fn unread_byte(&self) -> ByteFitResult<()> {
        let mut inner = self.inner.lock().unwrap();
        if inner.off == 0 {
            return Err(invalid_input("Cannot unread from empty buffer"));
        }
        inner.off -= 1;
        Ok(())
    /// Reads bytes until delimiter is found
    pub fn read_bytes(&self, delim: u8) -> ByteFitResult<Vec<u8>> {
        let mut result = Vec::new();
        loop {
            match self.read_byte() {
                Ok(byte) => {
                    result.push(byte);
                    if byte == delim {
                        break;
                    }
                }
            }
        }
        if result.is_empty() {
            Err(invalid_input("EOF"))
        } else {
            Ok(result)
        }
    }

    /// Reads string until delimiter is found
    pub fn read_string(&self, delim: u8) -> ByteFitResult<String> {
        let bytes = self.read_bytes(delim)?;
        match String::from_utf8(bytes) {
        }
    }

    /// Returns the next n bytes without advancing the read position
    pub fn next(&self, n: usize) -> Vec<u8> {
        let inner = self.inner.lock().unwrap();
        let available = &inner.buf[inner.off..];
        let end = std::cmp::min(n, available.len());
        available[..end].to_vec()
    // Enhanced methods

    /// Appends bytes to the buffer and returns self for chaining
    pub fn append_bytes(&self, data: &[u8]) -> &Self {
        let _ = self.write(data);
        self
    /// Appends a string to the buffer and returns self for chaining
    pub fn append_string(&self, s: &str) -> &Self {
        let _ = self.write_string(s);
        self
    /// Appends a byte to the buffer and returns self for chaining
    pub fn append_byte(&self, c: u8) -> &Self {
        let _ = self.write_byte(c);
        self
    /// Appends a rune to the buffer and returns self for chaining
    pub fn append_rune(&self, r: char) -> &Self {
        let _ = self.write_rune(r);
        self
    /// Appends an integer to the buffer in the specified base
    pub fn append_int(&self, i: i64, base: u32) -> &Self {
        let s = match base {
            _ => format!("{}", i), // Default to base 10
        let _ = self.write_string(&s);
        self
    /// Appends an unsigned integer to the buffer in the specified base
    pub fn append_uint(&self, u: u64, base: u32) -> &Self {
        let s = match base {
            _ => format!("{}", u), // Default to base 10
        let _ = self.write_string(&s);
        self
    /// Appends a float to the buffer with specified format and precision
    pub fn append_float(&self, f: f64, fmt: u8, prec: i32) -> &Self {
        let s = match fmt {
        let _ = self.write_string(&s);
        self
    /// Appends a boolean to the buffer
    pub fn append_bool(&self, b: bool) -> &Self {
        let s = if b { "true" } else { "false" };
        let _ = self.write_string(s);
        self
    /// Creates a clone of the buffer
    pub fn clone_buffer(&self) -> FitBuffer {
        let inner = self.inner.lock().unwrap();
        FitBuffer::new(Some(inner.buf.clone()))
    /// Replaces occurrences of old with new in the buffer
    pub fn replace(&self, old: &[u8], new: &[u8], n: usize) -> &Self {
        let bytes = self.bytes();
        let replaced = super::transform::replace(&bytes, old, new, n);
        self.reset();
        let _ = self.write(&replaced);
        self
    /// Replaces all occurrences of old with new in the buffer
    pub fn replace_all(&self, old: &[u8], new: &[u8]) -> &Self {
        let bytes = self.bytes();
        let replaced = super::transform::replace_all(&bytes, old, new);
        self.reset();
        let _ = self.write(&replaced);
        self
    /// Trims the buffer contents using the specified cutset
    pub fn trim(&self, cutset: &str) -> ByteFitResult<&Self> {
        let bytes = self.bytes();
        let trimmed = super::trim::trim(&bytes, cutset)?;
        self.reset();
        let _ = self.write(&trimmed);
        Ok(self)
    /// Trims whitespace from the buffer contents
    pub fn trim_space(&self) -> ByteFitResult<&Self> {
        let bytes = self.bytes();
        let trimmed = super::trim::trim_space(&bytes)?;
        self.reset();
        let _ = self.write(&trimmed);
        Ok(self)
    /// Checks if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Creates a new FitBuffer with optional initial data
pub fn new_fit_buffer(buf: Option<Vec<u8>>) -> FitBuffer {
    FitBuffer::new(buf)
impl Write for FitBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.write(buf) {
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

