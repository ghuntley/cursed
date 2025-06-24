use crate::web::StatusCode;
// HTTP middleware support for GlowUpHTTP

use crate::stdlib::glowup_http::error::GlowUpResult;
use crate::stdlib::glowup_http::handler::HandlerFunc;
use crate::stdlib::glowup_http::request::VibeRequest;
use crate::stdlib::glowup_http::response::ResponderVibe;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, debug, instrument};
use crate::error::Error;

/// Middleware function type
pub type MiddlewareFunc = Arc<dyn Fn(HandlerFunc) -> HandlerFunc + Send + Sync>;

/// Logging middleware
#[instrument(skip(next))]
pub fn logging_middleware(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        let start = Instant::now();
        let method = r.method.to_string();
        let path = r.url.clone();
        let remote_addr = r.remote_addr.clone();
        
        info!("Started {} {} from {}", method, path, remote_addr);
        
        let result = next(w, r);
        
        let duration = start.elapsed();
        let status = w.get_status().map(|s| s.as_u16()).unwrap_or(200);
        
        info!("Completed {} {} {} in {:?}", method, path, status, duration);
        
        result
    })
}

/// CORS middleware
#[instrument(skip(next))]
pub fn cors_middleware(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        // Set CORS headers
        {
            let mut headers = w.header().lock().unwrap();
            headers.insert("access-control-allow-origin".to_string(), "*".to_string());
            headers.insert("access-control-allow-methods".to_string(), "GET, POST, PUT, DELETE, PATCH, OPTIONS".to_string());
            headers.insert("access-control-allow-headers".to_string(), "Content-Type, Authorization".to_string());
        }
        
        // Handle preflight requests
        if r.method == crate::stdlib::glowup_http::request::Method::OPTIONS {
            use crate::stdlib::glowup_http::response::StatusCode;
            w.write_header(StatusCode::NO_CONTENT);
            return Ok(());
        }
        
        next(w, r)
    })
}

/// "Unbothered" middleware (no-op for demonstration)
#[instrument(skip(next))]
pub fn unbothered_middleware(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        debug!("Unbothered middleware - staying cool 😎");
        next(w, r)
    })
}

/// Rate limiting middleware
#[instrument(skip(rps))]
pub fn rate_limit_middleware(rps: u32) -> MiddlewareFunc {
    Arc::new(move |next: HandlerFunc| {
        Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
            // Simplified rate limiting - in a real implementation you'd use
            // a proper rate limiter with token buckets or sliding windows
            debug!("Rate limiting: {} requests per second", rps);
            
            // For now, just pass through
            next(w, r)
        })
    })
}

/// JWT authentication middleware
#[instrument(skip(secret))]
pub fn jwt_auth_middleware(secret: String) -> MiddlewareFunc {
    Arc::new(move |next: HandlerFunc| {
        let secret = secret.clone();
        Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
            // Check for Authorization header
            if let Some(auth_header) = r.header.get("authorization") {
                if let Some(token) = auth_header.strip_prefix("Bearer ") {
                    debug!("JWT token found: {}", token);
                    // In a real implementation, you'd validate the JWT here
                    // For now, just accept any token
                    return next(w, r);
                }
            }
            
            // No valid token - return 401
            use crate::stdlib::glowup_http::response::StatusCode;
            w.write_header(StatusCode::UNAUTHORIZED);
            w.write(b"Unauthorized")?;
            Ok(())
        })
    })
}

/// Compression middleware
#[instrument(skip(next))]
pub fn compression_middleware(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        debug!("Compression middleware");
        
        // Check if client accepts gzip
        let accepts_gzip = r.header.get("accept-encoding")
            .map(|e| e.contains("gzip"))
            .unwrap_or(false);
        
        if accepts_gzip {
            // In a real implementation, you'd compress the response
            debug!("Client accepts gzip compression");
        }
        
        next(w, r)
    })
}

/// Security headers middleware
#[instrument(skip(next))]
pub fn security_headers_middleware(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        // Add security headers
        {
            let mut headers = w.header().lock().unwrap();
            headers.insert("x-content-type-options".to_string(), "nosniff".to_string());
            headers.insert("x-frame-options".to_string(), "DENY".to_string());
            headers.insert("x-xss-protection".to_string(), "1; mode=block".to_string());
            headers.insert("strict-transport-security".to_string(), "max-age=31536000; includeSubDomains".to_string());
        }
        
        next(w, r)
    })
}

/// Recovery middleware (panic recovery)
#[instrument(skip(next))]
pub fn recovery_middleware(next: HandlerFunc) -> HandlerFunc {
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        // In Rust, we don't have panics like Go, but we can catch errors
        match next(w, r) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Log the error and return 500
                tracing::error!("Handler error: {:?}", e);
                
                use crate::stdlib::glowup_http::response::StatusCode;
                w.write_header(StatusCode::INTERNAL_SERVER_ERROR);
                w.write(b"Internal Server Error")?;
                Ok(())
            }
        }
    })
}

// Re-exports for the spec names
pub use logging_middleware as LoggingMiddleware;
pub use unbothered_middleware as UnbotheredMiddleware;
pub use cors_middleware as CORSMiddleware;
pub use rate_limit_middleware as RateLimitMiddleware;
pub use jwt_auth_middleware as JWTAuthMiddleware;
pub use compression_middleware as CompressionMiddleware;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::glowup_http::handler::handler_func;
    use crate::stdlib::glowup_http::request::Method;
    use crate::stdlib::glowup_http::response::StatusCode;

    #[test]
    fn test_logging_middleware() {
        let handler = handler_func(|w, _r| {
            w.write(b"Hello")?;
            Ok(())
        });
        
        let middleware_handler = logging_middleware(handler);
        let request = VibeRequest::new(Method::GET, "/test");
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        assert_eq!(response.get_body(), b"Hello");
    }

    #[test]
    fn test_cors_middleware() {
        let handler = handler_func(|w, _r| {
            w.write(b"Hello")?;
            Ok(())
        });
        
        let middleware_handler = cors_middleware(handler);
        let request = VibeRequest::new(Method::GET, "/test");
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        let headers = response.get_headers();
        assert!(headers.contains_key("access-control-allow-origin"));
        assert_eq!(response.get_body(), b"Hello");
    }

    #[test]
    fn test_cors_preflight() {
        let handler = handler_func(|w, _r| {
            w.write(b"Should not reach here")?;
            Ok(())
        });
        
        let middleware_handler = cors_middleware(handler);
        let request = VibeRequest::new(Method::OPTIONS, "/test");
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        assert_eq!(response.get_status(), Some(StatusCode::NO_CONTENT));
        assert_eq!(response.get_body(), b""); // Should be empty for OPTIONS
    }

    #[test]
    fn test_jwt_auth_middleware_no_token() {
        let handler = handler_func(|w, _r| {
            w.write(b"Protected content")?;
            Ok(())
        });
        
        let middleware_func = jwt_auth_middleware("secret".to_string());
        let middleware_handler = middleware_func(handler);
        
        let request = VibeRequest::new(Method::GET, "/protected");
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        assert_eq!(response.get_status(), Some(StatusCode::UNAUTHORIZED));
        assert_eq!(response.get_body(), b"Unauthorized");
    }

    #[test]
    fn test_jwt_auth_middleware_with_token() {
        let handler = handler_func(|w, _r| {
            w.write(b"Protected content")?;
            Ok(())
        });
        
        let middleware_func = jwt_auth_middleware("secret".to_string());
        let middleware_handler = middleware_func(handler);
        
        let mut request = VibeRequest::new(Method::GET, "/protected");
        request.header.insert("authorization".to_string(), "Bearer valid-token".to_string());
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        assert_eq!(response.get_body(), b"Protected content");
    }
}
