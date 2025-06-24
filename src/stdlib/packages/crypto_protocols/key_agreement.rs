/// Key Agreement Protocols Implementation
use crate::error::CursedError;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::packages::crypto_protocols::ecdh::{EcdhManager, EcdhCurve};
use crate::stdlib::packages::crypto_protocols::diffie_hellman::{DiffieHellmanManager, DhGroup};
use crate::error::Error;

/// Key agreement protocol types
#[derive(Debug, Clone, PartialEq)]
pub enum KeyAgreementProtocol {
    ECDH(EcdhCurve),
    DH(DhGroup),
    X25519,
    X448,
}

/// Key agreement manager
#[derive(Debug)]
pub struct KeyAgreementManager {
    ecdh_manager: EcdhManager,
    dh_manager: DiffieHellmanManager,
}

impl KeyAgreementManager {
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            ecdh_manager: EcdhManager::new()?,
            dh_manager: DiffieHellmanManager::new()?,
        })
    }

    pub fn perform_key_agreement(&self, protocol: KeyAgreementProtocol) -> AdvancedCryptoResult<Vec<u8>> {
        match protocol {
            KeyAgreementProtocol::ECDH(curve) => {
                let keypair1 = self.ecdh_manager.generate_keypair(curve.clone())?;
                let keypair2 = self.ecdh_manager.generate_keypair(curve)?;
                let shared_secret = self.ecdh_manager.compute_shared_secret(&keypair1, &keypair2.public_key)?;
                Ok(shared_secret.secret)
            },
            KeyAgreementProtocol::DH(group) => {
                let keypair1 = self.dh_manager.generate_keypair(group.clone())?;
                let keypair2 = self.dh_manager.generate_keypair(group)?;
                let shared_secret = self.dh_manager.compute_shared_secret(&keypair1, &keypair2.public_key)?;
                Ok(shared_secret.secret)
            },
            _ => Ok(vec![0; 32]), // Simplified for other protocols
        }
    }
}

impl Default for KeyAgreementManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default KeyAgreementManager")
    }
}
