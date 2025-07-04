use std::fmt;

/// FitBuffer - A dynamic buffer for byte operations
#[derive(Debug, Clone)]
pub struct FitBuffer {
    data: Vec<u8>,
    pos: usize,
}

impl FitBuffer {
    /// Create a new buffer with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            pos: 0,
        }
    }
    
    /// Create a new buffer with initial data
    pub fn with_data(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: 0,
        }
    }
    
    /// Append a string to the buffer
    pub fn append_string(&mut self, s: &str) -> &mut Self {
        self.data.extend_from_slice(s.as_bytes());
        self
    }
    
    /// Append an integer to the buffer
    pub fn append_int(&mut self, n: i64, base: u32) -> &mut Self {
        let s = match base {
            2 => format!("{:b}", n),
            8 => format!("{:o}", n),
            16 => format!("{:x}", n),
            _ => n.to_string(),
        };
        self.append_string(&s)
    }
    
    /// Append a boolean to the buffer
    pub fn append_bool(&mut self, b: bool) -> &mut Self {
        self.append_string(if b { "true" } else { "false" })
    }
    
    /// Append a float to the buffer
    pub fn append_float(&mut self, f: f64, fmt: char, precision: usize) -> &mut Self {
        let s = match fmt {
            'e' => format!("{:.prec$e}", f, prec = precision),
            'E' => format!("{:.prec$E}", f, prec = precision),
            'f' => format!("{:.prec$}", f, prec = precision),
            'g' => format!("{:.prec$}", f, prec = precision),
            _ => f.to_string(),
        };
        self.append_string(&s)
    }
    
    /// Get the buffer contents as a string
    pub fn string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
    
    /// Get the buffer length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get the buffer capacity
    pub fn cap(&self) -> usize {
        self.data.capacity()
    }
    
    /// Clone the buffer
    pub fn clone_buffer(&self) -> Self {
        self.clone()
    }
    
    /// Read a byte from the current position
    pub fn read_byte(&mut self) -> Result<u8, String> {
        if self.pos >= self.data.len() {
            return Err("EOF".to_string());
        }
        let byte = self.data[self.pos];
        self.pos += 1;
        Ok(byte)
    }
    
    /// Replace all occurrences in the buffer
    pub fn replace_all(&mut self, old: &[u8], new: &[u8]) -> &mut Self {
        let new_data = crate::stdlib::bytefit::replace_all(&self.data, old, new);
        self.data = new_data;
        self
    }
    
    /// Trim whitespace from the buffer
    pub fn trim_space(&mut self) -> &mut Self {
        let trimmed = String::from_utf8_lossy(&self.data).trim().as_bytes().to_vec();
        self.data = trimmed;
        self
    }
    
    /// Get the underlying data as bytes
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
    
    /// Reset the read position
    pub fn reset(&mut self) {
        self.pos = 0;
    }
}

impl fmt::Display for FitBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

/// Create a new fit buffer with optional capacity
pub fn new_fit_buffer(capacity: Option<usize>) -> FitBuffer {
    match capacity {
        Some(cap) => FitBuffer::new(cap),
        None => FitBuffer::new(0),
    }
}

/// Create a new fit buffer with optional initial data
pub fn new_fit_buffer_with_data(data: Option<Vec<u8>>) -> FitBuffer {
    match data {
        Some(d) => FitBuffer::with_data(d),
        None => FitBuffer::new(0),
    }
}
