/// Certificate Extensions

// use crate::stdlib::packages::crypto_pki::types::{PkiResult, PkiError};
use crate::error::CursedError;

pub struct Extension {
// Additional types
pub type ExtensionValue = Vec<u8>;
pub type ExtensionOid = String;
pub type ExtensionCriticality = bool;
pub type BasicConstraints = Extension;
pub type KeyUsage = Extension;
pub type ExtendedKeyUsage = Extension;
pub type SubjectAlternativeName = Extension;
pub type AuthorityKeyIdentifier = Extension;
pub type SubjectKeyIdentifier = Extension;
pub type CrlDistributionPoints = Extension;
pub type AuthorityInformationAccess = Extension;
pub type ExtensionError = PkiError;
pub type ExtensionResult<T> = PkiResult<T>;
pub type ExtensionBuilder = Extension;
pub type ExtensionValidator = Extension;

pub fn create_extension(oid: String, critical: bool, value: Vec<u8>) -> ExtensionResult<Extension> {
    Ok(Extension { oid, critical, value })
pub fn parse_extension(_data: &[u8]) -> ExtensionResult<Extension> {
    create_extension("2.5.29.1".to_string(), false, vec![0])
}
