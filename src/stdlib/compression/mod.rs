// Compression module for CURSED stdlib

pub mod utils;

pub use utils::{
    validate_compression_level, convert_quality_to_level, 
    use_parallel_compression, get_optimal_chunk_size,
    CompressionConfig
};
