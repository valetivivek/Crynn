// TLS/CA verification and security utilities for Crynn Browser
// Ensures secure connections with proper certificate validation

use anyhow::{Result, Context};
use std::sync::Arc;
use rustls::{ClientConfig, RootCertStore, OwnedTrustAnchor};
use rustls_pemfile::certs;
use std::io::BufReader;

/// Create a secure TLS client configuration with CA verification
/// Uses system CA store plus webpki trust roots
pub fn create_secure_client_config() -> Result<ClientConfig> {
    let mut root_store = RootCertStore::empty();
    
    // Add webpki trust roots (Mozilla CA bundle)
    // webpki_roots provides Mozilla's CA certificate store
    for anchor in webpki_roots::TLS_SERVER_ROOTS.iter() {
        root_store.add_trust_anchor(OwnedTrustAnchor::from_subject_spki_name_constraints(
            anchor.subject,
            anchor.spki,
            anchor.name_constraints,
        ));
    }
    
    // Try to load system CA certificates (optional, platform-dependent)
    #[cfg(unix)]
    {
        // Try loading from common CA certificate paths
        let ca_paths = vec![
            "/etc/ssl/certs/ca-certificates.crt",
            "/etc/ssl/certs/ca-bundle.crt",
            "/etc/pki/tls/certs/ca-bundle.crt",
            "/usr/local/share/certs/ca-root-nss.crt",
            "/etc/ssl/cert.pem",
        ];
        
        for path in ca_paths {
            if let Ok(file) = std::fs::File::open(path) {
                let mut reader = BufReader::new(file);
                let certs = certs(&mut reader)?;
                
                for cert in certs {
                    let _ = root_store.add(&rustls::Certificate(cert));
                }
                
                // Break after first successful load
                break;
            }
        }
    }
    
    #[cfg(windows)]
    {
        // Windows: Use system certificate store via schannel (rustls doesn't directly support this)
        // For now, rely on webpki trust roots
        // TODO: Integrate with native-tls or schannel for Windows CA store
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: Try loading from keychain
        // For now, rely on webpki trust roots
        // TODO: Integrate with Security Framework for macOS CA store
    }
    
    // Create client config with secure defaults
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    
    Ok(config)
}

/// Verify certificate against pinned certificates (optional)
pub struct CertificatePinner {
    pinned_certs: Vec<Vec<u8>>,
}

impl CertificatePinner {
    pub fn new(pinned_certs: Vec<Vec<u8>>) -> Self {
        Self { pinned_certs }
    }
    
    pub fn verify(&self, cert_chain: &[rustls::Certificate]) -> Result<bool> {
        if self.pinned_certs.is_empty() {
            return Ok(true); // No pinning configured
        }
        
        // Check if any certificate in chain matches pinned certs
        for cert in cert_chain {
            let cert_bytes = cert.0.as_slice();
            if self.pinned_certs.iter().any(|pinned| pinned == cert_bytes) {
                return Ok(true);
            }
        }
        
        Err(anyhow::anyhow!("Certificate pinning failed: no matching certificate"))
    }
}

/// Security policy for network requests
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub require_tls: bool,
    pub block_mixed_content: bool,
    pub enforce_hsts: bool,
    pub certificate_pinning: Option<CertificatePinner>,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            require_tls: true,
            block_mixed_content: true,
            enforce_hsts: true,
            certificate_pinning: None,
        }
    }
}

/// Check if a URL meets security requirements
pub fn check_url_security(url: &url::Url, policy: &SecurityPolicy) -> Result<()> {
    match url.scheme() {
        "https" => {
            // HTTPS is allowed
            Ok(())
        },
        "http" => {
            if policy.require_tls {
                return Err(anyhow::anyhow!("HTTP is blocked by security policy. Use HTTPS."));
            }
            Ok(())
        },
        "data" | "about" => {
            // Allow data and about URLs
            Ok(())
        },
        _ => {
            Err(anyhow::anyhow!("Unsupported or insecure URL scheme: {}", url.scheme()))
        }
    }
}

/// Check for mixed content (HTTP resources on HTTPS pages)
pub fn check_mixed_content(page_url: &url::Url, resource_url: &url::Url) -> Result<()> {
    if page_url.scheme() == "https" && resource_url.scheme() == "http" {
        return Err(anyhow::anyhow!(
            "Mixed content blocked: HTTP resource on HTTPS page"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_check_url_security_https() {
        let url = url::Url::parse("https://example.com").unwrap();
        let policy = SecurityPolicy::default();
        assert!(check_url_security(&url, &policy).is_ok());
    }
    
    #[test]
    fn test_check_url_security_http_blocked() {
        let url = url::Url::parse("http://example.com").unwrap();
        let policy = SecurityPolicy::default();
        assert!(check_url_security(&url, &policy).is_err());
    }
    
    #[test]
    fn test_check_mixed_content() {
        let https_page = url::Url::parse("https://example.com").unwrap();
        let http_resource = url::Url::parse("http://example.com/script.js").unwrap();
        assert!(check_mixed_content(&https_page, &http_resource).is_err());
    }
}

