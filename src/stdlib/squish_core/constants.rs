/// Compression levels and constants for squish_core module

/// No compression - data is stored as-is
pub const NO_COMPRESSION: i32 = 0;

/// Fast compression with minimal CPU usage
pub const BEST_SPEED: i32 = 1;

/// Maximum compression with high CPU usage
pub const BEST_COMPRESSION: i32 = 9;

/// Default compression level balancing speed and size
pub const DEFAULT_COMPRESSION: i32 = -1;

/// Huffman-only compression (no LZ77)
pub const HUFFMAN_ONLY: i32 = -2;

/// Compression level range constants
pub const MIN_COMPRESSION_LEVEL: i32 = 0;
pub const MAX_COMPRESSION_LEVEL: i32 = 9;

/// Default buffer sizes for various operations
pub const DEFAULT_BUFFER_SIZE: usize = 32 * 1024; // 32KB
pub const MIN_BUFFER_SIZE: usize = 1024; // 1KB
pub const MAX_BUFFER_SIZE: usize = 1024 * 1024; // 1MB

/// Dictionary size constants
pub const DEFAULT_DICTIONARY_SIZE: usize = 32 * 1024; // 32KB
pub const MIN_DICTIONARY_SIZE: usize = 256; // 256 bytes
pub const MAX_DICTIONARY_SIZE: usize = 64 * 1024; // 64KB

/// Window size constants for compression algorithms
pub const DEFAULT_WINDOW_SIZE: i32 = 15; // 32KB window
pub const MIN_WINDOW_SIZE: i32 = 9;     // 512 byte window
pub const MAX_WINDOW_SIZE: i32 = 15;    // 32KB window

/// Memory level constants (for zlib/gzip)
pub const DEFAULT_MEM_LEVEL: i32 = 8;
pub const MIN_MEM_LEVEL: i32 = 1;
pub const MAX_MEM_LEVEL: i32 = 9;

/// LZW constants
pub const LZW_MIN_LITERAL_WIDTH: u8 = 2;
pub const LZW_MAX_LITERAL_WIDTH: u8 = 8;
pub const LZW_DEFAULT_LITERAL_WIDTH: u8 = 8;

/// Magic numbers for format detection
pub const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];
pub const ZLIB_MAGIC: [u8; 2] = [0x78, 0x9c]; // Default compression
pub const BZIP2_MAGIC: [u8; 3] = [0x42, 0x5a, 0x68]; // "BZh"

/// File extensions for different compression formats
pub const GZIP_EXTENSION: &str = ".gz";
pub const ZLIB_EXTENSION: &str = ".zlib";
pub const BZIP2_EXTENSION: &str = ".bz2";
pub const LZW_EXTENSION: &str = ".Z";
pub const DEFLATE_EXTENSION: &str = ".deflate";

/// MIME types for compression formats
pub const GZIP_MIME_TYPE: &str = "application/gzip";
pub const ZLIB_MIME_TYPE: &str = "application/zlib";
pub const BZIP2_MIME_TYPE: &str = "application/x-bzip2";
pub const DEFLATE_MIME_TYPE: &str = "application/deflate";

/// Performance tuning constants
pub const PARALLEL_THRESHOLD: usize = 1024 * 1024; // 1MB - use parallel compression above this size
pub const CHUNK_SIZE_PARALLEL: usize = 64 * 1024;  // 64KB chunks for parallel processing
pub const MAX_PARALLEL_WORKERS: usize = 16;        // Maximum number of parallel workers

/// Adaptive compression thresholds
pub const ADAPTIVE_SAMPLE_SIZE: usize = 8 * 1024;  // 8KB sample for format detection
pub const COMPRESSION_RATIO_THRESHOLD: f64 = 0.9;  // Switch algorithms if ratio > 0.9
pub const SPEED_THRESHOLD_MS: u64 = 1000;          // 1 second speed threshold

/// Quality levels for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionQuality {
    /// Fastest compression, larger size
    Fastest,
    /// Fast compression, good for real-time applications
    Fast,
    /// Balanced compression speed and size
    Balanced,
    /// Good compression, slower speed
    Good,
    /// Best compression, slowest speed
    Best,
}

impl CompressionQuality {
    /// Convert quality level to numeric compression level
    pub fn to_level(self) -> i32 {
        match self {
            CompressionQuality::Fastest => 1,
            CompressionQuality::Fast => 3,
            CompressionQuality::Balanced => 6,
            CompressionQuality::Good => 8,
            CompressionQuality::Best => 9,
        }
    }
    
    /// Get all quality levels
    pub fn all() -> &'static [CompressionQuality] {
        &[
            CompressionQuality::Fastest,
            CompressionQuality::Fast,
            CompressionQuality::Balanced,
            CompressionQuality::Good,
            CompressionQuality::Best,
        ]
    }
}

impl Default for CompressionQuality {
    fn default() -> Self {
        CompressionQuality::Balanced
    }
}

/// Strategy hint for compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionStrategy {
    /// Default strategy suitable for most data
    Default,
    /// Strategy for data with mostly small values
    Filtered,
    /// Strategy for Huffman-only compression
    HuffmanOnly,
    /// Strategy for run-length encoding
    Rle,
    /// Strategy optimized for specific data patterns
    Fixed,
}

impl CompressionStrategy {
    /// Convert strategy to numeric value for libraries that need it
    pub fn to_value(self) -> i32 {
        match self {
            CompressionStrategy::Default => 0,
            CompressionStrategy::Filtered => 1,
            CompressionStrategy::HuffmanOnly => 2,
            CompressionStrategy::Rle => 3,
            CompressionStrategy::Fixed => 4,
        }
    }
}

impl Default for CompressionStrategy {
    fn default() -> Self {
        CompressionStrategy::Default
    }
}

/// Flush modes for compression streams
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushMode {
    /// No flush - accumulate data
    None,
    /// Partial flush - flush pending output
    Partial,
    /// Sync flush - flush all pending output and align to byte boundary
    Sync,
    /// Full flush - like sync flush but reset compression state
    Full,
    /// Finish - flush all data and complete compression
    Finish,
}

impl FlushMode {
    /// Convert flush mode to numeric value for libraries that need it
    pub fn to_value(self) -> i32 {
        match self {
            FlushMode::None => 0,
            FlushMode::Partial => 1,
            FlushMode::Sync => 2,
            FlushMode::Full => 3,
            FlushMode::Finish => 4,
        }
    }
}

impl Default for FlushMode {
    fn default() -> Self {
        FlushMode::None
    }
}

/// Validation function for compression levels
pub fn is_valid_compression_level(level: i32) -> bool {
    level == DEFAULT_COMPRESSION || 
    level == HUFFMAN_ONLY || 
    (level >= MIN_COMPRESSION_LEVEL && level <= MAX_COMPRESSION_LEVEL)
}

/// Get recommended compression level for given quality
pub fn quality_to_level(quality: CompressionQuality) -> i32 {
    quality.to_level()
}

/// Get recommended buffer size for given input size
pub fn recommended_buffer_size(input_size: usize) -> usize {
    if input_size < MIN_BUFFER_SIZE {
        MIN_BUFFER_SIZE
    } else if input_size > MAX_BUFFER_SIZE {
        MAX_BUFFER_SIZE
    } else {
        // Use power of 2 buffer size
        let mut size = MIN_BUFFER_SIZE;
        while size < input_size && size < MAX_BUFFER_SIZE {
            size *= 2;
        }
        size
    }
}

/// Check if parallel compression should be used
pub fn should_use_parallel(input_size: usize) -> bool {
    input_size >= PARALLEL_THRESHOLD
}

/// Calculate optimal chunk size for parallel compression
pub fn optimal_chunk_size(input_size: usize, num_workers: usize) -> usize {
    if num_workers == 0 {
        return CHUNK_SIZE_PARALLEL;
    }
    
    let base_chunk_size = input_size / num_workers;
    let chunk_size = base_chunk_size.max(CHUNK_SIZE_PARALLEL);
    
    // Round up to nearest power of 2 for better cache performance
    let mut optimal_size = 1;
    while optimal_size < chunk_size {
        optimal_size *= 2;
    }
    
    optimal_size.min(MAX_BUFFER_SIZE)
}
