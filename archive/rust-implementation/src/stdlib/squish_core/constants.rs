//! Constants and configuration types for SquishCore compression

/// Compression levels supported by SquishCore
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression (0)
    None,
    /// Fast compression (1)
    Fast,
    /// Default compression (6)
    Default,
    /// Best compression (9)
    Best,
    /// Custom level (0-9)
    Custom(u32),
}

impl CompressionLevel {
    /// Convert to numeric level for underlying compression libraries
    pub fn to_numeric(self) -> i32 {
        match self {
            CompressionLevel::None => 0,
            CompressionLevel::Fast => 1,
            CompressionLevel::Default => 6,
            CompressionLevel::Best => 9,
            CompressionLevel::Custom(level) => level.min(9) as i32,
        }
    }
    
    /// Create from numeric level
    pub fn from_numeric(level: i32) -> Self {
        match level {
            0 => CompressionLevel::None,
            1 => CompressionLevel::Fast,
            6 => CompressionLevel::Default,
            9 => CompressionLevel::Best,
            n if n >= 0 && n <= 9 => CompressionLevel::Custom(n as u32),
            _ => CompressionLevel::Default,
        }
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        CompressionLevel::Default
    }
}

/// Compression quality settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionQuality {
    /// Fast compression, larger size
    Fast,
    /// Balanced compression
    Balanced,
    /// Slow compression, smaller size
    Best,
}

/// Compression strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionStrategy {
    /// Default strategy
    Default,
    /// Filtered data
    Filtered,
    /// Huffman only
    HuffmanOnly,
    /// Run-length encoding
    Rle,
    /// Fixed compression
    Fixed,
}

/// Flush mode for compression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushMode {
    /// No flush
    None,
    /// Partial flush
    Partial,
    /// Sync flush
    Sync,
    /// Full flush
    Full,
    /// Finish compression
    Finish,
}

/// Minimum compression level
pub const MIN_COMPRESSION_LEVEL: i32 = 0;

/// Maximum compression level
pub const MAX_COMPRESSION_LEVEL: i32 = 9;

/// Default compression level
pub const DEFAULT_COMPRESSION_LEVEL: i32 = 6;

/// Compression window size
pub const DEFAULT_WINDOW_SIZE: u32 = 15;

/// Default memory level
pub const DEFAULT_MEMORY_LEVEL: u32 = 8;

/// Default chunk size for streaming compression
pub const DEFAULT_CHUNK_SIZE: usize = 16384;

/// Maximum chunk size
pub const MAX_CHUNK_SIZE: usize = 1024 * 1024; // 1MB
