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
}

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
    pub level: i32,
    pub use_parallel: bool,
    pub chunk_size: usize,
}

impl CompressionConfig {
    pub fn new(level: i32, data_size: usize) -> Result<Self, String> {
        let level = validate_compression_level(level)?;
        let use_parallel = use_parallel_compression(data_size);
        let chunk_size = get_optimal_chunk_size(data_size);

        Ok(Self {
            level,
            use_parallel,
            chunk_size,
        })
    }

    pub fn from_quality(quality: f32, data_size: usize) -> Self {
        let level = convert_quality_to_level(quality);
        let use_parallel = use_parallel_compression(data_size);
        let chunk_size = get_optimal_chunk_size(data_size);

        Self {
            level,
            use_parallel,
            chunk_size,
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: 6,
            use_parallel: false,
            chunk_size: 64 * 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_compression_level() {
        assert!(validate_compression_level(5).is_ok());
        assert!(validate_compression_level(-1).is_err());
        assert!(validate_compression_level(10).is_err());
    }

    #[test]
    fn test_convert_quality_to_level() {
        assert_eq!(convert_quality_to_level(0.0), 0);
        assert_eq!(convert_quality_to_level(1.0), 9);
        assert_eq!(convert_quality_to_level(0.5), 5);
    }

    #[test]
    fn test_use_parallel_compression() {
        assert!(!use_parallel_compression(1024));
        assert!(use_parallel_compression(2 * 1024 * 1024));
    }
}
