use std::collections::HashMap;
use std::sync::Mutex;

static mut COUNTER: u64 = 0;

fn generate_token() -> String {
    unsafe {
        COUNTER += 1;
        format!("tok_{:016x}", COUNTER)
    }
}

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: u64,
}

#[derive(Debug)]
pub struct AuthService {
    tokens: Mutex<HashMap<String, AuthToken>>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            tokens: Mutex::new(HashMap::new()),
        }
    }

    pub fn authenticate(&self, token: &str) -> Result<AuthToken, AuthError> {
        let tokens = self.tokens.lock().map_err(|_| AuthError::LockError)?;
        
        match tokens.get(token) {
            Some(auth_token) => {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| AuthError::TimeError)?
                    .as_secs();
                
                if current_time > auth_token.expires_at {
                    Err(AuthError::TokenExpired)
                } else {
                    Ok(auth_token.clone())
                }
            }
            None => Err(AuthError::InvalidToken),
        }
    }

    pub fn create_token(&self, user_id: String, duration_seconds: u64) -> Result<String, AuthError> {
        let token = generate_token();
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| AuthError::TimeError)?
            .as_secs();
        
        let auth_token = AuthToken {
            token: token.clone(),
            user_id,
            expires_at: current_time + duration_seconds,
        };

        let mut tokens = self.tokens.lock().map_err(|_| AuthError::LockError)?;
        tokens.insert(token.clone(), auth_token);
        
        Ok(token)
    }

    pub fn revoke_token(&self, token: &str) -> Result<(), AuthError> {
        let mut tokens = self.tokens.lock().map_err(|_| AuthError::LockError)?;
        tokens.remove(token);
        Ok(())
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    TokenExpired,
    LockError,
    TimeError,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid authentication token"),
            AuthError::TokenExpired => write!(f, "Authentication token has expired"),
            AuthError::LockError => write!(f, "Internal synchronization error"),
            AuthError::TimeError => write!(f, "System time error"),
        }
    }
}

impl std::error::Error for AuthError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_authenticate_token() {
        let auth_service = AuthService::new();
        let token = auth_service.create_token("user123".to_string(), 3600).unwrap();
        
        let auth_result = auth_service.authenticate(&token).unwrap();
        assert_eq!(auth_result.user_id, "user123");
        assert_eq!(auth_result.token, token);
    }

    #[test]
    fn test_invalid_token() {
        let auth_service = AuthService::new();
        let result = auth_service.authenticate("invalid_token");
        
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[test]
    fn test_revoke_token() {
        let auth_service = AuthService::new();
        let token = auth_service.create_token("user123".to_string(), 3600).unwrap();
        
        auth_service.revoke_token(&token).unwrap();
        let result = auth_service.authenticate(&token);
        
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }
}