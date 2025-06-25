// use crate::stdlib::embed_that::core::{ThatFile, ThatFiles, tea, FileSystemVibe};
// use crate::stdlib::embed_that::error::{EmbedError, EmbedResult};
use std::io::{Read, Write};
use crate::error::CursedError;

/// Compression support for embedded resources
pub struct CompressionSupport;

impl CompressionSupport {
    /// Decompress a file that was compressed during embedding
    pub fn decompress_file(embedded_file: &ThatFile) -> EmbedResult<Vec<u8>> {
        let content = embedded_file.content();
        
        // Detect compression type from file extension or content
        let compression_type = detect_compression_type(&embedded_file.name(), &content)?;
        
        match compression_type {
            CompressionType::Gzip => Self::decompress_gzip(&content),
            CompressionType::Zstd => Self::decompress_zstd(&content),
            CompressionType::Brotli => Self::decompress_brotli(&content),
            CompressionType::Deflate => Self::decompress_deflate(&content),
            CompressionType::None => Ok(content),
        }
    }
    
    /// Load a compressed embedded filesystem
    pub fn load_compressed_fs(pattern: &tea) -> EmbedResult<Box<dyn FileSystemVibe>> {
        let files = super::resource_loader::load_that_pattern(pattern)?;
        let mut decompressed_files = ThatFiles::new();
        
        for file in files.list() {
            let decompressed_content = Self::decompress_file(&file)?;
            
            // Remove compression extension from filename
            let decompressed_name = remove_compression_extension(&file.name());
            let decompressed_file = ThatFile::with_metadata(
                decompressed_name,
                decompressed_content,
                file.mod_time()
            );
            
            decompressed_files.add_file(decompressed_file);
        }
        
        Ok(decompressed_files.make_fs())
    }
    
    /// Compress data for embedding (used during build process)
    pub fn compress_data(data: &[u8], compression_type: CompressionType) -> EmbedResult<Vec<u8>> {
        match compression_type {
            CompressionType::Gzip => Self::compress_gzip(data),
            CompressionType::Zstd => Self::compress_zstd(data),
            CompressionType::Brotli => Self::compress_brotli(data),
            CompressionType::Deflate => Self::compress_deflate(data),
            CompressionType::None => Ok(data.to_vec()),
        }
    }
    
    /// Compress a file for embedding
    pub fn compress_file(file: &ThatFile, compression_type: CompressionType) -> EmbedResult<ThatFile> {
        let compressed_content = Self::compress_data(&file.content(), compression_type)?;
        
        let compressed_name = match compression_type {
            CompressionType::Gzip => format!("{}.gz", file.name()),
            CompressionType::Zstd => format!("{}.zst", file.name()),
            CompressionType::Brotli => format!("{}.br", file.name()),
            CompressionType::Deflate => format!("{}.deflate", file.name()),
            CompressionType::None => file.name(),
        };
        
        Ok(ThatFile::with_metadata(
            compressed_name,
            compressed_content,
            file.mod_time()
        ))
    }
    
    /// Compress a collection of files
    pub fn compress_files(files: &ThatFiles, compression_type: CompressionType) -> EmbedResult<ThatFiles> {
        let mut compressed_files = ThatFiles::new();
        
        for file in files.list() {
            let compressed_file = Self::compress_file(&file, compression_type)?;
            compressed_files.add_file(compressed_file);
        }
        
        Ok(compressed_files)
    }
    
    /// Get compression statistics for a file
    pub fn get_compression_stats(original: &[u8], compressed: &[u8]) -> CompressionStats {
        let original_size = original.len();
        let compressed_size = compressed.len();
        let ratio = if original_size > 0 {
            compressed_size as f64 / original_size as f64
        } else {
            1.0
        };
        let savings = if original_size > compressed_size {
            original_size - compressed_size
        } else {
            0
        };
        let savings_percent = if original_size > 0 {
            (savings as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };
        
        CompressionStats {
            original_size,
            compressed_size,
            compression_ratio: ratio,
            bytes_saved: savings,
            percent_saved: savings_percent,
        }
    }
    
    /// Decompress GZIP data
    fn decompress_gzip(data: &[u8]) -> EmbedResult<Vec<u8>> {
        use flate2::read::GzDecoder;
        
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| EmbedError::DecompressionError { 
                reason: format!("GZIP decompression failed: {}", e) 
            })?;
        
        Ok(decompressed)
    }
    
    /// Compress data with GZIP
    fn compress_gzip(data: &[u8]) -> EmbedResult<Vec<u8>> {
        use flate2::{write::GzEncoder, Compression};
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)
            .map_err(|e| EmbedError::CompressionError { 
                reason: format!("GZIP compression failed: {}", e) 
            })?;
        
        encoder.finish()
            .map_err(|e| EmbedError::CompressionError { 
                reason: format!("GZIP compression finalization failed: {}", e) 
            })
    }
    
    /// Decompress Zstandard data
    fn decompress_zstd(data: &[u8]) -> EmbedResult<Vec<u8>> {
        zstd::decode_all(data)
            .map_err(|e| EmbedError::DecompressionError { 
                reason: format!("Zstandard decompression failed: {}", e) 
            })
    }
    
    /// Compress data with Zstandard
    fn compress_zstd(data: &[u8]) -> EmbedResult<Vec<u8>> {
        zstd::encode_all(data, 3) // compression level 3
            .map_err(|e| EmbedError::CompressionError { 
                reason: format!("Zstandard compression failed: {}", e) 
            })
    }
    
    /// Decompress Brotli data
    fn decompress_brotli(data: &[u8]) -> EmbedResult<Vec<u8>> {
        let mut decompressed = Vec::new();
        brotli::Decompressor::new(data, 4096).read_to_end(&mut decompressed)
            .map_err(|e| EmbedError::DecompressionError { 
                reason: format!("Brotli decompression failed: {}", e) 
            })?;
        
        Ok(decompressed)
    }
    
    /// Compress data with Brotli
    fn compress_brotli(data: &[u8]) -> EmbedResult<Vec<u8>> {
        let mut compressed = Vec::new();
        let mut compressor = brotli::CompressorWriter::new(&mut compressed, 4096, 6, 22);
        
        compressor.write_all(data)
            .map_err(|e| EmbedError::CompressionError { 
                reason: format!("Brotli compression failed: {}", e) 
            })?;
        
        compressor.flush()
            .map_err(|e| EmbedError::CompressionError { 
                reason: format!("Brotli compression flush failed: {}", e) 
            })?;
        
        drop(compressor);
        Ok(compressed)
    }
    
    /// Decompress raw deflate data
    fn decompress_deflate(data: &[u8]) -> EmbedResult<Vec<u8>> {
        use flate2::read::DeflateDecoder;
        
        let mut decoder = DeflateDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| EmbedError::DecompressionError { 
                reason: format!("Deflate decompression failed: {}", e) 
            })?;
        
        Ok(decompressed)
    }
    
    /// Compress data with raw deflate
    fn compress_deflate(data: &[u8]) -> EmbedResult<Vec<u8>> {
        use flate2::{write::DeflateEncoder, Compression};
        
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)
            .map_err(|e| EmbedError::CompressionError { 
                reason: format!("Deflate compression failed: {}", e) 
            })?;
        
        encoder.finish()
            .map_err(|e| EmbedError::CompressionError { 
                reason: format!("Deflate compression finalization failed: {}", e) 
            })
    }
    
    /// Analyze which compression method would be best for given data
    pub fn analyze_compression(data: &[u8]) -> EmbedResult<CompressionAnalysis> {
        let mut results = Vec::new();
        
        // Test each compression method
        for compression_type in [
            CompressionType::Gzip,
            CompressionType::Zstd,
            CompressionType::Brotli,
            CompressionType::Deflate,
        ] {
            match Self::compress_data(data, compression_type) {
                Ok(compressed) => {
                    let stats = Self::get_compression_stats(data, &compressed);
                    results.push(CompressionResult {
                        compression_type,
                        stats,
                        success: true,
                        error: None,
                    });
                },
                Err(e) => {
                    results.push(CompressionResult {
                        compression_type,
                        stats: CompressionStats {
                            original_size: data.len(),
                            compressed_size: data.len(),
                            compression_ratio: 1.0,
                            bytes_saved: 0,
                            percent_saved: 0.0,
                        },
                        success: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        }
        
        // Find the best compression method
        let best_method = results.iter()
            .filter(|r| r.success)
            .min_by(|a, b| a.stats.compressed_size.cmp(&b.stats.compressed_size))
            .map(|r| r.compression_type)
            .unwrap_or(CompressionType::None);
        
        Ok(CompressionAnalysis {
            original_size: data.len(),
            results,
            recommended_method: best_method,
        })
    }
}

/// Compression types supported
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Gzip,
    Zstd,
    Brotli,
    Deflate,
}

impl CompressionType {
    /// Get file extension for this compression type
    pub fn extension(&self) -> &'static str {
        match self {
            CompressionType::None => "",
            CompressionType::Gzip => ".gz",
            CompressionType::Zstd => ".zst",
            CompressionType::Brotli => ".br",
            CompressionType::Deflate => ".deflate",
        }
    }
    
    /// Get MIME type for this compression
    pub fn mime_type(&self) -> &'static str {
        match self {
            CompressionType::None => "application/octet-stream",
            CompressionType::Gzip => "application/gzip",
            CompressionType::Zstd => "application/zstd",
            CompressionType::Brotli => "application/x-brotli",
            CompressionType::Deflate => "application/deflate",
        }
    }
}

/// Compression statistics
#[derive(Debug, Clone)]
pub struct CompressionStats {
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
    pub bytes_saved: usize,
    pub percent_saved: f64,
}

/// Result of compression test
#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub compression_type: CompressionType,
    pub stats: CompressionStats,
    pub success: bool,
    pub error: Option<tea>,
}

/// Analysis of compression options
#[derive(Debug, Clone)]
pub struct CompressionAnalysis {
    pub original_size: usize,
    pub results: Vec<CompressionResult>,
    pub recommended_method: CompressionType,
}

/// Detect compression type from filename and content
fn detect_compression_type(filename: &str, content: &[u8]) -> EmbedResult<CompressionType> {
    // Check file extension first
    if filename.ends_with(".gz") {
        return Ok(CompressionType::Gzip);
    } else if filename.ends_with(".zst") {
        return Ok(CompressionType::Zstd);
    } else if filename.ends_with(".br") {
        return Ok(CompressionType::Brotli);
    } else if filename.ends_with(".deflate") {
        return Ok(CompressionType::Deflate);
    }
    
    // Check magic bytes
    if content.len() >= 2 {
        // GZIP magic bytes
        if content[0] == 0x1f && content[1] == 0x8b {
            return Ok(CompressionType::Gzip);
        }
    }
    
    if content.len() >= 4 {
        // Zstandard magic bytes
        if &content[0..4] == b"\x28\xb5\x2f\xfd" {
            return Ok(CompressionType::Zstd);
        }
    }
    
    // If no compression detected, assume no compression
    Ok(CompressionType::None)
}

/// Remove compression extension from filename
fn remove_compression_extension(filename: &str) -> tea {
    if let Some(pos) = filename.rfind('.') {
        let extension = &filename[pos..];
        match extension {
            ".gz" | ".zst" | ".br" | ".deflate" => filename[..pos].to_string(),
            _ => filename.to_string(),
        }
    } else {
        filename.to_string()
    }
}

/// Embedded file with compression support
#[derive(Debug, Clone)]
pub struct CompressedEmbeddedFile {
    pub original_file: ThatFile,
    pub compression_type: CompressionType,
    pub compression_stats: CompressionStats,
}

impl CompressedEmbeddedFile {
    /// Create a new compressed embedded file
    pub fn new(file: ThatFile, compression_type: CompressionType) -> EmbedResult<Self> {
        let compressed_file = CompressionSupport::compress_file(&file, compression_type)?;
        let stats = CompressionSupport::get_compression_stats(&file.content(), &compressed_file.content());
        
        Ok(Self {
            original_file: file,
            compression_type,
            compression_stats: stats,
        })
    }
    
    /// Get the compressed file
    pub fn compressed_file(&self) -> EmbedResult<ThatFile> {
        CompressionSupport::compress_file(&self.original_file, self.compression_type)
    }
    
    /// Get the decompressed content
    pub fn decompressed_content(&self) -> EmbedResult<Vec<u8>> {
        let compressed_file = self.compressed_file()?;
        CompressionSupport::decompress_file(&compressed_file)
    }
}

/// Public API functions for compression support
pub fn decompress_file(embedded_file: &ThatFile) -> EmbedResult<Vec<u8>> {
    CompressionSupport::decompress_file(embedded_file)
}

pub fn load_compressed_fs(pattern: &tea) -> EmbedResult<Box<dyn FileSystemVibe>> {
    CompressionSupport::load_compressed_fs(pattern)
}

pub fn compress_data(data: &[u8], compression_type: CompressionType) -> EmbedResult<Vec<u8>> {
    CompressionSupport::compress_data(data, compression_type)
}

pub fn analyze_compression(data: &[u8]) -> EmbedResult<CompressionAnalysis> {
    CompressionSupport::analyze_compression(data)
}

pub fn get_compression_stats(original: &[u8], compressed: &[u8]) -> CompressionStats {
    CompressionSupport::get_compression_stats(original, compressed)
}
