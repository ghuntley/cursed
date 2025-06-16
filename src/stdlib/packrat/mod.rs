// PackRat (archive packages)
// Provides access to file archiving and compression formats

pub mod tar;
pub mod zip;
pub mod compression;
pub mod error;

// Re-export main types for convenience
pub use tar::{RatPack, RatStash, RatHeader, Format};
pub use zip::{HoardPack, HoardStash, HoardFile, HoardFileHeader};
pub use compression::{IsZip, IsTar, Compress, Decompress};
pub use error::{ArchiveError, ArchiveResult};

// Constants for format detection
pub const TAR_MAGIC: &[u8] = b"ustar";
pub const ZIP_SIGNATURE: u32 = 0x04034b50;

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;
    
    #[test]
    fn test_module_exports() {
        info!("Testing PackRat module exports");
        
        // Test that all main types are accessible
        let _format = Format::FormatPOSIX;
        info!("All PackRat module exports are accessible");
    }
}
