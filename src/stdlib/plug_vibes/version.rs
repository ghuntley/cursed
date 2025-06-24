use crate::error::Error;
/// Plugin versioning system for compatibility checking
use std::fmt;
use std::str::FromStr;
use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};

/// Semantic version structure for plugin compatibility
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
}

impl Version {
    /// Create a new version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
        }
    }

    /// Create a new version with pre-release tag
    pub fn new_with_prerelease(major: u32, minor: u32, patch: u32, pre_release: String) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: Some(pre_release),
        }
    }

    /// Check if this version is compatible with another version
    /// Compatible means same major version and this version >= other version
    pub fn compatible(&self, other: &Version) -> bool {
        // Same major version required for compatibility
        if self.major != other.major {
            return false;
        }

        // If major versions match, check if this version is >= other
        match self.minor.cmp(&other.minor) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => {
                match self.patch.cmp(&other.patch) {
                    std::cmp::Ordering::Greater => true,
                    std::cmp::Ordering::Less => false,
                    std::cmp::Ordering::Equal => {
                        // If both have pre-release, compare them
                        // If only one has pre-release, the one without is considered newer
                        match (&self.pre_release, &other.pre_release) {
                            (None, None) => true,
                            (Some(_), None) => false,
                            (None, Some(_)) => true,
                            (Some(a), Some(b)) => a >= b,
                        }
                    }
                }
            }
        }
    }

    /// Check if this version is greater than another
    pub fn greater_than(&self, other: &Version) -> bool {
        self > other
    }

    /// Check if this version is less than another
    pub fn less_than(&self, other: &Version) -> bool {
        self < other
    }

    /// Check if this version is equal to another
    pub fn equal(&self, other: &Version) -> bool {
        self == other
    }

    /// Check if this version is a breaking change from another
    pub fn is_breaking_change(&self, other: &Version) -> bool {
        self.major > other.major
    }

    /// Check if this version is a feature addition from another
    pub fn is_feature_addition(&self, other: &Version) -> bool {
        self.major == other.major && self.minor > other.minor
    }

    /// Check if this version is a patch from another
    pub fn is_patch(&self, other: &Version) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch > other.patch
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.pre_release {
            Some(pre) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, pre),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

impl FromStr for Version {
    type Err = PluginError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_version(s)
    }
}

/// Parse a version string into a Version struct
pub fn parse_version(version_str: &str) -> PluginResult<Version> {
    let (version_part, pre_release) = if let Some(dash_idx) = version_str.find('-') {
        let (v, p) = version_str.split_at(dash_idx);
        (v, Some(p[1..].to_string()))
    } else {
        (version_str, None)
    };

    let parts: Vec<&str> = version_part.split('.').collect();
    if parts.len() != 3 {
        return Err(PluginError::general(&format!(
            "Invalid version format '{}': expected major.minor.patch",
            version_str
        )));
    }

    let major = parts[0].parse::<u32>().map_err(|_| {
        PluginError::general(&format!("Invalid major version '{}': must be a number", parts[0]))
    })?;

    let minor = parts[1].parse::<u32>().map_err(|_| {
        PluginError::general(&format!("Invalid minor version '{}': must be a number", parts[1]))
    })?;

    let patch = parts[2].parse::<u32>().map_err(|_| {
        PluginError::general(&format!("Invalid patch version '{}': must be a number", parts[2]))
    })?;

    Ok(Version {
        major,
        minor,
        patch,
        pre_release,
    })
}

/// Version constraint for plugin dependencies
#[derive(Debug, Clone, PartialEq)]
pub enum VersionConstraint {
    /// Exact version match
    Exact(Version),
    /// Minimum version (inclusive)
    AtLeast(Version),
    /// Maximum version (inclusive)
    AtMost(Version),
    /// Version range (inclusive)
    Range(Version, Version),
    /// Compatible version (same major, >= minor.patch)
    Compatible(Version),
    /// Any version
    Any,
}

impl VersionConstraint {
    /// Check if a version satisfies this constraint
    pub fn satisfies(&self, version: &Version) -> bool {
        match self {
            VersionConstraint::Exact(v) => version == v,
            VersionConstraint::AtLeast(v) => version >= v,
            VersionConstraint::AtMost(v) => version <= v,
            VersionConstraint::Range(min, max) => version >= min && version <= max,
            VersionConstraint::Compatible(v) => version.compatible(v),
            VersionConstraint::Any => true,
        }
    }
}

impl fmt::Display for VersionConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionConstraint::Exact(v) => write!(f, "={}", v),
            VersionConstraint::AtLeast(v) => write!(f, ">={}", v),
            VersionConstraint::AtMost(v) => write!(f, "<={}", v),
            VersionConstraint::Range(min, max) => write!(f, "{} - {}", min, max),
            VersionConstraint::Compatible(v) => write!(f, "~{}", v),
            VersionConstraint::Any => write!(f, "*"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let v = parse_version("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.pre_release, None);
    }

    #[test]
    fn test_parse_version_with_prerelease() {
        let v = parse_version("1.2.3-alpha").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.pre_release, Some("alpha".to_string()));
    }

    #[test]
    fn test_version_compatibility() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 1, 0);
        let v3 = Version::new(2, 0, 0);

        assert!(v1.compatible(&v2));
        assert!(!v1.compatible(&v3));
        assert!(!v2.compatible(&v1));
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        let v3 = Version::new(1, 2, 3);

        assert!(v2.greater_than(&v1));
        assert!(v1.less_than(&v2));
        assert!(v1.equal(&v3));
    }

    #[test]
    fn test_version_constraint_satisfies() {
        let v1 = Version::new(1, 2, 3);
        
        assert!(VersionConstraint::Exact(v1.clone()).satisfies(&v1));
        assert!(VersionConstraint::AtLeast(Version::new(1, 0, 0)).satisfies(&v1));
        assert!(VersionConstraint::AtMost(Version::new(2, 0, 0)).satisfies(&v1));
        assert!(VersionConstraint::Compatible(Version::new(1, 1, 0)).satisfies(&v1));
        assert!(VersionConstraint::Any.satisfies(&v1));
    }
}
