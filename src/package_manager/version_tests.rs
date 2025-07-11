//! Tests for version requirement parsing

use crate::package_manager::version::{Version, VersionReq};
use crate::error::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_req_parsing() -> Result<()> {
        // Test the problematic case from the resolver test
        let req = VersionReq::parse(">=1.0.0, <2.0.0")?;
        
        // Should parse as a range
        if let VersionReq::Range(min, max) = req {
            assert_eq!(min, Version::new(1, 0, 0));
            assert_eq!(max, Version::new(2, 0, 0));
        } else {
            panic!("Expected Range variant, got {:?}", req);
        }
        
        // Test that it matches versions correctly
        let req = VersionReq::parse(">=1.0.0, <2.0.0")?;
        assert!(req.matches(&Version::new(1, 0, 0))); // Should match min
        assert!(req.matches(&Version::new(1, 5, 0))); // Should match middle
        assert!(!req.matches(&Version::new(2, 0, 0))); // Should not match max (exclusive)
        assert!(!req.matches(&Version::new(0, 9, 0))); // Should not match below min
        
        Ok(())
    }

    #[test]
    fn test_other_version_formats() -> Result<()> {
        // Test caret versions
        let req = VersionReq::parse("^1.0.0")?;
        assert!(req.matches(&Version::new(1, 0, 0)));
        assert!(req.matches(&Version::new(1, 5, 0)));
        assert!(!req.matches(&Version::new(2, 0, 0)));
        
        // Test tilde versions  
        let req = VersionReq::parse("~1.5.0")?;
        assert!(req.matches(&Version::new(1, 5, 0)));
        assert!(req.matches(&Version::new(1, 5, 9)));
        assert!(!req.matches(&Version::new(1, 6, 0)));
        
        // Test exact versions
        let req = VersionReq::parse("1.0.0")?;
        assert!(req.matches(&Version::new(1, 0, 0)));
        assert!(!req.matches(&Version::new(1, 0, 1)));
        
        Ok(())
    }

    #[test]
    fn test_invalid_comma_formats() {
        // Test unsupported comma formats
        let result = VersionReq::parse(">=1.0.0, <=2.0.0");
        assert!(result.is_err());
        
        let result = VersionReq::parse(">1.0.0, <2.0.0");
        assert!(result.is_err());
        
        let result = VersionReq::parse(">=1.0.0, <2.0.0, >0.5.0");
        assert!(result.is_err());
    }
}
