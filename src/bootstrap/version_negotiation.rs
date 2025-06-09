//! # Version Negotiation System
//!
//! This module handles version detection and feature negotiation between
//! different compiler stages in the CURSED bootstrap process. It ensures
//! compatibility and enables communication between compiler generations.

use std::collections::HashMap;
use std::fmt;
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};
use crate::bootstrap::feature_detection::{BootstrapStage, CompilerVersion, CompilerFeature, FeatureSupport};

/// Negotiation protocol versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ProtocolVersion {
    /// Version 1.0 - Basic negotiation
    V1_0,
    /// Version 1.1 - Enhanced feature detection
    V1_1,
    /// Version 2.0 - Advanced capabilities
    V2_0,
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolVersion::V1_0 => write!(f, "1.0"),
            ProtocolVersion::V1_1 => write!(f, "1.1"),
            ProtocolVersion::V2_0 => write!(f, "2.0"),
        }
    }
}

/// Capability advertisement from a compiler stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAdvertisement {
    pub compiler_version: CompilerVersion,
    pub protocol_version: ProtocolVersion,
    pub supported_features: HashMap<CompilerFeature, FeatureSupport>,
    pub custom_capabilities: HashMap<String, String>,
    pub compatibility_matrix: Vec<CompilerVersion>,
}

/// Negotiation request from one compiler to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationRequest {
    pub requester_version: CompilerVersion,
    pub protocol_version: ProtocolVersion,
    pub required_features: Vec<CompilerFeature>,
    pub preferred_features: Vec<CompilerFeature>,
    pub fallback_acceptable: bool,
}

/// Negotiation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationResponse {
    pub responder_version: CompilerVersion,
    pub protocol_version: ProtocolVersion,
    pub negotiation_result: NegotiationResult,
    pub agreed_features: HashMap<CompilerFeature, FeatureSupport>,
    pub unsupported_features: Vec<CompilerFeature>,
    pub fallback_options: Vec<FallbackOption>,
}

/// Result of version negotiation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NegotiationResult {
    /// Full compatibility achieved
    FullCompatibility,
    /// Partial compatibility with limitations
    PartialCompatibility(Vec<String>),
    /// Incompatible but fallbacks available
    IncompatibleWithFallbacks,
    /// Complete incompatibility
    Incompatible(String),
}

/// Fallback option for unsupported features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackOption {
    pub original_feature: CompilerFeature,
    pub fallback_feature: Option<CompilerFeature>,
    pub degradation_level: DegradationLevel,
    pub description: String,
}

/// Level of feature degradation in fallback
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DegradationLevel {
    /// No degradation, full functionality
    None,
    /// Minor limitations
    Minor,
    /// Significant limitations
    Major,
    /// Feature completely disabled
    Disabled,
}

/// Version negotiation system
#[derive(Debug)]
pub struct VersionNegotiator {
    current_version: CompilerVersion,
    supported_protocols: Vec<ProtocolVersion>,
    capability_cache: HashMap<CompilerVersion, CapabilityAdvertisement>,
    negotiation_history: Vec<NegotiationSession>,
}

/// Record of a negotiation session
#[derive(Debug, Clone)]
pub struct NegotiationSession {
    pub timestamp: std::time::SystemTime,
    pub peer_version: CompilerVersion,
    pub request: NegotiationRequest,
    pub response: NegotiationResponse,
    pub duration: std::time::Duration,
}

impl VersionNegotiator {
    /// Create a new version negotiator
    #[instrument(level = "debug")]
    pub fn new(current_version: CompilerVersion) -> Self {
        let version_clone = current_version.clone();
        let supported_protocols = match current_version.stage {
            BootstrapStage::Stage0 => vec![ProtocolVersion::V1_0],
            BootstrapStage::Stage1 => vec![ProtocolVersion::V1_0, ProtocolVersion::V1_1],
            BootstrapStage::Stage2 | BootstrapStage::Development => {
                vec![ProtocolVersion::V1_0, ProtocolVersion::V1_1, ProtocolVersion::V2_0]
            }
        };

        let protocols_clone = supported_protocols.clone();
        let negotiator = Self {
            current_version,
            supported_protocols,
            capability_cache: HashMap::new(),
            negotiation_history: Vec::new(),
        };

        info!(version = %version_clone, protocols = ?protocols_clone, "Version negotiator initialized");
        negotiator
    }

    /// Create capability advertisement for this compiler
    #[instrument(level = "debug", skip(self))]
    pub fn create_capability_advertisement(&self, features: HashMap<CompilerFeature, FeatureSupport>) -> CapabilityAdvertisement {
        let protocol_version = self.get_latest_supported_protocol();
        let compatibility_matrix = self.get_compatible_versions();

        CapabilityAdvertisement {
            compiler_version: self.current_version.clone(),
            protocol_version,
            supported_features: features,
            custom_capabilities: self.get_custom_capabilities(),
            compatibility_matrix,
        }
    }

    /// Get the latest supported protocol version
    fn get_latest_supported_protocol(&self) -> ProtocolVersion {
        *self.supported_protocols.iter().max().unwrap_or(&ProtocolVersion::V1_0)
    }

    /// Get list of compatible compiler versions
    fn get_compatible_versions(&self) -> Vec<CompilerVersion> {
        let mut compatible = Vec::new();
        
        // Same major version is generally compatible
        for minor in 0..=self.current_version.minor {
            for patch in 0..=if minor == self.current_version.minor { self.current_version.patch } else { 10 } {
                compatible.push(CompilerVersion {
                    major: self.current_version.major,
                    minor,
                    patch,
                    stage: self.current_version.stage,
                    commit_hash: None,
                    build_timestamp: None,
                });
            }
        }
        
        compatible
    }

    /// Get custom capabilities specific to this compiler
    fn get_custom_capabilities(&self) -> HashMap<String, String> {
        let mut capabilities = HashMap::new();
        
        capabilities.insert("bootstrap_stage".to_string(), format!("{:?}", self.current_version.stage));
        capabilities.insert("llvm_backend".to_string(), "yes".to_string());
        capabilities.insert("jit_support".to_string(), "yes".to_string());
        
        match self.current_version.stage {
            BootstrapStage::Stage0 => {
                capabilities.insert("rust_interop".to_string(), "full".to_string());
                capabilities.insert("native_compilation".to_string(), "yes".to_string());
            },
            BootstrapStage::Stage1 | BootstrapStage::Stage2 => {
                capabilities.insert("self_hosted".to_string(), "yes".to_string());
                capabilities.insert("bootstrapped".to_string(), "yes".to_string());
            },
            BootstrapStage::Development => {
                capabilities.insert("experimental_features".to_string(), "yes".to_string());
                capabilities.insert("debug_info".to_string(), "enhanced".to_string());
            },
        }
        
        capabilities
    }

    /// Negotiate with another compiler version
    #[instrument(level = "info", skip(self))]
    pub fn negotiate(&mut self, request: NegotiationRequest, peer_capabilities: CapabilityAdvertisement) -> NegotiationResponse {
        let start_time = std::time::Instant::now();
        
        debug!(peer_version = %peer_capabilities.compiler_version, "Starting version negotiation");
        
        // Check protocol compatibility
        let negotiated_protocol = self.negotiate_protocol(&request, &peer_capabilities);
        if negotiated_protocol.is_none() {
            return NegotiationResponse {
                responder_version: self.current_version.clone(),
                protocol_version: self.get_latest_supported_protocol(),
                negotiation_result: NegotiationResult::Incompatible("No compatible protocol version".to_string()),
                agreed_features: HashMap::new(),
                unsupported_features: request.required_features.clone(),
                fallback_options: Vec::new(),
            };
        }
        
        let protocol = negotiated_protocol.unwrap();
        
        // Analyze feature compatibility
        let (agreed_features, unsupported_features, fallback_options) = 
            self.analyze_feature_compatibility(&request, &peer_capabilities);
        
        // Determine overall negotiation result
        let negotiation_result = self.determine_negotiation_result(
            &request, 
            &unsupported_features, 
            &fallback_options
        );
        
        let response = NegotiationResponse {
            responder_version: self.current_version.clone(),
            protocol_version: protocol,
            negotiation_result,
            agreed_features,
            unsupported_features,
            fallback_options,
        };
        
        // Record negotiation session
        let duration = start_time.elapsed();
        let session = NegotiationSession {
            timestamp: std::time::SystemTime::now(),
            peer_version: peer_capabilities.compiler_version.clone(),
            request: request.clone(),
            response: response.clone(),
            duration,
        };
        
        self.negotiation_history.push(session);
        self.capability_cache.insert(peer_capabilities.compiler_version.clone(), peer_capabilities);
        
        info!(result = ?response.negotiation_result, duration_ms = duration.as_millis(), "Negotiation completed");
        response
    }

    /// Negotiate protocol version
    fn negotiate_protocol(&self, request: &NegotiationRequest, peer_capabilities: &CapabilityAdvertisement) -> Option<ProtocolVersion> {
        // Find highest mutually supported protocol
        let mut best_protocol = None;
        
        for &our_protocol in &self.supported_protocols {
            if our_protocol == peer_capabilities.protocol_version || our_protocol == request.protocol_version {
                best_protocol = Some(our_protocol);
                break;
            }
        }
        
        // Fallback to lowest common denominator
        if best_protocol.is_none() {
            for &our_protocol in &self.supported_protocols {
                if our_protocol <= peer_capabilities.protocol_version {
                    best_protocol = Some(our_protocol);
                }
            }
        }
        
        best_protocol
    }

    /// Analyze feature compatibility between versions
    fn analyze_feature_compatibility(
        &self, 
        request: &NegotiationRequest, 
        peer_capabilities: &CapabilityAdvertisement
    ) -> (HashMap<CompilerFeature, FeatureSupport>, Vec<CompilerFeature>, Vec<FallbackOption>) {
        let mut agreed_features = HashMap::new();
        let mut unsupported_features = Vec::new();
        let mut fallback_options = Vec::new();
        
        // Check required features
        for feature in &request.required_features {
            if let Some(support_level) = peer_capabilities.supported_features.get(feature) {
                if *support_level != FeatureSupport::Unsupported {
                    agreed_features.insert(feature.clone(), support_level.clone());
                } else {
                    unsupported_features.push(feature.clone());
                    if let Some(fallback) = self.create_fallback_option(feature) {
                        fallback_options.push(fallback);
                    }
                }
            } else {
                unsupported_features.push(feature.clone());
                if let Some(fallback) = self.create_fallback_option(feature) {
                    fallback_options.push(fallback);
                }
            }
        }
        
        // Check preferred features (best effort)
        for feature in &request.preferred_features {
            if let Some(support_level) = peer_capabilities.supported_features.get(feature) {
                if *support_level != FeatureSupport::Unsupported {
                    agreed_features.insert(feature.clone(), support_level.clone());
                }
            }
        }
        
        (agreed_features, unsupported_features, fallback_options)
    }

    /// Create fallback option for unsupported feature
    fn create_fallback_option(&self, feature: &CompilerFeature) -> Option<FallbackOption> {
        use CompilerFeature::*;
        
        match feature {
            Goroutines => Some(FallbackOption {
                original_feature: feature.clone(),
                fallback_feature: None,
                degradation_level: DegradationLevel::Major,
                description: "Use sequential execution instead of goroutines".to_string(),
            }),
            Channels => Some(FallbackOption {
                original_feature: feature.clone(),
                fallback_feature: None,
                degradation_level: DegradationLevel::Major,
                description: "Use direct function calls instead of channels".to_string(),
            }),
            OptimizedCodegen => Some(FallbackOption {
                original_feature: feature.clone(),
                fallback_feature: Some(LlvmCodegen),
                degradation_level: DegradationLevel::Minor,
                description: "Use unoptimized LLVM code generation".to_string(),
            }),
            AdvancedTypes => Some(FallbackOption {
                original_feature: feature.clone(),
                fallback_feature: Some(BasicTypes),
                degradation_level: DegradationLevel::Major,
                description: "Use basic types instead of advanced type features".to_string(),
            }),
            JitCompilation => Some(FallbackOption {
                original_feature: feature.clone(),
                fallback_feature: None,
                degradation_level: DegradationLevel::Disabled,
                description: "JIT compilation not available, use ahead-of-time compilation".to_string(),
            }),
            _ => None,
        }
    }

    /// Determine overall negotiation result
    fn determine_negotiation_result(
        &self,
        request: &NegotiationRequest,
        unsupported_features: &[CompilerFeature],
        fallback_options: &[FallbackOption],
    ) -> NegotiationResult {
        if unsupported_features.is_empty() {
            NegotiationResult::FullCompatibility
        } else if request.fallback_acceptable && !fallback_options.is_empty() {
            let limitations: Vec<String> = fallback_options
                .iter()
                .map(|f| format!("{}: {}", f.original_feature, f.description))
                .collect();
            
            if fallback_options.iter().any(|f| f.degradation_level == DegradationLevel::Disabled) {
                NegotiationResult::IncompatibleWithFallbacks
            } else {
                NegotiationResult::PartialCompatibility(limitations)
            }
        } else {
            let missing_features: Vec<String> = unsupported_features
                .iter()
                .map(|f| format!("{}", f))
                .collect();
            NegotiationResult::Incompatible(format!("Missing required features: {}", missing_features.join(", ")))
        }
    }

    /// Check if a specific version is compatible
    #[instrument(level = "trace", skip(self))]
    pub fn is_version_compatible(&self, other_version: &CompilerVersion) -> bool {
        // Same major version is compatible
        if self.current_version.major != other_version.major {
            return false;
        }
        
        // Check stage compatibility
        match (&self.current_version.stage, &other_version.stage) {
            // Stage 0 can work with any stage
            (BootstrapStage::Stage0, _) | (_, BootstrapStage::Stage0) => true,
            // Same stages are compatible
            (a, b) if a == b => true,
            // Adjacent stages are generally compatible
            (BootstrapStage::Stage1, BootstrapStage::Stage2) | 
            (BootstrapStage::Stage2, BootstrapStage::Stage1) => true,
            // Development stage is compatible with all
            (BootstrapStage::Development, _) | (_, BootstrapStage::Development) => true,
            _ => false,
        }
    }

    /// Get negotiation statistics
    pub fn get_negotiation_stats(&self) -> NegotiationStats {
        let total_sessions = self.negotiation_history.len();
        let successful_sessions = self.negotiation_history.iter()
            .filter(|s| matches!(s.response.negotiation_result, 
                NegotiationResult::FullCompatibility | 
                NegotiationResult::PartialCompatibility(_)))
            .count();
        
        let average_duration = if total_sessions > 0 {
            let total_duration: std::time::Duration = self.negotiation_history.iter()
                .map(|s| s.duration)
                .sum();
            total_duration / total_sessions as u32
        } else {
            std::time::Duration::from_millis(0)
        };
        
        NegotiationStats {
            total_sessions,
            successful_sessions,
            failure_rate: if total_sessions > 0 { 
                (total_sessions - successful_sessions) as f64 / total_sessions as f64 
            } else { 
                0.0 
            },
            average_duration,
            cached_capabilities: self.capability_cache.len(),
        }
    }

    /// Clear old negotiation history to save memory
    pub fn cleanup_old_sessions(&mut self, keep_last_n: usize) {
        if self.negotiation_history.len() > keep_last_n {
            self.negotiation_history.drain(0..self.negotiation_history.len() - keep_last_n);
        }
    }
}

/// Statistics about version negotiation sessions
#[derive(Debug, Clone)]
pub struct NegotiationStats {
    pub total_sessions: usize,
    pub successful_sessions: usize,
    pub failure_rate: f64,
    pub average_duration: std::time::Duration,
    pub cached_capabilities: usize,
}

impl fmt::Display for NegotiationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Version Negotiation Statistics:")?;
        writeln!(f, "  Total Sessions: {}", self.total_sessions)?;
        writeln!(f, "  Successful: {}", self.successful_sessions)?;
        writeln!(f, "  Failure Rate: {:.1}%", self.failure_rate * 100.0)?;
        writeln!(f, "  Average Duration: {}ms", self.average_duration.as_millis())?;
        writeln!(f, "  Cached Capabilities: {}", self.cached_capabilities)?;
        Ok(())
    }
}

/// Convenience function to create a negotiation request
pub fn create_negotiation_request(
    requester_version: CompilerVersion,
    required_features: Vec<CompilerFeature>,
    preferred_features: Vec<CompilerFeature>,
) -> NegotiationRequest {
    NegotiationRequest {
        requester_version,
        protocol_version: ProtocolVersion::V2_0, // Use latest by default
        required_features,
        preferred_features,
        fallback_acceptable: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::feature_detection::BootstrapStage;

    #[test]
    fn test_version_negotiator_creation() {
        let version = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let negotiator = VersionNegotiator::new(version);
        assert!(!negotiator.supported_protocols.is_empty());
        assert_eq!(negotiator.current_version.stage, BootstrapStage::Stage1);
    }

    #[test]
    fn test_protocol_support_by_stage() {
        let stage0_version = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage: BootstrapStage::Stage0,
            commit_hash: None, build_timestamp: None,
        };
        let stage0_negotiator = VersionNegotiator::new(stage0_version);
        assert_eq!(stage0_negotiator.supported_protocols, vec![ProtocolVersion::V1_0]);

        let stage2_version = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage: BootstrapStage::Stage2,
            commit_hash: None, build_timestamp: None,
        };
        let stage2_negotiator = VersionNegotiator::new(stage2_version);
        assert!(stage2_negotiator.supported_protocols.contains(&ProtocolVersion::V2_0));
    }

    #[test]
    fn test_version_compatibility() {
        let version1 = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None, build_timestamp: None,
        };
        let negotiator = VersionNegotiator::new(version1);

        let compatible_version = CompilerVersion {
            major: 0, minor: 1, patch: 1,
            stage: BootstrapStage::Stage2,
            commit_hash: None, build_timestamp: None,
        };
        assert!(negotiator.is_version_compatible(&compatible_version));

        let incompatible_version = CompilerVersion {
            major: 1, minor: 0, patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None, build_timestamp: None,
        };
        assert!(!negotiator.is_version_compatible(&incompatible_version));
    }

    #[test]
    fn test_capability_advertisement_creation() {
        let version = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None, build_timestamp: None,
        };
        let negotiator = VersionNegotiator::new(version);

        let mut features = HashMap::new();
        features.insert(CompilerFeature::BasicTypes, FeatureSupport::Stable);
        features.insert(CompilerFeature::Goroutines, FeatureSupport::Experimental);

        let advertisement = negotiator.create_capability_advertisement(features);
        assert_eq!(advertisement.compiler_version.stage, BootstrapStage::Stage1);
        assert!(!advertisement.custom_capabilities.is_empty());
    }

    #[test]
    fn test_fallback_option_creation() {
        let version = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None, build_timestamp: None,
        };
        let negotiator = VersionNegotiator::new(version);

        let fallback = negotiator.create_fallback_option(&CompilerFeature::Goroutines);
        assert!(fallback.is_some());
        
        let fallback = fallback.unwrap();
        assert_eq!(fallback.original_feature, CompilerFeature::Goroutines);
        assert_eq!(fallback.degradation_level, DegradationLevel::Major);
    }

    #[test]
    fn test_negotiation_request_creation() {
        let version = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None, build_timestamp: None,
        };

        let request = create_negotiation_request(
            version,
            vec![CompilerFeature::BasicTypes, CompilerFeature::LlvmCodegen],
            vec![CompilerFeature::Goroutines],
        );

        assert_eq!(request.required_features.len(), 2);
        assert_eq!(request.preferred_features.len(), 1);
        assert!(request.fallback_acceptable);
    }
}
