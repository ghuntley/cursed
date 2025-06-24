/// Certificate Templates

use crate::stdlib::packages::crypto_pki::types::{PkiResult, PkiError};
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct CertificateTemplate {
    pub subject: String,
    pub subject_alt_names: Vec<String>,
    pub key_usage: Vec<String>,
    pub extended_key_usage: Vec<String>,
    pub validity_days: u32,
}

impl CertificateTemplate {
    pub fn new(subject: String) -> Self {
        Self {
            subject,
            subject_alt_names: Vec::new(),
            key_usage: Vec::new(),
            extended_key_usage: Vec::new(),
            validity_days: 365,
        }
    }
}

// Additional types
pub type ServerTemplate = CertificateTemplate;
pub type ClientTemplate = CertificateTemplate;
pub type CaTemplate = CertificateTemplate;
pub type CodeSigningTemplate = CertificateTemplate;
pub type EmailTemplate = CertificateTemplate;
pub type TemplateError = PkiError;
pub type TemplateResult<T> = PkiResult<T>;

pub fn create_server_template(hostname: &str) -> TemplateResult<ServerTemplate> {
    let mut template = CertificateTemplate::new(format!("CN={}", hostname));
    template.subject_alt_names.push(hostname.to_string());
    template.key_usage.push("digitalSignature".to_string());
    template.key_usage.push("keyEncipherment".to_string());
    template.extended_key_usage.push("serverAuth".to_string());
    Ok(template)
}

pub fn create_client_template(subject: &str) -> TemplateResult<ClientTemplate> {
    let mut template = CertificateTemplate::new(subject.to_string());
    template.key_usage.push("digitalSignature".to_string());
    template.extended_key_usage.push("clientAuth".to_string());
    Ok(template)
}

pub fn create_ca_template(subject: &str) -> TemplateResult<CaTemplate> {
    let mut template = CertificateTemplate::new(subject.to_string());
    template.key_usage.push("keyCertSign".to_string());
    template.key_usage.push("cRLSign".to_string());
    template.validity_days = 3650; // 10 years for CA
    Ok(template)
}
