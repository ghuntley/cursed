// Semantic versioning for package management
use crate::error_types::CursedError;
use std::cmp::Ordering;
use std::str::FromStr;

/// Semantic version representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build_metadata: None,
        }
    }

    pub fn with_pre_release(mut self, pre_release: String) -> Self {
        self.pre_release = Some(pre_release);
        self
    }

    pub fn with_build_metadata(mut self, build_metadata: String) -> Self {
        self.build_metadata = Some(build_metadata);
        self
    }

    pub fn is_pre_release(&self) -> bool {
        self.pre_release.is_some()
    }

    pub fn is_compatible_with(&self, other: &Version) -> bool {
        // Compatible if major version matches and this version >= other
        self.major == other.major && self >= other
    }

    pub fn bump_major(mut self) -> Self {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self.pre_release = None;
        self.build_metadata = None;
        self
    }

    pub fn bump_minor(mut self) -> Self {
        self.minor += 1;
        self.patch = 0;
        self.pre_release = None;
        self.build_metadata = None;
        self
    }

    pub fn bump_patch(mut self) -> Self {
        self.patch += 1;
        self.pre_release = None;
        self.build_metadata = None;
        self
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        
        if let Some(build) = &self.build_metadata {
            write!(f, "+{}", build)?;
        }
        
        Ok(())
    }
}

impl FromStr for Version {
    type Err = CursedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('+');
        let version_part = parts.next().unwrap_or(s);
        let build_metadata = parts.next().map(|s| s.to_string());

        let mut parts = version_part.split('-');
        let core_version = parts.next().unwrap_or(version_part);
        let pre_release = if parts.count() > 0 {
            Some(parts.collect::<Vec<_>>().join("-"))
        } else {
            None
        };

        let version_numbers: Vec<&str> = core_version.split('.').collect();
        if version_numbers.len() != 3 {
            return Err(CursedError::Parse(format!("Invalid version format: {}", s)));
        }

        let major = version_numbers[0].parse()
            .map_err(|_| CursedError::Parse(format!("Invalid major version: {}", version_numbers[0])))?;
        let minor = version_numbers[1].parse()
            .map_err(|_| CursedError::Parse(format!("Invalid minor version: {}", version_numbers[1])))?;
        let patch = version_numbers[2].parse()
            .map_err(|_| CursedError::Parse(format!("Invalid patch version: {}", version_numbers[2])))?;

        Ok(Version {
            major,
            minor,
            patch,
            pre_release,
            build_metadata,
        })
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare major.minor.patch
        match (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch)) {
            Ordering::Equal => {
                // If core versions are equal, compare pre-release
                match (&self.pre_release, &other.pre_release) {
                    (None, None) => Ordering::Equal,
                    (Some(_), None) => Ordering::Less, // Pre-release < release
                    (None, Some(_)) => Ordering::Greater, // Release > pre-release
                    (Some(a), Some(b)) => a.cmp(b),
                }
            }
            other => other,
        }
    }
}

/// Version requirement specification
#[derive(Debug, Clone, PartialEq)]
pub enum VersionReq {
    Exact(Version),
    Range { min: Version, max: Version },
    GreaterThan(Version),
    GreaterEqual(Version),
    LessThan(Version),
    LessEqual(Version),
    Compatible(Version), // Caret requirement (^1.2.3)
    Tilde(Version),      // Tilde requirement (~1.2.3)
    Wildcard(u64, Option<u64>), // Wildcard (1.*, 1.2.*)
    Any,
}

impl VersionReq {
    pub fn exact(version: Version) -> Self {
        Self::Exact(version)
    }

    pub fn compatible(version: Version) -> Self {
        Self::Compatible(version)
    }

    pub fn tilde(version: Version) -> Self {
        Self::Tilde(version)
    }

    pub fn greater_than(version: Version) -> Self {
        Self::GreaterThan(version)
    }

    pub fn greater_equal(version: Version) -> Self {
        Self::GreaterEqual(version)
    }

    pub fn matches(&self, version: &Version) -> bool {
        match self {
            VersionReq::Exact(v) => version == v,
            VersionReq::Range { min, max } => version >= min && version <= max,
            VersionReq::GreaterThan(v) => version > v,
            VersionReq::GreaterEqual(v) => version >= v,
            VersionReq::LessThan(v) => version < v,
            VersionReq::LessEqual(v) => version <= v,
            VersionReq::Compatible(v) => version.is_compatible_with(v),
            VersionReq::Tilde(v) => {
                // ~1.2.3 allows >=1.2.3 and <1.3.0
                version.major == v.major && 
                version.minor == v.minor && 
                version >= v &&
                version.minor == v.minor
            }
            VersionReq::Wildcard(major, minor) => {
                version.major == *major && 
                minor.map_or(true, |m| version.minor == m)
            }
            VersionReq::Any => true,
        }
    }
}

impl FromStr for VersionReq {
    type Err = CursedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        
        if s == "*" {
            return Ok(VersionReq::Any);
        }

        if s.starts_with('^') {
            let version = Version::from_str(&s[1..])?;
            return Ok(VersionReq::Compatible(version));
        }

        if s.starts_with('~') {
            let version = Version::from_str(&s[1..])?;
            return Ok(VersionReq::Tilde(version));
        }

        if s.starts_with(">=") {
            let version = Version::from_str(&s[2..])?;
            return Ok(VersionReq::GreaterEqual(version));
        }

        if s.starts_with('>') {
            let version = Version::from_str(&s[1..])?;
            return Ok(VersionReq::GreaterThan(version));
        }

        if s.starts_with("<=") {
            let version = Version::from_str(&s[2..])?;
            return Ok(VersionReq::LessEqual(version));
        }

        if s.starts_with('<') {
            let version = Version::from_str(&s[1..])?;
            return Ok(VersionReq::LessThan(version));
        }

        // Handle wildcards
        if s.contains('*') {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() >= 1 {
                let major = parts[0].parse()
                    .map_err(|_| CursedError::Parse(format!("Invalid major version in wildcard: {}", parts[0])))?;
                
                let minor = if parts.len() >= 2 && parts[1] != "*" {
                    Some(parts[1].parse()
                        .map_err(|_| CursedError::Parse(format!("Invalid minor version in wildcard: {}", parts[1])))?)
                } else {
                    None
                };
                
                return Ok(VersionReq::Wildcard(major, minor));
            }
        }

        // Default to exact match
        let version = Version::from_str(s)?;
        Ok(VersionReq::Exact(version))
    }
}

impl std::fmt::Display for VersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionReq::Exact(v) => write!(f, "{}", v),
            VersionReq::Range { min, max } => write!(f, ">={}, <={}", min, max),
            VersionReq::GreaterThan(v) => write!(f, ">{}", v),
            VersionReq::GreaterEqual(v) => write!(f, ">={}", v),
            VersionReq::LessThan(v) => write!(f, "<{}", v),
            VersionReq::LessEqual(v) => write!(f, "<={}", v),
            VersionReq::Compatible(v) => write!(f, "^{}", v),
            VersionReq::Tilde(v) => write!(f, "~{}", v),
            VersionReq::Wildcard(major, Some(minor)) => write!(f, "{}.{}.*", major, minor),
            VersionReq::Wildcard(major, None) => write!(f, "{}.*", major),
            VersionReq::Any => write!(f, "*"),
        }
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
        assert_eq!(v.build_metadata, None);

        let v = Version::from_str("1.2.3-alpha.1+build.123").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.pre_release, Some("alpha.1".to_string()));
        assert_eq!(v.build_metadata, Some("build.123".to_string()));
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        let v3 = Version::new(1, 3, 0);
        
        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn test_version_requirements() {
        let v = Version::new(1, 2, 3);
        
        assert!(VersionReq::Exact(v.clone()).matches(&v));
        assert!(VersionReq::GreaterEqual(Version::new(1, 2, 2)).matches(&v));
        assert!(VersionReq::LessThan(Version::new(1, 2, 4)).matches(&v));
        assert!(VersionReq::Compatible(Version::new(1, 2, 0)).matches(&v));
    }
}
