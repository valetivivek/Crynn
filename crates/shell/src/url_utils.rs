// URL validation and normalization utilities for Crynn Browser
// Implements RFC 3986 URL normalization and security checks

use url::Url;
use anyhow::{Result, Context};

/// Normalize and validate a URL string
/// Handles protocol prefixes, IDN, and security checks
pub fn normalize_url(input: &str) -> Result<String> {
    let trimmed = input.trim();
    
    // Handle empty input
    if trimmed.is_empty() {
        return Err(anyhow::anyhow!("Empty URL"));
    }
    
    // Check for about: URLs (browser-internal)
    if trimmed.starts_with("about:") {
        return Ok(trimmed.to_string());
    }
    
    // Check if it already looks like a URL
    let mut url_string = trimmed.to_string();
    
    // Add protocol if missing
    if !url_string.contains("://") {
        // Check if it's a search query (no dots, spaces, or protocol-like strings)
        if !url_string.contains('.') && (url_string.contains(' ') || !is_domain_like(&url_string)) {
            // Treat as search query - redirect to search engine
            // This will be handled by the caller
            return Err(anyhow::anyhow!("Search query detected"));
        }
        
        // Assume HTTPS for domains
        url_string = format!("https://{}", url_string);
    }
    
    // Parse URL
    let url = Url::parse(&url_string)
        .context(format!("Failed to parse URL: {}", url_string))?;
    
    // Security: Block dangerous protocols
    match url.scheme() {
        "http" | "https" | "about" | "data" => {},
        "javascript" => {
            return Err(anyhow::anyhow!("javascript: URLs are blocked for security"));
        },
        "file" => {
            return Err(anyhow::anyhow!("file: URLs are blocked for security"));
        },
        scheme => {
            return Err(anyhow::anyhow!("Unsupported URL scheme: {}", scheme));
        }
    }
    
    // Normalize: build normalized URL
    let scheme = url.scheme().to_lowercase();
    let host = url.host_str().map(|h| h.to_lowercase()).unwrap_or_default();
    
    // Remove default ports
    let port = url.port().and_then(|p| {
        match (scheme.as_str(), p) {
            ("http", 80) | ("https", 443) => None,
            _ => Some(p),
        }
    });
    
    // Build path (remove trailing slashes except root)
    let mut path = url.path().to_string();
    if path.len() > 1 && path.ends_with('/') {
        path = path.trim_end_matches('/').to_string();
    }
    
    // Build query string
    let query = url.query();
    
    // Reconstruct URL (fragments are preserved but can be removed if needed)
    let normalized_str = if let Some(p) = port {
        if let Some(q) = query {
            format!("{}://{}:{}{}?{}", scheme, host, p, path, q)
        } else {
            format!("{}://{}:{}{}", scheme, host, p, path)
        }
    } else {
        if let Some(q) = query {
            format!("{}://{}{}?{}", scheme, host, path, q)
        } else {
            format!("{}://{}{}", scheme, host, path)
        }
    };
    
    // Re-parse to ensure validity
    let normalized = Url::parse(&normalized_str)
        .context("Failed to create normalized URL")?;
    
    Ok(normalized.as_str().to_string())
}

/// Check if a string looks like a domain name
fn is_domain_like(s: &str) -> bool {
    // Simple heuristic: contains dots and valid domain characters
    s.contains('.') && 
    s.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
}

/// Validate URL before navigation
pub fn validate_url(url: &str) -> Result<()> {
    let url = Url::parse(url)
        .context("Invalid URL format")?;
    
    // Security checks
    match url.scheme() {
        "javascript" => {
            return Err(anyhow::anyhow!("javascript: URLs are blocked"));
        },
        "file" => {
            return Err(anyhow::anyhow!("file: URLs are blocked"));
        },
        "data" => {
            // Allow data URLs but warn about size
            if let Some(data) = url.path().strip_prefix("data:") {
                // Simple size check (rough estimate)
                if data.len() > 10_000_000 { // 10MB limit
                    return Err(anyhow::anyhow!("Data URL too large"));
                }
            }
        },
        "http" | "https" => {
            // Validate host is present
            if url.host().is_none() {
                return Err(anyhow::anyhow!("URL must have a host"));
            }
        },
        _ => {
            return Err(anyhow::anyhow!("Unsupported URL scheme: {}", url.scheme()));
        }
    }
    
    Ok(())
}

/// Extract search query from input (for search engine redirect)
pub fn extract_search_query(input: &str) -> Option<String> {
    let trimmed = input.trim();
    
    // Don't treat URLs as search queries
    if trimmed.contains("://") || 
       (trimmed.contains('.') && is_domain_like(trimmed)) {
        return None;
    }
    
    // Treat as search query if it contains spaces or no dots
    if trimmed.contains(' ') || (!trimmed.contains('.') && !trimmed.starts_with("about:")) {
        return Some(trimmed.to_string());
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_url_with_protocol() {
        let result = normalize_url("https://example.com").unwrap();
        assert_eq!(result, "https://example.com/");
    }
    
    #[test]
    fn test_normalize_url_without_protocol() {
        let result = normalize_url("example.com").unwrap();
        assert_eq!(result, "https://example.com/");
    }
    
    #[test]
    fn test_normalize_url_removes_default_port() {
        let result = normalize_url("https://example.com:443").unwrap();
        assert_eq!(result, "https://example.com/");
    }
    
    #[test]
    fn test_normalize_url_blocks_javascript() {
        assert!(normalize_url("javascript:alert('xss')").is_err());
    }
    
    #[test]
    fn test_extract_search_query() {
        assert_eq!(extract_search_query("test query"), Some("test query".to_string()));
        assert_eq!(extract_search_query("example.com"), None);
    }
}

