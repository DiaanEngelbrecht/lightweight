use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tonic::metadata::MetadataMap;
use tonic::{Request, Status};

/// Custom JWT claims containing user identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub user_id: i64,
    pub email: String,
    #[serde(flatten)]
    pub registered: RegisteredClaims,
}

/// Errors that can occur during authentication
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authorization header")]
    MissingAuthHeader,

    #[error("Invalid authorization header format")]
    InvalidAuthHeader,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Failed to verify token: {0}")]
    VerificationFailed(String),
}

impl From<AuthError> for Status {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::MissingAuthHeader => Status::unauthenticated("Missing authentication token"),
            AuthError::InvalidAuthHeader => {
                Status::unauthenticated("Invalid authorization header format")
            }
            AuthError::InvalidToken(msg) => {
                Status::unauthenticated(format!("Invalid token: {}", msg))
            }
            AuthError::TokenExpired => Status::unauthenticated("Token has expired"),
            AuthError::VerificationFailed(msg) => {
                Status::unauthenticated(format!("Token verification failed: {}", msg))
            }
        }
    }
}

/// Extract JWT token from gRPC request metadata
/// Expects "authorization: Bearer {token}" format
fn extract_token_from_metadata(metadata: &MetadataMap) -> Result<String, AuthError> {
    let auth_header = metadata
        .get("authorization")
        .ok_or(AuthError::MissingAuthHeader)?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| AuthError::InvalidAuthHeader)?;

    // Parse "Bearer {token}" format
    if !auth_str.starts_with("Bearer ") {
        return Err(AuthError::InvalidAuthHeader);
    }

    let token = auth_str.strip_prefix("Bearer ").unwrap();
    Ok(token.to_string())
}

/// Verify JWT token and extract claims
pub fn verify_jwt(token: &str, secret: &str) -> Result<AuthClaims, AuthError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())
        .map_err(|e| AuthError::VerificationFailed(e.to_string()))?;

    // Parse and verify the token - extract custom claims directly
    let auth_claims: AuthClaims = token
        .verify_with_key(&key)
        .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

    // Check if token has expired
    if let Some(exp) = auth_claims.registered.expiration {
        let now = chrono::Utc::now().timestamp() as u64;
        if now >= exp {
            return Err(AuthError::TokenExpired);
        }
    }

    Ok(auth_claims)
}

/// Main authentication function - extracts and verifies JWT from request
/// Returns user_id on success, or Status error on failure
pub fn authenticate_request<T>(request: &Request<T>, secret: &str) -> Result<i64, Status> {
    let metadata = request.metadata();
    let token = extract_token_from_metadata(metadata)?;
    let claims = verify_jwt(&token, secret)?;
    Ok(claims.user_id)
}

/// Optional authentication - returns Some(user_id) if authenticated, None if not
/// Only returns error for invalid/expired tokens (not for missing tokens)
pub fn try_authenticate_request<T>(
    request: &Request<T>,
    secret: &str,
) -> Result<Option<i64>, Status> {
    let metadata = request.metadata();

    // If no auth header, return None (not an error)
    if metadata.get("authorization").is_none() {
        return Ok(None);
    }

    // If auth header exists, it must be valid
    let token = extract_token_from_metadata(metadata)?;
    let claims = verify_jwt(&token, secret)?;
    Ok(Some(claims.user_id))
}
