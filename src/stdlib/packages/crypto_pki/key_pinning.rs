/// Public Key Pinning

use crate::stdlib::packages::crypto_pki::types::{PkiResult, PkiError};
use crate::stdlib::packages::crypto_pki::certificate::Certificate;

pub struct PinSet;
pub struct PublicKeyPin;
pub struct PinValidation;
pub struct PinPolicy;
pub type PinError = PkiError;
pub type PinResult<T> = PkiResult<T>;

pub fn create_pin_set() -> PinResult<PinSet> {
    Ok(PinSet)
}

pub fn add_pin_from_certificate(_pin_set: &mut PinSet, _cert: &Certificate) -> PinResult<()> {
    Ok(())
}

pub fn verify_pin(_cert: &Certificate, _pin_set: &PinSet) -> PinResult<bool> {
    Ok(true)
}
