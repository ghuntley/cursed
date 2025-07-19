//! Version management for package manager
//!
//! This module provides semantic versioning support for package management

use crate::error::CursedError;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

/// Semantic version representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
    pub build: Option<String>,
}

/// Version requirement specification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionReq {
    Exact(Version),
    Range(Version, Version),
    Caret(Version),   // ^1.2.3 - compatible within major version
    Tilde(Version),   // ~1.2.3 - compatible within minor version
    Wildcard(u32, Option<u32>), // 1.* or 1.2.*
    Any,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }

    /// Parse version from string
    pub fn parse(version_str: &str) -> crate::error::Result<Self> {
        // Use FromStr implementation which handles pre-release/build metadata
        version_str.parse().map_err(Into::into)
    }

    pub fn with_pre_release(mut self, pre_release: String) -> Self {
        self.pre_release = Some(pre_release);
        self
    }

    pub fn with_build(mut self, build: String) -> Self {
        self.build = Some(build);
        self
    }

    pub fn is_compatible_with(&self, other: &Version) -> bool {
        self.major == other.major && 
        (self.minor > other.minor || 
         (self.minor == other.minor && self.patch >= other.patch))
    }

    pub fn is_pre_release(&self) -> bool {
        self.pre_release.is_some()
    }
}

impl FromStr for Version {
    type Err = CursedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() < 3 {
            return Err(CursedError::General(format!("Invalid version format: {}", s)));
        }

        let major = parts[0].parse::<u32>()
            .map_err(|_| CursedError::General(format!("Invalid major version: {}", parts[0])))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| CursedError::General(format!("Invalid minor version: {}", parts[1])))?;
        
        // Handle patch version with potential pre-release/build info
        let patch_part = parts[2];
        let (patch_str, pre_release, build) = if patch_part.contains('-') {
            let mut split = patch_part.split('-');
            let patch = split.next().unwrap();
            let pre_release = split.collect::<Vec<_>>().join("-");
            (patch, Some(pre_release), None)
        } else if patch_part.contains('+') {
            let mut split = patch_part.split('+');
            let patch = split.next().unwrap();
            let build = split.collect::<Vec<_>>().join("+");
            (patch, None, Some(build))
        } else {
            (patch_part, None, None)
        };

        let patch = patch_str.parse::<u32>()
            .map_err(|_| CursedError::General(format!("Invalid patch version: {}", patch_str)))?;

        Ok(Version {
            major,
            minor,
            patch,
            pre_release,
            build,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        if let Some(build) = &self.build {
            write!(f, "+{}", build)?;
        }
        Ok(())
    }
}

impl fmt::Display for VersionReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionReq::Exact(v) => write!(f, "={}", v),
            VersionReq::Range(start, end) => write!(f, ">={}, <{}", start, end),
            VersionReq::Caret(v) => write!(f, "^{}", v),
            VersionReq::Tilde(v) => write!(f, "~{}", v),
            VersionReq::Wildcard(major, Some(minor)) => write!(f, "{}.{}.*", major, minor),
            VersionReq::Wildcard(major, None) => write!(f, "{}.*", major),
            VersionReq::Any => write!(f, "*"),
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => match self.patch.cmp(&other.patch) {
                    Ordering::Equal => {
                        // Pre-release versions have lower precedence than normal versions
                        match (&self.pre_release, &other.pre_release) {
                            (None, None) => Ordering::Equal,
                            (Some(_), None) => Ordering::Less,
                            (None, Some(_)) => Ordering::Greater,
                            (Some(a), Some(b)) => a.cmp(b),
                        }
                    }
                    ord => ord,
                }
                ord => ord,
            }
            ord => ord,
        }
    }
}

impl VersionReq {
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            VersionReq::Exact(v) => version == v,
            VersionReq::Range(min, max) => version >= min && version < max,
            VersionReq::Caret(v) => {
                version.major == v.major && version >= v
            }
            VersionReq::Tilde(v) => {
                version.major == v.major && 
                version.minor == v.minor && 
                version >= v
            }
            VersionReq::Wildcard(major, minor) => {
                version.major == *major && 
                minor.map_or(true, |m| version.minor == m)
            }
            VersionReq::Any => true,
        }
    }

    pub fn parse(s: &str) -> Result<Self, CursedError> {
        if s == "*" {
            return Ok(VersionReq::Any);
        }

        // Handle single comparison operators like >=1.0.0, >1.0.0, <=1.0.0, <1.0.0
        if s.starts_with(">=") {
            let version_str = s.strip_prefix(">=").unwrap().trim();
            let version = Version::from_str(version_str)?;
            // Use a range from this version to a very high version
            let max_version = Version::new(999, 999, 999);
            return Ok(VersionReq::Range(version, max_version));
        }

        if s.starts_with("<=") {
            let version_str = s.strip_prefix("<=").unwrap().trim();
            let version = Version::from_str(version_str)?;
            // Use a range from 0.0.0 to this version (inclusive)
            let min_version = Version::new(0, 0, 0);
            let mut max_version = version.clone();
            // Increment patch for exclusive upper bound
            max_version.patch += 1;
            return Ok(VersionReq::Range(min_version, max_version));
        }

        if s.starts_with('>') && !s.starts_with(">=") {
            let version_str = s.strip_prefix('>').unwrap().trim();
            let mut version = Version::from_str(version_str)?;
            // Start from the next patch version
            version.patch += 1;
            let max_version = Version::new(999, 999, 999);
            return Ok(VersionReq::Range(version, max_version));
        }

        if s.starts_with('<') && !s.starts_with("<=") {
            let version_str = s.strip_prefix('<').unwrap().trim();
            let version = Version::from_str(version_str)?;
            let min_version = Version::new(0, 0, 0);
            return Ok(VersionReq::Range(min_version, version));
        }

        // Handle comma-separated constraints like ">=1.0.0, <2.0.0"
        if s.contains(',') {
            let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
            if parts.len() == 2 {
                let first = parts[0].trim();
                let second = parts[1].trim();
                
                // Parse range constraints (>=min, <max)
                if first.starts_with(">=") && second.starts_with('<') {
                    let min_str = first.strip_prefix(">=").unwrap().trim();
                    let max_str = second.strip_prefix('<').unwrap().trim();
                    
                    let min = Version::from_str(min_str)?;
                    let max = Version::from_str(max_str)?;
                    
                    return Ok(VersionReq::Range(min, max));
                }
            }
            return Err(CursedError::General("Unsupported comma-separated version constraint format".to_string()));
        }

        if s.starts_with('^') {
            let version = Version::from_str(&s[1..])?;
            return Ok(VersionReq::Caret(version));
        }

        if s.starts_with('~') {
            let version = Version::from_str(&s[1..])?;
            return Ok(VersionReq::Tilde(version));
        }

        if s.contains("..") {
            let parts: Vec<&str> = s.split("..").collect();
            if parts.len() != 2 {
                return Err(CursedError::General("Invalid range format".to_string()));
            }
            let min = Version::from_str(parts[0])?;
            let max = Version::from_str(parts[1])?;
            return Ok(VersionReq::Range(min, max));
        }

        if s.ends_with(".*") {
            let parts: Vec<&str> = s.trim_end_matches(".*").split('.').collect();
            if parts.len() == 1 {
                let major = parts[0].parse::<u32>()
                    .map_err(|_| CursedError::General("Invalid major version".to_string()))?;
                return Ok(VersionReq::Wildcard(major, None));
            } else if parts.len() == 2 {
                let major = parts[0].parse::<u32>()
                    .map_err(|_| CursedError::General("Invalid major version".to_string()))?;
                let minor = parts[1].parse::<u32>()
                    .map_err(|_| CursedError::General("Invalid minor version".to_string()))?;
                return Ok(VersionReq::Wildcard(major, Some(minor)));
            }
        }

        // Default to exact match
        let version = Version::from_str(s)?;
        Ok(VersionReq::Exact(version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let v = Version::from_str("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.pre_release, None);
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::from_str("1.2.3").unwrap();
        let v2 = Version::from_str("1.2.4").unwrap();
        assert!(v1 < v2);
    }

    #[test]
    fn test_version_requirements() {
        let v = Version::from_str("1.2.3").unwrap();
        let req = VersionReq::parse("^1.2.0").unwrap();
        assert!(req.matches(&v));
    }
}
