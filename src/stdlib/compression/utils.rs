// Compression utility functions

/// Validate compression level
pub fn validate_compression_level(level: i32) -> Result<i32, String> {
    if level < 0 || level > 9 {
        Err(format!("Invalid compression level: {}. Must be 0-9", level))
    } else {
        Ok(level)
    }
}

/// Convert quality to compression level
pub fn convert_quality_to_level(quality: f32) -> i32 {
    if quality <= 0.0 {
        0
    } else if quality >= 1.0 {
        9
    } else {
        (quality * 9.0).round() as i32
    }
}

/// Determine if parallel compression should be used
pub fn use_parallel_compression(data_size: usize) -> bool {
    // Use parallel compression for data larger than 1MB
    data_size > 1024 * 1024
/// Get optimal chunk size for compression
pub fn get_optimal_chunk_size(data_size: usize) -> usize {
    if data_size < 1024 {
        data_size
    } else if data_size < 1024 * 1024 {
        64 * 1024  // 64KB chunks
    } else {
        1024 * 1024  // 1MB chunks
    }
}

/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
impl CompressionConfig {
    pub fn new(level: i32, data_size: usize) -> Result<Self, String> {
        let level = validate_compression_level(level)?;
        let use_parallel = use_parallel_compression(data_size);
        let chunk_size = get_optimal_chunk_size(data_size);

        Ok(Self {
        })
    pub fn from_quality(quality: f32, data_size: usize) -> Self {
        let level = convert_quality_to_level(quality);
        let use_parallel = use_parallel_compression(data_size);
        let chunk_size = get_optimal_chunk_size(data_size);

        Self {
        }
    }
impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
        }
    }
